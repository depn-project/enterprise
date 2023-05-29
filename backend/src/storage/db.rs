use super::{repository::Repository, user::User};
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
             vpn_config TEXT NOT NULL,
             ip TEXT NOT NULL,
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
}

impl Repository for Database {
    fn init() -> Result<Self, String> {
        let conn = Connection::open("database.db");

        if let Ok(conn) = conn {
            let db = Database { conn };

            create_users_table(&db.conn).unwrap();
            create_servers_table(&db.conn).unwrap();

            return Ok(db);
        }

        Err("Can't connect to database".to_string())
    }
}
