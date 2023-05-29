pub mod db;
mod repository;
mod server;
mod user;

pub use db::Database;
pub use repository::Repository;
pub use server::Server;
pub use user::User;

pub struct Storage<T: Repository> {
    pub repository: T,
}

impl<T: Repository + User + Server> Storage<T> {
    pub fn new(repository: T) -> Self {
        Storage { repository }
    }
}
