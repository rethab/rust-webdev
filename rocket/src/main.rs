#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn hi() -> String {
    "hello, word".into()
}

fn main() {
    rocket::ignite().mount("/", routes![hi]).launch();
}
