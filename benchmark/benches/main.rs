#[macro_use]
extern crate bencher;
extern crate panopticon_amd64;
extern crate panopticon_data_flow;
extern crate panopticon_core;

mod core;
mod data_flow;


benchmark_main!(core::disassemble);
