default:
	cd rust; cargo build --release
	cp rust/target/release/librust_example-*.a ruby/ext/rust_example/librust_example.a
	cd ruby/ext/rust_example; ruby extconf.rb; make clean; make
	cd ruby; ruby -Ilib:ext -r rust_example -e "RustExample.hello; RustExample.make_hello"

node:
	mkdir -p node/tmp node/bin
	cd rust; cargo build --release
	cp rust/target/release/librust_example-*.a node/tmp/librust_example.a
	cd node/tmp; ar -x librust_example.a; gcc -shared *.o -o ../bin/librust_example.dylib
	cd node; npm run example;

.PHONY: node
