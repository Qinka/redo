// Redo Kernel
// Platform Related
// Copyright (c) 2021 Johann Li <me@qinka.pro>

#![no_std]
#![feature(const_mut_refs)]
#![feature(global_asm)]

// Platform Raspberry Pi2
#[cfg(board = "rpi2")]
#[path = "_platform/rpi2/memory.rs"]
pub mod memory;


#[cfg(board = "rpi2")]
#[path = "_platform/rpi2/console.rs"]
pub mod console;

#[cfg(board = "rpi2")]
#[path = "_platform/rpi2/time.rs"]
pub mod time;

#[cfg(board = "rpi2")]
#[path = "_platform/rpi2/boot.rs"]
pub mod boot;