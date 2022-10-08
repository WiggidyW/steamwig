use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct AudioState {
    primary_id: String,
}

pub fn modify_system_if_needed(
    desired_state: &AudioState,
    module_path: &Path,
) -> Result<Option<()>, crate::Error> {
    let system_state: AudioState = get_system_audio_state(module_path)?;
    let modify_system_objectives: AudioState = match get_modify_system_objectives(
        desired_state,
        &system_state,
    ) {
        Ok(Some(modify_task)) => modify_task,
        Ok(None) => return Ok(None),
        Err(e) => return Err(e),
    };
    modify_system(&modify_system_objectives, module_path).map(|_| Some(()))
}

fn modify_system(
    modify_system_objectives: &AudioState,
    module_path: &Path,
) -> Result<(), crate::Error> {
    adc_commands::set_primary_audio_device(module_path, &modify_system_objectives.primary_id)
        .map(|_| ())
}

fn get_modify_system_objectives(
    desired_state: &AudioState,
    system_state: &AudioState,
) -> Result<Option<AudioState>, crate::Error> {
    match desired_state == system_state {
        true => Ok(None),
        false => Ok(Some(desired_state.clone())),
    }
}

fn get_system_audio_state(module_path: &Path) -> Result<AudioState, crate::Error> {
    let mut system_audio_state: AudioState = AudioState {
        primary_id: String::new()
    };
    let system_audio_state_output: String = adc_commands::get_primary_audio_device(module_path)?
        .stdout()
        .ok_or(crate::Error::MalformedPowershellError)?;
    for line in system_audio_state_output.lines() {
        if line.get(0..2) == Some("ID") {
            system_audio_state.primary_id = line.get(line.len() - 55..line.len())
                .ok_or(crate::Error::MalformedPowershellError)?
                .to_string();
        }
    }
    match system_audio_state.primary_id.as_str() {
        "" => Err(crate::Error::MalformedPowershellError),
        _ => Ok(system_audio_state),
    }
}

mod adc_commands {
    use std::path::Path;

    pub fn set_primary_audio_device(
        module_path: &Path,
        audio_device_id: &str,
    ) -> Result<powershell_script::Output, crate::Error> {
        let script: String = format!("Import-Module {}; Set-AudioDevice -ID {}",
            module_path.to_string_lossy(),
            audio_device_id,
        );
        powershell_script::run(&script).map_err(|e| crate::Error::PowershellError(e))
    }

    pub fn get_primary_audio_device(
        module_path: &Path,
    ) -> Result<powershell_script::Output, crate::Error> {
        let script: String = format!("Import-Module {}; Get-AudioDevice -Playback",
            module_path.to_string_lossy(),
        );
        powershell_script::run(&script).map_err(|e| crate::Error::PowershellError(e))
    }
}