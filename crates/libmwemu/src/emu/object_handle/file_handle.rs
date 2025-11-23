use crate::emu::disassemble::InstructionCache;
use crate::{
    banzai::Banzai,
    breakpoint::Breakpoints,
    colors::Colors,
    config::Config,
    definitions::{Definition, StoredContext},
    global_locks::GlobalLocks,
    hooks::Hooks,
    maps::Maps,
    pe::{pe32::PE32, pe64::PE64},
    structures::MemoryOperation,
    thread_context::ThreadContext,
};
use crate::maps::heap_allocation::O1Heap;
use std::fs::File;
use std::fs::ReadDir;
use std::fs;
use std::path::Path;
use ahash::HashMap;

const DEFAULT_PATH: &str = "D:\\malware\\temp";

struct FileHandleManagement {
    handleMagemement: HashMap<u32, FileHandle>,
}

pub struct FileHandle {
    name: String,
    file: Option<File>,
    dir: Option<ReadDir>,
    isDir: bool,
}

pub fn WindowToEmulatePath<P: AsRef<Path>>(path: P) {
    Path::new(path.as_ref()).to_path_buf();

}

impl FileHandle {
    pub fn new(name: String) -> FileHandle {
        let isDir = fs::metadata(&name).expect(format!("File or directory doesn't exist: {}", name).as_str()).is_dir();
        let file = if !isDir {Some(File::open(&name).unwrap())} else {None};
        let dir = if isDir {Some(fs::read_dir(&name).unwrap())} else {None};
        FileHandle {
            name,
            file,
            dir,
            isDir
        }
    }
}