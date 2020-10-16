use svm::instr::instrument;

pub fn on_load() {
    println!("Hello, from `on_load()`, implemented in Rust.");
    instrument();
}

pub fn on_attach() {
    println!("Hello, from `on_attach()`, implemented in Rust.");
}

pub fn on_unload() {
    println!("Hello, from `on_unload()`, implemented in Rust.");
}
