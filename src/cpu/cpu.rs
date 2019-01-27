// Register set source:
// http://codebase64.org/doku.php?id=base:6502_registers

#![allow(non_snake_case)]
#![allow(unused)]

use super::mmu::MMU;

pub struct MOS6510 {

    pub A : u8,
    pub X : u8,
    pub Y : u8,
    pub S : u8,
    pub PC : u16,
    pub P : u8, // flags: N V 1 B D I Z C

    /* N - Negative flag
     * V - oVerflow flag
     * 1 - unused flag, always 1
     * B - Break flag
     * D - Decimal mode flag
     * I - Interrupt disable flag
     * Z - Zero flag
     * C - Carry flag */

     pub mmu : MMU,
}

impl MOS6510 {
    pub fn new() -> MOS6510 {
        MOS6510 {
            A : 0x00,
            X : 0x00,
            Y : 0x00,
            S : 0x00,
            PC : 0x0000,
            P : 0b0010_0000,
            mmu : MMU::new(),
        }
    }
}