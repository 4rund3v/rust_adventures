### Readme.md

Understanding the basics of rust
 - Initial Hello World Program
 - Compile and Build program
 - Minimum set of commands to keep track of


#### rustup
The rustup is the rust installer and version management tool

#### Cargo
the rust build tool and package tool
```
    cargo build
    cargo run
    cargo test
    cargo doc
    cargo publish
```
To create a helloworld program in rust
```
   cargo new hello-world
```
This generates the direcotry with the toml files and the src/main.rs file
```
   cargo run
```
To add a dependency within rust, we use the crates, crate is basically the library/package which we can use
```
--- manually add to cargo.toml
   cargo.toml <---
   [dependencies]
   package-name = "version"
--- auto add
    cargo add package-name
--- Build after addition to the dependencies
cargo build
```
which then creates a file, cargo.lock file which is a log of the exact version of the dependencies we need
```
use package-name::func;
```
after the build, the package can be used directly.
To generate the binary file which can then be used to execute the code
```
# add the line to cargo.toml file
[[bin]]
version
name
source

# then run the command, 
cargo run --bin 
# Generates the binary at the debug folder and it can be run.
```
##### Info:
Code is indented with 4 spaces !tab
println! calls a macro, ! indicates that a macro is called.
line breaks aer indicated with a (;)
Packages of code are called as ```crates```
cargo expects the code to be inside the src directory
outer level files are the readme files and the configuration files.
To build a cargo project
```cargo build```
Then a binary is generated which is then run
To check if the program builds we can use the check command
``` cargo check ```
To build the code for release, use the command
``` cargo build --release ```

