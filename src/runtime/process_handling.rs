use std::process::Command;

pub fn kill_plasma_processes() {
    // List of plasma-related processes to kill
    let plasma_processes = [
        "plasmashell",
        "ksmserver",
        "kwalletd5",
        "kwalletd6",
        "plasmanetworkmanagement",
        "spectacle",
        "kdeconnect",
        "kded5",
        "kded6"
    ];

    // Check if kquitapp5 or kquitapp6 is available
    let kquitapp = if Command::new("kquitapp5").output().is_ok() {
        "kquitapp5"
    } else if Command::new("kquitapp6").output().is_ok() {
        "kquitapp6"
    } else {
        eprintln!("Neither kquitapp5 nor kquitapp6 is available");
        return;
    };

    // Kill each plasma-related process
    for process in &plasma_processes {
        let _ = Command::new(kquitapp)
            .arg(process)
            .output()
            .map_err(|e| eprintln!("Failed to kill {}: {}", process, e));
    }
}
