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
	$(CXX) $(CXXARGS) $(INCLUDES) $(LIBS) -o $@ $<

clean:
	rm -f runtests
