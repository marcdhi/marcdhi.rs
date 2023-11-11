// RUST IS STATICALLY TYPED = MEANING WE ARE SUPPOSED TO KNOW THE TYPES AT THE COMPILE TIME

// There are 2 data types subset : SCALAR AND COMPOUND

// SCALAR TYPES : four primary scalar types: integers, floating-point numbers, Booleans, and characters

// Integer Types : 

// * Each variant can be either signed or unsigned and has an explicit size

// * Signed and unsigned refer to whether it’s possible for the number to be negative—in other words, whether the number needs to have a sign with it (signed) or whether it will only ever be positive and can therefore be represented without a sign (unsigned)

// Compound Types : Rust has two primitive compound types: tuples and arrays.

fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is : {x}")
}
