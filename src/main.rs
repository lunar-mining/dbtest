// rocksdb is the blockchain database
// it is a key value store
// sqlite is the encrypted wallet

use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    //let conn = Connection::open_in_memory()?;
    println!("Attempting to establish a connection...");
    let connector = connect()?;
    println!("Connection established");
    println!("Attempting to create an encrypted database...");
    create_key(&connector);
    test_key(&connector)?;
    Ok(())
}

fn connect() -> Result<Connection> {
    let path = "../src/db_test.db3";
    let connector = Connection::open(&path);
    println!("Path created at {}", path);
    connector
}

fn create_key(conn: &Connection) {
    match conn.execute("PRAGMA key = 'hellocanihaveasandwich'", []) {
        Ok(key) => println!("Encrypted database n.{} was created", key),
        Err(err) => println!("Failed to key database {}", err),
    }
}

fn test_key(conn: &Connection) -> Result<()> {
    match conn.execute(
        "PRAGMA key = 'hellocanihaveafffsandwich';
                       SELECT count(*) FROM sqlite_master",
        [],
    ) {
        Ok(count) => println!("Open encrypted database n.{}. Key successful.", count),
        Err(err) => println!("Test failed {}", err),
    }

    Ok(())
}
