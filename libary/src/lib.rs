/*
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 * Copyright (c) 2024 Ferdinand Linnenberg
 *
 * This file is part of CosmicCipher Project, which is dual-licensed under the Apache License 2.0
 * and the MIT License. You may choose either license to govern your use of this file.
 * See the LICENSE-APACHE.md and LICENSE-MIT.md files in the project root for more information.
 */


#![no_std]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::panic)]
#![deny(clippy::panicking_unwrap)]
#![deny(clippy::expect_used)]
#![feature(error_in_core)]
#![allow(dead_code)]

extern crate alloc;

pub(crate) mod pre_quantum;
pub(crate) mod post_quantum;
pub(crate) mod shared_interfaces;

