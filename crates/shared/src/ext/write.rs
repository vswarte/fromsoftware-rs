use std::io;

/// An extension on top of [io::Write] that writes data with length delimiters
/// so that it can be read back using
/// [LengthDelimitedReadExt](super::LengthDelimitedReadExt). This is
/// particularly useful for adding extra data before or after data structures
/// written by FSW games.
///
/// The specific format used by this trait is a little-endian four-byte header
/// indicating the length of the data in bytes, followed by the data itself.
pub trait LengthDelimitedWriteExt {
    /// Writes `data` to the underlying writer with a length delimiter.
    ///
    /// This is the dual of [LengthDelimitedReadExt.read_delimited].
    fn write_delimited(&mut self, data: &[u8]) -> io::Result<usize>;

    /// Writes `text` to the underlying writer as UTF-8 with a length delimiter.
    ///
    /// This is the dual of [LengthDelimitedReadExt.read_str_delimited].
    fn write_str_delimited(&mut self, text: &str) -> io::Result<usize> {
        self.write_delimited(text.as_bytes())
    }
}

impl<T: ?Sized + io::Write> LengthDelimitedWriteExt for T {
    fn write_delimited(&mut self, data: &[u8]) -> io::Result<usize> {
        let delimiter = u32::try_from(data.len())
            .map_err(io::Error::other)?
            .to_le_bytes();

        let size = self.write(&delimiter)?;
        if size < delimiter.len() {
            Ok(size)
        } else {
            Ok(size + self.write(data)?)
        }
    }
}
