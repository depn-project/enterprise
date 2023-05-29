pub trait Server {
    fn create(name: &str) -> Result<(), String>;
    fn remove(id: u32) -> Result<(), String>;
    fn get_all() -> Result<Vec<Self>, String>
    where
        Self: Sized;
}
