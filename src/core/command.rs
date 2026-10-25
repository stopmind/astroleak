use crate::core::Game;
use ggez::Context;
use log::error;

pub type CmdError = anyhow::Error;
pub type CmdResult<T = ()> = Result<T, CmdError>;

pub trait CommandFn: FnOnce(&mut Game, &mut Context) -> CmdResult + 'static {}
impl<T: FnOnce(&mut Game, &mut Context) -> CmdResult + 'static> CommandFn for T {}

struct Command(Box<dyn CommandFn>);

impl Command {
    fn call(self, game: &mut Game, ctx: &mut Context) -> CmdResult {
        self.0(game, ctx)
    }
}

#[derive(Default)]
pub struct CommandsQueue {
    cmds: Vec<Command>,
    buff: Vec<Command>,
}

impl CommandsQueue {
    fn command(&mut self, cmd: Command) {
        self.buff.push(cmd);
    }

    fn move_buffer(&mut self) {
        self.cmds.extend(self.buff.drain(..).rev());
    }

    pub fn func(&mut self, func: impl CommandFn) {
        self.command(Command(Box::new(func)));
    }

    pub fn execute(game: &mut Game, ctx: &mut Context) {
        game.cmds.move_buffer();
        while let Some(cmd) = game.cmds.cmds.pop() {
            if let Err(err) = cmd.call(game, ctx) {
                error!("failed to execute command: {}", err);
                game.cmds.move_buffer();
            }
        }
    }
}
