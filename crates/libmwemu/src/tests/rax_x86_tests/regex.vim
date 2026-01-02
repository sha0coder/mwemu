" init
" use crate::*;
" use crate::flags::Flags;

" api

%s/let mut regs = Registers::default();/let mut emu = emu64();/
%s/let regs = Registers::default();/let mut emu = emu64();/
%s/let regs2 = Registers::default();/let mut emu = emu64();/
%s/let mut regs2 = Registers::default();/let mut emu = emu64();/
%s/let regs = run_until_hlt(&mut vcpu).unwrap();/emu.run(None).unwrap();/
%s/run_until_hlt(&mut vcpu).unwrap();/emu.run(None).unwrap();/
%s/let (mut vcpu, _) = setup_vm(&code, Some(regs));/emu.load_code_bytes(\&code);/
%s/let (mut vcpu, mem) = setup_vm(&code, Some(regs));/emu.load_code_bytes(\&code);/
%s/let (mut vcpu2, _) = setup_vm(&code2, Some(regs2));/emu.load_code_bytes(\&code2);/

" regs

%s/regs.r\(.\)x = /emu.regs_mut().r\1x = /g
%s/regs.r\(.\)x/emu.regs().r\1x/g
%s/regs\.r\(..\) =/emu.regs_mut().r\1 =/g
%s/regs\.r\(..\) /emu.regs().r\1 /g
%s/regs\.r\(..\),/emu.regs().r\1,/g
%s/regs\.r\([0-9]\) =/emu.regs_mut().r\1 =/g

%s/regs2.r\(.\)x = /emu.regs_mut().r\1x = /g
%s/regs2.r\(.\)x/emu.regs().r\1x/g
%s/regs2\.r\(..\) =/emu.regs_mut().r\1 =/g
%s/regs2\.r\(..\) /emu.regs().r\1 /g
%s/regs2\.r\(..\),/emu.regs().r\1,/g
%s/regs2\.r\([0-9]\) =/emu.regs_mut().r\1 =/g

" flags

%s/\(.\)f_set(regs.rflags)/emu.flags().f_\1f/g
%s/flags::bits::\(..\)/flags::F_\1/g
%s/regs.rflags = \(.*\);/emu.flags_mut().load(\1);/g
%s/= regs\.rflags;/= emu.flags().dump();/g
%s/regs\.rflags /emu.flags().dump() /

" mem

%s/use crate::common::{DATA_ADDR, write_mem_u.., read_mem_u..};/let DATA_ADDR = 0x7000;/
%s/use crate::common::{DATA_ADDR, write_mem_u., read_mem_u.};/let DATA_ADDR = 0x7000;/

%s/write_mem_u8(&mem, \(.*\));/emu.maps.write_byte(DATA_ADDR, \1);/
%s/write_mem_u16(&mem, \(.*\));/emu.maps.write_word(DATA_ADDR, \1);/
%s/write_mem_u32(&mem, \(.*\));/emu.maps.write_dword(DATA_ADDR, \1);/
%s/write_mem_u64(&mem, \(.*\));/emu.maps.write_qword(DATA_ADDR, \1);/
%s/write_mem_u128(&mem, \(.*\));/emu.maps.write_u128(DATA_ADDR, \1);/

%s/read_mem_u8(&mem)/emu.maps.read_byte(DATA_ADDR).unwrap()/g
%s/read_mem_u16(&mem)/emu.maps.read_word(DATA_ADDR).unwrap()/g
%s/read_mem_u32(&mem)/emu.maps.read_dword(DATA_ADDR).unwrap()/g
%s/read_mem_u64(&mem)/emu.maps.read_qword(DATA_ADDR).unwrap()/g
%s/read_mem_u128(&mem)/emu.maps.read_u128(DATA_ADDR).unwrap()/g


