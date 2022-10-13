use crate::audio::{AudioState, AudioModifier};
use crate::error::ADCParseError;

use std::path::{Path, PathBuf};
use std::process;

use lazy_static::lazy_static;
use powershell_script;
use regex::Regex;

#[derive(Debug)]
pub struct ADCModifier {
    pub (crate) module_path: PathBuf,
}

impl AudioModifier for ADCModifier {
    fn get_system_state(&self) -> Result<AudioState, crate::Error> {
        Ok(AudioState {
            primary_device_id: get_primary_device(&self.module_path)?,
            volume: Some(get_volume(&self.module_path)?),
            muted: Some(get_muted(&self.module_path)?),
        })
    }

    fn set_primary_device(&self, id: &str) -> Result<(), crate::Error> {
        set_primary_device(&self.module_path, id)
    }

    fn set_volume(&self, volume: u8) -> Result<(), crate::Error> {
        set_volume(&self.module_path, volume)
    }
    
    fn set_muted(&self, muted: bool) -> Result<(), crate::Error> {
        set_muted(&self.module_path, muted)
    }
}

impl ADCModifier {
    pub fn new(module_path: std::path::PathBuf) -> Self {
        ADCModifier { module_path: module_path }
    }
}

fn powershell_run(module_path: &Path, args: &[&str]) -> Result<process::Output, crate::Error> {
    let mut script: String = format!("Import-Module {};", module_path.to_string_lossy());
    for arg in args {
        script.push(' ');
        script.push_str(arg);
    }
    match powershell_script::run(&script) {
        Ok(output) => Ok(output.into_inner()),
        Err(e) => Err(crate::Error::PowershellError(e)),
    }
}

fn get_primary_device(module_path: &Path) -> Result<String, crate::Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"\{\d\.\d\.\d\.(\d{8})\}\.\{([a-z]|\d){8}-([a-z]|\d){4}-([a-z]|\d){4}-([a-z]|\d){4}-([a-z]|\d){12}\}",
        ).unwrap();
    }
    let output: process::Output = powershell_run(module_path, &["Get-AudioDevice", "-Playback"])?;
    let stdout_str: &str = &String::from_utf8_lossy(&output.stdout);
    match RE.find(stdout_str).map(|m| m.as_str()) {
        Some(device_id) => Ok(device_id.to_string()),
        _ => Err(crate::Error::ADCParseError(ADCParseError {
            output: output.stdout,
            description: "should contain a device ID string 55 characters long",
        })),
    }
}

fn get_volume(module_path: &Path) -> Result<u8, crate::Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(100)|(\d{1,2})").unwrap();
    }
    let output: process::Output = powershell_run(module_path, &["Get-AudioDevice", "-PlaybackVolume"])?;
    let stdout_str: &str = &String::from_utf8_lossy(&output.stdout);
    match RE.find(stdout_str).map(|m| m.as_str().parse::<u8>()) {
        Some(Ok(volume)) => Ok(volume),
        _ => Err(crate::Error::ADCParseError(ADCParseError {
            output: output.stdout,
            description: "should contain an integer from 0 to 100",
        })),
    }
}

fn get_muted(module_path: &Path) -> Result<bool, crate::Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(True)|(False)").unwrap();
    }
    let output: process::Output = powershell_run(module_path, &["Get-AudioDevice", "-PlaybackMute"])?;
    let stdout_str: &str = &String::from_utf8_lossy(&output.stdout);
    match RE.find(stdout_str).map(|m| m.as_str()) {
        Some("True") => Ok(true),
        Some("False") => Ok(false),
        _ => Err(crate::Error::ADCParseError(ADCParseError {
            output: output.stdout,
            description: "should contain 'True' or 'False'",
        })),
    }
}

fn set_primary_device(module_path: &Path, id: &str) -> Result<(), crate::Error> {
    powershell_run(module_path, &["Set-AudioDevice", "-ID", id]).map(|_| ())
}

fn set_volume(module_path: &Path, volume: u8) -> Result<(), crate::Error> {
    powershell_run(module_path, &["Set-AudioDevice", "-ID", &volume.to_string()]).map(|_| ())
}

fn set_muted(module_path: &Path, muted: bool) -> Result<(), crate::Error> {
    let muted_str: &str = match muted {
        true => "True",
        false => "False",
    };
    powershell_run(module_path, &["Set-AudioDevice", "-ID", muted_str]).map(|_| ())
}