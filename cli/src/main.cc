#include <iostream>
#include <vector>
#include <functional>
#include <algorithm>
#include <tuple>

#include <boost/program_options/value_semantic.hpp>
#include <boost/program_options/variables_map.hpp>
#include <boost/program_options/options_description.hpp>
#include <boost/program_options/parsers.hpp>

#include <flowgraph.hh>

#include <input.hh>
#include <filter.hh>
#include <output.hh>

using namespace po;
using namespace std;
using namespace boost::program_options;

const list<tuple<string,string,string,function<flow_ptr(const string &)>>> input =
{
	make_tuple("in-avr","a","AVR opcodes",in_avr),
	make_tuple("in-ttl","t","Flowgraph encoded in Turtle triples",in_turtle),
	make_tuple("in-zip","z","RDF triples save in a zipped Berkeley DB",in_zip),
};

const list<tuple<string,string,string,function<void(flow_ptr)>>> filter =
{
	make_tuple("ric","r","RIC constant propagation",filter_ric),
};

const list<tuple<string,string,string,function<void(const flow_ptr, const string &)>>> output =
{
	make_tuple("out-ttl", "T", "Flowgraph encoded in Turtle triples", out_turtle),
	make_tuple("out-gv", "G", "Dump flowgraph in dot", out_gv),
	make_tuple("out-zip", "Z", "Dump flowgraph in zip", out_zip),
};

int main(int argc, char *argv[])
{
	try
	{
		// Declare the supported options.
		options_description input_opts("Input files");
		options_description filter_opts("Filter");
		options_description output_opts("Output files");
		options_description general_opts("General");
		options_description all_opts("Panopticum terminal interface");

		general_opts.add_options()
			("help", "This help message")
		;

		for(const tuple<string,string,string,function<flow_ptr(const string &)>> &i: input)
			input_opts.add_options()(string(get<0>(i) + "," + get<1>(i)).c_str(), value<string>(), get<2>(i).c_str());
		
		for(const tuple<string,string,string,function<void(flow_ptr)>> &i: filter)
			filter_opts.add_options()(string(get<0>(i) + "," + get<1>(i)).c_str(), get<2>(i).c_str());
		
		for(const tuple<string,string,string,function<void(const flow_ptr, const string &)>> &i: output)
			output_opts.add_options()(string(get<0>(i) + "," + get<1>(i)).c_str(), value<string>(), get<2>(i).c_str());

		all_opts.add(general_opts).add(input_opts).add(filter_opts).add(output_opts);
		vector<option> opts = parse_command_line(argc, argv, all_opts).options;

		if(opts.empty() || count_if(opts.begin(),opts.end(),[&](const option &o) { return o.string_key == "help"; }))
		{
				cout << all_opts << "\n";
				return 1;
		}
		
		list<flow_ptr> flowgraphs;

		for(const option &opt: opts)
		{
			cout << opt.string_key << " = " << (opt.value.size() ? opt.value[0] : "(nil)") << endl;
			
			auto iter_i = find_if(input.begin(),input.end(),[&](const tuple<string,string,string,function<flow_ptr(const string &)>> &t)
				{ return get<0>(t) == opt.string_key || get<1>(t) == opt.string_key; });

			if(iter_i != input.end())
			{
				flowgraphs.push_back(get<3>(*iter_i)(opt.value[0]));
				continue;
			}

			auto iter_f = find_if(filter.begin(),filter.end(),[&](const tuple<string,string,string,function<void(flow_ptr)>> &t)
				{ return get<0>(t) == opt.string_key || get<1>(t) == opt.string_key; });

			if(iter_f != filter.end())
			{
				for(flow_ptr f: flowgraphs)
					get<3>(*iter_f)(f);
				continue;
			}
			
			auto iter_o = find_if(output.begin(),output.end(),[&](const tuple<string,string,string,function<void(const flow_ptr, const string &)>> &t)
				{ return get<0>(t) == opt.string_key || get<1>(t) == opt.string_key; });

			if(iter_o != output.end())
			{
				for(flow_ptr f: flowgraphs)
					get<3>(*iter_o)(f,opt.value[0]);
				continue;
			}

			cerr << "unknown option!" << endl;
			return 1;
		}
	}
	catch(runtime_error &e)
	{
		cerr << e.what() << endl;
	}
	catch(...)
	{
		cerr << "Caught unknown exception!" << endl;
	}
	
	return 0;
}
