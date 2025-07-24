use log::info;

mod data_struct;
mod logger;
mod core;

fn main() {
    logger::init_logger();
    println!("Hello, world!");
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
