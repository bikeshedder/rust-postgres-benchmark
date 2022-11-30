use std::env;
use std::time::SystemTime;

use dotenv::dotenv;

mod api;
use api::Event;

const ITERATIONS: u128 = 1_000_000;

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

fn main() {
    dotenv().ok();
    let pg_connection_string = env::var("PG_CONNECTION_STRING")
        .expect("PG_CONNECTION_STRING missing in environment");
    let mut client = postgres::Client::connect(
        pg_connection_string.as_str(),
        postgres::NoTls
    ).expect("PG connection failed");
    println!("Running {} queries...", ITERATIONS);
    let begin = SystemTime::now();
    for _ in 0..ITERATIONS {
        fetch_sync(&mut client);
    }
    let elapsed = begin.elapsed().expect("elapsed() failed").as_millis();
    println!("Elapsed time: {} ms", elapsed);
    println!("Performance: {} req/s", ITERATIONS*1000/elapsed);
}
