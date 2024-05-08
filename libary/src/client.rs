/*
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 * Copyright (c) 2024 Ferdinand Linnenberg
 *
 * This file is part of CosmicCipher Project, which is dual-licensed under the Apache License 2.0
 * and the MIT License. You may choose either license to govern your use of this file.
 * See the LICENSE-APACHE.md and LICENSE-MIT.md files in the project root for more information.
 */

// Each client has a static private key and a public key
// The public part is expected to be shared with the server
// These keys are used for signing there ECDH ephemeral keys

// For ECDH the client generates an ephemeral key pair and sends the public part the peer
// The client then receives the public part of the peers ephemeral key and computes the shared secret
// The shared secret is then used to derive a symmetric key for encryption

// - curve25519 for ECDH (X25519)
// - ed25519 for signing
// - chacha20-poly1305 for encryption

use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use anyhow::Error;
use argon2::Argon2;
use chacha20poly1305::aead::generic_array::GenericArray;
use chacha20poly1305::{AeadCore, AeadInPlace, KeyInit, XChaCha20Poly1305};
use ed25519_dalek::pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey};
use ed25519_dalek::SecretKey;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use lz4_flex::compress_prepend_size;
use rand_chacha::rand_core::{RngCore, SeedableRng};
use serde::{Deserialize, Serialize};
use x25519_dalek::{PublicKey, StaticSecret};

struct CAData {
    secret_key: Option<SecretKey>,
    verifying_key: VerifyingKey,
}

struct Client {
    signing_key: ed25519_dalek::SigningKey,
    signing_key_signature: ed25519_dalek::Signature,
    ca_data: CAData,
    // Save the ep keys while the kex is ongoing
    // Key is the UUID or E-Mail of the recipient
    kex_map: hashbrown::HashMap<String, StaticSecret>,

    // Shared key map
    shared_keys: hashbrown::HashMap<String, [u8; 32]>,
}

impl Client {
    pub fn new_user() -> Self {
        let mut csprng = rand_chacha::ChaChaRng::from_entropy();
        let signing_key = SigningKey::generate(&mut csprng);
        let ca_signing_key = SigningKey::generate(&mut csprng);

        let sig = ca_signing_key.sign(signing_key.verifying_key().as_bytes());

        Self {
            signing_key,
            signing_key_signature: sig,
            ca_data: CAData {
                secret_key: Some(*ca_signing_key.as_bytes()),
                verifying_key: ca_signing_key.verifying_key(),
            },
            kex_map: hashbrown::HashMap::new(),
            shared_keys: hashbrown::HashMap::new(),
        }
    }

    pub fn export_user(&self, password: &[u8]) -> anyhow::Result<Vec<u8>> {
        let signing_key = self
            .signing_key
            .to_pkcs8_der()
            .map_err(Error::msg)?
            .as_bytes()
            .to_vec();
        // User is expected to have full CA data
        let ca_private_key = match self.ca_data.secret_key {
            None => {
                return Err(Error::msg("CA data is missing"));
            }
            Some(v) => v,
        };
        let ca_sig_key = SigningKey::from_bytes(&ca_private_key);
        let ca_key = ca_sig_key
            .to_pkcs8_der()
            .map_err(Error::msg)?
            .as_bytes()
            .to_vec();

        let user = UserForExport {
            signing_key,
            ca_key,
        };

        let mut serialized = bson::to_vec(&user).map_err(Error::msg)?;

        // Encrypt the user data with a password
        // Hash the password to expand it to a 32 byte key
        let mut csprng = rand_chacha::ChaChaRng::from_entropy();
        let mut salt = [0u8; 16];
        csprng.fill_bytes(&mut salt);

        let mut output_key_material = [0u8; 32];
        let argon2 = Argon2::default();
        argon2
            .hash_password_into(password, &salt, &mut output_key_material)
            .map_err(Error::msg)?;

        // Encrypt the user data with the derived key using chacha20-poly
        let key = GenericArray::from_slice(&output_key_material);
        let cipher = XChaCha20Poly1305::new(key);
        let nonce = XChaCha20Poly1305::generate_nonce(&mut csprng);

        // aead_data is salt + nonce
        let mut aead_data = salt.to_vec();
        aead_data.extend_from_slice(&nonce);

        cipher
            .encrypt_in_place(&nonce, &aead_data, &mut serialized)
            .map_err(Error::msg)?;

        aead_data.extend_from_slice(&serialized);

        Ok(aead_data)
    }

