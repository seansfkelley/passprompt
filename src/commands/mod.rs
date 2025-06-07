mod add;
mod ask;
mod config;
mod list;
mod remove;

pub use self::add::Args as AddArgs;
pub use self::add::command as add;
pub use self::ask::Args as AskArgs;
pub use self::ask::Which as AskWhich;
pub use self::ask::command as ask;
pub use self::config::Args as ConfigArgs;
pub use self::config::command as config;
pub use self::list::command as list;
pub use self::remove::Args as RemoveArgs;
pub use self::remove::command as remove;

pub struct CommandResult {
  pub save_config: bool,
  pub success: bool,
}
