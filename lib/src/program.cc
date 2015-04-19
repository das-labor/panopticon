/*
 * This file is part of Panopticon (http://panopticon.re).
 * Copyright (C) 2014 Panopticon authors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#include <algorithm>
#include <unordered_set>

#include <panopticon/program.hh>
#include <panopticon/basic_block.hh>

using namespace po;
using namespace std;
using namespace boost;

program::program(const string& r, const string &n)
: name(n), reg(r), _procedures(), _calls()
{}

void program::insert(proc_loc p)
{
	insert_vertex(variant<proc_loc,symbol>(p),calls());
}

digraph<variant<proc_loc,symbol>,nullptr_t>& program::calls(void)
{
	_procedures = boost::none;
	return _calls;
}

const digraph<variant<proc_loc,symbol>,nullptr_t>& program::calls(void) const
{
	return _calls;
}

const std::unordered_set<proc_loc>& program::procedures(void) const
{
	if(!_procedures)
	{
		_procedures = std::unordered_set<proc_loc>();
		for(auto v: iters(vertices(_calls)))
		{
			auto p = get_vertex(v,_calls);
			if(get<proc_loc>(&p))
			{
				ensure(_procedures->insert(get<proc_loc>(p)).second);
			}
		}
	}
	return *_procedures;
}

template<>
program* po::unmarshal(const uuid& u, const rdf::storage &store)
{
	rdf::node n = rdf::iri(u);
	ensure(store.has(n,rdf::ns_rdf("type"),rdf::ns_po("Program")));

	rdf::statement name = store.first(n,rdf::ns_po("name"));
	rdf::statement reg = store.first(n,rdf::ns_po("region-name"));
	rdf::statements procs_n = store.find(n,rdf::ns_po("include"));

	program *ret = new program(reg.object.as_literal(),name.object.as_literal());

	for(auto st: procs_n)
		ret->insert(proc_loc{st.object.as_iri().as_uuid(),store});

	for(proc_loc p: ret->procedures())
	{
		rdf::node pn = rdf::iri(p.tag());
		rdf::statements st = store.find(pn,rdf::ns_po("calls"));
		auto vx_a = find_node<variant<proc_loc,symbol>,nullptr_t>(p,ret->_calls);

		for(auto s: st)
		{
			if(s.object.is_iri())
			{
				uuid uu = s.object.as_iri().as_uuid();
				auto i = find_if(ret->procedures().begin(),ret->procedures().end(),[&](const proc_loc q)
					{ return q.tag() == uu; });

				ensure(i != ret->procedures().end());
				auto vx_b = find_node<variant<proc_loc,symbol>,nullptr_t>(*i,ret->_calls);

				insert_edge(nullptr,vx_a,vx_b,ret->_calls);
			}
			else
			{
				symbol sym = s.object.as_literal();
				auto vx_b = insert_vertex<variant<proc_loc,symbol>,nullptr_t>(sym,ret->_calls);

				insert_edge(nullptr,vx_a,vx_b,ret->_calls);
			}
		}
	}

	return ret;
}

template<>
archive po::marshal(const program* p, const uuid& u)
{
	rdf::statements ret;
	rdf::node n = rdf::iri(u);

	ret.emplace_back(n,rdf::ns_rdf("type"),rdf::ns_po("Program"));
	ret.emplace_back(n,rdf::ns_po("name"),rdf::lit(p->name));
	ret.emplace_back(n,rdf::ns_po("region-name"),rdf::lit(p->reg));

	for(proc_loc q: p->procedures())
	{
		auto vx = find_node(variant<proc_loc,symbol>(q),p->calls());
		rdf::node m = rdf::iri(q.tag());

		ret.emplace_back(n,rdf::ns_po("include"),rdf::iri(q.tag()));

		for(auto e: iters(out_edges(vx,p->calls())))
		{
			auto wx = target(e,p->calls());
			auto v = get_vertex(wx,p->calls());

			if(get<proc_loc>(&v))
				ret.emplace_back(m,rdf::ns_po("calls"),rdf::iri(get<proc_loc>(v).tag()));
			else
				ret.emplace_back(m,rdf::ns_po("calls"),rdf::lit(get<symbol>(v)));
		}
	}

	return ret;
}

void po::call(prog_loc p, proc_loc from, proc_loc to)
{
	auto vx_a = find_node<variant<proc_loc,symbol>,nullptr_t>(from,p->calls());
	auto vx_b = find_node<variant<proc_loc,symbol>,nullptr_t>(to,p->calls());

	ensure(p->procedures().count(from) && p->procedures().count(to));
	insert_edge(nullptr,vx_a,vx_b,p.write().calls());
}

void po::call(prog_loc p, proc_loc from, const symbol& to)
{
	ensure(p->procedures().count(from));
	auto vx_a = find_node<variant<proc_loc,symbol>,nullptr_t>(from,p->calls());

	try
	{
		auto vx_b = find_node<variant<proc_loc,symbol>,nullptr_t>(to,p->calls());
		insert_edge(nullptr,vx_a,vx_b,p.write().calls());
	}
	catch(const out_of_range&)
	{
		insert_edge(nullptr,vx_a,insert_vertex(variant<proc_loc,symbol>(to),p.write().calls()),p.write().calls());
	}
}

optional<proc_loc> po::find_procedure_by_bblock(prog_loc fg, offset a)
{
	std::unordered_set<proc_loc>::const_iterator i = fg->procedures().begin();

	while(i != fg->procedures().end())
		if(find_bblock(*i,a))
			return *i;
		else
			++i;

	return boost::none;
}

optional<proc_loc> po::find_procedure_by_entry(prog_loc fg, offset a)
{
	auto maybe_ret = find_procedure_by_bblock(fg,a);

	if(maybe_ret && (*maybe_ret)->entry && (*(*maybe_ret)->entry)->area().lower() == a)
		return maybe_ret;
	else
		return boost::none;
}

std::unordered_set<offset> po::collect_calls(proc_loc proc)
{
	std::unordered_set<offset> ret;
	has_symbol_visitor<call_symbol> call_vis;

	execute(proc,[&](const instr& i)
	{
		if(boost::apply_visitor(call_vis,i.function))
		{
			std::vector<rvalue> right = operands(i);
			ensure(right.size() == 1);

			if(is_constant(right[0]))
			{
				const constant &c = to_constant(right[0]);
				ret.insert(c.content());
			}
		}
	});

	return ret;
}
