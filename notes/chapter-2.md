# Programming a Guessing Game

In this chapter we start writing code which resembles more of code in real world projects.

PS: please do not forget your semi-colons ;)

**prelude**: these are features that are available in a rust program by default. Features outside the prelude have to be imported using the `use` syntax.

`use std::io;`
just like including header files in C, now the standard I/O library is in scope. And we can use everything in here. 
it is a **crate**. modules are used to organize code inside crates. std is the **crate** and io is the **module**.

`let mut guess = String::new();`
ahh, variables. variables in Rust are **immutable** by default.
hence why we have added `mut` that makes it mutable. kinda like ~inverse~ `const` lol.

`String::new()`
Creates a new, empty instance of a String. it's growable. kinda like malloc. you are allocating a space for data. but this is a better version, it's dynamic in size and it's typed. 
**IMP: because it's dynamic, runtime variable it's stored in the heap.**

**keep note of the `::` operator. it's behaving differently in different areas.**

new is a part of String -> that's why ::, like io is a part of std.

io::stdin() -> stdin() is a part of io. hence the ::

read_line() takes a mutable reference to a string and appends the value the user wrote to standard input. hehe a mouthful i know. 

read_line() returns a `Result` which is an enum which can take the variants - `Ok` & `Err`
if the result is Err, expect will make your code crash, which is not proper error handling, but baby steps.


okay let's improve this code
`use rand::Rng;`
`let secret_number = rand::thread_rng().gen_range(1..=100);`

this is to generate a random number.

Rng from rand lets us use gen_range() function. 
**The dot operator in rust does not work the same way as in JS/TS*

we call thread_rng() from rand which is a random number generator and uses the dot operator to pass the value to gen_range which spits out a value in between 1-100 inclusive of both.

```
side note: the rust compiler is amazing. it gives me the exact error and what to do to fix that??

some of the functions are deprecated, like thread_rng(), gen_range, other functions are now being used, but the compiler gave me the correct ones and that has been updated to the code.

and unused variables can be preceded with an _underscore
```


`use std::cmp::Ordering;`
```rust
match guess.cmp(&secret_number) 
{ Ordering::Less => println!("Too small!"), 
  Ordering::Greater => println!("Too big!"), 
  Ordering::Equal => println!("You win!"), }
```

let's see, new import: different module, but the same crate.

.cmp() is returning an Ordering which can be any of the three values.

`Ordering` is an enum with three variants. Like `Result` has two variants.

now `match` is used like a switch case to decide what to do next

**question: I imported the Ordering module specifically, and not cmp. but i have use .cmp() in the code. how?**

**question: why is each function taking the reference of the values and the actual owned value?**

