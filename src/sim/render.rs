use crate::sim::{Simulation, tiles::TileRender};
use ggez::{
    Context, GameResult,
    graphics::{Canvas, Image},
};

pub struct SimRender {
    pub tile: TileRender,
}

impl SimRender {
    pub fn new(ctx: &mut Context) -> GameResult<SimRender> {
        let tiles_atlas = Image::from_path(ctx, "/textures/tiles.png")?;

        Ok(Self {
            tile: TileRender::new(tiles_atlas),
        })
    }

    pub fn render(&self, sim: &Simulation, canvas: &mut Canvas) {
        self.tile.render(&sim.tiles_map, canvas);
    }
}
