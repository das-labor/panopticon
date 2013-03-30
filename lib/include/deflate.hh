#ifndef DEFLATE_HH
#define DEFLATE_HH

extern "C" {
#include <redland.h>
}
#include <string>
#include <iostream>
#include <cassert>

#include <mnemonic.hh>
#include <basic_block.hh>
#include <procedure.hh>
#include <flowgraph.hh>

namespace po
{
	class deflate
	{
	public:
		deflate(const std::string &path);
		~deflate(void);

		flow_ptr flowgraph(void);

	private:
		proc_ptr procedure(librdf_node *proc);
		bblock_ptr basic_block(librdf_node *bblock);
		void mnemonic(librdf_node *mne, std::list<mnemonic> &ms);
		rvalue value(librdf_node *val) const;

		static librdf_world *rdf_world;
		static raptor_world *rap_world;
		
		static librdf_node *rdf_type;
		static librdf_node *rdf_first;
		static librdf_node *rdf_rest;
		
		static librdf_node *po_Procedure;
		static librdf_node *po_BasicBlock;
		static librdf_node *po_name;
		static librdf_node *po_calls;
		static librdf_node *po_contains;
		static librdf_node *po_entry_point;
		static librdf_node *po_begin;
		static librdf_node *po_end;
		static librdf_node *po_precedes;
		static librdf_node *po_executes;
		static librdf_node *po_opcode;
		static librdf_node *po_operands;
		static librdf_node *po_format;
		static librdf_node *po_base;
		static librdf_node *po_subscript;
		static librdf_node *po_value;
		static librdf_node *po_offset;
		static librdf_node *po_endianess;
		static librdf_node *po_bytes;
		static librdf_node *po_Undefined;
		static librdf_node *po_Variable;
		static librdf_node *po_Memory;
		static librdf_node *po_Constant;

		
		librdf_storage *storage;
		librdf_model *model;
	};
}

#endif
