#![allow(non_snake_case)]
#![allow(unused)]

pub struct MMU {
    RAM : [u8; 65535],
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            RAM : [0; 65535],
        }
    }

    pub fn write(&mut self, byte : u8, address : u16) {
        self.RAM[address as usize] = byte;
    }

    pub fn read(&mut self, address : u16) -> u8 {
        self.RAM[address as usize]
    }
}
