use crate::emu::Emu;
use crate::syscall::syscall32;
use crate::{color, exception_type};
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Red"), ins);

    assert!(ins.op_count() == 1);

    let interrupt = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };

    let handle_interrupts = match emu.hooks.hook_on_interrupt {
        Some(hook_fn) => hook_fn(emu, emu.regs().rip, interrupt),
        None => true,
    };

    if handle_interrupts {
        match interrupt {
            0x80 => {
                emu.linux = true;
                syscall32::gateway(emu);
            }

            0x29 => {
                log::info!("call_stack = {:?}", emu.call_stack());
                log::info!("int 0x29: __fastfail {}", emu.regs().rcx);
                std::process::exit(1);
            }

            0x03 => {
                emu.show_instruction(color!("Red"), ins);
                log::info!("/!\\ int 0x3 sigtrap!!!!");
                emu.exception(exception_type::ExceptionType::Int3);
                return false;
            }

            0xdc => {
                log::info!("/!\\ direct syscall: NtAlpcSendWaitReceivePort");
            }

            _ => {
                log::info!("unimplemented interrupt {}", interrupt);
                return false;
            }
        }
    }
    true
}
