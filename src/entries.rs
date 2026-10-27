use crate::{
    core::{Game, command::CmdResult},
    sim::tiles::{CHUNK_SIZE, TileType},
};
use ggez::{
    Context,
    glam::{ivec2, uvec2},
};

pub fn dev_entry(game: &mut Game, _ctx: &mut Context) -> CmdResult {
    game.run_sim = true;
    game.free_cam = true;

    game.sim_render.zoom = 7.0;

    let chunk = game.sim.tiles_map.new_chunk(ivec2(0, 0));

    for y in 0..CHUNK_SIZE {
        let tile = chunk.tile_mut(uvec2(0, y as u32)).unwrap();
        tile.fg = TileType::Stone
    }

    for y in 0..CHUNK_SIZE {
        let tile = chunk
            .tile_mut(uvec2(CHUNK_SIZE as u32 - 1, y as u32))
            .unwrap();
        tile.fg = TileType::Stone
    }

    for x in 1..(CHUNK_SIZE - 1) {
        let tile = chunk.tile_mut(uvec2(x as u32, 0)).unwrap();
        tile.fg = TileType::Stone;

        let tile = chunk
            .tile_mut(uvec2(x as u32, CHUNK_SIZE as u32 - 1))
            .unwrap();
        tile.fg = TileType::Stone;
    }

    for x in 1..(CHUNK_SIZE - 1) {
        for y in 0..CHUNK_SIZE {
            let tile = chunk
                .tile_mut(uvec2(x as u32, y as u32))
                .unwrap();
            tile.bg = TileType::Ice
        }
    }

    Ok(())
}
