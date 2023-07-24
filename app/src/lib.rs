mod hello;

use hello::Printer;

pub struct State {
    pub counter: usize,
    printer: Printer,
}

impl State {
    pub fn new() -> Self {
        State {
            counter: 0,
            printer: Printer::new(),
        }
    }
}

#[no_mangle]
pub fn update(state: &mut State) {
    state.counter += 1;
    state.printer.print_counter(state.counter);
}
