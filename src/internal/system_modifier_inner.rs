use super::display::{DisplayState, DisplayModifier};
use super::audio::{AudioState, AudioModifier};
use super::steam::{SteamState, SteamModifier};
use super::task::Task;

use std::time;
use std::thread;

#[derive(Debug, Copy, Clone)]
pub struct InnerSystemModifier<D, A, S> {
    pub (crate) display_modifier: D,
    pub (crate) audio_modifier: A,
    pub (crate) steam_modifier: S,
    pub (crate) max_attempts: usize,
    pub (crate) sleep_interval: time::Duration,
}

impl<D, A, S> InnerSystemModifier<D, A, S>
where
    D: DisplayModifier,
    A: AudioModifier,
    S: SteamModifier,
{
    pub fn display_id_readout(&self) -> Result<String, crate::Error> {
        self.display_modifier.get_id_readout()
    }

    pub fn audio_id_readout(&self) -> Result<String, crate::Error> {
        self.audio_modifier.get_id_readout()
    }

    pub fn run(&self, task: &Task) -> Result<bool, crate::Error> {
        for _ in 0..self.max_attempts {
            let continue_run: bool = self.check_and_modify(&task.display_state, &task.audio_state, &task.steam_state)?;
            if !continue_run {
                return Ok(true)
            }
            thread::sleep(self.sleep_interval);
        }
        Ok(false)
    }

    fn check_and_modify(
        &self,
        display_state: &DisplayState,
        audio_state: &AudioState,
        steam_state: &SteamState,
    ) -> Result<bool, crate::Error> {
        let display_result: bool = self.display_modifier.check_and_modify(display_state)?;
        let audio_result: bool = self.audio_modifier.check_and_modify(audio_state)?;
        let steam_result: bool = self.steam_modifier.check_and_modify(steam_state)?;
        match (display_result, audio_result, steam_result) {
            (false, false, false) => Ok(false),
            _ => Ok(true),
        }
    }
}