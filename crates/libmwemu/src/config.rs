use std::collections::HashMap;

use crate::{constants, definitions::Definition};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub filename: String,  // filename with full path included
    pub trace_mem: bool,   // show memory operations in every step.
    pub trace_calls: bool, // trace every call
    pub trace_regs: bool,  // show all the regs in every step.
    pub trace_reg: bool,   // show value and content of a reg in every step.
    pub trace_filename: Option<String>,
    pub trace_start: u64,
    pub trace_string: bool,
    pub trace_flags: bool,
    pub reg_names: Vec<String>, // which reg to trace.
    pub verbose: u32,           // 0 only view the api, 1 api + messages, 2 asm code.
    pub console: bool,          // enable the console on specific moment?.
    pub console_num: u64,       // in which moment enable the console.
    pub loops: bool,            // loop mode count the iterations for every instruction, its slow.
    pub nocolors: bool,         // to redirecting the output to a file is better to remove colors.
    pub string_addr: u64,
    pub inspect: bool,
    pub inspect_seq: String,
    pub endpoint: bool,
    pub maps_folder: String,
    pub console2: bool,
    pub console_addr: u64,
    pub entry_point: u64,
    pub exit_position: u64,
    pub dump_on_exit: bool,
    pub dump_filename: Option<String>,
    pub code_base_addr: u64,
    pub is_64bits: bool, // 64bits mode
    pub stack_trace: bool,
    pub test_mode: bool,
    pub console_enabled: bool,
    pub skip_unimplemented: bool,
    pub stack_addr: u64,
    pub arguments: String,
    pub enable_threading: bool, // Enable multi-threading support
    pub verbose_at: Option<u64>,
    pub command: Option<String>,
    pub definitions: HashMap<u64, Definition>,
    pub entropy: bool,
    pub shellcode: bool,
    pub emulate_winapi: bool,

    // Configurable allocation cap (default 16MB). Allocations larger than this are truncated.
    pub max_alloc_size: u64,

    // Configurable environment constants (override hardcoded values in constants.rs)
    pub module_name: String,
    pub exe_name: String,
    pub user_name: String,
    pub host_name: String,
    pub temp_path: String,
    pub cwd_path: String,
    pub windows_directory: String,
    pub system_directory: String,

    // Execution limits
    pub max_instructions: Option<u64>,   // Stop emulation after N instructions
    pub timeout_secs: Option<f64>,       // Stop emulation after N seconds wall-clock time

    // Fault tracking
    pub max_faults: Option<u32>,         // Stop emulation after N faults/exceptions

    // Sleep/Wait short-circuit: if true, Sleep/Wait calls advance tick but return immediately
    pub short_circuit_sleep: bool,

    // HeapAlloc minimum padding: ensure all heap allocations are at least this size
    pub heap_alloc_min_size: u64,
    // HeapFree soft-free: if true, HeapFree marks memory as freed but doesn't deallocate
    pub heap_free_soft: bool,

    /// If true, disable the "empty code block" detector in the main `run()` loop.
    /// Some samples intentionally execute/scan zero-filled regions.
    pub allow_empty_code_blocks: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Config {
        Config {
            filename: String::new(),
            trace_mem: false,
            trace_calls: false,
            trace_regs: false,
            trace_reg: false,
            trace_filename: None,
            trace_start: 0,
            trace_string: false,
            trace_flags: false,
            reg_names: Vec::new(),
            verbose: 0,
            console: false,
            console_num: 0,
            loops: false,
            nocolors: false,
            string_addr: 0,
            inspect: false,
            inspect_seq: "".to_string(),
            endpoint: false,
            maps_folder: "".to_string(),
            console2: false,
            console_addr: 0,
            entry_point: constants::CFG_DEFAULT_BASE,
            exit_position: 0,
            dump_on_exit: false, // TODO: a way to make it false/set it through cli + lib
            dump_filename: Some("dumps/emu.bin".to_string()), // TODO: a way to set it through cli + lib
            code_base_addr: constants::CFG_DEFAULT_BASE,
            is_64bits: false,
            stack_trace: false,
            test_mode: false,
            console_enabled: false,
            skip_unimplemented: false,
            stack_addr: 0,
            arguments: "".to_string(),
            enable_threading: false, // Default to single-threaded for backward compatibility
            verbose_at: None,
            command: None,
            definitions: HashMap::new(),
            entropy: false,
            shellcode: false,
            emulate_winapi: false,
            max_alloc_size: 0xffffff, // 16MB default
            module_name: constants::MODULE_NAME.to_string(),
            exe_name: constants::EXE_NAME.to_string(),
            user_name: constants::USER_NAME.to_string(),
            host_name: constants::HOST_NAME.to_string(),
            temp_path: constants::TEMP_PATH.to_string(),
            cwd_path: constants::CWD_PATH.to_string(),
            windows_directory: constants::WINDOWS_DIRECTORY.to_string(),
            system_directory: constants::SYSTEM_DIRECTORY.to_string(),
            max_instructions: None,
            timeout_secs: None,
            max_faults: None,
            short_circuit_sleep: false,
            heap_alloc_min_size: 0,
            heap_free_soft: false,
            allow_empty_code_blocks: false,
        }
    }

    pub fn get_maps_folder(&self, filename: &str) -> String {
        let mut path = self.maps_folder.clone();
        if !path.ends_with('/') {
            path.push('/');
        }
        path.push_str(&filename);
        path
    }
}
