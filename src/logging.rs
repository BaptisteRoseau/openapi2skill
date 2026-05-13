use log::LevelFilter;

pub fn init_logger(enable: bool) {
    if !enable {
        return;
    }

    env_logger::builder()
        .format_timestamp_millis()
        .format_module_path(false)
        .format_target(false)
        .filter_level(LevelFilter::Info)
        .init();
}
