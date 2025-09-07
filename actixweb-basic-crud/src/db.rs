use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_pool(database_url: &str) -> Result<PgPool, r2d2::PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .max_size(8)
        .build(manager)
}


