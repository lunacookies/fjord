use is_executable::IsExecutable;
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::{fs, io};

#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) struct Commands {
    commands: HashMap<OsString, PathBuf>,
    search_path: Vec<PathBuf>,
}

impl Commands {
    pub(super) fn new(search_path: Vec<PathBuf>) -> io::Result<Self> {
        let mut commands = HashMap::new();

        let process_possible_command = |path: &Path| {
            if !path.is_executable() {
                return None;
            }

            // If we’re on Windows the user should not have to type out ‘.exe’ after the name of
            // every command they want to run, so we return the ‘file stem’ (file name without
            // extension).
            #[cfg(windows)]
            {
                path.file_stem().to_os_string()
            }

            // If we’re not on Windows, then we don’t have to worry about the file extenion, and the
            // user can just type out the full file name.
            #[cfg(not(windows))]
            {
                path.file_name().map(|file_name| file_name.to_os_string())
            }
        };

        for path in &search_path {
            for dir_entry in fs::read_dir(path)? {
                let path = dir_entry?.path();

                if let Some(name) = process_possible_command(&path) {
                    commands.insert(name, path);
                }
            }
        }

        Ok(Self {
            commands,
            search_path,
        })
    }

    pub(crate) fn get(&self, name: impl AsRef<OsStr>) -> Option<&Path> {
        self.commands.get(name.as_ref()).map(PathBuf::as_path)
    }
}
