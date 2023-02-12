module Test
using Test
using Daniel: greet
# write tests here...
Test.@test greet() == print("Hello World!")

end # module Test
