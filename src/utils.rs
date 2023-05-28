use crate::service::Status;
use crossterm::style::{style, Color, Stylize};
use prettytable::{Cell, Row, Table};

pub fn validate_py(path: &str) -> bool {
    if path.ends_with("py") {
        true
    } else {
        false
    }
}

pub fn create_table(status_vec: Vec<Status>) -> Table {
    let mut table = Table::new();

    let fields: Vec<Cell> = Status::get_field_names()
        .into_iter()
        .map(|f| Cell::new(style(&f).with(Color::Green).to_string().as_str()))
        .collect();

    table.add_row(Row::new(fields));

    for status in status_vec {
        table.add_row(Row::new(vec![
            Cell::new(&status.name),
            Cell::new(&status.pid.to_string()),
            Cell::new(&status.cpu.to_string()),
            Cell::new(&status.memory.to_string()),
            Cell::new(if status.active == "active" {
                "Yes"
            } else {
                "No"
            }),
            Cell::new(if status.enabled == "enabled" {
                "Yes"
            } else {
                "No"
            }),
            Cell::new(&status.params.path),
            Cell::new(&status.params.pyexec),
            Cell::new(&status.params.unit_file_path),
            Cell::new(&status.params.systemd_file_path),
        ]));
    }

    table
}
