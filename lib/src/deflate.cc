#include <functional>
#include <map>
#include <cstring>

#include <deflate.hh>

using namespace po;
using namespace std;

#define RDF_NS(x) (const unsigned char *)"http://www.w3.org/1999/02/22-rdf-syntax-ns#" x
#define PANOPTICUM_NS(x) (const unsigned char *)"http://panopticum.io/rdf/v1/rdf#" x

librdf_world *deflate::rdf_world = 0;
raptor_world *deflate::rap_world = 0;
librdf_node *deflate::rdf_type = 0;
librdf_node *deflate::rdf_first = 0;
librdf_node *deflate::rdf_rest = 0;
librdf_node *deflate::po_Procedure = 0;
librdf_node *deflate::po_BasicBlock = 0;
librdf_node *deflate::po_name = 0;
librdf_node *deflate::po_calls = 0;
librdf_node *deflate::po_contains = 0;
librdf_node *deflate::po_entry_point = 0;
librdf_node *deflate::po_begin = 0;
librdf_node *deflate::po_end = 0;
librdf_node *deflate::po_precedes = 0;
librdf_node *deflate::po_executes = 0;
librdf_node *deflate::po_opcode = 0;
librdf_node *deflate::po_operands = 0;
librdf_node *deflate::po_format = 0;
librdf_node *deflate::po_base = 0;
librdf_node *deflate::po_subscript = 0;
librdf_node *deflate::po_bytes = 0;
librdf_node *deflate::po_endianess = 0;
librdf_node *deflate::po_value = 0;
librdf_node *deflate::po_offset = 0;
librdf_node *deflate::po_Constant = 0;
librdf_node *deflate::po_Memory = 0;
librdf_node *deflate::po_Variable = 0;
librdf_node *deflate::po_Undefined = 0;

deflate::deflate(const string &path)
{
	if(!rdf_world || !rap_world)
	{
		rdf_world = librdf_new_world();
		librdf_world_open(rdf_world);
		
		rap_world = librdf_world_get_raptor(rdf_world);
		rdf_type = librdf_new_node_from_uri_string(rdf_world,RDF_NS("type"));
		rdf_first = librdf_new_node_from_uri_string(rdf_world,RDF_NS("first"));
		rdf_rest = librdf_new_node_from_uri_string(rdf_world,RDF_NS("rest"));
		po_Procedure = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("Procedure"));
		po_BasicBlock = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("BasicBlock"));
		po_name = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("name"));
		po_calls = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("calls"));
		po_contains = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("contains"));
		po_entry_point = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("entry_point"));
		po_begin = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("begin"));
		po_end = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("end"));
		po_precedes = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("precedes"));
		po_executes = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("executes"));
		po_opcode = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("opcode"));
		po_operands = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("operands"));
		po_format = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("format"));
		po_base = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("base"));
		po_subscript = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("subscript"));
		po_offset = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("offset"));
		po_bytes = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("bytes"));
		po_value = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("value"));
		po_endianess = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("endianess"));
		po_Constant = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("Constant"));
		po_Memory = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("Memory"));
		po_Variable = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("Variable"));
		po_Undefined = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("Undefined"));
	}

	assert(storage = librdf_new_storage(rdf_world,"memory",NULL,NULL));
	assert(model = librdf_new_model(rdf_world,storage,NULL));

	librdf_parser *parser;
	librdf_uri *uri;
	
	assert(parser = librdf_new_parser(rdf_world,"turtle",NULL,NULL));
	assert(uri = librdf_new_uri_from_filename(rdf_world,path.c_str()));
	assert(!librdf_parser_parse_into_model(parser,uri,uri,model));

	cout << librdf_model_size(model) << " triples in " << path << endl;	
	
	librdf_free_uri(uri);
	librdf_free_parser(parser);
}

