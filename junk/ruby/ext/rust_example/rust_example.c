#include<stdio.h>
#include<ruby.h>

extern char *hello_rust(void);
extern void  fill_slice(char *);
extern int   num(void);

VALUE hello(void) {
    char *hello = hello_rust();
    printf("%s\n", hello);

    return Qnil;
}

VALUE make_hello() {
    // "Hello, world!".length == 14, with the null
    char *hello = (char *)malloc(sizeof(char) * 14);

    fill_slice(hello);

    printf("%s\n", hello);

    free(hello);

    return Qnil;
}

VALUE number() {
    int x = num();

    return INT2FIX(x);
}

// https://github.com/ruby/ruby/blob/trunk/README.EXT#L682
void Init_rust_example(void) {
    VALUE rust_example = rb_define_module("RustExample");
    
    rb_define_singleton_method(rust_example, "hello", hello, 0);
    rb_define_singleton_method(rust_example, "make_hello", make_hello, 0);
    rb_define_singleton_method(rust_example, "number", number, 0);
}
