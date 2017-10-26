use bencher::Bencher;
use panopticon_amd64 as amd64;

fn static_amd64_elf_new(b: &mut Bencher) {
    use panopticon_core::{loader,neo,CallTarget,Rvalue};
    use std::path::Path;

    let (proj,_) = loader::load::<neo::Function>(Path::new("../test-data/static")).unwrap();
    let entries = proj.code[0].iter_callgraph().filter_map(|vx| if let &CallTarget::Todo(Rvalue::Constant{ value,.. },_,_) = vx { Some (value) } else { None }).collect::<Vec<u64>>();
    let reg = proj.region();

    b.bench_n(1,|b| {
        b.iter(|| {
            for &ep in entries.iter() {
                let _f: neo::Function = neo::Function::new::<amd64::Amd64>(amd64::Mode::Long,ep,&reg,Some("".into())).unwrap();
            }
        });
    });
}

fn static_amd64_elf_new_old(b: &mut Bencher) {
    use panopticon_core::{loader,neo,CallTarget,Rvalue, Statement};
    use std::path::Path;

    let (proj,_) = loader::load::<neo::Function<Vec<Statement>>>(Path::new("../test-data/static")).unwrap();
    let entries = proj.code[0].iter_callgraph().filter_map(|vx| if let &CallTarget::Todo(Rvalue::Constant{ value,.. },_,_) = vx { Some (value) } else { None }).collect::<Vec<u64>>();
    let reg = proj.region();

    b.bench_n(1,|b| {
        b.iter(|| {
            for &ep in entries.iter() {
                let _f: neo::Function<Vec<Statement>> = neo::Function::new::<amd64::Amd64>(amd64::Mode::Long,ep,&reg,Some("".into())).unwrap();
            }
        });
    });
}

fn static_amd64_elf_new_noop(b: &mut Bencher) {
    use panopticon_core::{loader,neo,CallTarget,Rvalue, Statement, Noop};
    use std::path::Path;

    let (proj,_) = loader::load::<neo::Function<Noop>>(Path::new("../test-data/static")).unwrap();
    let entries = proj.code[0].iter_callgraph().filter_map(|vx| if let &CallTarget::Todo(Rvalue::Constant{ value,.. },_,_) = vx { Some (value) } else { None }).collect::<Vec<u64>>();
    let reg = proj.region();

    b.bench_n(1,|b| {
        b.iter(|| {
            for &ep in entries.iter() {
                let _f: neo::Function<Noop> = neo::Function::new::<amd64::Amd64>(amd64::Mode::Long,ep,&reg,Some("".into())).unwrap();
            }
        });
    });
}

fn static_amd64_elf_old(b: &mut Bencher) {
    use panopticon_core::{loader,Function,CallTarget,Rvalue};
    use std::path::Path;

    let (proj,_) = loader::load::<Function>(Path::new("../test-data/static")).unwrap();
    let entries = proj.code[0].iter_callgraph().filter_map(|vx| if let &CallTarget::Todo(Rvalue::Constant{ value,.. },_,_) = vx { Some (value) } else { None }).collect::<Vec<u64>>();
    let reg = proj.region();

    b.bench_n(1,|b| {
        b.iter(|| {
            for &ep in entries.iter() {
                Function::new::<amd64::Amd64>(ep,&reg,None,amd64::Mode::Long).unwrap();
            }
        });
    });
}

benchmark_group!(disassemble, static_amd64_elf_new, static_amd64_elf_new_old, static_amd64_elf_new_noop, static_amd64_elf_old);
