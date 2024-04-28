use std::path::PathBuf;
use crate::lists::regexing;

pub fn local_share_list(username: &str) -> Vec<PathBuf> {
    let paths = vec![
        //True to regex the string, false to not, avoid regexing where possible

        //Folders
        (format!("/home/{}/.local/share/ark", username,), false),
        (format!("/home/{}/.local/share/baloo", username,), false),
        (format!("/home/{}/.local/share/klipper", username,), false),
        (format!("/home/{}/.local/share/knewstuff", username,), false),
        (format!("/home/{}/.local/share/knewstuff2", username,), false),
        (format!("/home/{}/.local/share/knewstuff3", username,), false),
        (format!("/home/{}/.local/share/konsole", username,), false),
        (format!("/home/{}/.local/share/kscreen", username,), false),
        (format!("/home/{}/.local/share/kwalletd", username,), false),
        (format!("/home/{}/.local/share/kwin", username,), false),
        (format!("/home/{}/.local/share/okular", username,), false),
        (format!("/home/{}/.local/share/plasma", username,), false),
        (format!("/home/{}/.local/share/plasma-systemmonitor", username,), false),
        (format!("/home/{}/.local/share/plasmasessionrestore", username,), false),
        (format!("/home/{}/.local/share/plasmashell", username,), false),
        (format!("/home/{}/.local/share/systemsettings", username,), false),
        (format!("/home/{}/.local/share/Trash", username,), false),
        (format!("/home/{}/.local/share/xdg-desktop-portal-kde", username,), false),

        //Files


    ];

    // Pass the list of paths to the regexer and store the result in a variable
    let local_share_paths = regexing::regexer(&paths);

    // Return the processed paths
    local_share_paths
}