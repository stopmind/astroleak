pub mod collision;
pub mod render;
pub mod tiles;

use crate::{
    core::{Game, command::CmdResult},
    sim::tiles::TilesMap,
};
use ggez::Context;
use hecs::World;

#[derive(Default)]
pub struct Simulation {
    pub world: World,
    pub tiles_map: TilesMap,
}

impl Simulation {
    pub fn update(_game: &mut Game, _ctx: &mut Context) -> CmdResult {
        Ok(())
    }
}
