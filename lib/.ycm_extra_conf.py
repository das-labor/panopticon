import os;

def FlagsForFile(a):
	dname = os.path.dirname(os.path.abspath(__file__ ))
	return {'flags': ['-Wall','-std=c++11','-x','c++','-I/usr/include/libarchive','-I',dname + "include"], 'do_cache': True}
