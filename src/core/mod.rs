use crate::{
    core::{
        command::{CommandFn, CommandsQueue},
        registry::Registry,
    },
    sim::{Simulation, render::SimRender},
};
use ggez::{
    Context, ContextBuilder, GameResult,
    conf::{WindowMode, WindowSetup},
    event,
    event::EventHandler,
    graphics::{Canvas, Color},
};
use std::env;

pub mod command;
pub mod registry;

pub struct Game {
    pub registry: Registry,
    pub cmds: CommandsQueue,

    pub sim: Simulation,
    pub sim_render: SimRender,
    pub run_sim: bool,
}

impl Game {
    fn init(ctx: &mut Context) -> GameResult<Game> {
        Ok(Game {
            registry: Registry::load(ctx),
            cmds: CommandsQueue::default(),

            sim: Simulation::default(),
            run_sim: false,
            sim_render: SimRender::new(ctx)?,
        })
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.run_sim {
            self.cmds.func(Simulation::update);
        }

        CommandsQueue::execute(self, ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(&ctx.gfx, Some(Color::BLACK));

        if self.run_sim {
            self.sim_render.render(&self.sim, &mut canvas);
        }

        canvas.finish(&mut ctx.gfx)?;

        Ok(())
    }
}

impl Game {
    pub fn run(entry: impl CommandFn) -> GameResult {
        let mut builder = ContextBuilder::new("astroleak", "stopmind")
            .window_mode(
                WindowMode::default()
                    .dimensions(1080., 720.)
                    .resizable(true),
            )
            .window_setup(WindowSetup::default().title("astroleak"));

        if cfg!(debug_assertions) {
            builder = builder.add_resource_path(env::current_dir()?.join("assets"));
        }

        let (mut ctx, eloop) = builder.build()?;

        let mut game = Game::init(&mut ctx)?;
        game.cmds.func(entry);
        CommandsQueue::execute(&mut game, &mut ctx);
        event::run(ctx, eloop, game)
    }
}
