use crate::{
    core::{Game, command::CmdResult},
    sim::generation::StandardGen,
};

pub fn dev_entry(game: &mut Game) -> CmdResult {
    game.run_sim = true;
    game.sim_render.free_cam = true;
    game.sim_render.zoom = 1.0;

    game.cmds.push(StandardGen::new(345345345));

    Ok(())
}
