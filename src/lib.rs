use std::{
    ffi::{c_char, c_void, CString},
    ptr::{self, null},
};

use anyhow::anyhow;
use anyhow::Result;

#[link(name = "drgnimpl", kind = "static")]
extern "C" {
    fn program_create() -> *const c_void;
    fn program_destroy(prog: *const c_void);
    fn object_free(obj: *const c_void);
    fn find_task(prog: *const c_void, pid: u64) -> *const c_void;
    fn find_object_variable(prog: *const c_void, name: *const c_char) -> *const c_void;
    fn get_obj_member(obj: *const c_void, name: *const c_char) -> *const c_void;
    fn deref_obj_member(obj: *const c_void, name: *const c_char) -> *const c_void;
    fn address_of(obj: *const c_void) -> *const c_void;
    fn obj2num(obj: *const c_void, out: *const u64) -> bool;
    fn obj2cstr(obj: *const c_void, out: *const *mut c_char) -> bool;
    fn container_of(ptr: *const c_void, typ: *const c_char, member: *const c_char)
        -> *const c_void;
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
            return Err(anyhow!("Fail to find_task from pid {pid}"));
        }
        Ok(Object::new(out))
    }

    pub fn find_object_variable(&self, name: &str) -> Result<Object> {
        let name_cstr = CString::new(name).unwrap();
        let out = unsafe { find_object_variable(self.prog, name_cstr.as_ptr()) };
        if out.is_null() {
            return Err(anyhow!("Fail to object which has name {name}"));
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

pub struct Object {
    object: *const c_void,
}

impl Object {
    pub fn new(object: *const c_void) -> Self {
        assert!(!object.is_null());
        Object { object }
    }

    pub fn member(&self, member: &str) -> Option<Object> {
        let member_cstr = CString::new(member).unwrap();
        let out = unsafe { get_obj_member(self.object, member_cstr.as_ptr()) };

        if out.is_null() {
            return None;
        }

        Some(Object::new(out))
    }

    pub fn deref_member(&self, member: &str) -> Option<Object> {
        let member_cstr = CString::new(member).unwrap();
        let out = unsafe { deref_obj_member(self.object, member_cstr.as_ptr()) };

        if out.is_null() {
            return None;
        }

        Some(Object::new(out))
    }

    pub fn container_of(&self, typ: &str, member: &str) -> Option<Object> {
        let typ = CString::new(typ).expect("Invalid typ");
        let member = CString::new(member).expect("Invalid member");

        let out = unsafe { container_of(self.object, typ.as_ptr(), member.as_ptr()) };

        if out.is_null() {
            return None;
        }

        Some(Object::new(out))
    }

    pub fn address_of(&self) -> Option<Object> {
        let out = unsafe { address_of(self.object) };

        if out.is_null() {
            return None;
        }

        Some(Object::new(out))
    }

    pub fn to_num(&self) -> Result<u64> {
        let out: u64 = 0;
        let ret = unsafe { obj2num(self.object, &out as *const u64) };
        if ret {
            return Ok(out);
        }

        Err(anyhow!("object can't convert to number"))
    }

    pub fn to_str(&self) -> Result<String> {
        let buf: *mut c_char = ptr::null_mut();
        let ret = unsafe { obj2cstr(self.object, &buf as *const *mut c_char) };
        if ret {
            let cstr = unsafe { CString::from_raw(buf) };
            return Ok(cstr.into_string()?);
        }

        Err(anyhow!("object can't convert to string"))
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

pub struct List {
    pos: Object,
    typ: CString,
    member: CString,
    head: u64,
    cnt: usize,
}

impl List {
    pub fn new(head: Object, typ: &str, member: &str) -> Self {
        let head = head.address_of().unwrap();
        let head_addr = head.to_num().unwrap();

        List {
            pos: head,
            typ: CString::new(typ).expect("Invalid typ to List"),
            member: CString::new(member).expect("Invalid member to List"),
            head: head_addr,
            cnt: 0,
        }
    }
}

impl Iterator for List {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.pos.deref_member("next")?;
        if next.to_num().unwrap() as u64 == self.head {
            return None;
        }

        self.pos = next;
        self.cnt += 1;
        let obj = unsafe { container_of(self.pos.object, self.typ.as_ptr(), self.member.as_ptr()) };
        return Some(Object::new(obj));
    }
}
