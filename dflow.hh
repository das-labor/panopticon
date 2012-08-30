#ifndef DFLOW_HH
#define DFLOW_HH

#include "cfg.hh"

namespace dflow {

set<name> set_difference(set<name> a, set<name> b);
set<name> set_union(set<name> a, set<name> b);
set<name> set_intersection(set<name> a, set<name> b);

void dominance_tree(cfg_ptr cfg);
void ssa(cfg_ptr cfg);
void liveness(cfg_ptr cfg);

};

#endif
