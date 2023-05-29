/*
#[macro_use]
extern crate rocket;
*/

mod args;
mod io;
mod server;
mod storage;
mod user;

use args::Cli;
use clap::Parser;
use nix::unistd::Uid;
use storage::{Database, Repository, Storage};
use user::UserService;

use crate::{server::ServerService, user::verify_password};

/*
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
*/

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
    if !Uid::effective().is_root() {
        panic!("You must run this executable with root permissions");
    }

    let args = Cli::parse();

    let storage = Storage::new(Database::init().unwrap());
    let user_service = UserService::new(&storage);
    let server_service = ServerService::new(&storage);

    use args::{Commands, ServerCommands, UserCommands};

    match args.command {
        Commands::Run => todo!(),
        Commands::User(command) => match command {
            UserCommands::Create { username, password } => {
                match user_service.create(username.clone(), password) {
                    Ok(_) => println!("[OK] Create user '{}'", &username),
                    Err(e) => eprintln!("[ERR] {}", e),
                }
            }
            UserCommands::List => match user_service.list() {
                Ok(users) => users.iter().for_each(|username| println!("{}", username)),
                Err(e) => eprintln!("[ERR] {}", e),
            },
            UserCommands::Remove { username } => match user_service.remove(username.clone()) {
                Ok(_) => println!("[OK] Removed user '{}'", &username),
                Err(e) => eprintln!("[ERR] {}", e),
            },
            UserCommands::Verify { username, password } => match user_service.get_user(username) {
                Ok(user) => match verify_password(password, user.password) {
                    true => println!("Valid"),
                    false => println!("Not valid"),
                },
                Err(e) => println!("[ERR] {}", e),
            },
        },
        Commands::Server(command) => match command {
            ServerCommands::Create {
                country,
                city,
                vpn_config,
                ip,
                port,
            } => match server_service.create(
                country,
                city,
                io::read_file_contents(vpn_config).unwrap(),
                ip.clone(),
                port,
            ) {
                Ok(_) => println!("[OK] Create server '{}'", &ip),
                Err(e) => eprintln!("[ERR] {}", e),
            },
            ServerCommands::List => match server_service.list() {
                Ok(servers) => io::print_server_table(&servers),
                Err(e) => eprintln!("[ERR] Can't show list of servers: {}", e),
            },
            ServerCommands::Remove { id } => match server_service.remove(id) {
                Ok(_) => println!("[OK] Remove server #{}", id),
                Err(e) => println!("[ERR] {}", e),
            },
            ServerCommands::Config { id } => match server_service.get_config(id) {
                Ok(config) => println!("{}", config),
                Err(e) => eprintln!("[ERR] {}", e),
            },
        },
    }
}
