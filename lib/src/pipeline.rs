use futures::sync::mpsc;
use futures::{stream,Stream,Sink,Future};
use std::thread;
use std::collections::{
    HashSet,
    HashMap,
};
use std::iter::FromIterator;
use {
    Architecture,
    Program,
    Function,
    Region,
    CallTarget,
    Rvalue,
    ssa_convertion,
};
use graph_algos::{
    VertexListGraphTrait,
    GraphTrait,
};
use std::fmt::Debug;
//use rayon::prelude::*;

pub fn pipeline<A: Architecture + Debug + 'static >(program: Program, region: Region, config: A::Configuration) -> Box<Stream<Item=Function,Error=()> + Send>
where A::Configuration: Debug {
    let (tx,rx) = mpsc::channel::<Function>(10);
    let name = region.name().clone();
    thread::spawn(move || {
        let mut tx = tx;
        let mut functions = HashMap::<u64,Function>::new();
        let mut targets = HashSet::<u64>::from_iter(program.call_graph.vertices().filter_map(|vx| {
            match program.call_graph.vertex_label(vx) {
                Some(&CallTarget::Todo(Rvalue::Constant{ value: ref entry,.. },ref maybe_name,ref uuid)) => Some(*entry),
                Some(a) => None,
                None => unreachable!(),
            }
        }));

        while !targets.is_empty() {
            info!("disassemble {:?}",targets);
            let (new_targets,new_fns): (Vec<Vec<u64>>,Vec<Function>) = targets.into_iter().map(|entry| {
                let mut f = Function::disassemble::<A>(None,config.clone(),&region,entry);
                let new_ct = f.collect_calls().into_iter().filter_map(|rv| {
                    if let Rvalue::Constant{ value,.. } = rv {
                        if !functions.contains_key(&value) && entry != value {
                            return Some(value);
                        }
                    }
                    None
                }).collect::<Vec<u64>>();

                ssa_convertion(&mut f);

                functions.insert(entry,f.clone());
                (new_ct,f)
            }).unzip();
            targets = new_targets.into_iter().flat_map(|x| x).collect();
            tx = tx.send_all(stream::iter(new_fns.into_iter().map(|x| Ok(x)))).wait().unwrap().0;
        }
    });

    Box::new(rx)
}
