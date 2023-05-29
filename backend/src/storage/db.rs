use super::{
    repository::Repository,
    server::{Server, ServerDTO},
    user::User,
    UserDTO,
};
use rusqlite::{params, Connection, Result, NO_PARAMS};

fn create_users_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
             id INTEGER PRIMARY KEY,
             username TEXT NOT NULL UNIQUE,
             password TEXT NOT NULL
         )",
        NO_PARAMS,
    )?;

    Ok(())
}

fn create_servers_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS servers (
             id INTEGER PRIMARY KEY,
             country TEXT NOT NULL,
             city TEXT NOT NULL,
             vpn_config TEXT NOT NULL UNIQUE,
             ip TEXT NOT NULL UNIQUE,
             port INTEGER NOT NULL
         )",
        NO_PARAMS,
    )?;

    Ok(())
}

pub struct Database {
    conn: Connection,
}

impl User for Database {
    fn create_user(&self, username: String, password: String) -> std::result::Result<(), String> {
        let query = self.conn.execute(
            "INSERT INTO users (username, password) VALUES (?1, ?2)",
            params![username, password],
        );

        match query {
            Ok(_) => Ok(()),
            Err(_) => Err("Can't create user".to_string()),
        }
    }

    fn get_all_users(&self) -> std::result::Result<Vec<String>, String> {
        let stmt = self.conn.prepare("SELECT username FROM users");

        if let Ok(mut users) = stmt {
            let rows = users.query_map(NO_PARAMS, |row| row.get(0)).unwrap();
            let usernames = rows.collect::<Result<Vec<String>>>().unwrap();

            return Ok(usernames);
        }

        Err("Can't get all users".to_string())
    }

    fn remove_user(&self, username: String) -> std::result::Result<(), String> {
        self.conn
            .execute("DELETE FROM users WHERE username = ?1", params![username])
            .unwrap();

        Ok(())
    }

    fn get_user(&self, username: String) -> std::result::Result<super::UserDTO, String> {
        let mut stmt = self
            .conn
            .prepare("SELECT username, password FROM users WHERE username = ?1")
            .unwrap();

        let mut rows = stmt.query(params![username]).unwrap();

        if let Some(row) = rows.next().unwrap() {
            let username: String = row.get(0).unwrap();
            let password: String = row.get(1).unwrap();

            Ok(UserDTO {
                username: username.clone(),
                password,
            })
        } else {
            Err(format!("User '{}' not found", username))
        }
    }
}

impl Server for Database {
    fn create_server(
        &self,
        country: String,
        city: String,
        vpn_config: String,
        ip: String,
        port: u16,
    ) -> std::result::Result<(), String> {
        let query = self.conn.execute(
            "INSERT INTO servers (country, city, vpn_config, ip, port) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![country, city, &vpn_config, ip, port],
        );

        match query {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    fn get_all_servers(&self) -> std::result::Result<Vec<ServerDTO>, String>
    where
        Self: Sized,
    {
        let stmt = self
            .conn
            .prepare("SELECT country, city, ip, port, id FROM servers");

        if let Ok(mut servers) = stmt {
            let rows = servers
                .query_map(NO_PARAMS, |row| {
                    Ok(ServerDTO {
                        country: row.get(0).unwrap(),
                        city: row.get(1).unwrap(),
                        ip: row.get(2).unwrap(),
                        port: row.get(3).unwrap(),
                        id: row.get(4).unwrap(),
                    })
                })
                .unwrap();
            let servers = rows.collect::<Result<Vec<ServerDTO>>>().unwrap();

            return Ok(servers);
        }

        Err("Can't get all servers".to_string())
    }

    fn get_server_config(&self, id: u32) -> std::result::Result<String, String> {
        let mut stmt = self
            .conn
            .prepare("SELECT vpn_config FROM servers WHERE id = ?1")
            .unwrap();

        let mut rows = stmt.query(params![id]).unwrap();

        if let Some(row) = rows.next().unwrap() {
            let config: String = row.get(0).unwrap();
            Ok(config)
        } else {
            Err(format!("Server with ID {} not found", id))
        }
    }

    fn remove_server(&self, id: u32) -> std::result::Result<(), String> {
        self.conn
            .execute("DELETE FROM servers WHERE id = ?1", params![id])
            .unwrap();

        Ok(())
    }
}

impl Repository for Database {
    fn init() -> Result<Self, String> {
        let conn = Connection::open("/var/db/depn.db");

        if let Ok(conn) = conn {
            let db = Database { conn };

            create_users_table(&db.conn).unwrap();
            create_servers_table(&db.conn).unwrap();

            return Ok(db);
        }

        Err("Can't connect to database".to_string())
    }
}
