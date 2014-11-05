#include <panopticon/program.hh>

#pragma once

namespace po
{
	// architecture_traits
	struct amd64_tag {};

	struct amd64_state
	{
		amd64_state(void)
		: addr_sz(AddrSz_32), op_sz(OpSz_32),
		  rex(false),
		  reg(boost::none), rm(boost::none), imm(boost::none)
		{}

		enum AddressSize
		{
			AddrSz_64,
			AddrSz_32,
			AddrSz_16,
		};

		enum OperandSize
		{
			OpSz_64,
			OpSz_32,
			OpSz_16,
			OpSz_8,
		};

		AddressSize addr_sz;
		OperandSize op_sz;

		bool rex;

		boost::optional<lvalue> reg, rm;
		boost::optional<constant> imm;
	};

	template<>
	struct architecture_traits<amd64_tag>
	{
		using token_type = uint8_t;
		using state_type = amd64_state;
	};

	template<>
	lvalue temporary(amd64_tag);

	template<>
	const std::vector<std::string> &registers(amd64_tag);

	template<>
	uint8_t width(std::string n, amd64_tag);

	namespace amd64
	{
		typedef sem_state<amd64_tag> sm;
		typedef std::function<void(sm &)> sem_action;
		typedef code_generator<amd64_tag> cg;

		// 8 bit gp registers
		extern const variable al,bl,cl,dl,
				 					 ah,bh,ch,dh,
									 r8l,r9l,r10l,r11l,r12l,r13l,r14l,r15l,
									 spl,bpl,sil,dil;
		// 16 bit gp registers
		extern const variable ax,bx,cx,dx,
				 					 r8w,r9w,r10w,r11w,r12w,r13w,r14w,r15w,
									 si,di,sp,bp;
		// 32 bit gp registers
		extern const variable eax,ebx,ecx,edx,
				 					 esi,edi,
									 r8d,r9d,r10d,r11d,r12d,r13d,r14d,r15d;
		// 64 bit gp registers
		extern const variable rax,rbx,rcx,rdx,
				 					 rsi,rdi,
									 r4,r5,r6,r7,r8,r9,r10,r11,r12,r13,r14,r15;
		// 32 bit management registers
		extern const variable esp,ebp,eip,/*eflags,*/CF,PF,AF,ZF,SF,TF,IF,DF,OF,IOPL,NT,RF,VM,AC,VIF,VIP,ID;
		// 64 bit management registers
		extern const variable rsp,rbp,rip,rflags;

		boost::optional<prog_loc> disassemble(boost::optional<prog_loc>, po::slab, const po::ref&);
	}
}
