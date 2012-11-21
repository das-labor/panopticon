#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>

#include <avr/avr.hh>
#include <flowgraph.hh>

using namespace po;

void decode(vector<uint16_t> &bytes)
{
	flow_ptr flow = avr::disassemble(bytes,0);
	cout << graphviz(flow) << endl;
}

int main(int argc, char *argv[])
{
	if(argc <= 1)
	{
		printf("AVR disasembler\n%s <files>\n",argv[0]);
		return 1;
	}

	int fn = 1;
	while(fn < argc)
	{
		std::ifstream f(argv[fn]);
		std::vector<uint16_t> bytes;

		if (f.bad())
        std::cout << "I/O error while reading" << std::endl;
    else if (f.fail())
        std::cout << "Non-integer data encountered" << std::endl;
		else 
		{
			while(f.good() && !f.eof())
			{
				uint16_t c;
				f.read((char *)&c,sizeof(c));
				bytes.push_back(c);
			}
			decode(bytes);
		}

		++fn;
	}

	return 0;
}
