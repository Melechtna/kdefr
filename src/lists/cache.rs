use crate::lists::regexing;
use std::path::PathBuf;

pub fn cache_list(username: &str) -> Vec<PathBuf> {
let paths = vec![
        //True to regex the string, false to not, avoid regexing where possible

        //Folders
        (format!("/home/{}/.cache/accountwizard", username,), false),
        (format!("/home/{}/.cache/discover", username,), false),
        (format!("/home/{}/.cache/drkonqi", username,), false),
        (format!("/home/{}/.cache/kamoso", username,), false),
        (format!("/home/{}/.cache/kcrash-metadata", username,), false),
        (format!("/home/{}/.cache/khelpcenter", username,), false),
        (format!("/home/{}/.cache/khistory", username,), false),
        (format!("/home/{}/.cache/kinfocenter", username,), false),
        (format!("/home/{}/.cache/kio_http", username,), false),
        (format!("/home/{}/.cache/kioexec", username,), false),
        (format!("/home/{}/.cache/krunner", username,), false),
        (format!("/home/{}/.cache/krunnerbookmarkrunnerfirefoxdbfile.sqlite", username,), false),
        (format!("/home/{}/.cache/kscreenlocker_greet", username,), false),
        (format!("/home/{}/.cache/ksmserver-logout-greeter", username,), false),
        (format!("/home/{}/.cache/ksplash", username,), false),
        (format!("/home/{}/.cache/kwin", username,), false),
        (format!("/home/{}/.cache/org.kde.unitconversion", username,), false),
        (format!("/home/{}/.cache/plasma-systemmonitor", username,), false),
        (format!("/home/{}/.cache/plasmashell", username,), false),
        (format!("/home/{}/.cache/polkit-kde-authentication-agent-1", username,), false),
        (format!("/home/{}/.cache/spectacle", username,), false),
        (format!("/home/{}/.cache/systemsettings", username,), false),
        (format!("/home/{}/.cache/xdg-desktop-portal-kde", username,), false),

        //Files
        (format!("/home/{}/.cache/audioCache.kcache", username,), false),
        (format!("/home/{}/.cache/icon-cache.kcache", username,), false),
        (format!("/home/{}/.cache/ksvg-elements", username,), false),

        //Files/Folders with unpredictable names (avoid using this if possible)
        (format!("/home/{}/.cache/ksycoca(.*)", username,), true),
        (format!("/home/{}/.cache/plasma_theme_(.*)", username,), true)

    ];

    // Pass the list of paths to the regexer and store the result in a variable
    let cache_paths = regexing::regexer(&paths);

    // Return the processed paths
    cache_paths
}