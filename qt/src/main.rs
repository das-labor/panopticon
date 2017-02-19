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

#[cfg(unix)]
extern crate xdg;

#[macro_use]
extern crate lazy_static;

mod controller;
mod project;
mod function;
mod sugiyama;
mod paths;

use qmlrs::{Variant};

use panopticon::result;
use panopticon::result::Result;

use controller::{
    create_singleton,
    Controller,
};

use paths::find_data_file;

use std::path::{
    Path,
    PathBuf
};

const USAGE: &'static str = "USAGE:\npanopticon [file]";

fn main() {
    use std::path::Path;
    use std::env;

    env_logger::init().unwrap();

    if cfg!(unix) {
        // workaround bug #165
        env::set_var("UBUNTU_MENUPROXY","");

        // workaround bug #183
        env::set_var("QT_QPA_PLATFORMTHEME","");
    }

    let title_screen = find_data_file(&Path::new("qml").join("Title.qml"));
    let main_window = find_data_file(&Path::new("qml").join("Window.qml"));

    let args = env::args().skip(1).collect::<Vec<String>>();
    let start_with_file = args.len() > 0;

    match (title_screen,main_window,start_with_file) {
        (_,Ok(Some(window)),true) => {
            qmlrs::register_singleton_type(&"Panopticon",1,0,&"Panopticon",create_singleton);

            let input_file_path = match filepath_from_args(args) {
                Some(v) => v,
                None => return
            };
            let filetype = match function::file_details_of_path(PathBuf::from(&input_file_path)) {
                Ok(details) => {
                    match details.into_format() {
                        Some(format) => format,
                        None => {
                            println!("no format");
                            return;
                        }
                    }
                },
                Err(e) => {
                    println!("invalid format: {}", e);
                    return;
                }
            };

            let request = format!("{{\"kind\": \"{}\", \"path\": \"{}\"}}", filetype, input_file_path);
            Controller::set_request(&request);
            let mut engine = qmlrs::Engine::new("Panopticon");
            engine.load_local_file(&format!("{}",window.display()));
            engine.exec();
        }
        (Ok(Some(title)),Ok(Some(window)),false) => {
            qmlrs::register_singleton_type(&"Panopticon",1,0,&"Panopticon",create_singleton);

            {
                let mut engine = qmlrs::Engine::new("Panopticon");
                engine.load_local_file(&format!("{}",title.display()));
                engine.exec();
            }

            if Controller::request().ok().unwrap_or(None).is_some() {
                let mut engine = qmlrs::Engine::new("Panopticon");
                engine.load_local_file(&format!("{}",window.display()));
                engine.exec();
            }
        },
        _ => {
            println!("Failed to open the QML files")
        },
    }
}

fn filepath_from_args(args_vec: Vec<String>) -> Option<String> {
    match args_vec.into_iter().next(){
        Some(filepath) => {
            if filepath == "--help" || filepath == "-h"{
                println!("{}", USAGE);
                return None;
            }
            if Path::new(&filepath).is_file() {
                return Some(filepath);
            }
            else {
                println!("'{}': no such file", filepath);
                return None;
            }
        },
        None => {
            println!("{}", USAGE);
            return None;
        }
    }
}
