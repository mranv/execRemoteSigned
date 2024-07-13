use winreg::enums::*;
use winreg::RegKey;
use winapi::um::sysinfoapi::GetVersionExW;
use winapi::um::winnt::OSVERSIONINFOW;

fn get_windows_version() -> Option<(u32, u32, u32)> {
    unsafe {
        let mut os_info: OSVERSIONINFOW = std::mem::zeroed();
        os_info.dwOSVersionInfoSize = std::mem::size_of::<OSVERSIONINFOW>() as u32;

        if GetVersionExW(&mut os_info as *mut _) != 0 {
            Some((os_info.dwMajorVersion, os_info.dwMinorVersion, os_info.dwBuildNumber))
        } else {
            None
        }
    }
}

fn main() {
    if let Some((major, minor, _build)) = get_windows_version() {
        println!("Windows version: {}.{}", major, minor);

        if major < 6 {
            println!("This script is not supported on Windows XP or earlier.");
            return;
        }

        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let key_path = r"SOFTWARE\Microsoft\PowerShell\1\ShellIds\Microsoft.PowerShell";

        match hklm.create_subkey(key_path) {
            Ok((key, _)) => {
                match key.set_value("ExecutionPolicy", &"RemoteSigned") {
                    Ok(_) => {
                        println!("PowerShell execution policy set to RemoteSigned.");
                    }
                    Err(e) => {
                        eprintln!("Failed to set the execution policy: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to open or create the registry key: {}", e);
            }
        }
    } else {
        eprintln!("Failed to get Windows version.");
    }
}
