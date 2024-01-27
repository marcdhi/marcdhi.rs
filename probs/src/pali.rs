fn reverse_int(y: i32) -> i32 {
    let rev_str: String = y.to_string().chars().rev().collect(); // converting the i32 to string, and reversing it
    let rev_num: i32 = rev_str.parse().unwrap_or(0); // parsing the above string and converting back to i32
    rev_num // returning the above generated reversed number, note for returning u need not use a semicolon ;
}

pub fn is_palindrome(x: i32) -> bool {
    let y = reverse_int(x);
    if y == x {
        println!("{:?}", true);
        true
    }
    else {
        println!("{:?}", false);
        false
    }
}