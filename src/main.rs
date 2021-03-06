#![feature(proc_macro_hygiene, decl_macro, option_result_contains)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;
extern crate serde_json;

mod api;
mod ui;
mod db;
mod structures;

use rocket_contrib::templates::Template;

fn main() {
	rocket::ignite()
		.attach(Template::fairing())
		.register(catchers![
			ui::not_found
     ])
		.mount("/", routes![
			ui::index,
			ui::boards,
			ui::board
		])
		.mount("/api/v1/", routes![
			api::post
		])
		.launch();
}
