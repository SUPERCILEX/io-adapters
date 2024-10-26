use std::{fmt, io};

use crate::WriteExtension;

/// Adapter that enables writing through a [`fmt::Write`] to an underlying
/// [`io::Write`].
///
/// # Examples
///
/// ```rust
/// # use std::{fmt, str};
/// use io_adapters::WriteExtension;
///
/// let mut output1 = String::new();
/// let mut output2 = [0u8; 13]; // Or io::stdout() for example
///
/// my_common_writer(&mut output1).unwrap();
/// my_common_writer(&mut output2.as_mut_slice().write_adapter()).unwrap();
///
/// fn my_common_writer(mut output: impl fmt::Write) -> fmt::Result {
///     write!(output, "Hello, World!")
/// }
///
/// assert_eq!(&output1, "Hello, World!");
/// assert_eq!(str::from_utf8(&output2).unwrap(), "Hello, World!");
/// ```
///
/// ## Error handling
///
/// Error handling is unpleasant with this adapter due to the [`fmt::Write`]
/// interface returning a stateless [`fmt::Error`] rather than an [`io::Error`].
/// Here is the suggested usage when unwrap doesn't cut it:
///
/// ```
/// # use std::{fmt, io, str};
/// # use io_adapters::WriteExtension;
///
/// let mut adapter = io::stdout().write_adapter();
/// match (conv(&mut adapter), adapter.error) {
///   (Ok(()), None) => {}
///   (Ok(()), Some(_)) => unreachable!(),
///   (Err(fmt::Error), None) => { /* Handle format error. */ }
///   (Err(fmt::Error), Some(e)) => { /* Handle I/O error. */ }
/// }
///
/// fn conv(output: impl fmt::Write) -> fmt::Result
/// # { Ok(()) }
/// ```
#[derive(Debug)]
pub struct FmtToIo<W> {
    inner: W,
    pub error: Option<io::Error>,
}

impl<W: io::Write> fmt::Write for FmtToIo<W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match self.inner.write_all(s.as_bytes()) {
            Ok(()) => {
                self.error = None;
                Ok(())
            }
            Err(e) => {
                self.error = Some(e);
                Err(fmt::Error)
            }
        }
    }
}

impl<W: io::Write> WriteExtension<FmtToIo<W>> for W {
    type Adapter = FmtToIo<W>;

    fn write_adapter(self) -> FmtToIo<W> {
        FmtToIo {
            inner: self,
            error: None,
        }
    }
}
