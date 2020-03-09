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

pub mod client;
pub mod server;

use anyhow::Error;
use clap::App as ClapApp;

use self::server::RscServer;

pub trait Runnable {
    fn run(&self) -> Result<(), Error>;
}

pub fn from_app(app: ClapApp) -> Result<impl Runnable, Error> {
    let matches = app.get_matches();

    match matches.subcommand() {
        ("server", Some(matches)) => RscServer::from_matches(&matches),
        ("client", Some(_matches)) => todo!(),
        (subcmd, _) => panic!("Missing impl for {} subcommand.", subcmd),
    }
}
