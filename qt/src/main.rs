/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2015  Panopticon authors
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

extern crate panopticon;
extern crate qmlrs;
extern crate libc;
extern crate graph_algos;
extern crate uuid;
extern crate rustc_serialize;
extern crate glpk_sys as glpk;
extern crate tempdir;
extern crate byteorder;

#[cfg(unix)]
extern crate xdg;

#[macro_use]
extern crate lazy_static;

mod controller;
mod project;
mod function;
mod sugiyama;

use std::env;
use std::fs::File;
use std::path::{PathBuf,Path};
use std::borrow::Cow;
use std::error::Error;

#[cfg(unix)]
use xdg::BaseDirectories;

use panopticon::result;
use panopticon::result::Result;

use controller::create_singleton;

#[cfg(unix)]
fn find_data_file(p: &Path) -> Result<Option<PathBuf>> {
    match BaseDirectories::with_prefix("panopticon") {
        Ok(dirs) => Ok(dirs.find_data_file(p).or(Some(Path::new(".").join(p)))),
        Err(e) => Err(result::Error(Cow::Owned(e.description().to_string()))),
    }
}

#[cfg(windows)]
fn find_data_file(p: &Path) -> Result<Option<PathBuf>> {
    match env::current_exe() {
        Ok(path) => Ok(Some(path.join("AppData/Local/Panopticon/Panopticon").join(p))),
        Err(e) => Err(result::Error(Cow::Owned(e.description().to_string()))),
    }
}

fn main() {
    // workaround bug #165
    if cfg!(unix) {
        env::set_var("UBUNTU_MENUPROXY","");
    }

    match find_data_file(Path::new("qml/Window.qml")) {
        Ok(Some(qml_main)) => {
            match File::open(&qml_main) {
                Ok(_) => {
                    qmlrs::register_singleton_type(&"Panopticon",1,0,&"Panopticon",create_singleton);

                    let mut engine = qmlrs::Engine::new();
                    engine.load_local_file(qml_main);
                    engine.exec();

                    return;
                },
                Err(e) => {
                    println!("Failed to open the QML files: {}",e);
                }
            }
        },
        Ok(None) => {
                    println!("Failed to open the QML files: Not Found");
        },
        Err(e) => {
            println!("Failed to find the QML files: {}",e);
        },
    }
}
