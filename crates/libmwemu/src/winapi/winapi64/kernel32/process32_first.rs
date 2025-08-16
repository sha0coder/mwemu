use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn Process32First(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let lppe = emu.regs().rdx;

    log::info!(
        "{}** {} kernel32!Process32First hndl: {:x} lppe: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        handle,
        lppe,
        emu.colors.nc
    );

    if !helper::handler_exist(handle) {
        emu.regs_mut().rax = 0;
        return;
    }

    emu.maps.write_string(lppe + 44, "smss.exe\x00");

    /*

                typedef struct tagPROCESSENTRY32 {
                DWORD     dwSize;                +0
                DWORD     cntUsage;              +4
                DWORD     th32ProcessID;         +8
                ULONG_PTR th32DefaultHeapID;    +12
                DWORD     th32ModuleID;         +16
                DWORD     cntThreads;           +20
                DWORD     th32ParentProcessID;  +24
                LONG      pcPriClassBase;       +28
                DWORD     dwFlags;              +32
                CHAR      szExeFile[MAX_PATH];  +36
                } PROCESSENTRY32;
    */

    emu.regs_mut().rax = 1;
}