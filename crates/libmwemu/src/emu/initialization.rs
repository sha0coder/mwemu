use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;
use std::time::Instant;

use atty::Stream;
use csv::ReaderBuilder;
use iced_x86::{Formatter as _, IntelFormatter};

use crate::console::Console;
use crate::emu::disassemble::InstructionCache;
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::peb::{peb32, peb64};
use crate::{
    banzai::Banzai, breakpoint::Breakpoints, colors::Colors, config::Config,
    global_locks::GlobalLocks, hooks::Hooks, maps::Maps, thread_context::ThreadContext,
};
use crate::{get_bit, kuser_shared, set_bit, structures, winapi::winapi32, winapi::winapi64};

use fast_log::appender::{Command, FastLogRecord, RecordFormat};
use crate::maps::heap_allocation::O1Heap;

pub struct CustomLogFormat;
impl RecordFormat for CustomLogFormat {
    fn do_format(&self, arg: &mut FastLogRecord) {
        match &arg.command {
            Command::CommandRecord => {
                arg.formated = format!("{}\n", arg.args);
            }
            Command::CommandExit => {}
            Command::CommandFlush(_) => {}
        }
    }
}

impl CustomLogFormat {
    pub fn new() -> CustomLogFormat {
        Self {}
    }
}

impl Emu {
    pub fn new() -> Emu {
        let mut formatter = IntelFormatter::new();
        formatter.options_mut().set_digit_separator("");
        formatter.options_mut().set_first_operand_char_index(6);
        Emu {
            formatter,
            maps: Maps::default(),
            hooks: Hooks::new(),
            exp: 0,
            break_on_alert: false,
            bp: Breakpoints::new(),
            cfg: Config::new(),
            colors: Colors::new(),
            pos: 0,
            max_pos: None,
            force_break: false,
            force_reload: false,
            tls_callbacks: Vec::new(),
            main_thread_cont: 0,
            gateway_return: 0,
            is_running: Arc::new(AtomicU32::new(0)),
            break_on_next_cmp: false,
            break_on_next_return: false,
            filename: String::new(),
            enabled_ctrlc: false,
            run_until_ret: false,
            running_script: true,
            banzai: Banzai::new(),
            mnemonic: String::new(),
            linux: false,
            now: Instant::now(),
            skip_apicall: false,
            its_apicall: None,
            last_instruction_size: 0,
            pe64: None,
            pe32: None,
            instruction: None,
            decoder_position: 0,
            memory_operations: vec![],
            rep: None,
            tick: 0,
            trace_file: None,
            base: 0,
            heap_addr: 0,
            rng: RefCell::new(rand::rng()),
            // Initialize with main thread as thread 0
            threads: vec![ThreadContext::new(0x1000)],
            current_thread_id: 0,
            global_locks: GlobalLocks::new(),
            instruction_cache: InstructionCache::new(),
            definitions: HashMap::new(),
            stored_contexts: HashMap::new(),
            entropy: 0.0,
            heap_management: None,
        }
    }

    /// This inits the 32bits stack, it's called from init_cpu() and init()
    pub fn init_stack32(&mut self) {
        // default if not set via clap args
        if self.cfg.stack_addr == 0 {
            self.cfg.stack_addr = 0x212000;
            let esp = self.cfg.stack_addr + 0x1c000 + 4;
            let ebp = self.cfg.stack_addr + 0x1c000 + 4 + 0x1000;
            self.regs_mut().set_esp(esp);
            self.regs_mut().set_ebp(ebp);
        }

        // Store register values in local variables
        let esp = self.regs().get_esp();
        let ebp = self.regs().get_ebp();

        let stack = self
            .maps
            .create_map(
                "stack",
                self.cfg.stack_addr,
                0x030000,
                Permission::READ_WRITE,
            )
            .expect("cannot create stack map");
        let stack_base = stack.get_base();
        let stack_bottom = stack.get_bottom();

        // Now do all the assertions using the local variables
        assert!(esp < ebp);
        assert!(esp > stack_base);
        assert!(esp < stack_bottom);
        assert!(ebp > stack_base);
        assert!(ebp < stack_bottom);
        assert!(stack.inside(esp));
        assert!(stack.inside(ebp));
    }

