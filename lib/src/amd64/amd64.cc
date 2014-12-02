#include <panopticon/disassembler.hh>

#include <panopticon/amd64/amd64.hh>
#include <panopticon/amd64/util.hh>

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
			"bpl","ebp","rbp",
			"spl","esp","rsp",
			"eip","rip",
				"eflags","rflags"
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
							rip = variable("rip",64),
							rflags = variable("rflags",64);
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

boost::optional<prog_loc> po::amd64::disassemble(boost::optional<prog_loc> prog, po::slab bytes, const po::ref& r)
{
	disassembler<amd64_tag> main, opsize_prfix, rex_prfix, rexw_prfix,
									generic_prfx, addrsize_prfx, rep_prfx,
									imm8, imm16, imm32, imm64,
									sib,
									rm8, rm16, rm32, rm64,
									rm8_0, rm16_0, rm32_0, rm64_0,
									rm8_1, rm16_1, rm32_1, rm64_1,
									rm8_2, rm16_2, rm32_2, rm64_2,
									rm8_3, rm16_3, rm32_3, rm64_3,
									rm8_4, rm16_4, rm32_4, rm64_4,
									rm8_5, rm16_5, rm32_5, rm64_5,
									rm8_6, rm16_6, rm32_6, rm64_6,
									rm8_7, rm16_7, rm32_7, rm64_7,
									disp8, disp16, disp32, disp64;

	opsize_prfix[ 0x66 ] = [](sm& st)
	{
		switch(st.state.mode)
		{
			case amd64_state::RealMode:		st.state.op_sz = amd64_state::OpSz_32; break;
			case amd64_state::ProtectedMode:	st.state.op_sz = amd64_state::OpSz_16; break; // assumes CS.d == 1
			case amd64_state::LongMode:		st.state.op_sz = amd64_state::OpSz_16; break;
			default: ensure(false);
		}
	};

	addrsize_prfx[ 0x67 ] = [](sm& st)
	{
		switch(st.state.mode)
		{
			case amd64_state::RealMode:		st.state.addr_sz = amd64_state::AddrSz_32; break;
			case amd64_state::ProtectedMode:	st.state.addr_sz = amd64_state::AddrSz_16; break; // assumes CS.d == 1
			case amd64_state::LongMode:		st.state.addr_sz = amd64_state::AddrSz_32; break;
			default: ensure(false);
		}
	};

	rep_prfx[ 0xf3 ] = [](sm& st) {};

	rex_prfix [ "0100 w@0 r@. x@. b@."_e ] = [](sm& st) { st.state.rex = true; };
	rexw_prfix[ "0100 w@1 r@. x@. b@."_e ] = [](sm& st) { st.state.rex = true; st.state.op_sz = amd64_state::OpSz_64; };

	generic_prfx[ rep_prfx														 ] = [](sm& st) {};
	generic_prfx[ *addrsize_prfx >> *opsize_prfix >> *addrsize_prfx ] = [](sm& st)
	{
		switch(st.state.mode)
		{
			case amd64_state::RealMode:		st.state.addr_sz = amd64_state::AddrSz_32; break;
			case amd64_state::ProtectedMode:	st.state.addr_sz = amd64_state::AddrSz_16; break; // assumes CS.d == 1
			case amd64_state::LongMode:		st.state.addr_sz = amd64_state::AddrSz_32; break;
			default: ensure(false);
		}
	};

	imm8 [ "imm@........"_e] = [](sm& st)
	{
		st.state.imm = constant(st.capture_groups.at("imm"));
	};
	imm16[ imm8 >> "imm@........"_e] = [](sm& st)
	{
		st.state.imm = constant(be16toh(st.capture_groups.at("imm")));
	};
	imm32[ imm16 >> "imm@........"_e >> "imm@........"_e] = [](sm& st)
	{
		st.state.imm = constant(be32toh(st.capture_groups.at("imm")));
	};
	imm64[ imm32 >> "imm@........"_e >> "imm@........"_e >> "imm@........"_e >> "imm@........"_e] = [](sm& st)
	{
		st.state.imm = constant(be64toh(st.capture_groups.at("imm")));
	};

	disp8 [ "disp@........"_e] = [](sm& st)
	{
		st.state.disp = constant(st.capture_groups.at("disp"));
	};
	disp16[ disp8 >> "disp@........"_e] = [](sm& st)
	{
		st.state.disp = constant(be16toh(st.capture_groups.at("disp")));
	};
	disp32[ disp16 >> "disp@........"_e >> "disp@........"_e] = [](sm& st)
	{
		st.state.disp = constant(be32toh(st.capture_groups.at("disp")));
	};
	disp64[ disp32 >> "disp@........"_e >> "disp@........"_e >> "disp@........"_e >> "disp@........"_e] = [](sm& st)
	{
		st.state.disp = constant(be64toh(st.capture_groups.at("disp")));
	};

	// sib
	sib [ "scale@00 index@... base@..."_e >> "disp@........"_e >> "disp@........"_e >> "disp@........"_e >> "disp@........"_e	] = [](sm& st)
	{
		st.state.disp = constant(be32toh(st.capture_groups.at("disp")));
	};
	sib [ "scale@01 index@... base@..."_e >> "sib@........"_e																			] = [](sm& st) {};
	sib [ "scale@10 index@... base@..."_e >> "disp@........"_e >> "disp@........"_e >> "disp@........"_e >> "disp@........"_e	] = [](sm& st)
	{
		st.state.disp = constant(be32toh(st.capture_groups.at("disp")));
	};
	sib [ "scale@.. index@... base@..."_e] = [](sm& st) {};

	std::function<void(sm&)> rm8_func = [&](sm& st)
	{
		ensure(!st.state.reg && !st.state.rm);

		if(st.capture_groups.count("reg"))
			st.state.reg = decode_reg8((1 << 3) * (st.capture_groups.count("r") && st.capture_groups.at("r")) + st.capture_groups.at("reg"),st.state.rex);

		boost::optional<std::tuple<unsigned int,unsigned int,unsigned int>> sib = boost::none;
		unsigned int b_rm = (1 << 3) * (st.capture_groups.count("b") && st.capture_groups.at("b")) + st.capture_groups.at("rm");

		if(st.capture_groups.count("scale") && st.capture_groups.count("index") && st.capture_groups.count("base"))
		{
			unsigned int scale = st.capture_groups.at("scale");
			unsigned int x_index = (st.state.rex ? (1 << 3) * st.capture_groups.count("x") : 0) + st.capture_groups.at("index");
			unsigned int b_base = (st.state.rex ? (1 << 3) * st.capture_groups.count("b") : 0) + st.capture_groups.at("base");

			sib = std::make_tuple(scale,x_index,b_base);
		}

		st.mnemonic(0,"internal-rm8","",[&](cg& c) -> std::list<rvalue>
		{
			st.state.rm = decode_modrm(st.capture_groups.at("mod"),b_rm,st.state.disp,sib,amd64_state::OpSz_8,st.state.addr_sz,c);
			return {};
		});
	};

	std::function<void(sm&)> rm16_func = [&](sm& st)
	{
		ensure(!st.state.reg && !st.state.rm);

		if(st.capture_groups.count("reg"))
			st.state.reg = decode_reg16((1 << 3) * (st.capture_groups.count("r") && st.capture_groups.at("r")) + st.capture_groups.at("reg"));

		boost::optional<std::tuple<unsigned int,unsigned int,unsigned int>> sib = boost::none;
		unsigned int b_rm = (1 << 3) * (st.capture_groups.count("b") && st.capture_groups.at("b")) + st.capture_groups.at("rm");

		if(st.capture_groups.count("scale") && st.capture_groups.count("index") && st.capture_groups.count("base"))
		{
			unsigned int scale = st.capture_groups.at("scale");
			unsigned int x_index = (st.state.rex ? (1 << 3) * st.capture_groups.count("x") : 0) + st.capture_groups.at("index");
			unsigned int b_base = (st.state.rex ? (1 << 3) * st.capture_groups.count("b") : 0) + st.capture_groups.at("base");

			sib = std::make_tuple(scale,x_index,b_base);
		}

		st.mnemonic(0,"internal-rm16","",[&](cg& c) -> std::list<rvalue>
		{
			st.state.rm = decode_modrm(st.capture_groups.at("mod"),b_rm,st.state.disp,sib,amd64_state::OpSz_16,st.state.addr_sz,c);
			return {};
		});
	};

	std::function<void(sm&)> rm32_func = [&](sm& st)
	{
		ensure(!st.state.reg && !st.state.rm);

		if(st.capture_groups.count("reg"))
			st.state.reg = decode_reg32((1 << 3) * (st.capture_groups.count("r") && st.capture_groups.at("r")) + st.capture_groups.at("reg"));

		boost::optional<std::tuple<unsigned int,unsigned int,unsigned int>> sib = boost::none;
		unsigned int b_rm = (1 << 3) * (st.capture_groups.count("b") && st.capture_groups.at("b")) + st.capture_groups.at("rm");

		if(st.capture_groups.count("scale") && st.capture_groups.count("index") && st.capture_groups.count("base"))
		{
			unsigned int scale = st.capture_groups.at("scale");
			unsigned int x_index = (st.state.rex ? (1 << 3) * st.capture_groups.count("x") : 0) + st.capture_groups.at("index");
			unsigned int b_base = (st.state.rex ? (1 << 3) * st.capture_groups.count("b") : 0) + st.capture_groups.at("base");

			sib = std::make_tuple(scale,x_index,b_base);
		}

		st.mnemonic(0,"internal-rm32","",[&](cg& c) -> std::list<rvalue>
		{
			st.state.rm = decode_modrm(st.capture_groups.at("mod"),b_rm,st.state.disp,sib,amd64_state::OpSz_32,st.state.addr_sz,c);
			return {};
		});
	};

	std::function<void(sm&)> rm64_func = [&](sm& st)
	{
		ensure(!st.state.reg && !st.state.rm);

		if(st.capture_groups.count("reg"))
			st.state.reg = decode_reg64((1 << 3) * (st.capture_groups.count("r") && st.capture_groups.at("r")) + st.capture_groups.at("reg"));

		boost::optional<std::tuple<unsigned int,unsigned int,unsigned int>> sib = boost::none;
		unsigned int b_rm = (1 << 3) * (st.capture_groups.count("b") && st.capture_groups.at("b")) + st.capture_groups.at("rm");

		if(st.capture_groups.count("scale") && st.capture_groups.count("index") && st.capture_groups.count("base"))
		{
			unsigned int scale = st.capture_groups.at("scale");
			unsigned int x_index = (st.state.rex ? (1 << 3) * st.capture_groups.count("x") : 0) + st.capture_groups.at("index");
			unsigned int b_base = (st.state.rex ? (1 << 3) * st.capture_groups.count("b") : 0) + st.capture_groups.at("base");

			sib = std::make_tuple(scale,x_index,b_base);
		}

		st.mnemonic(0,"internal-rm64","",[&](cg& c) -> std::list<rvalue>
		{
			st.state.rm = decode_modrm(st.capture_groups.at("mod"),b_rm,st.state.disp,sib,amd64_state::OpSz_64,st.state.addr_sz,c);
			return {};
		});
	};

	// mod = 00
	rm8 [ "mod@00 reg@... rm@101"_e >> disp32	] = rm8_func;
	rm8 [ "mod@00 reg@... rm@100"_e >> sib		] = rm8_func;
	rm8 [ "mod@00 reg@... rm@..."_e				] = rm8_func;
	rm16[ "mod@00 reg@... rm@101"_e >> disp32	] = rm16_func;
	rm16[ "mod@00 reg@... rm@100"_e >> sib		] = rm16_func;
	rm16[ "mod@00 reg@... rm@..."_e				] = rm16_func;
	rm32[ "mod@00 reg@... rm@101"_e >> disp32	] = rm32_func;
	rm32[ "mod@00 reg@... rm@100"_e >> sib		] = rm32_func;
	rm32[ "mod@00 reg@... rm@..."_e				] = rm32_func;
	rm64[ "mod@00 reg@... rm@101"_e >> disp32	] = rm64_func;
	rm64[ "mod@00 reg@... rm@100"_e >> sib		] = rm64_func;
	rm64[ "mod@00 reg@... rm@..."_e				] = rm64_func;

	// mod = 00 w/ extension opcode
	rm8_0 [ "mod@00 000 rm@101"_e >> disp32	] = rm8_func;
	rm8_0 [ "mod@00 000 rm@100"_e >> sib		] = rm8_func;
	rm8_0 [ "mod@00 000 rm@..."_e					] = rm8_func;
	rm16_0[ "mod@00 000 rm@101"_e >> disp32	] = rm16_func;
	rm16_0[ "mod@00 000 rm@100"_e >> sib		] = rm16_func;
	rm16_0[ "mod@00 000 rm@..."_e					] = rm16_func;
	rm32_0[ "mod@00 000 rm@101"_e >> disp32	] = rm32_func;
	rm32_0[ "mod@00 000 rm@100"_e >> sib		] = rm32_func;
	rm32_0[ "mod@00 000 rm@..."_e					] = rm32_func;
	rm64_0[ "mod@00 000 rm@101"_e >> disp32	] = rm64_func;
	rm64_0[ "mod@00 000 rm@100"_e >> sib		] = rm64_func;
	rm64_0[ "mod@00 000 rm@..."_e					] = rm64_func;

	rm8_1 [ "mod@00 001 rm@101"_e >> disp32	] = rm8_func;
	rm8_1 [ "mod@00 001 rm@100"_e >> sib		] = rm8_func;
	rm8_1 [ "mod@00 001 rm@..."_e					] = rm8_func;
	rm16_1[ "mod@00 001 rm@101"_e >> disp32	] = rm16_func;
	rm16_1[ "mod@00 001 rm@100"_e >> sib		] = rm16_func;
	rm16_1[ "mod@00 001 rm@..."_e					] = rm16_func;
	rm32_1[ "mod@00 001 rm@101"_e >> disp32	] = rm32_func;
	rm32_1[ "mod@00 001 rm@100"_e >> sib		] = rm32_func;
	rm32_1[ "mod@00 001 rm@..."_e					] = rm32_func;
	rm64_1[ "mod@00 001 rm@101"_e >> disp32	] = rm64_func;
	rm64_1[ "mod@00 001 rm@100"_e >> sib		] = rm64_func;
	rm64_1[ "mod@00 001 rm@..."_e					] = rm64_func;

	rm8_0 [ "mod@00 010 rm@101"_e >> disp32	] = rm8_func;
	rm8_0 [ "mod@00 010 rm@100"_e >> sib		] = rm8_func;
	rm8_0 [ "mod@00 010 rm@..."_e					] = rm8_func;
	rm16_0[ "mod@00 010 rm@101"_e >> disp32	] = rm16_func;
	rm16_0[ "mod@00 010 rm@100"_e >> sib		] = rm16_func;
	rm16_0[ "mod@00 010 rm@..."_e					] = rm16_func;
	rm32_0[ "mod@00 010 rm@101"_e >> disp32	] = rm32_func;
	rm32_0[ "mod@00 010 rm@100"_e >> sib		] = rm32_func;
	rm32_0[ "mod@00 010 rm@..."_e					] = rm32_func;
	rm64_0[ "mod@00 010 rm@101"_e >> disp32	] = rm64_func;
	rm64_0[ "mod@00 010 rm@100"_e >> sib		] = rm64_func;
	rm64_0[ "mod@00 010 rm@..."_e					] = rm64_func;

	rm8_0 [ "mod@00 011 rm@101"_e >> disp32	] = rm8_func;
	rm8_0 [ "mod@00 011 rm@100"_e >> sib		] = rm8_func;
	rm8_0 [ "mod@00 011 rm@..."_e					] = rm8_func;
	rm16_0[ "mod@00 011 rm@101"_e >> disp32	] = rm16_func;
	rm16_0[ "mod@00 011 rm@100"_e >> sib		] = rm16_func;
	rm16_0[ "mod@00 011 rm@..."_e					] = rm16_func;
	rm32_0[ "mod@00 011 rm@101"_e >> disp32	] = rm32_func;
	rm32_0[ "mod@00 011 rm@100"_e >> sib		] = rm32_func;
	rm32_0[ "mod@00 011 rm@..."_e					] = rm32_func;
	rm64_0[ "mod@00 011 rm@101"_e >> disp32	] = rm64_func;
	rm64_0[ "mod@00 011 rm@100"_e >> sib		] = rm64_func;
	rm64_0[ "mod@00 011 rm@..."_e					] = rm64_func;

	rm8_0 [ "mod@00 100 rm@101"_e >> disp32	] = rm8_func;
	rm8_0 [ "mod@00 100 rm@100"_e >> sib		] = rm8_func;
	rm8_0 [ "mod@00 100 rm@..."_e					] = rm8_func;
	rm16_0[ "mod@00 100 rm@101"_e >> disp32	] = rm16_func;
	rm16_0[ "mod@00 100 rm@100"_e >> sib		] = rm16_func;
	rm16_0[ "mod@00 100 rm@..."_e					] = rm16_func;
	rm32_0[ "mod@00 100 rm@101"_e >> disp32	] = rm32_func;
	rm32_0[ "mod@00 100 rm@100"_e >> sib		] = rm32_func;
	rm32_0[ "mod@00 100 rm@..."_e					] = rm32_func;
	rm64_0[ "mod@00 100 rm@101"_e >> disp32	] = rm64_func;
	rm64_0[ "mod@00 100 rm@100"_e >> sib		] = rm64_func;
	rm64_0[ "mod@00 100 rm@..."_e					] = rm64_func;

	rm8_0 [ "mod@00 101 rm@101"_e >> disp32	] = rm8_func;
	rm8_0 [ "mod@00 101 rm@100"_e >> sib		] = rm8_func;
	rm8_0 [ "mod@00 101 rm@..."_e					] = rm8_func;
	rm16_0[ "mod@00 101 rm@101"_e >> disp32	] = rm16_func;
	rm16_0[ "mod@00 101 rm@100"_e >> sib		] = rm16_func;
	rm16_0[ "mod@00 101 rm@..."_e					] = rm16_func;
	rm32_0[ "mod@00 101 rm@101"_e >> disp32	] = rm32_func;
	rm32_0[ "mod@00 101 rm@100"_e >> sib		] = rm32_func;
	rm32_0[ "mod@00 101 rm@..."_e					] = rm32_func;
	rm64_0[ "mod@00 101 rm@101"_e >> disp32	] = rm64_func;
	rm64_0[ "mod@00 101 rm@100"_e >> sib		] = rm64_func;
	rm64_0[ "mod@00 101 rm@..."_e					] = rm64_func;

	rm8_0 [ "mod@00 110 rm@101"_e >> disp32	] = rm8_func;
	rm8_0 [ "mod@00 110 rm@100"_e >> sib		] = rm8_func;
	rm8_0 [ "mod@00 110 rm@..."_e					] = rm8_func;
	rm16_0[ "mod@00 110 rm@101"_e >> disp32	] = rm16_func;
	rm16_0[ "mod@00 110 rm@100"_e >> sib		] = rm16_func;
	rm16_0[ "mod@00 110 rm@..."_e					] = rm16_func;
	rm32_0[ "mod@00 110 rm@101"_e >> disp32	] = rm32_func;
	rm32_0[ "mod@00 110 rm@100"_e >> sib		] = rm32_func;
	rm32_0[ "mod@00 110 rm@..."_e					] = rm32_func;
	rm64_0[ "mod@00 110 rm@101"_e >> disp32	] = rm64_func;
	rm64_0[ "mod@00 110 rm@100"_e >> sib		] = rm64_func;
	rm64_0[ "mod@00 110 rm@..."_e					] = rm64_func;

	rm8_0 [ "mod@00 111 rm@101"_e >> disp32	] = rm8_func;
	rm8_0 [ "mod@00 111 rm@100"_e >> sib		] = rm8_func;
	rm8_0 [ "mod@00 111 rm@..."_e					] = rm8_func;
	rm16_0[ "mod@00 111 rm@101"_e >> disp32	] = rm16_func;
	rm16_0[ "mod@00 111 rm@100"_e >> sib		] = rm16_func;
	rm16_0[ "mod@00 111 rm@..."_e					] = rm16_func;
	rm32_0[ "mod@00 111 rm@101"_e >> disp32	] = rm32_func;
	rm32_0[ "mod@00 111 rm@100"_e >> sib		] = rm32_func;
	rm32_0[ "mod@00 111 rm@..."_e					] = rm32_func;
	rm64_0[ "mod@00 111 rm@101"_e >> disp32	] = rm64_func;
	rm64_0[ "mod@00 111 rm@100"_e >> sib		] = rm64_func;
	rm64_0[ "mod@00 111 rm@..."_e					] = rm64_func;

	// mod = 01
	rm8 [ "mod@01 reg@... rm@100"_e >> sib		] = rm8_func;
	rm8 [ "mod@01 reg@... rm@..."_e >> disp8	] = rm8_func;
	rm16[ "mod@01 reg@... rm@100"_e >> sib		] = rm16_func;
	rm16[ "mod@01 reg@... rm@..."_e >> disp8	] = rm16_func;
	rm32[ "mod@01 reg@... rm@100"_e >> sib		] = rm32_func;
	rm32[ "mod@01 reg@... rm@..."_e >> disp8	] = rm32_func;
	rm64[ "mod@01 reg@... rm@100"_e >> sib		] = rm64_func;
	rm64[ "mod@01 reg@... rm@..."_e >> disp8	] = rm64_func;

	// mod = 01 w/ opcode extension
	rm8_0 [ "mod@01 000 rm@100"_e >> sib	] = rm8_func;
	rm8_0 [ "mod@01 000 rm@..."_e >> disp8	] = rm8_func;
	rm16_0[ "mod@01 000 rm@100"_e >> sib	] = rm16_func;
	rm16_0[ "mod@01 000 rm@..."_e >> disp8	] = rm16_func;
	rm32_0[ "mod@01 000 rm@100"_e >> sib	] = rm32_func;
	rm32_0[ "mod@01 000 rm@..."_e >> disp8	] = rm32_func;
	rm64_0[ "mod@01 000 rm@100"_e >> sib	] = rm64_func;
	rm64_0[ "mod@01 000 rm@..."_e >> disp8	] = rm64_func;

	rm8_1 [ "mod@01 001 rm@100"_e >> sib	] = rm8_func;
	rm8_1 [ "mod@01 001 rm@..."_e >> disp8	] = rm8_func;
	rm16_1[ "mod@01 001 rm@100"_e >> sib	] = rm16_func;
	rm16_1[ "mod@01 001 rm@..."_e >> disp8	] = rm16_func;
	rm32_1[ "mod@01 001 rm@100"_e >> sib	] = rm32_func;
	rm32_1[ "mod@01 001 rm@..."_e >> disp8	] = rm32_func;
	rm64_1[ "mod@01 001 rm@100"_e >> sib	] = rm64_func;
	rm64_1[ "mod@01 001 rm@..."_e >> disp8	] = rm64_func;

	rm8_2 [ "mod@01 010 rm@100"_e >> sib	] = rm8_func;
	rm8_2 [ "mod@01 010 rm@..."_e >> disp8	] = rm8_func;
	rm16_2[ "mod@01 010 rm@100"_e >> sib	] = rm16_func;
	rm16_2[ "mod@01 010 rm@..."_e >> disp8	] = rm16_func;
	rm32_2[ "mod@01 010 rm@100"_e >> sib	] = rm32_func;
	rm32_2[ "mod@01 010 rm@..."_e >> disp8	] = rm32_func;
	rm64_2[ "mod@01 010 rm@100"_e >> sib	] = rm64_func;
	rm64_2[ "mod@01 010 rm@..."_e >> disp8	] = rm64_func;

	rm8_3 [ "mod@01 011 rm@100"_e >> sib	] = rm8_func;
	rm8_3 [ "mod@01 011 rm@..."_e >> disp8	] = rm8_func;
	rm16_3[ "mod@01 011 rm@100"_e >> sib	] = rm16_func;
	rm16_3[ "mod@01 011 rm@..."_e >> disp8	] = rm16_func;
	rm32_3[ "mod@01 011 rm@100"_e >> sib	] = rm32_func;
	rm32_3[ "mod@01 011 rm@..."_e >> disp8	] = rm32_func;
	rm64_3[ "mod@01 011 rm@100"_e >> sib	] = rm64_func;
	rm64_3[ "mod@01 011 rm@..."_e >> disp8	] = rm64_func;

	rm8_4 [ "mod@01 100 rm@100"_e >> sib	] = rm8_func;
	rm8_4 [ "mod@01 100 rm@..."_e >> disp8	] = rm8_func;
	rm16_4[ "mod@01 100 rm@100"_e >> sib	] = rm16_func;
	rm16_4[ "mod@01 100 rm@..."_e >> disp8	] = rm16_func;
	rm32_4[ "mod@01 100 rm@100"_e >> sib	] = rm32_func;
	rm32_4[ "mod@01 100 rm@..."_e >> disp8	] = rm32_func;
	rm64_4[ "mod@01 100 rm@100"_e >> sib	] = rm64_func;
	rm64_4[ "mod@01 100 rm@..."_e >> disp8	] = rm64_func;

	rm8_5 [ "mod@01 101 rm@100"_e >> sib	] = rm8_func;
	rm8_5 [ "mod@01 101 rm@..."_e >> disp8	] = rm8_func;
	rm16_5[ "mod@01 101 rm@100"_e >> sib	] = rm16_func;
	rm16_5[ "mod@01 101 rm@..."_e >> disp8	] = rm16_func;
	rm32_5[ "mod@01 101 rm@100"_e >> sib	] = rm32_func;
	rm32_5[ "mod@01 101 rm@..."_e >> disp8	] = rm32_func;
	rm64_5[ "mod@01 101 rm@100"_e >> sib	] = rm64_func;
	rm64_5[ "mod@01 101 rm@..."_e >> disp8	] = rm64_func;

	rm8_6 [ "mod@01 110 rm@100"_e >> sib	] = rm8_func;
	rm8_6 [ "mod@01 110 rm@..."_e >> disp8	] = rm8_func;
	rm16_6[ "mod@01 110 rm@100"_e >> sib	] = rm16_func;
	rm16_6[ "mod@01 110 rm@..."_e >> disp8	] = rm16_func;
	rm32_6[ "mod@01 110 rm@100"_e >> sib	] = rm32_func;
	rm32_6[ "mod@01 110 rm@..."_e >> disp8	] = rm32_func;
	rm64_6[ "mod@01 110 rm@100"_e >> sib	] = rm64_func;
	rm64_6[ "mod@01 110 rm@..."_e >> disp8	] = rm64_func;

	rm8_7 [ "mod@01 111 rm@100"_e >> sib	] = rm8_func;
	rm8_7 [ "mod@01 111 rm@..."_e >> disp8	] = rm8_func;
	rm16_7[ "mod@01 111 rm@100"_e >> sib	] = rm16_func;
	rm16_7[ "mod@01 111 rm@..."_e >> disp8	] = rm16_func;
	rm32_7[ "mod@01 111 rm@100"_e >> sib	] = rm32_func;
	rm32_7[ "mod@01 111 rm@..."_e >> disp8	] = rm32_func;
	rm64_7[ "mod@01 111 rm@100"_e >> sib	] = rm64_func;
	rm64_7[ "mod@01 111 rm@..."_e >> disp8	] = rm64_func;

	// mod = 10
	rm8 [ "mod@10 reg@... rm@100"_e >> sib		] = rm8_func;
	rm8 [ "mod@10 reg@... rm@..."_e >> disp32	] = rm8_func;
	rm16[ "mod@10 reg@... rm@100"_e >> sib		] = rm16_func;
	rm16[ "mod@10 reg@... rm@..."_e >> disp32	] = rm16_func;
	rm32[ "mod@10 reg@... rm@100"_e >> sib		] = rm32_func;
	rm32[ "mod@10 reg@... rm@..."_e >> disp32	] = rm32_func;
	rm64[ "mod@10 reg@... rm@100"_e >> sib		] = rm64_func;
	rm64[ "mod@10 reg@... rm@..."_e >> disp32	] = rm64_func;

	// mod = 10 w/ opcode extension
	rm8_0 [ "mod@10 000 rm@100"_e >> sib		] = rm8_func;
	rm8_0 [ "mod@10 000 rm@..."_e >> disp32	] = rm8_func;
	rm16_0[ "mod@10 000 rm@100"_e >> sib		] = rm16_func;
	rm16_0[ "mod@10 000 rm@..."_e >> disp32	] = rm16_func;
	rm32_0[ "mod@10 000 rm@100"_e >> sib		] = rm32_func;
	rm32_0[ "mod@10 000 rm@..."_e >> disp32	] = rm32_func;
	rm64_0[ "mod@10 000 rm@100"_e >> sib		] = rm64_func;
	rm64_0[ "mod@10 000 rm@..."_e >> disp32	] = rm64_func;

	rm8_1 [ "mod@10 001 rm@100"_e >> sib		] = rm8_func;
	rm8_1 [ "mod@10 001 rm@..."_e >> disp32	] = rm8_func;
	rm16_1[ "mod@10 001 rm@100"_e >> sib		] = rm16_func;
	rm16_1[ "mod@10 001 rm@..."_e >> disp32	] = rm16_func;
	rm32_1[ "mod@10 001 rm@100"_e >> sib		] = rm32_func;
	rm32_1[ "mod@10 001 rm@..."_e >> disp32	] = rm32_func;
	rm64_1[ "mod@10 001 rm@100"_e >> sib		] = rm64_func;
	rm64_1[ "mod@10 001 rm@..."_e >> disp32	] = rm64_func;

	rm8_2 [ "mod@10 010 rm@100"_e >> sib		] = rm8_func;
	rm8_2 [ "mod@10 010 rm@..."_e >> disp32	] = rm8_func;
	rm16_2[ "mod@10 010 rm@100"_e >> sib		] = rm16_func;
	rm16_2[ "mod@10 010 rm@..."_e >> disp32	] = rm16_func;
	rm32_2[ "mod@10 010 rm@100"_e >> sib		] = rm32_func;
	rm32_2[ "mod@10 010 rm@..."_e >> disp32	] = rm32_func;
	rm64_2[ "mod@10 010 rm@100"_e >> sib		] = rm64_func;
	rm64_2[ "mod@10 010 rm@..."_e >> disp32	] = rm64_func;

	rm8_3 [ "mod@10 011 rm@100"_e >> sib		] = rm8_func;
	rm8_3 [ "mod@10 011 rm@..."_e >> disp32	] = rm8_func;
	rm16_3[ "mod@10 011 rm@100"_e >> sib		] = rm16_func;
	rm16_3[ "mod@10 011 rm@..."_e >> disp32	] = rm16_func;
	rm32_3[ "mod@10 011 rm@100"_e >> sib		] = rm32_func;
	rm32_3[ "mod@10 011 rm@..."_e >> disp32	] = rm32_func;
	rm64_3[ "mod@10 011 rm@100"_e >> sib		] = rm64_func;
	rm64_3[ "mod@10 011 rm@..."_e >> disp32	] = rm64_func;

	rm8_4 [ "mod@10 100 rm@100"_e >> sib		] = rm8_func;
	rm8_4 [ "mod@10 100 rm@..."_e >> disp32	] = rm8_func;
	rm16_4[ "mod@10 100 rm@100"_e >> sib		] = rm16_func;
	rm16_4[ "mod@10 100 rm@..."_e >> disp32	] = rm16_func;
	rm32_4[ "mod@10 100 rm@100"_e >> sib		] = rm32_func;
	rm32_4[ "mod@10 100 rm@..."_e >> disp32	] = rm32_func;
	rm64_4[ "mod@10 100 rm@100"_e >> sib		] = rm64_func;
	rm64_4[ "mod@10 100 rm@..."_e >> disp32	] = rm64_func;

	rm8_5 [ "mod@10 101 rm@100"_e >> sib		] = rm8_func;
	rm8_5 [ "mod@10 101 rm@..."_e >> disp32	] = rm8_func;
	rm16_5[ "mod@10 101 rm@100"_e >> sib		] = rm16_func;
	rm16_5[ "mod@10 101 rm@..."_e >> disp32	] = rm16_func;
	rm32_5[ "mod@10 101 rm@100"_e >> sib		] = rm32_func;
	rm32_5[ "mod@10 101 rm@..."_e >> disp32	] = rm32_func;
	rm64_5[ "mod@10 101 rm@100"_e >> sib		] = rm64_func;
	rm64_5[ "mod@10 101 rm@..."_e >> disp32	] = rm64_func;

	rm8_6 [ "mod@10 110 rm@100"_e >> sib		] = rm8_func;
	rm8_6 [ "mod@10 110 rm@..."_e >> disp32	] = rm8_func;
	rm16_6[ "mod@10 110 rm@100"_e >> sib		] = rm16_func;
	rm16_6[ "mod@10 110 rm@..."_e >> disp32	] = rm16_func;
	rm32_6[ "mod@10 110 rm@100"_e >> sib		] = rm32_func;
	rm32_6[ "mod@10 110 rm@..."_e >> disp32	] = rm32_func;
	rm64_6[ "mod@10 110 rm@100"_e >> sib		] = rm64_func;
	rm64_6[ "mod@10 110 rm@..."_e >> disp32	] = rm64_func;

	rm8_7 [ "mod@10 111 rm@100"_e >> sib		] = rm8_func;
	rm8_7 [ "mod@10 111 rm@..."_e >> disp32	] = rm8_func;
	rm16_7[ "mod@10 111 rm@100"_e >> sib		] = rm16_func;
	rm16_7[ "mod@10 111 rm@..."_e >> disp32	] = rm16_func;
	rm32_7[ "mod@10 111 rm@100"_e >> sib		] = rm32_func;
	rm32_7[ "mod@10 111 rm@..."_e >> disp32	] = rm32_func;
	rm64_7[ "mod@10 111 rm@100"_e >> sib		] = rm64_func;
	rm64_7[ "mod@10 111 rm@..."_e >> disp32	] = rm64_func;

	// mod = 11
	rm8 [ "mod@11 reg@... rm@..."_e ] = rm8_func;
	rm16[ "mod@11 reg@... rm@..."_e ] = rm16_func;
	rm32[ "mod@11 reg@... rm@..."_e ] = rm32_func;
	rm64[ "mod@11 reg@... rm@..."_e ] = rm64_func;

	// mod = 11 w/ opcode extension
	rm8_0 [ "mod@11 000 rm@..."_e ] = rm8_func;
	rm16_0[ "mod@11 000 rm@..."_e ] = rm16_func;
	rm32_0[ "mod@11 000 rm@..."_e ] = rm32_func;
	rm64_0[ "mod@11 000 rm@..."_e ] = rm64_func;

	rm8_1 [ "mod@11 001 rm@..."_e ] = rm8_func;
	rm16_1[ "mod@11 001 rm@..."_e ] = rm16_func;
	rm32_1[ "mod@11 001 rm@..."_e ] = rm32_func;
	rm64_1[ "mod@11 001 rm@..."_e ] = rm64_func;

	rm8_2 [ "mod@11 010 rm@..."_e ] = rm8_func;
	rm16_2[ "mod@11 010 rm@..."_e ] = rm16_func;
	rm32_2[ "mod@11 010 rm@..."_e ] = rm32_func;
	rm64_2[ "mod@11 010 rm@..."_e ] = rm64_func;

	rm8_3 [ "mod@11 011 rm@..."_e ] = rm8_func;
	rm16_3[ "mod@11 011 rm@..."_e ] = rm16_func;
	rm32_3[ "mod@11 011 rm@..."_e ] = rm32_func;
	rm64_3[ "mod@11 011 rm@..."_e ] = rm64_func;

	rm8_4 [ "mod@11 100 rm@..."_e ] = rm8_func;
	rm16_4[ "mod@11 100 rm@..."_e ] = rm16_func;
	rm32_4[ "mod@11 100 rm@..."_e ] = rm32_func;
	rm64_4[ "mod@11 100 rm@..."_e ] = rm64_func;

	rm8_5 [ "mod@11 101 rm@..."_e ] = rm8_func;
	rm16_5[ "mod@11 101 rm@..."_e ] = rm16_func;
	rm32_5[ "mod@11 101 rm@..."_e ] = rm32_func;
	rm64_5[ "mod@11 101 rm@..."_e ] = rm64_func;

	rm8_6 [ "mod@11 110 rm@..."_e ] = rm8_func;
	rm16_6[ "mod@11 110 rm@..."_e ] = rm16_func;
	rm32_6[ "mod@11 110 rm@..."_e ] = rm32_func;
	rm64_6[ "mod@11 110 rm@..."_e ] = rm64_func;

	rm8_7 [ "mod@11 111 rm@..."_e ] = rm8_func;
	rm16_7[ "mod@11 111 rm@..."_e ] = rm16_func;
	rm32_7[ "mod@11 111 rm@..."_e ] = rm32_func;
	rm64_7[ "mod@11 111 rm@..."_e ] = rm64_func;

	// 32 bits only
	main[ *generic_prfx >> 0x37			] = [](sm& m) { m.mnemonic(m.tokens.size(),"aaa","",std::list<rvalue>(),[](cg&) {}); };
	main[ *generic_prfx >> 0xd5 >> imm8	] = [](sm& m) { m.mnemonic(m.tokens.size(),"aad","{8}",*m.state.imm,[](cg&) {}); };
	main[ *generic_prfx >> 0xd4 >> imm8	] = [](sm& m) { m.mnemonic(m.tokens.size(),"aam","{8}",*m.state.imm,[](cg&) {}); };
	main[ *generic_prfx >> 0x3f			] = [](sm& m) { m.mnemonic(m.tokens.size(),"aas","",std::list<rvalue>(),[](cg&) {}); };

	// ADC
	std::function<void(cg&,rvalue,rvalue,boost::optional<std::pair<uint8_t,uint8_t>>)> adc = [](cg& m, rvalue a, rvalue b, boost::optional<std::pair<uint8_t,uint8_t>> sign_ext)
	{
		using dsl::operator%;
		using dsl::operator/;
		using dsl::operator+;
		using dsl::operator-;
		using dsl::operator*;

		if(sign_ext)
		{
			rvalue sign = b / (1 << (sign_ext->first - 1));
			rvalue rest = b % (1 << (sign_ext->first - 1));
			rvalue ex = (sign * (1 << (sign_ext->second - 1))) + rest;

			m.assign(to_lvalue(a),a + ex + CF);
		}
		else
		{
			m.assign(to_lvalue(a),a + b + CF);
		}
		// set OF, SF, ZF, AF, CF, and PF
	};

	main[ *generic_prfx >>						0x14 >> imm8				] = binary("adc",std::bind(decode_i,amd64_state::OpSz_8,pls::_1,pls::_2),
																											  std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));

	main[ *generic_prfx >> opsize_prfix	>> 0x15 >> imm16				] = binary("adc",std::bind(decode_i,amd64_state::OpSz_16,pls::_1,pls::_2),
																											  std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
	main[ *generic_prfx >>						0x15 >> imm32				] = binary("adc",std::bind(decode_i,amd64_state::OpSz_32,pls::_1,pls::_2),
																											  std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
	main[ *generic_prfx >> rexw_prfix	>> 0x15 >> imm32				] = binary("adc",std::bind(decode_i,amd64_state::OpSz_64,pls::_1,pls::_2),
																											  std::bind(adc,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(32,64)));

	main[ *generic_prfx >>						0x80 >> rm8_2 >> imm8	] = binary("adc",decode_mi,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
	main[ *generic_prfx >> rex_prfix		>> 0x80 >> rm8_2 >> imm8	] = binary("adc",decode_mi,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));

	main[ *generic_prfx >> opsize_prfix	>> 0x81 >> rm16_2 >> imm16	] = binary("adc",decode_mi,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
	main[ *generic_prfx >>						0x81 >> rm32_2 >> imm32	] = binary("adc",decode_mi,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
	main[ *generic_prfx >> rexw_prfix	>> 0x81 >> rm64_2 >> imm32	] = binary("adc",decode_mi,std::bind(adc,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(32,64)));

	main[ *generic_prfx >> opsize_prfix	>> 0x83 >> rm16_2 >> imm8	] = binary("adc",decode_mi,std::bind(adc,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,16)));
	main[ *generic_prfx >>						0x83 >> rm32_2 >> imm8	] = binary("adc",decode_mi,std::bind(adc,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,32)));
	main[ *generic_prfx >> rexw_prfix	>> 0x83 >> rm64_2 >> imm8	] = binary("adc",decode_mi,std::bind(adc,pls::_1,pls::_2,pls::_3,std::pair<uint8_t,uint8_t>(8,64)));

	main[ *generic_prfx >>						0x10 >> rm8					] = binary("adc",decode_mr,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
	main[ *generic_prfx >> rex_prfix		>> 0x10 >> rm8					] = binary("adc",decode_mr,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));

	main[ *generic_prfx >> opsize_prfix	>> 0x11 >> rm16				] = binary("adc",decode_mr,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
	main[ *generic_prfx >>						0x11 >> rm32				] = binary("adc",decode_mr,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
	main[ *generic_prfx >> rexw_prfix	>> 0x11 >> rm64				] = binary("adc",decode_mr,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));

	main[ *generic_prfx >>						0x12 >> rm8					] = binary("adc",decode_rm,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
	main[ *generic_prfx >> rex_prfix		>> 0x12 >> rm8					] = binary("adc",decode_rm,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));

	main[ *generic_prfx >> opsize_prfix	>> 0x13 >> rm16				] = binary("adc",decode_rm,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
	main[ *generic_prfx >> 						0x13 >> rm32				] = binary("adc",decode_rm,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));
	main[ *generic_prfx >> rexw_prfix	>> 0x13 >> rm64				] = binary("adc",decode_rm,std::bind(adc,pls::_1,pls::_2,pls::_3,boost::none));

	// ADD
	// ADX
	// AMX
	// AND
	// BOUND
	// BSF
	// BSR
	// BSWAP
	// BT
	// BTC
	// BTR
	// BTS
	// CALL
	// CALLF
	// CBW
	// CWDE
	// CWQE
	// CDQ
	// CLC
	// CLD
	// CLI
	// CMC
	// CMOVB
	// CMOVNAE
	// CMOVC
	// CMOVBE
	// CMOVNA
	// CMOVL
	// CMOVNGE
	// CMOVLE
	// CMOVNG
	// CMOVNB
	// CMOVAE
	// CMOVNC
	// CMOVNBE
	// CMOVA
	// CMOVNL
	// CMOVGE
	// CMOVNLE
	// CMOVG
	// CMOVNO
	// CMOVNP
	// CMOVPO
	// CMOVNS
	// CMOVNZ
	// CMOVNE
	// CMOVO
	// CMOVP
	// CMOVPE
	// CMOVS
	// CMOVZ
	// CMOVE
	// CMP
	// CMPS
	// CMPSW
	// CMPSB
	// CMPSD
	// CMPSQ
	// CMPXCHG
	// CMPXCHG8B
	// CMPXCHG16B
	// CPUID
	// CWD
	// CWQ
	// CDQ
	// CQO
	// CWDE
	// DAS
	// DEC
	// DIV
	// DAA
	// ENTER
	// HINT_NOP
	// IDIV
	// IMUL
	// IN
	// INC
	// INS
	// INSB
	// INSW
	// INSD
	// INT
	// INT1
	// ICEBP
	// INTO
	// IRET
	// IRETD
	// IRETQ
	// JB
	// JNAE
	// JC
	// JB
	// JBE
	// JNA
	// JCXZ
	// JECXZ
	// JRCXZ
	// JL
	// JNGE
	// JLE
	// JNG
	// JPE
	// JPF
	// JNB
	// JAE
	// JNC
	// JNBE
	// JA
	// JNL
	// JGE
	// JNLE
	// JNO
	// JNP
	// JNS
	// JNZ
	// JNE
	// JO
	// JP
	// JPE
	// JS
	// JZ
	// JE
	// LAHF
	// LDS
	// LEA
	// LEAVE
	// LES
	// LFS
	// LGS
	// LODS
	// LODSB
	// LODSW
	// LODSD
	// LODSQ
	// LOOP
	// LOOPNZ
	// LOOPNE
	// LOOPZ
	// LOOPE
	// LSS
	// MOV
	// MOVBE
	// MOVS
	// MOVSB
	// MOVSW
	// MOVSD
	// MOVSQ
	// MOVSXD
	// MOVSX
	// MOVZX
	// MUL
	// NEG
	// NOP
	// OR
	// OUT
	// OUTS
	// OUTSW
	// OUTSD
	// POP
	// POPA
	// POPAD
	// POPCNT
	// POPF
	// POPFQ
	// POPFD
	// PUSH
	// PUSHA
	// PUSHAD
	// PUSHF
	// PUSHFD
	// PUSHFQ
	// RCL
	// RCR
	// RETF
	// RETN
	// ROL
	// ROR
	// SAHF
	// SAL
	// SHL
	// SALC
	// SETALC
	// SAR
	// SBB
	// SCAS
	// SCASB
	// SCASW
	// SCASD
	// SCASQ
	// SETB
	// SETNE
	// SETNAE
	// SETC
	// SETBE
	// SETNA
	// SETL
	// SETNGE
	// SETLE
	// SETNG
	// SETNB
	// SETAE
	// SETNC
	// SETNBE
	// SETA
	// SETNL
	// SETGE
	// SETNLE
	// SETG
	// SETNO
	// SETNP
	// SETPO
	// SETNS
	// SETNZ
	// SETNE
	// SETO
	// SETP
	// SETPE
	// SETS
	// SETZ
	// SETE
	// SHL
	// SAL
	// SHLD
	// SHR
	// SHRD
	// STC
	// STD
	// STI
	// STOS
	// STOSB
	// STOSW
	// STOSD
	// STOSQ
	// SUB
	// TEST
	// UD
	// US2
	// XADD
	// XCHG
	// XOR

	return program::disassemble<amd64_tag>(main,bytes,r,prog);
}
