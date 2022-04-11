use ansi_term::Color;
use chrono::prelude::*;
use log::*;
use rusqlite::{Connection, OpenFlags, Result};

use crate::config::CONF;

#[derive(Debug)]
pub struct MatRow {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub author: String,
    pub time_added: String,
}

impl MatRow {
    pub fn new(name: &str, url: &str, author: &str) -> Self {
        Self {
            id: i64::default(),
            name: name.to_string(),
            url: url.to_string(),
            author: author.to_string(),
            time_added: Local::now().format("%F %r").to_string(),
        }
    }

    pub fn vec_from_database(table_name: &str) -> Result<Vec<MatRow>, &'static str> {
        // TODO: check for database_path existance
        let conn = match Connection::open(&CONF.database_path) {
            Ok(conn) => conn,
            _ => return Err("could not open database file"),
        };

        let query = format!(
            "SELECT ROWID, name, url, author, time_added FROM \"{}\"",
            table_name
        );
        let mut stmt = match conn.prepare(query.as_str()) {
            Ok(stmt) => stmt,
            Err(error) => {
                // unrecoverable error
                debug!("[{}] {}", Color::Red.paint("ERROR"), error);
                panic!("{}", error);
            }
        };

        let mat_rows = stmt.query_map([], |row| {
            Ok(MatRow {
                id: row.get(0)?,
                name: row.get(1)?,
                url: row.get(2)?,
                author: row.get(3)?,
                time_added: row.get(4)?,
            })
        });
        let mat_rows = match mat_rows {
            Ok(mat_rows) => mat_rows,
            _ => return Err("could not iterate over the table"), // TODO: formated string for table_name
        };
        let mat_rows = mat_rows
            .map(|row| {
                // TODO: find better way instead of unwraping every row
                row.unwrap()
            })
            .collect();
        Ok(mat_rows)
    }

    pub fn insert_into_database(&self, material_type: &str) -> Result<(), &'static str> {
        let conn = match Connection::open(&CONF.database_path) {
            Ok(conn) => conn,
            _ => return Err("could not open database file"),
        };

        let sql = format!(
            "INSERT INTO '{}' (name, url, author, time_added) VALUES (?1, ?2, ?3, ?4)",
            material_type
        );

        if let Err(error) = conn.execute(
            &sql,
            &[&self.name, &self.url, &self.author, &self.time_added],
        ) {
            debug!("[{}] {}", Color::Red.paint("ERROR"), error);
            return Err("could not insert into database");
        }
        Ok(())
    }
}

pub fn build_database() -> Result<(), &'static str> {
    let conn = match Connection::open_with_flags(
        &CONF.database_path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
    ) {
        Ok(conn) => conn,
        Err(error) => {
            debug!("[{}] {}", Color::Red.paint("ERROR"), error);
            return Err("could not open database file");
        }
    };

    for (material, _) in &CONF.material_types {
        let sql = format!(
            r#"CREATE TABLE IF NOT EXISTS '{}' (
                            'name'	TEXT,
                            'url'	TEXT,
                            'author'	TEXT,
                            'time_added'	TEXT
          )"#,
            material
        );
        match conn.execute(&sql, []) {
            Ok(_) => info!("the database now has {}", material),
            Err(error) => {
                debug!("[{}] {}", Color::Red.paint("ERROR"), error);
                return Err("could not build the database");
            }
        }
    }
    Ok(())
}

pub fn delete_from_database(material_type: &str, id: usize) -> Result<(), &'static str> {
    let conn = match Connection::open(&CONF.database_path) {
        Ok(conn) => conn,
        _ => return Err("could not open database file"),
    };

    let sql = format!("DELETE FROM '{}' WHERE ROWID = {}", material_type, id);

    match conn.execute(&sql, []) {
        Ok(number) => {
            if number < 1 {
                let error = "could not find the requested id";
                debug!("[{}] {}", Color::Red.paint("ERROR"), error);
                return Err(error);
            }
        }
        Err(error) => {
            debug!("[{}] {}", Color::Red.paint("ERROR"), error);
            return Err("could not delete from database");
        }
    }

    Ok(())
}
