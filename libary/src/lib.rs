#![no_std]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::panic)]
#![deny(clippy::panicking_unwrap)]
#![deny(clippy::expect_used)]
#![feature(error_in_core)]

extern crate alloc;

pub(crate) mod pre_quantum;
pub(crate) mod post_quantum;
pub(crate) mod shared_interfaces;

