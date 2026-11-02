use log::LevelFilter;

use crate::core::Game;

mod core;
mod entries;
pub mod sim;

#[macroquad::main("astroleak")]
async fn main() {
    stop_log::init(None, LevelFilter::Info);
    Game::run(entries::dev_entry).await.unwrap();
}
