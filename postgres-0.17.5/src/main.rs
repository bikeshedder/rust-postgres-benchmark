use std::time::SystemTime;

use dotenv::dotenv;

mod api;
use api::Event;
mod db;
use db::pg_config_from_env;

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
    let pg_config = pg_config_from_env().expect("Invalid PG config");
    let config = postgres::config::Config::from(pg_config.clone());
    let mut client = config.connect(postgres::NoTls).expect("PG connection failed");
    println!("Running {} queries...", ITERATIONS);
    let begin = SystemTime::now();
    for _ in 0..ITERATIONS {
        fetch_sync(&mut client);
    }
    let elapsed = begin.elapsed().expect("elapsed() failed").as_millis();
    println!("Elapsed time: {} ms", elapsed);
    println!("Performance: {} req/s", ITERATIONS*1000/elapsed);
}
