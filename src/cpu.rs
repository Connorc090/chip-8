use std::fs::File;
use std::io::Read;

//Default font for Chip-8 *TODO* move to a file and read it instead of hardcoding it in
const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

//Standard Chip-8 architecture
pub struct Chip8 {
    memory: [u8; 4096],
    pc: u16,
    index_reg: u16,
    var_regs: [u8; 16],
    stack: [u16; 16],
    delay_timer: u8,
    sound_timer: u8,
    pub display: [bool; 64 * 32]
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: [0; 4096],
            pc: 0x200,
            index_reg: 0,
            var_regs: [0; 16],
            stack: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            display: [false; 64 * 32]
        }
    }

    //Loads the fontset to memory from 0x50 to 0x9F
    pub fn load_font(&mut self) {
        for i in 0..FONTSET.len() {
            self.memory[0x50 + i] = FONTSET[i]
        }
    }
    //Loads the ROM specified by the path in main.rs
    pub fn load_rom(&mut self, path: &str) {
        let mut file = File::open(path).expect("ROM not found");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        for (i, &byte) in buffer.iter().enumerate() {
            let addr = 0x200 + i;
            if addr < 4096 {
                self.memory[addr] = byte;
            } else {
                break;
            }
        }
    }

    //Runs the fetch decode execute loop once
    pub fn tick(&mut self) {
        let instruction = self.fetch();
        let nibbles = self.decode(instruction);
        self.execute(nibbles);
    }

    //Fetches, concatenates, and returns 2 instruction bytes 
    fn fetch(&mut self) -> u16 {
        let instruction: u16 = ((self.memory[self.pc as usize] as u16) << 8) | (self.memory[self.pc as usize + 1] as u16);

        //Increments pc by 2 as instructions are 2 bytes wide
        self.pc += 2;
        
        instruction
    }
    
    //Seperates the instruction into 4 nibbles, from highest to lowest
    fn decode(&self, instruction: u16) -> (u16, u8, u8, u8, u8) {
        let n1: u8 = ((instruction >> 12) & 0x0F) as u8;
        let n2: u8 = ((instruction >> 8) & 0x0F) as u8;
        let n3: u8 = ((instruction >> 4) & 0x0F) as u8;
        let n4: u8 = (instruction & 0x0F) as u8;

        (instruction, n1, n2, n3, n4)
    }

    //Executes instruction based on instruction nibbles
    fn execute(&mut self, (instruction, n1, n2, n3, n4): (u16, u8, u8, u8, u8)) {
        let h2: u8 = n3 << 4 | n4;
        let nnn: u16 = ((n2 as u16) << 8) | ((n3 as u16) << 4) | n4 as u16;

        //Checks instruction type based on first nibble
        if n1 == 0x0 {
            if instruction == 0x00E0 {
                //Clear screen
                for i in 0..self.display.len() {
                    self.display[i] = false;
                }
            } else if instruction == 0x00EE {
                //Subroutine return
            } else {
                //*Depricated* Execute routine (do nothing)
            }
        } else if n1 == 0x1 {
            //Jump
            self.pc = nnn;
        } else if n1 == 0x2 {
            //Call subroutine
        } else if n1 == 0x3 {
            //Skip if equal (immediate)
        } else if n1 == 0x4 {
            //Skip if not equal (immediate)
        } else if n1 == 0x5 {
            //Skip if equal (var reg)
        } else if n1 == 0x6 {
            //Set register (immediate)
            self.var_regs[n2 as usize] = h2;
        } else if n1 == 0x7 {
            //Add (immediate)
            self.var_regs[n2 as usize] += h2;
        } else if n1 == 0x8 {
            //ALU
            if n4 == 0x0 {
                //Set register
            } else if n4 == 0x1 {
                //Binary OR
            } else if n4 == 0x2 {
                //Binary AND
            } else if n4 == 0x3 {
                //Logical XOR
            } else if n4 == 0x4 {
                //Add
            } else if n4 == 0x5 {
                //Subtract (x - y)
            } else if n4 == 0x6 {
                //Shift right
            } else if n4 == 0x7 {
                //Subtract (y - x)
            } else if n4 == 0xE {
                //Shift left
            } else {
                //Throw error
            }
        } else if n1 == 0x9 {
            //Skip if not equal (var reg)
        } else if n1 == 0xA {
            //Set index
            self.index_reg = nnn;
        } else if n1 == 0xB {
            //Jump with offset
        } else if n1 == 0xC {
            //Random
        } else if n1 == 0xD {
            //Display
            let x = self.var_regs[n2 as usize] % 64;
            let y = self.var_regs[n3 as usize] % 32;

            self.var_regs[0xF] = 0;

            let mut yc = y;

            for i in 0..n4 {
                let sprite_data = self.memory[(self.index_reg as usize) + (i as usize)];

                let mut xc = x;

                for j in (0..8).rev() {
                    let mask: u8 = 0b00000001;
                    let shifted_sprite_data = sprite_data >> j;
                    let display_bit: u8 = shifted_sprite_data & mask;
                    let display_bool: bool = display_bit != 0;

                    let index: u16 = ((yc as u16) * 64) + (xc as u16);
                    let pixel: bool = self.display[index as usize];

                    if display_bool & pixel {
                        self.var_regs[0xF] = 1;
                    } 

                    self.display[index as usize] ^= display_bool;

                    if xc + 1 > 64 {
                        break;
                    } else {
                        xc += 1;
                    }
                }

                if yc + 1 > 32 {
                    break;
                } else {
                    yc += 1;
                }

            }

        } else if n1 == 0xE {
            //Skip if key
            if h2 == 0x9E {
                //Skip if key press
            } else if h2 == 0xA1 {
                //Skip if key not pressed
            } else {
                //Throw error
            }
        } else if n1 == 0xF {
            if h2 == 0x07 {
                //Set reg to delay timer val
            } else if h2 == 0x15 {
                //Set delay timer to reg val
            } else if h2 == 0x18 {
                //Set sound timer to reg val
            } else if h2 == 0x1E {
                //Add to index
            } else if h2 == 0x0A {
                //Get key
            } else if h2 == 0x29 {
                //Font char
            } else if h2 == 0x33 {
                //Binary decimal conversion
            } else if h2 == 0x55 {
                //Store to memory
            } else if h2 == 0x65 {
                //Load from memory
            } else {
                //Throw error
            }

        } else {
            //Throw error
        }
    }
}