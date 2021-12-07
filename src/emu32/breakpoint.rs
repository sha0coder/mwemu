
pub struct Breakpoint {
    addr:u32,
    mem_read_addr:u32,
    mem_write_addr:u32,
}

impl Breakpoint {
    pub fn new() -> Breakpoint {
        Breakpoint {
            addr: 0,
            mem_read_addr: 0,
            mem_write_addr: 0,
        }
    }

    pub fn set_bp(&mut self, addr:u32) {
        self.addr = addr;
    }

    pub fn set_mem_read(&mut self, addr:u32) {
        self.mem_read_addr = addr;
    }

    pub fn set_mem_write(&mut self, addr:u32) {
        self.mem_write_addr = addr;
    }

    pub fn get_bp(&self) -> u32 {
        return self.addr;
    }

    pub fn get_mem_read(&self) -> u32 {
        return self.mem_read_addr;
    }

    pub fn get_mem_write(&self) -> u32 {
        return self.mem_write_addr;
    }
}