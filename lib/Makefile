include ../defaults.mk

LDFLAGS+=-shared
INCLUDES+=`pkg-config $(PKGCONFIG) --cflags`

HEADERS=$(wildcard include/*.hh) $(wildcard include/avr/*.hh)
OBJECTS=avr/until.o avr/avr.o mnemonic.o basic_block.o procedure.o flowgraph.o dflow.o value.o interpreter.o inflate.o deflate.o sat.o disassembler.o

.PHONY: clean test

all: libpanopticum.so test

libpanopticum.so: $(foreach i,$(OBJECTS),obj/$i)
	$(LD) $(LDFLAGS) -o $@ $^

obj/%.o: src/%.cc $(HEADERS)
	$(CXX) $(CXXARGS) -c $(INCLUDES) -o $@ $<

test:
	make -C test
	
clean:
	rm -f libpanopticum.so obj/*.o obj/avr/*.o
	make -C test clean

