/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2015,2017  Panopticon authors
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

#![recursion_limit = "1024"]

extern crate env_logger;
extern crate panopticon_core;
extern crate panopticon_glue;
extern crate panopticon_analysis;
extern crate panopticon_abstract_interp;
extern crate panopticon_data_flow;
extern crate panopticon_graph_algos;
extern crate panopticon_amd64;
extern crate panopticon_avr;
extern crate libc;
extern crate uuid;
extern crate cassowary;
extern crate tempdir;
extern crate chrono;
extern crate chrono_humanize;
extern crate clap;
extern crate futures;
extern crate futures_cpupool;
extern crate parking_lot;
extern crate multimap;

#[cfg(unix)]
extern crate xdg;

#[macro_use]
extern crate log;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;

mod sugiyama;
mod singleton;
mod control_flow_layout;
mod paths;
mod action;
mod qt;
mod errors {
    error_chain! {
        links {
            GlueError(::panopticon_glue::Error, ::panopticon_glue::ErrorKind);
        }

        foreign_links {
            Panopticon(::panopticon_core::Error);
            Time(::std::time::SystemTimeError);
            Io(::std::io::Error);
            NulError(::std::ffi::NulError);
            UuidParseError(::uuid::ParseError);
        }
    }
}
use clap::{App, Arg};
use errors::*;
use qt::Qt;

use std::path::{Path, PathBuf};
use std::result;

fn main() {
    use std::env;
    use paths::find_data_file;
    use panopticon_glue::Glue;

    env_logger::init().unwrap();

    if cfg!(unix) {
        // workaround bug #165
        env::set_var("UBUNTU_MENUPROXY", "");

        // workaround bug #183
        env::set_var("QT_QPA_PLATFORMTHEME", "");

        // needed for UI tests
        env::set_var("QT_LINUX_ACCESSIBILITY_ALWAYS_ON", "1");
    }


    let matches = App::new("Panopticon")
        .about("A libre cross-platform disassembler.")
        .arg(
            Arg::with_name("INPUT")
                .help("File to disassemble")
                .validator(exists_path_val)
                .index(1)
        )
        .get_matches();
    let main_window = find_data_file(&Path::new("qml"));

    match main_window {
        Ok(Some(ref path)) => {
            let recent_sessions = match read_recent_sessions() {
                Ok(s) => s,
                Err(s) => {
                    error!("Failed to read recent sessions: {}", s);
                    vec![]
                }
            };
            match Qt::exec(
                path,
                matches.value_of("INPUT").map(str::to_string),
                recent_sessions,
            ) {
                Ok(()) => {}
                Err(s) => error!("{}", s),
            }
        }
        Ok(None) => {
            error!("QML files not found :(");
        }
        Err(s) => {
            error!("{}", s);
        }
    }
}

fn exists_path_val(filepath: String) -> result::Result<(), String> {
    match Path::new(&filepath).is_file() {
        true => Ok(()),
        false => Err(format!("'{}': no such file", filepath)),
    }
}

fn read_recent_sessions() -> Result<Vec<(String, String, PathBuf, u32)>> {
    use std::fs;
    use std::time;
    use panopticon_core::Project;

    let path = paths::session_directory()?;
    let mut ret = vec![];

    if let Ok(dir) = fs::read_dir(path) {
        for ent in dir.filter_map(|x| x.ok()) {
            if let Ok(ref project) = Project::open(&ent.path()) {
                if let Ok(ref md) = ent.metadata() {
                    let mtime = md.modified()?
                        .duration_since(time::UNIX_EPOCH)?
                        .as_secs() as u32;
                    let fname = ent.path();
                    ret.push(
                        (project.name.clone(), "".to_string(), fname, mtime),
                    );
                }
            }
        }
    }
    Ok(ret)
}
