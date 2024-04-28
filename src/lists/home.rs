use crate::lists::regexing;
use std::path::PathBuf;

pub fn home_list(username: &str) -> Vec<PathBuf> {
    let paths = vec![
        //True to regex the string, false to not, avoid regexing where possible

        //Folders
        (format!("/home/{}/.kde", username,), false),

        //Files
        (format!("/home/{}/.gtkrc", username,), false),
        (format!("/home/{}/.gtkrc-2.0", username,), false),
        (format!("/home/{}/.gtkrc-2.0-kde4", username,), false)
    ];

    // Pass the list of paths to the regexer and store the result in a variable
    let home_paths = regexing::regexer(&paths);

    // Return the processed paths
    home_paths
}