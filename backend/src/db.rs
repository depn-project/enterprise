use rusqlite::{params, Connection, Result};

fn create_users_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
             id INTEGER PRIMARY KEY,
             username TEXT NOT NULL,
             password TEXT NOT NULL
         )",
        params![],
    )?;

    Ok(())
}

fn create_servers_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS servers (
             id INTEGER PRIMARY KEY,
             server_name TEXT NOT NULL,
             country TEXT NOT NULL,
             city TEXT NOT NULL,
             vpn_config TEXT NOT NULL,
             ip TEXT NOT NULL,
             port INTEGER NOT NULL
         )",
        params![],
    )?;

    Ok(())
}

pub fn init() -> Result<()> {
    let conn = Connection::open("/var/db/depn.db")?;

    create_users_table(&conn)?;
    println!("[OK] Users table");

    create_servers_table(&conn)?;
    println!("[OK] Servers table");

    Ok(())
}
