use std::time::SystemTime;

use dotenv::dotenv;

mod api;
use api::Event;
mod db;
use db::pg_config_from_env;

const ITERATIONS: u128 = 1_000_000;

async fn fetch(client: &tokio_postgres::Client) -> Result<Vec<Event>, tokio_postgres::Error> {
    let stmt = client.prepare("SELECT id, name FROM event").await?;
    let rows = client.query(&stmt, &[]).await?;
    Ok(rows.iter().map(|row| {
        Event {
            id: row.get(0),
            name: row.get(1)
        }
    }).collect())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv().ok();
    let pg_config = pg_config_from_env().expect("Invalid PG config");
    let (client, connection) = pg_config.connect(tokio_postgres::NoTls).await.unwrap();
    tokio::spawn(connection);
    let begin = SystemTime::now();
    println!("Running {} queries...", ITERATIONS);
    for _ in 0..ITERATIONS {
        fetch(&client).await.unwrap();
    }
    let elapsed = begin.elapsed().expect("elapsed() failed").as_millis();
    println!("Elapsed time: {} ms", elapsed);
    println!("Performance: {} req/s", ITERATIONS*1000/elapsed);
}
