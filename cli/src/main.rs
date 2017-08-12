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
extern crate termcolor;
extern crate atty;

use panopticon_amd64 as amd64;
use panopticon_analysis::analyze;
use panopticon_avr as avr;
use panopticon_core::{Machine, Function, loader};
use std::path::Path;
use std::result;
use structopt::StructOpt;

mod display;

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
    /// Dumps the il of the matched function
    #[structopt(long = "il", help = "Print the rreil of this function")]
    dump_il: bool,
    #[structopt(long = "color", help = "Forces coloring, even when piping to a file, etc.")]
    color: bool,
    /// Print every function the function calls
    #[structopt(short = "c", long = "calls", help = "Print every address of every function this function calls")]
    calls: bool,
    /// The specific function to disassemble
    #[structopt(short = "f", long = "function", help = "Disassemble the given function, or any of its aliases")]
    function_filter: Option<String>,
    /// The specific function address to disassemble
    #[structopt(short = "a", long = "address", help = "Disassemble the function at the given address")]
    address_filter: Option<String>,
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

struct Filter {
    name: Option<String>,
    addr: Option<u64>,
}

impl Filter {
    pub fn filtering(&self) -> bool {
        self.name.is_some() || self.addr.is_some()
    }
    pub fn is_match(&self, func: &Function) -> bool {
        if let Some(ref name) = self.name {
            if name == &func.name || func.aliases().contains(name){ return true }
        }
        if let Some(ref addr) = self.addr {
            return *addr == func.start()
        }
        !self.filtering()
    }
}

fn disassemble(args: Args) -> Result<()> {
    let binary = args.binary;
    let filter = Filter { name: args.function_filter, addr: args.address_filter.map(|addr| u64::from_str_radix(&addr, 16).unwrap()) };
    let (mut proj, machine) = loader::load(Path::new(&binary))?;
    let program = proj.code.pop().unwrap();
    let reg = proj.region().clone();
    info!("disassembly thread started");
    let program = {
        match machine {
            Machine::Avr => analyze::<avr::Avr>(program, reg.clone(), avr::Mcu::atmega103()),
            Machine::Ia32 => analyze::<amd64::Amd64>(program, reg.clone(), amd64::Mode::Protected),
            Machine::Amd64 => analyze::<amd64::Amd64>(program, reg.clone(), amd64::Mode::Long),
        }
    }?;
    let mut functions = program.functions().filter_map(|f| if filter.is_match(f) { Some(f) } else { None }).collect::<Vec<&Function>>();
    info!("disassembly thread finished with {} functions", functions.len());

    functions.sort_by(|f1, f2| {
        let entry1 = f1.start();
        let entry2 = f2.start();
        entry1.cmp(&entry2)
    });

    for function in functions {
        display::print_function(&function, &program, args.color)?;
        if args.calls {
            let calls = function.collect_call_addresses();
            println!("Calls ({}):", calls.len());
            for addr in calls {
                println!("{:>8x}", addr);
            }
        }
        if args.dump_il {
            let statements = function.statements();
            for statement in statements {
                println!("{}", statement);
            }
        }
        println!("Aliases: {:?}", function.aliases());
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
