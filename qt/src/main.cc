#include <QApplication>
#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>

#include <deflate.hh>
#include <avr/avr.hh>
#include <flowgraph.hh>

#include <window.hh>

int main(int argc, char *argv[])
{
	if(argc >= 1)
	{
		std::ifstream f(argv[1]);
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

			QApplication app(argc,argv);
			Window win(po::avr::disassemble(bytes,0));

			win.show();
			app.exec();
		}
	}

	return 0;
}
