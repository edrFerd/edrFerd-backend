use log::info;

mod interface;
mod logger;

fn main() {
    logger::init_logger();

    info!("Logger initialized successfully!");
    println!("Hello, world!");
    info!("that's a info");
}
