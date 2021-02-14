// Redo Kernel
// Architecture Related
// Copyright (c) 2021 Johann Li <me@qinka.pro>

#![feature(asm)]
#![feature(global_asm)]
#![no_std]

// interface
pub mod interface;

// CPU related

// #[cfg(target_arch = "arm")]
#[path = "_arch/armv7/cpu.rs"]
pub mod cpu;

#[cfg(target_arch = "arm")]
#[path = "_arch/armv7/boot.rs"]
pub mod boot;

#[cfg(target_arch = "arm")]
#[path = "_arch/armv7/memory.rs"]
pub mod memory;

// #[cfg(target_arch = "arm")]
#[path = "_arch/armv7/lock.rs"]
pub mod lock;

// #[cfg(target_arch = "arm")]
// #[path = "_arch/armv7/time.rs"]
// pub mod time;


// #[cfg(target_arch = "arm")]
// #[path = "_arch/armv7/smp.rs"]
// pub mod smp;