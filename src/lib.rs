#![cfg_attr(not(feature = "wasmtime-host"), no_std)]

extern crate alloc;

pub(crate) mod any;
pub(crate) mod intern;

pub mod de;
pub mod ser;
