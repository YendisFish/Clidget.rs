use std::env;
use std::path::Path;
use std::fs;
use std::process;
use std::process::Command;

pub fn StartupChecks() {
    let baseDir: bool = Path::new("/etc/Clidget/").exists();
    let coreDir: bool = Path::new("/etc/Clidget/Core/").exists();

    if baseDir == false {
        println!("Clidget could not find the required directories! maybe try using the installer to generate them!");
    }

    if coreDir == false {
        println!("Clidget could not find core directory! Try reinstalling!");
    }
}

pub fn Installer() {
    let baseDir: bool = Path::new("/etc/Clidget/").exists();

    if baseDir == false {
        fs::create_dir_all("/etc/Clidget/");
        fs::create_dir_all("/etc/Clidget/Core/");
        fs::create_dir_all("/etc/Clidget/Core/Accounts/");
    }

    let currentPath = env::current_exe().unwrap();

    Command::new("mv")
        .arg(currentPath.as_os_str())
        .arg("/usr/bin/");
}