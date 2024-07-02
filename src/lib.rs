use std::ffi::{c_char, c_void, CString};

#[link(name = "drgnimpl", kind = "static")]
extern "C" {
    fn program_create() -> *const c_void;
    fn program_destroy(prog: *const c_void);
    fn object_free(obj: *const c_void);
    fn find_task(prog: *const c_void, pid: u64) -> *const c_void;
    fn deref_obj_member(
        prog: *const c_void,
        obj: *const c_void,
        name: *const c_char,
    ) -> *const c_void;
    fn obj2num(obj: *const c_void, out: *const u64) -> bool;
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

    pub fn find_task(&self, pid: u64) -> Result<Object> {
        let out = unsafe { find_task(self.prog, pid) };
        if out.is_null() {
            return Err(());
        }
        Ok(Object::new(self.prog, out))
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            program_destroy(self.prog);
        }
    }
}

type Result<T> = std::result::Result<T, ()>;

pub struct Object {
    prog: *const c_void,
    object: *const c_void,
}

impl Object {
    pub fn new(prog: *const c_void, object: *const c_void) -> Self {
        assert!(!prog.is_null() && !object.is_null());
        Object { prog, object }
    }

    pub fn deref_member(&self, member: String) -> Result<Object> {
        let member_cstr = CString::new(member).unwrap();
        let out = unsafe { deref_obj_member(self.prog, self.object, member_cstr.as_ptr()) };

        if out.is_null() {
            return Err(());
        }

        Ok(Object::new(self.prog, out))
    }

    pub fn to_num(&self) -> Result<u64> {
        let out: u64 = 0;
        let ret = unsafe { obj2num(self.object, &out as *const u64) };
        if ret {
            return Ok(out);
        }

        Err(())
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        unsafe {
            object_free(self.object);
        }
    }
}
