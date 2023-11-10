# Important notes: "Getting Started"

## Basics 

- Rust is a statically typed language, which means that it must know the types of all variables at compile time 
- Rust style is to indent with four spaces, not a tab.
- using a "!" in `println!` means that you’re calling a macro instead of a normal function
- Most lines of Rust code end with a semicolon.
- Before running a Rust program, you must compile it using the Rust compiler and use `rustc` command by passing it the name of your source file, like this:

```bash
$ rustc main.rs

```

- Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed.

## Cargo

- Cargo is Rust’s build system and package manager. It does few things for you, such as building your code, downloading the libraries your code depends on, and building those libraries
- In Rust, packages of code are referred to as `crates`. https://crates.io/ is the Rust's crate registry.
- Cargo expects your source files to live inside the `src` directory
- `Cargo.lock` is a file that keeps track of the exact versions of dependencies in your project

### Cargo commands

- We can create a project using `cargo new <project_name>`.  The command creates a new directory and project with the name you pass to it. Passing `--vcs=git` to it will initialize a git, `--vcs=none` will make sure there is no version control.
- We can build a project using `cargo build`. It creates an executable file in `target/debug/<project>` rather than in your current directory.
- We can build and run a project in one step using `cargo run`. It compiles the code and then runs the resultant executable all in one command.
- We can build a project without producing a binary to check for errors using `cargo check`.
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the `target/debug` directory.