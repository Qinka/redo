// Redo Kernel
// Console for
// Copyright (c) 2021 Johann Li <me@qinka.pro>

use core::fmt;
use core::ptr::{read_volatile, write_volatile};

use redo_bare::register::Register;
use redo_bare::sync::Mutex;
use redo_bare_derive::Register;

use redo_bare::register::PeriRegLock;
use redo_cpu::interface::CPU;
use redo_cpu::cpu::Cpu;

use redo_bare::interface::DirectWrite;

// use redo_bare_derive;

#[repr(C)]
#[derive(Register)]
struct UART {
    DR    :  u32     ,
    RSRECR:  u32     ,
    _U1   : [u32; 4] ,
    FR    :  u32     ,
    ILPR  :  u32     ,
    IBRD  :  u32     ,
    FBRD  :  u32     ,
    LCRH  :  u32     ,
    CR    :  u32     ,
    IFLS  :  u32     ,
    IMSC  :  u32     ,
    RIS   :  u32     ,
    MIS   :  u32     ,
    ICR   :  u32     ,
    DMACR :  u32     ,
    _U2   : [u32; 13],
    ITCR  :  u32     ,
    ITIP  :  u32     ,
    ITOP  :  u32     ,
    TDR   :  u32     ,
}

impl UART {
    unsafe fn init(&mut self) {
        // Clear UART
        self.CR = 0x0000_0000;
        // write_volatile(&mut self.CR   as *mut u32, 0x0000_0000);

        // Clock init
        // self.GPPUD = 0x0000_0000;
        write_volatile(GPPUD      as *mut u32, 0x0000_0000);
        Cpu::delay(150);

        // 0xC000, (1 << 14) | (1 << 15)
        // self.GPPUDCLK0 = 0x0000_C000;
        write_volatile(GPPUDCLK0  as *mut u32, 0x0000_C000);
        Cpu::delay(150);

        // self.GPPUDCLK0 = 0x0000_0000;
        write_volatile(GPPUDCLK0  as *mut u32, 0x0000_0000);

        // Setup UART
        self.ICR = 0x0000_07FF;
        // write_volatile(&mut self.ICR  as *mut u32, 0x0000_07FF);

        self.IBRD = 26;
        self.FBRD = 27;
        // write_volatile(&mut self.IBRD as *mut u32, 26);
        // write_volatile(&mut self.FBRD as *mut u32, 27);

        // 0x70, (1 << 4) | (1 << 5) | (1 << 6)
        self.LCRH = 0x0000_0079;
        // write_volatile(&mut self.LCRH as *mut u32, 0x0000_0079);
        // 0x7f2 (1 << 1) | (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7) | (1 << 8) | (1 << 9) | (1 << 10)
        self.IMSC = 0x0000_07F2;
        // write_volatile(&mut self.IMSC as *mut u32, 0x0000_07F2);
        // 0x301 (1 << 0) | (1 << 8) | (1 << 9)
        self.CR = 0x0000_0301;
        // write_volatile(&mut self.CR   as *mut u32, 0x0000_0301);
    }

    #[inline(always)]
    unsafe fn write_u8(&mut self, c: u8) {
        // 0x20, (1 << 5)
        while (read_volatile(&mut self.FR) & 0x20) != 0 {};
        write_volatile(&mut self.DR as *mut u32 as *mut u8, c);
    }
}

const GPIO_BASE   : usize = 0x3F200000;
const GPPUD       : usize = GPIO_BASE + 0x0094;
const GPPUDCLK0   : usize = GPIO_BASE + 0x0098;

const UART0_BASE  : usize = GPIO_BASE + 0x1000;

// const UART0  : &'static mut UART  = regi::<UART>(UART0_BASE);

// const UART0_DR    : usize = UART0_BASE + 0x00;
// const UART0_RSRECR: usize = UART0_BASE + 0x04;
// const UART0_FR    : usize = UART0_BASE + 0x18;
// const UART0_ILPR  : usize = UART0_BASE + 0x20;
// const UART0_IBRD  : usize = UART0_BASE + 0x24;
// const UART0_FBRD  : usize = UART0_BASE + 0x28;
// const UART0_LCRH  : usize = UART0_BASE + 0x2C;
// const UART0_CR    : usize = UART0_BASE + 0x30;
// const UART0_IFLS  : usize = UART0_BASE + 0x34;
// const UART0_IMSC  : usize = UART0_BASE + 0x38;
// const UART0_RIS   : usize = UART0_BASE + 0x3C;
// const UART0_MIS   : usize = UART0_BASE + 0x40;
// const UART0_ICR   : usize = UART0_BASE + 0x44;
// const UART0_DMACR : usize = UART0_BASE + 0x48;
// const UART0_ITCR  : usize = UART0_BASE + 0x80;
// const UART0_ITIP  : usize = UART0_BASE + 0x84;
// const UART0_ITOP  : usize = UART0_BASE + 0x88;
// const UART0_TDR   : usize = UART0_BASE + 0x8C;

// pub struct Rpi2SerialKernel {
//     uart:  *const UART,
// }

// impl Rpi2SerialKernel {
//     const fn new() -> Rpi2SerialKernel {
//         Rpi2SerialKernel{ uart: UART0}
//     }

//     #[inline(always)]
//     fn write_char(&mut self, c: char) {
//         (*self.uart).write_u8(c as u8);
//     }

//     fn lock(&mut self, f: impl FnOnce(&mut Self) -> fmt::Result) {
//         f(self);
//     }
// }

impl fmt::Write for UART {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            unsafe { self.write_u8(c as u8); }
        }
        Ok(())
    }
}

pub struct Rpi2SerialOutput {
    // lock: PseudoLock<u32>,
    // uart: &'static mut UART,
    uart: PeriRegLock<UART>,
}

static RPI2_SER_OUTPUT: Rpi2SerialOutput = Rpi2SerialOutput::new();

impl Rpi2SerialOutput {
    /// Create a new instance.
    pub const fn new() -> Rpi2SerialOutput {
        Rpi2SerialOutput {
            uart: PeriRegLock::new(UART0_BASE),
            // lock: PseudoLock::new(0),
            // uart: UART0,
            // kernel: PseudoLock::new(Rpi2SerialKernel::new()),
        }
    }

    // #[inline(always)]
    // fn write_char(&mut self, c: char) {
    //     self.uart.lock(|pU| (*pU).write_u8(c as u8));
    //     // (*self.uart).write_u8(c as u8);
    // }

    // fn lock(&mut self, f: impl FnOnce(&mut Self) -> fmt::Result) {
    //     f(self);
    // }

    // /// With lock
    // pub fn lock(&self, f: impl FnOnce(&mut Rpi2SerialKernel) -> fmt::Result) -> fmt::Result {
    //     self.kernel.lock(|ker| f(ker))
    // }
}


impl DirectWrite for Rpi2SerialOutput {
    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result {
        use core::fmt::Write;
        self.uart.lock(|uart| Write::write_fmt(uart, args))
    }
}

/// Return a reference to the console.
pub fn console() -> &'static impl DirectWrite {
    &RPI2_SER_OUTPUT
}


// impl fmt::Write for Rpi2SerialOutput {
//     fn write_str(&mut self, s: &str) -> fmt::Result {
//         self.kernel.lock(|ker| fmt::Write::write_str(ker, s))
//     }
// }


/// Console init
pub unsafe fn init() {
    // UART 0 init

    UART::bare(UART0_BASE).init();
}