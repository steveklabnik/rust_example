default:
	cargo build
	cp target/librust_example-*.a ext/rust_example/librust_example.a
	cd ext/rust_example; ruby extconf.rb; make clean; make
	ruby -Ilib:ext -r rust_example -e "RustExample.hello"
