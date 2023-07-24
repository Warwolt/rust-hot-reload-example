pub struct Printer {}

impl Printer {
    pub fn new() -> Self {
        Printer {}
    }

    pub fn print_counter(&self, counter: usize) {
        let s = format!("doing stuff in iteration {}", counter);
        // println!("{}", s);
        log::info!("{}", s);
    }
}
