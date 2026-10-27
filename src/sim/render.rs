use crate::sim::{Simulation, tiles::TileRender};
use ggez::{
    Context, GameResult,
    graphics::{Canvas, Image},
};
use ggez::glam::{Mat4, Vec2};

pub struct SimRender {
    pub tile: TileRender,

    pub camera: Vec2,
    pub zoom: f32,
}

impl SimRender {
    pub fn new(ctx: &mut Context) -> GameResult<SimRender> {
        let tiles_atlas = Image::from_path(ctx, "/textures/tiles.png")?;

        Ok(Self {
            tile: TileRender::new(tiles_atlas),
            camera: Vec2::ZERO,
            zoom: 1.0,
        })
    }

    fn calc_projection(&self, size: impl Into<Vec2>) -> Mat4 {
        let [hw, hh] = (size.into() / self.zoom / 2.0).to_array();
        let [ox, oy] = (self.camera).to_array();
        Mat4::orthographic_lh(ox - hw, ox + hw, oy + hh, oy - hh, 0., 1.)
    }

    pub fn render(&self, sim: &Simulation, canvas: &mut Canvas) {
        let old = canvas.projection();

        canvas.set_projection(self.calc_projection(canvas.scissor_rect().size()));
        self.tile.render(&sim.tiles_map, canvas);
        canvas.set_projection(old);
    }
}
