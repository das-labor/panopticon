import os;

def FlagsForFile(a):
	dname = os.path.dirname(os.path.abspath(__file__ ))
	return {'flags': ['-Wall','-std=c++11','-x','c++','-I..//lib/include','-I/usr/include/rasqal','-I/usr/include/raptor2','-I/usr/include/minizip','-I',os.path.join(dname,'include')], 'do_cache': True}
