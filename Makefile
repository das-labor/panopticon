COMPONENTS=lib qt cli

.PHONY: clean $(COMPONENTS) doc

all: $(COMPONENTS)

qt: lib
cli: lib

$(COMPONENTS):
	$(MAKE) -C $@

doc:
	cd doc; doxygen doxyfile

clean:
	$(MAKE) -C lib clean
	$(MAKE) -C qt clean
	$(MAKE) -C cli clean
