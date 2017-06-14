/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2016  Panopticon authors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use panopticon_core::result;
use panopticon_core::result::Result;
use std::borrow::Cow;

use std::env;
use std::error::Error;
use std::fs::DirBuilder;
use std::path::{Path, PathBuf};

#[cfg(all(unix,not(target_os = "macos")))]
pub fn session_directory() -> Result<PathBuf> {
    use xdg::BaseDirectories;
    match BaseDirectories::with_prefix("panopticon") {
        Ok(dirs) => {
            let ret = dirs.get_data_home().join("sessions");
            DirBuilder::new().recursive(true).create(ret.clone())?;
            Ok(ret)
        }
        Err(e) => Err(result::Error(Cow::Owned(e.description().to_string()))),
    }
}

#[cfg(all(unix,target_os = "macos"))]
pub fn session_directory() -> Result<PathBuf> {
    match env::var("HOME") {
        Ok(home) => {
            let ret = Path::new(&home).join("Library").join("Application Support").join("Panopticon").join("sessions");
            DirBuilder::new().recursive(true).create(ret.clone())?;
            Ok(ret)
        }
        Err(e) => Err(result::Error(Cow::Owned(e.description().to_string()))),
    }
}

#[cfg(windows)]
pub fn session_directory() -> Result<PathBuf> {
    match env::var("APPDATA") {
        Ok(appdata) => {
            let ret = Path::new(&appdata).join("Panopticon").join("sessions");
            DirBuilder::new().recursive(true).create(ret.clone())?;
            Ok(ret)
        }
        Err(e) => Err(result::Error(Cow::Owned(e.description().to_string()))),
    }
}

pub fn find_data_file(p: &Path) -> Result<Option<PathBuf>> {
    match find_data_file_impl(p) {
        r @ Ok(Some(_)) => r,
        Ok(None) => {
            let q = env::current_exe()?.parent().unwrap().parent().unwrap().parent().unwrap().join(p);
            if q.exists() { Ok(Some(q)) } else { Ok(None) }
        }
        e @ Err(_) => e,
    }
}

#[cfg(all(unix,not(target_os = "macos")))]
fn find_data_file_impl(p: &Path) -> Result<Option<PathBuf>> {
    use xdg::BaseDirectories;
    match BaseDirectories::with_prefix("panopticon") {
        Ok(dirs) => Ok(dirs.find_data_file(p)),
        Err(e) => Err(result::Error(Cow::Owned(e.description().to_string()))),
    }
}

#[cfg(all(unix,target_os = "macos"))]
fn find_data_file_impl(p: &Path) -> Result<Option<PathBuf>> {
    match env::current_exe() {
        Ok(path) => Ok(path.parent().and_then(|x| x.parent()).map(|x| x.join("Resources").join(p)).and_then(|x| if x.exists() { Some(x) } else { None })),
        Err(e) => Err(result::Error(Cow::Owned(e.description().to_string()))),
    }
}

#[cfg(windows)]
fn find_data_file_impl(p: &Path) -> Result<Option<PathBuf>> {
    match env::current_exe() {
        Ok(path) => Ok(path.parent().map(|x| x.join(p)).and_then(|x| if x.exists() { Some(x) } else { None })),
        Err(e) => Err(result::Error(Cow::Owned(e.description().to_string()))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn paths() {
        assert!(find_data_file(Path::new("test")).is_ok());
        assert!(session_directory().is_ok());
    }
}
