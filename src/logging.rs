use tracing::{info, subscriber::set_global_default};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn init_tracing_subscriber() {
    let filter = EnvFilter::builder()
        .with_default_directive("debug".parse().unwrap())
        .from_env()
        .expect("RUST_LOG directives should be in a valid format");

    let filter_display_string = format!("{}", filter);

    let subscriber = FmtSubscriber::builder()
        .pretty()
        .with_env_filter(filter)
        .finish();

    set_global_default(subscriber).expect("should be able to set subscriber as global default");

    info!(filter = filter_display_string, "logging in pretty format");
}
