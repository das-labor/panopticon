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

use futures::Stream;
use panopticon_amd64 as amd64;
use panopticon_analysis::pipeline;
use panopticon_avr as avr;
use panopticon_core::{ControlFlowTarget, Function, Machine, loader};
use panopticon_graph_algos::GraphTrait;
use std::path::Path;
use std::result;
use std::sync::Arc;
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
    function_filter: Option<String>,
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

fn get_entry_point(func: &Function) -> Option<u64> {
    match func.entry_point() {
        &ControlFlowTarget::Resolved(ref bb) => {
            assert!(func.start == bb.area.start);
            return Some(bb.area.start);
        },
        _ => None
    }
}

fn disassemble(args: Args) -> Result<()> {
    let binary = args.binary;
    let filter = args.function_filter;
    let (mut proj, machine) = loader::load(Path::new(&binary))?;
    let maybe_prog = proj.code.pop();
    let reg = proj.data.dependencies.vertex_label(proj.data.root).unwrap().clone();

    if let Some(prog) = maybe_prog {
        let prog = Arc::new(prog);
        let pipe = {
            let prog = prog.clone();
            match machine {
                Machine::Avr => pipeline::<avr::Avr>(prog, reg.clone(), avr::Mcu::atmega103()),
                Machine::Ia32 => pipeline::<amd64::Amd64>(prog, reg.clone(), amd64::Mode::Protected),
                Machine::Amd64 => pipeline::<amd64::Amd64>(prog, reg.clone(), amd64::Mode::Long),
            }
        };

        info!("disassembly thread started");
        match filter {
            Some(filter) => {
                for function in pipe.wait() {
                    if let Ok(function) = function {
                        if filter == function.name {
                            println!("{}", function.display_with(&prog.clone()));
                            break;
                        }
                    }
                }
            }
            None => {
                let mut functions = pipe.wait()
                    .filter_map(|function| if let Ok(function) = function {
                        info!("{}", function.uuid());
                        Some(function)
                    } else {
                        None
                    })
                    .collect::<Vec<_>>();

                functions.sort_by(|f1, f2| {
                    use std::cmp::Ordering::*;
                    let entry1 = get_entry_point(f1);
                    let entry2 = get_entry_point(f2);
                    match (entry1, entry2) {
                        (Some(entry1), Some(entry2)) => entry1.cmp(&entry2),
                        (Some(_), None) => Greater,
                        (None, Some(_)) => Less,
                        (None, None) => Equal,
                    }
                });

                for function in functions {
                    println!("{}", function.display_with(&prog.clone()));
                }
            }
        }
        info!("disassembly thread finished");
    }
    Ok(())
}

fn run(args: Args) -> Result<()> {
    exists_path_val(&args.binary)?;
    disassemble(args)?;
    Ok(())
}

fn main() {
    env_logger::init().unwrap();
    match run(Args::from_args()) {
        Ok(()) => {}
        Err(s) => {
            error!("Error: {}", s);
            ::std::process::exit(1);
        }
    }
}
