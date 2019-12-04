use std::env;
use std::path::Path;

pub use tokio_postgres::config::Config as PgConfig;


pub fn pg_config_from_env() -> Result<PgConfig, String> {
    let mut config = PgConfig::new();
    if let Ok(host) = env::var("PG_HOST") {
        config.host(host.as_str());
    } else {
        if Path::new("/run/postgresql").exists() {
            config.host("/run/postgresql");
        } else {
            config.host("/tmp");
        }
    }
    if let Ok(port_string) = env::var("PG_PORT") {
        let port = port_string.parse::<u16>()
            .map_err(|_| format!("Invalid PG_PORT: {}", port_string))?;
        config.port(port);
    }
    if let Ok(user) = env::var("PG_USER") {
        config.user(user.as_str());
    } else if let Ok(user) = env::var("USER") {
        config.user(user.as_str());
    } else {
        return Err("Missing PG_USER. Fallback to USER failed as well.".into());
    }
    if let Ok(password) = env::var("PG_PASSWORD") {
        config.password(password.as_str());
    }
    if let Ok(dbname) = env::var("PG_DBNAME") {
        config.dbname(dbname.as_str());
    }
    Ok(config)
}
