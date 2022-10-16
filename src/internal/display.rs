#[derive(Debug, Clone, PartialEq)]
pub struct DisplayState {
    pub (crate) primary_device_id: String,
    pub (crate) enabled_device_ids: Vec<String>,
    pub (crate) disabled_device_ids: Vec<String>,
}

impl DisplayState {
    pub fn new() -> DisplayState {
        DisplayState {
            primary_device_id: String::new(),
            enabled_device_ids: Vec::new(),
            disabled_device_ids: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        if self.primary_device_id.is_empty()
            && self.enabled_device_ids.is_empty()
            && self.disabled_device_ids.is_empty()
        {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct DisplayModifierObjectives<'a> {
    primary_device_id: &'a str,
    enabled_device_ids: Vec<&'a str>,
    disabled_device_ids: Vec<&'a str>,
}

pub trait DisplayModifier {
    fn get_id_readout(&self) -> Result<String, crate::Error>;

    fn get_system_state(&self) -> Result<DisplayState, crate::Error>;

    fn enable_monitors(&self, device_ids: &[&str]) -> Result<(), crate::Error>;
    fn disable_monitors(&self, device_ids: &[&str]) -> Result<(), crate::Error>;
    fn set_primary_device(&self, id: &str) -> Result<(), crate::Error>;

    fn check_and_modify(&self, desired_state: &DisplayState) -> Result<bool, crate::Error> {
        if desired_state.is_empty() {
            return Ok(false)
        }
        let system_state: DisplayState = self.get_system_state()?;
        let objectives: DisplayModifierObjectives = match get_objectives(&desired_state, &system_state) {
            Some(o) => o,
            None => return Ok(false),
        };
        if !objectives.enabled_device_ids.is_empty() {
            self.enable_monitors(&objectives.enabled_device_ids)?;
        }
        if !objectives.disabled_device_ids.is_empty() {
            self.disable_monitors(&objectives.disabled_device_ids)?;
        }
        if !objectives.primary_device_id.is_empty() {
            self.set_primary_device(&objectives.primary_device_id)?;
        }
        Ok(true)
    }
}

fn get_objectives<'a>(
    desired_state: &'a DisplayState,
    system_state: &DisplayState,
) -> Option<DisplayModifierObjectives<'a>> {
    let primary_device_id: &str = match &desired_state.primary_device_id == &system_state.primary_device_id {
        true => "",
        false => &desired_state.primary_device_id,
    };
    let mut enabled_device_ids: Vec<&str> = Vec::with_capacity(desired_state.enabled_device_ids.len());
    for device_id in &desired_state.enabled_device_ids {
        if system_state.enabled_device_ids.contains(device_id) {
            enabled_device_ids.push(device_id);
        }
    }
    let mut disabled_device_ids: Vec<&str> = Vec::with_capacity(desired_state.disabled_device_ids.len());
    for device_id in &desired_state.disabled_device_ids {
        if system_state.disabled_device_ids.contains(device_id) {
            disabled_device_ids.push(device_id);
        }
    }

    match (primary_device_id.is_empty(), enabled_device_ids.is_empty(), disabled_device_ids.is_empty()) {
        (true, true, true) => None,
        _ => Some(DisplayModifierObjectives {
            primary_device_id: primary_device_id,
            enabled_device_ids: enabled_device_ids,
            disabled_device_ids: disabled_device_ids,
        }),
    }
}