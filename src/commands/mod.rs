mod cd;
mod ls;
use crate::shell_context::ShellContext;
pub use cd::CdCommand;
pub use ls::LsCommand;

pub trait Command {
    fn name(&self) -> &'static str;

    /// Executes a logic for a given function
    fn exec(&self, args: Vec<&str>, ctx: &mut ShellContext) -> Result<(), String>;
}
