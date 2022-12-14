use super::display_sys::MMTModifier;
use super::audio_sys::ADCModifier;
use super::steam_sys::U32Modifier;
use super::task::Task;

use std::path::PathBuf;
use std::time;

type InnerSystemModifier = super::system_modifier_inner::InnerSystemModifier<MMTModifier, ADCModifier, U32Modifier>;

static MMT_PATH: &[&str] = &["assets", "MultiMonitorTool.exe"];
static ADC_PATH: &[&str] = &["assets", "AudioDeviceCmdlets.dll"];

const DEFAULT_SLEEP_INTERVAL_SECS: u64 = 5;
const DEFAULT_MAX_ATTEMPTS: usize = 5;

pub struct SystemModifier {
    inner: InnerSystemModifier,
}

#[allow(dead_code)]
impl SystemModifier {
    pub fn run(&self, task: &Task) -> Result<bool, crate::Error> {
        self.inner.run(task)
    }

    pub fn display_id_readout(&self) -> Result<String, crate::Error> {
        self.inner.display_id_readout()
    }

    pub fn audio_id_readout(&self) -> Result<String, crate::Error> {
        self.inner.audio_id_readout()
    }
    
    pub fn new(steam_exe_path: PathBuf) -> SystemModifier {
        SystemModifier { inner: InnerSystemModifier {
            display_modifier: MMTModifier::new(MMT_PATH.iter().collect()),
            audio_modifier: ADCModifier::new(ADC_PATH.iter().collect()),
            steam_modifier: U32Modifier::new(steam_exe_path),
            max_attempts: DEFAULT_MAX_ATTEMPTS,
            sleep_interval: time::Duration::from_secs(DEFAULT_SLEEP_INTERVAL_SECS),
        }}
    }

    pub fn with_mmt_path(&mut self, path: PathBuf) {
        self.inner.display_modifier.exe_path = path
    }

    pub fn with_adc_path(&mut self, path: PathBuf) {
        self.inner.audio_modifier.module_path = path
    }

    pub fn with_steam_path(&mut self, path: PathBuf) {
        self.inner.steam_modifier.exe_path = path
    }

    pub fn with_max_attempts(&mut self, max_attempts: usize) {
        self.inner.max_attempts = max_attempts
    }

    pub fn with_sleep_interval(&mut self, sleep_interval: time::Duration) {
        self.inner.sleep_interval = sleep_interval
    }

    pub fn finalize(self) -> SystemModifier {
        self
    }
}