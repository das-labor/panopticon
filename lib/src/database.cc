#include <functional>
#include <map>

#include <database.hh>

#define RDF_NS(x) (const unsigned char *)"http://www.w3.org/1999/02/22-rdf-syntax-ns#" x
#define PANOPTICUM_NS(x) (const unsigned char *)"http://panopticum.io/rdf/v1/rdf#" x

librdf_world *database::rdf_world = 0;
raptor_world *database::rap_world = 0;
librdf_node *database::rdf_type = 0;
librdf_node *database::po_Procedure = 0;
librdf_node *database::po_BasicBlock = 0;
librdf_node *database::po_name = 0;
librdf_node *database::po_calls = 0;
librdf_node *database::po_contains = 0;
librdf_node *database::po_entry_point = 0;
librdf_node *database::po_begin = 0;
librdf_node *database::po_end = 0;
librdf_node *database::po_precedes = 0;

database::database(std::string &path)
{
	if(!rdf_world || !rap_world)
	{
		rdf_world = librdf_new_world();
		librdf_world_open(rdf_world);
		
		rap_world = librdf_world_get_raptor(rdf_world);
		rdf_type = librdf_new_node_from_uri_string(rdf_world,RDF_NS("type"));
		po_Procedure = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("Procedure"));
		po_BasicBlock = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("BasicBlock"));
		po_name = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("name"));
		po_calls = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("calls"));
		po_contains = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("contains"));
		po_entry_point = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("entry_point"));
		po_begin = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("begin"));
		po_end = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("end"));
		po_precedes = librdf_new_node_from_uri_string(rdf_world,PANOPTICUM_NS("precedes"));
	}

	assert(storage = librdf_new_storage(rdf_world,"memory",NULL,NULL));
	assert(model = librdf_new_model(rdf_world,storage,NULL));

	librdf_parser *parser;
	librdf_uri *uri;
	
	assert(parser = librdf_new_parser(rdf_world,"turtle",NULL,NULL));
	assert(uri = librdf_new_uri_from_filename(rdf_world,path.c_str()));
	assert(!librdf_parser_parse_into_model(parser,uri,uri,model));

	std::cout << librdf_model_size(model) << " triples in " << path << std::endl;	
	
	librdf_free_uri(uri);
	librdf_free_parser(parser);
}

flow_ptr database::flowgraph(void)
{
	librdf_iterator *i;
	std::map<librdf_node *,proc_ptr> procedures;
	
	assert(i = librdf_model_get_sources(model,rdf_type,po_Procedure));

	while(!librdf_iterator_end(i))
	{
		librdf_node *proc = (librdf_node *)librdf_iterator_get_object(i);
		proc_ptr p = procedure(proc);
			
		procedures.insert(std::make_pair(librdf_new_node_from_node(proc),p));
		librdf_iterator_next(i);
	}

	librdf_free_iterator(i);

	std::for_each(procedures.begin(),procedures.end(),[&](std::pair<librdf_node *,proc_ptr> p)
	{
		assert(i = librdf_model_get_targets(model,p.first,po_calls));
		
		while(!librdf_iterator_end(i))
		{
			librdf_node *calls = (librdf_node *)librdf_iterator_get_object(i);
			
			if(calls)
			{
				auto j = std::find_if(procedures.begin(),procedures.end(),[&](const std::pair<librdf_node *,proc_ptr> q) 
					{ return librdf_node_equals(calls,q.first); });

				assert(j != procedures.end());
				proc_ptr other = j->second;
				p.second->callees.push_back(other);
			}
			
			librdf_iterator_next(i);
		}
		librdf_free_iterator(i);
	});

	flow_ptr flow(new ::flowgraph());	
	
	std::for_each(procedures.begin(),procedures.end(),[&](std::pair<librdf_node *,proc_ptr> p)
	{ 
		librdf_free_node(p.first); 
		flow->procedures.insert(p.second);
	});

	return flow;
}

proc_ptr database::procedure(librdf_node *proc)
{
	librdf_node *entry = librdf_model_get_target(model,proc,po_entry_point);
	librdf_node *name = librdf_model_get_target(model,proc,po_name);
	librdf_iterator *bblocks = librdf_model_get_targets(model,proc,po_contains);
	proc_ptr ret(new ::procedure());
	std::list<std::pair<librdf_node *,bblock_ptr>> bblock_list;

	// deserialize procedure properties: name
	assert(bblocks && entry && name && librdf_node_is_literal(name));
	ret->name = std::string((const char *)librdf_node_get_literal_value(name));

	// deserialize bblocks
	while(!librdf_iterator_end(bblocks))
	{
		librdf_node *bblock = (librdf_node *)librdf_iterator_get_object(bblocks);
		librdf_node *begin = librdf_model_get_target(model,bblock,po_begin);
		librdf_node *end = librdf_model_get_target(model,bblock,po_end);

		assert(begin && end && librdf_node_is_literal(begin) && librdf_node_is_literal(end));
		addr_t b = strtoull((const char *)librdf_node_get_literal_value(begin),NULL,10);
		addr_t e = strtoull((const char *)librdf_node_get_literal_value(end),NULL,10);
		
		librdf_free_node(begin);
		librdf_free_node(end);

		bblock_ptr bb(new ::basic_block(range<addr_t>(b,e)));
		ret->insert_bblock(bb);
		if(librdf_node_equals(bblock,entry))
			ret->entry = bb;

		bblock_list.push_back(std::make_pair(librdf_new_node_from_node(bblock),bb));
		librdf_iterator_next(bblocks);
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
		auto i = find_if(bblock_list.begin(),bblock_list.end(),[&](const std::pair<librdf_node *,bblock_ptr> &p)
			{ return librdf_node_equals(p.first,bblock); });
		
		assert(i != bblock_list.end());
		bb = i->second;

		while(!librdf_iterator_end(predecessors))
		{
			librdf_node *pred = (librdf_node *)librdf_iterator_get_object(predecessors);
			bblock_ptr o;
			auto j = find_if(bblock_list.begin(),bblock_list.end(),[&](const std::pair<librdf_node *,bblock_ptr> &p)
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
	for_each(bblock_list.begin(),bblock_list.end(),[&](const std::pair<librdf_node *,bblock_ptr> &p)
		{ librdf_free_node(p.first); });

	return ret;
}

database::~database(void)
{
	librdf_free_model(model);
	librdf_free_storage(storage);
}	
