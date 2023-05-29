use crate::storage::ServerDTO;
use prettytable::{Cell, Row, Table};
use std::fs::File;
use std::io::Read;

pub fn read_file_contents(file_path: String) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn print_server_table(servers: &[ServerDTO]) {
    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("ID"),
        Cell::new("Country"),
        Cell::new("City"),
        Cell::new("IP"),
        Cell::new("Port"),
    ]));

    for server in servers {
        let id_cell = Cell::new(&server.id.to_string());
        let country_cell = Cell::new(&server.country);
        let city_cell = Cell::new(&server.city);
        let ip_cell = Cell::new(&server.ip);
        let port_cell = Cell::new(&server.port.to_string());

        table.add_row(Row::new(vec![
            id_cell,
            country_cell,
            city_cell,
            ip_cell,
            port_cell,
        ]));
    }

    table.printstd();
}
