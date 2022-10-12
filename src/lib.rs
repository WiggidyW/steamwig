use std::path::PathBuf;

mod error;
pub use error::Error;

// mod command;
// pub use command::{DisplayState, DisplayStateSingle, AudioState, SteamState, modify_system_if_needed};

// pub trait SystemModifier<T> {
//     fn new(path: std::path::PathBuf) -> Self;
//     fn get_system_state(&self) -> Result<T, crate::Error>;
// }

mod display;
pub use display::DisplayState;

mod display_sys;

mod audio;
pub use audio::AudioState;

mod audio_sys;

mod steam;
pub use steam::SteamState;

mod steam_sys;

pub struct SystemModifier<D, A, S> {
    display_modifier: D,
    audio_modifier: A,
    steam_modifier: S,
}

impl<D, A, S> SystemModifier<D, A, S>
where
    D: display::DisplayModifier,
    A: audio::AudioModifier,
    S: steam::SteamModifier,
{
    pub fn new(display_modifier_path: PathBuf, audio_modifier_path: PathBuf, steam_modifier_path: PathBuf) -> Self {
        SystemModifier {
            display_modifier: D::new(display_modifier_path),
            audio_modifier: A::new(audio_modifier_path),
            steam_modifier: S::new(steam_modifier_path),
        }
    }
}