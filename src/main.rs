#![no_main]
#![cfg_attr(target_os = "none", no_std)]
#![allow(clippy::single_component_path_imports)]
// Force linking to the `mycelium_kernel` lib.
#[allow(unused_imports)]
use mycelium_kernel;
