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
/// fn my_common_writer(output: &mut impl fmt::Write) -> fmt::Result {
///     write!(output, "Hello, World!")
/// }
///
/// assert_eq!(&output1, "Hello, World!");
/// assert_eq!(str::from_utf8(&output2).unwrap(), "Hello, World!");
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
    type Adapter<'a> = FmtToIo<&'a mut W> where W: 'a;

    fn write_adapter(&mut self) -> FmtToIo<&mut W> {
        FmtToIo {
            inner: self,
            error: None,
        }
    }
}
