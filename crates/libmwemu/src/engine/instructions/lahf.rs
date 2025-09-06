use crate::emu::Emu;
use crate::{color, set_bit};
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Red"), ins);

    //log::info!("\tlahf: flags = {:?}", emu.flags);

    let mut result: u8 = 0;
    set_bit!(result, 0, emu.flags().f_cf as u8);
    set_bit!(result, 1, true as u8);
    set_bit!(result, 2, emu.flags().f_pf as u8);
    set_bit!(result, 3, false as u8);
    set_bit!(result, 4, emu.flags().f_af as u8);
    set_bit!(result, 5, false as u8);
    set_bit!(result, 6, emu.flags().f_zf as u8);
    set_bit!(result, 7, emu.flags().f_sf as u8);
    emu.regs_mut().set_ah(result as u64);
    true
}
