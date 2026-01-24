//! User configuration for yup

import std.collections.HashSet
import std.path.PathBuf
import directories.ProjectDirs
import serde.{ Deserialize, Serialize }
import crate.engine.Manager

/// User configuration for yup
@[derive(Debug , Clone , Serialize , Deserialize , Default)]
public struct Config(enabledManagers: [Manager])

extension Config {
  /// Get the config file path (~/.config/yup/config.toml)
  public fn path(): PathBuf? {
    let projDirs = ProjectDirs.from("", "", "yup")?
    projDirs.configDir().join("config.toml")
  }
  /// Load config from disk, returns null if not exists or invalid
  public fn load(): Config? {
    let path = Config.path()?
    let content = std.fs.readToString(path).ok()?
    toml.fromStr(content).ok()
  }
  /// Save config to disk
  public fn save(consuming self: Self): anyhow.Result<()> {
    let path = match Config.path() {
      Some(p) -> p
      null -> return Err(anyhow.anyhow!("Cannot determine config path"))
    }
    if let parent = path.parent() {
      std.fs.createDirAll(parent)?
    }
    let content = toml.toStringPretty(self)?
    std.fs.write(path, content)?
    Ok(())
  }
  /// Check if config file exists
  public fn exists(): bool { Config.path().map {
      it.exists()
    }.unwrapOr(false) }
  /// Get enabled managers as a HashSet
  public fn enabledManagerSet(consuming self: Self): HashSet<Manager> {
    self.enabledManagers.iter().copied().collect()
  }
}
