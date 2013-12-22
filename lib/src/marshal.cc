#include <iostream>
#include <sstream>
#include <algorithm>

extern "C" {
#include <minizip/zip.h>
#include <minizip/unzip.h>
#include <dirent.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
}

#include <panopticon/marshal.hh>

using namespace po;
using namespace std;
using namespace boost;

marshal_exception::marshal_exception(const string &w)
: runtime_error(w)
{}

std::unique_ptr<rdf::world> rdf::world::_instance = nullptr;

rdf::world::world(void)
: enable_shared_from_this(), _rdf_world(nullptr), _rap_world(nullptr)
{
	assert(!_instance);

	_rdf_world = librdf_new_world();
	librdf_world_open(_rdf_world);
	_rap_world = librdf_world_get_raptor(_rdf_world);
}

rdf::world::~world() {}

rdf::world& rdf::world::instance(void)
{
	if(!_instance)
		_instance.reset(new world());
	return *_instance;
}

librdf_world *rdf::world::rdf(void) const
{
	return _rdf_world;
}

raptor_world *rdf::world::raptor(void) const
{
	return _rap_world;
}

rdf::storage rdf::storage::from_archive(const string &path)
{
	storage ret(false);
	const string &tempDir = ret._tempdir;

	// open target zip
	unzFile zf = unzOpen(path.c_str());
	if(zf == NULL)
		throw marshal_exception("can't open " + path);

	if(unzLocateFile(zf,"graph-po2s.db",0) != UNZ_OK ||
		 unzLocateFile(zf,"graph-so2p.db",0) != UNZ_OK ||
		 unzLocateFile(zf,"graph-sp2o.db",0) != UNZ_OK)
		throw marshal_exception("can't open " + path + ": no graph database in file");

	if(unzGoToFirstFile(zf) != UNZ_OK)
		throw marshal_exception("can't open " + path);

	// copy database files to tempdir
	char *buf = new char[4096];

	do
	{
		char fileName[256];

		if(unzGetCurrentFileInfo(zf,NULL,fileName,256,NULL,0,NULL,0) != UNZ_OK)
			throw marshal_exception("can't read files from " + path);

		string tmpName = tempDir + "/" + string(fileName);
		int fd = open(tmpName.c_str(),O_WRONLY | O_CREAT, S_IRUSR | S_IWUSR);

		if(fd < 0)
			throw marshal_exception("can't open " + path + " into tempdir: " + strerror(errno));

		if(unzOpenCurrentFile(zf) != UNZ_OK)
			throw marshal_exception("can't open file in " + path);

		size_t sz;
		while((sz = unzReadCurrentFile(zf,buf,4096)) != 0)
		{
			size_t p = 0;
			while(p < sz)
				p += write(fd,buf + p,sz - p);
		}

		if(close(fd) || unzCloseCurrentFile(zf) != UNZ_OK)
			throw marshal_exception("can't open " + path);

		cout << "read " << tmpName << " from " << path << endl;
	}
	while(unzGoToNextFile(zf) == UNZ_OK);

	if(unzClose(zf) != UNZ_OK)
		throw marshal_exception("can't open " + path);

	world &w = world::instance();
	assert(ret._storage = librdf_new_storage(w.rdf(),"hashes","graph",string("hash-type='bdb',dir='" + ret._tempdir + "'").c_str()));
	assert(ret._model = librdf_new_model(w.rdf(),ret._storage,NULL));

	return ret;
}

rdf::storage rdf::storage::from_turtle(const string &path)
{
	world &w = world::instance();
	storage ret;
	librdf_parser *parser = nullptr;
	librdf_uri *uri = nullptr;

	assert(parser = librdf_new_parser(w.rdf(),"turtle",NULL,NULL));
	assert(uri = librdf_new_uri_from_filename(w.rdf(),path.c_str()));
	assert(!librdf_parser_parse_into_model(parser,uri,uri,ret._model));

	librdf_free_uri(uri);
	librdf_free_parser(parser);

	return ret;
}

rdf::storage::storage(void)
: storage(true)
{}

rdf::storage::storage(bool openStore)
: _storage(nullptr), _model(nullptr), _tempdir("")
{
	char *tmp = new char[TEMPDIR_TEMPLATE.size() + 1];

	strncpy(tmp,TEMPDIR_TEMPLATE.c_str(),TEMPDIR_TEMPLATE.size() + 1);
	tmp = mkdtemp(tmp);

	_tempdir = string(tmp);
	delete[] tmp;

	if(openStore)
	{
		world &w = world::instance();
		assert(_storage = librdf_new_storage(w.rdf(),"hashes","graph",string("new='yes',hash-type='bdb',dir='" + _tempdir + "'").c_str()));
		assert(_model = librdf_new_model(w.rdf(),_storage,NULL));
	}
}

