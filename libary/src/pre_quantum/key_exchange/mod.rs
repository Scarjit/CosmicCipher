/*
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 * Copyright (c) 2024 Ferdinand Linnenberg
 *
 * This file is part of CosmicCipher Project, which is dual-licensed under the Apache License 2.0
 * and the MIT License. You may choose either license to govern your use of this file.
 * See the LICENSE-APACHE.md and LICENSE-MIT.md files in the project root for more information.
 */

use alloc::vec;
use alloc::vec::Vec;

use anyhow::Error;
use x25519_dalek::{EphemeralSecret, PublicKey};

use crate::pre_quantum::key_exchange::algorithm::Algorithm;
use crate::pre_quantum::key_exchange::algorithm::Algorithm::X25519;
use crate::shared::interfaces::{KemCapsule, KeyExchanger};

mod algorithm;
mod mod_test;

const KEM_ALGORITHM: Algorithm = X25519;

pub struct KeyExchange {
    secret_key: Option<EphemeralSecret>,
}

pub fn new() -> Result<KeyExchange, Error> {
    Ok(KeyExchange {
        secret_key: Some(EphemeralSecret::random()),
    })
}

impl KeyExchanger for KeyExchange {
    fn encapsulate(&mut self, recipient_public_key: &[u8]) -> Result<KemCapsule, Error> {
        let recipient_public_key =
            PublicKey::from(<[u8; 32]>::try_from(recipient_public_key).map_err(Error::msg)?);
        let ephemeral_secret = self
            .secret_key
            .take()
            .ok_or_else(|| Error::msg("No secret key"))?;
        let shared = ephemeral_secret.diffie_hellman(&recipient_public_key);
        return Ok(KemCapsule {
            ciphertext: vec![],
            shared_secret: shared.as_bytes().to_vec(),
        });
    }

    // decapsulate and encapsulate are the same function in x25519
    fn decapsulate(&mut self, recipient_public_key: &[u8]) -> Result<Vec<u8>, Error> {
        self.encapsulate(recipient_public_key)
            .map(|capsule| capsule.shared_secret)
    }

    fn export_public_key(&self) -> Result<Vec<u8>, Error> {
        let public_key = PublicKey::from(
            self.secret_key
                .as_ref()
                .ok_or_else(|| Error::msg("No secret key"))?,
        );
        return Ok(public_key.as_bytes().to_vec());
    }
}
