#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod controller;
mod models;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![controller::main_controller::index, 
                controller::main_controller::hello, 
                controller::main_controller::employee, 
                controller::main_controller::post_employee],
        )
        .launch();
}
