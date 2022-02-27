use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct MatRow {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub time_added: String,
}

impl MatRow {
    pub fn vec_from_database(database_path: &str, table_name: &str)
        -> Result<Vec<MatRow>, &'static str> {
        // TODO: check for database_path existance
        let conn = Connection::open(database_path);
        let conn = match conn {
            Ok(conn) => conn,
            _ => return Err("could not open database file")
        };
        let query = format!(
                "SELECT id, name, url, time_added FROM \"{}\"",
                table_name
            );
        let stmt = conn.prepare(query.as_str());
        let mut stmt = match stmt {
            Ok(stmt) => stmt,
            _ => return Err("could not prepare statement")
        };

        let mat_rows = stmt.query_map([], |row| {
            Ok(MatRow {
                id: row.get(0)?,
                name: row.get(1)?,
                url: row.get(2)?,
                time_added: row.get(3)?,
            })
        });
        let mat_rows = match mat_rows {
            Ok(mat_rows) => mat_rows,
            _ => return Err("could not iterate over the table")
            // TODO: formated string for table_name
        };
        let mat_rows = mat_rows.map(|row| {
            // TODO: find better way instead of unwraping every row
            row.unwrap()
        }).collect();
        Ok(mat_rows)
    }
}

