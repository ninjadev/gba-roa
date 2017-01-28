use gamepad;
use cpu;

pub trait InterconnectInterface {
    fn handle(&mut self, operation: Operation, address: u32, word: u32);
}

pub enum Operation {
    Read,
    Write,
}

#[derive(Debug)]
pub struct Interconnect {
    pub gamepad: gamepad::Gamepad,
}

impl Interconnect {
    pub fn new() -> Self {
        Interconnect {
            gamepad: gamepad::Gamepad::default(),
        }
    }
}

impl InterconnectInterface for Interconnect {
    fn handle(&mut self, operation: Operation, address: u32, word: u32) {
        match address {
            // General Internal Memory
            0x00000000 ... 0x00003FFF => (), //   BIOS - System ROM         (16 KBytes)
            0x00004000 ... 0x01FFFFFF => (), //   Not used
            0x02000000 ... 0x0203FFFF => (), //   WRAM - On-board Work RAM  (256 KBytes) 2 Wait
            0x02040000 ... 0x02FFFFFF => (), //   Not used
            0x03000000 ... 0x03007FFF => (), //   WRAM - In-chip Work RAM   (32 KBytes)
            0x03008000 ... 0x03FFFFFF => (), //   Not used
            0x04000000 ... 0x040003FE => self.gamepad.handle(operation, address, word), //   I/O Registers
            0x04000400 ... 0x04FFFFFF => (), //   Not used

            // Internal Display Memory
            0x05000000 ... 0x050003FF => (), //   BG/OBJ Palette RAM        (1 Kbyte)
            0x05000400 ... 0x05FFFFFF => (), //   Not used
            0x06000000 ... 0x06017FFF => (), //   VRAM - Video RAM          (96 KBytes)
            0x06018000 ... 0x06FFFFFF => (), //   Not used
            0x07000000 ... 0x070003FF => (), //   OAM - OBJ Attributes      (1 Kbyte)
            0x07000400 ... 0x07FFFFFF => (), //   Not used

            // External Memory (Game Pak)
            0x08000000 ... 0x09FFFFFF => (), //   Game Pak ROM/FlashROM (max 32MB) - Wait State 0
            0x0A000000 ... 0x0BFFFFFF => (), //   Game Pak ROM/FlashROM (max 32MB) - Wait State 1
            0x0C000000 ... 0x0DFFFFFF => (), //   Game Pak ROM/FlashROM (max 32MB) - Wait State 2
            0x0E000000 ... 0x0E00FFFF => (), //   Game Pak SRAM    (max 64 KBytes) - 8bit Bus width
            0x0E010000 ... 0x0FFFFFFF => (), //   Not used

            // Unused Memory Area
            0x10000000 ... 0xFFFFFFFF => (), //   Not used (upper 4bits of address bus unused),
            _ => panic!(),
        }
    }
}
