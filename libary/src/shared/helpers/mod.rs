/*
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 * Copyright (c) 2024 Ferdinand Linnenberg
 *
 * This file is part of CosmicCipher Project, which is dual-licensed under the Apache License 2.0
 * and the MIT License. You may choose either license to govern your use of this file.
 * See the LICENSE-APACHE.md and LICENSE-MIT.md files in the project root for more information.
 */

use anyhow::Error;

pub(crate) fn validate_key_algorithm(key: &[u8]) -> Result<&[u8], Error> {
    // SIGNATURE_ALGORITHM + key
    // Read until first null byte to get the algorithm
    let (alg, key) = key.split_at(
        crate::pre_quantum::signature::SIGNATURE_ALGORITHM
            .name()
            .len(),
    );
    if alg
        != crate::pre_quantum::signature::SIGNATURE_ALGORITHM
            .name()
            .as_bytes()
    {
        return Err(Error::msg("Invalid algorithm"));
    }
    Ok(key)
}
