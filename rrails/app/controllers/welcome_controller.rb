require 'rust_example'

class WelcomeController < ApplicationController
  def index
    @number = RustExample.number
  end
end
