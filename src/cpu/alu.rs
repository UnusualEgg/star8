use std::fmt::Display;

#[derive(PartialEq, Eq)]
enum ALUSate {
    Add,
    Sub,
    Inc,
}
pub struct ALU {
    a: u8,
    b: u8,
    state: ALUSate,
    value: u8,
    zero: bool,
    overflow: bool,
    negative: bool,
}
impl Display for ALU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.a))?;
        match self.state {
            ALUSate::Add => f.write_fmt(format_args!(" + {}", self.b))?,
            ALUSate::Sub => f.write_fmt(format_args!(" - {}", self.b))?,
            ALUSate::Inc => f.write_fmt(format_args!(" ++ [{}]", self.b))?,
        }
        f.write_fmt(format_args!(" = {}", self.value))?;
        if self.zero {
            f.write_str(" zero")?;
        }
        if self.overflow {
            f.write_str(" overflow")?;
        }
        if self.negative {
            f.write_str(" negative")?;
        }
        Ok(())
    }
}
impl ALU {
    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn zero(&self) -> bool {
        self.zero
    }

    pub fn overflow(&self) -> bool {
        self.overflow
    }

    pub fn negative(&self) -> bool {
        self.negative
    }
    pub const fn new() -> Self {
        ALU {
            a: 0,
            b: 0,
            state: ALUSate::Inc,
            value: 0,
            zero: false,
            overflow: false,
            negative: false,
        }
    }
    pub fn set_a(&mut self, a: u8) {
        self.a = a;
    }
    pub fn set_b(&mut self, b: u8) {
        self.b = b;
    }
    pub fn set_add(&mut self) {
        self.state = ALUSate::Add;
    }
    pub fn set_sub(&mut self) {
        self.state = ALUSate::Sub;
    }
    pub fn set_inc(&mut self) {
        self.state = ALUSate::Inc;
    }
    pub fn compute(&mut self) {
        match self.state {
            ALUSate::Inc => {
                let result = self.a.wrapping_add(1);
                self.value = result;
                self.overflow = false;
                self.zero = result == 0;
                self.negative = false;
            }
            ALUSate::Add => {
                let (result, overflow) = self.a.overflowing_add(self.b);
                self.value = result;
                self.overflow = overflow;
                self.zero = result == 0;
                self.negative = false;
            }
            ALUSate::Sub => {
                let result = self.a.wrapping_sub(self.b);
                let negative = (result & 0x80) == 0x80;
                self.value = result;
                self.overflow = false;
                self.zero = result == 0;
                self.negative = negative;
            }
        }
    }
}
