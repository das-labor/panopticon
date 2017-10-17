use bencher::Bencher;
use panopticon_amd64 as amd64;

fn static_amd64_elf_new(b: &mut Bencher) {
    use panopticon_core::{loader,neo,CallTarget,Rvalue};
    use panopticon_graph_algos::{VertexListGraphTrait,GraphTrait};
    use std::path::Path;

    let (proj,_) = loader::load::<neo::Function>(Path::new("../test-data/static")).unwrap();
    let entries = proj.code[0].call_graph.vertices().filter_map(|vx| if let Some(&CallTarget::Todo(Rvalue::Constant{ value,.. },_,_)) = proj.code[0].call_graph.vertex_label(vx) { Some(value) } else { None }).collect::<Vec<_>>();
    let reg = proj.data.dependencies.vertex_label(proj.data.root).unwrap();

    b.bench_n(1,|b| {
        b.iter(|| {
            for &ep in entries.iter() {
                neo::Function::new::<amd64::Amd64>(amd64::Mode::Long,ep,&reg,Some("".into())).unwrap();
            }
        });
    });
}

fn static_amd64_elf_old(b: &mut Bencher) {
    use panopticon_core::{loader,Function,CallTarget,Rvalue};
    use panopticon_graph_algos::{VertexListGraphTrait,GraphTrait};
    use std::path::Path;

    let (proj,_) = loader::load::<Function>(Path::new("../test-data/static")).unwrap();
    let entries = proj.code[0].call_graph.vertices().filter_map(|vx| if let Some(&CallTarget::Todo(Rvalue::Constant{ value,.. },_,_)) = proj.code[0].call_graph.vertex_label(vx) { Some(value) } else { None }).collect::<Vec<_>>();
    let reg = proj.data.dependencies.vertex_label(proj.data.root).unwrap();

    b.bench_n(1,|b| {
        b.iter(|| {
            for &ep in entries.iter() {
                Function::new::<amd64::Amd64>(ep,&reg,None,amd64::Mode::Long).unwrap();
            }
        });
    });
}

benchmark_group!(disassemble, static_amd64_elf_new, static_amd64_elf_old);
