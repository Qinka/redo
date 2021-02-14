// Redo Kernel
// Copyright (c) 2021 Johann Li <me@qinka.pro>


#![feature(asm)]
#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![no_main]
#![no_std]


// // Platform infos
// mod platform;

// Implements
mod bare;

// // Arch infos
// mod arch;

// Memory
mod memory;

// I/O
mod io;

// Synchronization
mod synchronization;

// Time
// mod time;


// const c0_delay: u32 = 0xF;
// const c1_delay: u32 = 0xFF;
// const c2_delay: u32 = 0xFFF;
// const c3_delay: u32 = 0xFFFF;
// const ag_delay: u32 = 0xFFFFF;

// pub fn core0_main() {
//     unsafe {
//         crate::arch::cpu::delay(c0_delay);
//     }

//     info!("Hello, world! From Core 0");

//     loop {
//         unsafe {
//             crate::arch::cpu::delay(ag_delay);
//         }
//         info!("Hello, world! From Core 0, again!");
//     }

// }


// pub fn core1_main() {
//     unsafe {
//         crate::arch::cpu::delay(c1_delay);
//     }

//     info!("Hello, world! From Core 1");

//     loop {
//         unsafe {
//             crate::arch::cpu::delay(ag_delay);
//         }
//         info!("Hello, world! From Core 1, again!");
//     }
// }


// pub fn core2_main() {
//     unsafe {
//         crate::arch::cpu::delay(c2_delay);
//     }

//     info!("Hello, world! From Core 2");

//     loop {
//         unsafe {
//             crate::arch::cpu::delay(ag_delay);
//         }
//         info!("Hello, world! From Core 2, again!");
//     }
// }


// pub fn core3_main() {
//     unsafe {
//         crate::arch::cpu::delay(c3_delay);
//     }

//     info!("Hello, world! From Core 3");

//     loop {
//         unsafe {
//             crate::arch::cpu::delay(ag_delay);
//         }
//         info!("Hello, world! From Core 3, again!");
//     }
// }

pub fn kernel_main() {

    loop {

    }
}

pub fn slave_main() {

    loop {

    }
}