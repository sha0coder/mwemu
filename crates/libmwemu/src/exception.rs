/*
    SEH, VEH and UEF

    - SEH: structured exception handler (try/catch, stack items)
    - VEH: vectorized exception handler (API)
            ntdll!RtlAddVectoredExceptionHandler
            ntdll!RtlRemoveVectoredExceptionHandler
    - UEF: unhandled exception filter (API)
            ntdll!RtlSetUnhandledExceptionFilter
            kernelbase!SetUnhandledExceptionFilter

*/

use crate::context::context32::Context32;
use crate::context::context64::Context64;
use crate::emu;
use crate::exception_type;
use crate::maps::mem64::Permission;

pub fn enter(emu: &mut emu::Emu, ex_type: exception_type::ExceptionType) {
    if emu.cfg.is_64bits {
        enter64(emu, ex_type);
    } else {
        enter32(emu, ex_type);
    }
}

pub fn exit(emu: &mut emu::Emu) {
    if emu.cfg.is_64bits {
        exit64(emu);
    } else {
        exit32(emu);
    }
}

pub fn enter32(emu: &mut emu::Emu, ex_type: exception_type::ExceptionType) {
    let ctx_addr = emu.maps.alloc(0x1000).expect("out of memory");
    if (ctx_addr + 0x1000) > u32::MAX as u64 {
        panic!("32bits allocator is giving a too big memory!! for the context32");
    }
    let ctx = emu
        .maps
        .create_map("ctx", ctx_addr, 0x1000, Permission::READ_WRITE_EXECUTE);
    emu.set_eh_ctx((ctx_addr + 0x100) as u32);

    emu.stack_push32(ctx_addr as u32); // 0x10f00
    emu.stack_push32(emu.regs().get_eip() as u32);

    emu.set_eh_ctx(ctx_addr as u32 + 8); // 0x10f08
    emu.maps.write_dword(ctx_addr + 4, emu.eh_ctx()); // 0x10f04
    emu.maps.write_dword(
        emu.eh_ctx() as u64,
        exception_type::exception_type_code(ex_type),
    ); // STATUS_BREAKPOINT

    let ctx = Context32::new(&emu.regs());
    ctx.save(emu.eh_ctx(), &mut emu.maps);
}

pub fn exit32(emu: &mut emu::Emu) {
    let mut ctx = Context32::new(&emu.regs());
    ctx.load(emu.eh_ctx(), &mut emu.maps);
    ctx.sync(emu.regs_mut());
    emu.set_eh_ctx(0);
    emu.force_reload = true;
    emu.maps.free("ctx");
}

pub fn enter64(emu: &mut emu::Emu, ex_type: exception_type::ExceptionType) {
    let ctx_addr = emu.maps.alloc(0x1000).expect("out of memory");
    let ctx = emu
        .maps
        .create_map("ctx", ctx_addr, 0x1000, Permission::READ_WRITE_EXECUTE);

    emu.stack_push64(ctx_addr); // 0x10f00
    emu.stack_push64(emu.regs().rip);

    if ctx_addr > u32::MAX as u64 {
        panic!("64bits allocator is giving a too big memory!! for the context64");
    }

    emu.set_eh_ctx(ctx_addr as u32 + 16);
    emu.maps.write_qword(ctx_addr + 8, emu.eh_ctx() as u64);
    let ctx = Context64::new(&emu.regs());
    ctx.save(emu.eh_ctx() as u64, &mut emu.maps);
}

pub fn exit64(emu: &mut emu::Emu) {
    let mut ctx = Context64::new(&emu.regs());
    ctx.load(emu.eh_ctx() as u64, &mut emu.maps);
    ctx.sync(emu.regs_mut());
    emu.set_eh_ctx(0);
    emu.force_reload = true;
    emu.maps.free("ctx");
}
