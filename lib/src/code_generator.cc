#include <panopticon/code_generator.hh>

#ifdef _MSC_VER
__declspec(thread) po::dsl::callback_list* po::dsl::current_code_generator = nullptr;
#else
__thread po::dsl::callback_list* po::dsl::current_code_generator = nullptr;
#endif
