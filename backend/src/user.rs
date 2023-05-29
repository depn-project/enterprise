use crate::storage::{Repository, Storage, User};
use bcrypt::hash;

pub struct UserService<'a, T: Repository + User> {
    storage: &'a Storage<T>,
}

impl<'a, T: Repository + User> UserService<'a, T> {
    pub fn new(storage: &'a Storage<T>) -> Self {
        UserService { storage }
    }

    pub fn create(&self, username: String, password: String) -> Result<(), String> {
        let encrypted_password = hash(password, 10).unwrap();
        let user = self
            .storage
            .repository
            .create_user(username.clone(), encrypted_password);

        user
    }

    pub fn remove(&self, username: String) -> Result<(), String> {
        self.storage.repository.remove_user(username)
    }

    pub fn list(&self) -> Result<Vec<String>, String> {
        self.storage.repository.get_all_users()
    }
}
