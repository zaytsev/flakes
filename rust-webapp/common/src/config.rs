use clap::Parser;

#[derive(Debug, Parser)]
pub struct Options {
    /// Database connection URL
    #[clap(long, env)]
    pub db_url: Option<String>,

    #[clap(long, env)]
    pub db_host: Option<String>,

    #[clap(long, env)]
    pub db_port: Option<u16>,

    #[clap(long, env)]
    pub db_user: Option<String>,

    #[clap(long, env)]
    pub db_password: Option<String>,

    #[clap(long, env)]
    pub db_name: Option<String>,

    #[clap(long, env, default_value = "false")]
    pub db_use_tls: bool,

    #[clap(long, env, default_value = "true")]
    pub db_migrate: bool,

    /// Port to listen for client's connections
    #[clap(long, env, default_value = "8080")]
    pub port: u16,

    /// Host address to listen for client's connections
    #[clap(long, env, default_value = "localhost")]
    pub host: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    DotEnvError(#[from] dotenvy::Error),

    #[error(transparent)]
    CommandLineArgsError(#[from] clap::Error),
}

impl Options {
    pub fn from_env() -> Result<Self, Error> {
        dotenvy::dotenv()?;
        let result = Options::try_parse()?;
        Ok(result)
    }
}
