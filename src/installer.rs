use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

fn install_program() -> io::Result<()> {
    // Define paths
    let install_dir = r"C:/Program Files/Redirectify";
    let executable_path = r"./target/release/redirectify.exe"; // Change as needed

    // Ensure the directory exists
    println!("fs::create_dir_all({install_dir})");
    fs::create_dir_all(install_dir)?;

    // Copy the executable to the install directory
    let dest_path = Path::new(install_dir).join("redirectify.exe");
    println!("fs::copy({executable_path}, {dest_path:?})?");
    fs::copy(executable_path, dest_path)?;

    // Register the program as the handler for HTTP and HTTPS
    println!("register_protocols");
    register_protocols()?;

    // Create a registry entry for uninstallation
    println!("create_uninstaller");
    create_uninstaller()?;

    println!("Redirectify installed successfully!");
    Ok(())
}

fn register_protocols() -> io::Result<()> {
    let install_dir = r"C:\Program Files\Redirectify";

    // Register the HTTP and HTTPS protocols in the registry
    Command::new("reg")
        .args(&[
            "add",
            r"HKEY_CURRENT_USER\Software\Classes\http\shell\open\command",
            "/v",
            "",
            "/t",
            "REG_SZ",
            "/d",
            &format!("\"{}\\redirectify.exe\" \"%1\"", install_dir),
            "/f",
        ])
        .output()?;

    Command::new("reg")
        .args(&[
            "add",
            r"HKEY_CURRENT_USER\Software\Classes\https\shell\open\command",
            "/v",
            "",
            "/t",
            "REG_SZ",
            "/d",
            &format!("\"{}\\redirectify.exe\" \"%1\"", install_dir),
            "/f",
        ])
        .output()?;

    // Add associations to start menu
    Command::new("reg")
        .args(&[
            "add",
            r"HKEY_CURRENT_USER\Software\Clients\StartMenuInternet\Redirectify",
            "/v",
            "LocalizedString",
            "/t",
            "REG_SZ",
            "/d",
            "Redirectify",
            "/f",
        ])
        .output()?;

    Ok(())
}

fn create_uninstaller() -> io::Result<()> {
    // let uninstall_cmd = r"C:\Program Files\Redirectify\uninstall.exe"; // Simple uninstaller script path
    let uninstall_script = r"uninstall.bat";

    // Write a simple batch file to remove the program and registry keys
    let mut file = fs::File::create(uninstall_script)?;
    file.write_all(
        format!(
            "@echo off\n\
        del \"{}\\redirectify.exe\"\n\
        reg delete \"HKEY_CURRENT_USER\\Software\\Classes\\http\" /f\n\
        reg delete \"HKEY_CURRENT_USER\\Software\\Classes\\https\" /f\n\
        reg delete \"HKEY_CURRENT_USER\\Software\\Clients\\StartMenuInternet\\Redirectify\" /f\n\
        echo Uninstallation complete.\n\
        pause\n",
            r"C:\Program Files\Redirectify"
        )
        .as_bytes(),
    )?;

    Ok(())
}

fn main() {
    match install_program() {
        Ok(_) => println!("Installation complete!"),
        Err(e) => eprintln!("Error during installation: {}", e),
    }
}
