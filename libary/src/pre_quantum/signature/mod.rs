/*
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 * Copyright (c) 2024 Ferdinand Linnenberg
 *
 * This file is part of CosmicCipher Project, which is dual-licensed under the Apache License 2.0
 * and the MIT License. You may choose either license to govern your use of this file.
 * See the LICENSE-APACHE.md and LICENSE-MIT.md files in the project root for more information.
 */

use alloc::string::String;
use alloc::vec::Vec;

use anyhow::Error;
use ed25519_dalek::{Verifier, VerifyingKey};
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};
#[cfg(feature = "serde")]
use serde::ser::SerializeStruct;
use sha3::Digest;

use crate::pre_quantum::signature::algorithm::Algorithm;
use crate::pre_quantum::signature::algorithm::Algorithm::Ed25519;
use crate::shared::helpers::validate_key_algorithm;
use crate::shared::interfaces::Signer;

mod algorithm;
mod mod_test;

// FIPS 186-5: Digital Signature Standard (DSS) (https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.186-5.pdf)
pub(crate) const SIGNATURE_ALGORITHM: Algorithm = Ed25519;
pub struct SigningKey {
    public_key: VerifyingKey,
    secret_key: Option<ed25519_dalek::SigningKey>,
}

pub fn new() -> Result<SigningKey, Error> {
    let mut small_rng = SmallRng::from_entropy();
    // Generate 256-bit secret key
    let mut secret_key_bytes = [0u8; 32];
    small_rng.fill_bytes(&mut secret_key_bytes);

    let keypair = ed25519_dalek::SigningKey::from_bytes(&secret_key_bytes);
    Ok(SigningKey {
        public_key: keypair.verifying_key(),
        secret_key: Some(keypair),
    })
}

pub fn new_from_public_key(public_key: &[u8]) -> Result<SigningKey, Error> {
    let key = validate_key_algorithm(public_key)?;
    let public_key = VerifyingKey::from_bytes(<&[u8; 32]>::try_from(key).map_err(Error::msg)?)
        .map_err(Error::msg)?;
    Ok(SigningKey {
        public_key,
        secret_key: None,
    })
}

pub fn new_from_private_key(private_key: &[u8], public_key: &[u8]) -> Result<SigningKey, Error> {
    let validated_private_key = validate_key_algorithm(private_key)?;

    let keypair = ed25519_dalek::SigningKey::from_bytes(
        <&ed25519_dalek::SecretKey>::try_from(validated_private_key).map_err(Error::msg)?,
    );
    let public_key_from_keypair = keypair.verifying_key();

    let validated_public_key = validate_key_algorithm(public_key)?;
    let public_key_imported =
        VerifyingKey::from_bytes(<&[u8; 32]>::try_from(validated_public_key).map_err(Error::msg)?)
            .map_err(Error::msg)?;
    if !public_key_imported.eq(&public_key_from_keypair) {
        return Err(Error::msg("Public key does not belong to private key"));
    }
    Ok(SigningKey {
        public_key: public_key_imported,
        secret_key: Some(keypair),
    })
}

impl Signer for SigningKey {
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, Error> {
        // Generate a hash of the message to sign using SHA-3 512
        let hash = sha3::Sha3_512::digest(message);

        // Sign the hash
        use ed25519_dalek::ed25519::signature::Signer;
        let signature = ed25519_dalek::SigningKey::sign(self.secret_key.as_ref().unwrap(), &hash);
        Ok(signature.to_bytes().to_vec())
    }

    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool, Error> {
        let hash = sha3::Sha3_512::digest(message);
        let signature = ed25519_dalek::Signature::from_bytes(
            <&[u8; 64]>::try_from(signature).map_err(Error::msg)?,
        );

        match self.public_key.verify(&hash, &signature) {
            Ok(_) => match self.public_key.verify_strict(&hash, &signature) {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            },
            Err(_) => Ok(false),
        }
    }

    fn export_public_key(&self) -> Result<Vec<u8>, Error> {
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
        private_key.extend_from_slice(secret_key.as_bytes());
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