    pub fn import_user(password: &[u8], data: &[u8]) -> anyhow::Result<Self> {
        // Retrieve salt & nonce from the data
        // The first 16 bytes are the salt
        // The next 24 bytes are the nonce
        let salt = &data[0..16];
        let nonce = &data[16..40];

        let mut serialized = data[40..].to_vec();

        // Hash the password to expand it to a 32 byte key
        let mut output_key_material = [0u8; 32];
        let argon2 = Argon2::default();
        argon2
            .hash_password_into(password, salt, &mut output_key_material)
            .map_err(Error::msg)?;

        // Decrypt the user data with the derived key using chacha20-poly
        let key = GenericArray::from_slice(&output_key_material);
        let cipher = XChaCha20Poly1305::new(key);
        let nonce = GenericArray::from_slice(nonce);
        let associated_data = &data[0..40];

        cipher
            .decrypt_in_place(nonce, associated_data, &mut serialized)
            .map_err(Error::msg)?;

        let user: UserForExport = bson::from_slice(&serialized).map_err(Error::msg)?;

        // Import the user data
        let signing_key = SigningKey::from_pkcs8_der(&user.signing_key).map_err(Error::msg)?;

        let ca_signing_key = SigningKey::from_pkcs8_der(&user.ca_key).map_err(Error::msg)?;

        let sig = ca_signing_key.sign(signing_key.verifying_key().as_bytes());

        Ok(Self {
            signing_key,
            signing_key_signature: sig,
            ca_data: CAData {
                secret_key: Some(ca_signing_key.to_bytes()),
                verifying_key: ca_signing_key.verifying_key(),
            },
            kex_map: hashbrown::HashMap::new(),
            shared_keys: hashbrown::HashMap::new(),
        })
    }

    pub fn generate_instance(&self) -> anyhow::Result<Vec<u8>> {
        // An instance is a Client without the CA private key
        let mut csprng = rand_chacha::ChaChaRng::from_entropy();
        let signing_key = SigningKey::generate(&mut csprng);

        let sig = match self.ca_data.secret_key {
            None => {
                return Err(Error::msg("CA data is missing"));
            }
            Some(v) => SigningKey::from_bytes(&v),
        }
        .sign(signing_key.verifying_key().as_bytes());

        let v = InstanceForExport {
            signing_key: signing_key
                .to_pkcs8_der()
                .map_err(Error::msg)?
                .as_bytes()
                .to_vec(),
            sig: sig.to_bytes().to_vec(),
            ca_verifying_key: self
                .ca_data
                .verifying_key
                .to_public_key_der()
                .map_err(Error::msg)?
                .as_bytes()
                .to_vec(),
        };

        let serialized = bson::to_vec(&v).map_err(Error::msg)?;

        Ok(serialized)
    }

    pub fn import_instance(data: &[u8]) -> anyhow::Result<Self> {
        let v: InstanceForExport = bson::from_slice(data).map_err(Error::msg)?;

        let signing_key = SigningKey::from_pkcs8_der(&v.signing_key).map_err(Error::msg)?;
        let sig = ed25519_dalek::Signature::from_slice(&v.sig).map_err(Error::msg)?;
        let ca_verifying_key =
            VerifyingKey::from_public_key_der(&v.ca_verifying_key).map_err(Error::msg)?;

        Ok(Self {
            signing_key,
            signing_key_signature: sig,
            ca_data: CAData {
                secret_key: None,
                verifying_key: ca_verifying_key,
            },
            kex_map: hashbrown::HashMap::new(),
            shared_keys: hashbrown::HashMap::new(),
        })
    }

