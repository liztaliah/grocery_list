use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Item {
    file_name: String,
}

impl Item {
    pub fn add(&self, number: &i32, item_name: &str) {
        let connection = Connection::open(&self.file_name).unwrap();
        connection.execute("insert into items (number, item_name) values (?1, ?2)",
            (number, item_name)).unwrap();
    }
}

fn main() {
    let db = Item {
        file_name: String::from("./data/data.db"),
    };

    db.add(&3, "pair");
}