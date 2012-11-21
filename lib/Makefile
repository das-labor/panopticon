CXX=g++
CXXARGS=-Wall -g -std=c++0x -fPIC -pipe -pedantic -Werror
LD=g++
LDFLAGS=-shared
PKGCONFIG=redland raptor2
LIBS=`pkg-config $(PKGCONFIG) --libs`
INCLUDES=-Iinclude `pkg-config $(PKGCONFIG) --cflags`

HEADERS=$(wildcard include/*.hh) $(wildcard include/avr/*.hh)
OBJECTS=avr/until.o avr/avr.o mnemonic.o basic_block.o procedure.o flowgraph.o dflow.o value.o interpreter.o inflate.o deflate.o

.PHONY: clean

all: libpanopticum.so

libpanopticum.so: $(foreach i,$(OBJECTS),obj/$i)
	$(LD) $(LDFLAGS) -o $@ $^

obj/%.o: src/%.cc $(HEADERS)
	$(CXX) $(CXXARGS) -c $(INCLUDES) -o $@ $<

tests: test/interpret_test.cc
	$(CXX) $(CXXARGS) $(LIBS) $(INCLUDES) -L. -lpanopticum -o test/interpret_test test/interpret_test.cc
	
clean:
	rm -f libpanopticum.so obj/*.o obj/avr/*.o

