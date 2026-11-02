use macroquad::{
    color::BLACK,
    texture::{FilterMode, set_default_filter_mode},
    window::{clear_background, next_frame},
};
use thiserror::Error;

use crate::{
    core::{
        command::{CommandFn, CommandsQueue},
        registry::Registry,
    },
    sim::{Simulation, render::SimRender},
};

pub mod command;
pub mod registry;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("macroquad error: {0}")]
    Macroquad(#[from] macroquad::Error),
}

pub type GameResult<R = ()> = Result<R, GameError>;

pub struct Game {
    pub registry: Registry,
    pub cmds: CommandsQueue,

    pub sim: Simulation,
    pub sim_render: SimRender,

    pub run_sim: bool,
}

impl Game {
    async fn init() -> GameResult<Game> {
        set_default_filter_mode(FilterMode::Nearest);

        Ok(Game {
            registry: Registry::load().await,
            cmds: CommandsQueue::default(),

            sim: Simulation::default(),
            sim_render: SimRender::new().await?,

            run_sim: false,
        })
    }
}

impl Game {
    fn update(&mut self) -> GameResult {
        if self.run_sim {
            self.cmds.push(Simulation::update);
            self.cmds.push(SimRender::update);
        }

        CommandsQueue::execute(self);
        Ok(())
    }

    fn draw(&mut self) -> GameResult {
        clear_background(BLACK);

        if self.run_sim {
            self.sim_render.render(&self.sim);
        }

        Ok(())
    }

    pub async fn run(entry: impl CommandFn) -> GameResult {
        let mut game = Game::init().await?;
        game.cmds.push(entry);
        CommandsQueue::execute(&mut game);

        loop {
            game.update()?;
            game.draw()?;

            next_frame().await;
        }
    }
}
