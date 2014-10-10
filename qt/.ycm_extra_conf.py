import os;

def FlagsForFile(a):
	dname = os.path.dirname(os.path.abspath(__file__ ))
	return {'flags': ['-Wall','-std=c++11','-x','c++','-I/usr/include/libarchive','-I/usr/include/qt5/QtConcurrent','-I/usr/include/qt5/QtGui','-I/usr/include/qt5/QtCore','-I/usr/include/qt5/QtQml','-I',dname + "/include"], 'do_cache': True}
