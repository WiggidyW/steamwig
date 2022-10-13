use std::path::Path;

use tempfile::TempDir;
use csv;

#[derive(Debug, Clone)]
pub struct DisplayState {
    displays: Vec<DisplayStateSingle>,
    primary_id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DisplayStateSingle {
    id: String,
    enabled: bool,
}

pub fn modify_system_if_needed(
    desired_state: &DisplayState,
    executable_path: &Path,
) -> Result<Option<()>, crate::Error> {
    let system_state: DisplayState = get_system_display_state(executable_path)?;
    let modify_system_objectives: DisplayState = match get_modify_system_objectives(
        desired_state,
        &system_state,
    ) {
        Ok(Some(modify_system_objectives)) => modify_system_objectives,
        Ok(None) => return Ok(None),
        Err(e) => return Err(e),
    };
    modify_system(&modify_system_objectives, executable_path).map(|_| Some(()))
}

fn modify_system(
    modify_system_objectives: &DisplayState,
    executable_path: &Path,
) -> Result<(), crate::Error> {
    let mut enable_monitors: Vec<&str> = Vec::with_capacity(
        modify_system_objectives.displays.len()
    );
    let mut disable_monitors: Vec<&str> = Vec::with_capacity(
        modify_system_objectives.displays.len()
    );
    for display_state_single in &modify_system_objectives.displays {
        match display_state_single.enabled {
            true => enable_monitors.push(display_state_single.id.as_str()),
            false => disable_monitors.push(display_state_single.id.as_str()),
        }
    }

    if enable_monitors.len() > 0 {
        mmt_commands::configure_monitors(executable_path, &enable_monitors, true)?;
    }
    if disable_monitors.len() > 0 {
        mmt_commands::configure_monitors(executable_path, &disable_monitors, false)?;
    }
    if !modify_system_objectives.primary_id.is_empty() {
        mmt_commands::primary_monitor(
            executable_path,
            modify_system_objectives.primary_id.as_str(),
        )?;
    }
    Ok(())
}

fn get_modify_system_objectives(
    desired_state: &DisplayState,
    system_state: &DisplayState,
) -> Result<Option<DisplayState>, crate::Error> {
    let mut modify_system_objectives: DisplayState = desired_state.clone();
    if &system_state.primary_id == &modify_system_objectives.primary_id {
        modify_system_objectives.primary_id = String::new();
    }
    for display_state_single in &system_state.displays {
        for i in 0..modify_system_objectives.displays.len() {
            if &modify_system_objectives.displays[i] == display_state_single {
                modify_system_objectives.displays.swap_remove(i);
                break;
            }
        }
    }
    match (
        modify_system_objectives.primary_id.is_empty(),
        modify_system_objectives.displays.is_empty(),
    ) {
        (true, true) => Ok(None),
        (_, _) => Ok(Some(modify_system_objectives)),
    }
}

fn get_system_display_state(executable_path: &Path) -> Result<DisplayState, crate::Error> {
    let temp_dir = TempDir::new().map_err(|e| crate::Error::TempDirError(e))?;
    let output_path = temp_dir.path().join("display_state.csv");
    mmt_commands::scomma(executable_path, &output_path)?;
    display_state_from_csv(&output_path)
}

// display_state_record.get(3) Active
// display_state_record.get(5) Primary
// display_state_record.get(17) Monitor Name
fn display_state_from_csv(csv_path: &Path) -> Result<DisplayState, crate::Error> {
    let mut display_state: DisplayState = DisplayState {
        displays: Vec::new(),
        primary_id: String::new(),
    };
    for display_state_single_record in csv::Reader::from_path(csv_path)
        .map_err(|e| crate::Error::CsvError(e))?
        .into_records() {
        let display_state_single_record = display_state_single_record
            .map_err(|e| crate::Error::CsvError(e))?;
        let display_state_single: DisplayStateSingle = DisplayStateSingle {
            id: match display_state_single_record.get(17) {
                Some(s) => s.to_owned(),
                None => return Err(crate::Error::MalformedCsvError),
            },
            enabled: match display_state_single_record.get(3) {
                Some("Yes") => true,
                Some("No") => false,
                Some(_) | None => return Err(crate::Error::MalformedCsvError),
            },
        };
        if display_state_single_record.get(5) == Some("Yes") {
            display_state.primary_id = display_state_single.id.clone();
        }
    }
    Ok(display_state)
}

mod mmt_commands {
    use std::process;
    use std::path::Path;

    pub fn configure_monitors(
        executable_path: &Path,
        monitor_ids: &Vec<&str>,
        enable: bool,
    ) -> Result<process::Output, crate::Error> {
        let mut command = process::Command::new(executable_path);
        command.stdin(process::Stdio::null());
        match enable {
            true => command.arg("/enable"),
            false => command.arg("/disable"),
        };
        for monitor_id in monitor_ids {
            command.arg(monitor_id);
        }
        command.output().map_err(|e| crate::Error::CommandError(e))
    }

    pub fn primary_monitor(
        executable_path: &Path,
        monitor_id: &str,
    ) -> Result<process::Output, crate::Error> {
        process::Command::new(executable_path).stdin(process::Stdio::null())
            .arg("/SetPrimary")
            .arg(monitor_id)
            .output()
            .map_err(|e| crate::Error::CommandError(e))
    }

    pub fn scomma(
        executable_path: &Path,
        output_path: &Path,
    ) -> Result<process::Output, crate::Error> {
        process::Command::new(executable_path).stdin(process::Stdio::null())
            .arg("/scomma")
            .arg(output_path)
            .output()
            .map_err(|e| crate::Error::CommandError(e))
    }
}