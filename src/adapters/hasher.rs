use std::{hash::Hasher, io};

use crate::WriteExtension;

/// Adapter that enables writing through an [`io::Write`]r to an underlying
/// [`Hasher`].
///
/// # Examples
///
/// ```rust
/// # use std::{hash::{DefaultHasher, Hasher}, io, io::Read};
/// use io_adapters::WriteExtension;
///
/// let mut hasher = DefaultHasher::new();
///
/// io::copy(
///     &mut io::repeat(42).take(10),
///     &mut (&mut hasher).write_adapter(),
/// )
/// .unwrap();
///
/// assert_eq!(hasher.finish(), 2882615036743451676);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct IoToHasher<H>(H);

impl<H: Hasher> io::Write for IoToHasher<H> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<H: Hasher> WriteExtension<IoToHasher<H>> for H {
    type Adapter = IoToHasher<H>;

    fn write_adapter(self) -> IoToHasher<H> {
        IoToHasher(self)
    }
}
