use crate::display::{DisplayState, DisplayModifier};
use crate::audio::{AudioState, AudioModifier};
use crate::steam::{SteamState, SteamModifier};

use std::time;
use std::thread;

#[derive(Debug, Copy, Clone)]
pub struct SystemModifier<D, A, S> {
    pub (crate) display_modifier: D,
    pub (crate) audio_modifier: A,
    pub (crate) steam_modifier: S,
    pub (crate) max_attempts: usize,
    pub (crate) sleep_interval: time::Duration,
}

impl<D, A, S> SystemModifier<D, A, S>
where
    D: DisplayModifier,
    A: AudioModifier,
    S: SteamModifier,
{
    pub fn run(
        &self,
        desired_display_state: Option<&DisplayState>,
        desired_audio_state: Option<&AudioState>,
        desired_steam_state: Option<&SteamState>,
    ) -> Result<bool, crate::Error> {
        for _ in 0..self.max_attempts {
            let continue_run: bool = self.check_and_modify(
                desired_display_state,
                desired_audio_state,
                desired_steam_state,
            )?;
            if !continue_run { return Ok(true) }
            thread::sleep(self.sleep_interval);
        }
        Ok(false)
    }

    fn check_and_modify(
        &self,
        desired_display_state: Option<&DisplayState>,
        desired_audio_state: Option<&AudioState>,
        desired_steam_state: Option<&SteamState>,
    ) -> Result<bool, crate::Error> {
        let display_result = match desired_display_state {
            Some(d) => self.display_modifier.check_and_modify(d)?,
            None => false,
        };
        let audio_result = match desired_audio_state {
            Some(a) => self.audio_modifier.check_and_modify(a)?,
            None => false,
        };
        let steam_result = match desired_steam_state {
            Some(s) => self.steam_modifier.check_and_modify(s)?,
            None => false,
        };
        match (display_result, audio_result, steam_result) {
            (false, false, false) => Ok(false),
            _ => Ok(true),
        }
    }
}