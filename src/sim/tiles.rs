use macroquad::{
    color::WHITE,
    math::{IVec2, Rect, UVec2, Vec2, ivec2, vec2},
    texture::{DrawTextureParams, Texture2D, draw_texture_ex},
};
use std::collections::HashMap;

pub const CHUNK_SIZE: usize = 16;
pub const TILE_SIZE: u32 = 8;

#[repr(u8)]
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub enum TileType {
    #[default]
    Empty,
    Stone,
    Ice,
}

#[derive(Default, Copy, Clone)]
pub struct Tile {
    pub bg: TileType,
    pub fg: TileType,
}

pub struct Chunk {
    tiles: [Tile; CHUNK_SIZE * CHUNK_SIZE],
}

fn tile_to_index(pos: UVec2) -> usize {
    pos.x as usize + pos.y as usize * CHUNK_SIZE
}

impl Chunk {
    pub fn tile(&self, pos: UVec2) -> &Tile {
        &self.tiles[tile_to_index(pos)]
    }

    pub fn tile_mut(&mut self, pos: UVec2) -> &mut Tile {
        &mut self.tiles[tile_to_index(pos)]
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            tiles: [Tile::default(); CHUNK_SIZE * CHUNK_SIZE],
        }
    }
}

#[derive(Default)]
pub struct TilesMap {
    chunks: HashMap<IVec2, Chunk>,
}

fn tile_to_chunk_local(pos: IVec2) -> (IVec2, UVec2) {
    let mut chunk = pos / ivec2(CHUNK_SIZE as i32, CHUNK_SIZE as i32);
    if pos.x < 0 {
        chunk.x -= 1
    };
    if pos.y < 0 {
        chunk.y -= 1
    };

    let mut local = pos % ivec2(CHUNK_SIZE as i32, CHUNK_SIZE as i32);
    if local.x < 0 {
        local.x += CHUNK_SIZE as i32
    };
    if local.y < 0 {
        local.y += CHUNK_SIZE as i32
    };

    (chunk, local.as_uvec2())
}

impl TilesMap {
    #[inline]
    pub fn clear(&mut self) {
        self.chunks.clear();
    }

    pub fn new_chunk(&mut self, pos: IVec2) -> &mut Chunk {
        self.chunks.entry(pos).or_default()
    }

    #[inline]
    pub fn chunk(&self, pos: IVec2) -> Option<&Chunk> {
        self.chunks.get(&pos)
    }

    #[inline]
    pub fn chunk_mut(&mut self, pos: IVec2) -> Option<&mut Chunk> {
        self.chunks.get_mut(&pos)
    }

    #[inline]
    pub fn tile(&self, pos: IVec2) -> Option<&Tile> {
        let (chunk, local) = tile_to_chunk_local(pos);
        self.chunk(chunk).map(|c| c.tile(local))
    }

    #[inline]
    pub fn tile_mut(&mut self, pos: IVec2) -> Option<&mut Tile> {
        let (chunk, local) = tile_to_chunk_local(pos);
        self.chunk_mut(chunk).map(|c| c.tile_mut(local))
    }
}

pub struct TileRender {
    atlas: Texture2D,
    atlas_width: u32,
    tile_count: u32,

    pub draw_empty: bool,
    pub draw_unknown: bool,
}

impl TileRender {
    pub fn new(atlas: Texture2D) -> Self {
        let width = atlas.width() as u32 / TILE_SIZE / 2;
        let tile_count = width * atlas.height() as u32 / TILE_SIZE;

        Self {
            atlas,
            atlas_width: width,
            tile_count,

            draw_empty: false,
            draw_unknown: true,
        }
    }

    fn tile_tex_rect(&self, tile: TileType, is_bg: bool) -> Option<Rect> {
        if tile == TileType::Empty && !self.draw_empty {
            return None;
        }

        let mut idx = tile as u32 + 1;

        if idx >= self.tile_count {
            if self.draw_unknown {
                idx = 0
            } else {
                return None;
            }
        }

        Some(Rect::new(
            (idx % self.atlas_width * TILE_SIZE * 2 + if is_bg { TILE_SIZE } else { 0 }) as f32,
            (idx / self.atlas_width * TILE_SIZE) as f32,
            TILE_SIZE as f32,
            TILE_SIZE as f32,
        ))
    }

    pub fn render(&self, map: &TilesMap) {
        const TILE_VEC: Vec2 = vec2(TILE_SIZE as f32, TILE_SIZE as f32);
        const CHUNK_VEC: Vec2 = vec2(
            (CHUNK_SIZE as u32 * TILE_SIZE) as f32,
            (CHUNK_SIZE as u32 * TILE_SIZE) as f32,
        );

        for (pos, chunk) in &map.chunks {
            let chunk_pos = CHUNK_VEC * pos.as_vec2();

            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    let tile = chunk.tiles[x + y * CHUNK_SIZE];

                    if let Some(src) = self
                        .tile_tex_rect(tile.fg, false)
                        .or_else(|| self.tile_tex_rect(tile.bg, true))
                    {
                        let pos = chunk_pos + TILE_VEC * vec2(x as f32, y as f32);
                        draw_texture_ex(
                            &self.atlas,
                            pos.x,
                            pos.y,
                            WHITE,
                            DrawTextureParams {
                                source: Some(src),
                                ..Default::default()
                            },
                        );
                    }
                }
            }
        }
    }
}
