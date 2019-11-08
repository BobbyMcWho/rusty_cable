require "helix_runtime"

begin
  require "rusty_cable/native"
rescue LoadError
  warn "Unable to load rusty_cable/native. Please run `rake build`"
end
