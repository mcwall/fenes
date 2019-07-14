use std::{thread, time, fs, env};
use fenes::cpu;

fn main() {
    let args: Vec<String> = env::args().collect();
    run_emulator(&args[1]);
}

fn run_emulator(file_name: &str){
    let mut cpu = cpu::Cpu::new();

    // let mut file = File::open(file_name).expect("Failed to read file");
    let rom_bytes = match fs::read(file_name) {
        Err(e) => panic!("ROM load error {}: {}", file_name, e),
        Ok(file) => file
    };

    cpu.load_rom(rom_bytes);

    // loop {
    //     emulator.tick_frame();
    //     clear_screen();
    //     print!("{}", emulator.display_out());

    //     thread::sleep(time::Duration::from_millis(16));
    // }
}
