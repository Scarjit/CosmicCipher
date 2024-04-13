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
    use crate::pre_quantum::key_exchange::new;
    use crate::shared::interfaces::KeyExchanger;

    #[test]
    fn generate_keypair() {
        let _ = new().unwrap();
    }

    #[test]
    fn generate_secret() {
        let mut keypair_1 = new().unwrap();
        let mut keypair_2 = new().unwrap();

        let keypair_public_key_1 = keypair_1.export_public_key().unwrap();
        let keypair_public_key_2 = keypair_2.export_public_key().unwrap();

        let shared_secret_1 = keypair_1
            .generate_shared_secret(&keypair_public_key_2)
            .unwrap();

        let shared_secret_2 = keypair_2
            .generate_shared_secret(&keypair_public_key_1)
            .unwrap();
        assert_eq!(shared_secret_1, shared_secret_2);

        // Let's try to encapsulate again (this should fail)
        let capsule = keypair_1.generate_shared_secret(&keypair_public_key_2);
        assert!(capsule.is_err());
        let capsule = keypair_2.generate_shared_secret(&keypair_public_key_1);
        assert!(capsule.is_err());
    }
}
