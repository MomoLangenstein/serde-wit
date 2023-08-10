#![cfg_attr(not(feature = "wasmtime-host"), no_std)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::indexing_slicing)]
#![deny(clippy::panic)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::unreachable)]
#![allow(clippy::module_name_repetitions)]

extern crate alloc;

pub(crate) mod any;
pub(crate) mod intern;

pub mod de;
pub mod ser;

#[must_use]
pub(crate) fn wit_to_usize(size: u32) -> usize {
    const _ASSERT_LOSSLESS_USIZE: [(); 0] = [(); {
        if core::mem::size_of::<u32>() <= core::mem::size_of::<usize>() {
            0
        } else {
            1
        }
    }];

    size as usize
}
