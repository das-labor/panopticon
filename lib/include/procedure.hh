#ifndef PROCEDURE_HH
#define PROCEDURE_HH

#include <memory>
#include <set>
#include <map>
#include <list>

#include "basic_block.hh"
#include "mnemonic.hh"

using namespace std;

typedef shared_ptr<struct domtree> dtree_ptr;
typedef shared_ptr<struct procedure> proc_ptr;

struct domtree
{
	domtree(bblock_ptr b);

	dtree_ptr intermediate;			// e.g. parent
	set<dtree_ptr> successors;
	set<dtree_ptr> frontiers;
	
	bblock_ptr basic_block;
};

class procedure
{
public:
	typedef list<bblock_ptr>::iterator iterator;

	procedure(void);
	procedure(list<bblock_ptr> &e);

	void insert_bblock(bblock_ptr m);
	void remove_bblock(bblock_ptr m);
	
	pair<iterator,iterator> rev_postorder(void);
	
	bblock_ptr entry;
	string name;
	vector<proc_ptr> callees;
	list<bblock_ptr> basic_blocks;	

protected:
	list<bblock_ptr> rpo;
};

void merge(proc_ptr proc, bblock_ptr block);
void extend(proc_ptr proc, bblock_ptr block);
bblock_ptr find_bblock(proc_ptr proc, addr_t a);
pair<bool,bblock_ptr> extend_procedure(proc_ptr proc, const mne_cptr cur_mne, const mne_cptr prev_mne, bblock_ptr prev_bb, guard_ptr g);
pair<bool,bblock_ptr> extend_procedure(proc_ptr proc, const mne_cptr cur_mne, bblock_ptr cur_bb, value_ptr v, guard_ptr g);
string graphviz(proc_ptr proc);

#endif
