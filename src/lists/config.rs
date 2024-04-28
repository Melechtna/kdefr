use std::path::PathBuf;
use crate::lists::regexing;

pub fn config_list(username: &str) -> Vec<PathBuf> {
    let paths = vec![
        //True to regex the string, false to not, avoid regexing where possible

        //Folders
        (format!("/home/{}/.config/autostart", username,), false),
        (format!("/home/{}/.config/dconf", username,), false),
        (format!("/home/{}/.config/dconf", username,), false),
        (format!("/home/{}/.config/glib", username,), false),
        (format!("/home/{}/.config/glib-2.0", username,), false),
        (format!("/home/{}/.config/gtk", username,), false),
        (format!("/home/{}/.config/gtk-2.0", username,), false),
        (format!("/home/{}/.config/gtk-3.0", username,), false),
        (format!("/home/{}/.config/gtk-4.0", username,), false),
        (format!("/home/{}/.config/KDE", username,), false),
        (format!("/home/{}/.config/kde.org", username,), false),
        (format!("/home/{}/.config/kdedefaults", username,), false),
        (format!("/home/{}/.config/libaccounts-glib", username,), false),
        (format!("/home/{}/.config/session", username,), false),
        (format!("/home/{}/.config/xsettingsd", username,), false),

        //Files
        (format!("/home/{}/.config/arkrc", username,), false),
        (format!("/home/{}/.config/baloofileinformationrc", username,), false),
        (format!("/home/{}/.config/baloofilerc", username,), false),
        (format!("/home/{}/.config/bluedevilglobalrc", username,), false),
        (format!("/home/{}/.config/arkrc", username,), false),
        (format!("/home/{}/.config/discoverrc", username,), false),
        (format!("/home/{}/.config/dolphinrc", username,), false),
        (format!("/home/{}/.config/drkonqirc", username,), false),
        (format!("/home/{}/.config/eventviewsrc", username,), false),
        (format!("/home/{}/.config/fedora-plasma-cacherc", username,), false),
        (format!("/home/{}/.config/filetypesrc", username,), false),
        (format!("/home/{}/.config/gtkrc", username,), false),
        (format!("/home/{}/.config/gtkrc-2.0", username,), false),
        (format!("/home/{}/.config/gwenviewrc", username,), false),
        (format!("/home/{}/.config/kactivitymanagerd-statsrc", username,), false),
        (format!("/home/{}/.config/kactivitymanagerdrc", username,), false),
        (format!("/home/{}/.config/kamosorc", username,), false),
        (format!("/home/{}/.config/katemetainfos", username,), false),
        (format!("/home/{}/.config/katerc", username,), false),
        (format!("/home/{}/.config/katevirc", username,), false),
        (format!("/home/{}/.config/kcmfonts", username,), false),
        (format!("/home/{}/.config/kcminputrc", username,), false),
        (format!("/home/{}/.config/kconf_updaterc", username,), false),
        (format!("/home/{}/.config/kdedrc", username,), false),
        (format!("/home/{}/.config/kded2rc", username,), false),
        (format!("/home/{}/.config/kded3rc", username,), false),
        (format!("/home/{}/.config/kded4rc", username,), false),
        (format!("/home/{}/.config/kded5rc", username,), false),
        (format!("/home/{}/.config/kded6rc", username,), false),
        (format!("/home/{}/.config/kdeglobals", username,), false),
        (format!("/home/{}/.config/kdialogrc", username,), false),
        (format!("/home/{}/.config/kfindrc", username,), false),
        (format!("/home/{}/.config/kglobalshortcutsrc", username,), false),
        (format!("/home/{}/.config/khelpcenterrc", username,), false),
        (format!("/home/{}/.config/kinfocenterrc", username,), false),
        (format!("/home/{}/.config/kiorc", username,), false),
        (format!("/home/{}/.config/knfsshare", username,), false),
        (format!("/home/{}/.config/kolourpaintrc", username,), false),
        (format!("/home/{}/.config/komparerc", username,), false),
        (format!("/home/{}/.config/konsolerc", username,), false),
        (format!("/home/{}/.config/konsolesshconfig", username,), false),
        (format!("/home/{}/.config/kservicemenurc", username,), false),
        (format!("/home/{}/.config/ksplashrc", username,), false),
        (format!("/home/{}/.config/ktrashrc", username,), false),
        (format!("/home/{}/.config/kwalletrc", username,), false),
        (format!("/home/{}/.config/kwinrc", username,), false),
        (format!("/home/{}/.config/kwriterc", username,), false),
        (format!("/home/{}/.config/mimeapps.list", username,), false),
        (format!("/home/{}/.config/okularpartrc", username,), false),
        (format!("/home/{}/.config/okularrc", username,), false),
        (format!("/home/{}/.config/partitionmanagerrc", username,), false),
        (format!("/home/{}/.config/plasma-localerc", username,), false),
        (format!("/home/{}/.config/plasma-org.kde.plasma.desktop-appletsrc", username,), false),
        (format!("/home/{}/.config/plasma-welcomerc", username,), false),
        (format!("/home/{}/.config/PlasmaDiscoverUpdates", username,), false),
        (format!("/home/{}/.config/plasmanotifyrc", username,), false),
        (format!("/home/{}/.config/plasmaparrc", username,), false),
        (format!("/home/{}/.config/plasmarc", username,), false),
        (format!("/home/{}/.config/plasmashellrc", username,), false),
        (format!("/home/{}/.config/powermanagementprofilesrc", username,), false),
        (format!("/home/{}/.config/spectaclerc", username,), false),
        (format!("/home/{}/.config/systemmonitorrc", username,), false),
        (format!("/home/{}/.config/systemsettingsrc", username,), false),
        (format!("/home/{}/.config/trashrc", username,), false),
        (format!("/home/{}/.config/xdg-desktop-portal-kderc", username,), false)

    ];

    // Pass the list of paths to the regexer and store the result in a variable
    let config_paths = regexing::regexer(&paths);

    // Return the processed paths
    config_paths
}