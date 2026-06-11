use tracing_subscriber::EnvFilter;

pub fn init_logging() {
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info,bank_api=debug"))
        .unwrap();

    tracing_subscriber::fmt().with_env_filter(filter).init();
}
