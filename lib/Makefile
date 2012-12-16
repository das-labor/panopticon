CXX=g++
CXXARGS=-Wall -g -std=c++0x -fPIC -pipe -pedantic -Werror -Wno-deprecated
LD=g++
LDFLAGS=-shared
PKGCONFIG=redland raptor2
LIBS=`pkg-config $(PKGCONFIG) --libs` -lcvc4
INCLUDES=-Iinclude `pkg-config $(PKGCONFIG) --cflags`

HEADERS=$(wildcard include/*.hh) $(wildcard include/avr/*.hh)
OBJECTS=avr/until.o avr/avr.o mnemonic.o basic_block.o procedure.o flowgraph.o dflow.o value.o interpreter.o inflate.o deflate.o sat.o

.PHONY: clean dirs

all: dirs libpanopticum.so

dirs:
	mkdir -p obj obj/avr

libpanopticum.so: $(foreach i,$(OBJECTS),obj/$i)
	$(LD) $(LDFLAGS) -o $@ $^

obj/%.o: src/%.cc $(HEADERS)
	$(CXX) $(CXXARGS) -c $(INCLUDES) -o $@ $<

tests: test/interpret_test.cc
	$(CXX) $(CXXARGS) $(LIBS) $(INCLUDES) -L. -lpanopticum -o test/interpret_test test/interpret_test.cc
	
clean:
	rm -f libpanopticum.so obj/*.o obj/avr/*.o

