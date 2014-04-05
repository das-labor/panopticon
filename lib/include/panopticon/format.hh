#include <string>
#include <functional>
#include <memory>

#include <boost/variant.hpp>

#include <panopticon/tree.hh>
#include <panopticon/region.hh>

namespace po
{
	struct format;

	struct format
	{
		struct ieee754
		{
			unsigned int bytes;
		};

		struct skip
		{
			offset bytes;
			std::string display;
		};

		struct integer
		{
			boost::variant<
				std::pair<bool,unsigned int>,															///< has_sign/bytes
				unsigned long long																				///< mask
			> mode;
			boost::variant<
				unsigned int,																							///< base
				std::pair<unsigned int,unsigned int>,											///< two bases
				std::shared_ptr<std::map<std::vector<byte>,std::string>>,	///< symbolic
				std::function<std::string(const std::list<byte>&)>				///< custom
			> display;
		};

		struct boolean
		{
			unsigned long long mask;
			std::pair<std::string,std::string> display;
		};

		struct reference
		{
			unsigned int bytes;
			offset off;
			std::string reg;
		};

		struct composition
		{
			boost::variant<
				std::string,
				std::function<std::string(const tree<format>&)>
			> display;
		};

		format(const std::string&, const ieee754&);
		format(const std::string&, const skip&);
		format(const std::string&, const integer&);
		format(const std::string&, const boolean&);
		format(const std::string&, const reference&);
		format(const std::string&, const composition&);

		std::string read(slab) const;
		unsigned int width(void) const;

		std::string name;
		boost::variant<ieee754,integer,boolean,composition> field;
	};

	tree<format> ipv4(region_loc reg);
}
