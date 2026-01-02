use std::io;

/// An extension on top of [io::Read] that reads data with length delimiters
/// that was written using
/// [LengthDelimitedWriteExt](super::LengthDelimitedWriteExt). This is
/// particularly useful for adding extra data before or after data structures
/// written by FSW games.
///
/// The specific format used by this trait is a little-endian four-byte header
/// indicating the length of the data in bytes, followed by the data itself.
pub trait LengthDelimitedReadExt {
    /// Reads a length-delimited amount of data from the underlying reader into
    /// a newly-allocated [Vec].
    ///
    /// This is the dual of [LengthDelimitedWriteExt.write_delimited].
    ///
    /// Unlike [io::Read.read], this will return an error result if there's not
    /// enough data available to read the full result.
    fn read_delimited(&mut self) -> io::Result<Vec<u8>>;

    /// Reads a length-delimited amount of data from the underlying reader into
    /// a newly-allocated [String].
    ///
    /// This is the dual of [LengthDelimitedWriteExt.write_str_delimited].
    ///
    /// Unlike [io::Read.read], this will return an error result if there's not
    /// enough data available to read the full result. This will also return an
    /// error if the data isn't valid UTF-8.
    fn read_str_delimited(&mut self) -> io::Result<String> {
        String::from_utf8(self.read_delimited()?).map_err(io::Error::other)
    }
}

impl<T: ?Sized + io::Read> LengthDelimitedReadExt for T {
    fn read_delimited(&mut self) -> io::Result<Vec<u8>> {
        let mut delimiter = [0; std::mem::size_of::<u32>()];
        if self.read(&mut delimiter)? < delimiter.len() {
            return Err(io::Error::other("Couldn't read delimiter"));
        }

        let delimiter: usize = u32::from_le_bytes(delimiter)
            .try_into()
            .map_err(io::Error::other)?;

        // Don't allow a delimiter to force us to allocate massive amounts of
        // memory. Read at most 16kb at a time and only continue if the stream
        // actually has that amount of data available.
        const CHUNK_SIZE: usize = 0x4000;
        let mut result = Vec::with_capacity(std::cmp::min(delimiter, CHUNK_SIZE));
        while result.len() < delimiter {
            let prev_len = result.len();
            result.resize(std::cmp::min(delimiter, prev_len + CHUNK_SIZE), 0);
            let size = self.read(&mut result[prev_len..])?;

            let expected = result.len() - prev_len;
            if size < expected {
                return Err(io::Error::other(format!(
                    "Expected {} bytes but only {} were read",
                    expected, size
                )));
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::LengthDelimitedReadExt;
    use crate::ext::LengthDelimitedWriteExt;

    #[test]
    fn write_and_read_small_bytes() {
        let mut writer = Vec::new();
        assert_eq!(writer.write_delimited(&[1, 2, 3, 4, 5]).unwrap(), 9);

        let mut reader = &writer[..];
        assert_eq!(reader.read_delimited().unwrap(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn write_and_read_small_string() {
        let mut writer = Vec::new();
        assert_eq!(writer.write_str_delimited("hello!").unwrap(), 10);

        let mut reader = &writer[..];
        assert_eq!(reader.read_str_delimited().unwrap(), "hello!");
    }

    #[test]
    fn write_and_read_large_bytes() {
        let mut data = [0; 0x5000];
        for (i, byte) in data.iter_mut().enumerate() {
            *byte = (i % 255).try_into().unwrap();
        }

        let mut writer = Vec::new();
        assert_eq!(writer.write_delimited(&data[..]).unwrap(), 0x5004);

        let mut reader = &writer[..];
        assert_eq!(reader.read_delimited().unwrap(), &data);
    }
}
