// Copyright (C) 2020 Kevin Dc
//
// This file is part of rsc.
//
// rsc is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rsc is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with rsc.  If not, see <http://www.gnu.org/licenses/>.

// Get default config depending on what user is running the program. If
// fallback is set to true, a normal user will pick the system-wide
// config file if its config doesn't exists.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use cfg_if::cfg_if;
use dirs;
use failure::Error;
use log::info;
use serde::Deserialize;

use crate::error::RscError;
use crate::util;

pub fn default_config_path(fallback: bool) -> PathBuf {
    if util::running_as_root() {
        PathBuf::from("/etc/rsc/config.toml")
    } else {
        let user_config = dirs::config_dir().unwrap().join("rsc/config.toml");
        if !user_config.exists() && fallback {
            PathBuf::from("/etc/rsc/config.toml")
        } else {
            user_config
        }
    }
}

pub fn generate_config_file() -> Result<(), Error> {
    let shared_config = Path::new("/usr/share/rsc/");

    // Get default config, but don't fallback to system-wide config
    // if current user isn't root.
    let config_path = default_config_path(false);

    info!("Generating config: \"{}\"", config_path.to_string_lossy());
    util::maybe_create_dir(config_path.parent().unwrap())?;

    if !shared_config.exists() {
        cfg_if! {
            if #[cfg(feature = "standalone")] {
                let mut config_file = File::create(config_path)?;
                write!(config_file, "{}", include_str!("../share/config.toml"))?;
            } else {
                return Err(RscError::ResourceNotFound {
                    resource: shared_config.to_string_lossy().to_string()
                }.into());
            }
        }
    } else {
        fs::copy(shared_config, config_path)?;
    }

    info!("Successfully generated config file");
    Ok(())
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    address: Option<String>,
    port: Option<u32>,
}

impl Config {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, Error> {
        let mut path = path.as_ref().to_owned();

        if !path.exists() {
            if path == default_config_path(true) {
                generate_config_file()?;
                path = default_config_path(false);
            } else {
                return Err(RscError::InvalidConfig {
                    path: path.to_string_lossy().to_string(),
                    reason: "not found".into(),
                }
                .into());
            }
        }

        let contents = fs::read_to_string(&path)?;
        toml::from_str(&contents).map_err(|e| {
            RscError::InvalidConfig {
                path: path.to_string_lossy().to_string(),
                reason: e.to_string(),
            }
            .into()
        })
    }
}
