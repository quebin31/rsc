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

use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum RscError {
    #[error("Invalid config file: \"{path}\" (reason: {reason})")]
    InvalidConfig { path: String, reason: String },

    #[error("Resource \"{resource}\" wasn't found!")]
    ResourceNotFound { resource: String },
}
