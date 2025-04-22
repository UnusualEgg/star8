use cpu::{bus::MemIO, cpu::Star8, ram::Ram};
use log::debug;
mod cpu;

struct PrintOut;
impl MemIO for PrintOut {
    fn range(&self) -> std::ops::RangeInclusive<u8> {
        64..=64
    }
    fn write(&mut self, _addr: u8, value: u8) {
        println!("{value}");
    }
}

fn main() {
    env_logger::init();

    let mut total = [0u8; 64];
    let mut args = std::env::args().skip(1);
    let prog = std::fs::read(args.next().expect("File")).unwrap();
    //let prog = [0b0100_0001, 69, 0b1000_0000];
    for (i, v) in prog.iter().enumerate() {
        total[i] = *v;
    }

    let mut proc = Star8::new(vec![Box::new(PrintOut)]).set_ram(Ram::preset(total));
    let mut count = 0;
    while count < 100000 && proc.current_ins() >> 4 != 8 {
        proc.tick();
        count += 1;
    }
    debug!("took: {} regs: {:?}", count, proc.regs);
}
