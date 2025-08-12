use crate::emu;
use crate::serialization;
use crate::winapi::winapi64::kernel32;
//use crate::winapi::winapi32::helper;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    match api.as_str() {
        "SysAllocStringLen" => SysAllocStringLen(emu),
        "SysReAllocStringLen" => SysReAllocStringLen(emu),
        "SysFreeString" => SysFreeString(emu),
        "VariantClear" => VariantClear(emu),

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

fn SysAllocStringLen(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let char_count = emu.regs().rdx;

    log::info!(
        "{}** {}:{:x} oleaut32!SysAllocStringLen str_ptr: 0x{:x} size: {}",
        emu.colors.light_red,
        emu.pos,
        emu.regs().rip,
        str_ptr,
        char_count
    );
    
    // Handle special case
    let actual_char_count = if char_count == 0xffffffff { 
        1024  // or calculate actual length from str_ptr
    } else { 
        char_count 
    };
    
    let byte_length = actual_char_count * 2;  // Wide chars are 2 bytes
    let total_size = 4 + byte_length + 2;     // length prefix + data + null terminator
    
    let base = emu.maps.alloc(total_size + 100)
        .expect("oleaut32!SysAllocStringLen out of memory");

    let name = format!("alloc_{:x}", base);
    emu.maps.create_map(&name, base, total_size + 100);
    
    // Write the byte length (not character count) at the beginning
    emu.maps.write_dword(base, byte_length as u32);
    
    let bstr_ptr = base + 4;
    
    // watch out for null?
    if str_ptr != 0 {
        emu.maps.memcpy(bstr_ptr, str_ptr, byte_length as usize);
    }
    
    // Write null terminator
    emu.maps.write_word(bstr_ptr + byte_length, 0);
    
    emu.regs_mut().rax = bstr_ptr;  // Return pointer to string data, not base
}

fn SysFreeString(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;

    log::info!(
        "{}** {} oleaut32!SysFreeString  0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        str_ptr,
        emu.colors.nc
    );

    //emu.maps.free(&format!("alloc_{:x}", str_ptr));
}

/*
INT SysReAllocStringLen(
  [in, out]      BSTR          *pbstr,
  [in, optional] const OLECHAR *psz,
  [in]           unsigned int  len
);
*/
fn SysReAllocStringLen(emu: &mut emu::Emu) {
    let pbstr_ptr = emu.regs().rcx;
    let psz = emu.regs().rdx;
    let len = emu.regs().r8;

    log::info!(
        "{}** {} oleaut32!SysReAllocStringLen pbstr_ptr: 0x{:x} psz: 0x{:x} len: {}",
        emu.colors.light_red,
        emu.pos,
        pbstr_ptr,
        psz,
        len
    );

    // Check if pbstr_ptr is NULL
    if pbstr_ptr == 0 {
        emu.regs_mut().rax = 0; // Return FALSE
        return;
    }

    let size = (len + 1) * 2; // Size in bytes (UTF-16 characters + null terminator)
    let total_size = size + 8; // Add metadata size

    // Allocate new memory
    let new_base = emu
        .maps
        .alloc(total_size + 100)
        .expect("oleaut32!SysReAllocStringLen out of memory");

    // Create new memory map
    let name = format!("alloc_{:x}", new_base);
    emu.maps.create_map(&name, new_base, total_size + 100);

    // Copy data from psz if it's not NULL
    if psz != 0 {
        emu.maps.memcpy(new_base + 8, psz, len as usize * 2);
    }

    // Free old string (reading old BSTR pointer from pbstr_ptr)
    let old_bstr = emu.maps.read_qword(pbstr_ptr).unwrap_or(0);
    if old_bstr != 0 {
        // Optional: Free the old allocation if needed
        // emu.maps.free(&format!("alloc_{:x}", old_bstr - 8));
    }

    // Update the BSTR pointer
    emu.maps.write_qword(pbstr_ptr, new_base + 8);

    log::info!(
        "{}** {} oleaut32!SysReAllocStringLen allocated new string at 0x{:x} size: {} {}",
        emu.colors.light_red,
        emu.pos,
        new_base + 8,
        size,
        emu.colors.nc
    );

    emu.regs_mut().rax = 1; // Return TRUE for success
}

/*
HRESULT VariantClear(
  [in, out] VARIANTARG *pvarg
);
*/
fn VariantClear(emu: &mut emu::Emu) {
    let pvarg = emu.regs().rcx;

    log::info!(
        "{}** {} oleaut32!VariantClear pvarg: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        pvarg,
        emu.colors.nc
    );

    // TODO: do something

    emu.regs_mut().rax = 0; // S_OK
}
