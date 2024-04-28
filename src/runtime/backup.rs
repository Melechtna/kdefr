use chrono::Utc;
use std::fs::{create_dir_all, remove_dir_all};
use std::path::PathBuf;
use std::process::Command;

pub fn backup_kde_settings(home: Vec<PathBuf>, cache: Vec<PathBuf>, config: Vec<PathBuf>, local_share: Vec<PathBuf>, username: &str) {
    println!("Backing up files...");

    // Create backup folder
    let backup_dir_home = format!("/home/{}/.backup", username);
    let backup_dir_cache = format!("/home/{}/.backup/.cache", username);
    let backup_dir_config = format!("/home/{}/.backup/.config", username);
    let backup_dir_local_share = format!("/home/{}/.backup/.local/share", username);
    let _ = create_dir_all(&backup_dir_home);
    let _ = create_dir_all(&backup_dir_cache);
    let _ = create_dir_all(&backup_dir_config);
    let _ = create_dir_all(&backup_dir_local_share);

    // Copy files using rsync (with -a flag for preserving attributes and -r for recursive)

    //Capture home root files/folders
    let mut command = Command::new("rsync");
    command.arg("-a");
    command.arg("-r");
    let os_str_args = home.iter().map(|path| path.as_os_str()).collect::<Vec<_>>();

    command.args(os_str_args);
    command.arg(&backup_dir_home);
    let _ = command.output();

    //Capture home cache files/folders
    let mut command = Command::new("rsync");
    command.arg("-a");
    command.arg("-r");
    let os_str_args = cache
        .iter()
        .map(|path| path.as_os_str())
        .collect::<Vec<_>>();

    command.args(os_str_args);
    command.arg(&backup_dir_cache);
    let _ = command.output();

    //Capture home config files/folders
    let mut command = Command::new("rsync");
    command.arg("-a");
    command.arg("-r");
    let os_str_args = config
        .iter()
        .map(|path| path.as_os_str())
        .collect::<Vec<_>>();

    command.args(os_str_args);
    command.arg(&backup_dir_config);
    let _ = command.output();

    //Capture home local share files/folders
    let mut command = Command::new("rsync");
    command.arg("-a");
    command.arg("-r");
    let os_str_args = local_share
        .iter()
        .map(|path| path.as_os_str())
        .collect::<Vec<_>>();

    command.args(os_str_args);
    command.arg(&backup_dir_local_share);
    let _ = command.output();

    // Use chrono to get current date and time for archive name
    let now = Utc::now();
    let formatted_time = now.format("%d-%m-%Y-%H%M%S").to_string();

    // Create tar archive with username and date
    let archive_name = format!("/home/{}/kdefr-backup-{}.tar.gz", username, formatted_time);

    // Call tar command to create archive from backup directory
    let mut command = Command::new("tar");
    command.arg("-czvf");
    command.arg(&archive_name);
    command.arg("-C");
    command.arg(&backup_dir_home);
    command.arg(".");
    let _ = command.output();

    // Delete backup folder
    let _ = remove_dir_all(&backup_dir_home);

    println!("Backup complete!");
}
