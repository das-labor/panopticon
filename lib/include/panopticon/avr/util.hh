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

#ifndef AVR_PRIVATE
#error "This header is not part of the public API!"
#endif

#include <panopticon/value.hh>

#pragma once

namespace po
{
	namespace avr
	{
		enum IndirectRegOp { PostInc, PreDec, PostDisplace, Nop };

		variable decode_reg(unsigned int r);
		variable decode_ioreg(unsigned int a);
		variable decode_preg(unsigned int r, IndirectRegOp op = Nop, int d = 0);

		memory sram(rvalue o);
		memory sram(unsigned int o);
		memory flash(rvalue o);
		memory flash(unsigned int o);

		sem_action unary_reg(::std::string x, ::std::function<void(cg &c, const variable &v)> func = ::std::function<void(cg&,const variable&)>());
		sem_action binary_reg(::std::string x, ::std::function<void(cg &,const variable&,const variable&)> func);
		sem_action branch(::std::string m, rvalue flag, bool set);
		sem_action binary_regconst(::std::string x, ::std::function<void(cg &,const variable&,const constant&)> func);
		sem_action binary_st(variable Rd1, variable Rd2, bool pre_dec = false, bool post_inc = false);
		sem_action binary_ld(variable Rr1, variable Rr2, bool pre_dec = false, bool post_inc = false);
		sem_action binary_stq(variable Rd1, variable Rd2);
		sem_action binary_ldq(variable Rr1, variable Rr2);
		sem_action simple(::std::string x, ::std::function<void(cg&)> fn);
	}
}
