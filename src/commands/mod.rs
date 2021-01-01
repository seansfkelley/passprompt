mod add;
mod ask;
mod config;
mod list;
mod remove;

pub use self::add::command as add;
pub use self::add::Args as AddArgs;
pub use self::ask::command as ask;
pub use self::ask::Args as AskArgs;
pub use self::config::command as config;
pub use self::config::Args as ConfigArgs;
pub use self::list::command as list;
pub use self::remove::command as remove;
pub use self::remove::Args as RemoveArgs;

pub struct CommandResult {
  pub should_save: bool,
  pub success: bool,
}
