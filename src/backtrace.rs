#[cfg(backtrace)]
pub(crate) use std::backtrace::Backtrace;

#[cfg(not(backtrace))]
pub(crate) enum Backtrace {}

#[cfg(backtrace)]
macro_rules! backtrace {
    () => {
        Some(Backtrace::capture())
    };
}

#[cfg(not(backtrace))]
macro_rules! backtrace {
    () => {
        None
    };
}
