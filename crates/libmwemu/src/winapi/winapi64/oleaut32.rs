use crate::emu;
use crate::constants;
use crate::maps::mem64::Permission;
use crate::serialization;
use crate::winapi::winapi64::kernel32;
//use crate::winapi::helper;

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
        "{}** {}:{:x} oleaut32!SysAllocStringLen str_ptr: 0x{:x} size: {} {}",
        emu.colors.light_red,
        emu.pos,
        emu.regs().rip,
        str_ptr,
        char_count,
        emu.colors.nc,
    );
    
    // Calculate sizes like the Python version
    let ws_len = (char_count + 1) * 2;  // Wide chars + null terminator
    let total_alloc_size = 4 + ws_len;  // 4-byte length prefix + string data
    
    // Allocate memory
    let bstr = emu.maps.alloc(total_alloc_size + 100)
        .expect("oleaut32!SysAllocStringLen out of memory");

    let name = format!("alloc_{:x}", bstr);
    emu.maps.create_map(&name, bstr, total_alloc_size + 100, Permission::READ_WRITE);
    
    if str_ptr == 0 {
        // Handle null input - just write the length prefix
        let length_bytes = (char_count * 2) as u32;
        emu.maps.write_dword(bstr, length_bytes);

        // Zero out the string data area
        for i in 0..=char_count {  // <= to include null terminator
            emu.maps.write_word(bstr + 4 + (i * 2), 0);
        }
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
    
    log::info!("{}** {} SysAllocStringLen returning: 0x{:x} (base: 0x{:x}) {}", 
        emu.colors.light_red, emu.pos, return_ptr, bstr, emu.colors.nc);
    
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

    if str_ptr == 0 {
        // NULL pointer - nothing to free (this is valid behavior)
        return;
    }

    // BSTR pointer points to string data, but allocation starts 4 bytes earlier (length prefix)
    let alloc_base = str_ptr - 4;
    
    // Read the length from the prefix to know how much to zero out
    if let Some(length_bytes) = emu.maps.read_dword(alloc_base) {
        let total_size = 4 + length_bytes as u64 + 2; // prefix + string + null terminator
        let string_length = length_bytes / 2; // Convert bytes to characters
        
        log::info!("{}** {} SysFreeString zeroing {} bytes starting at 0x{:x} (string data was {} bytes, {} chars) {}", 
            emu.colors.light_red, 
            emu.pos, 
            total_size,      // Total allocation size
            alloc_base,      // Base address
            length_bytes,    // String data in bytes
            string_length,   // String length in characters
            emu.colors.nc
        );
        
        // Zero out the entire BSTR allocation
        for i in 0..total_size {
            emu.maps.write_byte(alloc_base + i, 0);
        }
    } else {
        panic!(
            "{}** {} SysFreeString: Could not read length prefix at 0x{:x} {}",
            emu.colors.light_red,
            emu.pos,
            alloc_base,
            emu.colors.nc
        );
    }

    // Optionally, you could also try to free the map by name:
    // emu.maps.free(&format!("alloc_{:x}", alloc_base));
    // or
    // emu.maps.free(&format!("bstr_{:x}", alloc_base));
}