rdf::storage::storage(rdf::storage &&store)
: _storage(store._storage), _model(store._model), _tempdir(store._tempdir)
{}

rdf::storage::~storage(void)
{
	librdf_free_model(_model);
	librdf_free_storage(_storage);

	std::function<void(const string &path)> rm_r;
	rm_r = [&](const string &path)
	{
		// open dir
		DIR *dirDesc = opendir(path.c_str());
		if(!dirDesc)
			throw marshal_exception("can't delete " + path + ": " + strerror(errno));

		// delete contents
		struct dirent *dirEnt;
		struct stat st;

		while((dirEnt = readdir(dirDesc)))
		{
			string ent(dirEnt->d_name);
			string cur = path + "/" + ent;

			if(ent == "." || ent == "..")
				continue;

			if(stat(cur.c_str(),&st))
				throw marshal_exception("can't stat " + path + "/" + cur + ": " + strerror(errno));

			if(S_ISDIR(st.st_mode))
				rm_r(cur);
			else
				if(unlink(cur.c_str()))
					throw marshal_exception("can't unlink " + path + "/" + cur + ": " + strerror(errno));
		}

		if(closedir(dirDesc))
			throw marshal_exception("can't close directory " + path);

		if(rmdir(path.c_str()))
			throw marshal_exception("can't delete directory " + path);
	};

	try
	{
		rm_r(_tempdir);
	}
	catch(const marshal_exception &e)
	{
		cerr << "Exception in rdf::storage::~storage: " << e.what() << endl;
	}
}

void rdf::storage::snapshot(const string &path)
{
	if(path.empty())
		return;

	// delete existing `path'
	unlink(path.c_str());

	// sync bdb
	/// XXX: lock store against modifications
	if(librdf_storage_sync(_storage))
		throw marshal_exception("can't sync triple store");

	// open temp dir
	DIR *dirDesc = opendir(_tempdir.c_str());
	if(!dirDesc)
		throw marshal_exception("can't save to " + path + ": " + strerror(errno));

	// open target zip
	zipFile zf = zipOpen(path.c_str(),0);
	if(zf == NULL)
		throw marshal_exception("can't save to " + path + ": failed to open");

	// save database files
	struct dirent *dirEnt;
	char buf[4096];

	while((dirEnt = readdir(dirDesc)))
	{
		string entBase(dirEnt->d_name);
		string entPath = _tempdir + "/" + entBase;

		if(entBase == "." || entBase == "..")
			continue;

		int fd = open(entPath.c_str(),O_RDONLY);
		zip_fileinfo zif;

		memset(&zif,0,sizeof(zip_fileinfo));

		if(fd < 0)
			throw marshal_exception("can't save to " + path + ": " + strerror(errno));

		if(zipOpenNewFileInZip(zf,entBase.c_str(),&zif,NULL,0,NULL,0,NULL,Z_DEFLATED,Z_DEFAULT_COMPRESSION) != ZIP_OK)
			throw marshal_exception("can't save to " + path + ": " + strerror(errno));

		int ret;
		do
		{
			ret = read(fd,buf,4096);

			if(ret < 0)
				throw marshal_exception("can't save to " + path + ": error while reading " + entPath + "(" + strerror(errno) + ")");

			if(ret > 0 && zipWriteInFileInZip(zf,buf,ret) != ZIP_OK)
				throw marshal_exception("can't save to " + path + ": error while writing " + entPath);
		}
		while(ret);

		if(close(fd) || zipCloseFileInZip(zf) != ZIP_OK)
			throw marshal_exception("can't save to " + path + ": failed to close file descriptor");

		cout << "written " << entPath << " in " << path << endl;
	}

	if(closedir(dirDesc) || zipClose(zf,NULL) != ZIP_OK)
		throw marshal_exception("can't save to " + path + ": failed to close directory");
}

rdf::stream rdf::storage::select(optional<const rdf::node&> s, optional<const rdf::node&> p, optional<const rdf::node&> o) const
{
	if(s && p && o)
		return rdf::stream(librdf_model_find_statements(_model,statement(*s,*p,*o).inner()));
	else if(s && p && !o)
		return rdf::stream(librdf_model_get_targets(_model,s->inner(),p->inner()),s,p,o);
	else if(s && !p && o)
		return rdf::stream(librdf_model_get_arcs(_model,s->inner(),o->inner()),s,p,o);
	else if(s && !p && !o)
		throw std::invalid_argument("invalid query");
	else if(!s && p && o)
		return rdf::stream(librdf_model_get_sources(_model,o->inner(),p->inner()),s,p,o);
	else if(!s && p && !o)
		throw std::invalid_argument("invalid query");
	else if(!s && !p && o)
		throw std::invalid_argument("invalid query");
	else
		return rdf::stream(librdf_model_as_stream(_model));
}

