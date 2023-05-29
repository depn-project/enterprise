pub mod db;
mod repository;
mod user;

pub use db::Database;
pub use repository::Repository;
pub use user::User;

pub struct Storage<T: Repository + User> {
    pub repository: T,
}

impl<T: Repository + User> Storage<T> {
    pub fn new(repository: T) -> Self {
        Storage { repository }
    }
}
