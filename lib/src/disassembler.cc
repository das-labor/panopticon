#include <panopticon/disassembler.hh>

using namespace po;
using namespace std;

po::tokpat_error::tokpat_error(std::string w)
: std::invalid_argument(w)
{}
