use std::path::Path;
use std::{thread, time};

use super::{audio, display, steam};

pub fn modify_system_if_needed(
    max_attempts: usize,
    sleep_interval: time::Duration,
    desired_display_state: &display::DisplayState,
    desired_audio_state: &audio::AudioState,
    desired_steam_state: &steam::SteamState,
    display_executable_path: &Path,
    audio_module_path: &Path,
    steam_executable_path: &Path,
) -> Result<Option<()>, crate::Error> {
    for _ in 0..max_attempts {
        let display_modification_needed = display::modify_system_if_needed(
            desired_display_state,
            display_executable_path,
        )?;
        let audio_modification_needed = audio::modify_system_if_needed(
            desired_audio_state,
            audio_module_path,
        )?;
        let steam_modification_needed = steam::modify_system_if_needed(
            desired_steam_state,
            steam_executable_path,
        )?;
        if (display_modification_needed, audio_modification_needed, steam_modification_needed)
            == (None, None, None)
        {
            return Ok(Some(()));
        }
        thread::sleep(sleep_interval);
    }
    Ok(None)
}