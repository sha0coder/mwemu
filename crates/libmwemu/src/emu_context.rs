use iced_x86::Formatter as _;
use std::cell::RefCell;

use crate::emu::Emu;

thread_local! {
    static CURRENT_EMU: RefCell<Option<*const Emu>> = RefCell::new(None);
}

pub fn with_current_emu<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&Emu) -> R,
{
    CURRENT_EMU.with(|current| {
        current
            .borrow()
            .and_then(|ptr| unsafe { ptr.as_ref().map(|emu| f(emu)) })
    })
}

pub fn with_current_emu_mut<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&mut Emu) -> R,
{
    CURRENT_EMU.with(|current| {
        current
            .borrow()
            .and_then(|ptr| unsafe { (ptr as *mut Emu).as_mut().map(|emu| f(emu)) })
    })
}

pub fn set_current_emu(emu: &Emu) {
    CURRENT_EMU.with(|current| {
        *current.borrow_mut() = Some(emu as *const _);
    });
}

pub fn clear_current_emu() {
    CURRENT_EMU.with(|current| {
        *current.borrow_mut() = None;
    });
}

pub fn is_emu_set() -> bool {
    CURRENT_EMU.with(|current| current.borrow().is_some())
}

pub fn log_emu_state(emu: &mut Emu) {
    log::error!("=== EMULATOR STATE AT PANIC ===");
    log::error!("Current position: {}", emu.pos);

    let mut out: String = String::new();
    let color = "\x1b[0;31m";
    match emu.instruction {
        Some(ins) => {
            let ins = ins.clone();
            emu.formatter.format(&ins, &mut out);
            log::info!(
                "{}{} 0x{:x}: {}{}",
                color,
                emu.pos,
                ins.ip(),
                out,
                emu.colors.nc
            );
        }
        None => {}
    };

    // Log general purpose registers
    log::error!("Registers:");
    log::error!(
        "  RAX: 0x{:016x}  RBX: 0x{:016x}",
        emu.regs().rax,
        emu.regs().rbx
    );
    log::error!(
        "  RCX: 0x{:016x}  RDX: 0x{:016x}",
        emu.regs().rcx,
        emu.regs().rdx
    );
    log::error!(
        "  RSI: 0x{:016x}  RDI: 0x{:016x}",
        emu.regs().rsi,
        emu.regs().rdi
    );
    log::error!(
        "  RBP: 0x{:016x}  RSP: 0x{:016x}",
        emu.regs().rbp,
        emu.regs().rsp
    );
    log::error!(
        "  R8:  0x{:016x}  R9:  0x{:016x}",
        emu.regs().r8,
        emu.regs().r9
    );
    log::error!(
        "  R10: 0x{:016x}  R11: 0x{:016x}",
        emu.regs().r10,
        emu.regs().r11
    );
    log::error!(
        "  R12: 0x{:016x}  R13: 0x{:016x}",
        emu.regs().r12,
        emu.regs().r13
    );
    log::error!(
        "  R14: 0x{:016x}  R15: 0x{:016x}",
        emu.regs().r14,
        emu.regs().r15
    );
    log::error!("  RIP: 0x{:016x}", emu.regs().rip);

    // Log flags
    log::error!("EFLAGS: 0x{:08x}", emu.flags().dump());

    // Log last instruction if available
    if let Some(ref _instruction) = emu.instruction {
        log::error!("Last instruction: {}", emu.mnemonic);
        log::error!("Instruction size: {}", emu.last_instruction_size);
    }

    // Log call stack
    if !emu.call_stack().is_empty() {
        log::error!(
            "Call stack (last {} entries):",
            emu.call_stack().len().min(10)
        );
        for (i, entry) in emu.call_stack().iter().rev().take(10).enumerate() {
            log::error!("  {}: {:x}:call:{:x}", i, entry.0, entry.1);
        }
    }

    // Log execution info
    log::error!("Tick count: {}", emu.tick);
    log::error!("Base address: 0x{:x}", emu.base);
    log::error!("Filename: {}", emu.filename);

    log::error!("==============================");
}
