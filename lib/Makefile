include ../defaults.mk

LDFLAGS+=-shared
INCLUDES+=`pkg-config $(PKGCONFIG) --cflags`

HEADERS=$(wildcard include/*.hh) $(wildcard include/avr/*.hh)
OBJECTS=avr/until.o avr/avr.o mnemonic.o basic_block.o procedure.o flowgraph.o dflow.o value.o interpreter.o marshal.o sat.o disassembler.o

.PHONY: clean test

all: libpanopticum.so test

libpanopticum.so: .ycm_extra_conf.py $(foreach i,$(OBJECTS),obj/$i)
	$(LD) $(LDFLAGS) -o $@ $(foreach i,$(OBJECTS),obj/$i)

obj/%.o: src/%.cc $(HEADERS)
	$(CXX) $(CXXARGS) -c $(INCLUDES) -o $@ $<

test: libpanopticum.so
	$(MAKE) -C test

clean:
	rm -f libpanopticum.so obj/*.o obj/avr/*.o
	$(MAKE) -C test clean

.ycm_extra_conf.py: ../defaults.mk Makefile
	@echo "import os;" > $@
	@echo "" >> $@
	@echo "def FlagsForFile(a):" >> $@
	@echo -e "\tdname = os.path.dirname(os.path.abspath(__file__ ))" >> $@
	@echo -en "\treturn {'flags': ['-Wall','-std=c++11','-x','c++'," >> $@
	@for I in $(foreach i,$(INCLUDES),$i); do echo -n "'$$I',"; done >> $@
	@echo "'-I',os.path.join(dname,'include')], 'do_cache': True}" >> $@
