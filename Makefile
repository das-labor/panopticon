MAKE=make

COMPONENTS=lib qt

.PHONY: clean $(COMPONENTS)

all: $(COMPONENTS)

$(COMPONENTS):
	$(MAKE) -C $@

clean:
	$(MAKE) -C lib
	$(MAKE) -C qt
