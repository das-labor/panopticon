#include <QApplication>
#include <QDebug>

#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>
#include <thread>

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
			po::flow_ptr flow(new po::flowgraph());
			std::function<void(po::proc_ptr,unsigned int)> sig = [](po::proc_ptr p, unsigned int pos)
			{
				if(p)
					qDebug() << "insert" << QString::fromStdString(p->name) << "at" << pos;
				else
					qDebug() << "about to insert a procedure at" << pos;
			};
			std::thread l([&](void)
			{
				po::avr::disassemble(bytes,0,flow,sig);
			});

			l.join();
			Window win(flow);//po::avr::disassemble(bytes,0));

			win.show();
			app.exec();
		}
	}

	return 0;
}
