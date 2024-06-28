use std::ffi::{c_char, c_void, CString};

#[link(name = "drgnimpl", kind = "static")]
extern "C" {
    fn program_create() -> *const c_void;
    fn find_task(prog: *const c_void, pid: u64) -> *const c_void;
    fn find_task_member(prog: *const c_void, task: *const c_void, member: *const c_char) -> bool;
    fn program_destroy(prog: *const c_void);
}

pub struct Program {
    prog: *const c_void,
}

impl Program {
    pub fn new() -> Self {
        let prog = unsafe { program_create() };

        assert!(!prog.is_null());

        Program { prog }
    }

    pub fn find_task(&self, pid: u64) -> Object {
        Object::new(self.prog, unsafe { find_task(self.prog, pid) })
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            program_destroy(self.prog);
        }
    }
}

pub struct Object {
    prog: *const c_void,
    object: *const c_void,
}

impl Object {
    pub fn new(prog: *const c_void, object: *const c_void) -> Self {
        Object { prog, object }
    }

    pub fn find_member(&self, member: String) {
        let member_cstr = CString::new(member).unwrap();
        unsafe { find_task_member(self.prog, self.object, member_cstr.as_ptr()) };
    }
}
