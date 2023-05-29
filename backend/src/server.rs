use crate::storage::{Repository, Server, Storage};

pub struct ServerService<'a, T: Repository + Server> {
    storage: &'a Storage<T>,
}

impl<'a, T: Repository + Server> ServerService<'a, T> {
    pub fn new(storage: &'a Storage<T>) -> Self {
        ServerService { storage }
    }

    pub fn create(&self, country: String, city: String, vpn_config: String, ip: String, port: u16) {
        let server =
            self.storage
                .repository
                .create_server(country, city, vpn_config, ip.clone(), port);

        match server {
            Ok(_) => println!("[OK] Create server {}:{}", ip, port),
            Err(e) => println!("[ERROR] {}", e),
        }
    }
}
