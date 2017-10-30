#![allow(unused_doc_comment)]
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
extern crate panopticon_data_flow;
extern crate futures;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate termcolor;
extern crate atty;

use panopticon_amd64 as amd64;
use panopticon_analysis::analyze;
use panopticon_avr as avr;
use panopticon_core::{Machine, Function, FunctionKind, Program, Result, loader, neo};
use panopticon_data_flow::{DataFlow};

use std::path::Path;
use std::result;
use structopt::StructOpt;
use std::io::Write;
use termcolor::{BufferWriter, ColorChoice, WriteColor};
use termcolor::Color::*;

#[macro_use]
mod display;
use display::{PrintableStatements, PrintableFunction, PrintableIL};

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
    #[structopt(long = "noop", help = "Nope")]
    noop: bool,
    #[structopt(long = "old", help = "Use the old, deprecated format")]
    old: bool,
    #[structopt(long = "neo", help = "Use the new bincode")]
    neo: bool,
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
    pub fn is_match<IL>(&self, func: &neo::Function<IL>) -> bool {
        if let Some(ref name) = self.name {
            if name == &func.name() || func.aliases().contains(name){ return true }
        }
        if let Some(ref addr) = self.addr {
            return *addr == func.entry_address()
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

fn print_reverse_deps<IL, W: Write + WriteColor>(mut fmt: W, program: &Program<IL>, filter: &Filter) -> Result<()> {
    let name_and_address: Option<(u64, &str)> = {
        if let Some(f) = program.find_function_by(|f| filter.is_match_with(&f.name(), f.entry_address())) {
            Some((f.entry_address(), f.name()))
        } else {
            // not a function, so we search imports
            let mut name_and_address = None;
            for (addr, name) in program.imports.iter() {
                if filter.is_match_with(name, *addr) {
                    debug!("Found import with matching name or address, {:#x} - {}", addr, name);
                    name_and_address = Some((*addr, name.as_str()));
                }
            }
            name_and_address
        }
    };
    match name_and_address {
        Some((addr, name)) => {
            let mut reverse_deps: Vec<_> = program.functions().filter_map(|f| {
                //let call_addresses = f.collect_call_addresses();
                let call_addresses = vec![];
                debug!("Total call addresses for {}: {}", f.name(), call_addresses.len());
                if call_addresses.contains(&addr) {
                    Some((f.entry_address(), f.name().to_string()))
                } else {
                    for call_address in call_addresses {
                        let function = program.find_function_by(|f| f.entry_address() == call_address).expect(&format!("{} has a call address {:#x}, but there isn't a function with that address in the program object", f.name(), call_address));
                        debug!("Checking function {} with call address {:#x} for plt stub", function.name(), call_address);
                        match function.kind() {
                            &FunctionKind::Stub { ref plt_address, ref name } => {
                                debug!("Function {} is a plt stub for {}", function.name(), name);
                                if *plt_address == addr {
                                    debug!("Function {} plt address {:#x} matches reverse dep address {:#x}, returning", f.name(), plt_address, addr);
                                    return Some((f.entry_address(), f.name().to_string()))
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

fn disassemble<IL: neo::Language + Default + Send>(binary: &str) -> Result<Program<IL>> {
    let (mut proj, machine) = loader::load(Path::new(&binary))?;
    let program = proj.code.pop().unwrap();
    let reg = proj.region();
    info!("disassembly thread started");
    match machine {
        Machine::Avr =>   analyze::<avr::Avr, IL>(program, reg, avr::Mcu::atmega103()),
        Machine::Ia32 =>  analyze::<amd64::Amd64, IL>(program, reg, amd64::Mode::Protected),
        Machine::Amd64 => analyze::<amd64::Amd64, IL>(program, reg, amd64::Mode::Long),
    }
}

pub use panopticon_core::NoopStatement;
pub use panopticon_core::Noop;


//fn app_logic<'a, Function: Fun + DataFlow + PrintableFunction + PrintableStatements>(fmt: &mut termcolor::Buffer, program: &mut Program<Function>, args: Args) -> Result<()> {
fn app_logic<IL: neo::Language>(fmt: &mut termcolor::Buffer, program: &mut Program<IL>, args: Args) -> Result<()>
    where neo::Function<IL>: DataFlow + PrintableFunction<IL> + PrintableStatements,
{
    let filter = Filter { name: args.function_filter, addr: args.address_filter.map(|addr| u64::from_str_radix(&addr, 16).unwrap()) };

    debug!("Program.imports: {:#?}", program.imports);
    if args.reverse_deps && filter.filtering() {
        return print_reverse_deps(fmt, &program, &filter);
    }
    let mut functions = {
        // we iterate twice because rust ownership system sucks sometimes
        for f in program.functions_mut() {
            if filter.is_match(f) {
                // we use less memory and take less time if we perform the ssa conversion _only_ on functions
                // we want to examine (e.g., ones that match our filter)
                f.ssa_conversion()?;
            }
        }
        program.functions().filter_map(|f| if filter.is_match(f) { Some(f.clone()) } else { None }).collect::<Vec<_>>()
    };

    info!("disassembly thread finished with {} functions", functions.len());

    functions.sort_by(|f1, f2| {
        let entry1 = f1.first_address();
        let entry2 = f2.first_address();
        entry1.cmp(&entry2)
    });

    for function in functions {
        function.pretty_print(fmt, &program)?;
        if args.calls {
            // FIXME: add collect_call_addresses
//            let calls = function.collect_call_addresses();
//            write!(fmt, "Calls (")?;
//            color!(fmt, Green, calls.len().to_string())?;
//            writeln!(fmt, "):")?;
//            for addr in calls {
//                color_bold!(fmt, Red, format!("{:>8x}", addr))?;
//                writeln!(fmt, "")?;
//            }
        }
        // move this into pretty_print
        writeln!(fmt, "Aliases: {:?}", function.aliases())?;
        if args.dump_il {
            function.pretty_print_il(fmt)?;
        }
    }
    Ok(())
}

fn run(args: Args) -> Result<()> {
    exists_path_val(&args.binary)?;
    let cc = if args.color || atty::is(atty::Stream::Stdout) { ColorChoice::Auto } else { ColorChoice::Never };
    let writer = BufferWriter::stdout(cc);
    let mut fmt = writer.buffer();
    if args.noop {
        let mut program = disassemble::<Noop>(&args.binary)?;
        app_logic(&mut fmt, &mut program, args)?;
    } else {
        if args.neo {
            let mut program = disassemble::<neo::Bitcode>(&args.binary)?;
            app_logic(&mut fmt, &mut program, args)?;
        } else {
            let mut program = disassemble::<neo::RREIL>(&args.binary)?;
            app_logic(&mut fmt, &mut program, args)?;
        }
    }
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

/*
INFO:panopticon_analysis::pipeline: first wave done: success: 1748 failures: 0 targets: 806
INFO:panopticon_analysis::pipeline: targets - (806)
INFO:panopticon_analysis::pipeline: targets - (456)
INFO:panopticon_analysis::pipeline: targets - (245)
INFO:panopticon_analysis::pipeline: targets - (95)
INFO:panopticon_analysis::pipeline: targets - (49)
INFO:panopticon_analysis::pipeline: targets - (20)
INFO:panopticon_analysis::pipeline: Finished analysis: 2286 failures 0
INFO:panop: disassembly thread finished with 2286 functions
Total bytes: 753214720
15.62user 0.48system 0:05.32elapsed 302%CPU (0avgtext+0avgdata 1243060maxresident)k
0inputs+0outputs (0major+322752minor)pagefaults 0swaps

INFO:panopticon_analysis::pipeline: first wave done: success: 1748 failures: 0 targets: 0
INFO:panopticon_analysis::pipeline: Finished analysis: 1748 failures 0
Total_bytes: 13561144
3.67user 0.19system 0:01.31elapsed 294%CPU (0avgtext+0avgdata 154000maxresident)k
0inputs+0outputs (0major+80572minor)pagefaults 0swaps

Aliases: ["printf"]
21.74user 0.39system 0:07.71elapsed 287%CPU (0avgtext+0avgdata 443572maxresident)k
0inputs+0outputs (0major+133326minor)pagefaults 0swaps

RUST_LOG=panop=info /usr/bin/time ./panop /usr/lib/libc.so.6
52.02user 1.20system 0:16.66elapsed 319%CPU (0avgtext+0avgdata 2756540maxresident)k
0inputs+0outputs (0major+658616minor)pagefaults 0swaps

RUST_LOG=panop=info /usr/bin/time ./panop --neo /usr/lib/libc.so.6
21.26user 0.34system 0:06.98elapsed 309%CPU (0avgtext+0avgdata 450880maxresident)k
104inputs+0outputs (0major+134643minor)pagefaults 0swaps
*/