rdf::statement rdf::storage::first(optional<const rdf::node&> s,optional<const rdf::node&> p,optional<const rdf::node&> o) const
{
	rdf::stream st = select(s,p,o);

	if(st.eof())
		throw marshal_exception("no matching rdf statement");

	statement ret;
	st >> ret;

	return ret;
}

bool rdf::storage::has(optional<const rdf::node&> s,optional<const rdf::node&> p,optional<const rdf::node&> o) const
{
	return !select(s,p,o).eof();
}

void rdf::storage::insert(const rdf::statement &st)
{
	if(librdf_model_add_statement(_model,st.inner()))
		throw marshal_exception("failed to add statement");
}

void rdf::storage::insert(const rdf::node& s, const rdf::node& p, const rdf::node& o)
{
	if(librdf_model_add(_model,librdf_new_node_from_node(s.inner()),
															librdf_new_node_from_node(p.inner()),
															librdf_new_node_from_node(o.inner())))
		throw marshal_exception("failed to add statement");
}

void rdf::storage::remove(const rdf::statement &st)
{
	if(librdf_model_remove_statement(_model,st.inner()))
		throw marshal_exception("failed to remove statement");
}

string rdf::storage::dump(const std::string &format) const
{
	unsigned char *raw = librdf_model_to_string(_model,NULL,NULL,format.c_str(),NULL);

	if(raw == NULL)
		throw marshal_exception("failed to dump in '" + format + "' format");

	string ret(reinterpret_cast<const char*>(raw));
	free(raw);

	return ret;
}

rdf::node::node(void)
: _node(librdf_new_node_from_blank_identifier(world::instance().rdf(),NULL))
{}

rdf::node::node(librdf_node *n)
: _node(n)
{}

rdf::node::node(const rdf::node &n)
: _node(librdf_new_node_from_node(n._node))
{}

rdf::node::node(rdf::node &&n)
: _node(n._node)
{
	n._node = nullptr;
}

rdf::node::~node(void)
{
	if(_node)
		librdf_free_node(_node);
}

rdf::node &rdf::node::operator=(const rdf::node &n)
{
	if(_node)
		librdf_free_node(_node);
	_node = n._node ? librdf_new_node_from_node(n._node) : nullptr;

	return *this;
}

rdf::node &rdf::node::operator=(rdf::node &&n)
{
	_node = n._node;
	n._node = nullptr;

	return *this;
}

bool rdf::node::operator==(const rdf::node &n) const
{
	return librdf_node_equals(inner(),n.inner());
}

bool rdf::node::operator!=(const rdf::node &n) const
{
	return !(*this == n);
}

string rdf::node::to_string(void) const
{
	if(!_node)
		return string("NULL");
	else if(librdf_node_is_literal(_node))
		return string(reinterpret_cast<const char *>((librdf_node_get_literal_value(_node))));
	else if(librdf_node_is_resource(_node))
		return string(reinterpret_cast<const char *>(librdf_uri_as_string(librdf_node_get_uri(_node))));
	else if(librdf_node_is_blank(_node))
		return string(reinterpret_cast<const char *>(librdf_node_get_blank_identifier(_node)));
	else
		throw marshal_exception("unknown node type");
}

librdf_node *rdf::node::inner(void) const
{
	return _node;
}

std::ostream& rdf::operator<<(std::ostream &os, const rdf::node &n)
{
	os << n.to_string();
	return os;
}

rdf::node po::rdf::lit(const std::string &s)
{
	rdf::world &w = rdf::world::instance();
	librdf_uri *type = librdf_new_uri(w.rdf(),reinterpret_cast<const unsigned char *>(XSD"string"));
	rdf::node ret(librdf_new_node_from_typed_literal(w.rdf(),reinterpret_cast<const unsigned char *>(s.c_str()),NULL,type));

	librdf_free_uri(type);
	return ret;
}

rdf::node po::rdf::lit(unsigned long long n)
{
	rdf::world &w = rdf::world::instance();
	librdf_uri *type = librdf_new_uri(w.rdf(),reinterpret_cast<const unsigned char *>(XSD"nonNegativeInteger"));
	rdf::node ret(librdf_new_node_from_typed_literal(w.rdf(),reinterpret_cast<const unsigned char *>(std::to_string(n).c_str()),NULL,type));

	librdf_free_uri(type);
	return ret;
}

rdf::node po::rdf::ns_po(const std::string &s)
{
	return rdf::node(librdf_new_node_from_uri_string(rdf::world::instance().rdf(),
																									 reinterpret_cast<const unsigned char *>((std::string(PO) + s).c_str())));
}

rdf::node po::rdf::ns_rdf(const std::string &s)
{
	return rdf::node(librdf_new_node_from_uri_string(rdf::world::instance().rdf(),
																									 reinterpret_cast<const unsigned char *>((std::string(RDF) + s).c_str())));
}

