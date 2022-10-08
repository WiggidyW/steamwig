use std::ffi::CString;

fn main() {
    let lpClassName = CString::new("CUIEngineWin32").unwrap();
    let lpWindowName = CString::new("Steam").unwrap();
    unsafe {
        let i = winapi::um::winuser::FindWindowA(lpClassName.as_ptr(), lpWindowName.as_ptr());
        let ii = winapi::um::winuser::IsWindow(i);
        println!("Big Picture: {}", ii);
        let i = winapi::um::winuser::FindWindowA(std::ptr::null_mut(), lpWindowName.as_ptr());
        let ii = winapi::um::winuser::IsWindow(i);
        println!("Steam: {}", ii);
    }
    println!("Hello World!")
}