include defaults.mk

COMPONENTS=lib qt cli

.PHONY: clean $(COMPONENTS) doc

all: $(COMPONENTS)

qt: lib qt/Makefile
qt/Makefile: qt/qt.pro
	cd qt; qmake -spec $(QT_SPEC)

cli: lib

$(COMPONENTS):
	$(MAKE) -C $@

doc:
	cd doc; doxygen doxyfile

clean:
	$(MAKE) -C lib clean
	$(MAKE) -C qt clean
	$(MAKE) -C cli clean
