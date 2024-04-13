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

use crate::shared::helpers::validate_key_algorithm;
use crate::shared::interfaces::KeyExchanger;

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

impl KeyExchanger for KeyExchange {
    fn generate_shared_secret(&mut self, recipient_public_key: &[u8]) -> Result<Vec<u8>, Error> {
        let validated_private_key =
            validate_key_algorithm(recipient_public_key, KEM_ALGORITHM.name())?;
        let key_exchange_algorithm = Kem::new(KEM_ALGORITHM).map_err(Error::msg)?;
        let rcpt_public_key = key_exchange_algorithm
            .public_key_from_bytes(&validated_private_key)
            .ok_or_else(|| Error::msg("Invalid public key"))?;
        let (_, shared_secret) = key_exchange_algorithm
            .encapsulate(rcpt_public_key)
            .map_err(Error::msg)?;
        Ok(shared_secret.to_owned().into_vec())
    }

    fn export_public_key(&self) -> Result<Vec<u8>, Error> {
        // Algorithm + public_key
        let mut public_key: Vec<u8> = Vec::new();
        public_key.extend_from_slice(KEM_ALGORITHM.name().as_bytes());
        public_key.extend_from_slice(self.public_key.as_ref());
        Ok(public_key)
    }
}
