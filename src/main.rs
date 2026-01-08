mod cpu;
use minifb::{self, WindowOptions};

use crate::cpu::Chip8;

//Creates CPU object, loads assets and runs cycle
fn main() {
    let mut this_cpu = cpu::Chip8::new();

    this_cpu.load_font();
    this_cpu.load_rom("assets/roms/ibm.ch8");

    cycle(&mut this_cpu);
}

//Creates the window and calls tick at -1kHz with a 60HZ display
fn cycle(this_cpu: &mut Chip8) {
    let mut window = minifb::Window::new( 
        "Chip-8 Emulator",
        64, 32,
        WindowOptions {
            scale: minifb::Scale::X8,
            ..WindowOptions::default()
        }
    ).unwrap();

    window.set_target_fps(60);

    let mut buffer: Vec<u32> = vec![0; 64 * 32];

    while window.is_open() {
        for _ in 0..16 {
            this_cpu.tick();
        }

        //this_cpu.update_timers() <- need to implement

        for i in 0..2048 {
            buffer[i] = if this_cpu.display[i] {0xFFFFFF} else {0};
        }

        window.update_with_buffer(&buffer, 64, 32).unwrap();
    }
}