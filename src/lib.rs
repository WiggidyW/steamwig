mod error;
pub use error::Error;

mod display;
pub use display::DisplayState;

mod display_sys;
pub use display_sys::MMTParseError;

mod audio;
pub use audio::AudioState;

mod audio_sys;
pub use audio_sys::ADCParseError;

mod steam;
pub use steam::SteamState;

mod steam_sys;

mod system_modifier;

mod public {
    use crate::display::{DisplayState, DisplayModifier};
    use crate::audio::{AudioState, AudioModifier};
    use crate::steam::{SteamState, SteamModifier, self};
    use crate::display_sys::MMTModifier;
    use crate::audio_sys::ADCModifier;
    use crate::steam_sys::U32Modifier;

    use std::path::PathBuf;
    use std::time;

    type InnerSystemModifier = crate::system_modifier::SystemModifier<MMTModifier, ADCModifier, U32Modifier>;

    static MMT_PATH: &[&str] = &["assets", "MultiMonitorTool.exe"];
    static ADC_PATH: &[&str] = &["assets", "AudioDeviceCmdlets.dll"];

    const DEFAULT_SLEEP_INTERVAL_SECS: u64 = 5;
    const DEFAULT_MAX_ATTEMPTS: usize = 5;

    pub struct SystemModifier {
        inner: InnerSystemModifier,
    }

    impl SystemModifier {
        pub fn new(steam_exe_path: PathBuf) -> SystemModifier {
            SystemModifier { inner: InnerSystemModifier {
                display_modifier: MMTModifier::new(MMT_PATH.iter().collect()),
                audio_modifier: ADCModifier::new(ADC_PATH.iter().collect()),
                steam_modifier: U32Modifier::new(steam_exe_path),
                max_attempts: DEFAULT_MAX_ATTEMPTS,
                sleep_interval: time::Duration::from_secs(DEFAULT_SLEEP_INTERVAL_SECS),
            }}
        }
    }
}
pub use public::SystemModifier;