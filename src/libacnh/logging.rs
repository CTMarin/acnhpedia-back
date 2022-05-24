use tracing::{Level, info, error};
use tracing_subscriber::FmtSubscriber;

pub fn error(message: &str) {
    let subscriber = create_subscriber();
    tracing::subscriber::with_default(subscriber, || {
        error!("{}", message);
    });
}

pub fn info(message: &str) {
    let subscriber = create_subscriber();
    tracing::subscriber::with_default(subscriber, || {
        info!("{}", message);
    });
}

fn create_subscriber() -> FmtSubscriber {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    subscriber
}
