/*
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 * Copyright (c) 2024 Ferdinand Linnenberg
 *
 * This file is part of CosmicCipher Project, which is dual-licensed under the Apache License 2.0
 * and the MIT License. You may choose either license to govern your use of this file.
 * See the LICENSE-APACHE.md and LICENSE-MIT.md files in the project root for more information.
 */

use crate::shared::helpers::validate_key_algorithm;
use alloc::string::String;
use alloc::vec::Vec;
use anyhow::Error;
use oqs::sig;
use oqs::sig::{Algorithm, PublicKey, SecretKey};
use serde::ser::SerializeStruct;
use sha3::Digest;

use crate::shared::interfaces::Signer;

mod mod_test;

// Cyber-Dilithium was standardized as ML-DSA in FIPS 204 (https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.204.ipd.pdf)
const SIGNATURE_ALGORITHM: Algorithm = Algorithm::MlDsa87Ipd;
pub struct SigningKey {
    public_key: PublicKey,
    secret_key: Option<SecretKey>,
    signature_algorithm: sig::Sig,
}

pub fn new() -> Result<SigningKey, Error> {
    let signature_algorithm = sig::Sig::new(SIGNATURE_ALGORITHM).map_err(Error::msg)?;
    let keypair = signature_algorithm.keypair().map_err(Error::msg)?;
    Ok(SigningKey {
        public_key: keypair.0,
        secret_key: Some(keypair.1),
        signature_algorithm,
    })
}

pub fn new_from_public_key(public_key: &[u8]) -> Result<SigningKey, Error> {
    let signature_algorithm = sig::Sig::new(SIGNATURE_ALGORITHM).map_err(Error::msg)?;
    let key = validate_key_algorithm(public_key)?;

    let public_key = signature_algorithm
        .public_key_from_bytes(key)
        .ok_or_else(|| Error::msg("Invalid public key"))?;

    Ok(SigningKey {
        public_key: public_key.to_owned(),
        secret_key: None,
        signature_algorithm,
    })
}

pub fn new_from_private_key(private_key: &[u8], public_key: &[u8]) -> Result<SigningKey, Error> {
    let signature_algorithm = sig::Sig::new(SIGNATURE_ALGORITHM).map_err(Error::msg)?;

    let validated_private_key = validate_key_algorithm(public_key)?;
    let public_key = signature_algorithm
        .public_key_from_bytes(validated_private_key)
        .ok_or_else(|| Error::msg("Invalid public key"))?;

    let validated_public_key = validate_key_algorithm(private_key)?;
    let secret_key = signature_algorithm
        .secret_key_from_bytes(validated_public_key)
        .ok_or_else(|| Error::msg("Invalid secret key"))?;

    let sk = SigningKey {
        public_key: public_key.to_owned(),
        secret_key: Some(secret_key.to_owned()),
        signature_algorithm,
    };

    // Validate that public key belongs to the private key by generating and validating a signature
    let message = b"Hello, World!";
    let signature = sk.sign(message)?;
    if !sk.verify(message, &signature)? {
        return Err(Error::msg("Public key does not belong to private key"));
    }

    Ok(sk)
}

impl Signer for SigningKey {
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, Error> {
        // Generate a hash of the message to sign using SHA-3 512
        let hash = sha3::Sha3_512::digest(message);

        // Sign the hash
        let secret_key = self
            .secret_key
            .as_ref()
            .ok_or_else(|| Error::msg("No secret key"))?;
        let signature = self
            .signature_algorithm
            .sign(&hash, secret_key)
            .map_err(Error::msg)?;
        Ok(signature.into_vec())
    }

    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool, Error> {
        // Generate a hash of the message to sign using SHA-3 512
        let hash = sha3::Sha3_512::digest(message);

        // Validate the signature against the message hash
        let signature = self
            .signature_algorithm
            .signature_from_bytes(signature)
            .ok_or_else(|| Error::msg("Invalid signature"))?;

        let validation_result = self
            .signature_algorithm
            .verify(&hash, signature, &self.public_key);

        // Let's check the validation result
        match validation_result {
            Ok(()) => Ok(true),
            Err(e) => {
                // Check the type of Error
                match e {
                    oqs::Error::Error => Ok(false),
                    _ => Err(Error::msg(e)),
                }
            }
        }
    }

    fn export_public_key(&self) -> Result<Vec<u8>, Error> {
        // Algorithm + public_key
        let mut public_key: Vec<u8> = Vec::new();
        public_key.extend_from_slice(SIGNATURE_ALGORITHM.name().as_bytes());
        public_key.extend_from_slice(self.public_key.as_ref());
        Ok(public_key)
    }

    fn export_private_key(&self) -> Result<Vec<u8>, Error> {
        let secret_key = self
            .secret_key
            .as_ref()
            .ok_or_else(|| Error::msg("No secret key"))?;
        let mut private_key: Vec<u8> = Vec::new();
        private_key.extend_from_slice(SIGNATURE_ALGORITHM.name().as_bytes());
        private_key.extend_from_slice(secret_key.as_ref());
        Ok(private_key)
    }
}

// Implement serialization and deserialization for SigningKey
#[cfg(feature = "serde")]
impl serde::Serialize for SigningKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize the public key
        let public_key = self
            .export_public_key()
            .map_err(serde::ser::Error::custom)?;
        // Serialize the secret key
        let secret_key = self
            .export_private_key()
            .map_err(serde::ser::Error::custom)?;

        // Serialize the public key and secret key
        let mut state = serializer.serialize_struct("SigningKey", 2)?;
        state.serialize_field(FIELDS[0], &public_key)?;
        state.serialize_field(FIELDS[1], &secret_key)?;
        state.end()
    }
}
const FIELDS: &'static [&'static str] = &["public_key", "secret_key"];

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SigningKey {
    fn deserialize<D>(deserializer: D) -> Result<SigningKey, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct("SigningKey", FIELDS, SigningKeyVisitor)
    }
}

#[cfg(feature = "serde")]
struct SigningKeyVisitor;
#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for SigningKeyVisitor {
    type Value = SigningKey;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a struct with fields public_key and secret_key")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut public_key: Option<Vec<u8>> = None;
        let mut secret_key: Option<Vec<u8>> = None;
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "public_key" => {
                    public_key = Some(map.next_value()?);
                }
                "secret_key" => {
                    secret_key = Some(map.next_value()?);
                }
                _ => return Err(serde::de::Error::unknown_field(&key, FIELDS)),
            }
        }
        let public_key = public_key.ok_or_else(|| serde::de::Error::missing_field("public_key"))?;
        let secret_key = secret_key.ok_or_else(|| serde::de::Error::missing_field("secret_key"))?;

        let keypair =
            new_from_private_key(&secret_key, &public_key).map_err(serde::de::Error::custom)?;
        Ok(keypair)
    }
}
