# PPL Programming Language


PPL is an embeddable programming language written in Rust. It's original purpose
is to serve as an API or as a point of entry for safe Rust applications that target
non-advanced programmer. In my case, I want PPL Scripts to become the input for the 
[SIMPLE Simulation Program](https://github.com/SIMPLE-BuildingSimulation/simple_main),
whose users are Building Scientists, Architects and Engineers.

It is still in very early stages of development, so feel free to provide feedback and
contribute. The plan is for this language to have the following features (not all of
them yet implemented):

* Avoid arbitrary behaviour that often causes user errors (e.g., Trying to use an initialized variable causes an error. 'Nil' is neither True nor False, it cannot be multiplied, etc.)
* The syntax should, hopefully, be friendly for beginners (check the Syntax goals for more)
* Functions are first-class citizens. This is crucial for my applications and I think also for others
* Suggest others, please...

The reason why this repo is public so early in the development is to ask for 
feedback! 

## Syntax

The syntax is supposed to be similar to Rust's. This allows keeping the transition between
the development of the Front- and the Back-End relatively smooth.

```rust
// This code is currently supported
fn fib(n){    
    if n < 2 {
        return n
    }else{
        return fib(n - 1) + fib(n - 2)
    }
}

let y = fib(40)

io::print(y == 102334155)
```

I want to add a feature that comes from Lua's syntax, though. Calling a 
function with a single object as an argument, the parentheses should not
be required. This allows new users or non-programmers feel like they are 
not programming, but **simply specifying their inputs on a text file**.

```rust
// This can be scary if you don't know about functions
building::add_heater_to_space({
    space : livingroom,
    power: 200, // Watts
    heating_setpoint: 22 //C
})

// This is not yet supported, but it could be a JSON or any
// other txt format.
space::heating {
    space : livingroom,
    power: 200 // Watts
    heating_setpoint: 22 //C
}

```

Then, when users are more experienced, they can start using more powerful
features with a smooth learning curve.

```rust
fn add_200W_heating_to_space(space){
    space::heating {
        space: space,
        power: 200 // Watts
        heating_setpoint: 22 //C
    }
}

add_200W_heating_to_space(livingroom)
add_200W_heating_to_space(space)
add_200W_heating_to_space(space)

```

or even

```rust

fn add_200W_heating_to_space(space){
    space::heating {
        space: space,
        power: 200 // Watts

        // a heating setpoint which is a function
        heating_setpoint: fn(date){
            if date::weekday(date) {
                return 22
            }else{
                return 0
            }
        } 
    }
}

add_200W_heating_to_space(livingroom)
add_200W_heating_to_space(space)
add_200W_heating_to_space(space)

```

## Internals

The internals, so far are:

* A scanner/parser that seem to work fine. Some expressions are not yet supported.
* A stack-based virtual machine that does not perform too well compared to Ruby and Lua.
