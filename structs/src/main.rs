struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main() {

let username = String::from("marcdhi");

let email = String::from("gandhi.mardav@gmail.com");

let user1 = build_user(username, email);

let user2 = User {
    email: String::from("new_email"),
    ..user1
};

let user2_username = user2.username;

println!("This is the new username: {user2_username}");

}


fn build_user(username: String, email: String) -> User {
    User {
        active : true,
        username,
        email,
        sign_in_count: 1,
    }
}

// ----------------------COMMENT THE ABOVE CODE WHILE TESTING-------


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}


impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 20
    };

    let sq = Rectangle::square(4);

    println!("The area of the rectangle is {}", rect1.area());
    println!("The area of the rectangle is {}", sq.area());

}
