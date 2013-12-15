import os;

def FlagsForFile(a):
	dname = os.path.dirname(os.path.abspath(__file__ ))
	return {'flags': ['-Wall','-std=c++11','-x','c++','-I/usr/include/rasqal','-I/usr/include/raptor2','-I/usr/include/libxml2','-I/usr/include/minizip','-I',dname], 'do_cache': True}
