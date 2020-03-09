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

#[cfg(not(unix))]
compile_error!("Rsc doesn't support Windows yet (maybe never)");

mod config;
mod error;
mod logger;
mod rsc;
mod util;

use std::process;

use anyhow::Error;
use clap::clap_app;
use human_panic::setup_panic;
use log::{error, info};

use crate::logger::setup_logger;
use crate::rsc::Runnable;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");

fn run() -> Result<(), Error> {
    let app = clap_app! { rsc =>
        (version: VERSION)
        (author: AUTHORS)
        (about: ABOUT)
        (@subcommand server =>
            (@arg init: -i --init "Initialize server")
            (@arg config: -c --config +takes_value "Custom config file"))
    };

    rsc::from_app(app)?.run()
}

fn main() {
    setup_panic!();
    setup_logger();

    process::exit(match run() {
        Ok(_) => {
            info!("Good bye!");
            0
        }
        Err(e) => {
            error!("{}", e);
            1
        }
    })
}
