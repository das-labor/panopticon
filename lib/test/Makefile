include ../../defaults.mk

LIBS+=-lcppunit -lpanopticum

API=$(wildcard ../include/*.hh) $(wildcard ../include/avr/*.hh)
TESTS=$(wildcard *.hh)
TESTS=test_codegenerator.hh

.PHONY: clean

all: test

test: test.cc $(API) $(TESTS)
	$(CXX) $(CXXARGS) $(INCLUDES) $(LDFLAGS) $(LIBS) -o $@ $<

clean:
	rm -f runtests
