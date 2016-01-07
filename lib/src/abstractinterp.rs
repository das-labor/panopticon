/// Models both under- and overapproximation
trait AbstractDomain {
    type Value: Clone + PartialEq + Hash + Debug;

    fn abstraction(&Rvalue) -> Value;
    fn execute(&Operation,&HashMap<Value,Rvalue>) -> Value;
    fn combine(&Value,&Value) -> Value;
    fn widen(&Value,&Value) -> Value;
    fn more_exact(&Value,&Value) -> bool;
    fn initial() -> Value;
}

fn approximate<A: AbstractDomain>(func: &Function) -> HashMap<Rvalue,A::Value> {
    let rpo = {
        let ret = func.postorder();
        ret.reverse();
        ret
    };
    let mut fixpoint = false;
    let mut ret = HashMap::new();

    while !fixpoint {
        fixpoint = true;

        for v in rpo.iter() {
            if let Some(&ControlFlowTarget::Resolved(ref bb)) = func.cflow_graph.vertex_label(v) {
                bb.execute(|i| {
                    let new = match i {
                        Instr{ op: Operation::Phi(ref ops), ref assignee } =>
                            match ops.size() {
                                0 => panic!("Phi function w/o arguments"),
                                1 => A::abstraction(ops[0]),
                                _ => ops.iter().fold(A::initial(),A::combine),
                            },
                        Instr{ ref op, ref assignee } => A::execute(op,ret),
                    };
                    let cur = ret.entry(assignee).or_insert(A::initial());

                    if A::more_exact(v,cur) {
                        fixpoint = false;
                        ret.insert(assignee,v);
                    }
                });
            }
        }
    }

    ret
}

impl KSet for AbstractDomain {
    // None -> Top, Some(vec![]) -> Bot
    type Value = Option<Vec<Rvalue>>;

    fn abstraction(v: &Rvalue) -> Value {
        Some(vec![v.clone()])
    }

    fn execute(op: &Operation, env: &HashMap<Rvalue,Rvalue>) -> Value {
        let mut ret = HashSet::new();
    }

    fn combine(a: &Value, b: &Value) -> Value {
    }

    fn widen(a: &Value, b: &Value) -> Value {
    }

    fn initial() -> Value {
        Some(vec![]);
    }
}
