use chrono::prelude::*;
use log::*;
use rusqlite::{Connection, Result};

use crate::config::Config;

#[derive(Debug)]
pub struct MatRow {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub author: String,
    pub time_added: String,
}

impl MatRow {
    pub fn new(name: &str, url: &str, author: &str) -> Self {
        Self {
            id: -1,
            name: name.to_string(),
            url: url.to_string(),
            author: author.to_string(),
            time_added: Local::now().format("%F %r").to_string(),
        }
    }

    pub fn vec_from_database(table_name: &str) -> Result<Vec<MatRow>, &'static str> {
        // TODO: check for database_path existance
        let conf = match Config::from_json_file("settings.json") {
            Ok(conf) => conf,
            Err(error) => {
                error!("{}", error);
                panic!("{}", error);
            }
        };

        let conn = match Connection::open(&conf.database_path) {
            Ok(conn) => conn,
            _ => return Err("could not open database file"),
        };

        let query = format!(
            "SELECT id, name, url, author, time_added FROM \"{}\"",
            table_name
        );
        let mut stmt = match conn.prepare(query.as_str()) {
            Ok(stmt) => stmt,
            _ => return Err("could not prepare statement"),
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
        let conf = match Config::from_json_file("settings.json") {
            Ok(conf) => conf,
            Err(error) => {
                error!("{}", error);
                panic!("{}", error);
            }
        };

        let conn = match Connection::open(&conf.database_path) {
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
            trace!("[ERROR] {}", error);
            return Err("could not insert into database");
        }
        Ok(())
    }
}

pub fn build_database() -> Result<(), &'static str> {
    let conf = match Config::from_json_file("settings.json") {
        Ok(conf) => conf,
        Err(error) => {
            error!("{}", error);
            panic!("{}", error);
        }
    };

    let conn = match Connection::open(&conf.database_path) {
        Ok(conn) => conn,
        _ => return Err("could not open database file"),
    };

    for (material, _) in &conf.material_types {
        let sql = format!(
            r#"CREATE TABLE IF NOT EXISTS '{}' (
                            'id'	INTEGER NOT NULL UNIQUE,
                            'name'	TEXT,
                            'url'	TEXT,
                            'author'	TEXT,
                            'time_added'	TEXT,
                            PRIMARY KEY('id' AUTOINCREMENT)
          )"#,
            material
        );
        match conn.execute(&sql, []) {
            Ok(_) => info!("the database now has {}", material),
            Err(error) => {
                trace!("[ERROR] {}", error);
                return Err("could not build the database");
            }
        }
    }
    Ok(())
}
