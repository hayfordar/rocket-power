pub mod connection;

use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ ConnectionManager, Pool, PooledConnection };

pub type MySQLConnectionPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MySQLPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub fn spawn(connectionString : &str) -> MySQLConnectionPool {
    let manager = ConnectionManager::<MysqlConnection>::new(connectionString);
    Pool::new(manager).expect("Unable to initialize connection pool")
}