use crate::emu;
use crate::constants;
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
    let pbstr_ptr = emu.regs().rcx;  // Pointer to BSTR*
    let psz = emu.regs().rdx;        // Source string (can be NULL)
    let len = emu.regs().r8;  // Length in characters

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

    // Read the current BSTR pointer
    let old_bstr = emu.maps.read_qword(pbstr_ptr).unwrap_or(0);
    
    // Calculate sizes
    let byte_len = len * 2;  // Length in bytes (UTF-16)
    let total_alloc_size = 4 + byte_len + 2;  // 4-byte prefix + string + null terminator
    
    if old_bstr != 0 {
        // Case 1: Reallocating existing BSTR
        log::info!("{}** {} Reallocating existing BSTR at 0x{:x} {}", 
                   emu.colors.light_red, emu.pos, old_bstr, emu.colors.nc);
        
        // Log the old string content
        let old_alloc_base = old_bstr - 4;
        let old_len_bytes = emu.maps.read_dword(old_alloc_base).unwrap_or(0);
        let old_len_chars = old_len_bytes / 2;
        if old_len_chars > 0 {
            let old_string = emu.maps.read_wide_string_n(old_bstr, old_len_chars as usize);
            log::info!("{}** {} Old BSTR content: \"{}\" (length: {} chars) {}", 
                       emu.colors.light_red, emu.pos, old_string, old_len_chars, emu.colors.nc);
        } else {
            log::info!("{}** {} Old BSTR was empty {}", emu.colors.light_red, emu.pos, emu.colors.nc);
        }
        
        // Log the new source string if provided
        if psz != 0 && len > 0 {
            let new_string = emu.maps.read_wide_string_n(psz, len as usize);
            log::info!("{}** {} New source string: \"{}\" (length: {} chars) {}", 
                       emu.colors.light_red, emu.pos, new_string, len, emu.colors.nc);
        }
        
        // Free the old BSTR (old_bstr points to string data, so allocation starts at old_bstr - 4)
        // Note: In a real implementation, you'd use HeapReAlloc here
        
        // Allocate new memory
        let new_base = emu.maps.alloc(total_alloc_size + 100)
            .expect("oleaut32!SysReAllocStringLen out of memory");
        
        let name = format!("bstr_{:x}", new_base);
        emu.maps.create_map(&name, new_base, total_alloc_size + 100);
        
        // Write length prefix (in bytes, not including null terminator)
        emu.maps.write_dword(new_base, byte_len as u32);
        
        // Copy data from psz if provided, otherwise preserve old data
        if psz != 0 {
            // Copy from the provided source string
            if len > 0 {
                emu.maps.memcpy(new_base + 4, psz, len as usize * 2);
            }
        } else {
            // Copy from old BSTR (preserve existing data, but truncated to new length)
            let copy_len = std::cmp::min(len, old_len_chars as u64);
            if copy_len > 0 {
                emu.maps.memcpy(new_base + 4, old_bstr, copy_len as usize * 2);
            }
        }
        
        // Write null terminator
        emu.maps.write_word(new_base + 4 + byte_len, 0);
        
        // Update the BSTR pointer to point to the string data (skip the 4-byte length prefix)
        let new_bstr = new_base + 4;
        emu.maps.write_qword(pbstr_ptr, new_bstr);
        
        // Log the final string content
        if len > 0 {
            let final_string = emu.maps.read_wide_string_n(new_bstr, len as usize);
            log::info!("{}** {} Final BSTR content: \"{}\" (length: {} chars) {}", 
                       emu.colors.light_red, emu.pos, final_string, len, emu.colors.nc);
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
        
    } else {
        // Case 2: First allocation (*pbstr is NULL) - delegate to SysAllocStringLen
        log::info!("{}** {} First allocation (old BSTR is NULL), calling SysAllocStringLen {}", 
                   emu.colors.light_red, emu.pos, emu.colors.nc);
        
        // Log the source string if provided
        if psz != 0 && len > 0 {
            let source_string = emu.maps.read_wide_string_n(psz, len as usize);
            log::info!("{}** {} Source string: \"{}\" (length: {} chars) {}", 
                       emu.colors.light_red, emu.pos, source_string, len, emu.colors.nc);
        }
        
        // Allocate new memory
        let new_base = emu.maps.alloc(total_alloc_size + 100)
            .expect("oleaut32!SysReAllocStringLen out of memory");
        
        let name = format!("bstr_{:x}", new_base);
        emu.maps.create_map(&name, new_base, total_alloc_size + 100);
        
        // Write length prefix (in bytes)
        emu.maps.write_dword(new_base, byte_len as u32);
        
        // Copy data from psz if provided
        if psz != 0 && len > 0 {
            emu.maps.memcpy(new_base + 4, psz, len as usize * 2);
        } else {
            // Initialize to zeros
            for i in 0..len {
                emu.maps.write_word(new_base + 4 + (i * 2), 0);
            }
        }
        
        // Write null terminator
        emu.maps.write_word(new_base + 4 + byte_len, 0);
        
        // Set the BSTR pointer to point to the string data
        let new_bstr = new_base + 4;
        emu.maps.write_qword(pbstr_ptr, new_bstr);
        
        // Log the final string content
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
    }

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

    // TODO: do something

    emu.regs_mut().rax = 0; // S_OK
}
