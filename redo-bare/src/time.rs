







use core::time::Duration;

/// For logger timer count.
pub trait LoggerTimerCount {
    /// Timer Ccount higher bits
    /// Timer count
    fn uptime_hi(&self) -> u32;
    fn uptime_lo(&self) -> u32;
}