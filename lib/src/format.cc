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
