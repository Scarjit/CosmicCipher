/*
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 * Copyright (c) 2024 Ferdinand Linnenberg
 *
 * This file is part of CosmicCipher Project, which is dual-licensed under the Apache License 2.0
 * and the MIT License. You may choose either license to govern your use of this file.
 * See the LICENSE-APACHE.md and LICENSE-MIT.md files in the project root for more information.
 */

use lz4_flex::compress_prepend_size;

#[test]
fn test_lz4_compression() {
    let input = include_bytes!("input.json");

    let compressed = compress_prepend_size(input);

    println!("Uncompressed: {}", input.len());
    println!("Compressed: {}", compressed.len());
    let ratio = input.len() as f64 / compressed.len() as f64;
    print!("Ratio: {}", ratio);
}