    /// This inits the 64bits stack, it's called from init_cpu() and init()
    pub fn init_stack64(&mut self) {
        let stack_size = 0x100000;

        // default if not set via clap args
        if self.cfg.stack_addr == 0 {
            self.cfg.stack_addr = 0x22a000;
            // Set up 1MB stack
            self.regs_mut().rsp = self.cfg.stack_addr + stack_size; // 1MB offset
            self.regs_mut().rbp = self.cfg.stack_addr + stack_size + 0x1000; // Extra page for frame
        }

        // Store register values in local variables
        let rsp = self.regs().rsp;
        let rbp = self.regs().rbp;

        // Add extra buffer beyond rbp to ensure it's strictly less than bottom
        let stack = self
            .maps
            .create_map(
                "stack",
                self.cfg.stack_addr,
                stack_size + 0x2000,
                Permission::READ_WRITE,
            ) // Increased size
            .expect("cannot create stack map");
        let stack_base = stack.get_base();
        let stack_bottom = stack.get_bottom();

        // Now do all the assertions using the local variables
        assert!(rsp < rbp);
        assert!(rsp > stack_base);
        assert!(rsp < stack_bottom);
        assert!(rbp > stack_base);
        assert!(rbp < stack_bottom);
        assert!(stack.inside(rsp));
        assert!(stack.inside(rbp));
    }

    //TODO: tests only in tests.rs
    pub fn init_stack64_tests(&mut self) {
        self.regs_mut().rsp = 0x000000000014F4B0;
        self.regs_mut().rbp = 0x0000000000000000;
        let stack = self.maps.get_mem_mut("stack");
        stack.set_base(0x0000000000149000);
        stack.set_size(0x0000000000007000);
    }

    //TODO: tests only in tests.rs
    pub fn init_regs_tests(&mut self) {
        self.regs_mut().rax = 0x00000001448A76A4;
        self.regs_mut().rbx = 0x000000007FFE0385;
        self.regs_mut().rcx = 0x0000000140000000;
        self.regs_mut().rdx = 0x0000000000000001;
        self.regs_mut().rsi = 0x0000000000000001;
        self.regs_mut().rdi = 0x000000007FFE0384;
        self.regs_mut().r10 = 0x000000007FFE0384;
        self.regs_mut().r11 = 0x0000000000000246;
        self.regs_mut().r12 = 0x00000001448A76A4;
        self.regs_mut().r14 = 0x0000000140000000;
    }

    //TODO: tests only in tests.rs
    pub fn init_flags_tests(&mut self) {
        self.flags_mut().clear();

        self.flags_mut().f_zf = true;
        self.flags_mut().f_pf = true;
        self.flags_mut().f_af = false;

        self.flags_mut().f_of = false;
        self.flags_mut().f_sf = false;
        self.flags_mut().f_df = false;

        self.flags_mut().f_cf = false;
        self.flags_mut().f_tf = false;
        self.flags_mut().f_if = true;

        self.flags_mut().f_nt = false;
    }

    pub fn init_logger(&mut self) {
        fast_log::init(
            fast_log::Config::new()
                .format(CustomLogFormat::new())
                .console()
                .chan_len(Some(100000)),
        )
        .unwrap();
    }


