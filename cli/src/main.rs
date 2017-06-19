extern crate structopt;
#[macro_use]
extern crate structopt_derive;
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
use structopt::StructOpt;

mod errors {
    error_chain! {
        foreign_links {
            Panopticon(::panopticon_core::Error);
        }
    }
}
use errors::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "panop", about = "A libre cross-platform disassembler.")]
struct Args {
    /// The specific function to disassemble
    #[structopt(short = "f", long = "function", help = "Disassemble the given function")]
    function: Option<String>,
    /// The binary to disassemble
    #[structopt(help = "The binary to disassemble")]
    binary: String,
}

fn exists_path_val(filepath: &str) -> result::Result<(), String> {
    match Path::new(filepath).is_file() {
        true => Ok(()),
        false => Err(format!("'{}': no such file", filepath)),
    }
}

fn disassemble(binary: &str, filter: &Option<String>) -> Result<()> {
    let (mut proj, machine) = loader::load(Path::new(binary))?;
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
                if let &Some(ref filter) = filter {
                    if func.name.as_str() == filter {
                        info!("Filter matches {} - {:?}", func.name, func.cflow_graph);
                    }
                }
            }
        }
        info!("disassembly thread finished");
    }

    Ok(())
}

fn run(args: Args) -> Result<()> {
    exists_path_val(&args.binary)?;
    disassemble(&args.binary, &args.function)?;
    Ok(())
}

fn main() {
    env_logger::init().unwrap();
    match run(Args::from_args()) {
        Ok(()) => {}
        Err(s) => {
            error!("Error: {}",s);
            ::std::process::exit(1);
        }
    }
}
