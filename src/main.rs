// src/main.rs
use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=postgres dbname=postgres", NoTls).await?;

    // Create the connection 
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Create a sample table
    client.execute(
        "CREATE TABLE IF NOT EXISTS sample_table (
            id SERIAL PRIMARY KEY,
            name VARCHAR(100) NOT NULL
        )",
        &[],
    ).await?;

    // Add some data to be queried later.
    client.execute(
        "INSERT INTO sample_table (name) VALUES ($1)",
        &[&"Donald Duck"],
    ).await?;
    client.execute(
        "INSERT INTO sample_table (name) VALUES ($1)",
        &[&"Daisy Duck"],
    ).await?;
    client.execute(
        "INSERT INTO sample_table (name) VALUES ($1)",
        &[&"Scrooge McDuck"],
    ).await?;

    // Query the table and print the names in the table.
    let rows = client.query("SELECT id, name FROM sample_table", &[]).await?;
    for row in rows {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);

        println!("id: {}, name: {}", id, name);
    }

    // For sake of the example we delete the data afterwards by dropping the table.
    client.execute(
        "DROP table sample_table",
        &[]
    ).await?;

    Ok(())
}