#include<stdio.h>
#include<ruby.h>

VALUE hello(void) {
    printf("Hello, world");

    return Qnil;
}

// https://github.com/ruby/ruby/blob/trunk/README.EXT#L682
void Init_rust_example(void) {
    VALUE rust_example = rb_define_module("RustExample");
    
    rb_define_singleton_method(rust_example, "hello", hello, 0);
}
