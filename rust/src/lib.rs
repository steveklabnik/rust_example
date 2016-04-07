#![feature(lang_items)]
#![no_std]

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

    core::intrinsics::copy_nonoverlapping(s.as_ptr(), buffer, 14);
}

#[no_mangle]
pub extern "C" fn num() -> i32 {
    5
}

#[no_mangle]
pub extern "C" fn rust_example_init() { }

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
