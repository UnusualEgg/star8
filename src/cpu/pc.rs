pub struct PC {
    pc: u8,
    tmp: u8,
    use_pc: bool,
}
impl PC {
    pub fn new() -> Self {
        PC {
            pc: 0,
            tmp: 0,
            use_pc: true,
        }
    }
    pub fn inc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }
    pub fn get(&self) -> u8 {
        if self.use_pc {
            self.pc
        } else {
            self.tmp
        }
    }
    pub fn load(&mut self, addr: u8) {
        self.pc = addr;
    }
    pub fn set_tmp(&mut self, addr: u8) {
        self.tmp = addr;
    }
    pub fn read_tmp(&mut self, read: bool) {
        self.use_pc = !read;
    }
}
