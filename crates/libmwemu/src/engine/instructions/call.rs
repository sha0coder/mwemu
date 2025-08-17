use crate::emu::Emu;
use crate::{color};
use iced_x86::{Instruction};

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Yellow"), ins);

    if ins.op_count() != 1 {
        unimplemented!("weird variant of call");
    }

    let addr = match emu.get_jump_value(ins, 0) {
        Some(a) => a,
        None => return false,
    };

    if emu.cfg.trace_calls {
        log::info!("{} 0x{:x} CALL 0x{:x}", emu.pos, emu.regs().rip, addr);
    }

    if emu.regs_mut().rip == addr - 5 {
        if emu.cfg.verbose >= 1 {
            log::info!("call next instruction, prolly call/pop");
        }
        //emu.stack_lvl[emu.stack_lvl_idx] -= 1;
    } /*else {
          emu.stack_lvl.push(0);
          emu.stack_lvl_idx += 1;
      }*/

    let call_stack_label = format!("{:x}:call:{:x}", emu.regs().rip, addr);
    emu.call_stack_mut()
        .push(call_stack_label);

    if emu.cfg.is_64bits {
        if !emu.stack_push64(emu.regs().rip + instruction_sz as u64) {
            return false;
        }
        return emu.set_rip(addr, false);
    } else {
        if !emu.stack_push32(emu.regs().get_eip() as u32 + instruction_sz as u32) {
            return false;
        }
        return emu.set_eip(addr, false);
    }
    //true
}
