// auto-generated by: jiff-cli generate shared

macro_rules! err {
    ($($tt:tt)*) => {{
        crate::shared::util::error::Error::from_args(format_args!($($tt)*))
    }}
}

pub(crate) use err;

/// An error that can be returned when parsing.
#[derive(Clone, Debug)]
pub struct Error {
    message: alloc::boxed::Box<str>,
}

impl Error {
    pub(crate) fn from_args<'a>(message: core::fmt::Arguments<'a>) -> Error {
        {
            use alloc::string::ToString;

            let message = message.to_string().into_boxed_str();
            Error { message }
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.message, f)
    }
}
