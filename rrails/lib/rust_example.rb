require 'fiddle'

class RustExample
  def self.number
    rust_example = Fiddle.dlopen('../rust/target/release/librust_example.so')

    num = Fiddle::Function.new(
        rust_example['num'],
          [],
          Fiddle::TYPE_INT
    )

    num.call
  end
end
