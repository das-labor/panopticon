#include <iostream>
#include <fstream>
#include <vector>

#include "avr.hh"

using namespace std;

/*
 * TODO
 * - finish disassemble
 * - cfg reconstr
 * main | <skip> | <2tok instr> = add to control_trans and instr
 * main | <skip> = 1 tok instr
 * avr.cc/hh instr_ptr/guard_ptr generieren lassen
 * cfg von instr_ptr/guard_ptr
 * call instr
 */

void decode(vector<uint16_t> &bytes)
{
	avr_decode(bytes,6310);
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
