pub trait Repository {
    fn init() -> Result<Self, String>
    where
        Self: Sized;
}
