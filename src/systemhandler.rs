use std::env;
use std::path::Path;
use std::fs;

pub fn GetOs() -> String {
    let ret: String = String::from(env::consts::OS);
    return ret;
}

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
        await!(fs::create_dir_all("/etc/Clidget/"));
        await!(fs::create_dir_all("/etc/Clidget/Core/"));
        await!(fs::create_dir_all("/etc/Clidget/Core/Accounts/"));
    }
}