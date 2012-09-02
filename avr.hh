#ifndef AVR_HH
#define AVR_HH

#include "mnemonic.hh"
#include "flowgraph.hh"

class reg : public variable
{
public:
	enum IndirectReg { X = 26, Y = 28, Z = 30 };
	enum IndirectRegOp { PostInc, PreDec, PostDisplace, Nop };

	reg(IndirectReg r, IndirectRegOp op = Nop, int d = 0) : variable(string("")), registerA(r)
	{
		switch(r)
		{
			case X: nam.base = "X"; break;
			case Y: nam.base = "Y"; break;
			case Z: nam.base = "Z"; break;
			default: nam.base = "INVALID: " + to_string(r);
		}

		switch(op)
		{
			case PostInc: nam.base += "+"; break;
			case PreDec: nam.base = "-" + nam.base; break;
			case Nop: break;
			case PostDisplace: if(r != X) { nam.base += "+" + to_string(d); break; }
			default: nam.base = "INVALID: " + to_string(r);
		}
	}

	reg(int r) : variable("r" + to_string(r)), registerA(r) {};
	reg(int rb, int ra) : variable("r" + to_string(ra) + ":r" + to_string(rb)), registerA(ra) {};
	
	int number(void) { return registerA; };

private:
	int registerA;
};

class ioreg : public variable
{
public:
	ioreg(int r) : variable(string("")), ioRegister(r)
	{
		switch(r)
		{
			default: nam.base = "io" + to_string(r); break;
			case 0x00: nam.base = "ubrr1"; break;
			case 0x01: nam.base = "ucsr1b"; break;
			case 0x02: nam.base = "ucsr1a"; break;
			case 0x03: nam.base = "udr1"; break;
			case 0x05: nam.base = "pine"; break;
			case 0x06: nam.base = "ddre"; break;
			case 0x07: nam.base = "porte"; break;
			case 0x08: nam.base = "acsr"; break;
			case 0x09: nam.base = "ubrr0"; break;
			case 0x0A: nam.base = "ucsr0b"; break;
			case 0x0B: nam.base = "ucsr0a"; break;
			case 0x0C: nam.base = "udr0"; break;
			case 0x0D: nam.base = "spcr"; break;
			case 0x0E: nam.base = "spsr"; break;
			case 0x0F: nam.base = "spdr"; break;
			case 0x10: nam.base = "pind"; break;
			case 0x11: nam.base = "ddrd"; break;
			case 0x12: nam.base = "portd"; break;
			case 0x13: nam.base = "pinc"; break;
			case 0x14: nam.base = "ddrc"; break;
			case 0x15: nam.base = "portc"; break;
			case 0x16: nam.base = "pinb"; break;
			case 0x17: nam.base = "ddrb"; break;
			case 0x18: nam.base = "portb"; break;
			case 0x19: nam.base = "pina"; break;
			case 0x1A: nam.base = "ddra"; break;
			case 0x1B: nam.base = "porta"; break;
			case 0x1C: nam.base = "eecr"; break;
			case 0x1D: nam.base = "eedr"; break;
			case 0x1E: nam.base = "eearl"; break;
			case 0x1F: nam.base = "eearh"; break;
			case 0x20: nam.base = "ubrrh"; break;
			case 0x21: nam.base = "wdtcr"; break;
			case 0x22: nam.base = "ocr2"; break;
			case 0x23: nam.base = "tcnt2"; break;
			case 0x24: nam.base = "icr1l"; break;
			case 0x25: nam.base = "icr1h"; break;
			case 0x26: nam.base = "assr"; break;
			case 0x27: nam.base = "tccr2"; break;
			case 0x28: nam.base = "ocr1bl"; break;
			case 0x29: nam.base = "ocr1bh"; break;
			case 0x2A: nam.base = "ocr1al"; break;
			case 0x2B: nam.base = "ocr1ah"; break;
			case 0x2C: nam.base = "tcnt1l"; break;
			case 0x2D: nam.base = "tcnt1h"; break;
			case 0x2E: nam.base = "tccr1b"; break;
			case 0x2F: nam.base = "tccr1a"; break;
			case 0x30: nam.base = "sfior"; break;
			case 0x31: nam.base = "ocr0"; break;
			case 0x32: nam.base = "tcnt0"; break;
			case 0x33: nam.base = "tccr0"; break;
			case 0x34: nam.base = "mcusr"; break;
			case 0x35: nam.base = "mcucr"; break;
			case 0x36: nam.base = "emcucr"; break;
			case 0x37: nam.base = "spmcr"; break;
			case 0x38: nam.base = "tifr"; break;
			case 0x39: nam.base = "timsk"; break;
			case 0x3A: nam.base = "gifr"; break;
			case 0x3B: nam.base = "gimsk"; break;
			case 0x3D: nam.base = "spl"; break;
			case 0x3E: nam.base = "sph"; break;
			case 0x3F: nam.base = "sreg"; break;
			//EEAR => _SFR_IO16(0x1E)
			//ICR1 => _SFR_IO16(0x24)
			//OCR1A => _SFR_IO16(0x2A)
			//OCR1B => _SFR_IO16(0x28)
			//TCNT1 => _SFR_IO16(0x2C)
		};
	}

	int number(void) { return ioRegister; };

private:
	int ioRegister;
};

flow_ptr avr_decode(vector<uint16_t> &bytes,addr_t entry);

#endif
