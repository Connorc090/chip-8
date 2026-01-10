mod cpu;
use minifb::{self, WindowOptions};

use crate::cpu::Chip8;

//16 keypad keys
const KEYS: [minifb::Key; 16] = [
    minifb::Key::X, // 0
    minifb::Key::Key1, // 1
    minifb::Key::Key2, // 2
    minifb::Key::Key3, // 3
    minifb::Key::Q,    // 4
    minifb::Key::W,    // 5
    minifb::Key::E,    // 6
    minifb::Key::A,    // 7
    minifb::Key::S,    // 8
    minifb::Key::D,    // 9
    minifb::Key::Z,    // A
    minifb::Key::C,    // B
    minifb::Key::Key4, // C
    minifb::Key::R,    // D
    minifb::Key::F,    // E
    minifb::Key::V     // F
];

//Creates CPU object, loads assets and runs cycle
fn main() {
    let mut this_cpu = cpu::Chip8::new();

    this_cpu.load_font();
    this_cpu.load_rom("assets/roms/Brick (Brix hack, 1990).ch8");

    cycle(&mut this_cpu);
}

//Creates the window and calls tick at -1kHz with a 60HZ display
fn cycle(this_cpu: &mut Chip8) {
    let mut window = minifb::Window::new( 
        "Chip-8 Emulator",
        64, 32,
        WindowOptions {
            scale: minifb::Scale::X16,
            ..WindowOptions::default()
        }
    ).unwrap();

    window.set_target_fps(60);

    let mut buffer: Vec<u32> = vec![0; 64 * 32];

    while window.is_open() {

        for (i, key) in KEYS.iter().enumerate() {
            this_cpu.keypad[i] = window.is_key_down(*key);
        }

        for _ in 0..16 {
            this_cpu.tick();
        }

        this_cpu.update_timers();

        for i in 0..2048 {
            buffer[i] = if this_cpu.display[i] {0xFFFFFF} else {0};
        }

        window.update_with_buffer(&buffer, 64, 32).unwrap();
    }
}