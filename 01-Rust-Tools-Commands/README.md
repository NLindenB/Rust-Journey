<h1 align=center><strong>Rust Tools & Commands</strong></h1>

## **Rust Installation**

- The best way to install Rust is to use `rustup` from: https://rustup.rs.
- `Rustup` is Rust **installation manager** equivalent to `NPM` for Node and `RVM` for Ruby.

## **Step-by-Step Installation Guide**

1. Run this on terminal:

        $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

2. Type `1` for finishing the installation when asked.

3. Verifiy that these 3 Rust tools are successfully installed by running these command in your terminal:

        $ cargo --version
        $ rustc --version
        $ rustdoc --version

4. To update Rust, run:

        $ rustup update

## **Rust Tools**

- `cargo` - Rust compilation manager, package manager, and general purpose tool. Utilization of Cargo:
  - **Start** a new Rust project;
  - **Build** and **Run** the program;
  - **Manage** dependencies from external libraries;

- `rustc` - Rust compiler (we could also use `cargo` to invoke the compiler).

- `rustdoc` - Rust documentation tool. It builds HTML from our comments in our source code (we could also use `cargo` to run `rustdoc`).

## **Rust Project**

- ### **Create** a new package

  - **Cargo New Command**

    Cargo creates a new package for our Rust project with two distinctions `--bin` and `--lib`. It also initializes a new `git` repository **by default**.

    - Create a new package for a binary program (**used by own project**):

        $ cargo new <PROJECT_NAME> --bin

    - Create a new package for a library (**used by other projects**):

        $ cargo new <PROJECT_NAME> --lib

    - Create a new package without initialization of a `git` repository:

        $ cargo new <PROJECT_NAME> --lib --vcs none

- ### **Compile** and **Run** the code in terminal

  - **Cargo Run**

    This command automatically compiles and runs our code in `src/main.rs`.

        $ cargo run

  - **Cargo Build** + **Manual Run**

    The first command compiles our code and save it in `target/debug/`. To manually run the code, we need the full path to the saving location of the newly compiled code that is named after the directory name.

        $ cargo build
        $ ./target/debug/DIR_NAME

  - **RustC** + **Manual Run**

    This command utilizes Rust's compiling tool and save the compiled code in the root directory instead of inside the `target/`. Then, we need to invoke the compiled file directly from the terminal.

        $ rustc src/main.rs
        $ ./main

- ### **Checking Code's Compileability**

  This command is used to check whether or not our code is compilable. The different is in the compilation speed since this command will **skip the step of producing an executable** from our code; Thus this command is often used during the development time frame where we don't want to see any output yet.

        $ cargo check

- ### **Building for Production**

  This command will compile our code and save it in `target/release/`. This indicates that the code is compiled for production.

        $ cargo build --release

- ### **Cleaning Artifacts**

  The **basic command** is used to remove all artefacts from the target directory that **Cargo** has generated. This command is executable **only** inside a directory with `Cargo.toml`.

        $ cargo clean

  The above command removes both the `target/debug/` and `target/release/` without any response message in the terminal.

  - #### **Extra Arguments**

    To remove specific artefacts from Cargo, there are extra arguments to be added. These arguments can be inspected with the following command:

        $ cargo clean --help
