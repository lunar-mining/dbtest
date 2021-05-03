// rocksdb is the blockchain database
// it is a key value store
// sqlite is the encrypted wallet

use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    println!("Attempting to establish a connection...");
    let connector = connect()?;
    println!("Connection established");
    println!("Attempting to create an encrypted database...");
    //create_key(&connector);
    encrypt(&connector)?;
    println!("Created encrypted database.");
    println!("Attempting to decrypt database...");
    decrypt(&connector)?;
    println!("Decrypted database.");
    Ok(())
}

fn connect() -> Result<Connection> {
    let path = "/home/x/src/dbtest/src/db.db";
    let connector = Connection::open(&path);
    println!("Path created at {}", path);
    connector
}

fn encrypt(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "ATTACH DATABASE 'encrypted.db' AS encrypted KEY 'testkey';
                SELECT sqlcipher_export('encrypted');
                DETACH DATABASE encrypted;",
    )
}

fn decrypt(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "ATTACH DATABASE 'plaintext.db' AS plaintext KEY 'testkey';
                SELECT sqlcipher_export('plaintext');
                DETACH DATABASE plaintext;",
    )
}
