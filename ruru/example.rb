require 'fiddle'

library = Fiddle::dlopen('target/release/librurulol.so')

function = Fiddle::Function.new(library['initialize_my_app'], [], Fiddle::TYPE_VOIDP)
function.call

puts Calculator.new.pow_3(5)
