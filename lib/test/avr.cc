/*
 * This file is part of Panopticon (http://panopticon.re).
 * Copyright (C) 2015 Kai Michaelis
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

#include <gtest/gtest.h>

#include <panopticon/avr/avr.hh>

using namespace po;

#ifdef HAVE_TESTFILES_AVR
TEST(avr,all_opcodes_01)
{
	region_loc reg = region::mmap("flash",TESTDATA_DIR "/avr/all-01.obj");

	po::slab sl = reg->read();
	boost::optional<prog_loc> maybe_proc = avr::disassemble(avr_state::mega128(),boost::none,sl,po::ref{"flash",0});

	ASSERT_TRUE(!!maybe_proc);
    ASSERT_TRUE((*maybe_proc)->procedures().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder()[0]->mnemonics().size() >= 1u);

}

TEST(avr,all_opcodes_02)
{
	region_loc reg = region::mmap("flash",TESTDATA_DIR "/avr/all-02.obj");

	po::slab sl = reg->read();
	boost::optional<prog_loc> maybe_proc = avr::disassemble(avr_state::mega128(),boost::none,sl,po::ref{"flash",0});

	ASSERT_TRUE(!!maybe_proc);
    ASSERT_TRUE((*maybe_proc)->procedures().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder()[0]->mnemonics().size() >= 1u);

}

TEST(avr,all_opcodes_03)
{
	region_loc reg = region::mmap("flash",TESTDATA_DIR "/avr/all-03.obj");

	po::slab sl = reg->read();
	boost::optional<prog_loc> maybe_proc = avr::disassemble(avr_state::mega128(),boost::none,sl,po::ref{"flash",0});

	ASSERT_TRUE(!!maybe_proc);
    ASSERT_TRUE((*maybe_proc)->procedures().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder()[0]->mnemonics().size() >= 1u);

}

TEST(avr,all_opcodes_04)
{
	region_loc reg = region::mmap("flash",TESTDATA_DIR "/avr/all-04.obj");

	po::slab sl = reg->read();
	boost::optional<prog_loc> maybe_proc = avr::disassemble(avr_state::mega128(),boost::none,sl,po::ref{"flash",0});

	ASSERT_TRUE(!!maybe_proc);
    ASSERT_TRUE((*maybe_proc)->procedures().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder()[0]->mnemonics().size() >= 1u);

}

TEST(avr,all_opcodes_05)
{
	region_loc reg = region::mmap("flash",TESTDATA_DIR "/avr/all-05.obj");

	po::slab sl = reg->read();
	boost::optional<prog_loc> maybe_proc = avr::disassemble(avr_state::mega128(),boost::none,sl,po::ref{"flash",0});

	ASSERT_TRUE(!!maybe_proc);
    ASSERT_TRUE((*maybe_proc)->procedures().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder()[0]->mnemonics().size() >= 1u);

}

TEST(avr,all_opcodes_06)
{
	region_loc reg = region::mmap("flash",TESTDATA_DIR "/avr/all-06.obj");

	po::slab sl = reg->read();
	boost::optional<prog_loc> maybe_proc = avr::disassemble(avr_state::mega128(),boost::none,sl,po::ref{"flash",0});

	ASSERT_TRUE(!!maybe_proc);
    ASSERT_TRUE((*maybe_proc)->procedures().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder()[0]->mnemonics().size() >= 1u);

}

TEST(avr,all_opcodes_07)
{
	region_loc reg = region::mmap("flash",TESTDATA_DIR "/avr/all-07.obj");

	po::slab sl = reg->read();
	boost::optional<prog_loc> maybe_proc = avr::disassemble(avr_state::mega128(),boost::none,sl,po::ref{"flash",0});

	ASSERT_TRUE(!!maybe_proc);
    ASSERT_TRUE((*maybe_proc)->procedures().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder()[0]->mnemonics().size() >= 1u);

}

TEST(avr,all_opcodes_08)
{
	region_loc reg = region::mmap("flash",TESTDATA_DIR "/avr/all-08.obj");

	po::slab sl = reg->read();
	boost::optional<prog_loc> maybe_proc = avr::disassemble(avr_state::mega128(),boost::none,sl,po::ref{"flash",0});

	ASSERT_TRUE(!!maybe_proc);
    ASSERT_TRUE((*maybe_proc)->procedures().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder()[0]->mnemonics().size() >= 1u);

}

TEST(avr,all_opcodes_09)
{
	region_loc reg = region::mmap("flash",TESTDATA_DIR "/avr/all-09.obj");

	po::slab sl = reg->read();
	boost::optional<prog_loc> maybe_proc = avr::disassemble(avr_state::mega128(),boost::none,sl,po::ref{"flash",0});

	ASSERT_TRUE(!!maybe_proc);
    ASSERT_TRUE((*maybe_proc)->procedures().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder()[0]->mnemonics().size() >= 1u);

}

TEST(avr,all_opcodes_10)
{
	region_loc reg = region::mmap("flash",TESTDATA_DIR "/avr/all-10.obj");

	po::slab sl = reg->read();
	boost::optional<prog_loc> maybe_proc = avr::disassemble(avr_state::mega128(),boost::none,sl,po::ref{"flash",0});

	ASSERT_TRUE(!!maybe_proc);
    ASSERT_TRUE((*maybe_proc)->procedures().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder()[0]->mnemonics().size() >= 1u);

}

TEST(avr,all_opcodes_11)
{
	region_loc reg = region::mmap("flash",TESTDATA_DIR "/avr/all-11.obj");

	po::slab sl = reg->read();
	boost::optional<prog_loc> maybe_proc = avr::disassemble(avr_state::mega128(),boost::none,sl,po::ref{"flash",0});

	ASSERT_TRUE(!!maybe_proc);
    ASSERT_TRUE((*maybe_proc)->procedures().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder()[0]->mnemonics().size() >= 1u);

}

TEST(avr,all_opcodes_12)
{
	region_loc reg = region::mmap("flash",TESTDATA_DIR "/avr/all-12.obj");

	po::slab sl = reg->read();
	boost::optional<prog_loc> maybe_proc = avr::disassemble(avr_state::mega128(),boost::none,sl,po::ref{"flash",0});

	ASSERT_TRUE(!!maybe_proc);
    ASSERT_TRUE((*maybe_proc)->procedures().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder()[0]->mnemonics().size() >= 1u);

}

TEST(avr,all_opcodes_13)
{
	region_loc reg = region::mmap("flash",TESTDATA_DIR "/avr/all-13.obj");

	po::slab sl = reg->read();
	boost::optional<prog_loc> maybe_proc = avr::disassemble(avr_state::mega128(),boost::none,sl,po::ref{"flash",0});

	ASSERT_TRUE(!!maybe_proc);
    ASSERT_TRUE((*maybe_proc)->procedures().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder()[0]->mnemonics().size() >= 1u);

}

TEST(avr,all_opcodes_14)
{
	region_loc reg = region::mmap("flash",TESTDATA_DIR "/avr/all-14.obj");

	po::slab sl = reg->read();
	boost::optional<prog_loc> maybe_proc = avr::disassemble(avr_state::mega128(),boost::none,sl,po::ref{"flash",0});

	ASSERT_TRUE(!!maybe_proc);
    ASSERT_TRUE((*maybe_proc)->procedures().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder()[0]->mnemonics().size() >= 1u);

}

TEST(avr,all_opcodes_15)
{
	region_loc reg = region::mmap("flash",TESTDATA_DIR "/avr/all-15.obj");

	po::slab sl = reg->read();
	boost::optional<prog_loc> maybe_proc = avr::disassemble(avr_state::mega128(),boost::none,sl,po::ref{"flash",0});

	ASSERT_TRUE(!!maybe_proc);
    ASSERT_TRUE((*maybe_proc)->procedures().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder()[0]->mnemonics().size() >= 1u);

}

TEST(avr,jmp_overflow)
{
	region_loc reg = region::mmap("flash",TESTDATA_DIR "avr-jmp-overflow.obj");

	po::slab sl = reg->read();
	boost::optional<prog_loc> maybe_proc = avr::disassemble(avr_state::mega128(),boost::none,sl,po::ref{"flash",0});

	ASSERT_TRUE(!!maybe_proc);
    ASSERT_TRUE((*maybe_proc)->procedures().size() == 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder().size() == 2u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder()[0]->mnemonics().size() >= 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder()[1]->mnemonics().size() >= 1u);
}

#endif // HAVE_TESTFILES_AVR
