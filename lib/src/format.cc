#include <panopticon/format.hh>

po::tree<po::format> po::ipv4(po::region_loc reg)
{
	po::tree<format> ret(field{"IPv4 packet",composition{""}});
	auto root = ret.root();

	auto hdr = fields.insert(root,format{"header",integer{false,1,16}});
	fields.insert(hdr,format{"version",integer{0xe0,10}});
	fields.insert(hdr,format{"double word count",integer{0x1f,10}});
	fields.insert(root,format{"tos",integer{false,1,16}});
	fields.insert(root,format{"total length",integer{false,2,10}});
	fields.insert(root,format{"ident",integer{false,2,10}});
	auto frag = fields.insert(root,{"fragmentation",integer{false,2,16}});
	fields.insert(frag,format{"Reserved",boolean{0xf0,"","Invalid. Must be zero."});
	fields.insert(frag,format{"DF",boolean{0xd0,"Can be fragmented.","Don't fragment."});
	fields.insert(frag,format{"MF",boolean{0xc0,"No fragments.","More fragments."});
	fields.insert(frag,format{"offset",integer{false,0xbf,10}});
	fields.insert(root,format{"time to live",integer{false,1,10}});
	fields.insert(root,format{"protocol",integer{false,1,{make_pair({6},"TCP"),make_pair({17},"UDP")}}});
	fields.insert(root,format{"checksum",integer{false,2,16}});
	fields.insert(root,format{"source address",integer{false,4,16}});
	fields.insert(root,format{"destination address",integer{false,4,16}});

	return ret;
}
