// Redo Kernel
// Codes to start cpu cores
// Copyright (c) 2021 Johann Li <me@qinka.pro>


use core::cell::UnsafeCell;
use redo_cpu::cpu::Cpu;
use redo_cpu::interface::CPU;

use redo_platform::boot as platform_boot;
use redo_cpu::boot as cpu_boot;

use crate::info;
use crate::println;

static mut SMP_SLAVE_ABLE: u32 = 0x0000_0000;


// #[naked]
// #[no_mangle]
// pub unsafe fn _boot () -> ! {
//     if MASTER_CORE_ID == Cpu::cpu_id() {
//         Cpu::sp_set(MASTER_CORE_SP);
//         primary_core_launch();
//     } else {
//         secondary_core_launch();
//     }
// }

// #[inline(always)]
#[no_mangle]
pub unsafe fn primary_core_launch () -> ! {



    prim_hardware_init();

    // for _ in 0..0xF {
    //     core::ptr::write_volatile(&mut SMP_SLAVE_ABLE, 1);
    //     Cpu::sev();
    //     Cpu::delay(0xFF);
    // }


    loop {
        Cpu::wfe();
    }
}

#[no_mangle]
pub unsafe fn secondary_core_launch() -> ! {
    // while core::ptr::read_volatile(&SMP_SLAVE_ABLE) == 0 {
    // }

    loop {
        Cpu::wfe();
    }
}

/// Init hardware before all core run
pub unsafe fn prim_hardware_init() {
    // info!("__smp_slave_able {}", __smp_slave_able);
    redo_platform::console::init();
    info!("UART init done!");
    // info!("UART init done!");
    info!("Hardware init done!");
}


// /// Core 0 start
// #[no_mangle]
// pub unsafe fn core0_init() -> !{
//     prim_hardware_init();

//     info!("Core 0 launched!");
//     crate::core0_main();
//     info!("Core 0 finished!");

//     loop {
//         cpu::wfe();
//     }
// }

// /// Core 1 start
// #[no_mangle]
// pub unsafe fn core1_init() -> !{
//     slave_ready_wait();

//     info!("Core 1 launched!");
//     crate::core1_main();
//     info!("Core 1 finished!");

//     loop {
//         cpu::wfe();
//     }
// }

// /// Core 2 start
// #[no_mangle]
// pub unsafe fn core2_init() -> ! {
//     slave_ready_wait();

//     info!("Core 2 launched!");
//     crate::core2_main();
//     info!("Core 2 finished!");

//     loop {
//         cpu::wfe();
//     }
// }

// /// Core 3 start
// #[no_mangle]
// pub unsafe fn core3_init() -> ! {
//     slave_ready_wait();

//     info!("Core 3 launched!");
//     crate::core3_main();
//     info!("Core 3 finished!");

//     loop {
//         cpu::wfe();
//     }
// }
