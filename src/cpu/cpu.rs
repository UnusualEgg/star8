use std::time::Instant;

use log::debug;

use super::{
    alu::ALU,
    bus::MemIO,
    ins::{double_reg, single_reg, Reg},
    pc::PC,
    ram::{Ram, RAM},
};

#[derive(Debug)]
enum State {
    Fetch,
    Execute,
}
pub struct Star8 {
    pub regs: [u8; 4],
    ram: Ram,
    memio: Vec<Box<dyn MemIO>>,
    pc: PC,
    alu: ALU,
    last_clock: Instant,
    clock_speed: u64, //ms
    state: State,
    ins_count: u8,
    ins: u8,
    first_tick: bool,
    bus: u8,
}
impl Star8 {
    pub fn new(memio: Vec<Box<dyn MemIO>>) -> Self {
        Self {
            regs: [0; 4],
            ram: Ram::zero(),
            memio,
            pc: PC::new(),
            alu: ALU::new(),
            last_clock: Instant::now(),
            clock_speed: 8,
            state: State::Fetch,
            ins_count: 0,
            ins: 0,
            first_tick: true,
            bus: 0,
        }
    }
    pub fn current_ins(&self) -> u8 {
        self.ins
    }
    pub fn set_ram(mut self, ram: Ram) -> Self {
        self.ram = ram;
        self
    }
    fn read(&mut self, addr: u8) -> u8 {
        if (0..RAM).contains(&(addr as usize)) {
            let result = self.ram.read(addr);
            self.bus = result;
            result
        } else {
            todo!("MEMIO")
        }
    }
    fn write(&mut self, addr: u8, value: u8) {
        if (0..RAM).contains(&(addr as usize)) {
            self.ram.write(addr, value);
        } else {
            for io in &mut self.memio {
                if io.range().contains(&addr) {
                    io.write(addr, value);
                    break;
                }
            }
        }
    }
    fn count_up(&mut self) {
        self.pc.inc();
    }
    fn set_state(&mut self, state: State) {
        self.first_tick = true;
        self.state = state;
    }
    fn next(&mut self) {
        self.set_state(State::Fetch);
    }
    fn set_tmp_addr(&mut self) {
        //technically reads directly from ram
        self.pc.set_tmp(self.bus);
    }
    fn read_mem(&mut self) {
        self.read(self.pc.get());
    }
    fn write_mem(&mut self, value: u8) {
        self.write(self.pc.get(), value);
    }
    fn write_reg(&mut self, reg: Reg) {
        self.regs[reg as usize] = self.bus;
    }
    fn read_reg(&mut self, reg: Reg) {
        //technically reads directly from acc
        self.bus = self.regs[reg as usize];
    }
    fn write_reg_single(&mut self) {
        if let Some(reg) = single_reg(self.ins) {
            self.regs[reg as usize] = self.bus;
        };
    }
    fn read_reg_single(&mut self) {
        self.bus = if let Some(reg) = single_reg(self.ins) {
            self.regs[reg as usize]
        } else {
            0
        };
    }
    fn write_reg1(&mut self) {
        self.regs[double_reg(self.ins).0 as usize] = self.bus;
    }
    fn read_reg1(&mut self) {
        self.bus = self.regs[double_reg(self.ins).0 as usize];
    }
    fn read_reg2(&mut self) {
        self.bus = self.regs[double_reg(self.ins).1 as usize];
    }
    fn save_ins(&mut self) {
        self.ins = self.bus;
    }
    fn compute(&mut self) {
        self.alu.compute();
        self.bus = self.alu.value();
    }
    fn set_a(&mut self) {
        self.alu.set_a(self.bus);
    }
    fn set_b(&mut self) {
        self.alu.set_b(self.bus);
    }
    pub fn tick(&mut self) {
        if !self.first_tick {
            self.ins_count += 1;
        } else {
            self.ins_count = 0;
            self.first_tick = false;
        }
        let op: u8 = self.ins >> 4;
        let arg: u8 = self.ins & 0x0f;
        match self.state {
            State::Fetch => {
                match self.ins_count {
                    0 => self.read_mem(), //read=ture
                    //save
                    1 => self.save_ins(),
                    //read=false
                    2 => self.bus = 0,
                    3 => self.count_up(),
                    4 => self.set_state(State::Execute),
                    _ => unreachable!(),
                }
            }
            State::Execute => match op {
                //nop
                0b0000 => match self.ins_count {
                    0 => self.next(),
                    _ => unreachable!(),
                },
                //inc
                0b0001 => match self.ins_count {
                    //read reg
                    0 => self.read_reg_single(),
                    //set alu a
                    1 => self.set_a(),
                    //read reg = false
                    2 => self.bus = 0,
                    3 => self.alu.set_inc(),
                    //alu compute = true
                    4 => self.compute(),
                    5 => self.write_reg_single(),
                    //alue compute = false
                    6 => self.bus = 0,
                    7 => self.next(),
                    _ => unreachable!(),
                },
                //computation (add or sub)
                0b0010 | 0b0011 => match self.ins_count {
                    //read reg1
                    0 => self.read_reg1(),
                    1 => self.set_a(),
                    //read reg1=falae
                    2 => self.bus = 0,
                    //nop?
                    3 => (),
                    //read reg2
                    4 => self.read_reg2(),
                    5 => self.set_b(),
                    //read reg2=false
                    6 => self.bus = 0,
                    //set add or sub
                    7 => {
                        if op & 1 == 1 {
                            self.alu.set_sub();
                        } else {
                            self.alu.set_add();
                        }
                    }
                    //alu compute
                    8 => self.compute(),
                    9 => self.write_reg1(),
                    //alu compute= false
                    10 => self.bus = 0,
                    11 => self.next(),
                    _ => unreachable!(),
                },
                //load immidiate
                0b0100 => match self.ins_count {
                    //read mem
                    0 => self.read_mem(),
                    //set reg
                    1 => self.write_reg_single(),
                    //read mem==false
                    2 => self.bus = 0,
                    3 => self.count_up(),
                    4 => self.next(),
                    _ => unreachable!(),
                },
                //store reg to mem
                0b0101 => match self.ins_count {
                    0 => self.read_mem(),
                    //set tmp addr
                    1 => self.set_tmp_addr(),
                    //read mem=false
                    2 => self.bus = 0,
                    3 => self.pc.read_tmp(true),
                    4 => self.read_reg_single(),
                    5 => self.write_mem(self.bus),
                    6 => self.bus = 0,
                    7 => self.pc.read_tmp(false),
                    8 => self.count_up(),
                    9 => self.next(),
                    _ => unreachable!(),
                },
                //jmp
                0b0110 => match self.ins_count {
                    0 => self.read_mem(),
                    1 => self.pc.load(self.bus),
                    2 => self.bus = 0,
                    3 => self.next(),
                    _ => unreachable!(),
                },
                //jmp if (jif)
                0b0111 => match arg {
                    //A==0 or A==B
                    0 | 1 => match self.ins_count {
                        0 => self.bus = self.regs[0],
                        1 => self.set_a(),
                        2 => self.bus = 0,
                        3 => {
                            if arg == 1 {
                                self.bus = self.regs[1];
                            }
                        }
                        4 => self.set_b(),
                        5 => {
                            if arg == 1 {
                                self.bus = 0;
                            }
                        }
                        6 => self.alu.set_sub(),
                        7 => self.compute(),
                        //oddly blank
                        8 => (),
                        //alu compute = false
                        9 => self.bus = 0,
                        10 => {
                            if !self.alu.zero() {
                                self.count_up();
                            }
                        }
                        11 => {
                            if self.alu.zero() {
                                self.read_mem();
                            }
                        }
                        12 => {
                            if self.alu.zero() {
                                self.pc.load(self.bus);
                            }
                        }
                        13 => {
                            if self.alu.zero() {
                                //read=false
                                self.bus = 0;
                            }
                        }
                        14 => self.next(),
                        _ => unreachable!(),
                    },
                    //zero | ovf | neg
                    0b1000 | 0b0100 | 0b0010 => match self.ins_count {
                        0 | 1 | 2 | 3 => {
                            let result: bool = match self.ins & 0x0f {
                                0b0010 => self.alu.negative(),
                                0b0100 => self.alu.overflow(),
                                0b1000 => self.alu.zero(),
                                _ => unreachable!(),
                            };
                            if !result && self.ins_count == 0 {
                                self.count_up();
                            } else if result {
                                match self.ins_count {
                                    0 => (),
                                    1 => self.read_mem(),
                                    2 => self.pc.load(self.bus),
                                    3 => self.bus = 0,
                                    _ => unreachable!(),
                                };
                            }
                        }
                        4 => self.next(),
                        _ => unreachable!(),
                    },
                    //deadlock
                    _ => (),
                },
                //load from addr to reg
                //<reg> = *addr
                0b1001 => match self.ins_count {
                    0 => self.read_mem(),
                    1 => self.set_tmp_addr(),
                    2 => {
                        self.pc.read_tmp(true);
                        //in original impl, this is done auto
                        self.read_mem();
                    }
                    3 => self.write_reg_single(),
                    4 => {
                        self.bus = 0;
                        self.pc.read_tmp(false);
                        self.count_up();
                    }
                    5 => self.next(),
                    _ => unreachable!(),
                },

                //halt is 0b1000
                //but technically any uassigned insruction functions as such
                _ => (),
            },
        }
        debug!(
            "state: {:?} ins: {:02x} bus: {} pc:{:02x}({}) alu:|{}|",
            self.state,
            self.ins,
            self.bus,
            self.pc.get(),
            self.ins_count,
            self.alu
        );
    }
}
