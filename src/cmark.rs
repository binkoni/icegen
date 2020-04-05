#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use std::os::raw::c_char;
use std::ffi::CString;
use std::os::raw::c_ulong;

pub fn md_to_html(md: &str) -> String {
    unsafe {
        let ptr = cmark_markdown_to_html(CString::new(md).unwrap().as_ptr() as *const c_char, md.len() as c_ulong, 0);
        CString::from_raw(ptr).into_string().unwrap()
    }
}
