CXX=g++
CXXARGS=-Wall -g -std=c++0x -fPIC -pipe
LD=g++
LDFLAGS=
LIBS=
INCLUDES=

HEADERS=$(wildcard *.hh)
OBJECTS=avr.o mnemonic.o basic_block.o procedure.o flow.o

.PHONY: clean

all: flow

flow: $(foreach i,$(OBJECTS),obj/$i)
	$(LD) -o $@ $^

obj/%.o: %.cc $(HEADERS)
	$(CXX) $(CXXARGS) -c $(INCLUDES) -o $@ $<

clean:
	rm -f flow obj/*
