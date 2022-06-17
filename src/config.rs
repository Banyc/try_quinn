use std::time::Duration;

// pub const LISTEN_ADDR_STR: &str = "0.0.0.0:29485";
pub const LISTEN_ADDR_STR: &str = "127.0.0.1:29485";
pub const ANY_ADDR_STR: &str = "0.0.0.0:0";
pub const SERVER_NAME: &str = "localhost";
pub const SERVER_CERT: &str = "certs/test.cert.pem";
pub const CA_CERT: &str = "certs/ca.cert.pem";
pub const SERVER_KEY: &str = "certs/test.pri.pem";
pub const READ_SIZE_LIMIT: usize = 50;
pub const IDLE_TIMEOUT_S: u64 = 60;
pub const KEEP_ALIVE_INTERVAL_S: u64 = 10;

pub static IDLE_TIMEOUT: Duration = Duration::from_secs(IDLE_TIMEOUT_S);
pub static KEEP_ALIVE_INTERVAL: Duration = Duration::from_secs(KEEP_ALIVE_INTERVAL_S);
