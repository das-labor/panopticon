#include <panopticon/code_generator.hh>

#ifdef MSVC
__declspec(thread) po::dsl::callback_list* po::dsl::current_code_generator = nullptr;
#elif
__thread po::dsl::callback_list* po::dsl::current_code_generator = nullptr;
#endif
