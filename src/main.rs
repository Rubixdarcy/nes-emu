
use opcodes::AddressMode;

mod ines;
mod opcodes;

fn main() {

    let memory = ines::read_rom("rom/Super Mario Bros.nes").unwrap();
    
    dbg!(memory.prg.len(), memory.chr.len());

    show_program(&memory.prg);

}


fn show_program(mut program: &[u8]) {
    for _ in 0..50 {
        let code = program[0];
        let opcode = opcodes::OPCODES[code as usize];

        program = &program[1..];

        let addr_mode_asm = opcode.mode.get_asm(program);

        program = &program[opcode.mode.operand_size()..];

        println!("{:?} {}  # {}", opcode.instruction, addr_mode_asm, opcode.instruction.description());
    }

}


#[derive(Debug, Default)]
struct CPU {
    pub pc: u16,
    pub sp: u8,
    pub acc: u8,
    pub flags: Flags,
    pub x: u8,
    pub y: u8,
}

impl CPU {
    pub fn new(bus: impl Bus) -> Self {
        Self::default().reset(bus)
    }

    pub fn reset(self, bus: impl Bus) -> Self {
        let sp = self.sp.wrapping_sub(3);
        let pc = u16::from_le_bytes([bus.read(0xFFFC), bus.read(0xFFFD)]);

        Self {
            pc,
            sp,
            flags: Flags {
                interrupt_disable: true,
                ..self.flags
            },
            ..self
        }
    }
}

fn read_inst_data(cpu: CPU, bus: impl Bus, mode: AddressMode) -> u8 {
    match mode {
        AddressMode::Accumulator => cpu.acc,
        AddressMode::Absolute => todo!(),
        AddressMode::AbsoluteX => todo!(),
        AddressMode::AbsoluteY => todo!(),
        AddressMode::Immediate => todo!(),
        AddressMode::Implied => todo!(),
        AddressMode::Indirect => todo!(),
        AddressMode::XIndirect => todo!(),
        AddressMode::IndirectY => todo!(),
        AddressMode::Relative => todo!(),
        AddressMode::Zeropage => todo!(),
        AddressMode::ZeropageX => todo!(),
        AddressMode::ZeropageY => todo!(),
    }
}

#[derive(Debug, Default)]
pub struct Flags {
    pub negative: bool,
    pub overflow: bool,
    pub break_: bool,
    pub decimal: bool,
    pub interrupt_disable: bool,
    pub zero: bool,
    pub carry: bool,
}

struct Memory {
    pub prg: Vec<u8>,
    pub chr: Vec<u8>,
    pub ram: Vec<u8>
}

impl Bus for Memory {
    fn read(&self, index: u16) -> u8 {
         index as u8
    }

    fn write(&mut self, index: u16, value: u8) {
        println!("{}{}", index, value);
    }
}



trait Bus {
    fn read(&self, index: u16) -> u8;

    fn write(&mut self, index: u16, value: u8);
}
