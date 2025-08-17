use crate::constants;
use crate::emu;
use crate::serialization;
use crate::winapi::helper;
use crate::winapi::winapi64;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    match api.as_str() {
        "StartServiceCtrlDispatcherA" => StartServiceCtrlDispatcherA(emu),
        "StartServiceCtrlDispatcherW" => StartServiceCtrlDispatcherW(emu),
        "RegOpenKeyExA" => RegOpenKeyExA(emu),
        "RegQueryValueExA" => RegQueryValueExA(emu),
        "RegCloseKey" => RegCloseKey(emu),
        "GetUserNameA" => GetUserNameA(emu),
        "GetUserNameW" => GetUserNameW(emu),

        _ => {
            if emu.cfg.skip_unimplemented == false {
                if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                    serialization::Serialization::dump_to_file(
                        &emu,
                        emu.cfg.dump_filename.as_ref().unwrap(),
                    );
                }

                unimplemented!("atemmpt to call unimplemented API 0x{:x} {}", addr, api);
            }
            log::warn!(
                "calling unimplemented API 0x{:x} {} at 0x{:x}",
                addr,
                api,
                emu.regs().rip
            );
            return api;
        }
    }

    String::new()
}

fn StartServiceCtrlDispatcherA(emu: &mut emu::Emu) {
    let service_table_entry_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("advapi32!StartServiceCtrlDispatcherA error reading service_table_entry pointer");

    let service_name = emu
        .maps
        .read_dword(service_table_entry_ptr as u64)
        .expect("advapi32!StartServiceCtrlDispatcherA error reading service_name");
    let service_name = emu
        .maps
        .read_dword((service_table_entry_ptr + 4) as u64)
        .expect("advapi32!StartServiceCtrlDispatcherA error reading service_name");

    emu.regs_mut().set_eax(1);
}

fn StartServiceCtrlDispatcherW(emu: &mut emu::Emu) {
    let service_table_entry_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("advapi32!StartServiceCtrlDispatcherW error reading service_table_entry pointer");

    emu.regs_mut().set_eax(1);
}

fn RegOpenKeyExA(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let opts = emu.regs().r8;
    let result = emu.regs().r9;

    let subkey = emu.maps.read_string(subkey_ptr);

    log::info!(
        "{}** {} advapi32!RegOpenKeyExA {} {}",
        emu.colors.light_red,
        emu.pos,
        subkey,
        emu.colors.nc
    );

    emu.maps
        .write_qword(result, helper::handler_create(&subkey));
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}

fn RegCloseKey(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;

    log::info!(
        "{}** {} advapi32!RegCloseKey {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );

    helper::handler_close(hkey);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}

fn RegQueryValueExA(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let value_ptr = emu.regs().rdx;
    let reserved = emu.regs().r8;
    let typ_out = emu.regs().r9;
    let data_out = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("error reading api aparam");
    let datasz_out = emu
        .maps
        .read_qword(emu.regs().rsp + 0x28)
        .expect("error reading api param");

    let mut value = String::new();
    if value_ptr > 0 {
        value = emu.maps.read_string(value_ptr);
    }

    log::info!(
        "{}** {} advapi32!RegQueryValueExA {} {}",
        emu.colors.light_red,
        emu.pos,
        value,
        emu.colors.nc
    );

    if data_out > 0 {
        emu.maps.write_string(data_out, "some_random_reg_contents");
    }
    if datasz_out > 0 {
        emu.maps.write_qword(datasz_out, 24);
    }
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}

fn GetUserNameA(emu: &mut emu::Emu) {
    let out_username = emu.regs().rcx;
    let in_out_sz = emu.regs().rdx;

    let mut sz = 0;

    if out_username > 0 && in_out_sz > 0 {
        if emu.maps.is_mapped(in_out_sz) && emu.maps.is_mapped(out_username) {
            sz = emu.maps.read_qword(in_out_sz).expect("advapi32!GetUserNameA cannot read the in_out_sz");
            let len = constants::USER_NAME.len() as u64 + 1;
            if sz >= len {
                emu.maps.write_string(out_username, constants::USER_NAME);
                sz = len;
                emu.maps.write_qword(in_out_sz, sz);
            } else {
                sz = 0;
            }
        }
    }

    if sz == 0 {
        log::info!(
            "{}** {} advapi32!GetUserNameA  bad buffer or sizeptr or size!  buf: 0x{:x} szptr: 0x{:x} {}",
            emu.colors.light_red,
            emu.pos,
            out_username,
            in_out_sz,
            emu.colors.nc
        );
        emu.regs_mut().rax = constants::FALSE;
    } else {
        log::info!(
            "{}** {} advapi32!GetUserNameA buf: 0x{:x} `{}` {}",
            emu.colors.light_red,
            emu.pos,
            out_username,
            constants::USER_NAME,
            emu.colors.nc
        );
        emu.regs_mut().rax = constants::TRUE;
    }

}


fn GetUserNameW(emu: &mut emu::Emu) {
    let out_username = emu.regs().rcx;     // LPWSTR lpBuffer
    let in_out_sz = emu.regs().rdx;        // LPDWORD pcbBuffer

    log::info!(
        "{}** {} advapi32!GetUserNameW lpBuffer: 0x{:x} pcbBuffer: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        out_username,
        in_out_sz,
        emu.colors.nc
    );

    // Check if size pointer is valid
    if in_out_sz == 0 || !emu.maps.is_mapped(in_out_sz) {
        log::info!("{}** {} GetUserNameW: Invalid pcbBuffer pointer {}", 
                  emu.colors.light_red, emu.pos, emu.colors.nc);
        emu.regs_mut().rax = constants::FALSE;
        return;
    }

    // Read current buffer size (in characters)
    let buffer_size = emu.maps.read_dword(in_out_sz).expect("Cannot read buffer size") as usize;
    
    // Calculate required size in characters (including null terminator)
    let username_chars = constants::USER_NAME.chars().count();
    let required_size = username_chars + 1; // +1 for null terminator

    // Always update the size to show required characters
    emu.maps.write_dword(in_out_sz, required_size as u32);

    // Check if output buffer is valid
    if out_username == 0 || !emu.maps.is_mapped(out_username) {
        log::info!("{}** {} GetUserNameW: Invalid lpBuffer pointer {}", 
                  emu.colors.light_red, emu.pos, emu.colors.nc);
        emu.regs_mut().rax = constants::FALSE;
        return;
    }

    // Check if buffer is large enough
    if buffer_size < required_size {
        log::info!(
            "{}** {} GetUserNameW: Buffer too small. Required: {}, Provided: {} {}",
            emu.colors.light_red,
            emu.pos,
            required_size,
            buffer_size,
            emu.colors.nc
        );
        emu.regs_mut().rax = constants::FALSE;
        return;
    }

    // Buffer is large enough, write the username
    emu.maps.write_wide_string(out_username, constants::USER_NAME);

    log::info!(
        "{}** {} GetUserNameW returning: '{}' (size: {}) {}",
        emu.colors.light_red,
        emu.pos,
        constants::USER_NAME,
        required_size,
        emu.colors.nc
    );

    emu.regs_mut().rax = constants::TRUE;
}
