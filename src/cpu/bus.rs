use std::ops::RangeInclusive;

pub trait MemIO {
    fn range(&self) -> RangeInclusive<u8>;
    fn read(&mut self, addr: u8) -> u8 {
        0
    }
    fn write(&mut self, addr: u8, value: u8) {}
}
