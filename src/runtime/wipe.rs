use std::fs::{metadata, remove_dir_all, remove_file};
use std::io::BufRead;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use std::{io, thread};
use crate::runtime;

pub fn wipe_kde_settings(
    home: Vec<PathBuf>,
    cache: Vec<PathBuf>,
    config: Vec<PathBuf>,
    local_share: Vec<PathBuf>,
    username: &String,
) {
    // State which user we're working on
    println!("\nThis will erase all core KDE settings files and bring KDE back to a");
    println!("first run state, are you sure you wish to continue?");
    println!("\nPress Ctrl+C to cancel or press Enter to continue...");

    // Wait for the user to press Enter to continue
    let stdin = io::stdin();
    let _ = stdin.lock().lines().next();
    println!("Performing wipe for user {}.\n", username);
    println!("Please do not interact with your computer until a reboot occurs");
    println!("Any interaction with KDE can and will restore settings mid wipe");
    println!("All but the Konsole will be shutdown to avoid config regeneration");
    println!("Errors during wipe are expected and normal. Beginning wipe...");

    // Wait for 5 seconds so the user can read the above text
    thread::sleep(Duration::from_secs(5));

    //Kill everything plasma related we can except for the konsole
    runtime::process_handling::kill_plasma_processes();

    // Wipe home files/folders
    wipe_paths_with_logging(&home);

    // Wipe cache files/folders
    wipe_paths_with_logging(&cache);

    // Wipe config files/folders
    wipe_paths_with_logging(&config);

    // Wipe local/share files/folders
    wipe_paths_with_logging(&local_share);

    // Append completion message
    println!("Wipe script execution completed. Syncing disk...");

    //Sync the disk so changes are assured
    let _ = Command::new("sync")
        .output()
        .expect("Failed to execute sync command");

    // Append completion message
    println!("Disk sync completed. Await reboot...");

    // Wait for 10 seconds for the disk to catch up
    thread::sleep(Duration::from_secs(10));

    // Reboot the system using qdbus
    match Command::new("qdbus")
        .arg("org.kde.Shutdown")
        .arg("/Shutdown")
        .arg("logoutAndReboot")
        .output()
    {
        Ok(_) => println!("Rebooting..."),
        Err(e) => eprintln!("Failed to reboot: {}", e),
    }
}

pub fn wipe_paths_with_logging(paths: &[PathBuf]) {
    for path in paths {
        match metadata(path) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    // It's a directory, so remove it and all its contents
                    if let Err(err) = remove_dir_all(path) {
                        println!("Failed to remove {}: {}\n", path.display(), err);
                    } else {
                        println!("Successfully removed directory: {}\n", path.display());
                    }
                } else if metadata.is_file() {
                    // It's a file, so remove it
                    if let Err(err) = remove_file(path) {
                        println!("Failed to remove {}: {}\n", path.display(), err);
                    } else {
                        println!("Successfully removed file: {}\n", path.display());
                    }
                } else {
                    // It's neither a directory nor a file, handle the error case
                    println!("Invalid entry: {}\n", path.display());
                }
            }
            Err(err) => {
                // Handle the case where metadata retrieval failed
                println!(
                    "Failed to retrieve metadata for {}: {}\n",
                    path.display(),
                    err
                );
            }
        }
    }
}
