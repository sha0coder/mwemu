use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    assert!(emu.cfg.is_64bits);

    if emu.rep.is_some() {
        if emu.rep.unwrap() == 0 || emu.cfg.verbose >= 3 {
            emu.show_instruction(color!("LightCyan"), ins);
        }
    } else {
        emu.show_instruction(color!("LightCyan"), ins);
    }

    let value0: u64 = match emu.maps.read_qword(emu.regs().rsi) {
        Some(v) => v,
        None => {
            log::info!("cannot read rsi");
            return false;
        }
    };
    let value1: u64 = match emu.maps.read_qword(emu.regs().rdi) {
        Some(v) => v,
        None => {
            log::info!("cannot read rdi");
            return false;
        }
    };

    if emu.flags().f_df {
        emu.regs_mut().rsi -= 8;
        emu.regs_mut().rdi -= 8;
    } else {
        emu.regs_mut().rsi += 8;
        emu.regs_mut().rdi += 8;
    }

    emu.flags_mut().sub64(value0, value1);

    if emu.cfg.verbose >= 2 {
        if value0 > value1 {
            log::info!("\tcmp: 0x{:x} > 0x{:x}", value0, value1);
        } else if value0 < value1 {
            log::info!("\tcmp: 0x{:x} < 0x{:x}", value0, value1);
        } else {
            log::info!("\tcmp: 0x{:x} == 0x{:x}", value0, value1);
        }
    }
    true
}
