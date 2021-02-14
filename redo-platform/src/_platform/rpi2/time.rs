//! Architectural timer primitives.
//!
//! # Orientation
//!
//! Since arch modules are imported into generic modules using the path attribute, the path of this
//! file is:
//!
//! crate::time::arch_time

use redo_bare::time;

use redo_bare::register::Register;
use redo_bare::sync::Mutex;
use redo_bare_derive::Register;

use redo_bare::register::PeriRegLock;
use redo_cpu::interface::CPU;
use redo_cpu::cpu::Cpu;

use core::time::Duration;
use core::ptr;



//--------------------------------------------------------------------------------------------------
// Private Definitions
//--------------------------------------------------------------------------------------------------

const SYS_TIMER_BASE: usize = 0x3F003000;
const SYS_TIMER_CLO : usize = SYS_TIMER_BASE + 0x4;
const SYS_TIMER_CHI : usize = SYS_TIMER_BASE + 0x8;

#[repr(C)]
#[derive(Register)]
struct SYS_TIMER {
    _U1: [u32; 1],
    CLO: u32,
    CHI: u32,
}

impl SYS_TIMER {
    unsafe fn init(&mut self) {
        // TODO
    }

}



/// Bare metal logger timer.
struct BareLoggerTimer {
    sys_timer: PeriRegLock<SYS_TIMER>,
}

impl BareLoggerTimer {
    pub const fn new() -> BareLoggerTimer {
        BareLoggerTimer {
            sys_timer: PeriRegLock::new(SYS_TIMER_BASE)
        }
    }

    pub unsafe fn reg_clo(&self) -> u32 {
        self.sys_timer.lock(|timer| {
            ptr::read_volatile(&timer.CLO)
        })
    }

    pub unsafe fn reg_chi(&self) -> u32 {
        self.sys_timer.lock(|timer| {
            ptr::read_volatile(&timer.CHI)
        })
    }
}

static BARE_LOG_TIME_MANAGER: BareLoggerTimer = BareLoggerTimer::new();

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

/// Return a reference to the time manager.
pub fn bare_log_time_manager() -> &'static impl time::LoggerTimerCount {
    &BARE_LOG_TIME_MANAGER
}

//------------------------------------------------------------------------------
// OS Interface Code
//------------------------------------------------------------------------------

impl time::LoggerTimerCount for BareLoggerTimer {


    fn uptime_hi(&self) -> u32 {
        unsafe {
            self.reg_chi()
        }
    }

    fn uptime_lo(&self) -> u32 {
        unsafe {
            self.reg_clo()
        }
    }
}