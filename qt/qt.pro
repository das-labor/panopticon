TEMPLATE = app
TARGET = panopticum
DEPENDPATH += . include src
INCLUDEPATH += . include ../lib/include
QMAKE_CXXFLAGS += -std=c++0x
CONFIG += debug link_pkgconfig
OBJECTS_DIR = obj
MOC_DIR = obj
LIBS += -L../lib -lpanopticum
PKGCONFIG += raptor2 redland

# Input
HEADERS += include/graph.hh \
					 include/window.hh \
					 include/bgl.hh \
					 include/callgraph.hh \
					 include/cflowgraph.hh \
					 include/scene.hh \
					 include/model.hh
SOURCES += src/main.cc \
					 src/graph.cc \
					 src/bgl.cc \
					 src/window.cc \
					 src/callgraph.cc \
					 src/cflowgraph.cc \
					 src/scene.cc \
					 src/model.cc
