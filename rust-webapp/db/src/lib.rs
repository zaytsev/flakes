use std::ops::DerefMut;

pub use cornucopia_async::{ArrayIterator, ArraySql, BytesSql, IterSql, JsonSql, StringSql};

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

pub async fn migrate(pool: &ConnectionPool) -> Result<(), refinery::Error> {
    let mut client = pool.get().await.unwrap();
    let client = client.deref_mut().deref_mut();
    let migration_runner = embedded::migrations::runner();
    migration_runner.run_async(client).await?;
    Ok(())
}

pub type Client = deadpool_postgres::Client;
pub type ConnectionPool = deadpool_postgres::Pool;
pub type ConnectionPoolError = deadpool_postgres::PoolError;
pub type CreateConnectionPoolError = deadpool_postgres::CreatePoolError;
pub type Error = tokio_postgres::Error;

#[derive(Debug, Default)]
pub struct Config {
    pub url: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub database: Option<String>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub use_tls: bool,
}

pub async fn create(
    Config {
        url,
        host,
        port,
        database,
        user,
        password,
        use_tls,
    }: Config,
) -> Result<ConnectionPool, CreateConnectionPoolError> {
    let mut cfg = deadpool_postgres::Config::new();

    cfg.url = url;
    cfg.port = port;
    cfg.host = host;
    cfg.dbname = database;
    cfg.user = user;
    cfg.password = password;

    let tls = if use_tls {
        unimplemented!()
    } else {
        tokio_postgres::NoTls
    };

    Ok(cfg.create_pool(Some(deadpool_postgres::Runtime::Tokio1), tls)?)
}

include!(concat!(env!("OUT_DIR"), "/queries.rs"));
