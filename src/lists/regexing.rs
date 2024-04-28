use regex::Regex;
use std::path::{Path, PathBuf};

pub fn regexer(paths: &[(String, bool)]) -> Vec<PathBuf> {
    let mut matching_paths = Vec::new();

    for (path_str, is_regex) in paths {
        let path = Path::new(path_str);

        if !is_regex {
            // If it's not a regex pattern, just add the path directly
            matching_paths.push(path.to_path_buf());
        } else {
            // Extract the regex pattern from the path string
            let regex_pattern = path.file_name().unwrap().to_str().unwrap();

            // Construct the regex
            let regex = Regex::new(regex_pattern).unwrap();

            // Search for matches within the same directory
            if let Some(parent_dir) = path.parent() {
                if let Ok(entries) = parent_dir.read_dir() {
                    for entry in entries.flatten() {
                        if let Some(file_name) = entry.file_name().to_str() {
                            if regex.is_match(file_name) {
                                matching_paths.push(entry.path());
                            }
                        }
                    }
                }
            }
        }
    }

    matching_paths
}
