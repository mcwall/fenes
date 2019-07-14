use super::cpu;

pub struct Emulator {
    cpu: cpu::Cpu,
}

impl Emulator {
    pub fn new() -> Emulator {
        let cpu = cpu::Cpu::new();

        Emulator {
            cpu,
        }
    }

    // pub fn load_rom(&mut self, rom: Vec<u8>) {
    //     self.load_fonts();

    //     for (i, rom_byte) in rom.iter().enumerate() {
    //         self.ram[PRG_OFFSET + i] = *rom_byte;
    //     }
    // }
}