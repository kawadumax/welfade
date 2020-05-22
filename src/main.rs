
#![feature(proc_macro_hygiene, decl_macro)];

#[macro_use] extern crate rocket;

use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> Template {
    let context = context();
    Template::render("index", &context)
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
    .attach(Template::fairing())
    .mount("/", routes![index])
    .mount("/foo", routes![bar])
    .mount("/", routes![hello])
    .launch();
}