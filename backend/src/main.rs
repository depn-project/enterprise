#[macro_use]
extern crate rocket;

mod args;
mod server;
mod storage;
mod user;

use args::Cli;
use clap::Parser;
use storage::{Database, Repository, Storage, User};
use user::UserService;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/*
#[launch]
fn rocket() -> _ {
    let storage = Storage::new(Database::init().unwrap());
    let user_service = UserService::new(&storage);

    user_service.create("user".to_string(), "password".to_string());

    let users = storage.repository.get_all_users().unwrap();

    for user in users {
        println!("{}", user);
    }

    rocket::build().mount("/", routes![index])
}
*/

fn main() {
    let args = Cli::parse();
}
