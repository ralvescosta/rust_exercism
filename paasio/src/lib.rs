use std::{
    fs::File,
    io::{self, Read, Result, Write},
    usize,
};

pub struct ReadStats<R> {
    data: R,
    bytes: usize,
    ops: usize,
}
impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            data: wrapped,
            bytes: 0,
            ops: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        return &self.data;
    }

    pub fn bytes_through(&self) -> usize {
        return self.bytes;
    }

    pub fn reads(&self) -> usize {
        return self.ops;
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let data = self.data.read(buf)?;
        self.ops += 1;
        self.bytes += data;
        Ok(data)
    }
}

pub struct WriteStats<W> {
    data: W,
    bytes: usize,
    ops: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            data: wrapped,
            bytes: 0,
            ops: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        return &self.data;
    }

    pub fn bytes_through(&self) -> usize {
        return self.bytes;
    }

    pub fn writes(&self) -> usize {
        return self.ops;
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let data = self.data.write(buf)?;
        self.ops += 1;
        self.bytes += data;
        Ok(data)
    }

    fn flush(&mut self) -> Result<()> {
        self.data.flush()?;
        self.ops += 1;
        Ok(())
    }
}
