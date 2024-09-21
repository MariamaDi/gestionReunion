use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::{Pool, PooledConnection};

#[derive(Clone)]
pub struct AppState {
    pub conn: Pool<ConnectionManager<PgConnection>>,
}
impl AppState {
    pub fn get_conn(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.conn
            .get()
            .expect("Failed to get a connection from the pool.")
    }
}
