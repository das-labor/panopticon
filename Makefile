MAKE=make

COMPONENTS=lib qt

.PHONY: clean $(COMPONENTS)

all: $(COMPONENTS)

qt: lib

$(COMPONENTS):
	$(MAKE) -C $@

clean:
	$(MAKE) -C lib clean
	$(MAKE) -C qt clean
