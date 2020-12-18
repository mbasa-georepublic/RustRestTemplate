#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod controller;
mod models;

fn main() {
    rocket::ignite().mount("/", routes![
        controller::index,
        controller::hello,
        controller::employee]).launch();
}