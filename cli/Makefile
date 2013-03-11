include ../defaults.mk

LIBS+=-lpanopticum

HEADERS=$(wildcard include/*.hh)
OBJECTS=main.o

.PHONY: clean test

all: panop

panop: $(foreach i,$(OBJECTS),obj/$i)
	$(LD) $(LDFLAGS) $(LIBS) -o $@ $^

obj/%.o: src/%.cc $(HEADERS)
	$(CXX) $(CXXARGS) -c $(INCLUDES) -o $@ $<

test:
	make -C test
	
clean:
	rm -f panop obj/*.o
	make -C test clean
