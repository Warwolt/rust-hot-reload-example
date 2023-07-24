pub struct Printer {}

impl Printer {
    pub fn new() -> Self {
        Printer {}
    }

    pub fn print_counter(&self, counter: usize) {
        println!("doing stuff in iteration {}", counter);
    }
}
