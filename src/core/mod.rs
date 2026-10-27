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
use ggez::glam::Vec2;
use ggez::graphics::Sampler;
use ggez::input::keyboard::KeyCode;

pub mod command;
pub mod registry;

pub struct Game {
    pub registry: Registry,
    pub cmds: CommandsQueue,

    pub sim: Simulation,
    pub sim_render: SimRender,

    pub run_sim: bool,
    pub free_cam: bool,
}

impl Game {
    fn init(ctx: &mut Context) -> GameResult<Game> {
        Ok(Game {
            registry: Registry::load(ctx),
            cmds: CommandsQueue::default(),

            sim: Simulation::default(),
            sim_render: SimRender::new(ctx)?,

            run_sim: false,
            free_cam: false,
        })
    }

    fn free_cam(&mut self, ctx: &mut Context) {
        const SPEED: f32 = 100.0;

        let mut mov = Vec2::ZERO;

        if ctx.keyboard.is_physical_key_pressed(&KeyCode::KeyW.into()) { mov.y -= 1.0 }
        if ctx.keyboard.is_physical_key_pressed(&KeyCode::KeyS.into()) { mov.y += 1.0 }
        if ctx.keyboard.is_physical_key_pressed(&KeyCode::KeyA.into()) { mov.x -= 1.0 }
        if ctx.keyboard.is_physical_key_pressed(&KeyCode::KeyD.into()) { mov.x += 1.0 }

        if let Some(mov) = mov.try_normalize() {
            self.sim_render.camera += mov * SPEED * ctx.time.delta().as_secs_f32();
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.run_sim {
            self.cmds.func(Simulation::update);
        }

        if self.free_cam {
            self.free_cam(ctx);
        }

        CommandsQueue::execute(self, ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(&ctx.gfx, Some(Color::BLACK));
        canvas.set_sampler(Sampler::nearest_clamp());

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
