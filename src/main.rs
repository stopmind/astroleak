mod core;
mod entries;
pub mod sim;

use crate::core::Game;
use log::LevelFilter;

fn main() {
    stop_log::init(None, LevelFilter::Info);
    Game::run(entries::dev_entry).unwrap();
}
