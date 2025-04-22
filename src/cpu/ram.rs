use crate::MemIO;

pub const RAM: usize = 64;
type RamInner = [u8; RAM];
pub struct Ram(RamInner);
impl Ram {
    pub fn zero() -> Self {
        Ram([0; RAM])
    }
    pub fn preset(data: RamInner) -> Self {
        Ram(data)
    }
}
impl MemIO for Ram {
    fn range(&self) -> std::ops::RangeInclusive<u8> {
        0..=63
    }
    fn read(&mut self, addr: u8) -> u8 {
        self.0[addr as usize]
    }
    fn write(&mut self, addr: u8, value: u8) {
        self.0[addr as usize] = value;
    }
}
