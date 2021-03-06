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

use ansi_term::Color::{Cyan, Red, Yellow};
use log::{set_logger, set_max_level, Level, LevelFilter, Log, Metadata, Record};

pub struct RscLogger;

static RSC_LOGGER: RscLogger = RscLogger;

pub fn setup_logger() {
    set_logger(&RSC_LOGGER)
        .map(|_| set_max_level(LevelFilter::Info))
        .unwrap();
}

impl Log for RscLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        match record.level() {
            Level::Info => println!("{} {}", Cyan.bold().paint("[i]"), record.args()),
            Level::Warn => println!("{} {}", Yellow.bold().paint("[w]"), record.args()),
            Level::Error => println!("{} {}", Red.bold().paint("[e]"), record.args()),
            _ => {}
        }
    }

    fn flush(&self) {}
}
