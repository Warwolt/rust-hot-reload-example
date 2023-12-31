mod hello;

use hello::Printer;

pub struct State {
    pub counter: usize,
    printer: Printer,
}

#[no_mangle]
pub fn init() -> State {
    State {
        counter: 0,
        printer: Printer::new(),
    }
}

#[no_mangle]
pub fn setup_logger(
    logger: &'static dyn log::Log,
    level: log::LevelFilter,
) -> Result<(), log::SetLoggerError> {
    log::set_max_level(level);
    log::set_logger(logger)
}

#[no_mangle]
pub fn update(state: &mut State) {
    state.counter += 1;
    state.printer.print_counter(state.counter);
}
