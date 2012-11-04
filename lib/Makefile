CXX=g++
CXXARGS=-Wall -g -std=c++0x -fPIC -pipe
LD=g++
LDFLAGS=-shared
PKGCONFIG=redland raptor2
LIBS=`pkg-config $(PKGCONFIG) --libs`
INCLUDES=-Iinclude `pkg-config $(PKGCONFIG) --cflags`

HEADERS=$(wildcard include/*.hh)
OBJECTS=avr.o mnemonic.o basic_block.o procedure.o flowgraph.o dflow.o absinterp.o database.o

.PHONY: clean

all: libpanopticum.so

libpanopticum.so: $(foreach i,$(OBJECTS),obj/$i)
	$(LD) $(LDFLAGS) -o $@ $^

obj/%.o: src/%.cc $(HEADERS)
	$(CXX) $(CXXARGS) -c $(INCLUDES) -o $@ $<

clean:
	rm -f libpanopticum.so obj/*
