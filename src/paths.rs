use std::path::{Path, PathBuf};

use anyhow::Result;
use xdg::BaseDirectories;

/// Provides file paths for application based on XDG standard
///
/// * `xdg_dirs`: underlying path constructor
struct PathProvider {
    xdg_dirs: BaseDirectories,
}

#[allow(dead_code)]
impl PathProvider {
    /// Attempts to construct `PathProvider` instance for provided `app_name`.
    ///
    /// * `app_name`: Name of app used as prefix in all XDG directories
    pub fn try_new<P: AsRef<Path>>(app_name: P) -> Result<Self> {
        let xdg_dirs = BaseDirectories::with_prefix(app_name)?;
        Ok(PathProvider { xdg_dirs })
    }

    /// Returns path for storing user's loc commands.
    /// Creates prefix directories if they don't exist.
    pub fn get_command_storage_path(&self) -> Result<PathBuf> {
        Ok(self.xdg_dirs.place_data_file("commands.json")?)
    }

    /// Returns path to file used for on-the-fly editing of existing commands.
    /// Creates prefix directories if they don't exist.
    pub fn get_command_edit_path(&self) -> Result<PathBuf> {
        Ok(self.xdg_dirs.place_state_file("command_edit")?)
    }

    /// Returns path to loc's configuration file.
    /// Creates prefix directories if they don't exist.
    pub fn get_loc_config_path(&self) -> Result<PathBuf> {
        Ok(self.xdg_dirs.place_config_file("config.toml")?)
    }
}
