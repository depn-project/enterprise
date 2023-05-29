pub trait Server {
    fn create_server(
        &self,
        country: String,
        city: String,
        vpn_config: String,
        ip: String,
        port: u16,
    ) -> Result<(), String>;
    fn remove_server(&self, id: u32) -> Result<(), String>;
    fn get_all_servers(&self) -> Result<Vec<ServerDTO>, String>
    where
        Self: Sized;
    fn get_server_config(&self, id: u32) -> Result<String, String>;
}

pub struct ServerDTO {
    pub country: String,
    pub city: String,
    pub ip: String,
    pub port: u16,
    pub id: u32,
}
