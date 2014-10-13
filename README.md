# RustExample

This repository has two directories. One is `rust`, and one is `ruby`. They
work together.

The end result? A Ruby gem that's actually implemented in Rust.

## Trying it out

I've provided a handy `Makefile` that does all the right stuff in the right
order to make this work. So just

    $ make

And hopefully it'll work. I built this on Linux, and I _think_ it should work
on OS X. I don't know anything about Windows in general. Help very welcome!

So what does `make` do? This:

## Rust

The Rust stuff is in the `rust` directory. It's a regular old Cargo project
that builds a static library (`.a`) that exposes a C ABI.

To build, like with all Cargo projects, just type

    $ cargo build

inside the `rust` directory.

## Ruby

The `ruby` directory contains a gem. This gem has a C extension. That's right,
there's actually _three_ languages here, not just two.

The C extension relies on a static library. You know, like the one the Rust
project builds.

To build the gem, do this:

    $ cp rust/target/librust_example*.a ruby/ext/rust_example/

We need to put our shiny new Rust library in the right place for Ruby to see
it.

And then do this from within the `ruby` directory:

    $ cd ext/rust_example
    $ ruby extconf.rb
    $ cd ../..
    $ bundle

## Why all the mess?

If this was a more 'real' gem, we'd be doing a lot more stuff. And Ruby doesn't
really have a "C extension API" as much as it has "please link against my
internals thanks." So you end up using special types like `VALUE` and macros
like `NIL_P(obj)` and [other stuff like
this](https://github.com/ruby/ruby/blob/trunk/README.EXT#L89) to marshall
between "MRI stuff" and "regular C stuff." Re-implementing all this in Rust
seems error prone, so I think that a thin C layer which uses this stuff
and then passes the "regular C stuff" to Rust makes more sense than
reimplementing [stuff like
this](https://github.com/ruby/ruby/blob/97d3b04c9b4ac50f9110001a772d59590a6f6274/include/ruby/ruby.h#L491-L507)
just to get rid of the C.
