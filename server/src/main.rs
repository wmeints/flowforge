#[tokio::main]
async fn main() {
    setup_tracing();
}

fn setup_tracing() {
    use tracing_subscriber::fmt;
    use tracing_subscriber::prelude::*;

    tracing_subscriber::registry().with(fmt::layer()).init();
}
