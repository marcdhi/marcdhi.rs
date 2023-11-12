fn main() {
    let sentence = String::from("Mardav Gandhi!");

    let word = first_word(&sentence);

    println!("My name is {word}");

}
    

fn first_word(s: &str) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]

}