/*
INT SysReAllocStringLen(
  [in, out]      BSTR          *pbstr,
  [in, optional] const OLECHAR *psz,
  [in]           unsigned int  len
);
*/
fn SysReAllocStringLen(emu: &mut emu::Emu) {
    let pbstr_ptr = emu.regs().rcx;  // Pointer to BSTR*
    let psz = emu.regs().rdx;        // Source string (can be NULL)
    let len = emu.regs().r8;         // Length in characters

    log::info!(
        "{}** {} oleaut32!SysReAllocStringLen pbstr_ptr: 0x{:x} psz: 0x{:x} len: {} {}",
        emu.colors.light_red,
        emu.pos,
        pbstr_ptr,
        psz,
        len,
        emu.colors.nc
    );

    // Check if pbstr_ptr is NULL
    if pbstr_ptr == 0 {
        emu.regs_mut().rax = constants::FALSE;
        return;
    }

    // Read the current BSTR pointer (might be NULL for first allocation)
    let old_bstr = emu.maps.read_qword(pbstr_ptr).unwrap_or(0);
    
    // Log old content if it exists
    if old_bstr != 0 {
        let old_alloc_base = old_bstr - 4;
        let old_len_bytes = emu.maps.read_dword(old_alloc_base).unwrap_or(0);
        let old_len_chars = old_len_bytes / 2;
        if old_len_chars > 0 {
            let old_string = emu.maps.read_wide_string_n(old_bstr, old_len_chars as usize);
            log::info!("{}** {} Old BSTR content: \"{}\" (length: {} chars) {}", 
                       emu.colors.light_red, emu.pos, old_string, old_len_chars, emu.colors.nc);
        }
    }

    // Log new source string if provided
    if psz != 0 && len > 0 {
        let new_string = emu.maps.read_wide_string_n(psz, len as usize);
        log::info!("{}** {} New source string: \"{}\" (length: {} chars) {}", 
                   emu.colors.light_red, emu.pos, new_string, len, emu.colors.nc);
    }

    // Calculate allocation size
    let byte_len = len * 2;  // Length in bytes (UTF-16)
    let total_alloc_size = 4 + byte_len + 2;  // 4-byte prefix + string + null terminator
    
    // Always allocate new memory (simpler than trying to realloc)
    let new_base = emu.maps.alloc(total_alloc_size + 100)
        .expect("oleaut32!SysReAllocStringLen out of memory");
    
    let name = format!("bstr_{:x}", new_base);
    emu.maps.create_map(&name, new_base, total_alloc_size + 100, Permission::READ_WRITE);
    
    // Write length prefix (in bytes, not including null terminator)
    emu.maps.write_dword(new_base, byte_len as u32);
    
    // Copy data from source if provided
    if psz != 0 && len > 0 {
        emu.maps.memcpy(new_base + 4, psz, len as usize * 2);
    } else if old_bstr != 0 && len > 0 {
        // No new source provided, preserve existing data (truncated to new length)
        let old_alloc_base = old_bstr - 4;
        let old_len_bytes = emu.maps.read_dword(old_alloc_base).unwrap_or(0);
        let old_len_chars = old_len_bytes / 2;
        let copy_len = std::cmp::min(len, old_len_chars as u64);
        
        if copy_len > 0 {
            emu.maps.memcpy(new_base + 4, old_bstr, copy_len as usize * 2);
        }
        
        // Zero out any remaining space if new length is longer than old length
        for i in copy_len..len {
            emu.maps.write_word(new_base + 4 + (i * 2), 0);
        }
    } else {
        // Initialize to zeros (empty string)
        for i in 0..len {
            emu.maps.write_word(new_base + 4 + (i * 2), 0);
        }
    }
    
    // Write null terminator
    emu.maps.write_word(new_base + 4 + byte_len, 0);
    
    // Update the BSTR pointer to point to the string data (skip the 4-byte length prefix)
    let new_bstr = new_base + 4;
    emu.maps.write_qword(pbstr_ptr, new_bstr);
    
    // Log the final result
    if len > 0 {
        let final_string = emu.maps.read_wide_string_n(new_bstr, len as usize);
        log::info!("{}** {} Final BSTR content: \"{}\" (length: {} chars) {}", 
                   emu.colors.light_red, emu.pos, final_string, len, emu.colors.nc);
    } else {
        log::info!("{}** {} Created empty BSTR {}", emu.colors.light_red, emu.pos, emu.colors.nc);
    }
    
    log::info!(
        "{}** {} oleaut32!SysReAllocStringLen allocated new string at 0x{:x} size: {} (base: 0x{:x}) {}",
        emu.colors.light_red,
        emu.pos,
        new_bstr,
        byte_len,
        new_base,
        emu.colors.nc
    );

    // Note: In a real implementation, you'd free the old BSTR here
    // but in an emulator, we might want to keep it for debugging
    
    emu.regs_mut().rax = constants::TRUE;
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

    // Basic validation
    if pvarg == 0 || !emu.maps.is_mapped(pvarg) {
        log::info!(
            "{}** {} VariantClear: Invalid pvarg pointer {}",
            emu.colors.light_red,
            emu.pos, 
            emu.colors.nc
        );
        emu.regs_mut().rax = 0x80070057; // E_INVALIDARG
        return;
    }

    // Clear the variant by setting vt field to VT_EMPTY (0)
    // The vt field is typically at offset 0 in the VARIANT structure
    emu.maps.write_word(pvarg, 0); // VT_EMPTY = 0

    log::info!(
        "{}** {} VariantClear: Cleared variant (set vt to VT_EMPTY) {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );

    emu.regs_mut().rax = 0; // S_OK
}
