CXX=g++
CXXARGS=-Wall -g -std=c++0x -fPIC -pipe
LD=g++
LDFLAGS=-shared
LIBS=
INCLUDES=-Iinclude

HEADERS=$(wildcard include/*.hh)
OBJECTS=avr.o mnemonic.o basic_block.o procedure.o flowgraph.o dflow.o absinterp.o

.PHONY: clean

all: libpanopticum.so

libpanopticum.so: $(foreach i,$(OBJECTS),obj/$i)
	$(LD) $(LDFLAGS) -o $@ $^

obj/%.o: src/%.cc $(HEADERS)
	$(CXX) $(CXXARGS) -c $(INCLUDES) -o $@ $<

clean:
	rm -f libpanopticum.so obj/*
