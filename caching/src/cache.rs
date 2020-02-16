use crate::api_error::ApiError;
use lazy_static::lazy_static;
use r2d2;
use redis::{Client, ConnectionLike};
use std::env;

type Pool = r2d2::Pool<Client>;
pub type CacheConnection = r2d2::PooledConnection<Client>;

lazy_static! {
    static ref POOL: Pool = {
        let redis_url = env::var("REDIS_URL").expect("Redis url not set");
        let client = redis::Client::open(redis_url).expect("Failed to create redis client");
        Pool::new(client).expect("Failed to create redis pool")
    };
}

pub fn init() {
    info!("Initializing Cache");
    lazy_static::initialize(&POOL);
    let mut conn = connection().expect("Failed to get redis connection");
    assert_eq!(true, conn.check_connection(), "Redis connection check failed");
}

pub fn connection() -> Result<CacheConnection, ApiError> {
    POOL.get()
        .map_err(|e| ApiError::new(500, format!("Failed getting db connection: {}", e)))
}
