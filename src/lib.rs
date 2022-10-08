use std::path::Path;

mod error;
pub use error::Error;

mod desired_state;
pub use desired_state::{DesiredState, DesiredSteamState};

mod command;

pub struct Executor<'a> {
    _mmt_path: &'a Path,
    _nircmd_path: &'a Path,
}

impl<'a> Executor<'a> {
    pub fn execute<S>(&self, _desired_state: DesiredState<S>) -> Result<(), crate::Error>
    where
        S: AsRef<str>
    {
        unimplemented!()
    }
}