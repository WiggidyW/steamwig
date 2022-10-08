pub enum DesiredSteamState {
    BigPicture,
    Normal,
    Off,
}

pub struct DesiredState<S>
where
    S: AsRef<str>
{
    monitors: Vec<(S, bool)>,
    primary_monitor: Option<S>,
    primary_audio: Option<S>,
    steam: Option<DesiredSteamState>,
}

impl<S> DesiredState<S>
where
    S: AsRef<str>
{
    pub fn new() -> DesiredState<S> {
        return DesiredState {
            monitors: Vec::new(),
            primary_monitor: None,
            primary_audio: None,
            steam: None,
        }
    }

    pub fn enable_monitor(mut self, monitor_id: S) -> DesiredState<S> {
        self.monitors.push((monitor_id, true));
        self
    }

    pub fn disable_monitor(mut self, monitor_id: S) -> DesiredState<S> {
        self.monitors.push((monitor_id, false));
        self
    }

    pub fn primary_monitor(mut self, monitor_id: S) -> DesiredState<S> {
        self.primary_monitor = Some(monitor_id);
        self
    }

    pub fn primary_audio(mut self, audio_id: S) -> DesiredState<S> {
        self.primary_audio = Some(audio_id);
        self
    }

    pub fn steam(mut self, steam_state: DesiredSteamState) -> DesiredState<S> {
        self.steam = Some(steam_state);
        self
    }
}