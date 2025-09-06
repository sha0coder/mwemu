use crate::color;
use crate::console::Console;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Orange"), ins);

    assert!(ins.op_count() == 2);

    let value0 = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };

    let value1 = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    if emu.cfg.verbose >= 2 {
        if value0 > value1 {
            log::info!("\tcmp: 0x{:x} > 0x{:x}", value0, value1);
        } else if value0 < value1 {
            log::info!("\tcmp: 0x{:x} < 0x{:x}", value0, value1);
        } else {
            log::info!("\tcmp: 0x{:x} == 0x{:x}", value0, value1);
        }
    }

    if emu.break_on_next_cmp {
        Console::spawn_console(emu);
        emu.break_on_next_cmp = false;

        let value0 = match emu.get_operand_value(ins, 0, true) {
            Some(v) => v,
            None => return false,
        };

        let value1 = match emu.get_operand_value(ins, 1, true) {
            Some(v) => v,
            None => return false,
        };

        if emu.cfg.verbose >= 2 {
            if value0 > value1 {
                log::info!("\tcmp: 0x{:x} > 0x{:x}", value0, value1);
            } else if value0 < value1 {
                log::info!("\tcmp: 0x{:x} < 0x{:x}", value0, value1);
            } else {
                log::info!("\tcmp: 0x{:x} == 0x{:x}", value0, value1);
            }
        }
    }

    match emu.get_operand_sz(ins, 0) {
        64 => {
            emu.flags_mut().sub64(value0, value1);
        }
        32 => {
            emu.flags_mut().sub32(value0, value1);
        }
        16 => {
            emu.flags_mut().sub16(value0, value1);
        }
        8 => {
            emu.flags_mut().sub8(value0, value1);
        }
        _ => {
            panic!("wrong size {}", emu.get_operand_sz(ins, 0));
        }
    }
    true
}
