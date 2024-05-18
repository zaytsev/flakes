use anyhow::Context;
use common::config::Options;
use tracing::info;
use tracing_subscriber::{fmt, fmt::format::FmtSpan, EnvFilter};

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    setup_tracing();

    let opts = Options::from_env().with_context(|| "Error parsing configuration options")?;

    let cfg = db::Config {
        url: opts.db_url,
        host: opts.db_host,
        port: opts.db_port,
        database: opts.db_name,
        user: opts.db_user,
        password: opts.db_password,
        use_tls: opts.db_use_tls,
    };

    let pool = db::create(cfg)
        .await
        .with_context(|| "Error creating database connection pool")?;

    if opts.db_migrate {
        info!("About to apply database migrations");
        db::migrate(&pool)
            .await
            .with_context(|| "Error applying database migrations")?;
    }

    web::server::start(&opts.host, opts.port, web::router::routes(pool)).await?;

    Ok(())
}

fn setup_tracing() {
    let subscriber = fmt()
        .compact()
        .with_span_events(FmtSpan::CLOSE)
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
