#include <QApplication>
#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>

#include <database.hh>
#include <avr.hh>
#include <flowgraph.hh>

#include <window.hh>

void decode(vector<uint16_t> &bytes)
{
	flow_ptr flow = avr_decode(bytes,0);
	cout << turtle(flow) << endl;
}

int main(int argc, char *argv[])
{
	if(argc <= 1)
	{
		QApplication app(argc,argv);
		Window win;

		win.show();
		app.exec();
		return 1;
	}

	int fn = 1;
	while(fn < argc)
	{
		ifstream f(argv[fn]);
		vector<uint16_t> bytes;

		if (f.bad())
        cout << "I/O error while reading" << endl;
    else if (f.fail())
        cout << "Non-integer data encountered" << endl;
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
