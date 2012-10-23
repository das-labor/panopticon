#include "code_generator.hh"

/*
code_stream::code_stream(named_t n, anonymous_t a,block_ptr bb)
: named(n), anonymous(a), basic_block(bb)
{ }

pair<bblock_ptr,bblock_ptr> code_stream::branch(valproxy a, guard::Relation rel, valproxy b, std::function<void(cgen_ptr)> true_fn, std::function<void(cgen_ptr)> false_fn)
{
	return branch(guard_ptr(new guard(a,rel,b)),true_fn,false_fn);
}

pair<bblock_ptr,bblock_ptr> code_stream::branch(guard_ptr g, std::function<void(cgen_ptr)> true_fn, std::function<void(cgen_ptr)> false_fn)
{
	bblock_ptr true_bb(new basic_block());
	code_stream true_cs(named,anonymous,true_bb);

	true_fn(true_cs);
	assert(!true_bb->instructions().empty());
	conditional_jump(g,basic_block,true_bb);
	
	if(false_fn)
	{
		bblock_ptr false_bb(new basic_block());
		code_stream false_cs(named,anonymous,false_bb);

		false_fn(cs_false);
		assert(!false_bb->instructions().empty());
		conditional_jump(g->negation(),basic_block,false_bb);

		return make_pair(true_bb,false_bb);
	}

	// TODO connect true_bb/false_bb to something
	return make_pair(true_bb,bblock_ptr(0));
}

bblock_ptr code_stream::jump(bblock_ptr bb, valproxy a, guard::Relation rel, valproxy b)
{
	return jump(bb,guard_ptr(new guard(a,rel,b)));
}

bblock_ptr code_stream::jump(bblock_ptr bb, guard_ptr g)
{
	if(!g)
		unconditional_jump(basic_block,bb);
	else
		conditional_jump(basic_block,bb,g);
}

void indirect_jump(valproxy tgt, valproxy a, guard::Relation rel, valproxy b)
{
	return indirect_jump(tgt,guard_ptr(new guard(a,rel,b)));
}

void indirect_jump(valproxy tgt, guard_ptr g)
{
	infer_width(tgt.value);
	indirect_jump(basic_block,tgt.value,g);
}*/


