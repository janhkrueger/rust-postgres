// src/main.rs
use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=postgres dbname=postgres", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Create a table.
    client.execute(
        "CREATE TABLE IF NOT EXISTS your_table (
            id SERIAL PRIMARY KEY,
            name VARCHAR(100) NOT NULL
        )",
        &[],
    ).await?;

    // Insert data into the table.
    client.execute(
        "INSERT INTO your_table (name) VALUES ($1)",
        &[&"John Doe"],
    ).await?;

    // Now we can execute a simple query.
    let rows = client.query("SELECT id, name FROM your_table", &[]).await?;

    // Print the results.
    for row in rows {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);

        println!("id: {}, name: {}", id, name);
    }

    Ok(())
}