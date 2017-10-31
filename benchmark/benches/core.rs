use bencher::Bencher;
use panopticon_amd64 as amd64;
use std::path::Path;
use panopticon_core::{loader,CallTarget,Rvalue,Function,RREIL};
use panopticon_core::il::noop::Noop;

fn static_amd64_elf_neo(b: &mut Bencher) {
    let (proj,_) = loader::load::<Function>(Path::new("../test-data/static")).unwrap();
    let entries = proj.code[0].iter_callgraph().filter_map(|vx| if let &CallTarget::Todo(Rvalue::Constant{ value,.. },_,_) = vx { Some (value) } else { None }).collect::<Vec<u64>>();
    let reg = proj.region();

    b.bench_n(1,|b| {
        b.iter(|| {
            for &ep in entries.iter() {
                let _f: Function = Function::new::<amd64::Amd64>(amd64::Mode::Long,ep,&reg,Some("".into())).unwrap();
            }
        });
    });
}

fn static_amd64_elf_rreil(b: &mut Bencher) {
    let (proj,_) = loader::load::<Function<RREIL>>(Path::new("../test-data/static")).unwrap();
    let entries = proj.code[0].iter_callgraph().filter_map(|vx| if let &CallTarget::Todo(Rvalue::Constant{ value,.. },_,_) = vx { Some (value) } else { None }).collect::<Vec<u64>>();
    let reg = proj.region();

    b.bench_n(1,|b| {
        b.iter(|| {
            for &ep in entries.iter() {
                let _f: Function<RREIL> = Function::new::<amd64::Amd64>(amd64::Mode::Long,ep,&reg,Some("".into())).unwrap();
            }
        });
    });
}

fn static_amd64_elf_noop(b: &mut Bencher) {
    let (proj,_) = loader::load::<Function<Noop>>(Path::new("../test-data/static")).unwrap();
    let entries = proj.code[0].iter_callgraph().filter_map(|vx| if let &CallTarget::Todo(Rvalue::Constant{ value,.. },_,_) = vx { Some (value) } else { None }).collect::<Vec<u64>>();
    let reg = proj.region();

    b.bench_n(1,|b| {
        b.iter(|| {
            for &ep in entries.iter() {
                let _f: Function<Noop> = Function::new::<amd64::Amd64>(amd64::Mode::Long,ep,&reg,Some("".into())).unwrap();
            }
        });
    });
}

benchmark_group!(disassemble, static_amd64_elf_neo, static_amd64_elf_rreil, static_amd64_elf_noop);
