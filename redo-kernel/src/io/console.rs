use redo_platform as platform;
use core::fmt;

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use redo_bare::interface::DirectWrite;

    platform::console::console()
        .write_fmt(args).unwrap();
}

/// Prints without a newline.
///
/// Carbon copy from https://doc.rust-lang.org/src/std/macros.rs.html
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::_print(format_args!($($arg)*)));
}

/// Prints with a newline.
///
/// Carbon copy from https://doc.rust-lang.org/src/std/macros.rs.html
#[macro_export]
macro_rules! println {
    () => ($crate::console!("\n"));
    ($($arg:tt)*) => ({
        $crate::io::console::_print(format_args_nl!($($arg)*));
    })
}

/// Prints an info, with a newline.
#[macro_export]
macro_rules! info {
    ($string:expr) => ({
        #[allow(unused_imports)]
        use redo_bare::time::LoggerTimerCount;

        let hi = redo_platform::time::bare_log_time_manager().uptime_hi();
        let lo = redo_platform::time::bare_log_time_manager().uptime_lo();

        $crate::io::console::_print(format_args_nl!(
            concat!("[I {:>8X}'{:08X}] ", $string),
            hi, lo
        ));
    });
    ($format_string:expr, $($arg:tt)*) => ({
        #[allow(unused_imports)]
        use crate::time::interface::LoggerTimerCount;

        let hi = $crate::time::bare_log_time_manager().uptime_hi();
        let lo = $crate::time::bare_log_time_manager().uptime_lo();

        $crate::console::_print(format_args_nl!(
            concat!("[I {:>8X}'{:08X}] ", $format_string),
            hi, lo, $($arg)*
        ));
    })
}

// /// Prints a warning, with a newline.
#[macro_export]
macro_rules! warn {
    ($string:expr) => ({
        #[allow(unused_imports)]
        use crate::time::interface::LoggerTimerCount;

        let hi = $crate::time::bare_log_time_manager().uptime_hi();
        let lo = $crate::time::bare_log_time_manager().uptime_lo();

        $crate::console::_print(format_args_nl!(
            concat!("[W {:>8X}'{:08X}] ", $string),
            hi, lo
        ));
    });
    ($format_string:expr, $($arg:tt)*) => ({
        #[allow(unused_imports)]
        use crate::time::interface::LoggerTimerCount;

        let hi = $crate::time::bare_log_time_manager().uptime_hi();
        let lo = $crate::time::bare_log_time_manager().uptime_lo();

        $crate::console::_print(format_args_nl!(
            concat!("[W {:>8X}'{:08X}] ", $format_string),
            hi, lo, $($arg)*
        ));
    })
}
