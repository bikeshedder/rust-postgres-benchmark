use std::time::SystemTime;

use dotenv::dotenv;
use futures::{Future, Stream, future::ok as fut_ok};
use tokio;
extern crate serde_derive;

mod api;
use api::Event;
mod db;
use db::pg_config_from_env;

const ITERATIONS: u128 = 100_000;

fn fetch_sync(client: &mut postgres::Client) -> Vec<Event> {
    client
        .query("SELECT id, name FROM event", &[])
        .expect("client.query failed")
        .iter().map(|row| {
            Event {
                id: row.get(0),
                name: row.get(1)
            }
        }).collect()
}

fn benchmark_sync(pg_config: &tokio_postgres::Config) {
    let config = postgres::config::Config::from(pg_config.clone());
    println!("Benchmark 1 (postgres):");
    println!("  Connecting...");
    let mut client = config.connect(postgres::NoTls).expect("PG connection failed");
    println!("  Running {} queries...", ITERATIONS);
    let begin = SystemTime::now();
    for _ in 0..ITERATIONS {
        fetch_sync(&mut client);
    }
    let elapsed = begin.elapsed().expect("elapsed() failed").as_millis();
    println!("  Elapsed time: {} ms", elapsed);
    println!("  Performance: {} req/s", ITERATIONS*1000/elapsed);
}

fn fetch_async(mut client: tokio_postgres::Client, count: u128) -> Box<Future<Item=(), Error=tokio_postgres::Error>> {
    if count == 0 {
        return Box::new(fut_ok(()));
    }
    let prep = client.prepare("SELECT id, name FROM event");
    let events = prep.and_then(move |stmt| {
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
    let client_future = events.map(|(client, _)| client);
    Box::new(client_future
        .and_then(move |client: tokio_postgres::Client| {
            fetch_async(client, count-1)
        })
    )
}

fn benchmark_async(pg_config: &tokio_postgres::Config) {
    let mut sys = actix_rt::System::new("actix_example");
    println!("Benchmark 1 (tokio-postgres):");
    println!("  Connecting...");
    let connect = pg_config.connect(tokio_postgres::NoTls);
    let fut = connect
        .map(|(client, connection)| {
            let connection = connection.map_err(|e| eprintln!("connection error: {}", e));
            tokio::spawn(connection);
            println!("  Running {} queries...", ITERATIONS);
            client
        })
        .and_then(|client| fetch_async(client, ITERATIONS));
    sys.block_on(fut).expect("Nay :-(");
}

fn main() {
    dotenv().ok();
    let pg_config = pg_config_from_env().expect("Invalid PG config");
    benchmark_sync(&pg_config);
    benchmark_async(&pg_config);
}

