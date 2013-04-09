SELF_DIR=$(dir $(lastword $(MAKEFILE_LIST)))

CXX=clang++
CXXARGS=-Wall -g -std=c++11 -fPIC -pipe -pedantic -Werror -Weffc++ -Wno-deprecated
LD=clang++
PKGCONFIG=redland raptor2 minizip
LIBS=`pkg-config $(PKGCONFIG) --libs` -lcvc4
LDFLAGS=-L$(SELF_DIR)/lib
INCLUDES=-I$(SELF_DIR)/lib/include
