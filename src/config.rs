use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::io::{Read, Write};

pub const CONFIG_FILE: &str = "devshell.toml";

/// @brief Represents the structure of the devshell.toml file.
///
/// Maintains a set of packages, an optional shell hook, and an optional purity flag.
/// BTreeSet is used to keep packages sorted and unique.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct DevShellConfig {
    /// @brief Set of package names to be included in the development shell.
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub packages: BTreeSet<String>,

    /// @brief Optional shell hook command to be executed in the shell.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shell-hook")]
    pub shell_hook: Option<String>,

    /// @brief Optional flag to indicate if the shell should be pure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pure: Option<bool>,
}

impl DevShellConfig {
    /// @brief Loads the configuration from CONFIG_FILE or returns a default config if not found.
    /// @return Result containing the loaded DevShellConfig or an I/O error.
    pub fn load() -> Result<Self, std::io::Error> {
        match fs::File::open(CONFIG_FILE) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                toml::from_str(&contents)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(DevShellConfig::default()),
            Err(e) => Err(e),
        }
    }

    /// @brief Saves the current configuration to CONFIG_FILE.
    /// @return Result indicating success or an I/O error.
    pub fn save(&self) -> Result<(), std::io::Error> {
        let contents = toml::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let mut file = fs::File::create(CONFIG_FILE)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }

    /// @brief Appends a new shell hook command to the existing shell hook.
    ///
    /// If a shell hook already exists, the new hook is appended with a separator.
    /// If not, the new hook is set as the shell hook.
    /// @param new_hook The shell hook command to append.
    pub fn append_hook(&mut self, new_hook: &str) {
        let new_hook = new_hook.trim().trim_end_matches(';');
        if new_hook.is_empty() {
            return;
        }

        if let Some(existing_hook) = self.shell_hook.as_mut() {
            existing_hook.push_str(";\n");
            existing_hook.push_str(new_hook);
        } else {
            self.shell_hook = Some(new_hook.to_string());
        }
    }
}
