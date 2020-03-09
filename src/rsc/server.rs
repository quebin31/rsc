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

use anyhow::Error;
use clap::ArgMatches;

use super::Runnable;
use crate::config::{self, Config};

#[derive(Debug, Clone)]
pub struct RscServer {
    pub config: Config,
}

impl RscServer {
    pub fn from_matches(matches: &ArgMatches) -> Result<Self, Error> {
        // Get value of custom provided config, or get default config with
        // fallback enable.
        let config_path = matches
            .value_of("config")
            .map_or(config::default_config_path(true), |v| v.into());

        let config = Config::from_path(config_path)?;

        Ok(Self { config })
    }
}

impl Runnable for RscServer {
    fn run(&self) -> Result<(), Error> {
        println!("{:?}", self);

        Ok(())
    }
}
