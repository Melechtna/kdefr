mod lists {
    pub mod cache;
    pub mod config;
    pub mod home;
    pub mod local_share;
    pub mod regexing;
}
mod runtime {
    pub mod backup;
    pub mod prompt;
    pub mod wipe;
    pub mod process_handling;
}

fn main() {
    // Retrieve username and should_backup from prompt function
    let (username, should_backup) = runtime::prompt::prompt();

    // Pass username and should_backup to the list function
    list(&username, should_backup);
}

pub fn list(username: &String, should_backup: bool) {
    // Root of home folders/files
    let home = lists::home::home_list(username);

    // Cache folders/files
    let cache = lists::cache::cache_list(username);

    // Config folders/files
    let config = lists::config::config_list(username);

    // local/share folders/files
    let local_share = lists::local_share::local_share_list(username);

    // If should_backup is true, perform backup before wiping
    if should_backup {
        runtime::backup::backup_kde_settings(
            home.clone(),
            cache.clone(),
            config.clone(),
            local_share.clone(),
            &username,
        );
    }

    // Wipe KDE settings
    runtime::wipe::wipe_kde_settings(home, cache, config, local_share, &username);
}
