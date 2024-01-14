// fn main() {
//     // panic!("crash and burn!")

//     let v = vec![1,2,3];
//     v[100];
// }
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {

    // let greeting_file_result = File::open("hello.txt");

    // let _greet_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem in creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };
    

    // ALTERNATIVE APPROACH to have cleaner code, avoiding nested match 

    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem in creating the file: {:?}", error);
    //         })
    //     }else {
    //         panic!("Problem in opening the file {:?}", error)
    //     }
    // });

    // let greeting_file = File::open("heljjlo.txt").expect("We expect u to have hello.txt file in this project!!");

    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }
    
}

