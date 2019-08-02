use std::time::SystemTime;

use dotenv::dotenv;
use futures::{Future, Stream};
use tokio;

mod api;
use api::Event;
mod db;
use db::pg_config_from_env;

const ITERATIONS: u128 = 1_000_000;

fn fetch_async(mut client: tokio_postgres::Client) -> Box<Future<Item=(tokio_postgres::Client, Vec<Event>), Error=tokio_postgres::Error>> {
    let prep = client.prepare("SELECT id, name FROM event");
    let client_fut = prep.and_then(move |stmt| {
        client
            .query(&stmt, &[])
            .map(|row| {
                Event {
                    id: row.get(0),
                    name: row.get(1)
                }
            })
            .collect()
            .map(|events| (client, events))
    });
    Box::new(client_fut)
}

fn main() {
    dotenv().ok();
    let pg_config = pg_config_from_env().expect("Invalid PG config");
    let mut sys = actix_rt::System::new("actix_example");
    println!("Connecting...");
    let connect = pg_config.connect(tokio_postgres::NoTls);
    let client_fut = connect
        .map(|(client, connection)| {
            let connection = connection.map_err(|e| eprintln!("connection error: {}", e));
            tokio::spawn(connection);
            client
        });
    let mut client = sys.block_on(client_fut).expect("Connect failed");
    let begin = SystemTime::now();
    println!("Running {} queries...", ITERATIONS);
    for _ in 0..ITERATIONS {
        client = sys.block_on(fetch_async(client)).expect("Fetch failed").0;
    }
    let elapsed = begin.elapsed().expect("elapsed() failed").as_millis();
    println!("Elapsed time: {} ms", elapsed);
    println!("Performance: {} req/s", ITERATIONS*1000/elapsed);
}



