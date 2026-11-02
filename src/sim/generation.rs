use crate::{
    core::command::{Command, IntoCommand},
    sim::tiles::{CHUNK_SIZE, TileType, TilesMap},
};
use macroquad::math::{IVec2, ivec2, uvec2};
use noise::{NoiseFn, Perlin};

pub struct StandardGen {
    pub noise: Perlin,
}

impl StandardGen {
    pub fn new(seed: u32) -> StandardGen {
        StandardGen {
            noise: Perlin::new(seed),
        }
    }

    fn gen_chunk(&self, tiles: &mut TilesMap, pos: IVec2) {
        fn choose_tile(v: f64) -> TileType {
            match v {
                -1.0..-0.4 => TileType::Stone,
                -0.4..0.0 => TileType::Ice,
                _ => TileType::Empty,
            }
        }

        let chunk = tiles.new_chunk(pos);
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let tile_pos = uvec2(x as u32, y as u32);
                let tile = chunk.tile_mut(tile_pos);

                let npos = tile_pos.as_dvec2() / (CHUNK_SIZE as f64) + pos.as_dvec2();
                let value = self.noise.get(npos.to_array());
                tile.bg = choose_tile(value - 1.0);
                tile.fg = choose_tile(value);
            }
        }
    }
}

impl IntoCommand for StandardGen {
    fn into_command(self) -> Command {
        Command::func(move |game| {
            let tiles: &mut TilesMap = &mut game.sim.tiles;

            for x in 0..5 {
                for y in 0..5 {
                    self.gen_chunk(tiles, ivec2(x, y));
                }
            }

            Ok(())
        })
    }
}
