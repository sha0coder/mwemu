use crate::console::Console;
use crate::emu::Emu;
use crate::{color, exception};
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    let ret_addr: u64 = if emu.cfg.is_64bits {
        match emu.stack_pop64(false) {
            Some(v) => v,
            None => return false,
        }
    } else {
        match emu.stack_pop32(false) {
            Some(v) => v as u64,
            None => return false,
        }
    };

    emu.show_instruction_ret(color!("Yellow"), ins, ret_addr);

    if emu.break_on_next_return {
        emu.break_on_next_return = false;
        Console::spawn_console(emu);
    }

    if ins.op_count() > 0 {
        let arg = emu
            .get_operand_value(ins, 0, true)
            .expect("weird crash on ret");
        // apply stack compensation of ret operand

        if emu.cfg.is_64bits {
            if arg % 8 != 0 {
                log::info!("weird ret argument!");
                return false;
            }

            emu.regs_mut().rsp += arg;
            //emu.stack_lvl[emu.stack_lvl_idx] -= arg as i32 / 8;
        } else {
            if arg % 4 != 0 {
                log::info!("weird ret argument!");
                return false;
            }

            let esp = emu.regs().get_esp() + arg;
            emu.regs_mut().set_esp(esp);
            //emu.stack_lvl[emu.stack_lvl_idx] -= arg as i32 / 4;
        }
    }

    emu.call_stack_mut().pop();

    if emu.run_until_ret {
        return true;
    }

    if emu.eh_ctx() != 0 {
        exception::exit(emu);
        return true;
    }

    if emu.cfg.is_64bits {
        return emu.set_rip(ret_addr, false);
    } else {
        return emu.set_eip(ret_addr, false);
    }
    //true
}
