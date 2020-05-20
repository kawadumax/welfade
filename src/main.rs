#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/bar")]
fn bar() -> &'static str {
  "FooBar"
}

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
  format!("Hello, {}({})", name, age)
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index])
    .mount("/foo", routes![bar])
    .mount("/", routes![hello])
    .launch();
}