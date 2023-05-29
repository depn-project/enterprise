use crate::storage::{Repository, Server, ServerDTO, Storage};

pub struct ServerService<'a, T: Repository + Server> {
    storage: &'a Storage<T>,
}

impl<'a, T: Repository + Server> ServerService<'a, T> {
    pub fn new(storage: &'a Storage<T>) -> Self {
        ServerService { storage }
    }

    pub fn create(
        &self,
        country: String,
        city: String,
        vpn_config: String,
        ip: String,
        port: u16,
    ) -> Result<(), String> {
        self.storage
            .repository
            .create_server(country, city, vpn_config, ip.clone(), port)
    }

    pub fn list(&self) -> Result<Vec<ServerDTO>, String> {
        self.storage.repository.get_all_servers()
    }

    pub fn get_config(&self, id: u32) -> Result<String, String> {
        self.storage.repository.get_server_config(id)
    }

    pub fn remove(&self, id: u32) -> Result<(), String> {
        self.storage.repository.remove_server(id)
    }
}
