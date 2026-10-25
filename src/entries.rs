use crate::{
    core::{Game, command::CmdResult},
    sim::tiles::{CHUNK_SIZE, Tile},
};
use ggez::{
    Context,
    glam::{ivec2, uvec2},
};

pub fn dev_entry(game: &mut Game, _ctx: &mut Context) -> CmdResult {
    game.run_sim = true;

    let chunk = game.sim.tiles_map.new_chunk(ivec2(0, 0));

    for y in 0..CHUNK_SIZE {
        *(chunk.tile_mut(uvec2(0, y as u32)).unwrap()) = Tile::Stone
    }

    for y in 0..CHUNK_SIZE {
        *(chunk
            .tile_mut(uvec2(CHUNK_SIZE as u32 - 1, y as u32))
            .unwrap()) = Tile::Stone
    }

    for x in 1..(CHUNK_SIZE - 1) {
        *(chunk.tile_mut(uvec2(x as u32, 0)).unwrap()) = Tile::Stone;
        *(chunk
            .tile_mut(uvec2(x as u32, CHUNK_SIZE as u32 - 1))
            .unwrap()) = Tile::Stone;
    }

    Ok(())
}
