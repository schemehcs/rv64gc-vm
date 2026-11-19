pub struct Register {
    file: [i64; 32],
}

impl Register {
    pub fn new() -> Self {
        Self { file: [0; 32] }
    }

    pub fn read(&self, x: usize) -> i64 {
        self.file[x]
    }

    pub fn read_pair(&self, x: usize, y: usize) -> (i64, i64) {
        (self.file[x], self.file[y])
    }

    pub fn read_pairu(&self, x: usize, y: usize) -> (u64, u64) {
        (self.file[x] as u64, self.file[y] as u64)
    }

    pub fn read_pairw(&self, x: usize, y: usize) -> (i32, i32) {
        (self.file[x] as i32, self.file[y] as i32)
    }

    pub fn read_pairwu(&self, x: usize, y: usize) -> (u32, u32) {
        (self.file[x] as u32, self.file[y] as u32)
    }

    pub fn write(&mut self, x: usize, val: i64) {
        if x == 0 {
            return;
        }
        self.file[x] = val;
    }

    pub fn read_u(&self, x: usize) -> u64 {
        self.file[x] as u64
    }

    pub fn write_u(&mut self, x: usize, val: u64) {
        self.write(x, val as i64)
    }

    pub fn read_w(&self, x: usize) -> i32 {
        self.file[x] as i32
    }

    pub fn write_w(&mut self, x: usize, val: i32) {
        self.write(x, val as i64);
    }

    pub fn write_wu(&mut self, x: usize, val: u32) {
        self.write(x, val as i32 as i64);
    }

    pub fn read_wu(&self, x: usize) -> u32 {
        self.file[x] as u32
    }
}
