mod error;
pub use error::Error;

mod command;
pub use command::{DisplayState, DisplayStateSingle, AudioState, SteamState, modify_system_if_needed};

trait SystemModifier<T> {
    fn new(path: std::path::PathBuf) -> SystemModifier<T>;
    fn get_system_state(&self) -> Result<T, crate::Error>;
}

pub struct AudioState {
    primary_device_id: String,
    volume: u8,
    muted: bool,
}

trait AudioModifier {
    fn set_primary_device(&self, primary_device_id: &str) -> Result<(), crate::Error>;
    fn set_volume(&self, volume: u8) -> Result<(), crate::Error>;
    fn set_muted(&self, muted: bool) -> Result<(), crate::Error>;
    //
}

trait SteamModifier {
    fn launch_steam(&self) -> Result<(), crate::Error>;
    fn kill_steam(&self) -> Result<(), crate::Error>;
    fn enable_big_picture(&self) -> Result<(), crate::Error>;
    fn disable_big_picture(&self) -> Result<(), crate::Error>;
}

trait DisplayModifier {
    fn set_primary_device(&self, primary_device_id: &str) -> Result<(), crate::Error>;
    fn disable_monitors(&self, device_ids: &[&str]) -> Result<(), crate::Error>;
    fn enable_monitors(&self, device_ids: &[&str]) -> Result<(), crate::Error>;
}