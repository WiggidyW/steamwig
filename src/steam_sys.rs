use crate::steam::{SteamState, SteamModifier};

use std::path::{Path, PathBuf};
use std::ffi::CString;
use std::process;

use lazy_static::lazy_static;
use winapi::shared::windef;
use winapi::um::winuser;

#[derive(Debug)]
pub struct U32Modifier {
    pub (crate) exe_path: PathBuf,
}

impl SteamModifier for U32Modifier {
    fn get_system_state(&self) -> Result<SteamState, crate::Error> {
        Ok(get_system_state())
    }

    fn kill_steam(&self) -> Result<(), crate::Error> {
        kill_steam()
    }

    fn launch_steam(&self) -> Result<(), crate::Error> {
        launch_steam(&self.exe_path)
    }

    fn launch_big_picture(&self) -> Result<(), crate::Error> {
        launch_big_picture(&self.exe_path)
    }

    fn enable_big_picture(&self) -> Result<(), crate::Error> {
        self.launch_big_picture()
    }

    fn disable_big_picture(&self) -> Result<(), crate::Error> {
        self.kill_steam()?;
        self.launch_steam()
    }
}

impl U32Modifier {
    pub fn new(exe_path: std::path::PathBuf) -> Self {
        U32Modifier { exe_path: exe_path }
    }
}

fn get_system_state() -> SteamState {
    lazy_static! {
        static ref LP_CLASS_NAME: CString = CString::new("CUIEngineWin32").unwrap();
        static ref LP_WINDOW_NAME: CString = CString::new("Steam").unwrap();
    }
    let lp_class_name_ptr: *const i8 = LP_CLASS_NAME.as_ptr();
    let lp_window_name_ptr: *const i8 = LP_WINDOW_NAME.as_ptr();
    if unsafe {is_big_picture_running(lp_class_name_ptr, lp_window_name_ptr)} {
        SteamState::RunningBigPicture
    }
    else if unsafe {is_steam_running(lp_window_name_ptr)} {
        SteamState::RunningNormal
    }
    else {
        SteamState::NotRunning
    }
}

unsafe fn is_big_picture_running(lp_class_name_ptr: *const i8, lp_window_name_ptr: *const i8) -> bool {
    let hwnd: windef::HWND = winuser::FindWindowA(lp_class_name_ptr, lp_window_name_ptr);
    match winuser::IsWindow(hwnd) {
        0 => false,
        1 => true,
        _ => unreachable!(),
    }
}

unsafe fn is_steam_running(lp_window_name_ptr: *const i8) -> bool {
    let hwnd: windef::HWND = winuser::FindWindowA(std::ptr::null_mut(), lp_window_name_ptr);
    match winuser::IsWindow(hwnd) {
        0 => false,
        1 => true,
        _ => unreachable!(),
    }
}

fn command_run(arg: &str, extra_args: &[&str]) -> Result<process::Output, crate::Error> {
    process::Command::new(arg)
        .args(extra_args)
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .output()
        .map_err(|e| crate::Error::CommandError(e))
}

fn kill_steam() -> Result<(), crate::Error> {
    command_run("taskkill", &["/F", "/IM", "steam.exe"]).map(|_| ())
}

fn launch_steam(exe_path: &Path) -> Result<(), crate::Error> {
    command_run(&exe_path.to_string_lossy(), &[]).map(|_| ())
}

fn launch_big_picture(exe_path: &Path) -> Result<(), crate::Error> {
    command_run(&exe_path.to_string_lossy(), &["-start", "steam://open/bigpicture"]).map(|_| ())
}