use anyhow::bail;
pub struct Memory {
    ram: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self { ram: vec![0; size] }
    }

    pub fn guard_addr(&self, addr: usize) -> anyhow::Result<()> {
        if addr >= self.ram.len() {
            bail!("memory addr out of range:{}", addr)
        }
        Ok(())
    }

    pub fn slice(&self, addr: usize, len: usize) -> anyhow::Result<&[u8]> {
        self.guard_addr(addr + len - 1)?;
        Ok(&self.ram[addr..addr + len])
    }

    pub fn store_slice(&mut self, addr: usize, slice: &[u8]) -> anyhow::Result<()> {
        let sl_len = slice.len();
        self.guard_addr(addr + sl_len - 1)?;
        self.ram[addr..addr + sl_len].copy_from_slice(slice);
        Ok(())
    }

    pub fn load_u8(&self, addr: usize) -> anyhow::Result<u8> {
        self.guard_addr(addr)?;
        Ok(self.ram[addr])
    }

    pub fn store_u8(&mut self, addr: usize, byte: u8) -> anyhow::Result<()> {
        self.guard_addr(addr)?;
        self.ram[addr] = byte;
        Ok(())
    }

    pub fn load_u16(&self, addr: usize) -> anyhow::Result<u16> {
        Ok(u16::from_le_bytes(self.slice(addr, 2)?.try_into()?))
    }

    pub fn load_u32(&self, addr: usize) -> anyhow::Result<u32> {
        Ok(u32::from_le_bytes(self.slice(addr, 4)?.try_into()?))
    }

    pub fn load_u64(&self, addr: usize) -> anyhow::Result<u64> {
        Ok(u64::from_le_bytes(self.slice(addr, 8)?.try_into()?))
    }
}
