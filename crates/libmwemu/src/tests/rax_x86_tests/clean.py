'''
    x86 test conversor from rax to mwemu
    @sha0coder

    I created those regex manually to modify all the rax 
    x86's tests to use mwemu api.
    It's not necesary to launch this script again because
    the tests where already converted.

'''

import os
import re
import sys

root_dir = sys.argv[1] 

class Fix:

    def __init__(self):
        self.data = []
        self._debug = False

    def debug(self):
        self._debug = True

    def no_debug(self):
        self._debug = False

    def write(self, data):
        self.data = data.split('\n')

    def contains(self, data):
        for l in self.data:
            if data in l:
                return True
        return False

    def read(self):
        return '\n'.join(self.data)

    def replace_line(self, a, b):
        '''
            If a is matched, the whole line is replaced by b.
            for doing partial replacements use sub
        '''
        for i in range(len(self.data)):
            if not self.data[i].startswith('// ') and \
                    not self.data[i].startswith('fn ') and \
                    not self.data[i].startswith('    // '):
                if re.search(a, self.data[i]):
                    self.data[i] = b

    def sub(self, a, b):
        '''
            Line based sub regex to regex or regex to string.
            since the code is sotred in an array there is no \n bytes.
        '''
        for i in range(len(self.data)):
            if not self.data[i].startswith('//') and \
                    not self.data[i].startswith('fn ') and \
                    not self.data[i].startswith('    // '):
                self.data[i] = re.sub(a, b, self.data[i])
                if '\1' in self.data[i] or '\2' in self.data[i] or '\3' in self.data[i]:
                    print(f"/!\\ SOH detected: {a} -> {b}")
                    sys.exit(1)

    def sub_all(self, a, b):
        '''
            This puts all the code array in one string with \n
            And apply the sub there.
        '''
        tmp = self.read()
        tmp = re.sub(a, b, tmp, flags=re.MULTILINE)
        if '\1' in tmp or '\2' in tmp  or '\3' in tmp:
            print(f"/!\\ SOH detected: {a} -> {b}")
        self.write(tmp)

    def add(self, a, b):
        '''
            If matches a, then concat b.
            Usually for creating a new line after a.
        '''
        for i in range(len(self.data)):
            if not self.data[i].startswith('//'):
                if re.search(a, self.data[i]):
                    self.data[i] += b

    def clean(self, a):
        '''
            If matches remove the whole line.
        '''
        self.data = [d for d in self.data if not re.search(a, d)]

    def find(self, patt):
        return self.read().find(patt)

    def search(self, patt):
        for d in self.data:
            if re.search(patt, d):
                return True
        return False

    def search_all(self, patt):
        re.search(patt, '\n'.join(self.data))





