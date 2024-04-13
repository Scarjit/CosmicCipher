/*
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 * Copyright (c) 2024 Ferdinand Linnenberg
 *
 * This file is part of CosmicCipher Project, which is dual-licensed under the Apache License 2.0
 * and the MIT License. You may choose either license to govern your use of this file.
 * See the LICENSE-APACHE.md and LICENSE-MIT.md files in the project root for more information.
 */

#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]
#![allow(clippy::panicking_unwrap)]
#![allow(clippy::expect_used)]

#[cfg(test)]
mod test {
    use crate::post_quantum::key_exchange::{new, new_from_private_key, new_from_public_key};
    use crate::shared::interfaces::KeyExchanger;

    #[test]
    fn generate_keypair() {
        let keypair = new().unwrap();
        assert!(keypair.secret_key.is_some());
    }

    #[test]
    fn encapsulate_decapsulate() {
        let keypair = new().unwrap();
        let keypair2 = new().unwrap();
        let capsule = keypair
            .encapsulate(&keypair2.export_public_key().unwrap())
            .unwrap();
        let shared_secret = keypair2.decapsulate(&capsule.ciphertext).unwrap();
        assert_eq!(capsule.shared_secret, shared_secret);
    }

    #[test]
    fn import_public_key() {
        let keypair = new().unwrap();
        let public_key = keypair.export_public_key().unwrap();
        let public_key_only = new_from_public_key(&public_key).unwrap();
        assert!(public_key_only.secret_key.is_none());
        assert_eq!(public_key_only.public_key, keypair.public_key);
    }

    #[test]
    fn import_private_and_public_key() {
        let keypair = new().unwrap();
        let private_key = keypair.export_private_key().unwrap();
        let public_key = keypair.export_public_key().unwrap();
        let keypair2 = new_from_private_key(&private_key, &public_key).unwrap();
        assert_eq!(keypair.public_key, keypair2.public_key);
    }
}
