use crate::color;
use crate::emu::Emu;
use crate::winapi::{winapi32, winapi64};
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Yellow"), &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins));

    if ins.op_count() != 1 {
        unimplemented!("weird variant of call");
    }

    let addr = match emu.get_jump_value(ins, 0) {
        Some(a) => a,
        None => return false,
    };

    if emu.cfg.trace_calls {
        let callee_sym = if emu.cfg.is_x64() {
            winapi64::kernel32::guess_api_name(emu, addr)
        } else {
            winapi32::kernel32::guess_api_name(emu, addr as u32)
        };
        let callee_note = if callee_sym.is_empty() {
            String::new()
        } else {
            format!(" {}", callee_sym)
        };
        log::trace!(
            "{} 0x{:x} CALL 0x{:x}{} (0x{:x}, 0x{:x}, 0x{:x})",
            emu.pos,
            emu.regs().rip,
            addr,
            callee_note,
            emu.regs().rcx,
            emu.regs().rdx,
            emu.regs().r8
        );
    }

    if emu.regs_mut().rip == addr - 5 {
        if emu.cfg.verbose >= 1 {
            log::trace!("call next instruction, prolly call/pop");
        }
        //emu.stack_lvl[emu.stack_lvl_idx] -= 1;
    } /*else {
          emu.stack_lvl.push(0);
          emu.stack_lvl_idx += 1;
      }*/

    let rip = emu.regs().rip;
    emu.call_stack_mut().push((rip, addr));

    if emu.cfg.is_x64() {
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
