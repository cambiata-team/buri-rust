# Buri

> "Simplicity is about subtracting the obvious and adding the meaningful."
>
> ~ John Maeda: The Laws of Simplicity.

Buri is a strongly and statically typed functional programming languages.

- Compile to anywhere (e.g., executable binary or JavaScript)
- Predictable
- Fearless refactoring

It's main goal is to dramatically increase developer velocity.

## Name

This language, and all tools descended from the language, will be named after Norse mythology (because why not!?).

In Norse mythology, [Búri](https://en.wikipedia.org/wiki/B%C3%BAri) was the first god from which all other gods descended. Likewise, all tools, libraries, and applications in this ecosystem are created from this programming language.

## Contributing

### Environment setup

#### Using a Codespace

The easiest way to get setup is to start up a new GitHub Codespace. This will ensure you have all the necessary tools and dependencies installed.

#### Developing locally

If you'd rather not use a Codespace, you'll need to install the latest version of the following:

- [Rust](https://www.rust-lang.org/tools/install)
- Cargo
- Clippy
- [cargo-cranky](https://github.com/ericseppanen/cargo-cranky)

Then, you can use the following commands:

```sh
cargo build # Build the Buri Rust compiler
cargo test # Run the Buri Rust compiler unit
cargo cranky # Lint the Buri Rust compiler (wraps Clippy)
```

Lastly, ensure you install all the recommended VS Code extensions for an optimal developer experience.

## Compiling Buri code using Cargo

This project contains an example compiler for Buri called "Buri JS Lite". Currently this compiler can only compile to JavaScript, and can only compile Buri programs consisting of a single source file.

To invoke this compiler using Cargo, use the following:

```
cargo run --bin buri_js_lite source.buri <destination.mjs>
```

Where `source.buri` is the file path of the Buri file you want to compile, and `destination.mjs` is the file path where you want to save the compiled output.

The destination parameter is optional. If not supplied, the compiler will write to a file with the same name as the source, replacing the `.buri` extension with `.mjs`.