def fix_file(file_path):
    print(f"Fixing {file_path}...")
    fix = Fix()
    with open(file_path, 'r') as f:
        fix.write(f.read())

    # 0. Cleanup specific imports and legacy code

    off = fix.find('\ruse')
    if off >= 0:
        print(f'found off: {off} {file_path}')
        sys.exit(1)

    if fix.contains('TestCase::from'):
        fix.sub(r'use crate::common.*;', r'use crate::*;\nuse crate::tests::rax_x86_tests::common::*;')
    else:
        fix.sub(r'use crate::common.*;', r'use crate::*;')
    fix.sub_all(r'use crate::\*;\nuse crate::\*;', 'use crate::*;')



    # temporal fix:
    #fix.sub(r'use crate::tests::rax_x86_test::','use crate::tests::rax_x86_tests::')
    #fix.add(r'use crate::', '\nuse crate::tests::rax_x86_tests::common::TestCase;')
    


    # code blocks in one line  (verify that splitted well)
    fix.clean(r'^    // [A-Z][a-z]')
    fix.sub_all(r'; }', ';\n}')
    fix.sub_all(r'\); ([a-z])', r');\n \1') 
    fix.sub_all(r'{ ([a-z])', r'{\n \1')
    #fix.sub(r'^const DATA_ADDR', 'pub const DATA_ADDR')
    #fix.clean('pub const DATA_ADDR: u64 = 0x2000;')
    fix.clean('use std::sync::Arc;')


    # expand, only trigger once



    # 2. fix run variants
    '''
        ~/s/m/c/l/s/t/rax_x86_tests ❯❯❯ find . -name '*.rs' | grep -rsn 'run_until' . | grep -v 'unwrap' | cut -d ':' -f 3 | sort -u
        # 2 cases of run vcpu.run or run_until and match vcpu.run
        assert!(run_until_hlt(&mut vcpu).is_err());
        fix.sub(r'let vm = run_until_hlt(vm);',
    fn run_until_hlt(vcpu
    /// Legacy run_until_hlt that takes and returns VM (for tests using old API)
        let regs = run_until_hlt(&mut vcpu);
            let result = run_until_hlt(&mut vcpu);
        let result = run_until_hlt(&mut vcpu);
            let _ = run_until_hlt(&mut vcpu);
        let _ = run_until_hlt(&mut vcpu);
        let vm1 = run_until_hlt(vm1);
        let vm2 = run_until_hlt(vm2);
            let vm = run_until_hlt(vm);
        let vm = run_until_hlt(vm);
    pub fn run_until_hlt_legacy(mut vm
    pub fn run_until_hlt(vcpu
    use crate
        vm = run_until_hlt(vm);
    '''

    fix.replace_line(r'    run_until', '    emu.run(None).unwrap();')
    fix.replace_line(r'= run_until', '    emu.run(None).unwrap();')
    fix.replace_line(r'assert!\(run_until', '    emu.run(None).unwrap();')
    fix.replace_line(r'run_until_hlt\(', '    emu.run(None).unwrap();')
    fix.replace_line(r'= vcpu\.run\(', '    emu.run(None).unwrap();')
    fix.sub(r'final_emu\.regs', r'emu.regs')
    fix.sub(r': \&GuestMemoryMmap,', ': u64,')



    #content = re.sub('let mut regs = vcpu.get_regs().unwrap();', '', content)
    #content = re.sub('vcpu.set_regs(&regs).unwrap();', '', content)
    fix.sub('//!', '// ')

    fix.clean('= Registers::default();')

    # ud
    fix.sub(r'match result {', r'match result & 0xff {')
    fix.sub(r'Ok\(VcpuExit::Hlt\) =>', r'2 =>')
    fix.sub(r'Ok\(VcpuExit::Shutdown\) =>', r'0 | 1 =>')
    fix.sub(r'Err\(.\) => {},', '')

    # regs
    fix.sub(r'vm\.r(.)x,', r'emu.regs().r\1x,')
    fix.sub(r'vm\.r([0-9][0-9]),', r'emu.regs().r\1,')
    fix.sub(r'vm\.r([0-9]),', r'emu.regs().r\1,')
   
    # mem
    fix.sub(r'vm\.read_memory\((.*), (.*)\)', 
            r'emu.maps.read_bytes(\1, \2).try_into().unwrap()')

    #fix.sub(r'emu\.read_bytes', 'emu.maps.read_bytes')


    #fix.sub(r'emu\.maps\.read_f(..)\(([0-9a-fA-Fx]*)\),', 
    #        r'emu.maps.read_f\1(\2).unwrap(),')

    fix.sub(r'read_mem_at_u8\([^,]*, (.*)\);', 
            r'emu.maps.read_byte(\1).unwrap();')
    fix.sub(r'read_mem_at_u16\([^,]*, (.*)\);', 
            r'emu.maps.read_word(\1).unwrap();')
    fix.sub(r'read_mem_at_u32\([^,]*, (.*)\);', 
            r'emu.maps.read_dword(\1).unwrap();')

    fix.sub(r'read_mem_at_u64\([^,]*, ([^)]*)\)', 
            r'emu.maps.read_qword(\1).unwrap()')


    # assert_eq!(read_mem_at_u64(&vm, 0x1000 - 32), 0x44444444);



    fix.sub(r'read_mem_at_u128\([^,]*, (.*)\);', 
            r'emu.maps.read_u128_le(\1).unwrap();')
    fix.sub(r'read_mem_at_f32\([^,]*, (.*)\);', 
            r'f32::from_bits(emu.maps.read_dword(\1).unwrap());')
    fix.sub(r'read_mem_at_f64\([^,]*, (.*)\);', 
            r'f64::from_bits(emu.maps.read_qword(\1).unwrap());')

    fix.sub(r'read_u8\(([^,]*), ([^)]*)\)', r'emu.maps.read_byte(\2).unwrap()')
    fix.sub(r'read_u16\(([^,]*), ([^)]*)\)', r'emu.maps.read_word(\2).unwrap()')
    fix.sub(r'read_u32\(([^,]*), ([^)]*)\)', r'emu.maps.read_dword(\2).unwrap()')
    fix.sub(r'read_u64\(([^,]*), ([^)]*)\)', r'emu.maps.read_qword(\2).unwrap()')
    fix.sub(r'read_u128\(([^,]*), ([^)]*)\)', r'emu.maps.read_128bits_le(\2).unwrap()')
    fix.sub(r'read_i8\(([^,]*), ([^)]*)\)', r'emu.maps.read_byte(\2).unwrap() as i8')
    fix.sub(r'read_i16\(([^,]*), ([^)]*)\)', r'emu.maps.read_word(\2).unwrap() as i16')
    fix.sub(r'read_i32\(([^,]*), ([^)]*)\)', r'emu.maps.read_dword(\2).unwrap() as i32')
    fix.sub(r'read_i64\(([^,]*), ([^)]*)\)', r'emu.maps.read_qword(\2).unwrap() as i64')
    fix.sub(r'read_i128\(([^,]*), ([^)]*)\)', r'emu.maps.read_128bits_le(\2).unwrap() as i128')

    fix.sub(r'read_bcd\(([^,]*), ([^)]*)\)', r'emu.maps.read_bytes(\2, 10)')

    fix.sub(r'parse_bcd\(\&([^)]*)\)', r'parse_bcd((\1).try_into().unwrap())')

    # write
    fix.sub(r'write_i8\(([^,]*), ([^,]*), ([^)]*)\)', r'emu.maps.write_byte(\2, (\3) as i8 as u8)')
    fix.sub(r'write_i16\(([^,]*), ([^,]*), ([^)]*)\)', r'emu.maps.write_word(\2, (\3) as i16 as u16)')
    fix.sub(r'write_i32\(([^,]*), ([^,]*), ([^)]*)\)', r'emu.maps.write_dword(\2, (\3) as i32 as u32)')
    fix.sub(r'write_i64\(([^,]*), ([^,]*), ([^)]*)\)', r'emu.maps.write_qword(\2, (\3) as i64 as u64)')
    fix.sub(r'write_u8\(([^,]*), ([^,]*), ([^)]*)\)', r'emu.maps.write_byte(\2, \3)')
    fix.sub(r'write_u16\(([^,]*), ([^,]*), ([^)]*)\)', r'emu.maps.write_word(\2, \3)')
    fix.sub(r'write_u32\(([^,]*), ([^,]*), ([^)]*)\)', r'emu.maps.write_dword(\2, \3)')
    fix.sub(r'write_u64\(([^,]*), ([^,]*), ([^)]*)\)', r'emu.maps.write_qword(\2, \3)')



    # write_mem 
    fix.sub(r'write_mem_at_u8\([^,]*, (.*?), (.*?)\);', r'emu.maps.write_byte(\1, \2);')
    fix.sub(r'write_mem_at_u16\([^,]*, (.*?), (.*?)\);', r'emu.maps.write_word(\1, \2);')
    fix.sub(r'write_mem_at_u32\([^,]*, (.*?), (.*?)\);', r'emu.maps.write_dword(\1, \2);')
    fix.sub(r'write_mem_at_u64\([^,]*, (.*?), (.*?)\);', r'emu.maps.write_qword(\1, \2);')

    fix.sub(r'write_bcd\([^,]*, (.*), (.*)\);', r'emu.maps.write_bytes_slice(\1, \2);')

    fix.sub(r'write_mem_u8\(&[a-z_]+, (.*?)\);', r'emu.maps.write_byte(DATA_ADDR, \1);')
    fix.sub(r'write_mem_u16\(&[a-z_]+, (.*?)\);', r'emu.maps.write_word(DATA_ADDR, \1);')
    fix.sub(r'write_mem_u32\(&[a-z_]+, (.*?)\);', r'emu.maps.write_dword(DATA_ADDR, \1);')
    fix.sub(r'write_mem_u64\(&[a-z_]+, (.*?)\);', r'emu.maps.write_qword(DATA_ADDR, \1);')
    fix.sub(r'write_mem_u128\(&[a-z_]+, (.*?)\);', r'emu.maps.write_u128(DATA_ADDR, \1);')

    fix.sub(r'write_mm_via_mem\(\&[a-z_]*, (.*), (.*)\);',
        r'emu.maps.write_qword(\1, \2);')

    #fix.sub(r'write_bounds_32(\&mem,

    fix.sub(r'read_mem_u8\(&[a-z_]+\)', r'emu.maps.read_byte(DATA_ADDR).unwrap()')
    fix.sub(r'read_mem_u16\(&[a-z_]+\)', r'emu.maps.read_word(DATA_ADDR).unwrap()')
    fix.sub(r'read_mem_u32\(&[a-z_]+\)', r'emu.maps.read_dword(DATA_ADDR).unwrap()')
    fix.sub(r'read_mem_u64\(&[a-z_]+\)', r'emu.maps.read_qword(DATA_ADDR).unwrap()')
    fix.sub(r'read_mem_u128\(&[a-z_]+\)', r'emu.maps.read_u128(DATA_ADDR).unwrap()')

    # write_slice


    fix.sub_all(r'(mem|vm)\.write_slice\(([^,]*),\n? *(vm_memory::)?GuestAddress\(([^)]*)\)\)\n? *\.unwrap\(\);',
            r'emu.maps.write_bytes_slice(\4, \2);')


    fix.sub_all(r'(mem|vm)\.write_slice\(([^,]*),\n *([^,]*),\n *(vm_memory::)?GuestAddress\(([^)]*)\)\)\.unwrap\(\);',
        r'emu.maps.write_bytes_slice(\5, \2, \3);')


    

    # read_slice

    fix.sub(r'mem\.read_slice\(\&mut (.*), (vm_memory::)?GuestAddress\((.*)\)\)\.unwrap\(\);',
            r'emu.maps.read_bytes_buff(&mut \1, \3);')

    # float
    fix.sub(r'write_f(32|64)\(([^,]*), ', r'emu.maps.write_f\1(')
    #fix.sub(r'= read_f(32|64)\(\&[^,]*, (.*)\);',  r'= emu.maps.read_f\1(\2).unwrap();') 

    fix.sub(r'read_f(64|32)\(\&([^,]*), ([^)]*)\)',
            r'emu.maps.read_f\1(\3).unwrap()')

    #fix.sub(r'\(read_f([0-9][0-9])\(&mem, (.*)\);', r'(emu.maps.read_f\1(\2).unwrap();') 
    #fix.sub(r'!\\\(emu\.maps\.', r'!(emu.maps.') 


    # clean remaining stuff
    fix.clean(r'let mut regs = vcpu.get_regs\(\).unwrap\(\);')
    fix.clean(r'vcpu.set_regs\(&regs\).unwrap\(\);')
    fix.clean(r'let regs = vcpu.get_regs\(\).unwrap\(\);')
    fix.clean(r'let mut regs = Registers::default\(\);')
    fix.clean(r'let mut regs = cpu::Registers::default\(\);')
    fix.clean(r'let mut regs = rax::cpu::Registers::default\(\);')
    fix.clean(r'use crate::common::{DATA_ADDR,.*?};')
    fix.clean(r'use vm_memory::{Bytes, (vm_memory::)?GuestAddress};')
    fix.clean(r'use vm_memory::.*;')
    fix.clean(r'use rax::.*')

    # unbalanced parenthesis
    # fix.sub(r'emu\.maps\.write_bytes\(ALIGNED_ADDR\)\)',
    #        r'emu.maps.write_bytes(ALIGNED_ADDR)')


    # Load code bytes
    fix.sub(r'let .* = setup_vm\(\&([a-zA-Z0-9_]+),.*;', r'emu.load_code_bytes(&\1);')
    fix.sub(r'let .* = setup_vm_compat\(\&([a-zA-Z0-9_]+),.*;', r'emu.load_code_bytes(&\1);')


    fix.sub(r'otherwise 0', '// otherwise 0') # expansivo


    # flags
    #fix.sub(r'([a-z]{1,2})_set\(emu\.flags\(\)\.dump\(\)\)', r'emu.flags().f_\1')
    fix.sub(r'([a-z]{1,2})_set\([a-z0-9_]*regs\d?\.rflags\)', r'emu.flags().f_\1')
    fix.sub(r'flags::bits::([A-Z]{2,4})', r'flags::F_\1')
    fix.sub(r'regs\d?\.rflags\s*=\s*(.*?);', r'emu.flags_mut().load(\1);')
    fix.sub(r'=\s*regs\d?\.rflags;', r'= emu.flags().dump();')
    #fix.sub(r'[a-z0-9_]*regs\d?\.rflags([\s,\)])', r'emu.flags().dump()\1')
    fix.sub(r'[a-z0-9_]*regs\.rflags', r'emu.flags().dump()')
    
    # reg-subfuncs
    fix.clean(r'fn get_r[a-z]+(vm: &crate::common::VM) -> u64 { vm.r[a-z]+ }')

    fix.sub(r'vm\.read_slice\(\&mut ([^,]*), GuestAddress\(([^)]*)\)\)\.unwrap\(\);',
            r'\1 = emu.maps.read_bytes(\2, \1.len()).try_into().unwrap();')

    # regs
    fix.sub(r'[a-z0-9_]*regs\d?\.r([a-z0-9]+)\s*=', r'emu.regs_mut().r\1 =') # set
    fix.sub(r'[a-z0-9_]*regs\d?\.r([a-z0-9]+)', r'emu.regs().r\1')

    #fix.sub(r'(?<!regs_mut\(\)\.)regs\d?\.r([a-z0-9]+)', r'emu.regs().r\1')  # get
    fix.sub(r'vm\.r([a-z0-9]+) =', r'emu.regs_mut().r\1 =')
    fix.sub(r'vm\.r([a-z0-9]+)', r'emu.regs().r\1')
    fix.sub(r'result\.r([a-z0-9]+)', r'emu.regs().r\1')

    # temporal

    # params of unused previous functions
    fix.sub_all(r'\&vm_memory::GuestMemoryMmap', 'u64')
    fix.sub_all(r': \&Arc\<GuestMemoryMmap\>', ': u64')

    # fpu

    fix.sub(r'read_st0_as_f64\(([^,]*), ([^)]*)\)', 'emu.fpu_mut().get_st(0)')

    if not fix.contains('const DATA_ADDR'):
        fix.add(r'use crate::\*;', '\nconst DATA_ADDR: u64 = 0x7000;')

    #if fix.contains('const DATA_ADDR'):
    #fix.sub_all(r'(  *fn [^{]*{)\n(?!\s*TestCase)', r'\1\n    let mut emu = emu64();') 
    fix.sub_all(r'(fn [^{]*{)\n(?!\s*TestCase)', r'\1\n    let mut emu = emu64();\n') 
    #fix.sub_all(r'^(#\[test\] fn [^{]*{)\n(?!\s*TestCase)', r'\1\n    let mut emu = emu64();') 

    # (vm_memory::)?GuestAddress(someaddr)
    fix.clean(r'assert!\(matches!\(exit_reason, VcpuExit::Hlt\)')

    fix.clean(r'assert!\(result\.is_ok\(\)')

    # tryinto trait
    if fix.contains('parse_bcd') or fix.contains('try_into'):
        fix.add('use crate::*','\nuse std::convert::TryInto;')

    #fix.sub(r'(?<!regs_mut\(\)\.)regs\d?\.r([a-z0-9]+)', r'emu.regs().r\1', content)

    with open(file_path, 'w') as f:
        f.write(fix.read())


for root, dirs, files in os.walk(root_dir):
    for file in files:
        if file.endswith(".rs"):
            fix_file(os.path.join(root, file))
