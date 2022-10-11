mod error;
pub use error::Error;

mod command;
pub use command::{DisplayState, DisplayStateSingle, AudioState, SteamState, modify_system_if_needed};

trait SystemModifier<T> {
    fn new(path: std::path::PathBuf) -> Self;
    fn get_system_state(&self) -> Result<T, crate::Error>;
    fn set_objectives(&mut self, desired_state: &T, system_state: &T) -> Result<Option<()>, crate::Error>;
    fn modify_system(&self) -> Result<(), crate::Error>;
}

pub struct 