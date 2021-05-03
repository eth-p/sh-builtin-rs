#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(improper_ctypes)] // Bash uses u128 return types.

use std::fmt::Formatter;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Allow mutable static builtin.
// Bash requires it to be mutable, and it isn't ever touched by Rust code.
unsafe impl Send for builtin {}
unsafe impl Sync for builtin {}

// Conversions.
impl From<&WORD_LIST> for Vec<String> {
    fn from(list: &WORD_LIST) -> Self {
        let mut vec: Vec<String> = vec![];

        let mut current = list as *const WORD_LIST;
        while !current.is_null() {
            let current_ref: &WORD_LIST = &unsafe { *current };
            assert!(!current_ref.word.is_null());

            vec.push(unsafe { *current_ref.word }.to_string());
            current = current_ref.next;
        }

        vec
    }
}

impl std::fmt::Display for WORD_DESC {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unsafe { std::ffi::CStr::from_ptr(self.word) }
            .to_string_lossy()
            .fmt(f)
    }
}
