pub trait User {
    fn create_user(&self, username: String, password: String) -> Result<(), String>;
    fn remove_user(&self, username: String) -> Result<(), String>;
    fn get_all_users(&self) -> Result<Vec<String>, String>;
}
