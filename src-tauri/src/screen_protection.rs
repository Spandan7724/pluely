#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::{GetLastError, HWND},
    UI::WindowsAndMessaging::{
        GetWindowDisplayAffinity, SetWindowDisplayAffinity,
        WDA_EXCLUDEFROMCAPTURE, WDA_NONE,
    },
};

#[cfg(target_os = "windows")]
fn affinity_error(context: &str) -> String {
    unsafe {
        let code = GetLastError().0;
        format!("{} (win32 error code: {})", context, code)
    }
}

#[cfg(target_os = "windows")]
pub fn enable_screen_protection(hwnd: isize) -> Result<(), String> {
    unsafe {
        let window = HWND(hwnd);
        SetWindowDisplayAffinity(window, WDA_EXCLUDEFROMCAPTURE)
            .map_err(|_| affinity_error("Failed to enable screen capture protection"))
    }
}

#[cfg(not(target_os = "windows"))]
pub fn enable_screen_protection(_hwnd: isize) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn disable_screen_protection(hwnd: isize) -> Result<(), String> {
    unsafe {
        let window = HWND(hwnd);
        SetWindowDisplayAffinity(window, WDA_NONE)
            .map_err(|_| affinity_error("Failed to disable screen capture protection"))
    }
}

#[cfg(not(target_os = "windows"))]
pub fn disable_screen_protection(_hwnd: isize) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
fn is_protection_enabled(hwnd: isize) -> Result<bool, String> {
    unsafe {
        let window = HWND(hwnd);
        let mut affinity: u32 = 0;
        match GetWindowDisplayAffinity(window, &mut affinity as *mut u32) {
            Ok(_) => Ok(affinity == WDA_EXCLUDEFROMCAPTURE.0),
            Err(_) => Err(affinity_error("Failed to read window display affinity")),
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn is_protection_enabled(_hwnd: isize) -> Result<bool, String> {
    Ok(false)
}

#[tauri::command]
pub fn toggle_screen_protection(
    window: tauri::WebviewWindow,
    enabled: bool,
) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let hwnd = window
            .hwnd()
            .map_err(|e| format!("Failed to access window handle: {e}"))?;

        if enabled {
            enable_screen_protection(hwnd.0 as isize)
        } else {
            disable_screen_protection(hwnd.0 as isize)
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = window;
        let _ = enabled;
        Ok(())
    }
}

#[cfg(target_os = "windows")]
pub fn ensure_screen_protection(hwnd: isize) {
    match enable_screen_protection(hwnd) {
        Ok(_) => {
            if let Ok(true) = is_protection_enabled(hwnd) {
                eprintln!("✅ Screen capture protection enabled");
            }
        }
        Err(err) => {
            eprintln!("⚠️ Failed to enable screen protection: {}", err);
            eprintln!(
                "   This feature requires Windows 10 build 19041+ or Windows 11"
            );
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn ensure_screen_protection(_hwnd: isize) {}
