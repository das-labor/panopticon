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
use panopticon_core::{Machine, Function, Program, Result, loader};
use panopticon_core::function::Kind;
use std::path::Path;
use std::result;
use structopt::StructOpt;
use std::io::Write;
use termcolor::{BufferWriter, ColorChoice, WriteColor};
use termcolor::Color::*;

#[macro_use]
mod display;

mod errors {
    error_chain! {
        foreign_links {
            Panopticon(::panopticon_core::Error);
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "panop", about = "A libre cross-platform disassembler.")]
struct Args {
    #[structopt(long = "reverse-deps", help = "Print every function that calls the function in -f")]
    reverse_deps: bool,
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

#[derive(Debug)]
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
    pub fn is_match_with(&self, name: &str, addr: u64) -> bool {
        if let Some(ref name_) = self.name {
            if name == name_ { return true }
        }
        if let Some(ref addr_) = self.addr {
            return addr == *addr_
        }
        !self.filtering()
    }
}

fn print_reverse_deps<W: Write + WriteColor>(mut fmt: W, program: &Program, filter: &Filter) -> Result<()> {
    let name_and_address = {
        if let Some(f) = program.find_function_by(|f| filter.is_match_with(&f.name, f.start())) {
            Some((f.start(), &f.name))
        } else {
            // not a function, so we search imports
            let mut name_and_address = None;
            for (addr, name) in program.imports.iter() {
                if filter.is_match_with(name, *addr) {
                    debug!("Found import with matching name or address, {:#x} - {}", addr, name);
                    name_and_address = Some((*addr, name));
                }
            }
            name_and_address
        }
    };
    match name_and_address {
        Some((addr, name)) => {
            let mut reverse_deps: Vec<_> = program.functions().filter_map(|f| {
                let call_addresses = f.collect_call_addresses();
                debug!("Total call addresses for {}: {}", f.name, call_addresses.len());
                if call_addresses.contains(&addr) {
                    Some((f.start(), f.name.to_string()))
                } else {
                    for call_address in call_addresses {
                        let function = program.find_function_by(|f| f.start() == call_address).unwrap();
                        debug!("Checking function {} with call address {:#x} for plt stub", function.name, call_address);
                        match function.kind() {
                            &Kind::Stub { ref plt_address, ref name } => {
                                debug!("Function {} is a plt stub for {}", function.name, name);
                                if *plt_address == addr {
                                    debug!("Function {} plt address {:#x} matches reverse dep address {:#x}, returning", f.name, plt_address, addr);
                                    return Some((f.start(), f.name.to_string()))
                                }
                            },
                            _ => ()
                        }
                    }
                    None
                }
            }).collect();

            reverse_deps.sort();

            write!(fmt, "Found ")?;
            color!(fmt, Green, reverse_deps.len().to_string())?;
            write!(fmt, " reverse dependencies for ")?;
            color_bold!(fmt, Yellow, name)?;
            write!(fmt, " @ ")?;
            color_bold!(fmt, Red, format!("{:#x}", addr))?;
            writeln!(fmt, "")?;
            for (addr, name) in reverse_deps {
               color_bold!(fmt, Red, format!("{: >16x} ", addr))?;
               color_bold!(fmt, Yellow, name)?;
               writeln!(fmt, "")?;
            }
        },
        None => {
            return Err(format!("function {:?} did not apply",  filter).into());
        }
    }
    Ok(())
}

fn disassemble(binary: &str) -> Result<Program> {
    let (mut proj, machine) = loader::load(Path::new(&binary))?;
    let program = proj.code.pop().unwrap();
    let reg = proj.region().clone();
    info!("disassembly thread started");
    Ok(match machine {
        Machine::Avr => analyze::<avr::Avr>(program, reg.clone(), avr::Mcu::atmega103()),
        Machine::Ia32 => analyze::<amd64::Amd64>(program, reg.clone(), amd64::Mode::Protected),
        Machine::Amd64 => analyze::<amd64::Amd64>(program, reg.clone(), amd64::Mode::Long),
    }?)
}

fn format(fmt: &mut termcolor::Buffer, program: Program, args: Args) -> Result<()> {
    let filter = Filter { name: args.function_filter, addr: args.address_filter.map(|addr| u64::from_str_radix(&addr, 16).unwrap()) };

    debug!("Program.imports: {:#?}", program.imports);
    if args.reverse_deps && filter.filtering() {
        return print_reverse_deps(fmt, &program, &filter);
    }
    let mut functions = program.functions().filter_map(|f| if filter.is_match(f) { Some(f) } else { None }).collect::<Vec<&Function>>();
    info!("disassembly thread finished with {} functions", functions.len());

    functions.sort_by(|f1, f2| {
        let entry1 = f1.start();
        let entry2 = f2.start();
        entry1.cmp(&entry2)
    });

    for function in functions {
        display::print_function(fmt, &function, &program)?;
        if args.calls {
            let calls = function.collect_call_addresses();
            write!(fmt, "Calls (")?;
            color!(fmt, Green, calls.len().to_string())?;
            writeln!(fmt, "):")?;
            for addr in calls {
                color_bold!(fmt, Red, format!("{:>8x}", addr))?;
                writeln!(fmt, "")?;
            }
        }
        if args.dump_il {
            let statements = function.statements();
            for statement in statements {
                writeln!(fmt, "{}", statement)?;
            }
        }
        writeln!(fmt, "Aliases: {:?}", function.aliases())?;
    }
    Ok(())
}

fn run(args: Args) -> Result<()> {
    exists_path_val(&args.binary)?;
    let program = disassemble(&args.binary)?;
    let cc = if args.color || atty::is(atty::Stream::Stdout) { ColorChoice::Auto } else { ColorChoice::Never };
    let writer = BufferWriter::stdout(cc);
    let mut fmt = writer.buffer();
    format(&mut fmt, program, args)?;
    writer.print(&fmt)?;
    Ok(())
}

fn main() {
    env_logger::init().unwrap();
    match run(Args::from_args()) {
        Ok(()) => {}
        Err(s) => {
            error!("Error: {}", s);
            println!("Error: {}", s);
            ::std::process::exit(1);
        }
    }
}
