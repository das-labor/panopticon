#ifndef OUTPUT_HH
#define OUTPUT_HH

#include <flowgraph.hh>

void out_turtle(const po::flow_ptr f, const std::string &path);
void out_gv(const po::flow_ptr f, const std::string &path);
void out_zip(const po::flow_ptr f, const std::string &path);

#endif
