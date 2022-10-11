use std::process;
use std::path::Path;
use std::ffi::CString;

use winapi::um::winuser;
use winapi::shared::windef;

#[derive(Debug, Clone, PartialEq)]
pub enum SteamState {
    NotRunning,
    RunningNormal,
    RunningBigPicture,
}

#[derive(Debug, PartialEq)]
enum ModifySystemObjectives {
    Kill,
    KillThenRunNormal,
    RunNormal,
    RunBigPicture,
}

pub fn modify_system_if_needed(
    desired_state: &SteamState,
    executable_path: &Path,
) -> Result<Option<()>, crate::Error> {
    let system_state: SteamState = get_system_steam_state()?;
    let modify_system_objectives: ModifySystemObjectives = match get_modify_system_objectives(
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
    modify_system_objectives: &ModifySystemObjectives,
    executable_path: &Path,
) -> Result<(), crate::Error> {
    match modify_system_objectives {
        ModifySystemObjectives::Kill | ModifySystemObjectives::KillThenRunNormal =>
            process::Command::new("taskkill").stdin(process::Stdio::null())
            .arg("/F")
            .arg("/IM")
            .arg("steam.exe")
            .output(),
        ModifySystemObjectives::RunNormal => process::Command::new(executable_path)
            .stdin(process::Stdio::null())
            .output(),
        ModifySystemObjectives::RunBigPicture => process::Command::new(executable_path)
            .stdin(process::Stdio::null())
            .arg("-start")
            .arg("steam://open/bigpicture")
            .output(),
    }.map_err(|e| crate::Error::CommandError(e))?;
    if modify_system_objectives == &ModifySystemObjectives::KillThenRunNormal {
        process::Command::new(executable_path).stdin(process::Stdio::null())
            .output()
            .map_err(|e| crate::Error::CommandError(e))?;
    }
    Ok(())
}

fn get_modify_system_objectives(
    desired_state: &SteamState,
    system_state: &SteamState,
) -> Result<Option<ModifySystemObjectives>, crate::Error> {
    match (desired_state, system_state) {
        (SteamState::NotRunning, SteamState::NotRunning)
            | (SteamState::RunningNormal, SteamState::RunningNormal)
            | (SteamState::RunningBigPicture, SteamState::RunningBigPicture) =>
            Ok(None),
        (SteamState::NotRunning, _) => Ok(Some(ModifySystemObjectives::Kill)),
        (SteamState::RunningNormal, SteamState::NotRunning) =>
            Ok(Some(ModifySystemObjectives::RunNormal)),
        (SteamState::RunningBigPicture, SteamState::NotRunning) =>
            Ok(Some(ModifySystemObjectives::RunBigPicture)),
        (SteamState::RunningBigPicture, SteamState::RunningNormal) =>
            Ok(Some(ModifySystemObjectives::RunBigPicture)),
        (SteamState::RunningNormal, SteamState::RunningBigPicture) =>
            Ok(Some(ModifySystemObjectives::KillThenRunNormal)),
    }
}

fn get_system_steam_state() -> Result<SteamState, crate::Error> {
    let lp_class_name: CString = CString::new("CUIEngineWin32").unwrap();
    let lp_window_name: CString = CString::new("Steam").unwrap();
    unsafe {
        let big_picture_hwnd: windef::HWND = winuser::FindWindowA(
            lp_class_name.as_ptr(),
            lp_window_name.as_ptr(),
        );
        if winuser::IsWindow(big_picture_hwnd) == 1 {
            return Ok(SteamState::RunningBigPicture);
        }
        let normal_hwnd: windef::HWND = winuser::FindWindowA(
            std::ptr::null_mut(),
            lp_window_name.as_ptr(),
        );
        if winuser::IsWindow(normal_hwnd) == 1 {
            return Ok(SteamState::RunningNormal);
        }
    }
    Ok(SteamState::NotRunning)
}