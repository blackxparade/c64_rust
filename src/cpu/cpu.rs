// Register set source:
// http://codebase64.org/doku.php?id=base:6502_registers

#![allow(non_snake_case)]
#![allow(unused)]

use super::mmu::MMU;
use super::debugger::Debugger;
use super::opcode::Opcode;
use std::fs::File;
use std::io::Read;

pub struct MOS6510 {

    pub A  : u8,
    pub X  : u8,
    pub Y  : u8,
    pub S  : u8,
    pub PC : u16,
    pub P  : u8, // flags: N V 1 B D I Z C

    /* N - Negative flag
     * V - oVerflow flag
     * 1 - unused flag, always 1
     * B - Break flag
     * D - Decimal mode flag
     * I - Interrupt disable flag
     * Z - Zero flag
     * C - Carry flag */

     pub mmu : MMU,
     pub dbg : Debugger,
     pub opc : Opcode,
}

impl MOS6510 {
    pub fn new() -> MOS6510 {
        MOS6510 {
            A   : 0x00,
            X   : 0x00,
            Y   : 0x00,
            S   : 0x00,
            PC  : 0x0000,
            P   : 0b0010_0000,
            mmu : MMU::new(),
            dbg : Debugger::new(),
            opc : Opcode::new(),
        }
    }

    pub fn cycle(&mut self) {
        loop {
            if (!self.dbg.poll()) { return; }

            // let's do the trusty old fetch-decode-execute steps



            // debugger initialization
            // it should be at the end of a cycle
            let text : Vec<String> =
                vec![String::from("This is a test. , : / ( ) [ ] { } = ? ! - + # ' % $"); 30];

            self.dbg.clear();
            self.dbg.assemble_text(&self.opc.recent_opc);
            self.mmu.randomize();
            self.dbg.memory_map(&self.mmu.RAM);
            self.dbg.render();
        }
    }

    pub fn load_to_ram(&mut self, path : String, address : u16) {
        let mut buffer : Vec<u8> = Vec::new();

        // open the file
        let mut f = File::open(&path)
        .expect("\n Error with file loading! \n");

        // read the file to rom_buffer
        f.read_to_end(&mut buffer)
        .expect("Error with file reading!");

        // copy buffer to a pecific RAM address
        for i in 0..buffer.len() {
            self.mmu.write(buffer[i], address);
        }

    } // fn load_bootrom
}
