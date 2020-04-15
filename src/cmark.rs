#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use std::os::raw::c_char;
use std::ffi::CString;
use std::os::raw::c_ulong;
use std::fs;
use std::path::Path;
use std::io;

pub fn md_to_html(md: &str) -> String {
    unsafe {
        let data = CString::new(md).unwrap();
        let len = md.len();
        let ptr = cmark_markdown_to_html(data.as_ptr() as *const c_char, len as c_ulong, 0);
        CString::from_raw(ptr).into_string().unwrap()
    }
}

pub fn md_to_html_file<P: AsRef<Path>>(src_path: P, dest_path: P) -> std::io::Result<()> {
    let data = fs::read_to_string(src_path)?;
    let data = md_to_html(&data);
    fs::write(dest_path, data)
}
