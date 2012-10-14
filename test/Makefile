CXX=g++
CXXARGS=-Wall -g -std=c++0x -fPIC -pipe
LD=g++
LDFLAGS=
LIBS=-lcppunit
INCLUDES=-I../src/include

API=$(wildcard ../src/include/*.hh)
HEADERS=$(wildcard *.hh)

.PHONY: clean

all: runtests

runtests: runtests.cc $(HEADERS) $(API)
	$(CXX) $(CXXARGS) $(INCLUDES) $(LIBS) ../src/obj/mnemonic.o ../src/obj/code_generator.o ../src/obj/basic_block.o ../src/obj/procedure.o ../src/obj/flowgraph.o ../src/obj/absinterp.o -o $@ $<

clean:
	rm -f runtests
