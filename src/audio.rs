#[derive(Debug, Clone, PartialEq)]
pub struct AudioState {
    pub (crate) primary_device_id: String,
    pub (crate) volume: Option<u8>,
    pub (crate) muted: Option<bool>,
}

#[derive(Debug)]
struct AudioModifierObjectives<'a> {
    primary_device_id: &'a str,
    volume: Option<u8>,
    muted: Option<bool>,
}

pub trait AudioModifier {
    fn new(path: std::path::PathBuf) -> Self;

    fn get_system_state(&self) -> Result<AudioState, crate::Error>;

    fn set_primary_device(&self, primary_device_id: &str) -> Result<(), crate::Error>;
    fn set_volume(&self, volume: u8) -> Result<(), crate::Error>;
    fn set_muted(&self, muted: bool) -> Result<(), crate::Error>;

    fn check_and_modify(&self, desired_state: &AudioState) -> Result<bool, crate::Error> {
        let system_state: AudioState = self.get_system_state()?;
        let objectives: AudioModifierObjectives = match get_objectives(&desired_state, &system_state) {
            Some(o) => o,
            None => return Ok(false),
        };
        if !objectives.primary_device_id.is_empty() {
            self.set_primary_device(&objectives.primary_device_id)?;
        }
        if !objectives.volume.is_none() {
            self.set_volume(objectives.volume.unwrap())?;
        }
        if !objectives.muted.is_none() {
            self.set_muted(objectives.muted.unwrap())?;
        }
        Ok(true)
    }
}

fn get_objectives<'a>(
    desired_state: &'a AudioState,
    system_state: &AudioState,
) -> Option<AudioModifierObjectives<'a>> {
    let primary_device_id: &str = match &desired_state.primary_device_id == &system_state.primary_device_id {
        true => "",
        false => &desired_state.primary_device_id,
    };
    let volume: Option<u8> = match (&desired_state.volume == &system_state.volume, primary_device_id.is_empty()) {
        (true, true) => None,
        _ => desired_state.volume,
    };
    let muted: Option<bool> = match (&desired_state.muted == &system_state.muted, primary_device_id.is_empty()) {
        (true, true) => None,
        _ => desired_state.muted,
    };

    match (primary_device_id.is_empty(), volume.is_none(), muted.is_none()) {
        (true, true, true) => None,
        _ => Some(AudioModifierObjectives {
            primary_device_id: primary_device_id,
            volume: volume,
            muted: muted,
        })
    }
}

impl AudioState {
    pub fn new() -> Self {
        AudioState {
            primary_device_id: String::new(),
            volume: None,
            muted: None,
        }
    }
}