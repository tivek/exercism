use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    reader: R,
    bytes_read: usize,
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            reader: wrapped,
            bytes_read: 0,
            reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self.reader.read(buf) {
            Ok(bytes_read) => {
                self.bytes_read += bytes_read;
                self.reads += 1;
                Ok(bytes_read)
            }
            Err(err) => Err(err),
        }
    }
}

pub struct WriteStats<W> {
    writer: W,
    bytes_written: usize,
    writes: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            writer: wrapped,
            bytes_written: 0,
            writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_written
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self.writer.write(buf) {
            Ok(bytes_written) => {
                self.bytes_written += bytes_written;
                self.writes += 1;
                Ok(bytes_written)
            }
            Err(err) => Err(err),
        }
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
