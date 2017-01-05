#[macro_use]
extern crate mrusty;

use mrusty::*;

fn main() {

    let mruby = Mruby::new();

    struct Cont {
        value: i32
    }

    // Cont should not flood the current namespace. We will add it with require.
    mrclass!(Cont, "Container", {
        def!("initialize", |v: i32| {
            Cont { value: v }
        });

        def!("value", |mruby, slf: Cont| {
            mruby.fixnum(slf.value)
        });
    });

    // Add file to the context, making it requirable.
    mruby.def_file::<Cont>("cont");

    // Add spec testing.
    describe!(Cont, "
      context 'when containing 1' do
        it 'returns 1 when calling #value' do
          expect(Container.new(1).value).to eql 1
        end
      end
    ");

    let result = mruby.run("
        require 'cont'

        Container.new(3).value
    ").unwrap(); // Returns Value.

    println!("{}", result.to_i32().unwrap());

    let result = mruby.run("
        require './greeting'

        Greeting.new.to_string
    ").unwrap();

    println!("{}", result.to_str().unwrap());

    //let repl = Repl::new(mruby);

    //repl.start(&GnuReadLine);
}