rdf::node po::rdf::ns_xsd(const std::string &s)
{
	return rdf::node(librdf_new_node_from_uri_string(rdf::world::instance().rdf(),
																									 reinterpret_cast<const unsigned char *>((std::string(XSD) + s).c_str())));
}

rdf::node po::rdf::ns_local(const std::string &s)
{
	return rdf::node(librdf_new_node_from_uri_string(rdf::world::instance().rdf(),
																									 reinterpret_cast<const unsigned char *>((std::string(LOCAL) + s).c_str())));
}

rdf::statement::statement(const rdf::node &s, const rdf::node &p, const rdf::node &o)
: _statement(librdf_new_statement_from_nodes(world::instance().rdf(),
																							s.inner() ? librdf_new_node_from_node(s.inner()) : NULL,
																							p.inner() ? librdf_new_node_from_node(p.inner()) : NULL,
																							o.inner() ? librdf_new_node_from_node(o.inner()) : NULL))
{}

rdf::statement::statement(librdf_statement *n)
: _statement(n)
{}

rdf::statement::statement(const rdf::statement &n)
: _statement(librdf_new_statement_from_statement(n._statement))
{}

rdf::statement::statement(rdf::statement &&n)
: _statement(n._statement)
{
	n._statement = nullptr;
}

rdf::statement::~statement(void)
{
	if(_statement)
		librdf_free_statement(_statement);
}

rdf::statement &rdf::statement::operator=(const rdf::statement &n)
{
	if(_statement)
		librdf_free_statement(_statement);
	_statement = n._statement ? librdf_new_statement_from_statement(n._statement) : nullptr;

	return *this;
}

rdf::statement &rdf::statement::operator=(rdf::statement &&n)
{
	_statement = n._statement;
	n._statement = nullptr;

	return *this;
}

rdf::node rdf::statement::subject(void) const
{
	if(!_statement || !librdf_statement_get_subject(_statement))
		throw marshal_exception("can't get subject");

	return node(librdf_new_node_from_node(librdf_statement_get_subject(_statement)));
}

rdf::node rdf::statement::predicate(void) const
{
	if(!_statement || !librdf_statement_get_predicate(_statement))
		throw marshal_exception("can't get predicate");

	return node(librdf_new_node_from_node(librdf_statement_get_predicate(_statement)));
}

rdf::node rdf::statement::object(void) const
{
	if(!_statement || !librdf_statement_get_object(_statement))
		throw marshal_exception("can't get object");

	return node(librdf_new_node_from_node(librdf_statement_get_object(_statement)));
}

librdf_statement *rdf::statement::inner(void) const
{
	return _statement;
}

ostream& rdf::operator<<(ostream &os, const rdf::statement &s)
{
	os << "(" << s.subject() << ", " << s.predicate() << ", " << s.object() << ")";
	return os;
}

rdf::stream::stream(librdf_stream *n)
: _stream(n)
{}

rdf::stream::stream(librdf_iterator *i, boost::optional<const rdf::node&> s, boost::optional<const rdf::node&> p, boost::optional<const rdf::node&> o)
: _stream(NULL)
{
	if(!s && p && o)
		_stream = librdf_new_stream_from_node_iterator(i,statement(node(NULL),*p,*o).inner(),LIBRDF_STATEMENT_SUBJECT);
	else if(s && !p && o)
		_stream = librdf_new_stream_from_node_iterator(i,statement(*s,node(NULL),*o).inner(),LIBRDF_STATEMENT_PREDICATE);
	else if(s && p && !o)
		_stream = librdf_new_stream_from_node_iterator(i,statement(*s,*p,node(NULL)).inner(),LIBRDF_STATEMENT_OBJECT);
	else
		throw marshal_exception("invalid statement template");
	//librdf_free_iterator(i);
}

rdf::stream::stream(rdf::stream &&n)
: _stream(n._stream)
{
	n._stream = nullptr;
}

rdf::stream::~stream(void)
{
	if(_stream)
		librdf_free_stream(_stream);
}

rdf::stream &rdf::stream::operator>>(rdf::statement &st)
{
	if(eof())
		throw marshal_exception("stream at eof");

	st = statement(librdf_new_statement_from_statement(librdf_stream_get_object(_stream)));
	librdf_stream_next(_stream);

	return *this;
}

bool rdf::stream::eof(void) const
{
	return _stream && librdf_stream_end(_stream) != 0;
}

rdf::nodes rdf::read_list(const rdf::node &n, const rdf::storage &store)
{
	nodes ret;
	node cur = n;

	while(cur != "nil"_rdf)
	{
		statement s = store.first(cur,"first"_rdf,none);

		ret.push_back(s.object());
		cur = store.first(cur,"rest"_rdf,none).object();
	}

	return ret;
}