flow_ptr deflate::flowgraph(void)
{
	librdf_iterator *i;
	map<librdf_node *,proc_ptr> procedures;
	
	assert(i = librdf_model_get_sources(model,rdf_type,po_Procedure));

	while(!librdf_iterator_end(i))
	{
		librdf_node *proc = (librdf_node *)librdf_iterator_get_object(i);
		proc_ptr p = procedure(proc);
			
		procedures.insert(make_pair(librdf_new_node_from_node(proc),p));
		librdf_iterator_next(i);
	}

	librdf_free_iterator(i);

	for_each(procedures.begin(),procedures.end(),[&](pair<librdf_node *,proc_ptr> p)
	{
		assert(i = librdf_model_get_targets(model,p.first,po_calls));
		
		while(!librdf_iterator_end(i))
		{
			librdf_node *calls = (librdf_node *)librdf_iterator_get_object(i);
			
			if(calls)
			{
				auto j = find_if(procedures.begin(),procedures.end(),[&](const pair<librdf_node *,proc_ptr> q) 
					{ return librdf_node_equals(calls,q.first); });

				assert(j != procedures.end());
				proc_ptr other = j->second;
				call(p.second,other);
			}
			
			librdf_iterator_next(i);
		}
		librdf_free_iterator(i);
	});

	flow_ptr flow(new ::flowgraph());	
	
	for_each(procedures.begin(),procedures.end(),[&](pair<librdf_node *,proc_ptr> p)
	{ 
		librdf_free_node(p.first); 
		flow->procedures.insert(p.second);
	});

	return flow;
}

proc_ptr deflate::procedure(librdf_node *proc)
{
	librdf_node *entry = librdf_model_get_target(model,proc,po_entry_point);
	librdf_node *name = librdf_model_get_target(model,proc,po_name);
	librdf_iterator *bblocks = librdf_model_get_targets(model,proc,po_contains);
	proc_ptr ret(new ::procedure());
	list<pair<librdf_node *,bblock_ptr>> bblock_list;

	// deserialize procedure properties: name
	assert(bblocks && entry && name && librdf_node_is_literal(name));
	ret->name = string((const char *)librdf_node_get_literal_value(name));

	// deserialize bblocks
	while(!librdf_iterator_end(bblocks))
	{
		librdf_node *bblock = (librdf_node *)librdf_iterator_get_object(bblocks);
		librdf_iterator *mnemonics = librdf_model_get_targets(model,bblock,po_executes);
		list< ::mnemonic> ms;
		bblock_ptr bb(new ::basic_block());

		assert(mnemonics);
		
		ret->basic_blocks.insert(bb);
		if(librdf_node_equals(bblock,entry))
			ret->entry = bb;

		while(!librdf_iterator_end(mnemonics))
		{
			librdf_node *mne = (librdf_node *)librdf_iterator_get_object(mnemonics);

			mnemonic(mne,ms);
			librdf_iterator_next(mnemonics);
		}

		ms.sort([](const ::mnemonic &a, const ::mnemonic &b)
			{ return a.area < b.area; });

		bb->mutate_mnemonics([&](vector< ::mnemonic> &m)
			{ move(ms.begin(),ms.end(),inserter(m,m.end())); });

		bblock_list.push_back(make_pair(librdf_new_node_from_node(bblock),bb));
		librdf_iterator_next(bblocks);
		librdf_free_iterator(mnemonics);
	}
	
	librdf_free_node(entry);
	librdf_free_node(name);
	librdf_free_iterator(bblocks);

	// resolve po:precedes
	bblocks = librdf_model_get_targets(model,proc,po_contains);

	while(!librdf_iterator_end(bblocks))
	{
		librdf_node *bblock = (librdf_node *)librdf_iterator_get_object(bblocks);
		librdf_iterator *predecessors = librdf_model_get_targets(model,bblock,po_precedes);
		bblock_ptr bb;
		auto i = find_if(bblock_list.begin(),bblock_list.end(),[&](const pair<librdf_node *,bblock_ptr> &p)
			{ return librdf_node_equals(p.first,bblock); });
		
		assert(i != bblock_list.end());
		bb = i->second;

		while(!librdf_iterator_end(predecessors))
		{
			librdf_node *pred = (librdf_node *)librdf_iterator_get_object(predecessors);
			bblock_ptr o;
			auto j = find_if(bblock_list.begin(),bblock_list.end(),[&](const pair<librdf_node *,bblock_ptr> &p)
				{ return librdf_node_equals(p.first,pred); });
		
			assert(j != bblock_list.end());
			o = j->second;

			unconditional_jump(bb,o);
			librdf_iterator_next(predecessors);
		}

		librdf_free_iterator(predecessors);
		librdf_iterator_next(bblocks);
	}
		
	librdf_free_iterator(bblocks);
	for(const pair<librdf_node *,bblock_ptr> &p: bblock_list)
		librdf_free_node(p.first);

	return ret;
}

