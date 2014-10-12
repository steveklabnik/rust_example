require "mkmf"

have_library("rust_example","rust_example_init")

create_makefile "rust_example/rust_example"
