use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn init_logger(enable: bool) {
    if !enable {
        return;
    }

    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .without_time()
        .with_level(false)
        .with_target(false)
        .init();
}
