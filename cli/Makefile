include ../defaults.mk

LIBS+=-lpanopticum -lboost_program_options
INCLUDES+=-Iinclude
HEADERS=$(wildcard include/*.hh)
API=$(wildcard ../lib/include/*.hh)
OBJECTS=main.o avr.o turtle.o gv.o ric.o

.PHONY: clean test

all: panop

panop: $(foreach i,$(OBJECTS),obj/$i)
	$(LD) $(LDFLAGS) $(LIBS) -o $@ $^

obj/%.o: src/%.cc $(HEADERS) $(API)
	$(CXX) $(CXXARGS) -c $(INCLUDES) -o $@ $<

test:
	make -C test
	
clean:
	rm -f panop obj/*.o
	make -C test clean
