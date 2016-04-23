#![no_std]
extern crate ruru;

use ruru::VM;
use ruru::Hash;
use ruru::Fixnum;
use ruru::Class;
use ruru::types::Argc;
use ruru::AnyObject;

#[no_mangle]
pub extern fn pow_3(argc: Argc, argv: *const AnyObject, _: Fixnum) -> Hash {
    let argv = VM::parse_arguments(argc, argv);
    let num = argv[0].as_fixnum().to_i64();

    let mut hash = Hash::new();

    for i in 1..num + 1 {
        hash.store(Fixnum::new(i), Fixnum::new(i.pow(3)));
    }

    hash
}


#[no_mangle]
pub extern fn initialize_my_app() {
    Class::new("Calculator").define(|itself| {
        itself.def("pow_3", pow_3);
    });
}
