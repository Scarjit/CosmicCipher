/*
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 * Copyright (c) 2024 Ferdinand Linnenberg
 *
 * This file is part of CosmicCipher Project, which is dual-licensed under the Apache License 2.0
 * and the MIT License. You may choose either license to govern your use of this file.
 * See the LICENSE-APACHE.md and LICENSE-MIT.md files in the project root for more information.
 */


use alloc::vec::Vec;
use anyhow::Error;

pub(crate) trait Signer {
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, Error>;
    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool,Error>;
    fn export_public_key(&self) -> Result<Vec<u8>,Error>;
    
    fn export_private_key(&self) -> Result<Vec<u8>,Error>;
}