#![no_std]

#![feature(core)]
#![feature(no_std)]
#![feature(lang_items)]
#![feature(intrinsics)]

extern crate core;
extern crate rlibc;

use core::str::StrExt;

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() {}

#[no_mangle]
pub extern "C" fn hello_rust() -> *const u8 {
    "Hello, world!\0".as_ptr()
}

/// `fill_slice` fills up a `buffer` with "Hello, world!"
///
/// # Unsafe
///
/// This function assumes that you've allocated at least fourteen bytes of memory at `buffer`. If
/// you haven't, bad things may happen.
#[no_mangle]
pub unsafe extern "C" fn fill_slice(buffer: *mut u8) {
    let s = "Hello, world!\0";

    rlibc::memcpy(buffer, s.as_ptr(), 14);
}

#[no_mangle]
pub extern "C" fn rust_example_init() { }
