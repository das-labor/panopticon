include ../../defaults.mk

LIBS+=-lcppunit -lpanopticum
INCLUDES+=`pkg-config $(PKGCONFIG) --cflags`

API=$(wildcard ../include/*.hh) $(wildcard ../include/avr/*.hh)
TESTS=$(wildcard test_*.hh)

.PHONY: clean

all: runtests

runtests: runtests.cc $(API) $(TESTS)
	$(CXX) $(CXXARGS) $(INCLUDES) $(LDFLAGS) $(LIBS) -o $@ $<

clean:
	rm -f runtests
