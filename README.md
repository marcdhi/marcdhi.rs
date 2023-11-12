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

* Running - 
```bash
./main
```

## Adding the Cargo magic

* What is a [TOML](https://toml.io/en/) file? - TOML aims to be a minimal configuration file format that's easy to read due to obvious semantics. TOML is designed to map unambiguously to a hash table. TOML should be easy to parse into data structures in a wide variety of languages.

* Cargo generates a TOML for us that initially contains no dependencies. Just has the package information.


# Rust Features ;

1. In Rust, variables are immutable by default, meaning once we give the variable a value, the value won’t change

```bash
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

## Ownership in Rust

> #### The real reason to start using Rust : **Ownership**
> 
> - All programs have a way to manage a way the memory is running.
> - Few languages have Garbage Collectors, a few others have to explicitly allocate the memory and then free it up when not in use.

* Rust uses a different approach at this, memory is managed through a system of ownership with a set of compile time rules
* If any of the rule is violated, the program wont compile

#### Understanding *Stack* and *Heap*

Both stack and heap are available at the runtime for the memory storage

* Stack - LiFo approach, data must have a fixed size to store in the stack
* Heap - 
    * Data with unknown size at the compile time or a size that might change must be stored inside a heap.
    * Heap is un-organised. It's like a space where you request a certain amount of room, and the system finds a spot big enough for you. It returns a pointer (an address) to that location. Used for data with an unknown size at compile time or a size that might change.

        * Allocating on the heap involves requesting and getting a specific space.
        * The pointer to the heap has a fixed size and can be stored on the stack.
        * Accessing data on the heap is slower than on the stack as it involves following a pointer.

In simpler terms, pushing to the stack is faster, like adding plates to a stack, while allocating on the heap is a bit more work, like finding a suitable table in a restaurant. Understanding the stack and heap becomes crucial when managing data in your code efficiently.

> #### Ownership rules :
> * Each value in Rust has an *owner*
> * There can only be one owner at a time.
> * When the owner goes out of scope, the value will be dropped.

# Understanding Variable Scope, String Type, and Ownership in Rust

## Variable Scope

- **Scope Definition:** A scope is the range within a program for which an item, like a variable, is valid.
- **Example:**
  ```rust
  {
      let s = "hello"; // s is valid from this point forward
      // do stuff with s
  } // this scope is now over, and s is no longer valid

## String Type

- **String Type**: Represents text data stored on the heap, allowing mutable operations.
- **Example**:
```rust
let s = String::from("hello");
let mut s2 = s.clone();
s2.push_str(", world!"); // push_str() appends a literal to a String
println!("{}", s2); // This will print `hello, world!`
```

## Memory and Allocation

- **Heap Allocation** : Used for data with unknown size or mutable content, requiring memory allocation.
- **Example** :

```rust
let s1 = String::from("hello");
let s2 = s1; // s1's data (heap) is copied to s2, but s1 is invalidated
```

## Variables and Data Interacting with Move

- **Move Concept** : When a variable is assigned to another, it moves the data, invalidating the first variable.
- **Example** :

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 is moved into s2, and s1 is no longer valid
```

## Variables and Data Interacting with Clone
- **Clone Method** : Deeply copies heap data, creating independent instances.
- **Example** :
```rust

let s1 = String::from("hello");
let s2 = s1.clone(); // s1's heap data is cloned, creating a new instance
```
## Stack-Only Data: Copy
- **Copy Trait** : Annotates types stored on the stack, allowing trivial copying.
- **Example** :

```rust
let x = 5;
let y = x; // x is copied to y because integers are stack-only and implement Copy
```

## Ownership and Functions
- **Function Parameters** : Passing variables to functions moves or copies, following ownership rules.
- **Example** :
```rust

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // some_string goes out of scope, `drop` is called, and the memory is freed

```
## Return Values and Scope
- **Return Values** : Functions can transfer ownership through return values.
- **Example** :
```rust

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // some_string is returned and moves out to the calling function
}
```
## References
- **Reference Concept** : Allows using a value without transferring ownership.
- **Example** :

```rust

fn calculate_length(s: &String) -> usize {
    s.len() // Using a reference to get the length without taking ownership
}
```