// fn main() {
//     // panic!("crash and burn!")

//     let v = vec![1,2,3];
//     v[100];
// }
use std::fs::File;

fn main() {

    let greeting_file_result = File::open("hello.txt");

    let _greet_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error)
    };
    

    

}

