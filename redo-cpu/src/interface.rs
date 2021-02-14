
pub trait CPU {
    // #[inline(always)]
    unsafe fn wfe();
    // #[inline(always)]
    unsafe fn nop();
    // #[inline(always)]
    unsafe fn sev();
    // #[inline(always)]
    unsafe fn cpu_id() -> usize;
    // #[inline(always)]
    unsafe fn sp_set(address: usize);
    // #[inline(always)]
    unsafe fn delay(count:u32);
}
