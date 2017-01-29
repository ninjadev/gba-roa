use gamepad;
use cpu;
use display;

pub trait InterconnectRead {
    fn read(&self, address: u32) -> u32;
}

pub trait InterconnectWrite {
    fn write(&mut self, address: u32, word: u32);
}

enum Operation {
    Read,
    Write,
}

pub struct Interconnect {
    pub gamepad: gamepad::Gamepad,
    pub display: display::Display,
}

impl Interconnect {
    pub fn new() -> Self {
        Interconnect {
            gamepad: gamepad::Gamepad::default(),
            display: display::Display::new(),
        }
    }

    fn handle(&mut self, operation: Operation, address: u32, word: u32) -> Option<u32> {
        match address {
            // General Internal Memory
            0x00000000...0x00003FFF => None, //   BIOS - System ROM         (16 KBytes)
            0x00004000...0x01FFFFFF => None, //   Not used
            0x02000000...0x0203FFFF => None, //   WRAM - On-board Work RAM  (256 KBytes) 2 Wait
            0x02040000...0x02FFFFFF => None, //   Not used
            0x03000000...0x03007FFF => None, //   WRAM - In-chip Work RAM   (32 KBytes)
            0x03008000...0x03FFFFFF => None, //   Not used
            0x04000000...0x040003FE => Some(self.gamepad.read(address)), //   I/O Registers
            0x04000400...0x04FFFFFF => None, //   Not used

            // Internal Display Memory
            0x05000000...0x050003FF => None, //   BG/OBJ Palette RAM        (1 Kbyte)
            0x05000400...0x05FFFFFF => None, //   Not used
            0x06000000...0x06017FFF => {
                self.display.write(address - 0x06000000, word);
                Some(0)
            } //   VRAM - Video RAM          (96 KBytes)
            0x06018000...0x06FFFFFF => None, //   Not used
            0x07000000...0x070003FF => None, //   OAM - OBJ Attributes      (1 Kbyte)
            0x07000400...0x07FFFFFF => None, //   Not used

            // External Memory (Game Pak)
            0x08000000...0x09FFFFFF => None, //   Game Pak ROM/FlashROM (max 32MB) - Wait State 0
            0x0A000000...0x0BFFFFFF => None, //   Game Pak ROM/FlashROM (max 32MB) - Wait State 1
            0x0C000000...0x0DFFFFFF => None, //   Game Pak ROM/FlashROM (max 32MB) - Wait State 2
            0x0E000000...0x0E00FFFF => None, //   Game Pak SRAM    (max 64 KBytes) - 8bit Bus width
            0x0E010000...0x0FFFFFFF => None, //   Not used

            // Unused Memory Area
            0x10000000...0xFFFFFFFF => None, //   Not used (upper 4bits of address bus unused),
            _ => panic!(),
        }
    }
}

// impl InterconnectRead for Interconnect {
//     fn read(&self, address: u32) -> u32 {
//         self.handle(Operation::Read, address).unwrap()
//     }
// }

impl InterconnectWrite for Interconnect {
    fn write(&mut self, address: u32, word: u32) {
        self.handle(Operation::Write, address, word);
    }
}
