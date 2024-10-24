use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Item {
    file_name: String,
}

impl Item {
    pub fn add(&self, name: &str, completed: i32) {
        let connection = Connection::open(&self.file_name).unwrap();
        connection.execute("insert into items (name, completed) values (?1, ?2)",
            (name, completed)).unwrap();
    }
}

fn main() {
    let db = Item {
        file_name: String::from("./data/data.db"),
    };

    db.add("pair", 0);
}