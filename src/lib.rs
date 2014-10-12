#![no_std]

#![feature(lang_items)]
#![feature(intrinsics)]

extern crate core;
use core::str::StrSlice;

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "fail_fmt"] fn fail_fmt() -> ! { loop {} }

#[no_mangle]
pub extern "C" fn hello_rust() -> *const u8 {
    "Hello, world!\0".as_ptr()
}

#[no_mangle]
pub extern "C" fn rust_example_init() { }
