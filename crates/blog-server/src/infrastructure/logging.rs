use tracing_subscriber::{fmt, EnvFilter};

pub fn init_logging() {
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info,bank_api=debug"))
        .unwrap();

    fmt().with_env_filter(filter).init();
}
