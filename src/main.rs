fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().unwrap();
    let value = args.next().unwrap();

    let mut db = Database::new().unwrap();

    read_db(&mut db);

    db.insert(key, value);

    let contents = db.to_string();
    let _result = std::fs::write("kv.db", contents);
}

fn read_db(db: &mut Database) {
    let contents = std::fs::read_to_string("kv.db").unwrap();

    for line in contents.lines() {
        let chunks: Vec<&str> = line.split('\t').collect();
        if chunks.len() != 2 {
            todo!("Return error");
        }
        let key = chunks[0];
        let value = chunks[1];
        db.insert(key.to_string(), value.to_string());
    }
}

struct Database {
    inner: std::collections::HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let contents = std::fs::read_to_string("kv.db")?;
        let mut inner = std::collections::HashMap::new();

        for line in contents.lines() {
            let chunks: Vec<&str> = line.split('\t').collect();
            if chunks.len() != 2 {
                todo!("Return error");
            }
            let key = chunks[0];
            let value = chunks[1];
            inner.insert(key.to_owned(), value.to_owned());
        };
        Ok(Database {
            inner: std::collections::HashMap::new()
        })
    }

    fn insert(&mut self, key: String, value: String) {
        self.inner.insert(key, value);
    }

    fn to_string(&self) -> String {
        let mut string = "".to_owned();

        for (key, value) in self.inner.iter() {
            let line = format!("{}\t{}\n", key, value);
            string.push_str(&line);
        }

        return string;
    }
}