use crate::{es_plugin_dir, InstalledPlugin};
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

/// Attempts to read plug-ins from the default directory
pub(crate) fn scan_plugins() -> Result<Vec<InstalledPlugin>> {
    debug!("Scanning for installed plug-ins");
    let plugin_dir = es_plugin_dir().ok_or_else(|| anyhow!("Failed to get ES Plug-In dir"))?;
    if plugin_dir.exists() {
        let plugins: Vec<InstalledPlugin> = plugin_dir
            .read_dir()?
            .filter_map(|res| match res {
                Ok(entry) => {
                    debug!("Considering {}", entry.path().to_string_lossy());
                    if entry.file_type().ok()?.is_file() {
                        debug!("Is a file, skipping");
                        None
                    } else {
                        let version = match read_version(&entry.path()) {
                            Ok(v) => {
                                debug!("Found version {}", v);
                                v
                            }
                            Err(e) => {
                                warn!(
                                    "Failed to read version file for plug-in {}: {}",
                                    entry.file_name().to_string_lossy(),
                                    e
                                );
                                String::from("unknown")
                            }
                        };
                        Some(InstalledPlugin {
                            name: String::from(entry.file_name().to_str()?),
                            version,
                        })
                    }
                }
                Err(e) => {
                    error!("Failed to read an entry from the plug-in directory: {}", e);
                    None
                }
            })
            .collect();
        debug!("Found {} plug-ins", plugins.len());
        Ok(plugins)
    } else {
        debug!("Plug-in directory doesn't exist");
        Ok(vec![])
    }
}

fn read_version(plugin_dir: &PathBuf) -> std::io::Result<String> {
    let mut path = plugin_dir.clone();
    path.push(".version");
    fs::read_to_string(path)
}
