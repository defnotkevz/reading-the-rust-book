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