/*
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 * Copyright (c) 2024 Ferdinand Linnenberg
 *
 * This file is part of CosmicCipher Project, which is dual-licensed under the Apache License 2.0
 * and the MIT License. You may choose either license to govern your use of this file.
 * See the LICENSE-APACHE.md and LICENSE-MIT.md files in the project root for more information.
 */

use alloc::borrow::ToOwned;
use alloc::vec::Vec;

use anyhow::Error;
use oqs::kem;
use oqs::kem::{Kem, PublicKey, SecretKey};

use crate::shared::interfaces::{KemCapsule, KeyExchanger};

mod mod_test;

const KEM_ALGORITHM: kem::Algorithm = kem::Algorithm::ClassicMcEliece8192128f;
pub struct KeyExchange {
    public_key: PublicKey,
    secret_key: Option<SecretKey>,
    key_exchange_algorithm: Kem,
}

pub fn new() -> Result<KeyExchange, Error> {
    let key_exchange_algorithm = kem::Kem::new(KEM_ALGORITHM).map_err(Error::msg)?;
    let keypair = key_exchange_algorithm.keypair().map_err(Error::msg)?;
    Ok(KeyExchange {
        public_key: keypair.0,
        secret_key: Some(keypair.1),
        key_exchange_algorithm,
    })
}

pub fn new_from_public_key(public_key: &[u8]) -> Result<KeyExchange, Error> {
    let key_exchange_algorithm = kem::Kem::new(KEM_ALGORITHM).map_err(Error::msg)?;
    let key = key_exchange_algorithm
        .public_key_from_bytes(public_key)
        .ok_or_else(|| Error::msg("Invalid public key"))?;
    Ok(KeyExchange {
        public_key: key.to_owned(),
        secret_key: None,
        key_exchange_algorithm,
    })
}

pub fn new_from_private_key(private_key: &[u8], public_key: &[u8]) -> Result<KeyExchange, Error> {
    let key_exchange_algorithm = kem::Kem::new(KEM_ALGORITHM).map_err(Error::msg)?;
    let key = key_exchange_algorithm
        .secret_key_from_bytes(private_key)
        .ok_or_else(|| Error::msg("Invalid private key"))?;
    let public_key = key_exchange_algorithm
        .public_key_from_bytes(public_key)
        .ok_or_else(|| Error::msg("Invalid public key"))?;
    Ok(KeyExchange {
        public_key: public_key.to_owned(),
        secret_key: Some(key.to_owned()),
        key_exchange_algorithm,
    })
}

impl KeyExchanger for KeyExchange {
    fn encapsulate(&self, recipient_public_key: &[u8]) -> Result<KemCapsule, Error> {
        let key_exchange_algorithm = Kem::new(KEM_ALGORITHM).map_err(Error::msg)?;
        let rcpt_public_key = key_exchange_algorithm
            .public_key_from_bytes(recipient_public_key)
            .ok_or_else(|| Error::msg("Invalid public key"))?;
        let (ciphertext, shared_secret) = key_exchange_algorithm
            .encapsulate(rcpt_public_key)
            .map_err(Error::msg)?;
        Ok(KemCapsule {
            ciphertext: ciphertext.to_owned().into_vec(),
            shared_secret: shared_secret.to_owned().into_vec(),
        })
    }

    fn decapsulate(&self, cipher_text: &[u8]) -> Result<Vec<u8>, Error> {
        let key_exchange_algorithm = Kem::new(KEM_ALGORITHM).map_err(Error::msg)?;
        let ciphertext = key_exchange_algorithm
            .ciphertext_from_bytes(cipher_text)
            .ok_or_else(|| Error::msg("Invalid cipher text"))?;
        let shared_secret = key_exchange_algorithm
            .decapsulate(
                self.secret_key
                    .as_ref()
                    .ok_or_else(|| Error::msg("No secret key"))?,
                ciphertext,
            )
            .map_err(Error::msg)?;
        Ok(shared_secret.to_owned().into_vec())
    }

    fn export_public_key(&self) -> Result<Vec<u8>, Error> {
        Ok(self.public_key.as_ref().to_vec())
    }

    fn export_private_key(&self) -> Result<Vec<u8>, Error> {
        Ok(self
            .secret_key
            .as_ref()
            .ok_or_else(|| Error::msg("No secret key"))?
            .as_ref()
            .to_vec())
    }
}
