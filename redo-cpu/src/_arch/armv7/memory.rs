// Redo Kernel
// Memory related
// Copyright (c) 2021 Johann Li <me@qinka.pro>

use core::{cell::UnsafeCell, ops::RangeInclusive};

// Symbols from the linker script.
extern "Rust" {
    static __bss_start: UnsafeCell<u32>;
    static __bss_end: UnsafeCell<u32>;
}

/// Return the inclusive range spanning the .bss section.
///
/// # Safety
///
/// - Values are provided by the linker script and must be trusted as-is.
/// - The linker-provided addresses must be u64 aligned.
pub fn bss_range_inclusive() -> RangeInclusive<*mut u32> {
    let range;
    unsafe {
        range = RangeInclusive::new(__bss_start.get(), __bss_end.get());
    }
    assert!(!range.is_empty());

    range
}

