use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Items {
    file_name: String,
}

#[derive(Debug)]
struct Item {
    number: i32,
    name: String,
    completed: i32,
}

impl Items {
    pub fn add(&self, name: &str, completed: i32) {
        let connection = Connection::open(&self.file_name).unwrap();
        connection.execute("insert into items (name, completed) values (?1, ?2)",
            (name, completed)).unwrap();
    }

    pub fn list(&self) -> Result<()> {
        let connection = Connection::open(&self.file_name).unwrap();
        let mut statement = connection.prepare(
            "select number, name, completed from items"
        )?;
        let item_iter = statement.query_map([], |row| {
            Ok(Item {
                number: row.get(0)?,
                name: row.get(1)?,
                completed: row.get(2)?,
            })
        })?;

        for item in item_iter {
            println!("Found Item {:?}", item.unwrap());
        }
        Ok(())
    }
}

fn main() {
    let db = Items {
        file_name: String::from("./data/data.db"),
    };

    db.add("orange", 0);
    db.list();
}