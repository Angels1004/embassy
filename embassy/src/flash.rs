use core::future::Future;
use core::pin::Pin;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Error {
    Failed,
    AddressMisaligned,
    BufferMisaligned,
}

pub trait Flash {
    type ReadFuture<'a>: Future<Output = Result<(), Error>>;
    type WriteFuture<'a>: Future<Output = Result<(), Error>>;
    type ErasePageFuture<'a>: Future<Output = Result<(), Error>>;

    /// Reads data from the flash device.
    ///
    /// address must be a multiple of self.read_size().
    /// buf.len() must be a multiple of self.read_size().
    fn read<'a>(self: Pin<&'a mut Self>, address: usize, buf: &'a mut [u8])
        -> Self::ReadFuture<'a>;

    /// Writes data to the flash device.
    ///
    /// address must be a multiple of self.write_size().
    /// buf.len() must be a multiple of self.write_size().
    fn write<'a>(self: Pin<&'a mut Self>, address: usize, buf: &'a [u8]) -> Self::WriteFuture<'a>;

    /// Erases a single page from the flash device.
    ///
    /// address must be a multiple of self.erase_size().
    fn erase<'a>(self: Pin<&'a mut Self>, address: usize) -> Self::ErasePageFuture<'a>;

    /// Returns the total size, in bytes.
    /// This is not guaranteed to be a power of 2.
    fn size(&self) -> usize;

    /// Returns the read size in bytes.
    /// This is guaranteed to be a power of 2.
    fn read_size(&self) -> usize;

    /// Returns the write size in bytes.
    /// This is guaranteed to be a power of 2.
    fn write_size(&self) -> usize;

    /// Returns the erase size in bytes.
    /// This is guaranteed to be a power of 2.
    fn erase_size(&self) -> usize;
}
