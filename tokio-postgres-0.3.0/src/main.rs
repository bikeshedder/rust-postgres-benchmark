use std::env;
use std::time::SystemTime;

use dotenv::dotenv;
use futures::{Future, future::loop_fn, future::Loop};
use futures_state_stream::StateStream;
use tokio_core::reactor::Core;
mod api;
use api::Event;

const ITERATIONS: u128 = 1_000_000;

fn fetch_async(client: tokio_postgres::Connection) -> Box<Future<Item=(Vec<Event>, tokio_postgres::Connection), Error=tokio_postgres::Error>> {
    let prep = client.prepare("SELECT id, name FROM event");
    let client_fut = prep.and_then(move |(stmt, client)| {
        client
            .query(&stmt, &[])
            .map(|row| {
                Event {
                    id: row.get(0),
                    name: row.get(1)
                }
            })
            .collect()

    })
    .map_err(|(error, _)| error);
    Box::new(client_fut)
}

fn main() {
    dotenv().ok();
    let core = Core::new().unwrap();
    let pg_connection_string = env::var("PG_CONNECTION_STRING")
        .expect("PG_CONNECTION_STRING missing in environment");
    let mut sys = actix_rt::System::new("actix_example");
    let client_fut = tokio_postgres::Connection::connect(
        pg_connection_string,
        tokio_postgres::TlsMode::None,
        &core.handle()
    );

    let client = sys.block_on(client_fut).expect("Connect failed");
    let begin = SystemTime::now();
    println!("Running {} queries...", ITERATIONS);
    sys.block_on(loop_fn((client, 0u128), |(client, count)| {
        fetch_async(client)
            .and_then(move |(_, client)| {
                if count < ITERATIONS {
                    Ok(Loop::Continue((client, count+1)))
                } else {
                    Ok(Loop::Break((client, count+1)))
                }
            })
    })).expect("Fetch failed");
    let elapsed = begin.elapsed().expect("elapsed() failed").as_millis();
    println!("Elapsed time: {} ms", elapsed);
    println!("Performance: {} req/s", ITERATIONS*1000/elapsed);
}
