include ../defaults.mk

LIBS+=-lpanopticum -lboost_program_options `pkg-config $(PKGCONFIG) --libs`
INCLUDES+=-Iinclude `pkg-config $(PKGCONFIG) --cflags`

HEADERS=$(wildcard include/*.hh)
API=$(wildcard ../lib/include/*.hh) $(wildcard ../lib/include/avr/*.hh)
OBJECTS=main.o avr.o turtle.o gv.o ric.o zip.o

.PHONY: clean test

all: panop

panop: $(foreach i,$(OBJECTS),obj/$i)
	$(LD) $(LDFLAGS) $(LIBS) -o $@ $^

obj/%.o: src/%.cc $(HEADERS) $(API)
	$(CXX) $(CXXARGS) -c $(INCLUDES) -o $@ $<

test:
	$(MAKE) -C test

clean:
	rm -f panop obj/*.o
	$(MAKE) -C test clean
