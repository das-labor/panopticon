.PHONY: clean src test

all: src test

src:
	$(MAKE) -C $@ all

test: src
	$(MAKE) -C $@ all

clean:
	$(MAKE) -C src clean
	$(MAKE) -C test clean
