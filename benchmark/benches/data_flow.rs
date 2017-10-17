use bencher::Bencher;
use panopticon_amd64 as amd64;
use panopticon_data_flow::DataFlow;
use panopticon_data_flow::neo::rewrite_to_ssa;

fn ssa_convertion_new(b: &mut Bencher) {
    use panopticon_core::{loader,neo,CallTarget,Rvalue};
    use std::path::Path;

    let (proj,_) = loader::load::<neo::Function>(Path::new("../test-data/static")).unwrap();
    let entries = proj.code[0].iter_callgraph().filter_map(|vx| if let &CallTarget::Todo(Rvalue::Constant{ value,.. },_,_) = vx { Some (value) } else { None }).collect::<Vec<_>>();
    let reg = proj.region();
    let mut funcs = vec![];

    for &ep in entries.iter() {
        let func = neo::Function::new::<amd64::Amd64>(amd64::Mode::Long,ep,&reg,Some("".into())).unwrap();
        funcs.push(func);
    }

    b.bench_n(1,|b| {
        b.iter(|| {
            for f in funcs.iter_mut() { rewrite_to_ssa(f).unwrap(); }
        });
    });
}

fn ssa_convertion_old(b: &mut Bencher) {
    use panopticon_core::{loader,Function,CallTarget,Rvalue};
    use std::path::Path;

    let (proj,_) = loader::load::<Function>(Path::new("../test-data/static")).unwrap();
    let entries = proj.code[0].iter_callgraph().filter_map(|vx| if let &CallTarget::Todo(Rvalue::Constant{ value,.. },_,_) = vx { Some (value) } else { None }).collect::<Vec<_>>();
    let reg = proj.region();
    let mut funcs = vec![];

    for &ep in entries.iter() {
        let func = Function::new::<amd64::Amd64>(ep,&reg,None,amd64::Mode::Long).unwrap();
        funcs.push(func);
    }

    b.bench_n(1,|b| {
        b.iter(|| {
            // NB: this is broken right now for old functions using petgraph, unwrap() on None
            // 11: <petgraph::algo::dominators::Dominators<N>>::dominance_frontiers
            // at /home/m4b/.cargo/git/checkouts/petgraph-d1e6db80f82b1362/6ea8c94/src/algo/dominators.rs:112
            // 12: panopticon_data_flow::ssa::phi_functions
            //  at /home/m4b/projects/panopticon/data-flow/src/ssa.rs:151
            for f in funcs.iter_mut() { f.ssa_conversion().unwrap(); }
        });
    });
}

benchmark_group!(ssa, ssa_convertion_new, ssa_convertion_old);