    pub fn init_dh_kex(&mut self, recipient: &str) -> anyhow::Result<(PublicKey, Signature)> {
        let csprng = rand_chacha::ChaChaRng::from_entropy();
        let ephemeral_key = StaticSecret::random_from_rng(csprng);
        let pubkey = PublicKey::from(&ephemeral_key);
        self.kex_map.insert(recipient.to_string(), ephemeral_key);

        // Sign pubkey
        let sig = self.signing_key.sign(pubkey.as_bytes());

        // Self validate signature
        if self
            .signing_key
            .verifying_key()
            .verify(pubkey.as_bytes(), &sig)
            .is_err()
        {
            return Err(Error::msg("Signature validation failed"));
        }

        Ok((pubkey, sig))
    }

    pub fn complete_dh_kex(
        &mut self,
        recipient: &str,
        pubkey: &PublicKey,
        pubkey_sig: Signature,
        sender_verifing_key: VerifyingKey,
        sender_verifing_key_sig: Signature,
    ) -> anyhow::Result<[u8; 32]> {
        let ephemeral_key = match self.kex_map.remove(recipient) {
            None => {
                return Err(Error::msg("No ephemeral key found"));
            }
            Some(v) => v,
        };

        let shared_secret = ephemeral_key.diffie_hellman(pubkey);
        self.shared_keys
            .insert(recipient.to_string(), shared_secret.to_bytes());

        // Verify the signature (first check if CA signed)
        if self
            .ca_data
            .verifying_key
            .verify(sender_verifing_key.as_bytes(), &sender_verifing_key_sig)
            .is_err()
        {
            return Err(Error::msg("Sender key not signed by CA"));
        }

        if sender_verifing_key
            .verify(pubkey.as_bytes(), &pubkey_sig)
            .is_err()
        {
            return Err(Error::msg("Pubkey not signed by sender"));
        }

        Ok(shared_secret.to_bytes())
    }

    pub fn generate_kex_packet(
        &self,
        public_key: PublicKey,
        sig: Signature,
    ) -> anyhow::Result<Vec<u8>> {
        let kex_packet = KexPacket {
            public_key: public_key.to_bytes(),
            sig: sig.to_bytes().to_vec(),
            verifying_key: self
                .signing_key
                .verifying_key()
                .to_public_key_der()
                .map_err(Error::msg)?
                .as_bytes()
                .to_vec(),
            signing_key_sig: self.signing_key_signature.to_bytes().to_vec(),
        };

        let serialized = bson::to_vec(&kex_packet).map_err(Error::msg)?;

        Ok(serialized)
    }

    pub fn unpack_kex_packet(
        data: &[u8],
    ) -> anyhow::Result<(PublicKey, Signature, VerifyingKey, Signature)> {
        let kex_packet: KexPacket = bson::from_slice(data).map_err(Error::msg)?;

        let public_key = PublicKey::from(kex_packet.public_key);
        let sig = Signature::from_slice(&kex_packet.sig).map_err(Error::msg)?;
        let verifying_key =
            VerifyingKey::from_public_key_der(&kex_packet.verifying_key).map_err(Error::msg)?;
        let signing_key_sig =
            Signature::from_slice(&kex_packet.signing_key_sig).map_err(Error::msg)?;

        Ok((public_key, sig, verifying_key, signing_key_sig))
    }

    pub fn encrypt_message_for_recipient(
        &self,
        recipient: &str,
        message: &[u8],
    ) -> anyhow::Result<Vec<u8>> {
        let shared_key = match self.shared_keys.get(recipient) {
            None => {
                return Err(Error::msg("No shared key found"));
            }
            Some(v) => v,
        };

        let mut csprng = rand_chacha::ChaChaRng::from_entropy();
        let mut nonce = [0u8; 24];
        csprng.fill_bytes(&mut nonce);

        let key = GenericArray::from_slice(shared_key);
        let cipher = XChaCha20Poly1305::new(key);
        let nonce = GenericArray::from_slice(&nonce);

        let mut aead_data = nonce.to_vec();

        // Compression step
        let compressed = compress_prepend_size(message);

        let mut buffer = compressed;

        cipher
            .encrypt_in_place(nonce, &aead_data, &mut buffer)
            .map_err(Error::msg)?;

        aead_data.extend_from_slice(&buffer);

        Ok(aead_data)
    }

