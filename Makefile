default:
	cd rust; cargo build --release
	cp rust/target/release/librust_example*.a ruby/ext/rust_example/librust_example.a
	cd ruby/ext/rust_example; ruby extconf.rb; make clean; make
	cd ruby; ruby -Ilib:ext -r rust_example -e "RustExample.hello; RustExample.make_hello"
