use anyhow::{Result, bail};

pub struct ByteReader<'a> {
    cursor: usize,
    buffer: &'a [u8],
}

impl<'a> ByteReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            cursor: 0,
            buffer: bytes,
        }
    }

    pub fn read_c_str(&mut self) -> Result<String> {
        // We reading until null terminated or the end of buffer, it doesn't matter the length
        self.has_space(1)?;
        let len = self.buffer[self.cursor..]
            .iter()
            .position(|&b| b == b'\0')
            .unwrap_or(self.buffer.len());

        let s = String::from_utf8_lossy(&self.buffer[self.cursor..self.cursor + len]).into_owned();
        self.cursor = self.cursor + len + 1;
        Ok(s)
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        let size = size_of::<u8>();
        self.has_space(size)?;
        let n = self.buffer[self.cursor];
        self.cursor += size;
        Ok(n)
    }

    pub fn read_u16_be(&mut self) -> Result<u16> {
        let size = size_of::<u16>();
        self.has_space(size)?;
        let bytes = &self.buffer[self.cursor..self.cursor + size];
        let n = u16::from_be_bytes([bytes[0], bytes[1]]);
        self.cursor += size;
        Ok(n)
    }

    pub fn read_u16_le(&mut self) -> Result<u16> {
        let size = size_of::<u16>();
        self.has_space(size)?;
        let bytes = &self.buffer[self.cursor..self.cursor + size];
        let n = u16::from_le_bytes([bytes[0], bytes[1]]);
        self.cursor += size;
        Ok(n)
    }

    pub fn read_u32_be(&mut self) -> Result<u32> {
        let size = size_of::<u32>();
        self.has_space(size)?;
        let bytes = &self.buffer[self.cursor..self.cursor + size];
        let n = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        self.cursor += size;
        Ok(n)
    }

    pub fn read_u32_le(&mut self) -> Result<u32> {
        let size = size_of::<u32>();
        self.has_space(size)?;
        let bytes = &self.buffer[self.cursor..self.cursor + size];
        let n = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        self.cursor += size;
        Ok(n)
    }

    pub fn read_u64_be(&mut self) -> Result<u64> {
        let size = size_of::<u64>();
        self.has_space(size)?;
        let bytes = &self.buffer[self.cursor..self.cursor + size];
        let n = u64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]);
        self.cursor += size;
        Ok(n)
    }

    pub fn read_u64_le(&mut self) -> Result<u64> {
        let size = size_of::<u64>();
        self.has_space(size)?;
        let bytes = &self.buffer[self.cursor..self.cursor + size];
        let n = u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]);
        self.cursor += size;
        Ok(n)
    }

    pub fn advance(&mut self, n: usize) {
        self.cursor += n;
    }

    pub fn align(&mut self, n: usize) {
        let offset_before_align = self.cursor;
        let remain = offset_before_align % n;
        let offset_after_align = match remain {
            0 => offset_before_align,
            _ => offset_before_align - remain + n,
        };
        self.cursor = offset_after_align;
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn read_block(&mut self, n: usize) -> Result<&[u8]> {
        self.has_space(n)?;
        let v = &self.buffer[self.cursor..self.cursor + n];
        self.cursor += n;
        Ok(v)
    }

    /// Does not advance the cursor
    pub fn get_block_at(&self, position: usize, length: usize) -> Result<&[u8]> {
        if position + length > self.buffer.len() {
            bail!(
                "Position: {position}, and Length: {length}, exceeds the buffer size {}",
                self.buffer.len()
            );
        }

        Ok(&self.buffer[position..position + length])
    }

    pub fn rewind(&mut self, n: usize) -> Result<()> {
        if n < self.cursor {
            bail!(
                "Rewinding by n: {n}, will put the cursor position in negative value. If you want to reset cursor position use `reader.reset()` instead"
            );
        }

        self.cursor -= n;
        Ok(())
    }

    pub fn reset(&mut self) {
        self.cursor = 0;
    }

    fn has_space(&self, length: usize) -> Result<()> {
        if self.cursor + length > self.buffer.len() {
            bail!(
                "ByteReader has reached the end! cannot read anymore bytes, consider rewinding if you want to re-read some bytes"
            );
        }

        Ok(())
    }
}
