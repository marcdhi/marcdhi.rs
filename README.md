# Rust Lang

## Installation

Follow [this](https://www.rust-lang.org/tools/install) guide to install the Rust language on your locals 

## Anatomy of a Rust Program

```bash
fn main() {

}
```
- `main` function is always the first code that runs in every executable Rust program.
- `println!("Hello, world!");` Here the `println!` calls a `Rust Macro`. `!` calls a macro. If we do not add that for a while, and want a function inside the println can do that as well.
PS: Do not forget the `semi-colon` in the end

## Compiling vs Running

* Compile step - `rustc main.rs` this outputs a binary executable. Along with this a file called `main.pdb` is a file containing the debugging information with the `.pdb` extension

* Running - ```bash
./main```

## Adding the Cargo magic

* What is a [TOML](https://toml.io/en/) file? - TOML aims to be a minimal configuration file format that's easy to read due to obvious semantics. TOML is designed to map unambiguously to a hash table. TOML should be easy to parse into data structures in a wide variety of languages.

* Cargo generates a TOML for us that initially contains no dependencies. Just has the package information.
