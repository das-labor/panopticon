/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2016 Kai Michaelis
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

use disassembler::*;
use amd64::decode::*;
use amd64::semantic::*;
use amd64::*;

use std::sync::Arc;

pub fn fpu(_: Arc<Disassembler<Amd64>>,
           rm0: Arc<Disassembler<Amd64>>, rm1: Arc<Disassembler<Amd64>>,
           rm2: Arc<Disassembler<Amd64>>, rm3: Arc<Disassembler<Amd64>>,
           rm4: Arc<Disassembler<Amd64>>, rm5: Arc<Disassembler<Amd64>>,
           rm6: Arc<Disassembler<Amd64>>, rm7: Arc<Disassembler<Amd64>>
           ) -> Arc<Disassembler<Amd64>> {
    new_disassembler!(Amd64 =>
        // F2XM1
        [ 0xd9, 0xf0 ] = nonary("f2xm1",f2xm1),

        // FABS
        [ 0xd9, 0xe1 ] = nonary("fabs",fabs),

        // FADD
        [ 0xd8, rm0 ] = binary_rv("fadd",&*ST0,decode_m32fp,fadd),
        [ 0xdc, rm0 ] = binary_rv("fadd",&*ST0,decode_m64fp,fadd),

        [ 0xd8, 0xc0 ] = binary_rv("fadd",&*ST0,decode_sti,fadd),
        [ 0xd8, 0xc1 ] = binary_rv("fadd",&*ST0,decode_sti,fadd),
        [ 0xd8, 0xc2 ] = binary_rv("fadd",&*ST0,decode_sti,fadd),
        [ 0xd8, 0xc3 ] = binary_rv("fadd",&*ST0,decode_sti,fadd),
        [ 0xd8, 0xc4 ] = binary_rv("fadd",&*ST0,decode_sti,fadd),
        [ 0xd8, 0xc5 ] = binary_rv("fadd",&*ST0,decode_sti,fadd),
        [ 0xd8, 0xc6 ] = binary_rv("fadd",&*ST0,decode_sti,fadd),
        [ 0xd8, 0xc7 ] = binary_rv("fadd",&*ST0,decode_sti,fadd),

        [ 0xdc, 0xc0 ] = binary_vr("fadd",decode_sti,&*ST0,fadd),
        [ 0xdc, 0xc1 ] = binary_vr("fadd",decode_sti,&*ST0,fadd),
        [ 0xdc, 0xc2 ] = binary_vr("fadd",decode_sti,&*ST0,fadd),
        [ 0xdc, 0xc3 ] = binary_vr("fadd",decode_sti,&*ST0,fadd),
        [ 0xdc, 0xc4 ] = binary_vr("fadd",decode_sti,&*ST0,fadd),
        [ 0xdc, 0xc5 ] = binary_vr("fadd",decode_sti,&*ST0,fadd),
        [ 0xdc, 0xc6 ] = binary_vr("fadd",decode_sti,&*ST0,fadd),
        [ 0xdc, 0xc7 ] = binary_vr("fadd",decode_sti,&*ST0,fadd),

        // FADDP
        [ 0xde, 0xc0 ] = binary_vr("faddp",decode_sti,&*ST0,fadd),
        [ 0xde, 0xc1 ] = binary_vr("faddp",decode_sti,&*ST0,fadd),
        [ 0xde, 0xc2 ] = binary_vr("faddp",decode_sti,&*ST0,fadd),
        [ 0xde, 0xc3 ] = binary_vr("faddp",decode_sti,&*ST0,fadd),
        [ 0xde, 0xc4 ] = binary_vr("faddp",decode_sti,&*ST0,fadd),
        [ 0xde, 0xc5 ] = binary_vr("faddp",decode_sti,&*ST0,fadd),
        [ 0xde, 0xc6 ] = binary_vr("faddp",decode_sti,&*ST0,fadd),
        [ 0xde, 0xc7 ] = binary_vr("faddp",decode_sti,&*ST0,fadd),

        // FIADD
        [ 0xda, rm0 ] = unary("fiadd",decode_m32int,fiadd),
        [ 0xde, rm0 ] = unary("fiadd",decode_m16int,fiadd),

        // FBLD
        [ 0xdf, rm4 ] = unary("fbld",decode_m80dec,fbld),

        // FBSTP
        [ 0xdf, rm6 ] = unary("fbstp",decode_m80bcd,fbld),

        // FCHS
        [ 0xd9, 0xe0 ] = nonary("fchs",fchs),

        // F(N)CLEX
        [ 0x9b, 0xdb, 0xe2 ] = nonary("fclex",fclex),
        [ 0xdb, 0xe2 ] = nonary("fnclex",fnclex),

        // FCMOVB
        [ 0xda, 0xc0 ] = binary_rv("fcmovb",&*ST0,decode_sti,fcmovb),
        [ 0xda, 0xc1 ] = binary_rv("fcmovb",&*ST0,decode_sti,fcmovb),
        [ 0xda, 0xc2 ] = binary_rv("fcmovb",&*ST0,decode_sti,fcmovb),
        [ 0xda, 0xc3 ] = binary_rv("fcmovb",&*ST0,decode_sti,fcmovb),
        [ 0xda, 0xc4 ] = binary_rv("fcmovb",&*ST0,decode_sti,fcmovb),
        [ 0xda, 0xc5 ] = binary_rv("fcmovb",&*ST0,decode_sti,fcmovb),
        [ 0xda, 0xc6 ] = binary_rv("fcmovb",&*ST0,decode_sti,fcmovb),
        [ 0xda, 0xc7 ] = binary_rv("fcmovb",&*ST0,decode_sti,fcmovb),

        // FCMOVE
        [ 0xda, 0xc8 ] = binary_rv("fcmove",&*ST0,decode_sti,fcmove),
        [ 0xda, 0xc9 ] = binary_rv("fcmove",&*ST0,decode_sti,fcmove),
        [ 0xda, 0xca ] = binary_rv("fcmove",&*ST0,decode_sti,fcmove),
        [ 0xda, 0xcb ] = binary_rv("fcmove",&*ST0,decode_sti,fcmove),
        [ 0xda, 0xcc ] = binary_rv("fcmove",&*ST0,decode_sti,fcmove),
        [ 0xda, 0xcd ] = binary_rv("fcmove",&*ST0,decode_sti,fcmove),
        [ 0xda, 0xce ] = binary_rv("fcmove",&*ST0,decode_sti,fcmove),
        [ 0xda, 0xcf ] = binary_rv("fcmove",&*ST0,decode_sti,fcmove),

        // FCMOVBE
        [ 0xda, 0xd0 ] = binary_rv("fcmovbe",&*ST0,decode_sti,fcmovbe),
        [ 0xda, 0xd1 ] = binary_rv("fcmovbe",&*ST0,decode_sti,fcmovbe),
        [ 0xda, 0xd2 ] = binary_rv("fcmovbe",&*ST0,decode_sti,fcmovbe),
        [ 0xda, 0xd3 ] = binary_rv("fcmovbe",&*ST0,decode_sti,fcmovbe),
        [ 0xda, 0xd4 ] = binary_rv("fcmovbe",&*ST0,decode_sti,fcmovbe),
        [ 0xda, 0xd5 ] = binary_rv("fcmovbe",&*ST0,decode_sti,fcmovbe),
        [ 0xda, 0xd6 ] = binary_rv("fcmovbe",&*ST0,decode_sti,fcmovbe),
        [ 0xda, 0xd7 ] = binary_rv("fcmovbe",&*ST0,decode_sti,fcmovbe),

        // FCMOVU
        [ 0xda, 0xd8 ] = binary_rv("fcmovu",&*ST0,decode_sti,fcmovu),
        [ 0xda, 0xd9 ] = binary_rv("fcmovu",&*ST0,decode_sti,fcmovu),
        [ 0xda, 0xda ] = binary_rv("fcmovu",&*ST0,decode_sti,fcmovu),
        [ 0xda, 0xdb ] = binary_rv("fcmovu",&*ST0,decode_sti,fcmovu),
        [ 0xda, 0xdc ] = binary_rv("fcmovu",&*ST0,decode_sti,fcmovu),
        [ 0xda, 0xdd ] = binary_rv("fcmovu",&*ST0,decode_sti,fcmovu),
        [ 0xda, 0xde ] = binary_rv("fcmovu",&*ST0,decode_sti,fcmovu),
        [ 0xda, 0xdf ] = binary_rv("fcmovu",&*ST0,decode_sti,fcmovu),

        // FCMOVNB
        [ 0xdb, 0xc0 ] = binary_rv("fcmovnb",&*ST0,decode_sti,fcmovnb),
        [ 0xdb, 0xc1 ] = binary_rv("fcmovnb",&*ST0,decode_sti,fcmovnb),
        [ 0xdb, 0xc2 ] = binary_rv("fcmovnb",&*ST0,decode_sti,fcmovnb),
        [ 0xdb, 0xc3 ] = binary_rv("fcmovnb",&*ST0,decode_sti,fcmovnb),
        [ 0xdb, 0xc4 ] = binary_rv("fcmovnb",&*ST0,decode_sti,fcmovnb),
        [ 0xdb, 0xc5 ] = binary_rv("fcmovnb",&*ST0,decode_sti,fcmovnb),
        [ 0xdb, 0xc6 ] = binary_rv("fcmovnb",&*ST0,decode_sti,fcmovnb),
        [ 0xdb, 0xc7 ] = binary_rv("fcmovnb",&*ST0,decode_sti,fcmovnb),

        // FCMOVNE
        [ 0xdb, 0xc8 ] = binary_rv("fcmovne",&*ST0,decode_sti,fcmovne),
        [ 0xdb, 0xc9 ] = binary_rv("fcmovne",&*ST0,decode_sti,fcmovne),
        [ 0xdb, 0xca ] = binary_rv("fcmovne",&*ST0,decode_sti,fcmovne),
        [ 0xdb, 0xcb ] = binary_rv("fcmovne",&*ST0,decode_sti,fcmovne),
        [ 0xdb, 0xcc ] = binary_rv("fcmovne",&*ST0,decode_sti,fcmovne),
        [ 0xdb, 0xcd ] = binary_rv("fcmovne",&*ST0,decode_sti,fcmovne),
        [ 0xdb, 0xce ] = binary_rv("fcmovne",&*ST0,decode_sti,fcmovne),
        [ 0xdb, 0xcf ] = binary_rv("fcmovne",&*ST0,decode_sti,fcmovne),

        // FCMOVNBE
        [ 0xdb, 0xd0 ] = binary_rv("fcmovnbe",&*ST0,decode_sti,fcmovnbe),
        [ 0xdb, 0xd1 ] = binary_rv("fcmovnbe",&*ST0,decode_sti,fcmovnbe),
        [ 0xdb, 0xd2 ] = binary_rv("fcmovnbe",&*ST0,decode_sti,fcmovnbe),
        [ 0xdb, 0xd3 ] = binary_rv("fcmovnbe",&*ST0,decode_sti,fcmovnbe),
        [ 0xdb, 0xd4 ] = binary_rv("fcmovnbe",&*ST0,decode_sti,fcmovnbe),
        [ 0xdb, 0xd5 ] = binary_rv("fcmovnbe",&*ST0,decode_sti,fcmovnbe),
        [ 0xdb, 0xd6 ] = binary_rv("fcmovnbe",&*ST0,decode_sti,fcmovnbe),
        [ 0xdb, 0xd7 ] = binary_rv("fcmovnbe",&*ST0,decode_sti,fcmovnbe),

        // FCMOVNU
        [ 0xdb, 0xd8 ] = binary_rv("fcmovnu",&*ST0,decode_sti,fcmovnu),
        [ 0xdb, 0xd9 ] = binary_rv("fcmovnu",&*ST0,decode_sti,fcmovnu),
        [ 0xdb, 0xda ] = binary_rv("fcmovnu",&*ST0,decode_sti,fcmovnu),
        [ 0xdb, 0xdb ] = binary_rv("fcmovnu",&*ST0,decode_sti,fcmovnu),
        [ 0xdb, 0xdc ] = binary_rv("fcmovnu",&*ST0,decode_sti,fcmovnu),
        [ 0xdb, 0xdd ] = binary_rv("fcmovnu",&*ST0,decode_sti,fcmovnu),
        [ 0xdb, 0xde ] = binary_rv("fcmovnu",&*ST0,decode_sti,fcmovnu),
        [ 0xdb, 0xdf ] = binary_rv("fcmovnu",&*ST0,decode_sti,fcmovnu),

        // FCOM
        [ 0xd8, rm2 ] = binary_rv("fcom",&*ST0,decode_m32fp,fcom),
        [ 0xdc, rm2 ] = binary_rv("fcom",&*ST0,decode_m64fp,fcom),
        [ 0xd8, 0xd0 ] = binary_rv("fcom",&*ST0,decode_sti,fcom),
        [ 0xd8, 0xd1 ] = binary_rv("fcom",&*ST0,decode_sti,fcom),
        [ 0xd8, 0xd2 ] = binary_rv("fcom",&*ST0,decode_sti,fcom),
        [ 0xd8, 0xd3 ] = binary_rv("fcom",&*ST0,decode_sti,fcom),
        [ 0xd8, 0xd4 ] = binary_rv("fcom",&*ST0,decode_sti,fcom),
        [ 0xd8, 0xd5 ] = binary_rv("fcom",&*ST0,decode_sti,fcom),
        [ 0xd8, 0xd6 ] = binary_rv("fcom",&*ST0,decode_sti,fcom),
        [ 0xd8, 0xd7 ] = binary_rv("fcom",&*ST0,decode_sti,fcom),
        [ 0xd8, 0xd8 ] = binary_rv("fcom",&*ST0,decode_sti,fcom),
        [ 0xd8, 0xd9 ] = binary_rv("fcom",&*ST0,decode_sti,fcom),
        [ 0xd8, 0xda ] = binary_rv("fcom",&*ST0,decode_sti,fcom),
        [ 0xd8, 0xdb ] = binary_rv("fcom",&*ST0,decode_sti,fcom),
        [ 0xd8, 0xdc ] = binary_rv("fcom",&*ST0,decode_sti,fcom),
        [ 0xd8, 0xdd ] = binary_rv("fcom",&*ST0,decode_sti,fcom),
        [ 0xd8, 0xde ] = binary_rv("fcom",&*ST0,decode_sti,fcom),
        [ 0xd8, 0xdf ] = binary_rv("fcom",&*ST0,decode_sti,fcom),

        // FCOMP
        [ 0xd8, rm3 ] = unary("fcomp",decode_m32fp,fcomp),
        [ 0xdc, rm3 ] = unary("fcomp",decode_m64fp,fcomp),

        // FCOMPP
        [ 0xde, 0xd9 ] = nonary("fcompp",fcompp),

        // FCOMI
        [ 0xdb, 0xf0 ] = binary_rv("fcomi",&*ST0,decode_sti,fcomi),
        [ 0xdb, 0xf1 ] = binary_rv("fcomi",&*ST0,decode_sti,fcomi),
        [ 0xdb, 0xf2 ] = binary_rv("fcomi",&*ST0,decode_sti,fcomi),
        [ 0xdb, 0xf3 ] = binary_rv("fcomi",&*ST0,decode_sti,fcomi),
        [ 0xdb, 0xf4 ] = binary_rv("fcomi",&*ST0,decode_sti,fcomi),
        [ 0xdb, 0xf5 ] = binary_rv("fcomi",&*ST0,decode_sti,fcomi),
        [ 0xdb, 0xf6 ] = binary_rv("fcomi",&*ST0,decode_sti,fcomi),
        [ 0xdb, 0xf7 ] = binary_rv("fcomi",&*ST0,decode_sti,fcomi),

        // FCOMIP
        [ 0xdf, 0xf0 ] = binary_rv("fcomip",&*ST0,decode_sti,fcomip),
        [ 0xdf, 0xf1 ] = binary_rv("fcomip",&*ST0,decode_sti,fcomip),
        [ 0xdf, 0xf2 ] = binary_rv("fcomip",&*ST0,decode_sti,fcomip),
        [ 0xdf, 0xf3 ] = binary_rv("fcomip",&*ST0,decode_sti,fcomip),
        [ 0xdf, 0xf4 ] = binary_rv("fcomip",&*ST0,decode_sti,fcomip),
        [ 0xdf, 0xf5 ] = binary_rv("fcomip",&*ST0,decode_sti,fcomip),
        [ 0xdf, 0xf6 ] = binary_rv("fcomip",&*ST0,decode_sti,fcomip),
        [ 0xdf, 0xf7 ] = binary_rv("fcomip",&*ST0,decode_sti,fcomip),

        // FUCOMI
        [ 0xdb, 0xe8 ] = binary_rv("fucomi",&*ST0,decode_sti,fucomi),
        [ 0xdb, 0xe9 ] = binary_rv("fucomi",&*ST0,decode_sti,fucomi),
        [ 0xdb, 0xea ] = binary_rv("fucomi",&*ST0,decode_sti,fucomi),
        [ 0xdb, 0xeb ] = binary_rv("fucomi",&*ST0,decode_sti,fucomi),
        [ 0xdb, 0xec ] = binary_rv("fucomi",&*ST0,decode_sti,fucomi),
        [ 0xdb, 0xed ] = binary_rv("fucomi",&*ST0,decode_sti,fucomi),
        [ 0xdb, 0xee ] = binary_rv("fucomi",&*ST0,decode_sti,fucomi),
        [ 0xdb, 0xef ] = binary_rv("fucomi",&*ST0,decode_sti,fucomi),

        // FUCOMIP
        [ 0xdf, 0xe8 ] = binary_rv("fucomip",&*ST0,decode_sti,fucomip),
        [ 0xdf, 0xe9 ] = binary_rv("fucomip",&*ST0,decode_sti,fucomip),
        [ 0xdf, 0xea ] = binary_rv("fucomip",&*ST0,decode_sti,fucomip),
        [ 0xdf, 0xeb ] = binary_rv("fucomip",&*ST0,decode_sti,fucomip),
        [ 0xdf, 0xec ] = binary_rv("fucomip",&*ST0,decode_sti,fucomip),
        [ 0xdf, 0xed ] = binary_rv("fucomip",&*ST0,decode_sti,fucomip),
        [ 0xdf, 0xee ] = binary_rv("fucomip",&*ST0,decode_sti,fucomip),
        [ 0xdf, 0xef ] = binary_rv("fucomip",&*ST0,decode_sti,fucomip),

        // FCOS
        [ 0xd9, 0xff ] = nonary("fcos",fcos),

        // FDECSTP
        [ 0xd9, 0xf6 ] = nonary("fdecstp",fdecstp),

        // FDIV
        [ 0xd8, rm6 ] = binary_rv("fdiv",&*ST0,decode_m32fp,fdiv),
        [ 0xdc, rm6 ] = binary_rv("fdiv",&*ST0,decode_m64fp,fdiv),

        // FDIV
        [ 0xd8, 0xf0 ] = binary_rv("fdiv",&*ST0,decode_sti,fdiv),
        [ 0xd8, 0xf1 ] = binary_rv("fdiv",&*ST0,decode_sti,fdiv),
        [ 0xd8, 0xf2 ] = binary_rv("fdiv",&*ST0,decode_sti,fdiv),
        [ 0xd8, 0xf3 ] = binary_rv("fdiv",&*ST0,decode_sti,fdiv),
        [ 0xd8, 0xf4 ] = binary_rv("fdiv",&*ST0,decode_sti,fdiv),
        [ 0xd8, 0xf5 ] = binary_rv("fdiv",&*ST0,decode_sti,fdiv),
        [ 0xd8, 0xf6 ] = binary_rv("fdiv",&*ST0,decode_sti,fdiv),
        [ 0xd8, 0xf7 ] = binary_rv("fdiv",&*ST0,decode_sti,fdiv),

        // FDIV
        [ 0xdc, 0xf8 ] = binary_vr("fdiv",decode_sti,&*ST0,fdiv),
        [ 0xdc, 0xf9 ] = binary_vr("fdiv",decode_sti,&*ST0,fdiv),
        [ 0xdc, 0xfa ] = binary_vr("fdiv",decode_sti,&*ST0,fdiv),
        [ 0xdc, 0xfb ] = binary_vr("fdiv",decode_sti,&*ST0,fdiv),
        [ 0xdc, 0xfc ] = binary_vr("fdiv",decode_sti,&*ST0,fdiv),
        [ 0xdc, 0xfd ] = binary_vr("fdiv",decode_sti,&*ST0,fdiv),
        [ 0xdc, 0xfe ] = binary_vr("fdiv",decode_sti,&*ST0,fdiv),
        [ 0xdc, 0xff ] = binary_vr("fdiv",decode_sti,&*ST0,fdiv),

        // FDIVP
        [ 0xde, 0xf8 ] = binary_vr("fdivp",decode_sti,&*ST0,fdivp),
        [ 0xde, 0xf9 ] = binary_vr("fdivp",decode_sti,&*ST0,fdivp),
        [ 0xde, 0xfa ] = binary_vr("fdivp",decode_sti,&*ST0,fdivp),
        [ 0xde, 0xfb ] = binary_vr("fdivp",decode_sti,&*ST0,fdivp),
        [ 0xde, 0xfc ] = binary_vr("fdivp",decode_sti,&*ST0,fdivp),
        [ 0xde, 0xfd ] = binary_vr("fdivp",decode_sti,&*ST0,fdivp),
        [ 0xde, 0xfe ] = binary_vr("fdivp",decode_sti,&*ST0,fdivp),
        [ 0xde, 0xff ] = binary_vr("fdivp",decode_sti,&*ST0,fdivp),

        // FIDIV
        [ 0xda, rm6 ] = unary("fidiv",decode_m32int,fidiv),
        [ 0xde, rm6 ] = unary("fidiv",decode_m16int,fidiv),

        // FDIVR
        [ 0xd8, rm7 ] = binary_vr("fdivr",decode_m32fp,&*ST0,fdivr),
        [ 0xdc, rm7 ] = binary_vr("fdivr",decode_m64fp,&*ST0,fdivr),

        // FDIV
        [ 0xd8, 0xf8 ] = binary_rv("fdivr",&*ST0,decode_sti,fdivr),
        [ 0xd8, 0xf9 ] = binary_rv("fdivr",&*ST0,decode_sti,fdivr),
        [ 0xd8, 0xfa ] = binary_rv("fdivr",&*ST0,decode_sti,fdivr),
        [ 0xd8, 0xfb ] = binary_rv("fdivr",&*ST0,decode_sti,fdivr),
        [ 0xd8, 0xfc ] = binary_rv("fdivr",&*ST0,decode_sti,fdivr),
        [ 0xd8, 0xfd ] = binary_rv("fdivr",&*ST0,decode_sti,fdivr),
        [ 0xd8, 0xfe ] = binary_rv("fdivr",&*ST0,decode_sti,fdivr),
        [ 0xd8, 0xff ] = binary_rv("fdivr",&*ST0,decode_sti,fdivr),

        // FDIVR
        [ 0xdc, 0xf0 ] = binary_vr("fdivr",decode_sti,&*ST0,fdivr),
        [ 0xdc, 0xf1 ] = binary_vr("fdivr",decode_sti,&*ST0,fdivr),
        [ 0xdc, 0xf2 ] = binary_vr("fdivr",decode_sti,&*ST0,fdivr),
        [ 0xdc, 0xf3 ] = binary_vr("fdivr",decode_sti,&*ST0,fdivr),
        [ 0xdc, 0xf4 ] = binary_vr("fdivr",decode_sti,&*ST0,fdivr),
        [ 0xdc, 0xf5 ] = binary_vr("fdivr",decode_sti,&*ST0,fdivr),
        [ 0xdc, 0xf6 ] = binary_vr("fdivr",decode_sti,&*ST0,fdivr),
        [ 0xdc, 0xf7 ] = binary_vr("fdivr",decode_sti,&*ST0,fdivr),

        // FDIVRP
        [ 0xde, 0xf0 ] = binary_vr("fdivrp",decode_sti,&*ST0,fdivrp),
        [ 0xde, 0xf1 ] = binary_vr("fdivrp",decode_sti,&*ST0,fdivrp),
        [ 0xde, 0xf2 ] = binary_vr("fdivrp",decode_sti,&*ST0,fdivrp),
        [ 0xde, 0xf3 ] = binary_vr("fdivrp",decode_sti,&*ST0,fdivrp),
        [ 0xde, 0xf4 ] = binary_vr("fdivrp",decode_sti,&*ST0,fdivrp),
        [ 0xde, 0xf5 ] = binary_vr("fdivrp",decode_sti,&*ST0,fdivrp),
        [ 0xde, 0xf6 ] = binary_vr("fdivrp",decode_sti,&*ST0,fdivrp),
        [ 0xde, 0xf7 ] = binary_vr("fdivrp",decode_sti,&*ST0,fdivrp),

        // FIDIVR
        [ 0xda, rm7 ] = unary("fidivr",decode_m32int,fidivr),
        [ 0xde, rm7 ] = unary("fidivr",decode_m16int,fidivr),

        // FFREE
        [ 0xdd, 0xc0 ] = nonary("ffree",ffree),
        [ 0xdd, 0xc1 ] = nonary("ffree",ffree),
        [ 0xdd, 0xc2 ] = nonary("ffree",ffree),
        [ 0xdd, 0xc3 ] = nonary("ffree",ffree),
        [ 0xdd, 0xc4 ] = nonary("ffree",ffree),
        [ 0xdd, 0xc5 ] = nonary("ffree",ffree),
        [ 0xdd, 0xc6 ] = nonary("ffree",ffree),
        [ 0xdd, 0xc7 ] = nonary("ffree",ffree),

        // FICOM
        [ 0xda, rm2 ] = unary("ficom",decode_m32int,ficom),
        [ 0xde, rm2 ] = unary("ficom",decode_m16int,ficom),

        // FICOMP
        [ 0xda, rm3 ] = unary("ficomp",decode_m32int,ficomp),
        [ 0xde, rm3 ] = unary("ficomp",decode_m16int,ficomp),

        // FILD
        [ 0xdf, rm0 ] = unary("fild",decode_m16int,fild),
        [ 0xdb, rm0 ] = unary("fild",decode_m32int,fild),
        [ 0xdf, rm5 ] = unary("fild",decode_m64int,fild),

        // FINCSTP
        [ 0xd9, 0xf7 ] = nonary("fincstp",fincstp),

        // FINIT
        [ 0x9b, 0xdb, 0xe3 ] = nonary("finit",finit),

        // FNINIT
        [ 0xdb, 0xe3 ] = nonary("fninit",fninit),

        // FIST
        [ 0xdf, rm2 ] = unary("ficom",decode_m16int,ficom),
        [ 0xdb, rm2 ] = unary("ficom",decode_m32int,ficom),

        // FISTP
        [ 0xdf, rm2 ] = unary("fistp",decode_m16int,fistp),
        [ 0xdb, rm2 ] = unary("fistp",decode_m32int,fistp),
        [ 0xdf, rm7 ] = unary("fistp",decode_m64int,fistp),

        // FISTTP
        [ 0xdf, rm1 ] = unary("fisttp",decode_m16int,fisttp),
        [ 0xdb, rm1 ] = unary("fisttp",decode_m32int,fisttp),
        [ 0xdd, rm1 ] = unary("fisttp",decode_m64int,fisttp),

        // FLD
        [ 0xd9, rm0 ] = unary("fld",decode_m32fp,fld),
        [ 0xdd, rm0 ] = unary("fld",decode_m64fp,fld),
        [ 0xdb, rm5 ] = unary("fld",decode_m80fp,fld),

        [ 0xd9, 0xc0 ] = unary("fld",decode_sti,fld),
        [ 0xd9, 0xc1 ] = unary("fld",decode_sti,fld),
        [ 0xd9, 0xc2 ] = unary("fld",decode_sti,fld),
        [ 0xd9, 0xc3 ] = unary("fld",decode_sti,fld),
        [ 0xd9, 0xc4 ] = unary("fld",decode_sti,fld),
        [ 0xd9, 0xc5 ] = unary("fld",decode_sti,fld),
        [ 0xd9, 0xc6 ] = unary("fld",decode_sti,fld),
        [ 0xd9, 0xc7 ] = unary("fld",decode_sti,fld),

        // FLD1
        [ 0xd9, 0xe8 ] = nonary("fld1",fld1),

        // FLDL2T
        [ 0xd9, 0xe9 ] = nonary("fldl2t",fldl2t),

        // FLDL2E
        [ 0xd9, 0xea ] = nonary("fldl2e",fldl2e),

        // FLDPI
        [ 0xd9, 0xeb ] = nonary("fldpi",fldpi),

        // FLDLG2
        [ 0xd9, 0xec ] = nonary("fldlg2",fldlg2),

        // FLDLN2
        [ 0xd9, 0xed ] = nonary("fldln2",fldln2),

        // FLDZ
        [ 0xd9, 0xee ] = nonary("fldz",fldz),

        // FLDCW
        [ 0xd9, rm5 ] = unary("fldcw",decode_m2byte,fldcw),

        // FLDENV
        [ 0xd9, rm4 ] = unary("fldenv",decode_m14_28byte,fldenv),

        // FMUL
        [ 0xd8, 0xc8 ] = binary_rv("fmul",&*ST0,decode_sti,fmul),
        [ 0xd8, 0xc9 ] = binary_rv("fmul",&*ST0,decode_sti,fmul),
        [ 0xd8, 0xca ] = binary_rv("fmul",&*ST0,decode_sti,fmul),
        [ 0xd8, 0xcb ] = binary_rv("fmul",&*ST0,decode_sti,fmul),
        [ 0xd8, 0xcc ] = binary_rv("fmul",&*ST0,decode_sti,fmul),
        [ 0xd8, 0xcd ] = binary_rv("fmul",&*ST0,decode_sti,fmul),
        [ 0xd8, 0xce ] = binary_rv("fmul",&*ST0,decode_sti,fmul),
        [ 0xd8, 0xcf ] = binary_rv("fmul",&*ST0,decode_sti,fmul),

        // FMUL
        [ 0xdc, 0xc8 ] = binary_vr("fmul",decode_sti,&*ST0,fmul),
        [ 0xdc, 0xc9 ] = binary_vr("fmul",decode_sti,&*ST0,fmul),
        [ 0xdc, 0xca ] = binary_vr("fmul",decode_sti,&*ST0,fmul),
        [ 0xdc, 0xcb ] = binary_vr("fmul",decode_sti,&*ST0,fmul),
        [ 0xdc, 0xcc ] = binary_vr("fmul",decode_sti,&*ST0,fmul),
        [ 0xdc, 0xcd ] = binary_vr("fmul",decode_sti,&*ST0,fmul),
        [ 0xdc, 0xce ] = binary_vr("fmul",decode_sti,&*ST0,fmul),
        [ 0xdc, 0xcf ] = binary_vr("fmul",decode_sti,&*ST0,fmul),

        // FMULP
        [ 0xde, 0xc8 ] = binary_vr("fmulp",decode_sti,&*ST0,fmulp),
        [ 0xde, 0xc9 ] = binary_vr("fmulp",decode_sti,&*ST0,fmulp),
        [ 0xde, 0xca ] = binary_vr("fmulp",decode_sti,&*ST0,fmulp),
        [ 0xde, 0xcb ] = binary_vr("fmulp",decode_sti,&*ST0,fmulp),
        [ 0xde, 0xcc ] = binary_vr("fmulp",decode_sti,&*ST0,fmulp),
        [ 0xde, 0xcd ] = binary_vr("fmulp",decode_sti,&*ST0,fmulp),
        [ 0xde, 0xce ] = binary_vr("fmulp",decode_sti,&*ST0,fmulp),
        [ 0xde, 0xcf ] = binary_vr("fmulp",decode_sti,&*ST0,fmulp),

        // FIMUL
        [ 0xda, rm1 ] = unary("fimul",decode_m32int,fimul),
        [ 0xde, rm1 ] = unary("fimul",decode_m16int,fimul),

        // FNOP
        [ 0xd9, 0xd0 ] = nonary("fnop",fnop),

        // FPATAN
        [ 0xd9, 0xf3 ] = nonary("fpatan",fpatan),

        // FPREM
        [ 0xd9, 0xf8 ] = nonary("fprem",fprem),

        // FPREM1
        [ 0xd9, 0xf5 ] = nonary("fprem1",fprem1),

        // FPTAN
        [ 0xd9, 0xf2 ] = nonary("fptan",fptan),

        // FRNDINT
        [ 0xd9, 0xfc ] = nonary("frndint",frndint),

        // FRSTOR
        [ 0xdd, rm4 ] = unary("frstor",decode_m94_108byte,frstor),

        // FSAVE
        [ 0x9b, 0xdd, rm6 ] = unary("fsave",decode_m94_108byte,fsave),

        // FNSAVE
        [ 0xdd, rm6 ] = unary("fnsave",decode_m94_108byte,fnsave),

        // FSCALE
        [ 0xd9, 0xfd ] = nonary("fscale",fscale),

        // FSIN
        [ 0xd9, 0xfe ] = nonary("fsin",fsin),

        // FSINCOS
        [ 0xd9, 0xfb ] = nonary("fsincos",fsincos),

        // FSQRT
        [ 0xd9, 0xfa ] = nonary("fsqrt",fsqrt),

        // FST
        [ 0xd9, rm2 ] = unary("fst",decode_m32fp,fst),
        [ 0xd9, rm2 ] = unary("fst",decode_m64fp,fst),

        [ 0xdd, 0xd0 ] = unary("fst",decode_sti,fst),
        [ 0xdd, 0xd1 ] = unary("fst",decode_sti,fst),
        [ 0xdd, 0xd2 ] = unary("fst",decode_sti,fst),
        [ 0xdd, 0xd3 ] = unary("fst",decode_sti,fst),
        [ 0xdd, 0xd4 ] = unary("fst",decode_sti,fst),
        [ 0xdd, 0xd5 ] = unary("fst",decode_sti,fst),
        [ 0xdd, 0xd6 ] = unary("fst",decode_sti,fst),
        [ 0xdd, 0xd7 ] = unary("fst",decode_sti,fst),

        // FSTP
        [ 0xd9, rm3 ] = unary("fstp",decode_m32fp,fstp),
        [ 0xdd, rm3 ] = unary("fstp",decode_m64fp,fstp),
        [ 0xdb, rm7 ] = unary("fstp",decode_m80fp,fstp),

        [ 0xdd, 0xd8 ] = unary("fstp",decode_sti,fstp),
        [ 0xdd, 0xd9 ] = unary("fstp",decode_sti,fstp),
        [ 0xdd, 0xda ] = unary("fstp",decode_sti,fstp),
        [ 0xdd, 0xdb ] = unary("fstp",decode_sti,fstp),
        [ 0xdd, 0xdc ] = unary("fstp",decode_sti,fstp),
        [ 0xdd, 0xdd ] = unary("fstp",decode_sti,fstp),
        [ 0xdd, 0xde ] = unary("fstp",decode_sti,fstp),
        [ 0xdd, 0xdf ] = unary("fstp",decode_sti,fstp),

        // FSTCW
        [ 0x9b, 0xd9, rm7 ] = unary("fstcw",decode_m2byte,fstcw),

        // FNSTCW
        [ 0xd9, rm7 ] = unary("fstcw",decode_m2byte,fstcw),

        // FLDENV
        [ 0xd9, rm4 ] = unary("fldenv",decode_m14_28byte,fldenv),

        // FSTENV
        [ 0x9b, 0xd9, rm6 ] = unary("fstenv",decode_m14_28byte,fstenv),

        // FNSTENV
        [ 0xd9, rm6 ] = unary("fnstenv",decode_m14_28byte,fnstenv),

        // FSTSW
        [ 0x9b, 0xdd, rm7 ] = unary("fstsw",decode_m2byte,fstsw),
        [ 0x9b, 0xdf, 0xe0 ] = unary("fstsw",reg_a,fstsw),

        // FNSTSW
        [ 0xdd, rm7 ] = unary("fnstsw",decode_m2byte,fnstsw),
        [ 0xdf, 0xe0 ] = unary("fnstsw",reg_a,fnstsw),

        // FSUB
        [ 0xd8, rm4 ] = binary_rv("fsub",&*ST0,decode_m32fp,fsub),
        [ 0xdc, rm4 ] = binary_rv("fsub",&*ST0,decode_m64fp,fsub),

        [ 0xd8, 0xe0 ] = binary_rv("fsub",&*ST0,decode_sti,fsub),
        [ 0xd8, 0xe1 ] = binary_rv("fsub",&*ST0,decode_sti,fsub),
        [ 0xd8, 0xe2 ] = binary_rv("fsub",&*ST0,decode_sti,fsub),
        [ 0xd8, 0xe3 ] = binary_rv("fsub",&*ST0,decode_sti,fsub),
        [ 0xd8, 0xe4 ] = binary_rv("fsub",&*ST0,decode_sti,fsub),
        [ 0xd8, 0xe5 ] = binary_rv("fsub",&*ST0,decode_sti,fsub),
        [ 0xd8, 0xe6 ] = binary_rv("fsub",&*ST0,decode_sti,fsub),
        [ 0xd8, 0xe7 ] = binary_rv("fsub",&*ST0,decode_sti,fsub),

        [ 0xdc, 0xe8 ] = binary_vr("fsub",decode_sti,&*ST0,fsub),
        [ 0xdc, 0xe9 ] = binary_vr("fsub",decode_sti,&*ST0,fsub),
        [ 0xdc, 0xea ] = binary_vr("fsub",decode_sti,&*ST0,fsub),
        [ 0xdc, 0xeb ] = binary_vr("fsub",decode_sti,&*ST0,fsub),
        [ 0xdc, 0xec ] = binary_vr("fsub",decode_sti,&*ST0,fsub),
        [ 0xdc, 0xed ] = binary_vr("fsub",decode_sti,&*ST0,fsub),
        [ 0xdc, 0xee ] = binary_vr("fsub",decode_sti,&*ST0,fsub),
        [ 0xdc, 0xef ] = binary_vr("fsub",decode_sti,&*ST0,fsub),

        // FSUBP
        [ 0xde, 0xe8 ] = binary_vr("fsubp",decode_sti,&*ST0,fsubp),
        [ 0xde, 0xe9 ] = binary_vr("fsubp",decode_sti,&*ST0,fsubp),
        [ 0xde, 0xea ] = binary_vr("fsubp",decode_sti,&*ST0,fsubp),
        [ 0xde, 0xeb ] = binary_vr("fsubp",decode_sti,&*ST0,fsubp),
        [ 0xde, 0xec ] = binary_vr("fsubp",decode_sti,&*ST0,fsubp),
        [ 0xde, 0xed ] = binary_vr("fsubp",decode_sti,&*ST0,fsubp),
        [ 0xde, 0xee ] = binary_vr("fsubp",decode_sti,&*ST0,fsubp),
        [ 0xde, 0xef ] = binary_vr("fsubp",decode_sti,&*ST0,fsubp),

        // FISUB
        [ 0xda, rm4 ] = unary("fisub",decode_m32int,fisub),
        [ 0xde, rm4 ] = unary("fisub",decode_m16int,fisub),

        // FSUBR
        [ 0xd8, rm5 ] = binary_vr("fsubr",decode_m32fp,&*ST0,fsubr),
        [ 0xdc, rm5 ] = binary_vr("fsubr",decode_m64fp,&*ST0,fsubr),

        [ 0xd8, 0xe8 ] = binary_vr("fsubr",decode_sti,&*ST0,fsubr),
        [ 0xd8, 0xe9 ] = binary_vr("fsubr",decode_sti,&*ST0,fsubr),
        [ 0xd8, 0xea ] = binary_vr("fsubr",decode_sti,&*ST0,fsubr),
        [ 0xd8, 0xeb ] = binary_vr("fsubr",decode_sti,&*ST0,fsubr),
        [ 0xd8, 0xec ] = binary_vr("fsubr",decode_sti,&*ST0,fsubr),
        [ 0xd8, 0xed ] = binary_vr("fsubr",decode_sti,&*ST0,fsubr),
        [ 0xd8, 0xee ] = binary_vr("fsubr",decode_sti,&*ST0,fsubr),
        [ 0xd8, 0xef ] = binary_vr("fsubr",decode_sti,&*ST0,fsubr),

        [ 0xdc, 0xe0 ] = binary_vr("fsubr",decode_sti,&*ST0,fsubr),
        [ 0xdc, 0xe1 ] = binary_vr("fsubr",decode_sti,&*ST0,fsubr),
        [ 0xdc, 0xe2 ] = binary_vr("fsubr",decode_sti,&*ST0,fsubr),
        [ 0xdc, 0xe3 ] = binary_vr("fsubr",decode_sti,&*ST0,fsubr),
        [ 0xdc, 0xe4 ] = binary_vr("fsubr",decode_sti,&*ST0,fsubr),
        [ 0xdc, 0xe5 ] = binary_vr("fsubr",decode_sti,&*ST0,fsubr),
        [ 0xdc, 0xe6 ] = binary_vr("fsubr",decode_sti,&*ST0,fsubr),
        [ 0xdc, 0xe7 ] = binary_vr("fsubr",decode_sti,&*ST0,fsubr),

        // FSUBRP
        [ 0xde, 0xe0 ] = binary_vr("fsubr",decode_sti,&*ST0,fsubr),
        [ 0xde, 0xe1 ] = binary_vr("fsubr",decode_sti,&*ST0,fsubr),
        [ 0xde, 0xe2 ] = binary_vr("fsubr",decode_sti,&*ST0,fsubr),
        [ 0xde, 0xe3 ] = binary_vr("fsubr",decode_sti,&*ST0,fsubr),
        [ 0xde, 0xe4 ] = binary_vr("fsubr",decode_sti,&*ST0,fsubr),
        [ 0xde, 0xe5 ] = binary_vr("fsubr",decode_sti,&*ST0,fsubr),
        [ 0xde, 0xe6 ] = binary_vr("fsubr",decode_sti,&*ST0,fsubr),
        [ 0xde, 0xe7 ] = binary_vr("fsubr",decode_sti,&*ST0,fsubr),

        // FISUBR
        [ 0xda, rm5 ] = unary("fisubr",decode_m32int,fisubr),
        [ 0xde, rm5 ] = unary("fisubr",decode_m16int,fisubr),

        // FTST
        [ 0xd9, 0xe4 ] = nonary("ftst",ftst),

        // FUCOM
        [ 0xdd, 0xe0 ] = binary_rv("fucom",&*ST0,decode_sti,fucom),
        [ 0xdd, 0xe1 ] = binary_rv("fucom",&*ST0,decode_sti,fucom),
        [ 0xdd, 0xe2 ] = binary_rv("fucom",&*ST0,decode_sti,fucom),
        [ 0xdd, 0xe3 ] = binary_rv("fucom",&*ST0,decode_sti,fucom),
        [ 0xdd, 0xe4 ] = binary_rv("fucom",&*ST0,decode_sti,fucom),
        [ 0xdd, 0xe5 ] = binary_rv("fucom",&*ST0,decode_sti,fucom),
        [ 0xdd, 0xe6 ] = binary_rv("fucom",&*ST0,decode_sti,fucom),
        [ 0xdd, 0xe7 ] = binary_rv("fucom",&*ST0,decode_sti,fucom),

        [ 0xdd, 0xe8 ] = binary_rv("fucomp",&*ST0,decode_sti,fucomp),
        [ 0xdd, 0xe9 ] = binary_rv("fucomp",&*ST0,decode_sti,fucomp),
        [ 0xdd, 0xea ] = binary_rv("fucomp",&*ST0,decode_sti,fucomp),
        [ 0xdd, 0xeb ] = binary_rv("fucomp",&*ST0,decode_sti,fucomp),
        [ 0xdd, 0xec ] = binary_rv("fucomp",&*ST0,decode_sti,fucomp),
        [ 0xdd, 0xed ] = binary_rv("fucomp",&*ST0,decode_sti,fucomp),
        [ 0xdd, 0xee ] = binary_rv("fucomp",&*ST0,decode_sti,fucomp),
        [ 0xdd, 0xef ] = binary_rv("fucomp",&*ST0,decode_sti,fucomp),

        // FCOMPP
        [ 0xda, 0xe9 ] = nonary("fucompp",fucompp),

        // FXAM
        [ 0xd9, 0xe5 ] = nonary("fxam",fxam),

        // FXCH
        [ 0xd9, 0xc8 ] = binary_rv("fxch",&*ST0,decode_sti,fxch),
        [ 0xd9, 0xc9 ] = binary_rv("fxch",&*ST0,decode_sti,fxch),
        [ 0xd9, 0xca ] = binary_rv("fxch",&*ST0,decode_sti,fxch),
        [ 0xd9, 0xcb ] = binary_rv("fxch",&*ST0,decode_sti,fxch),
        [ 0xd9, 0xcc ] = binary_rv("fxch",&*ST0,decode_sti,fxch),
        [ 0xd9, 0xcd ] = binary_rv("fxch",&*ST0,decode_sti,fxch),
        [ 0xd9, 0xce ] = binary_rv("fxch",&*ST0,decode_sti,fxch),
        [ 0xd9, 0xcf ] = binary_rv("fxch",&*ST0,decode_sti,fxch),

        // FXTRACT
        [ 0xd9, 0xf4 ] = nonary("fxtract",fxtract),

        // FYL2X
        [ 0xd9, 0xf1 ] = nonary("fyl2x",fyl2x),

        // FYL2XP1
        [ 0xd9, 0xf9 ] = nonary("fyl2xp1",fyl2xp1))
}

pub fn mpx(rm: Arc<Disassembler<Amd64>>) -> Arc<Disassembler<Amd64>> {
    new_disassembler!(Amd64 =>
        // BNDC*
        [ 0xf2, 0x0f, 0x1a, rm ] = binary("bndcu",decode_rm,bndcu),
        [ 0xf2, 0x0f, 0x1b, rm ] = binary("bndcn",decode_rm,bndcn),
        [ 0xf3, 0x0f, 0x1a, rm ] = binary("bndcl",decode_rm,bndcl),

        // BNDLDX
        [ 0x0f, 0x1a, rm ] = binary("bndldx",decode_rm,bndldx),

        // BNDMK
        [ 0xf3, 0x0f, 0x1b, rm ] = binary("bndmk",decode_rm,bndmk),

        // BNDMOV
        [ 0x66, 0x0f, 0x1a, rm ] = binary("bndmov",decode_rm,bndmov),
        [ 0x66, 0x0f, 0x1b, rm ] = binary("bndmov",decode_mr,bndmov),

        // BNDSTX
        [ 0x0f, 0x1b, rm ] = binary("bndstx",decode_mr,bndstx))
}
