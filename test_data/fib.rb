#!/usr/bin/ruby
def fib(n)
    if n < 2 then
        return n
    else
        return fib(n-1)+fib(n-2)
    end
end

y = fib(40)
puts y
puts y == 102334155