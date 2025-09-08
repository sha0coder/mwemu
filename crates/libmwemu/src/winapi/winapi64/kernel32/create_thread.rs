use crate::maps::mem64::Permission;
use crate::thread_context::ThreadContext;
use crate::winapi::helper;
use crate::{constants, emu};

pub fn CreateThread(emu: &mut emu::Emu) {
    // Add comprehensive debugging
    log::info!("=== CreateThread Debug Info ===");
    log::info!("Current RIP: 0x{:x}", emu.regs().rip);
    log::info!("Current RSP: 0x{:x}", emu.regs().rsp);
    log::info!("Is 64-bit: {}", emu.cfg.is_64bits);

    // Log all register values
    log::info!("RCX (lpThreadAttributes): 0x{:x}", emu.regs().rcx);
    log::info!("RDX (dwStackSize): 0x{:x}", emu.regs().rdx);
    log::info!("R8 (lpStartAddress): 0x{:x}", emu.regs().r8);
    log::info!("R9 (lpParameter): 0x{:x}", emu.regs().r9);

    // Check if RSP is mapped and dump stack area
    let rsp = emu.regs().rsp;
    log::info!("RSP mapped: {}", emu.maps.is_mapped(rsp));

    if emu.maps.is_mapped(rsp) {
        log::info!("Stack dump around RSP:");
        for i in 0..8 {
            let addr = rsp + (i * 8);
            if emu.maps.is_mapped(addr) {
                match emu.maps.read_qword(addr) {
                    Some(value) => log::info!("  [RSP+0x{:02x}] = 0x{:x}", i * 8, value),
                    None => log::info!("  [RSP+0x{:02x}] = None (banzai mode)", i * 8),
                }
            } else {
                log::info!("  [RSP+0x{:02x}] = UNMAPPED", i * 8);
            }
        }
    }

    // Try to read the stack parameters with error handling
    let flags_addr = emu.regs().rsp + 0x20;
    let tid_ptr_addr = emu.regs().rsp + 0x28;

    log::info!("Trying to read flags from 0x{:x}", flags_addr);
    log::info!("Flags address mapped: {}", emu.maps.is_mapped(flags_addr));

    log::info!("Trying to read tid_ptr from 0x{:x}", tid_ptr_addr);
    log::info!(
        "Tid_ptr address mapped: {}",
        emu.maps.is_mapped(tid_ptr_addr)
    );

    let flags = match emu.maps.read_qword(flags_addr) {
        Some(f) => {
            log::info!("Successfully read flags: 0x{:x}", f);
            f
        }
        None => {
            panic!(
                "Failed to read flags from 0x{:x} - unmapped memory",
                flags_addr
            );
        }
    };

    let tid_ptr = match emu.maps.read_qword(tid_ptr_addr) {
        Some(t) => {
            log::info!("Successfully read tid_ptr: 0x{:x}", t);
            t
        }
        None => {
            panic!(
                "Failed to read tid_ptr from 0x{:x} - unmapped memory",
                tid_ptr_addr
            );
        }
    };

    // Check if tid_ptr points to valid memory
    if tid_ptr > 0 {
        log::info!(
            "Checking if tid_ptr 0x{:x} is mapped: {}",
            tid_ptr,
            emu.maps.is_mapped(tid_ptr)
        );

        // Try to find what memory region this might be in
        log::info!("Memory maps around tid_ptr:");
        // You might want to add a method to dump nearby memory regions
    }

    // Check if the thread start address is mapped
    let start_addr = emu.regs().r8;
    log::info!(
        "Thread start address 0x{:x} mapped: {}",
        start_addr,
        emu.maps.is_mapped(start_addr)
    );

    log::info!("==============================");

    // Original CreateThread logic with better error handling
    let sec_attr = emu.regs().rcx;
    let stack_sz = emu.regs().rdx;
    let code = emu.regs().r8;
    let param = emu.regs().r9;

    let new_thread_id = 0x1000 + emu.threads.len();
    let mut new_thread = ThreadContext::new(new_thread_id as u64);

    // Initialize thread context with entry point and parameter
    new_thread.regs.rip = code;
    new_thread.regs.rcx = param;
    new_thread.regs.rax = 0;

    // Allocate stack if requested
    if stack_sz > 0 {
        if let Some(stack_base) = emu.maps.alloc(stack_sz) {
            new_thread.regs.rsp = stack_base + stack_sz - 8;
            new_thread.regs.rbp = new_thread.regs.rsp;
            emu.maps
                .create_map(
                    &format!("thread_stack_{:x}", new_thread_id),
                    stack_base,
                    stack_sz,
                    Permission::READ_WRITE,
                )
                .ok();
            log::info!(
                "Allocated stack: 0x{:x} - 0x{:x}",
                stack_base,
                stack_base + stack_sz
            );
        } else {
            panic!("Failed to allocate stack of size 0x{:x}", stack_sz);
        }
    }

    // Sync FPU instruction pointer
    new_thread.fpu.set_ip(code);

    // Set suspended state if CREATE_SUSPENDED flag is set
    if (flags & constants::CREATE_SUSPENDED) != 0 {
        new_thread.suspended = true;
        log::info!("Thread created in suspended state (flags: 0x{:x})", flags);
    } else {
        new_thread.suspended = false;
        log::info!("Thread created in running state (flags: 0x{:x})", flags);
    }

    emu.threads.push(new_thread);

    // Write thread ID with detailed logging
    if tid_ptr > 0 {
        if emu.maps.is_mapped(tid_ptr) {
            log::info!("Writing thread ID {} to 0x{:x}", new_thread_id, tid_ptr);
            emu.maps.write_dword(tid_ptr, new_thread_id as u32);
        } else {
            panic!(
                "CANNOT WRITE: tid_ptr 0x{:x} points to unmapped memory!",
                tid_ptr
            );
        }
    } else {
        log::info!("tid_ptr is NULL, not writing thread ID");
    }

    log_red!(
        emu,
        "kernel32!CreateThread code: 0x{:x} param: 0x{:x} flags: 0x{:x}",
        code,
        param,
        flags
    );

    log::info!("THREAD ARRAY STATE:");
    for (idx, thread) in emu.threads.iter().enumerate() {
        log::info!(
            "  threads[{}]: ID=0x{:x}, suspended={}, RIP=0x{:x}",
            idx,
            thread.id,
            thread.suspended,
            thread.regs.rip
        );
    }
    log::info!("current_thread_id = {}", emu.current_thread_id);

    emu.regs_mut().rax = helper::handler_create(&format!("tid://0x{:x}", new_thread_id));
    log::info!("Returning handle: 0x{:x}", emu.regs().rax);
}
