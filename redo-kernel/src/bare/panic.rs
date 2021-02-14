// Redo Kernel
// Handler of panic
// Copyright (c) 2021 Johann Li <me@qinka.pro>

use core::panic::PanicInfo;
use redo_cpu::cpu::Cpu;
use redo_cpu::interface::CPU;
// use crate::io::

/// Panic Handler
#[panic_handler]
unsafe fn panic(info: &PanicInfo) -> ! {
    // info!("{}", info);

    loop {
        Cpu::wfe();
    }
}