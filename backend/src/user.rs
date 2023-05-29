use crate::storage::{Repository, Storage, User};
use bcrypt::hash;

pub struct UserService<'a, T: Repository + User> {
    storage: &'a Storage<T>,
}

impl<'a, T: Repository + User> UserService<'a, T> {
    pub fn new(storage: &'a Storage<T>) -> Self {
        UserService { storage }
    }

    pub fn create(&self, username: String, password: String) {
        let encrypted_password = hash(password, 10).unwrap();
        let user = self
            .storage
            .repository
            .create_user(username.clone(), encrypted_password);

        match user {
            Ok(_) => println!("[OK] Create user '{}'", username),
            Err(e) => println!("[ERROR] {}", e),
        }
    }
}
