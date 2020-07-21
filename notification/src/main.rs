extern crate log;
use dotenv::dotenv;
use envconfig::Envconfig;
use lazy_static::lazy_static;
use std::process;

mod config;

static PKG_NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
static PKG_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

lazy_static! {
    static ref CONFIGURATION: config::Config = {
        dotenv().ok();
        match config::Config::init() {
            Ok(val) => val,
            Err(e) => {
                log::error!("{}", e);
                process::exit(1);
            }
        }
    };
}

fn main() {
    println!(
        "SERVICE: {} | VERSION: {}\n",
        PKG_NAME.unwrap_or("<unknown>"),
        PKG_VERSION.unwrap_or("<unknown>")
    );
    env_logger::init();
    log::info!("Starting service");
}
