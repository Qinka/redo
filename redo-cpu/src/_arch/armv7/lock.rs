
// global_asm!(include_str!("lock.s"));

extern "C" {
    // pub fn asm_try_lock(lock: *mut usize) -> usize;
}

#[inline(always)]
// #[inline(never)]
// #[no_mangle]
pub unsafe fn asm_try_lock(lock: *mut usize) -> usize {
    let flag: usize;
    let used: usize = 0;
    asm!(
        // "mov r1, #0",
        "swp {o}, {u}, [{b}]",
        o = out(reg) flag,
        b = in(reg) lock,
        u = in(reg) used,
        options(nomem, nostack)
    );
    return flag;

    // mov         r1, #0 // r1 -> 0
    // stores the value in r1 into the address in r0, and stores the value at the address in r0 into r2
    // swp         r2, r1, [r0]
    // mov         r0, r2
    // blx         lr
}