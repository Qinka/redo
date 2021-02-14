
use crate::interface::CPU;

pub struct Armv7A;

impl CPU for Armv7A {
    #[inline(always)]
    unsafe fn delay(count: u32) {
        for _ in 0..count {
            asm!("nop", options(nomem, nostack))
        }
    }

    #[inline(always)]
    unsafe fn nop() {
        asm!("nop", options(nomem, nostack))
    }

    #[inline(always)]
    unsafe fn wfe() {
        asm!("wfe", options(nomem, nostack));
    }

    #[inline(always)]
    unsafe fn sev() {
        asm!("sev", options(nomem, nostack));
    }

    #[inline(always)]
    unsafe fn cpu_id() -> usize {
        let id: usize;
        asm!(
            "mrc p15, 0, {reg}, c0, c0, 5",
            reg = out(reg) id,
            options(pure, nomem, nostack),
        );
        id & 0b11
    }

    #[inline(always)]
    unsafe fn sp_set(address: usize) {
        asm!(
            "mov sp, {reg}",
            reg = in(reg) address,
            options(nomem, nostack)
        );
    }
}

pub use Armv7A as Cpu;
