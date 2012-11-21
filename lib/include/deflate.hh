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
		deflate(::std::string &path);
		~deflate(void);

		flow_ptr flowgraph(void);

	private:
		proc_ptr procedure(librdf_node *proc);
		bblock_ptr basic_block(librdf_node *bblock);
		void mnemonic(librdf_node *mne, ::std::list<mnemonic> &ms);

		static librdf_world *rdf_world;
		static raptor_world *rap_world;
		static librdf_node *rdf_type;
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

		librdf_storage *storage;
		librdf_model *model;
	};
}

#endif
