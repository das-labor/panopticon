/*
 * This file is part of Panopticon (http://panopticon.re).
 * Copyright (C) 2014 Kai Michaelis
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

#include <panopticon/disassembler.hh>

#include <panopticon/amd64/traits.hh>
#include <panopticon/amd64/decode.hh>
#include <panopticon/amd64/syntax.hh>

#include <endian.h>

using namespace po;
using namespace po::amd64;
using namespace po::dsl;

namespace po
{
	namespace amd64
	{
		unsigned int next_unused = 0;
		std::vector<std::string> registers({
			"al","ah","ax","eax","rax",
			"bl","bh","bx","ebx","rbx",
			"cl","ch","cx","ecx","rcx",
			"dh","dl","dx","edx","rdx",
			"dil","di","edi","rdi",
			"sil","si","esi","rsi",
			"r4","r5","r6","r7",
			"r8","r10","r11","r12",
			"r13","r14","r15",
			"bp","bpl","ebp","rbp",
			"sp","spl","esp","rsp",
			"ip","eip","rip",
			"CS","FS","SS","DS",
			"CF","PF","AF","ZF","SF","TF","IF","DF","OF","IOPL","NT","RF","AC","VIF","VIP","VM","ID"
			});

			const variable al = variable("al",8),
								bl = variable("bl",8),
								cl = variable("cl",8),
								dl = variable("dl",8),
								ah = variable("al",8),
								bh = variable("bl",8),
								ch = variable("cl",8),
								dh = variable("dl",8),
								dil = variable("dil",8),
								sil = variable("sil",8),
								bpl = variable("bpl",8),
								spl = variable("spl",8),
								r4l = variable("r4l",8),
								r5l = variable("r5l",8),
								r6l = variable("r6l",8),
								r7l = variable("r7l",8),
								r8l = variable("r8l",8),
								r9l = variable("r9l",8),
								r10l = variable("r10l",8),
								r11l = variable("r11l",8),
								r12l = variable("r12l",8),
								r13l = variable("r13l",8),
								r14l = variable("r14l",8),
								r15l = variable("r15l",8),

								// 16 bit gp registers
								ax = variable("ax",16),
								bx = variable("bx",16),
								cx = variable("cx",16),
								dx = variable("dx",16),
								di = variable("di",16),
								si = variable("si",16),
								r4w = variable("r4w",16),
								r5w = variable("r5w",16),
								r6w = variable("r6w",16),
								r7w = variable("r7w",16),
								r8w = variable("r8w",16),
								r9w = variable("r9w",16),
								r10w = variable("r10w",16),
								r11w = variable("r11w",16),
								r12w = variable("r12w",16),
								r13w = variable("r13w",16),
								r14w = variable("r14w",16),
								r15w = variable("r15w",16),

								// 32 bit gp registers
								eax = variable("eax",32),
								ebx = variable("ebx",32),
								ecx = variable("ecx",32),
								edx = variable("edx",32),
								esi = variable("esi",32),
								edi = variable("edx",32),
								r4d = variable("r4d",32),
								r5d = variable("r5d",32),
								r6d = variable("r6d",32),
								r7d = variable("r7d",32),
								r8d = variable("r8d",32),
								r9d = variable("r9d",32),
								r10d = variable("r10d",32),
								r11d = variable("r11d",32),
								r12d = variable("r12d",32),
								r13d = variable("r13d",32),
								r14d = variable("r14d",32),
								r15d = variable("r15d",32),

								// 64 bit gp registers
								rax = variable("rax",64),
								rbx = variable("rbx",64),
								rcx = variable("rcx",64),
								rdx = variable("rdx",64),
								rdi = variable("rdi",64),
								rsi = variable("rsi",64),
								r4 = variable("r4",64),
								r5 = variable("r5",64),
								r6 = variable("r6",64),
								r7 = variable("r7",64),
								r8 = variable("r8",64),
								r9 = variable("r9",64),
								r10 = variable("r10",64),
								r11 = variable("r11",64),
								r12 = variable("r12",64),
								r13 = variable("r13",64),
								r14 = variable("r14",64),
								r15 = variable("r15",64),

								// Segment registers
								CS = variable("CS",16),
								FS = variable("FS",16),
								SS = variable("SS",16),
								DS = variable("DS",16),

								// 16 bit management registes
								ip = variable("ip",16),
								sp = variable("sp",16),
								bp = variable("bp",16),

								// 32 bit management registers
								esp = variable("esp",32),
								ebp = variable("ebp",32),
								eip = variable("eip",32),
								CF = variable("CF",1),
								PF = variable("PF",1),
								AF = variable("AF",1),
								ZF = variable("ZF",1),
								SF = variable("SF",1),
								TF = variable("TF",1),
								IF = variable("IF",1),
								DF = variable("DF",1),
								OF = variable("OF",1),
								IOPL = variable("IOPL",2),
								NT = variable("NT",1),
								RF = variable("RF",1),
								AC = variable("AC",1),
								VIF = variable("VIF",1),
								VIP = variable("VIP",1),
								VM = variable("VM",1),
								ID = variable("ID",1),

								// 64 bit management registers
								rsp = variable("rsp",64),
								rbp = variable("rbp",64),
								rip = variable("rip",64);
	}
}

template<>
lvalue po::temporary(amd64_tag)
{
	return variable("t" + std::to_string(po::amd64::next_unused++),64);
}

template<>
const std::vector<std::string> &po::registers(amd64_tag)
{
	return po::amd64::registers;
}

template<>
uint8_t po::width(std::string n, amd64_tag)
{
	ensure(n.size() >= 2);

	if(n.c_str()[0] == 'r')
		return 64;
	else if(n.c_str()[0] == 'e')
		return 32;
	else if(n.c_str()[0] == 't')
		return 64;
	else if(n.c_str()[1] == 'l' || n.c_str()[1] == 'h')
		return 8;
	else if(n.c_str()[1] == 'x')
		return 16;
	else
		ensure(false);
}

namespace pls = std::placeholders;


