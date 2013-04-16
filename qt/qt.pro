TEMPLATE = app
TARGET = panopticum
DEPENDPATH += . include src
INCLUDEPATH += . include ../lib/include
QMAKE_CXXFLAGS += -std=c++11 -Wno-deprecated
CONFIG += debug link_pkgconfig
OBJECTS_DIR = obj
MOC_DIR = obj
LIBS += -L../lib -lpanopticum -lcvc4
PKGCONFIG += raptor2 redland libgvc minizip

# Input
HEADERS += $$files(include/*.hh) \
					 $$files(../lib/include/*.hh) \
					 $$files(../include/lib/avr/*.hh)
SOURCES += $$files(src/*.cc)

