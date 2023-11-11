// INCORRECT WAY

// fn main() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// Above gives an error, as variables by default are immutable in rust

// To resolve this we may add `mut` to make a variable mutable ahead

fn shadow() {
    let x = 1;

    let x = x + 1; // x = 2

    { 
        let x = x * 2; //overshadowing value of x by changing the scope

        println!("Value of x is {x}"); // x = 4

    }

    println!("this is the value of {x}"); // x = 2

}

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // CONSTANTS vs VARIABLES

    // 1. cannot use `mut` with a constant 
    // 2. Constants are always immutable, unlike variables which could be mutable by adding `mut`
    // 3. Naming convention of CONSTANTS in Rust = All cap and underscored example `THREE_HOURS_IN_SECONDS`

    shadow()

    // 1. Shadowing is different from marking a variable as mut because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

    // 2. The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name. For example, say our program asks a user to show how many spaces they want between some text by inputting space characters, and then we want to store that input as a number:




}