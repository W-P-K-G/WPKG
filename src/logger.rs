use tracing::level_filters::LevelFilter;

const MAX_LEVEL: LevelFilter = LevelFilter::INFO;

pub fn init() {
    tracing_subscriber::fmt().with_max_level(MAX_LEVEL).init();
}
