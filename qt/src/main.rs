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

#[macro_use]
extern crate lazy_static;

mod controller;
mod project;
mod function;
mod sugiyama;

use std::env::home_dir;
use std::fs::File;
use std::path::Path;

use controller::create_singleton;

fn main() {
    let maybe_cur = Some(Path::new("qml").to_path_buf());
    let maybe_repo = Some(Path::new("qt/res").to_path_buf());
    let maybe_home = home_dir().map(|mut x| {
        x.push(".panopticon");
        x.push("qml");
        x
    });
    let maybe_usr = Some(Path::new("/usr/share/panopticon/qml").to_path_buf());
    let maybe_local = Some(Path::new("/usr/local/share/panopticon/qml").to_path_buf());
    let search_path = [maybe_repo,maybe_home,maybe_usr,maybe_local,maybe_cur];

    for p in search_path.into_iter().filter_map(|x| x.clone()) {
        let mut file = p.clone();

        file.push("Window.qml");

        match File::open(file.clone()) {
            Ok(_) => {
                qmlrs::register_singleton_type(&"Panopticon",1,0,&"Panopticon",create_singleton);

                let mut engine = qmlrs::Engine::new();
                engine.load_local_file(file);
                engine.exec();

                return;
            },
            Err(_) => {},
        }
    }

    println!("Failed to find the QML files. Looked in");
    for s in search_path.iter() {
        match s {
            &Some(ref p) => println!("\t{}",p.to_str().unwrap_or("(encoding error)")),
            &None => {}
        }
    }
}
