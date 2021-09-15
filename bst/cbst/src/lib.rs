use libc::{c_void, size_t};
use std::mem::MaybeUninit;

#[repr(C)]
pub struct CBstSet {
    head: *mut c_void,
    len: size_t,
}

extern "C" {
    fn cbst_init(cbst: *mut CBstSet);
    fn cbst_free(cbst: *mut CBstSet);
    fn cbst_contains(cbst: *mut CBstSet, key: i64) -> bool;
    fn cbst_insert(cbst: *mut CBstSet, key: i64) -> bool;
    fn cbst_remove(cbst: *mut CBstSet, key: i64) -> bool;
}

impl CBstSet {
    pub fn new() -> Self {
        let mut cbst: MaybeUninit<CBstSet> = MaybeUninit::uninit();
        unsafe {
            cbst_init(cbst.as_mut_ptr());
            cbst.assume_init()
        }
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    #[inline]
    pub fn contains(&self, key: i64) -> bool {
        unsafe { cbst_contains(self as *const CBstSet as *mut CBstSet, key) }
    }

    #[inline]
    pub fn insert(&mut self, key: i64) -> bool {
        unsafe { cbst_insert(self as *mut CBstSet, key) }
    }

    #[inline]
    pub fn remove(&mut self, key: i64) -> bool {
        unsafe { cbst_remove(self as *mut CBstSet, key) }
    }
}

impl Drop for CBstSet {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            cbst_free(self as *mut CBstSet);
        }
    }
}
