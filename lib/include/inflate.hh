#ifndef INFLATE_HH
#define INFLATE_HH

#include <sstream>

namespace po
{
	class odotstream : public std::ostringstream
	{
	public:
		odotstream(void);

		bool calls;
		bool body;
		bool subgraph;
		bool instrs;
	};
	
	odotstream &operator<<(odotstream &os, odotstream &(*func)(odotstream &os));

	odotstream &calls(odotstream &os);
	odotstream &nocalls(odotstream &os);
	odotstream &body(odotstream &os);
	odotstream &nobody(odotstream &os);
	odotstream &subgraph(odotstream &os);
	odotstream &nosubgraph(odotstream &os);
	odotstream &instrs(odotstream &os);
	odotstream &noinstrs(odotstream &os);

	template<typename T>
	std::string unique_name(const T &t)
	{
		return std::string("generic_") + std::to_string((uintptr_t)&t);
	}

	template<typename T>
	odotstream &operator<<(odotstream &os, const T &t)
	{
		static_cast<std::ostringstream &>(os) << t;
		return os;
	}

	class oturtlestream : public std::ostringstream
	{
	public:
		oturtlestream(void);

		std::string blank(void);
	
	private:
		unsigned long long m_blank;
	};
	
	oturtlestream &operator<<(oturtlestream &os, std::ostream& (*func)(std::ostream&));

	template<typename T>
	oturtlestream &operator<<(oturtlestream &os, const T &t)
	{
		static_cast<std::ostringstream &>(os) << t;
		return os;
	}
}

#endif
