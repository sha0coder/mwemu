use crate::color;
use crate::console::Console;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Cyan"), ins);
    //TODO: crash if arrive to zero or max value

    if emu.cfg.is_64bits {
        let val = match emu.maps.read_byte(emu.regs().rsi) {
            Some(v) => v,
            None => {
                log::info!("lodsb: memory read error");
                Console::spawn_console(emu);
                0
            }
        };

        emu.regs_mut().set_al(val as u64);
        if emu.flags().f_df {
            emu.regs_mut().rsi -= 1;
        } else {
            emu.regs_mut().rsi += 1;
        }
    } else {
        let val = match emu.maps.read_byte(emu.regs().get_esi()) {
            Some(v) => v,
            None => {
                log::info!("lodsb: memory read error");
                Console::spawn_console(emu);
                0
            }
        };

        emu.regs_mut().set_al(val as u64);
        if emu.flags().f_df {
            let esi = emu.regs().get_esi() - 1;
            emu.regs_mut().set_esi(esi);
        } else {
            let esi = emu.regs().get_esi() + 1;
            emu.regs_mut().set_esi(esi);
        }
    }
    true
}
