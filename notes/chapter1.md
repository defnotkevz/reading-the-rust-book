
# Getting Started

`rustfmt` - for formatting code according to the opinionated standard of the community (not a bad thing tbh)


Sample Program
```
fn main(){
    println!("Hello World")
}
```

the main function is the entry point - like C
println! -> the bang operator is used for rust macros.

how do you run this? like C. compile and execute the binary.
`rustc ~ gcc`

that is like the very basics.

## Cargo

cargo is the build system and package manager of rust. it makes it easier to manage **dependencies**(other pieces of code).
something like npm?

you can also init a project with cargo
`cargo new <insert-project-name>`

and it has a .toml file just like a package.json with the list of dependencies
and a .lock file like package-lock.json

packages of code are called **crates**

*cargo puts all your code inside a /src folder, the top level is for configs and other stuff*

**working with cargo**
`cargo init` - start a project from an already created folder

**building a project**
run `cargo build` then run the executable in the /target/debug
OR
you can just run `cargo run`

for quick check whether your project compiles, use `cargo check`, this does not create an executable so it's much faster than `cargo build`

**for building your final prod build**
`cargo build --release`
