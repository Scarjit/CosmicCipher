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
    fn encapsulate_decapsulate() {
        let mut keypair = new().unwrap();
        let keypair_public_key = keypair.export_public_key().unwrap();

        let mut keypair2 = new().unwrap();
        let keypair2_public_key = keypair2.export_public_key().unwrap();

        let capsule = keypair.encapsulate(&keypair2_public_key).unwrap();

        let shared_secret = keypair2.decapsulate(&keypair_public_key).unwrap();
        assert_eq!(capsule.shared_secret, shared_secret);

        // TODO: Ensure that keypair and keypair2's secret keys are dropped after decapsulation
    }
}
