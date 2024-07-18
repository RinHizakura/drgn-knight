use std::{
    ffi::{c_char, c_void, CString},
    ptr::null,
};

#[link(name = "drgnimpl", kind = "static")]
extern "C" {
    fn program_create() -> *const c_void;
    fn program_destroy(prog: *const c_void);
    fn object_free(obj: *const c_void);
    fn find_task(prog: *const c_void, pid: u64) -> *const c_void;
    fn deref_obj_member(obj: *const c_void, name: *const c_char) -> *const c_void;
    fn obj_addr(obj: *const c_void, out: *const u64) -> bool;
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
        Ok(Object::new(out))
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
    object: *const c_void,
}

impl Object {
    pub fn new(object: *const c_void) -> Self {
        assert!(!object.is_null());
        Object { object }
    }

    pub fn deref_member(&self, member: &str) -> Option<Object> {
        let member_cstr = CString::new(member).unwrap();
        let out = unsafe { deref_obj_member(self.object, member_cstr.as_ptr()) };

        if out.is_null() {
            return None;
        }

        Some(Object::new(out))
    }

    pub fn address_of(&self) -> Result<u64> {
        let out: u64 = 0;
        let ret = unsafe { obj_addr(self.object, &out as *const u64) };
        if ret {
            return Ok(out);
        }

        Err(())
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

impl Default for Object {
    fn default() -> Self {
        Object { object: null() }
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        unsafe {
            object_free(self.object);
        }
    }
}
