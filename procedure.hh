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
	
	pair<iterator,iterator> all(void);
	//pair<iter,iter> rev_postorder(void) const;
	bblock_ptr entry(void);

protected:
	list<bblock_ptr> basic_blocks;	// in rpo

	//stash<bblock_ptr> bblock_map;

	// dflow::liveness
	//set<name> names;									// global (procedure-wide) names
	//map<name,set<bblock_ptr>> usage;	// maps names to blocks that use them
};

bblock_ptr find_bblock(proc_ptr proc, addr_t a);
pair<bool,bblock_ptr> extend_procedure(proc_ptr proc, const mne_cptr cur_mne, const mne_cptr prev_mne, bblock_ptr prev_bb);

#endif
