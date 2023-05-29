use crate::storage::{Repository, Storage, User, UserDTO};
use bcrypt::{hash, verify};

pub struct UserService<'a, T: Repository + User> {
    storage: &'a Storage<T>,
}

pub fn verify_password(password: String, hashed_password: String) -> bool {
    verify(password, &hashed_password).unwrap()
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

    pub fn get_user(&self, username: String) -> Result<UserDTO, String> {
        self.storage.repository.get_user(username)
    }
}
