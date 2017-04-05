// Panopticon - A libre disassembler
// Copyright (C) 2015  Panopticon authors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate panopticon;
extern crate qmlrs;
extern crate libc;
extern crate graph_algos;
extern crate uuid;
extern crate rustc_serialize;
extern crate cassowary;
extern crate tempdir;
extern crate byteorder;
extern crate chrono;
extern crate chrono_humanize;
extern crate goblin;
extern crate clap;

#[cfg(unix)]
extern crate xdg;

#[macro_use]
extern crate lazy_static;

mod controller;
mod project;
mod function;
mod sugiyama;
mod paths;

use clap::{App, Arg};

use controller::{create_singleton, Controller};

use paths::find_data_file;

use std::path::{Path, PathBuf};

fn main() {
    use std::path::Path;
    use std::env;

    env_logger::init().unwrap();

    if cfg!(unix) {
        // workaround bug #165
        env::set_var("UBUNTU_MENUPROXY", "");

        // workaround bug #183
        env::set_var("QT_QPA_PLATFORMTHEME", "");
    }

    let title_screen = find_data_file(&Path::new("qml").join("Title.qml"));
    let main_window = find_data_file(&Path::new("qml").join("Window.qml"));

    let matches = App::new("Panopticon")
        .about("A libre cross-platform disassembler.")
        .arg(Arg::with_name("INPUT")
            .help("File to disassemble")
            .validator(exists_path_val)
            .index(1))
        .get_matches();

    let (start_with_file, input_file_path) = match matches.value_of("INPUT") {
        Some(v) => (true, v),
        None => (false, ""),
    };

    match (title_screen, main_window, start_with_file) {
        (_, Ok(Some(window)), true) => {
            qmlrs::register_singleton_type(&"Panopticon", 1, 0, &"Panopticon", create_singleton);

            let fileformat = match function::file_details_of_path(PathBuf::from(&input_file_path)) {
                Ok(details) => {
                    match details.format().clone() {
                        Some(format) => format,
                        None => {
                            let filestate = details.state();
                            println!("no format (file state: {})", filestate.to_string());
                            return;
                        }
                    }
                }
                Err(e) => {
                    println!("invalid format: {}", e);
                    return;
                }
            };

            let request = format!("{{\"kind\": \"{}\", \"path\": \"{}\"}}",
                                  fileformat.to_string(),
                                  input_file_path);
            Controller::set_request(&request);
            let mut engine = qmlrs::Engine::new("Panopticon");
            engine.load_local_file(&format!("{}", window.display()));
            engine.exec();
        }
        (Ok(Some(title)), Ok(Some(window)), false) => {
            qmlrs::register_singleton_type(&"Panopticon", 1, 0, &"Panopticon", create_singleton);

            {
                let mut engine = qmlrs::Engine::new("Panopticon");
                engine.load_local_file(&format!("{}", title.display()));
                engine.exec();
            }

            if Controller::request().ok().unwrap_or(None).is_some() {
                let mut engine = qmlrs::Engine::new("Panopticon");
                engine.load_local_file(&format!("{}", window.display()));
                engine.exec();
            }
        }
        _ => println!("Failed to open the QML files"),
    }
}

fn exists_path_val(filepath: String) -> Result<(), String> {
    match Path::new(&filepath).is_file() {
        true => Ok(()),
        false => Err(format!("'{}': no such file", filepath)),
    }
}
