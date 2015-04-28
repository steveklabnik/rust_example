# coding: utf-8
lib = File.expand_path('../lib', __FILE__)
$LOAD_PATH.unshift(lib) unless $LOAD_PATH.include?(lib)
require 'rust_example/version'

Gem::Specification.new do |spec|
  spec.name          = "rust_example"
  spec.version       = RustExample::VERSION
  spec.authors       = ["Steve Klabnik"]
  spec.email         = ["steve@steveklabnik.com"]
  spec.summary       = %q{A Ruby gem, implemented in Rust.}
  spec.description   = %q{A Ruby gem, implemented in the Rust programming language. http://rust-lang.org}
  spec.homepage      = "https://github.com/steveklabnik/rust_example"
  spec.license       = "MIT"

  spec.files         = `git ls-files -z`.split("\x0")
  spec.executables   = spec.files.grep(%r{^bin/}) { |f| File.basename(f) }
  spec.test_files    = spec.files.grep(%r{^(test|spec|features)/})
  spec.require_paths = ["lib", "ext"]

  # This only works on Linux for now, but might as well make it generic for the
  # future
  spec.platform = Gem::Platform::CURRENT

  spec.add_development_dependency "bundler", "~> 1.6"
  spec.add_development_dependency "rake"

  spec.executables = "rust_example"

  spec.extensions << "ext/rust_example/extconf.rb"
end
