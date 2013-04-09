#ifndef INPUT_HH
#define INPUT_HH

#include <string>
#include <flowgraph.hh>

po::flow_ptr in_avr(const std::string &path);
po::flow_ptr in_turtle(const std::string &path);
po::flow_ptr in_zip(const std::string &path);

#endif
