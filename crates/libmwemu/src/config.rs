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
    pub shellcode: bool
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
        }
    }
}
