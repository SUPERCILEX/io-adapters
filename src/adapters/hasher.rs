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
/// io::copy(&mut io::repeat(42).take(10), &mut hasher.write_adapter()).unwrap();
///
/// assert_eq!(hasher.finish(), 2882615036743451676);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct IoToHasher<H>(H);

impl<H: Hasher> IoToHasher<H> {
    pub fn new(inner: H) -> Self {
        Self(inner)
    }
}

impl<H: Hasher> io::Write for IoToHasher<&mut H> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<H: Hasher> WriteExtension<IoToHasher<H>> for H {
    type Adapter<'a> = IoToHasher<&'a mut H> where H: 'a;

    fn write_adapter(&mut self) -> IoToHasher<&mut H> {
        IoToHasher(self)
    }
}
