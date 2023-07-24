use simple_logger::SimpleLogger;

#[hot_lib_reloader::hot_module(dylib = "app")]
mod hot_app {
    pub struct State;
    hot_functions_from_file!("app/src/lib.rs");

    #[lib_updated]
    pub fn was_updated() -> bool {}
}

fn init_logging() {
    SimpleLogger::new()
        .with_module_level("hot_lib_reloader", log::LevelFilter::Info)
        .init()
        .unwrap();
}

fn main() {
    init_logging();
    log::info!("Program start");
    let mut state = hot_app::init();
    loop {
        if hot_app::was_updated() {
            hot_app::setup_logger(log::logger(), log::max_level()).unwrap();
        }

        hot_app::update(&mut state);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
