#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::ffi::{CString, c_char, CStr};

use dent_parse::{Dent, Value};

static mut DENT: Option<Dent> = None;

#[no_mangle]
pub extern "C" fn dent_init() {
    unsafe {
        DENT = Some(Dent::default());
    }
}

#[no_mangle]
pub extern "C" fn dent_shutdown() {
    unsafe {
        DENT = None;
    }
}

#[no_mangle]
pub extern "C" fn dent_parse(input: *const c_char, len: usize) -> *mut Value<'static> {
    let input = unsafe { std::slice::from_raw_parts(input as *const u8, len) };
    let input = std::str::from_utf8(input).unwrap();
    let parser = unsafe { DENT.as_mut().unwrap() };
    let value = parser.parse(input).unwrap();
    Box::into_raw(Box::new(value))
}

#[no_mangle]
pub extern "C" fn dent_parse_file(path: *const c_char) -> *mut Value<'static> {
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_str().unwrap();
    let parser = unsafe { DENT.as_mut().unwrap() };
    let value = parser.parse_file(path).unwrap();
    Box::into_raw(Box::new(value))
}

#[no_mangle]
pub extern "C" fn dent_free(value: *mut Value<'static>) {
    unsafe {
        drop(Box::from_raw(value));
    }
}

#[no_mangle]
pub extern "C" fn dent_get(
    value: *const Value<'static>,
    key: *const c_char,
) -> *mut Value<'static> {
    let key = unsafe { CStr::from_ptr(key) };
    let key = key.to_str().unwrap();
    let value = unsafe { &*value };
    let value = &value[key];

    value as *const Value<'static> as *mut Value<'static>
}

#[no_mangle]
pub extern "C" fn dent_get_index(value: *const Value<'static>, index: usize) -> *mut Value<'static> {
    let value = unsafe { &*value };
    let value = &value[index];

    value as *const Value<'static> as *mut Value<'static>
}

#[no_mangle]
pub extern "C" fn dent_is_none(value: *const Value<'static>) -> bool {
    let value = unsafe { &*value };
    value.is_none()
}

#[no_mangle]
pub extern "C" fn dent_is_str(value: *const Value<'static>) -> bool {
    let value = unsafe { &*value };
    value.is_str()
}

#[no_mangle]
pub extern "C" fn dent_is_bool(value: *const Value<'static>) -> bool {
    let value = unsafe { &*value };
    value.is_bool()
}

#[no_mangle]
pub extern "C" fn dent_is_int(value: *const Value<'static>) -> bool {
    let value = unsafe { &*value };
    value.is_int()
}

#[no_mangle]
pub extern "C" fn dent_is_float(value: *const Value<'static>) -> bool {
    let value = unsafe { &*value };
    value.is_float()
}

#[no_mangle]
pub extern "C" fn dent_is_list(value: *const Value<'static>) -> bool {
    let value = unsafe { &*value };
    value.is_list()
}

#[no_mangle]
pub extern "C" fn dent_is_dict(value: *const Value<'static>) -> bool {
    let value = unsafe { &*value };
    value.is_dict()
}

#[no_mangle]
pub extern "C" fn dent_len(value: *const Value<'static>) -> usize {
    let value = unsafe { &*value };
    value.len().unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn dent_is_empty(value: *const Value<'static>) -> bool {
    let value = unsafe { &*value };
    value.is_empty()
}

#[no_mangle]
pub extern "C" fn dent_as_str(value: *const Value<'static>) -> *mut c_char {
    let value = unsafe { &*value };
    let value = value.as_str().unwrap();
    CString::new(value).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn dent_free_str(value: *mut c_char) {
    unsafe {
        drop(CString::from_raw(value));
    }
}

#[no_mangle]
pub extern "C" fn dent_as_bool(value: *const Value<'static>) -> bool {
    let value = unsafe { &*value };
    value.as_bool().unwrap()
}

#[no_mangle]
pub extern "C" fn dent_as_int(value: *const Value<'static>) -> i64 {
    let value = unsafe { &*value };
    value.as_int().unwrap()
}

#[no_mangle]
pub extern "C" fn dent_as_float(value: *const Value<'static>) -> f64 {
    let value = unsafe { &*value };
    value.as_float().unwrap()
}

#[no_mangle]
pub extern "C" fn dent_list_get(
    value: *mut Value<'static>,
    index: usize,
) -> *mut Value<'static> {
    let value = unsafe { &*value };
    let value = &value[index];

    value as *const Value<'static> as *mut Value<'static>
}

#[no_mangle]
pub extern "C" fn dent_dict_get(
    value: *mut Value<'static>,
    key: *const c_char,
) -> *mut Value<'static> {
    let key = unsafe { CStr::from_ptr(key) };
    let key = key.to_str().unwrap();
    let value = unsafe { &*value };
    let value = &value[key];

    value as *const Value<'static> as *mut Value<'static>
}

#[no_mangle]
pub extern "C" fn dent_to_str(value: *const Value<'static>) -> *mut c_char {
    let value = unsafe { &*value };
    let value = value.to_string();
    CString::new(value).unwrap().into_raw()
}