    pub fn decrypt_message_from_sender(
        &self,
        sender: &str,
        data: &[u8],
    ) -> anyhow::Result<Vec<u8>> {
        let shared_key = match self.shared_keys.get(sender) {
            None => {
                return Err(Error::msg("No shared key found"));
            }
            Some(v) => v,
        };

        let nonce = &data[0..24];
        let mut buffer = data[24..].to_vec();

        let key = GenericArray::from_slice(shared_key);
        let cipher = XChaCha20Poly1305::new(key);
        let nonce = GenericArray::from_slice(nonce);

        let associated_data = &data[0..24];

        cipher
            .decrypt_in_place(nonce, associated_data, &mut buffer)
            .map_err(Error::msg)?;

        // Decompression step
        let decompressed = lz4_flex::decompress_size_prepended(&buffer).map_err(Error::msg)?;

        Ok(decompressed)
    }
}

#[derive(Serialize, Deserialize)]
struct KexPacket {
    public_key: [u8; 32],
    sig: Vec<u8>,
    verifying_key: Vec<u8>,
    signing_key_sig: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct UserForExport {
    signing_key: Vec<u8>,
    ca_key: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct InstanceForExport {
    signing_key: Vec<u8>,
    sig: Vec<u8>,
    ca_verifying_key: Vec<u8>,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_client() {
        let client = Client::new_user();
        let client_sig = client.signing_key_signature;
        let client_vk = client.signing_key.verifying_key();

        let verification_result = client
            .ca_data
            .verifying_key
            .verify(client_vk.as_bytes(), &client_sig);
        assert!(verification_result.is_ok());
    }

    #[test]
    fn test_export_import_user() {
        let client = Client::new_user();
        let password = b"password";
        let exported = client.export_user(password).unwrap();

        let _ = Client::import_user(password, &exported).unwrap();
    }

    #[test]
    fn test_export_import_instance() {
        let client = Client::new_user();
        let exported = client.generate_instance().unwrap();

        let _ = Client::import_instance(&exported).unwrap();
    }

    #[test]
    fn test_dh_kex() {
        let mut client1 = Client::new_user();

        let instance_data = client1.generate_instance().unwrap();
        let mut client2 = Client::import_instance(instance_data.as_slice()).unwrap();

        let rcpt1 = "client1";
        let rcpt2 = "client2";

        let kexpacket1;

        {
            let pubkey1 = client1.init_dh_kex(rcpt2).unwrap();
            kexpacket1 = client1.generate_kex_packet(pubkey1.0, pubkey1.1).unwrap();
        }

        let kexpacket2;
        {
            let pubkey2 = client2.init_dh_kex(rcpt1).unwrap();
            kexpacket2 = client2.generate_kex_packet(pubkey2.0, pubkey2.1).unwrap();
        }

        {
            let (pubkey1, pubkey1_sig, sender_vk1, sender_vk1_sig) =
                Client::unpack_kex_packet(kexpacket2.as_slice()).unwrap();
            let shared_secret1 = client1
                .complete_dh_kex(rcpt2, &pubkey1, pubkey1_sig, sender_vk1, sender_vk1_sig)
                .unwrap();

            let (pubkey2, pubkey2_sig, sender_vk2, sender_vk2_sig) =
                Client::unpack_kex_packet(kexpacket1.as_slice()).unwrap();
            let shared_secret2 = client2
                .complete_dh_kex(rcpt1, &pubkey2, pubkey2_sig, sender_vk2, sender_vk2_sig)
                .unwrap();

            assert_eq!(shared_secret1, shared_secret2);
        }

        let message = b"Hello World!";
        let encrypted = client1
            .encrypt_message_for_recipient(rcpt2, message)
            .unwrap();
        let decrypted = client2
            .decrypt_message_from_sender(rcpt1, encrypted.as_slice())
            .unwrap();

        assert_eq!(message, decrypted.as_slice());
    }
}