void deflate::mnemonic(librdf_node *mne, list< ::mnemonic> &ms)
{
	librdf_node *begin = librdf_model_get_target(model,mne,po_begin);
	librdf_node *end = librdf_model_get_target(model,mne,po_end);
	librdf_node *opcode = librdf_model_get_target(model,mne,po_opcode);
	librdf_node *ops = librdf_model_get_target(model,mne,po_operands);
	librdf_node *fmt = librdf_model_get_target(model,mne,po_format);

	assert(begin && end && opcode && librdf_node_is_literal(begin) && librdf_node_is_literal(end) && librdf_node_is_literal(opcode));
	addr_t b = strtoull((const char *)librdf_node_get_literal_value(begin),NULL,10);
	addr_t e = strtoull((const char *)librdf_node_get_literal_value(end),NULL,10);
	string oc((const char *)librdf_node_get_literal_value(opcode));
	string f((const char *)librdf_node_get_literal_value(fmt));

	ms.emplace_back(::mnemonic(range<addr_t>(b,e),oc,f,{},{}));
	while(ops)
	{
		librdf_node *val = librdf_model_get_target(model,ops,rdf_first);
		librdf_node *tmp = librdf_model_get_target(model,ops,rdf_rest);

		if(val)
		{
			ms.back().operands.emplace_back(value(val));
			librdf_free_node(val);
		}

		librdf_free_node(ops);
		ops = tmp;
	}

	librdf_free_node(begin);
	librdf_free_node(end);
	librdf_free_node(opcode);
	librdf_free_node(fmt);
}

rvalue deflate::value(librdf_node *val) const
{
	librdf_node *type = librdf_model_get_target(model,val,rdf_type);

	if(librdf_node_equals(type,po_Undefined))	
	{
		return undefined();
	}
	else if(librdf_node_equals(type,po_Variable))
	{
		librdf_node *base = librdf_model_get_target(model,val,po_base);
		librdf_node *subscript = librdf_model_get_target(model,val,po_subscript);
		
		assert(base && subscript);
		return variable((const char *)librdf_node_get_literal_value(base),strtoull((const char *)librdf_node_get_literal_value(subscript),NULL,10));
	}
	else if(librdf_node_equals(type,po_Constant))
	{
		librdf_node *value = librdf_model_get_target(model,val,po_value);

		assert(value);
		unsigned long long v = strtoull((const char *)librdf_node_get_literal_value(value),NULL,10);
		return constant(v,flsll(v));
	}
	else if(librdf_node_equals(type,po_Memory))
	{
		librdf_node *offset = librdf_model_get_target(model,val,po_offset);
		librdf_node *bytes = librdf_model_get_target(model,val,po_bytes);
		librdf_node *name = librdf_model_get_target(model,val,po_name);
		librdf_node *endianess = librdf_model_get_target(model,val,po_endianess);

		assert(offset && bytes && name && endianess);
		rvalue o = value(offset);
		string n((const char *)librdf_node_get_literal_value(name));
		string e((const char *)librdf_node_get_literal_value(endianess));
		unsigned long long b = strtoull((const char *)librdf_node_get_literal_value(bytes),NULL,10);

		assert((e == "little" || e == "big") && b && n.size());
		return memory(o,b,e == "little" ? memory::LittleEndian : memory::BigEndian,n);
	}
	else
		assert(false);
}

deflate::~deflate(void)
{
	librdf_free_model(model);
	librdf_free_storage(storage);
}	
