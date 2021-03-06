use rocket::request::Form;
use rocket::response::Redirect;

use crate::db::*;
use crate::structures::*;

/// POST: Create a new thread.
#[post("/post", data = "<thread>")]
pub fn post(thread: Form<Thread>) -> Redirect {
	// Pretty rudimentary error handling.
	match new_thread(Thread {
		board: thread.board.clone(),
		name: thread.name.clone(),
		comment: thread.comment.clone(),
	}) {
		Ok(()) => { },
		Err(why) => println!("Error creating new thread: {}", why)
	}

	// Redirect to all posts.
	Redirect::to(format!("/board/{}", thread.board))
}