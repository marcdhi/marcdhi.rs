fn main() {
    // let a = [1,2,3];
    // let mut v:Vec<i32> = Vec::new();

    // v.push(1);
    // v.push(1);

    // // Demonstrating a way to remove the values from heap after the scope ends
    // {let v2 = vec![1,2,3];}

    let mut v = vec![1,2,3,4,5,6];

    // let var = &v[1]; // v = [1,2,3,4,5,6]

    // v.push(12345234); // v = [1,2,3,4,5,6,12345234]

    // println!("Lets print it! {}", var);

    // let var2 = &v[6];

    // println!("Lets print it! {}", var2);


    // match new_vec.get(21) {
    //     Some(fourth) => println!("The fourth element of vec is {}", fourth),
    //     None => println!("Boss u are empty!")
    // }


    // When you borrow an element from a vector using &v[1], you're creating an immutable reference to that element. However, when you later push a new element to the vector, it might trigger a reallocation, invalidating the reference.

    // To resolve this, you can use the get method to obtain an Option<&T> and then match on it to handle the case where the index is out of bounds
        

    v.push(2121);
    if let Some(var) = v.get(1) {
        println!("Hurrah {}", var);
    }else {
        println!("Out of bound!!")
    }
}
