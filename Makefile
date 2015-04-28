default:
	cd rust; cargo build --release
	cp rust/target/release/librust_example-*.a ruby/ext/rust_example/librust_example.a
	cd ruby; bundle
	cd ruby/ext/rust_example; ruby extconf.rb; make clean; make
	cd ruby; bundle exec ruby -e "require 'rust_example'; RustExample.hello; RustExample.make_hello"
