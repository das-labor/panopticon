#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate panopticon_core;
extern crate panopticon_amd64;
extern crate panopticon_avr;
extern crate panopticon_analysis;
extern crate panopticon_graph_algos;
extern crate futures;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::result;
use std::path::Path;
use panopticon_core::{Machine, loader};
use panopticon_amd64 as amd64;
use panopticon_avr as avr;
use panopticon_analysis::pipeline;
use panopticon_graph_algos::{GraphTrait};
use futures::Stream;

mod errors {
    error_chain! {
        foreign_links {
            Panopticon(::panopticon_core::Error);
        }
    }
}
use errors::*;

fn main() {
    env_logger::init().unwrap();

    let matches = clap_app!(Panopticon =>
        (version: "0.1")
        (about: "A libre cross-platform disassembler.")
        (@arg INPUT: +required ... {exists_path_val} "Files to disassemble")
    ).get_matches();

    if let Some(vals) = matches.values_of("INPUT") {
        for p in vals.map(str::to_string) {
            info!("{}",p);

            match disassemble(&p) {
                Ok(()) => {}
                Err(s) => {
                    error!("Error: {}",s);
                    return;
                }
            }
        }
    }
}

fn exists_path_val(filepath: String) -> result::Result<(), String> {
    match Path::new(&filepath).is_file() {
        true => Ok(()),
        false => Err(format!("'{}': no such file", filepath)),
    }
}

fn disassemble(p: &String) -> Result<()> {
    let (mut proj, machine) = loader::load(Path::new(p))?;
    let maybe_prog = proj.code.pop();
    let reg = proj.data.dependencies.vertex_label(proj.data.root).unwrap().clone();

    if let Some(prog) = maybe_prog {
        let pipe = match machine {
            Machine::Avr => pipeline::<avr::Avr>(prog, reg.clone(), avr::Mcu::atmega103()),
            Machine::Ia32 => pipeline::<amd64::Amd64>(prog, reg.clone(), amd64::Mode::Protected),
            Machine::Amd64 => pipeline::<amd64::Amd64>(prog, reg.clone(), amd64::Mode::Long),
        };

        info!("disassembly thread started");
        for i in pipe.wait() {
            if let Ok(func) = i {
                info!("{}",func.uuid);
            }
        }
        info!("disassembly thread finished");

    }

    Ok(())
}
