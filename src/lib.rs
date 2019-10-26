//! Low level access to Cortex-A processors
//!
//! This crate provides:
//!
//! - Safe wrappers around assembly instructions
//!
//! For now, there's not much. I will update it gradually.
//! If you want to contribute, feel free to reach out!

#![no_std]
#![feature(asm)]
#![feature(core_intrinsics)]

pub mod asm;
pub mod barrier;
pub mod regs;
pub mod interrupt;
