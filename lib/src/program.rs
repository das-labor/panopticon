use function::{ControlFlowTarget,Function};
use graph_algos::{AdjacencyList,GraphTrait,MutableGraphTrait};
use graph_algos::adjacency_list::{AdjacencyListEdgeDescriptor,AdjacencyListVertexDescriptor};
use graph_algos::{VertexListGraphTrait,EdgeListGraphTrait};
use num::traits::NumCast;
use std::fmt::{Display,Debug};
use std::ops::{BitAnd,BitOr,Shl,Shr,Not};
use std::rc::Rc;
use disassembler::{Architecture,Disassembler,State};
use layer::LayerIter;
use std::collections::HashSet;

#[derive(RustcDecodable,RustcEncodable)]
pub enum CallTarget {
    Concrete(Function),
    Symbolic(String),
}

pub type CallGraph = AdjacencyList<CallTarget,()>;
pub type CallGraphRef = AdjacencyListVertexDescriptor;

#[derive(RustcDecodable,RustcEncodable)]
pub struct Program {
    pub name: String,
    pub call_graph: CallGraph,
}

pub enum DisassembleEvent {
    Discovered(u64),
    Started(u64),
    Done(u64),
}

impl Program {
    pub fn new(n: &str) -> Program {
        Program{
            name: n.to_string(),
            call_graph: CallGraph::new(),
        }
    }

    pub fn find_function_by_entry(&self, a: u64) -> Option<CallGraphRef> {
        self.call_graph.vertices().find(|&x| match self.call_graph.vertex_label(x) {
            Some(&CallTarget::Concrete(ref s)) => {
                if let Some(e) = s.entry_point {
                    if let Some(&ControlFlowTarget::Resolved(ref ee)) = s.cflow_graph.vertex_label(e) {
                        ee.area.start == a
                    } else {
                        false
                    }
                } else {
                    false
                }
            },
            _ => false
        })
    }

    pub fn disassemble<A: Architecture,F: Fn(DisassembleEvent)>(cont: Option<Program>, dec: Rc<Disassembler<A>>, init: A::Configuration, data: LayerIter,
                                                                start: u64, progress: Option<F>) -> Program {
        if cont.is_some() && cont.as_ref().map(|x| x.find_function_by_entry(start)).is_some() {
            return cont.unwrap();
        }

        let mut worklist = HashSet::new();
        let mut ret = cont.unwrap_or(Program::new(&format!("prog_{}",start)));

		worklist.insert(start);

        if let Some(ref f) = progress {
            f(DisassembleEvent::Discovered(start))
        }

		while !worklist.is_empty() {
			let tgt = *worklist.iter().next().unwrap();
			worklist.remove(&tgt);

            if let Some(ref f) = progress {
                f(DisassembleEvent::Started(tgt))
            }

            if ret.find_function_by_entry(tgt).is_some() {
                continue;
            }

            println!("Disassemble at {}",tgt);

            let new_fun = Function::disassemble::<A>(None,dec.clone(),init.clone(),data.clone(),tgt);

            if let Some(ref f) = progress {
                f(DisassembleEvent::Done(tgt));
            }

            if new_fun.cflow_graph.num_vertices() > 0 {
				// XXX: compute dominance tree
				// XXX: compute liveness information
				// XXX: resolve indirect calls

				// add to call graph
				let new_vx = ret.call_graph.add_vertex(CallTarget::Concrete(new_fun));
                let mut new = Vec::new();

                if let Some(&CallTarget::Concrete(ref fun)) = ret.call_graph.vertex_label(new_vx) {
                    // insert call edges and new procedures to disassemble
                    for a in fun.collect_calls() {
                        if let Some(other_fun) = ret.find_function_by_entry(a) {
                            new.push(other_fun);
                        } else {
                            if let Some(ref f) = progress {
                                f(DisassembleEvent::Discovered(a))
                            }
                            worklist.insert(a);
                        }
                    }
                }

                for other_fun in new {
                    ret.call_graph.add_edge((),new_vx,other_fun);
                }
            }
		}

		ret
	}
}

