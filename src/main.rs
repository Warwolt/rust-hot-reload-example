#[hot_lib_reloader::hot_module(dylib = "app")]
mod hot_app {
    pub use app::State;
    hot_functions_from_file!("app/src/lib.rs");
}

fn main() {
    let mut state = hot_app::State::new();
    loop {
        hot_app::update(&mut state);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
