#include <gtest/gtest.h>

#include <panopticon/amd64/amd64.hh>

using namespace po;

TEST(amd64,simple)
{
	region_loc reg = region::wrap("ram",{
0x48,0x11,0x1c,0x25,0xa1,0x1a,0x00,0x00
	});

	po::slab sl = reg->read();
	boost::optional<prog_loc> maybe_proc = amd64::disassemble(boost::none,sl,po::ref{"ram",0});

	ASSERT_TRUE(!!maybe_proc);
	ASSERT_EQ((*maybe_proc)->procedures().size(), 1u);
	ASSERT_EQ((*(*maybe_proc)->procedures().begin())->rev_postorder().size(), 1u);
	ASSERT_EQ((*(*maybe_proc)->procedures().begin())->rev_postorder()[0]->mnemonics().size(), 1u);
}
