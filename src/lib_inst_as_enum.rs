pub struct Cpu {
    pc: usize,
    reg: [u8; 16],
    ram: Vec<u8>
}

enum AddressMode {
    Immediate,

}

enum Instruction {
    LDA(address_mode: AddressMode, operand: u16)
}

impl Instruction {
    fn cycles(&self) -> u8 {
        match (self) {
            LDA(address_mode, operand) => {

            }
        }
    }

    fn exec_lda(address_mode: AddressMode, operand: u8) {

    }
}

impl Cpu {
    pub fn new() -> Cpu {
        let pc = 0;
        let reg = [0; 16];

        Cpu {
            pc,
            reg
        }
    }

    pub fn decode(a: u8, b: u8, c: u8) -> Instruction {

    }

    // TODO: Return Result
    
}