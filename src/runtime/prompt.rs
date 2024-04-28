use std::env;
use dialoguer::Input;
use std::io::{self, BufRead};

pub fn prompt() -> (String, bool) {
    // Get the current username from the environment variable
    let username = env::var("USER").unwrap_or_else(|_| String::from("unknown"));

    // Display a warning message
    println!("This script will wipe your ({}\'s) KDE settings and reboot,", username);
    println!("Please save any work, you will be prompted if you'd like a backup.");
    println!("Then take your normal procedures as you would when rebooting your system,");
    println!("before letting this script continue.");
    println!("\nPress Ctrl+C to cancel or press Enter to continue...");

    // Wait for the user to press Enter to continue
    let stdin = io::stdin();
    let _ = stdin.lock().lines().next();

    // Get the user's choice about creating a backup
    let input: String = Input::new()
        .with_prompt("Would you like to create a backup for everything deleted? (Y/N)")
        .interact()
        .unwrap();

    let should_backup = input.trim().to_uppercase().chars().next().unwrap() == 'Y';

    // Return both username and should_backup
    (username, should_backup)
}
