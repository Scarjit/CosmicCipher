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
    use crate::post_quantum::signature::new_public_key_only;
use crate::post_quantum::signature::new;
    use crate::shared_interfaces::Signer;

    #[test]
    fn generate_keypair() {
        let keypair = new().unwrap();
        assert!(keypair.secret_key.is_some());
    }

    #[test]
    fn generate_public_key_only() {
        let keypair = new().unwrap();
        let public_key = keypair.export_public_key().unwrap();
        let public_key_only = new_public_key_only(&public_key).unwrap();
        assert!(public_key_only.secret_key.is_none());
        assert_eq!(public_key_only.public_key, keypair.public_key);
    }

    #[test]
    fn sign_verify() {
        let keypair = new().unwrap();
        let message = b"Hello, World!";
        let signature = keypair.sign(message).unwrap();
        assert!(keypair.verify(message, &signature).unwrap());
    }

    #[test]
    fn export_sign_import_verify() {
        let keypair = new().unwrap();
        let message = b"Hello, World!";
        let signature = keypair.sign(message).unwrap();
        let public_key = keypair.export_public_key().unwrap();
        let public_key_only = new_public_key_only(&public_key).unwrap();
        assert!(public_key_only.verify(message, &signature).unwrap());
    }
}