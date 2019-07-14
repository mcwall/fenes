pub const RAM_SIZE: usize = 0x1000;

pub struct Cpu {
    pc: usize,
    reg: [u8; 16],
    ram: Vec<u8>,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
}

enum RomFormat {
    INES,
    NES2,
    Unknown
}

enum AddressMode {
    Immediate,

}

enum Instruction {

}

struct Execution {
    inst: Option<Instruction>,
    cycles: usize
}

impl Cpu {
    pub fn new() -> Cpu {
        let pc = 0;
        let reg = [0; 16];
        let ram = vec![0; RAM_SIZE];
        let prg_rom = vec![0; 0];
        let chr_rom = vec![0; 0];

        Cpu {
            pc,
            reg,
            ram,
            prg_rom,
            chr_rom
        }
    }

    pub fn tick() {
        
    }
    
    fn decode(a: u8, b: u8, c: u8) -> u8 {
        0
    }
    
    fn exec_lda(address_mode: AddressMode, operand: u8) {

    }

    /// Very primitive ROM verification and loading
    /// Currently only supports iNES format
    /// Only loads PRG_ROM and CHR_ROM, ignores mapper, trainer, battery, etc
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        if rom.len() < 16 {
            panic!("Unsupported ROM");
        }

        let header = &rom[0..16];
        let header_check = header[0] == 0x4E  // N
                        && header[1] == 0x45  // E
                        && header[2] == 0x53  // S
                        && header[3] == 0x1A; // <EOF>
        if !header_check {
            panic!("Unsupported ROM");
        }

        // NES 2.0
        if header[7] & 0x0C == 0x08 {
            panic!("Unsupported NES 2.0 ROM");
            return;
        }

        // iNES
        if header[7] & 0x0C == 0x00 {
            let prg_size = header[4] as usize * 0x4000;
            let chr_size = header[5] as usize * 0x2000;

            let prg_start = 16;
            let prg_end = prg_start + prg_size;
            let chr_end = prg_end + chr_size;

            self.prg_rom = rom[prg_start..prg_end].to_vec();
            self.chr_rom = rom[prg_end..chr_end].to_vec();
            return;
        }

        panic!("Unsupported ROM");
    }

    // fn load_rom_header(&self, rom: Vec<u8>) -> RomFormat {
        // if rom.len() < 16 {
        //     return RomFormat::Unknown;
        // }

        // let header = &rom[0..16];
        // let nes: String = header[0..3].iter().map(|&b| { b as char }).collect();
        // if nes != "NES" {
        //     return RomFormat::Unknown;
        // }

        // return RomFormat::INES;
    // }
}