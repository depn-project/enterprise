pub trait User {
    fn create_user(&self, username: String, password: String) -> Result<(), String>;
    fn remove_user(&self, username: String) -> Result<(), String>;
    fn get_all_users(&self) -> Result<Vec<String>, String>;
    fn get_user(&self, username: String) -> Result<UserDTO, String>;
}

pub struct UserDTO {
    pub username: String,
    pub password: String,
}
