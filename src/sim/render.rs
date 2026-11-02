use macroquad::{
    camera::{Camera2D, set_camera, set_default_camera},
    color::PURPLE,
    input::{KeyCode, is_key_down},
    math::{UVec2, Vec2},
    miniquad::window::screen_size,
    shapes::draw_line,
    texture::load_texture,
    time::get_frame_time,
};

use crate::{
    core::{Game, GameResult, command::CmdResult},
    sim::{
        Simulation,
        tiles::{CHUNK_SIZE, TILE_SIZE, TileRender},
    },
};

pub struct SimRender {
    pub tile: TileRender,

    pub camera: Vec2,
    pub zoom: f32,

    pub free_cam: bool,
    pub draw_grid: bool,
}

impl SimRender {
    pub async fn new() -> GameResult<SimRender> {
        let tiles_atlas = load_texture("./assets/textures/tiles.png").await?;

        Ok(Self {
            tile: TileRender::new(tiles_atlas),
            camera: Vec2::ZERO,
            zoom: 1.0,
            free_cam: false,
            draw_grid: false,
        })
    }

    fn free_cam(&mut self) {
        const SPEED: f32 = 100.0;

        let mut mov = Vec2::ZERO;

        if is_key_down(KeyCode::W) {
            mov.y -= 1.0
        }
        if is_key_down(KeyCode::S) {
            mov.y += 1.0
        }
        if is_key_down(KeyCode::A) {
            mov.x -= 1.0
        }
        if is_key_down(KeyCode::D) {
            mov.x += 1.0
        }

        if let Some(mov) = mov.try_normalize() {
            self.camera += mov * SPEED * get_frame_time()
        }
    }

    fn update_inner(&mut self) {
        if self.free_cam {
            self.free_cam()
        }
    }

    pub fn update(game: &mut Game) -> CmdResult {
        game.sim_render.update_inner();
        Ok(())
    }

    pub fn render(&self, sim: &Simulation) {
        let size = Vec2::from(screen_size());
        let camera = Camera2D {
            target: self.camera,
            zoom: Vec2::ONE * 2.0 / size * self.zoom,
            ..Default::default()
        };

        set_camera(&camera);
        self.tile.render(&sim.tiles);
        set_default_camera();

        if self.draw_grid {
            let real_chunk = TILE_SIZE as f32 * CHUNK_SIZE as f32 * self.zoom;

            let zero = (size / 2.) - self.camera * self.zoom;
            let chunk_offset = zero % real_chunk;
            let chunk_count = (size / real_chunk).as_uvec2() + UVec2::ONE * 2;

            for x in (0..chunk_count.x).map(|x| x as f32 * real_chunk + chunk_offset.x) {
                draw_line(x, 0.0, x, size.y, 1.0, PURPLE);
            }

            for y in (0..chunk_count.y).map(|y| y as f32 * real_chunk + chunk_offset.y) {
                draw_line(0.0, y, size.x, y, 1.0, PURPLE);
            }
        }
    }
}
