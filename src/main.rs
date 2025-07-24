use log::info;

mod data_struct;
mod logger;

fn main() {
    logger::init_logger();
    println!("Hello, world!");
}
