use function::{ControlFlowTarget,Function};
use graph_algos::{AdjacencyList,GraphTrait,MutableGraphTrait,AdjacencyMatrixGraphTrait};
use graph_algos::adjacency_list::AdjacencyListVertexDescriptor;
use graph_algos::VertexListGraphTrait;
use std::rc::Rc;
use disassembler::{Architecture,Disassembler};
use layer::LayerIter;
use std::collections::HashSet;
use uuid::Uuid;

#[derive(RustcDecodable,RustcEncodable)]
pub enum CallTarget {
    Concrete(Function),
    Symbolic(String,Uuid),
    Todo(u64,Uuid),
}

impl CallTarget {
    pub fn uuid(&self) -> Uuid {
        match self {
            &CallTarget::Concrete(Function{ uuid,..}) => uuid,
            &CallTarget::Symbolic(_,uuid) => uuid,
            &CallTarget::Todo(_,uuid) => uuid,
        }
    }
}

pub type CallGraph = AdjacencyList<CallTarget,()>;
pub type CallGraphRef = AdjacencyListVertexDescriptor;

#[derive(RustcDecodable,RustcEncodable)]
pub struct Program {
    pub uuid: Uuid,
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
            uuid: Uuid::new_v4(),
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

    pub fn find_function_by_uuid<'a>(&'a self, a: &Uuid) -> Option<&'a Function> {
        self.call_graph.vertices().find(|&x| match self.call_graph.vertex_label(x) {
            Some(&CallTarget::Concrete(ref s)) => s.uuid == *a,
            _ => false,
        }).and_then(|r| match self.call_graph.vertex_label(r) {
            Some(&CallTarget::Concrete(ref s)) => Some(s),
            _ => None
        })
    }

    pub fn find_function_by_uuid_mut<'a>(&'a mut self, a: &Uuid) -> Option<&'a mut Function> {
        let ct = self.call_graph.vertices().find(|&x| match self.call_graph.vertex_label(x) {
            Some(&CallTarget::Concrete(ref s)) => s.uuid == *a,
            _ => false,
        });

        if ct.is_none() {
            return None;
        }

        match self.call_graph.vertex_label_mut(ct.unwrap()) {
            Some(&mut CallTarget::Concrete(ref mut s)) => Some(s),
            _ => None
        }
    }

    pub fn disassemble<A: Architecture,F: Fn(DisassembleEvent)>(cont: Option<Program>, dec: Rc<Disassembler<A>>, init: A::Configuration, data: LayerIter,
                                                                start: u64, reg: String, progress: Option<F>) -> Program {
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

            let new_fun = Function::disassemble::<A>(None,dec.clone(),init.clone(),data.clone(),tgt,reg.clone());

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

    pub fn insert(&mut self, new_fun: Function) -> Vec<Uuid> {
        let maybe_vx = self.call_graph.vertices().find(|ct| {
            let uu = new_fun.uuid;

            match self.call_graph.vertex_label(*ct) {
                Some(&CallTarget::Concrete(Function{ uuid: _uu,..})) => _uu == uu,
                Some(&CallTarget::Todo(_,_uu)) => _uu == uu,
                Some(&CallTarget::Symbolic(_,_uu)) => _uu == uu,
                _ => false
            }
        });

        let new_vx = if let Some(vx) = maybe_vx {
            *self.call_graph.vertex_label_mut(vx).unwrap() = CallTarget::Concrete(new_fun);
            vx
        } else {
            self.call_graph.add_vertex(CallTarget::Concrete(new_fun))
        };

        let mut other_funs = Vec::new();
        let mut ret = Vec::new();
        let calls = if let Some(&CallTarget::Concrete(ref fun)) = self.call_graph.vertex_label(new_vx) {
            fun.collect_calls()
        } else {
            unreachable!();
        };

        for a in calls {
            let l = other_funs.len();

            for w in self.call_graph.vertices() {
                match self.call_graph.vertex_label(w) {
                    Some(&CallTarget::Concrete(Function{ cflow_graph: ref cg, entry_point: Some(ent),.. })) => {
                        if let Some(&ControlFlowTarget::Resolved(ref bb)) = cg.vertex_label(ent) {
                            if bb.area.start == a {
                                other_funs.push(w);
                                break;
                            }
                        }
                    },
                    Some(&CallTarget::Todo(_a,_)) => {
                        if _a == a {
                            other_funs.push(w);
                            break;
                        }
                    },
                    _ => {
                    }
                }
            }

            if l == other_funs.len() {
                let uu = Uuid::new_v4();
                let v = self.call_graph.add_vertex(CallTarget::Todo(a,uu));

                self.call_graph.add_edge((),new_vx,v);
                ret.push(uu);
            }
        }

        for other_fun in other_funs {
            if self.call_graph.edge(new_vx,other_fun) == None {
                self.call_graph.add_edge((),new_vx,other_fun);
            }
        }

        ret
    }

    pub fn find_call_target_by_uuid<'a>(&'a self,uu: &Uuid) -> Option<CallGraphRef> {
        for vx in self.call_graph.vertices() {
            if let Some(lb) = self.call_graph.vertex_label(vx) {
                if lb.uuid() == *uu {
                    return Some(vx);
                }
            } else {
                unreachable!();
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use function::{ControlFlowTarget,Function};
    use mnemonic::Mnemonic;
    use graph_algos::traits::{VertexListGraph,Graph,MutableGraph,AdjacencyMatrixGraph,EdgeListGraph};
    use basic_block::BasicBlock;
    use uuid::Uuid;
    use value::{Lvalue,Rvalue};
    use instr::{Operation,Instr};

    #[test]
    fn find_by_entry() {
        let mut prog = Program::new("prog_test");
        let mut func = Function::new("test2".to_string(),"ram".to_string());

        let bb0 = BasicBlock::from_vec(vec!(Mnemonic::dummy(0..10)));
        func.entry_point = Some(func.cflow_graph.add_vertex(ControlFlowTarget::Resolved(bb0)));

        prog.call_graph.add_vertex(CallTarget::Concrete(Function::new("test".to_string(),"ram".to_string())));
        let vx1 = prog.call_graph.add_vertex(CallTarget::Concrete(func));

        assert_eq!(prog.find_function_by_entry(0),Some(vx1));
        assert_eq!(prog.find_function_by_entry(1),None);
    }

    #[test]
    fn insert_replaces_todo() {
        let uu = Uuid::new_v4();
        let mut prog = Program::new("prog_test");

        let tvx = prog.call_graph.add_vertex(CallTarget::Todo(12,uu));
        let vx0 = prog.call_graph.add_vertex(CallTarget::Concrete(Function::new("test".to_string(),"ram".to_string())));
        let vx1 = prog.call_graph.add_vertex(CallTarget::Concrete(Function::new("test2".to_string(),"ram".to_string())));

        let e1 = prog.call_graph.add_edge((),tvx,vx0);
        let e2 = prog.call_graph.add_edge((),vx1,tvx);

        let mut func = Function::with_uuid("test3".to_string(),uu.clone(),"ram".to_string());
        let bb0 = BasicBlock::from_vec(vec!(Mnemonic::dummy(12..20)));
        func.entry_point = Some(func.cflow_graph.add_vertex(ControlFlowTarget::Resolved(bb0)));
        let uuf = func.uuid.clone();

        let new = prog.insert(func);

        assert_eq!(new,vec!());

        if let Some(&CallTarget::Concrete(ref f)) = prog.call_graph.vertex_label(tvx) {
            assert_eq!(f.uuid,uuf);
            assert!(f.entry_point.is_some());
        } else {
            unreachable!();
        }
        assert!(prog.call_graph.vertex_label(vx0).is_some());
        assert!(prog.call_graph.vertex_label(vx1).is_some());
        assert_eq!(prog.call_graph.edge(tvx,vx0),e1);
        assert_eq!(prog.call_graph.edge(vx1,tvx),e2);
        assert_eq!(prog.call_graph.num_edges(),2);
        assert_eq!(prog.call_graph.num_vertices(),3);
    }

    #[test]
    fn insert_ignores_new_todo() {
        let uu1 = Uuid::new_v4();
        let uu2 = Uuid::new_v4();
        let mut prog = Program::new("prog_test");

        let tvx = prog.call_graph.add_vertex(CallTarget::Todo(12,uu1));

        let mut func = Function::with_uuid("test3".to_string(),uu2.clone(),"ram".to_string());
        let ops1 = vec![];
        let i1 = vec![Instr{ op: Operation::IntCall(Rvalue::Constant(12)), assignee: Lvalue::Undefined}];
        let mne1 = Mnemonic::new(0..10,"call".to_string(),"12".to_string(),ops1.iter(),i1.iter());
        let bb0 = BasicBlock::from_vec(vec!(mne1));
        func.entry_point = Some(func.cflow_graph.add_vertex(ControlFlowTarget::Resolved(bb0)));
        let uuf = func.uuid.clone();

        let new = prog.insert(func);

        assert_eq!(new,vec!());

        if let Some(&CallTarget::Concrete(ref f)) = prog.call_graph.vertex_label(tvx) {
            assert_eq!(f.uuid,uuf);
            assert!(f.entry_point.is_some());
        }
        assert!(prog.call_graph.vertex_label(tvx).is_some());
        assert_eq!(prog.call_graph.num_edges(),1);
        assert_eq!(prog.call_graph.num_vertices(),2);
    }
}
