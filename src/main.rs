use std::io;
use winreg::enums::*;
use winreg::RegKey;

fn main() {
    // Define the registry path and value
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key_path = r"SOFTWARE\Microsoft\PowerShell\1\ShellIds\Microsoft.PowerShell";

    // Open or create the registry key
    match hklm.create_subkey(key_path) {
        Ok((key, _)) => {
            // Set the execution policy to RemoteSigned
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
}
