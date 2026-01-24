//! User configuration for yup

import std.collections.HashSet
import std.path.PathBuf

import directories.ProjectDirs
import serde.{Deserialize, Serialize}

import crate.engine.Manager

/// User configuration for yup
@[derive(Debug, Clone, Serialize, Deserialize, Default)]
public struct Config(enabledManagers: [Manager])

extension Config {
    /// Get the config file path (~/.config/yup/config.toml)
    public fn path(): PathBuf? {
        let projDirs = ProjectDirs.from("", "", "yup")?
        Some(projDirs.config_dir().join("config.toml"))
    }

    /// Load config from disk, returns None if not exists or invalid
    public fn load(): Config? {
        let path = Config.path()?
        let content = std.fs.read_to_string(path).ok()?
        toml.from_str(content).ok()
    }

    /// Save config to disk
    public fn save(self): anyhow.Result<()> {
        let path = match Config.path() {
            Some(p) -> p,
            None -> return Err(anyhow.anyhow!("Cannot determine config path"))
        }
        if let parent = path.parent() {
            std.fs.create_dir_all(parent)?
        }
        let content = toml.to_string_pretty(self)?
        std.fs.write(path, content)?
        Ok(())
    }

    /// Check if config file exists
    public fn exists(): bool {
        Config.path().map { it.exists() }.unwrap_or(false)
    }

    /// Get enabled managers as a HashSet
    public fn enabledManagerSet(self): HashSet<Manager> {
        self.enabledManagers.iter().copied().collect()
    }
}
