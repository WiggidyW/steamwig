use super::display::DisplayState;
use super::audio::AudioState;
use super::steam::SteamState;

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub (crate) display_state: DisplayState,
    pub (crate) audio_state: AudioState,
    pub (crate) steam_state: SteamState,
}

#[allow(dead_code)]
impl Task {
    pub fn new() -> Task {
        Task {
            display_state: DisplayState::new(),
            audio_state: AudioState::new(),
            steam_state: SteamState::new(),
        }
    }

    pub fn set_primary_display(&mut self, id: String) {
        self.display_state.primary_device_id = id
    }

    pub fn enable_display(&mut self, id: String) {
        self.display_state.enabled_device_ids.push(id)
    }

    pub fn enable_displays(&mut self, ids: Vec<String>) {
        for id in ids {
            self.display_state.enabled_device_ids.push(id)
        }
    }

    pub fn disable_display(&mut self, id: String) {
        self.display_state.disabled_device_ids.push(id)
    }

    pub fn disable_displays(&mut self, ids: Vec<String>) {
        for id in ids {
            self.display_state.disabled_device_ids.push(id)
        }
    }

    pub fn set_primary_audio(&mut self, id: String) {
        self.audio_state.primary_device_id = id
    }

    pub fn set_volume(&mut self, volume: u8) {
        let v: u8 = match volume > 100 {
            true => 100,
            false => volume,
        };
        self.audio_state.volume = Some(v)
    }

    pub fn set_muted(&mut self, muted: bool) {
        self.audio_state.muted = Some(muted)
    }

    pub fn set_steam_not_running(&mut self) {
        self.steam_state = SteamState::NotRunning
    }

    pub fn set_steam_running_normal(&mut self) {
        self.steam_state = SteamState::RunningNormal
    }

    pub fn set_steam_running_big_picture(&mut self) {
        self.steam_state = SteamState::RunningBigPicture
    }

    pub fn finalize(self) -> Task {
        self
    }
}