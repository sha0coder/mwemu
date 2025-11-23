use slab::Slab;
use file_handle::FileHandle;

mod file_handle;

// TODO: support more handle: registry, thread, heap, etc
enum HandleType {
    FileHandle(FileHandle),
}

struct HandleManagement {
    number_of_handle: usize,
    handle_types: Slab<HandleType>,
}

impl HandleManagement {
    pub fn new() -> Self {
        Self {
            handle_types: Slab::with_capacity(200),
            number_of_handle: 0,
        }
    }
}