    /// Initialize windows simulator, this does like init_cpu() but also setup the windows memory.
    /// This require having the map files in place, otherwise use just init_cpu() but emu32() and
    /// emu64() already call init_cpu()
    /// This is called from load_code if the code is a PE or shellcode.
    /// load_code_bytes() and other loading ways don't call this, if you need windows simulation call this.
    pub fn init(&mut self, clear_registers: bool, clear_flags: bool) {
        self.pos = 0;

        if !atty::is(Stream::Stdout) {
            self.cfg.nocolors = true;
            self.colors.disable();
            self.cfg.console_enabled = false;
            self.disable_ctrlc();
        }

        //log::info!("initializing regs");
        if clear_registers {
            self.regs_mut().clear::<64>();
        }
        if clear_flags {
            self.flags_mut().clear();
        }
        //self.regs().rand();

        self.regs_mut().rip = self.cfg.entry_point;
        if self.cfg.is_64bits {
            self.maps.is_64bits = true;
            //self.init_regs_tests(); // TODO: not sure why this was on
            self.init_mem64();
            self.init_stack64();
            //self.init_stack64_tests();
            //self.init_flags_tests();
        } else {
            // 32bits
            self.maps.is_64bits = false;
            self.regs_mut().sanitize32();
            self.init_mem32();
            self.init_stack32();
        }

        // loading banzai on 32bits
        if !self.cfg.is_64bits {
            let mut rdr = ReaderBuilder::new()
                .from_path(format!("{}/banzai.csv", self.cfg.maps_folder))
                .expect("banzai.csv not found on maps folder, please download last mwemu maps");

            for result in rdr.records() {
                let record = result.expect("error parsing banzai.csv");
                let api = &record[0];
                let params: i32 = record[1].parse().expect("error parsing maps32/banzai.csv");

                self.banzai.add(api, params);
            }
        }

        //self.init_tests();
    }

    /// The minimum initializations necessary to emualte asm with no OS simulation.
    pub fn init_cpu(&mut self) {
        self.pos = 0;
        //self.regs_mut().clear::<64>();
        self.flags_mut().clear();

        if self.cfg.is_64bits {
            self.maps.is_64bits = true;
            self.init_stack64();
        } else {
            self.maps.is_64bits = false;
            self.regs_mut().sanitize32();
            self.init_stack32()
        }
    }

    /// Initialize linux simulation, it's called from load_code() if the sample is an ELF.
    pub fn init_linux64(&mut self, dyn_link: bool) {
        //self.regs_mut().clear::<64>();
        self.flags_mut().clear();
        self.flags_mut().f_if = true;

        let orig_path = std::env::current_dir().unwrap();
        std::env::set_current_dir(self.cfg.maps_folder.clone());
        if dyn_link {
            //self.regs_mut().rsp = 0x7fffffffe2b0;
            self.regs_mut().rsp = 0x7fffffffe790;
            self.maps
                .create_map(
                    "linux_dynamic_stack",
                    0x7ffffffde000,
                    0x100000,
                    Permission::READ_WRITE,
                )
                .expect("cannot create linux_dynamic_stack map");
            //self.maps.create_map("dso_dyn").load_at(0x7ffff7ffd0000);
            self.maps
                .create_map("dso_dyn", 0x7ffff7ffd000, 0x1000, Permission::READ_WRITE)
                .expect("cannot create dso_dyn map");
            self.maps
                .create_map(
                    "linker",
                    0x7ffff7ffd000 - 0x1000 - 0x10000,
                    0x10000,
                    Permission::READ_WRITE,
                )
                .expect("cannot create linker map");
        } else {
            self.regs_mut().rsp = 0x7fffffffe270;
            self.maps
                .create_map(
                    "linux_static_stack",
                    0x7ffffffde000,
                    0x100000,
                    Permission::READ_WRITE,
                )
                .expect("cannot create linux_static_stack map");
            self.maps
                .create_map("dso", 0x7ffff7ffd000, 0x100000, Permission::READ_WRITE)
                .expect("cannot create dso map");
        }
        let tls = self
            .maps
            .create_map("tls", 0x7ffff8fff000, 0xfff, Permission::READ_WRITE)
            .expect("cannot create tls map");
        tls.load("tls.bin");

        std::env::set_current_dir(orig_path);

        if dyn_link {
            //heap.set_base(0x555555579000);
        } else {
            // here we are allocating 4MB of heap memory
            let heap_sz = 0x885900 - 0x4b5000;
            self.heap_addr = self.maps.alloc(heap_sz).expect("cannot allocate heap");
            let heap = self
                .maps
                .create_map(".heap", self.heap_addr, heap_sz, Permission::READ_WRITE) //.create_map("heap", 0x4b5b00, 0x4d8000 - 0x4b5000)
                .expect("cannot create heap map");
            heap.load("heap.bin");

            self.heap_management = Some(Box::new(O1Heap::new(self.heap_addr, heap_sz as u32).expect("Expect new heap_management but failed")));
        }
        
        self.regs_mut().rbp = 0;

        self.fs_mut().insert(0xffffffffffffffc8, 0); //0x4b6c50
        self.fs_mut().insert(0xffffffffffffffd0, 0);
        self.fs_mut().insert(0xffffffffffffffd8, 0x4b27a0);
        self.fs_mut().insert(0xffffffffffffffa0, 0x4b3980);
        self.fs_mut().insert(0x18, 0);
        self.fs_mut().insert(40, 0x4b27a0);
    }

