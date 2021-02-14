#[rustfmt::skip]

pub(super) mod map {
    pub const BOOT_CORE_STACK_END: usize = 0x8000;
}

#[inline(always)]
pub fn boot_core_stack_end() -> usize {
    map::BOOT_CORE_STACK_END
}