use crate::core::Game;
use log::error;

pub type CmdError = anyhow::Error;
pub type CmdResult<T = ()> = Result<T, CmdError>;

pub trait CommandFn: FnOnce(&mut Game) -> CmdResult + 'static {}
impl<T: FnOnce(&mut Game) -> CmdResult + 'static> CommandFn for T {}

pub struct Command(Box<dyn CommandFn>);

impl Command {
    pub fn func(func: impl CommandFn) -> Self {
        Command(Box::new(func))
    }
    pub fn call(self, game: &mut Game) -> CmdResult {
        self.0(game)
    }
}

pub trait IntoCommand {
    fn into_command(self) -> Command;
}

impl IntoCommand for Command {
    fn into_command(self) -> Command {
        self
    }
}

impl<F> IntoCommand for F
where
    F: CommandFn,
{
    fn into_command(self) -> Command {
        Command::func(self)
    }
}

#[derive(Default)]
pub struct CommandsQueue {
    cmds: Vec<Command>,
    buff: Vec<Command>,
}

impl CommandsQueue {
    pub fn push(&mut self, cmd: impl IntoCommand) {
        self.buff.push(cmd.into_command());
    }

    fn move_buffer(&mut self) {
        self.cmds.extend(self.buff.drain(..).rev());
    }

    pub fn execute(game: &mut Game) {
        game.cmds.move_buffer();
        while let Some(cmd) = game.cmds.cmds.pop() {
            if let Err(err) = cmd.call(game) {
                error!("failed to execute command: {}", err);
                game.cmds.buff.clear();
            }
            game.cmds.move_buffer();
        }
    }
}