    /// This is called from init(), this setup the 32bits windows memory simulation.
    pub fn init_mem32(&mut self) {
        log::info!("loading memory maps");

        self.maps.is_64bits = false;

        let orig_path = std::env::current_dir().unwrap();
        std::env::set_current_dir(self.cfg.maps_folder.clone());

        //self.maps.create_map("m10000", 0x10000, 0).expect("cannot create m10000 map");
        //self.maps.create_map("m20000", 0x20000, 0).expect("cannot create m20000 map");
        //self.maps.create_map("code", self.cfg.code_base_addr, 0);

        //self.maps.write_byte(0x2c3000, 0x61); // metasploit trick

        std::env::set_current_dir(orig_path);

        peb32::init_peb(self);
        winapi32::kernel32::load_library(self, "ntdll.dll");
        let ntdll_base = self.maps.get_mem("ntdll.pe").get_base();
        peb32::update_peb_image_base(self, ntdll_base as u32);

        winapi32::kernel32::load_library(self, "kernel32.dll");
        winapi32::kernel32::load_library(self, "kernelbase.dll");
        winapi32::kernel32::load_library(self, "iphlpapi.dll");
        winapi32::kernel32::load_library(self, "ws2_32.dll");
        winapi32::kernel32::load_library(self, "advapi32.dll");
        //winapi32::kernel32::load_library(self, "comctl32.dll");
        winapi32::kernel32::load_library(self, "winhttp.dll");
        winapi32::kernel32::load_library(self, "wininet.dll");
        //winapi32::kernel32::load_library(self, "dnsapi.dll");
        winapi32::kernel32::load_library(self, "shell32.dll");
        //winapi32::kernel32::load_library(self, "shlwapi.dll");

        let teb_map = self.maps.get_mem_mut("teb");
        let mut teb = structures::TEB::load_map(teb_map.get_base(), teb_map);
        teb.nt_tib.stack_base = self.cfg.stack_addr as u32;
        teb.nt_tib.stack_limit = (self.cfg.stack_addr + 0x30000) as u32;
        teb.save(teb_map);
    }

    //TODO: tests on tests.rs
    pub fn init_tests(&mut self) {
        let mem = self
            .maps
            .create_map("test", 0, 1024, Permission::READ_WRITE_EXECUTE)
            .expect("cannot create test map");
        mem.write_qword(0, 0x1122334455667788);
        assert!(mem.read_qword(0) == 0x1122334455667788);
        self.maps.free("test");

        // some tests
        assert!(get_bit!(0xffffff00u32, 0) == 0);
        assert!(get_bit!(0xffffffffu32, 5) == 1);
        assert!(get_bit!(0xffffff00u32, 5) == 0);
        assert!(get_bit!(0xffffff00u32, 7) == 0);
        assert!(get_bit!(0xffffff00u32, 8) == 1);

        let mut a: u32 = 0xffffff00;
        set_bit!(a, 0, 1);
        set_bit!(a, 1, 1);
        set_bit!(a, 2, 1);
        set_bit!(a, 3, 1);
        set_bit!(a, 4, 1);
        set_bit!(a, 5, 1);
        set_bit!(a, 6, 1);
        set_bit!(a, 7, 1);

        assert!(a == 0xffffffff);

        set_bit!(a, 0, 0);
        set_bit!(a, 1, 0);
        set_bit!(a, 2, 0);
        set_bit!(a, 3, 0);
        set_bit!(a, 4, 0);
        set_bit!(a, 5, 0);
        set_bit!(a, 6, 0);
        set_bit!(a, 7, 0);

        assert!(a == 0xffffff00);

        /*
        remove this test because it isn't that correct
        let mut r: u64;
        (r, _) = engine::logic::shrd(self, 0x9fd88893, 0x1b, 0x6, 32);
        assert!(r == 0x6e7f6222);
        (r, _) = engine::logic::shrd(self, 0x6fdcb03, 0x0, 0x6, 32);
        assert!(r == 0x1bf72c);
        (r, _) = engine::logic::shrd(self, 0x91545f1d, 0x6fe2, 0x6, 32);
        assert!(r == 0x8a45517c);
        (r, _) = engine::logic::shld(self, 0x1b, 0xf1a7eb1d, 0xa, 32);
        assert!(r == 0x6fc6);
        (r, _) = engine::logic::shld(self, 0x1, 0xffffffff, 4, 32);
        assert!(r == 0x1f);
        (r, _) = engine::logic::shld(self, 0x1, 0xffffffff, 33, 32);
        assert!(r == 0x3);
        (r, _) = engine::logic::shld(self, 0x144e471f8, 0x14F498, 0x3e, 64);
        assert!(r == 0x53d26);
        */

        if self.maps.mem_test() {
            log::info!("memory test Ok.");
        } else {
            log::error!("It doesn't pass the memory tests!!");
            Console::spawn_console(self);
            std::process::exit(1);
        }
    }

