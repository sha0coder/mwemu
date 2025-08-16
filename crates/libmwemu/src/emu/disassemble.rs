use iced_x86::{Decoder, DecoderOptions, Formatter as _, Instruction};

use crate::emu::Emu;

impl Emu {
    /// Disassemble an amount of instruccions on an specified address.
    /// This not used on the emulation engine, just from console, 
    /// but the api could be used programatilcally.
    pub fn disassemble(&mut self, addr: u64, amount: u32) -> String {
        let mut out = String::new();
        let code = self.maps.get_mem_by_addr(addr).expect("address not mapped");
        let block = code.read_from(addr);

        let bits: u32 = if self.cfg.is_64bits { 64 } else { 32 };
        let mut decoder = Decoder::with_ip(bits, block, addr, DecoderOptions::NONE);
        let mut output = String::new();
        let mut instruction = Instruction::default();
        let mut count: u32 = 1;
        while decoder.can_decode() {
            decoder.decode_out(&mut instruction);
            output.clear();
            self.formatter.format(&instruction, &mut output);
            if self.cfg.is_64bits {
                out.push_str(&format!("0x{:x}: {}\n", instruction.ip(), output));
                //log::info!("0x{:x}: {}", instruction.ip(), output);
            } else {
                out.push_str(&format!("0x{:x}: {}\n", instruction.ip32(), output));
                //log::info!("0x{:x}: {}", instruction.ip32(), output);
            }
            count += 1;
            if count == amount {
                break;
            }
        }
        out
    }
}
