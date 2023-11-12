# Rust Ownership and Borrowing

## Overview

This README provides insights into Rust ownership and borrowing, specifically focusing on references. Understanding these concepts is crucial for writing safe and efficient Rust code.

## References and Borrowing

In Rust, ownership ensures memory safety, and references are a key part of managing ownership without sacrificing performance.

### Reference Basics

To avoid transferring ownership, references allow access to a value without owning it. Here's an example:

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

### Borrowing and Mutability

Attempting to modify a borrowed value without using a mutable reference results in a compilation error. However, with mutable references, modification is allowed:

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

### Restrictions on References

Rust enforces strict rules on references to prevent data races and ensure safety. For instance, having multiple mutable references to the same value simultaneously is not allowed.

```rust
let mut s = String::from("hello");
{
    let r1 = &mut s;
} // r1 goes out of scope, allowing a new reference

let r2 = &mut s;
```

### Dangling References

Rust prevents dangling references, ensuring that references never outlive the data they point to. Attempting to return a reference to a local variable results in a compilation error.

```rust
fn dangle() -> &String {
    let s = String::from("hello");
    &s // Error: returning a reference to a local variable
}
```

## Conclusion

Understanding Rust's ownership and borrowing is essential for writing robust and efficient code. By following these rules, you can leverage the power of Rust's ownership system to prevent common programming errors and ensure memory safety.