    /// This is called from init(), this setup the 64bits windows memory simulation.
    pub fn init_mem64(&mut self) {
        log::info!("loading memory maps");
        self.maps.is_64bits = true;

        /*
        let orig_path = std::env::current_dir().unwrap();
        std::env::set_current_dir(self.cfg.maps_folder.clone());

        self.maps.create_map("m10000", 0x10000, 0).expect("cannot create m10000 map");
        self.maps.create_map("m20000", 0x20000, 0).expect("cannot create m20000 map");
        self.maps.create_map("m520000", 0x520000, 0).expect("cannot create m520000 map");
        self.maps.create_map("m53b000", 0x53b000, 0).expect("cannot create m53b000 map");
        self.maps.create_map("code", self.cfg.code_base_addr, 0);

        std::env::set_current_dir(orig_path);
        */

        peb64::init_peb(self);
        kuser_shared::init_kuser_shared_data(self);

        winapi64::kernel32::load_library(self, "ntdll.dll");
        let ntdll_base = self.maps.get_mem("ntdll.pe").get_base();
        peb64::update_peb_image_base(self, ntdll_base);

        winapi64::kernel32::load_library(self, "kernel32.dll");
        winapi64::kernel32::load_library(self, "kernelbase.dll");
        winapi64::kernel32::load_library(self, "iphlpapi.dll");
        winapi64::kernel32::load_library(self, "ws2_32.dll");
        winapi64::kernel32::load_library(self, "advapi32.dll");
        winapi64::kernel32::load_library(self, "comctl32.dll");
        winapi64::kernel32::load_library(self, "winhttp.dll");
        winapi64::kernel32::load_library(self, "wininet.dll");
        winapi64::kernel32::load_library(self, "dnsapi.dll");
        winapi64::kernel32::load_library(self, "shell32.dll");
        winapi64::kernel32::load_library(self, "shlwapi.dll");

        let teb_map = self.maps.get_mem_mut("teb");
        let mut teb = structures::TEB64::load_map(teb_map.get_base(), teb_map);
        teb.nt_tib.stack_base = self.cfg.stack_addr;
        let stack_size = 0x100000;
        teb.nt_tib.stack_limit = self.cfg.stack_addr + stack_size + 0x2000;
        teb.save(teb_map);

        let heap_sz = 0x885900 - 0x4b5000;
        self.heap_addr = self.maps.alloc(heap_sz).expect("cannot allocate heap");
        let heap = self
            .maps
            .create_map(".heap", self.heap_addr, heap_sz, Permission::READ_WRITE)
            .expect("cannot create heap map");

        self.heap_management = Some(Box::new(O1Heap::new(self.heap_addr, heap_sz as u32).expect("Expect new heap_management but failed")));
    }
}
