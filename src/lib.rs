use std::ffi::c_void;

#[link(name = "drgnimpl", kind = "static")]
extern "C" {
    fn program_create() -> *const c_void;
    fn find_task_member(prog: *const c_void, pid: u64);
    fn program_destroy(prog: *const c_void);
}

pub struct Program {
    prog: *const c_void,
}

impl Program {
    pub fn new() -> Self {
        let prog =  unsafe { program_create() };

        assert!(!prog.is_null());

        Program {
            prog,
        }
    }

    pub fn find_task_member(&self, pid: u64) {
        unsafe { find_task_member(self.prog, pid)}
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { program_destroy(self.prog); }
    }
}


