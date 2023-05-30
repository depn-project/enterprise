pub mod db;
mod repository;
mod server;
mod user;

pub use db::Database;
pub use repository::Repository;
pub use server::{Server, ServerDTO};
pub use user::{User, UserDTO};

pub struct Storage<T: Repository> {
    pub repository: T,
}

unsafe impl<T: Repository> Send for Storage<T> {}
unsafe impl<T: Repository> Sync for Storage<T> {}

impl<T: Repository + User + Server> Storage<T> {
    pub fn new(repository: T) -> Self {
        Storage { repository }
    }
}
