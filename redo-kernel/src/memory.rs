// Redo Kernel
// Memory related
// Copyright (c) 2021 Johann Li <me@qinka.pro>

use core::ops::RangeInclusive;
use redo_cpu::memory::bss_range_inclusive;

#[inline(always)]
pub unsafe fn zero_bss() {
    zero_volatile(bss_range_inclusive());
}

unsafe fn zero_volatile<T>
    (range: RangeInclusive<*mut T>)
    where T: From<u8>,
{
    let mut ptr = *range.start();
    let end_inclusive = *range.end();

    while ptr <= end_inclusive {
        core::ptr::write_volatile(ptr, T::from(0));
        ptr = ptr.offset(1);
    }
}