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
    
    // Calculate sizes like the Python version
    let ws_len = (char_count + 1) * 2;  // Wide chars + null terminator
    let total_alloc_size = 4 + ws_len;  // 4-byte length prefix + string data
    
    // Allocate memory
    let bstr = emu.maps.alloc(total_alloc_size + 100)
        .expect("oleaut32!SysAllocStringLen out of memory");

    let name = format!("alloc_{:x}", bstr);
    emu.maps.create_map(&name, bstr, total_alloc_size + 100);
    
    if str_ptr == 0 {
        // Handle null input - just write the length prefix
        let length_bytes = (char_count * 2) as u32;
        emu.maps.write_dword(bstr, length_bytes);
    } else {
        // Read the input string
        let input_string = emu.maps.read_wide_string_n(str_ptr, char_count as usize);
        
        // Truncate to requested length and add null terminator
        let mut truncated = input_string;
        if truncated.len() > char_count as usize {
            truncated.truncate(char_count as usize);
        }
        truncated.push('\0');
        
        // Write length prefix (byte count, not char count)
        let byte_count = (char_count * 2) as u32;
        emu.maps.write_dword(bstr, byte_count);
        
        // Write the wide string data
        emu.maps.write_wide_string(bstr + 4, &truncated);
    }
    
    let return_ptr = bstr + 4;  // Return pointer to string data (after length prefix)
    
    log::info!("{}** {} SysAllocStringLen returning: 0x{:x} (base: 0x{:x})", 
        emu.colors.light_red, emu.pos, return_ptr, bstr);
    
    emu.regs_mut().rax = return_ptr;
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
