#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SteamState {
    NotRunning,
    RunningNormal,
    RunningBigPicture,
}

pub trait SteamModifier {
    fn new(path: std::path::PathBuf) -> Self;

    fn get_system_state(&self) -> Result<SteamState, crate::Error>;

    fn kill_steam(&self) -> Result<(), crate::Error>;
    fn launch_steam(&self) -> Result<(), crate::Error>;
    fn launch_big_picture(&self) -> Result<(), crate::Error>;
    fn enable_big_picture(&self) -> Result<(), crate::Error>;
    fn disable_big_picture(&self) -> Result<(), crate::Error>;

    fn check_and_modify(&self, desired_state: &SteamState) -> Result<bool, crate::Error> {
        let system_state: SteamState = self.get_system_state()?;
        match (&system_state, desired_state) {
            (_, _) if &system_state == desired_state => Ok(false),
            (_, SteamState::NotRunning) => self.kill_steam().and(Ok(true)),
            (SteamState::NotRunning, SteamState::RunningNormal) => self.launch_steam().and(Ok(true)),
            (SteamState::NotRunning, SteamState::RunningBigPicture) => self.launch_big_picture().and(Ok(true)),
            (SteamState::RunningNormal, SteamState::RunningBigPicture) => self.enable_big_picture().and(Ok(true)),
            (SteamState::RunningBigPicture, SteamState::RunningNormal) => self.disable_big_picture().and(Ok(true)),
            (_, _) => unreachable!(),
        }
    }
}