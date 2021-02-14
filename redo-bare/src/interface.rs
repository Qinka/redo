
use core::fmt;

pub trait DirectWrite {
    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
}
