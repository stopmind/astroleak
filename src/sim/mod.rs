pub mod collision;
pub mod generation;
pub mod render;
pub mod tiles;

use crate::{
    core::{Game, command::CmdResult},
    sim::tiles::TilesMap,
};
use hecs::World;

#[derive(Default)]
pub struct Simulation {
    pub world: World,
    pub tiles: TilesMap,
}

impl Simulation {
    pub fn clear(&mut self) {
        self.world.clear();
        self.tiles.clear();
    }

    pub fn update(_game: &mut Game) -> CmdResult {
        Ok(())
    }
}
