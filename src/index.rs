use crate::{es_plugin_dir, util, AvailablePlugin, InstalledPlugin};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use ureq;

#[derive(Serialize, Deserialize)]
struct PluginIndex(Vec<AvailablePlugin>);

impl AvailablePlugin {
    pub fn download(&self) -> Result<InstalledPlugin> {
        let mut destination =
            es_plugin_dir().ok_or_else(|| anyhow!("Failed to get ES Plug-In Dir"))?;
        destination.push(&self.name);

        info!(
            "Downloading {} to {}",
            self.name,
            destination.to_string_lossy()
        );

        if destination.exists() {
            fs::remove_dir_all(&destination)?;
        }
        fs::create_dir_all(&destination)?;

        let bytes = util::download(&self.url)?;
        util::unzip(&destination, bytes)?;

        let mut version_file_path = destination;
        version_file_path.push(".version");
        let mut version_file = fs::File::create(version_file_path)?;
        version_file.write_all(self.version.as_bytes())?;

        info!("Done!");
        Ok(InstalledPlugin {
            name: self.name.clone(),
            version: self.version.clone(),
        })
    }
}

pub(crate) fn get_available_plugins() -> Result<Vec<AvailablePlugin>> {
    debug!("Fetching available plug-ins");
    let resp = ureq::get(
        "https://github.com/EndlessSkyCommunity/endless-sky-plugins/raw/master/generated/plugins.json",
    )
    .call()?;
    let index: PluginIndex = resp.into_body().read_json()?;
    debug!("Got {} available plug-ins", index.0.len());
    Ok(index.0)
}
