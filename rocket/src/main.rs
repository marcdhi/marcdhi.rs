#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[post("/hello")]
pub fn hello() -> &'static str {
    "Hello"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
    rocket::build().mount("/hello", routes![hello])
}