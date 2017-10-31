use bencher::Bencher;
use panopticon_amd64 as amd64;
use panopticon_core::{loader,CallTarget,Rvalue,Function,RREIL};
use panopticon_core::il::noop::Noop;
use panopticon_data_flow::{DataFlow, rewrite_to_ssa};
use std::path::Path;

fn ssa_convertion_neo(b: &mut Bencher) {
    let (proj,_) = loader::load::<Function>(Path::new("../test-data/static")).unwrap();
    let entries = proj.code[0].iter_callgraph().filter_map(|vx| if let &CallTarget::Todo(Rvalue::Constant{ value,.. },_,_) = vx { Some (value) } else { None }).collect::<Vec<_>>();
    let reg = proj.region();
    let mut funcs = vec![];

    for &ep in entries.iter() {
        let func = Function::new::<amd64::Amd64>(amd64::Mode::Long,ep,&reg,Some("".into())).unwrap();
        funcs.push(func);
    }

    b.bench_n(1,|b| {
        b.iter(|| {
            for f in funcs.iter_mut() { rewrite_to_ssa(f).unwrap(); }
        });
    });
}

fn ssa_convertion_rreil(b: &mut Bencher) {
    let (proj,_) = loader::load::<Function<RREIL>>(Path::new("../test-data/static")).unwrap();
    let entries = proj.code[0].iter_callgraph().filter_map(|vx| if let &CallTarget::Todo(Rvalue::Constant{ value,.. },_,_) = vx { Some (value) } else { None }).collect::<Vec<_>>();
    let reg = proj.region();
    let mut funcs = vec![];

    for &ep in entries.iter() {
        let func: Function<RREIL> = Function::new::<amd64::Amd64>(amd64::Mode::Long,ep,&reg,Some("".into())).unwrap();
        funcs.push(func);
    }

    b.bench_n(1,|b| {
        b.iter(|| {
            for f in funcs.iter_mut() { f.ssa_conversion().unwrap(); }
        });
    });
}

fn ssa_convertion_noop(b: &mut Bencher) {
    let (proj,_) = loader::load::<Function<Noop>>(Path::new("../test-data/static")).unwrap();
    let entries = proj.code[0].iter_callgraph().filter_map(|vx| if let &CallTarget::Todo(Rvalue::Constant{ value,.. },_,_) = vx { Some (value) } else { None }).collect::<Vec<_>>();
    let reg = proj.region();
    let mut funcs = vec![];

    for &ep in entries.iter() {
        let func: Function<Noop> = Function::new::<amd64::Amd64>(amd64::Mode::Long,ep,&reg,Some("".into())).unwrap();
        funcs.push(func);
    }

    b.bench_n(1,|b| {
        b.iter(|| {
            for f in funcs.iter_mut() { f.ssa_conversion().unwrap(); }
        });
    });
}

benchmark_group!(ssa, ssa_convertion_neo, ssa_convertion_rreil, ssa_convertion_noop);
