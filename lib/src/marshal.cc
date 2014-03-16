#include <iostream>
#include <sstream>
#include <algorithm>

extern "C" {
#include <archive.h>
#include <archive_entry.h>
#include <dirent.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
}

#include <boost/uuid/uuid_generators.hpp>
#include <boost/uuid/uuid_io.hpp>

#include <panopticon/marshal.hh>

using namespace po;
using namespace std;
using namespace rdf;
using namespace boost;

marshal_exception::marshal_exception(const string &w)
: runtime_error(w)
{}

/*rdf::storage rdf::storage::from_archive(const string &path)
{
	storage ret(false);
	const string &tempDir = ret._tempdir;

	// open target zip
	archive *ar = archive_read_new();
	if(ar == NULL)
		throw marshal_exception("can't allocate archive struct");

	if(archive_read_support_format_cpio(ar) != ARCHIVE_OK)
		throw marshal_exception("can't set archive format");

	if(archive_read_support_filter_lzma(ar) != ARCHIVE_OK)
		throw marshal_exception("can't set compression algorithm");

	try
	{
		if(archive_read_open_filename(ar,path.c_str(),4096) != ARCHIVE_OK)
			throw marshal_exception("can't open " + path);

		// XXX: depends on internals of the berkeley model of librdf
		bool found_po2s = false, found_so2p = false, found_sp2o = false;

		// copy database files to tempdir
		struct archive_entry *ae;

		while(archive_read_next_header(ar,&ae) == ARCHIVE_OK)
		{
			string pathName(archive_entry_pathname(ae));
			string tmpName = tempDir + "/" + pathName;

			found_po2s = found_po2s | (pathName.substr(pathName.size() - 13,std::string::npos) == "graph-po2s.db");
			found_so2p = found_so2p | (pathName.substr(pathName.size() - 13,std::string::npos) == "graph-so2p.db");
			found_sp2o = found_sp2o | (pathName.substr(pathName.size() - 13,std::string::npos) == "graph-sp2o.db");
			int fd = open(tmpName.c_str(),O_WRONLY | O_CREAT, S_IRUSR | S_IWUSR);

			if(fd < 0 || archive_read_data_into_fd(ar,fd) != ARCHIVE_OK || close(fd))
					throw marshal_exception("can't open " + path + " into tempdir: " + strerror(errno));

			cout << "read " << tmpName << " from " << path << endl;
		}

		if(!(found_po2s && found_so2p && found_sp2o))
			throw marshal_exception("can't open " + path + ": no graph database in file");
	}
	catch(...)
	{
		archive_read_free(ar);
		throw;
	}

	if(archive_read_free(ar) != ARCHIVE_OK)
		throw marshal_exception("can't open " + path);

	world &w = world::instance();
	assert(ret._storage = librdf_new_storage(w.rdf(),"hashes","graph",string("hash-type='bdb',dir='" + ret._tempdir + "'").c_str()));
	assert(ret._model = librdf_new_model(w.rdf(),ret._storage,NULL));

	return ret;
}

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
	struct archive *ar = archive_write_new();
	if(ar == NULL)
		throw marshal_exception("can't save to " + path + ": failed to allocate archive struct");

	try
	{
		if(archive_write_add_filter_lzma(ar) != ARCHIVE_OK)
			throw marshal_exception("can't save to " + path + ": failed setting compression algorithm");

		// save into *.cpio.lzma
		if(archive_write_set_format_cpio(ar) != ARCHIVE_OK)
			throw marshal_exception("can't save to " + path + ": failed setting archive format");

		if(archive_write_open_filename(ar,path.c_str()) != ARCHIVE_OK)
			throw marshal_exception("can't save to " + path + ": failed to open");

		// save database files
		struct dirent *dirEnt;
		char buf[4096];
		struct archive_entry *ae = archive_entry_new();

		if(!ae)
			throw marshal_exception("can't save to " + path + ": failed to allocate archive entry struct");

		try
		{
			while((dirEnt = readdir(dirDesc)))
			{
				string entBase(dirEnt->d_name);
				string entPath = _tempdir + "/" + entBase;

				if(entBase == "." || entBase == "..")
					continue;

				int fd = open(entPath.c_str(),O_RDONLY);
				if(fd < 0)
					throw marshal_exception("can't save to " + path + ": " + strerror(errno));

				try
				{
					struct stat st;

					fstat(fd,&st);
					archive_entry_clear(ae);
					archive_entry_copy_pathname(ae,entBase.c_str());
					archive_entry_copy_stat(ae,&st);

					if(archive_write_header(ar,ae) != ARCHIVE_OK)
						throw marshal_exception("can't save to " + path + ": failed to write header");

					int ret;
					do
					{
						ret = read(fd,buf,4096);

						if(ret < 0)
							throw marshal_exception("can't save to " + path + ": error while reading " + entPath + "(" + strerror(errno) + ")");

						if(ret > 0 && archive_write_data(ar,buf,ret) != ARCHIVE_OK)
							throw marshal_exception("can't save to " + path + ": error while writing " + entPath);
					}
					while(ret);
				}
				catch(...)
				{
					close(fd);
					throw;
				}

				if(close(fd))
					throw marshal_exception("can't save to " + path + ": failed to close file descriptor");

				cout << "written " << entPath << " in " << path << endl;
			}
		}
		catch(...)
		{
			archive_entry_free(ae);
			throw;
		}

		archive_entry_free(ae);
	}
	catch(...)
	{
		closedir(dirDesc);
		archive_write_free(ar);

		throw;
	}

	if(closedir(dirDesc) || archive_write_free(ar) != ARCHIVE_OK)
		throw marshal_exception("can't save to " + path + ": failed to close directory");
}*/

node node::blank(void) { return node(boost::uuids::random_generator()()); }

node::node(const iri& n) : _inner(n) {}
node::node(const string& s, const iri& t) : _inner(make_pair(s,t)) {}
node::node(const uuid& u) : _inner(u) {}

bool node::is_iri(void) const { return !!get<iri>(&_inner); }
bool node::is_literal(void) const { return !!get<pair<string,iri>>(&_inner); }
bool node::is_blank(void) const { return !!get<uuid>(&_inner); }

const iri& node::as_iri(void) const { return get<iri>(_inner); }
const iri& node::as_literal(void) const { return get<pair<string,iri>>(_inner).first; }
const iri& node::literal_type(void) const { return get<pair<string,iri>>(_inner).second; }
const uuid& node::as_uuid(void) const { return get<uuid>(_inner); }

bool node::operator==(const node& n) const
{
	return _inner == n._inner;
}

bool node::operator<(const node& n) const
{
	return _inner < n._inner;
}

statement::statement(const node& s, const node& p, const node& o)
: subject(s), predicate(p), object(o) {}

bool statement::operator==(const statement& st) const
{
	return subject == st.subject &&
				 predicate == st.predicate &&
				 object == st.object;
}

bool statement::operator<(const statement& st) const
{
	return subject < st.subject ||
				 (subject == st.subject && predicate < st.predicate) ||
				 (subject == st.subject && predicate == st.predicate && object < st.object);
}

storage::storage(void)
: _meta()
{
	if(!_meta.open("+",PolyDB::OWRITER | PolyDB::OCREATE))
		throw marshal_exception("can't open database");
}

storage::storage(const string& base)
: _meta()
{
	if(!_meta.open(base + "meta.kct",PolyDB::OWRITER | PolyDB::OCREATE))
		throw marshal_exception("can't open database");
}

storage::~storage(void)
{
	_meta.close();
}

bool storage::has(const node& s, const node& p, const node& o) const
{
	return has(statement(s,p,o));
}

bool storage::has(const statement& st) const
{
	return _meta.check(encode_key(st)) > -1;
}

list<statement> storage::find(const node &sub, const node &pred) const
{
	list<statement> ret;
	vector<string> keys;
	string s = encode_node(sub), p = encode_node(pred);

	_meta.match_prefix(encode_varint(s.size()) + s + encode_varint(p.size()) + p,&keys);
	transform(keys.begin(),keys.end(),inserter(ret,ret.begin()),[&](const string &k) { return decode_key(k.begin(),k.end()).first; });

	return ret;
}

list<statement> storage::find(const node &sub) const
{
	list<statement> ret;
	vector<string> keys;
	string s = encode_node(sub);

	_meta.match_prefix(encode_varint(s.size()) + s,&keys);
	transform(keys.begin(),keys.end(),inserter(ret,ret.begin()),[&](const string &k) { return decode_key(k.begin(),k.end()).first; });

	return ret;
}

statement storage::first(const node &s, const node &p) const
{
	statements st = find(s,p);

	if(st.size() > 0)
		return st.front();
	else
		throw marshal_exception("no statement found");
}

int64_t storage::count(void) const
{
	return _meta.count();
}

bool storage::insert(const node& s, const node& p, const node& o)
{
	return insert(statement(s,p,o));
}

bool storage::insert(const statement& st)
{
	if(has(st))
		return false;

	_meta.set(encode_key(st),"");
	return true;
}

bool storage::remove(const node& s, const node& p, const node& o)
{
	return remove(statement(s,p,o));
}

bool storage::remove(const statement& st)
{
	return _meta.remove(encode_key(st));
}

string storage::encode_node(const node& n)
{
	if(n.is_iri())
		return string(1,Named) + n.as_iri();
	else if(n.is_literal())
		return string(1,Literal) + encode_varint(n.as_literal().size()) + n.as_literal() + n.literal_type();
	else if(n.is_blank())
		return string(1,Blank) + to_string(n.as_uuid());
	else
		throw marshal_exception("unknown node type");
}

std::pair<node,storage::iter> storage::decode_node(iter b, iter e)
{
	switch(static_cast<node_type>(*b))
	{
		case Named:
			return make_pair(node(iri(string(next(b),e))),e);
		case Literal:
		{
			pair<size_t,iter> len = decode_varint(next(b),e);
			string lit(len.second,next(len.second,len.first));
			string ty(next(len.second,len.first),e);
			return make_pair(node(lit,ty),e);
		}
		case Blank:
		{
			boost::uuids::string_generator s;
			return make_pair(node(s(string(next(b),e))),e);
		}
		default:
			throw marshal_exception("unknown node type");
	}
}

string storage::encode_key(const statement& st)
{
	string s = encode_node(st.subject), p = encode_node(st.predicate), o = encode_node(st.object);
	return encode_varint(s.size()) + s + encode_varint(p.size()) + p + encode_varint(o.size()) + o;
}

pair<statement,storage::iter> storage::decode_key(iter b, iter e)
{
	pair<size_t,iter> s_sz = decode_varint(b,e);
	pair<node,iter> s = decode_node(s_sz.second,next(s_sz.second,s_sz.first));
	pair<size_t,iter> p_sz = decode_varint(s.second,e);
	pair<node,iter> p = decode_node(p_sz.second,next(p_sz.second,p_sz.first));
	pair<size_t,iter> o_sz = decode_varint(p.second,e);
	pair<node,iter> o = decode_node(o_sz.second,next(o_sz.second,o_sz.first));

	return make_pair(statement(s.first,p.first,o.first),o.second);
}

string storage::encode_varint(size_t sz)
{
	string tmp;

	while(sz)
	{
		tmp.push_back(sz & 0x7f);
		sz >>= 7;
	}

	string ret;
	auto i = tmp.rbegin();
	while(i != tmp.rend())
	{
		ret.push_back(*i | (next(i) == tmp.rend() ? 0 : 0x80));
		++i;
	}

	return ret;
}

pair<size_t,storage::iter> storage::decode_varint(iter b, iter e)
{
	size_t ret = 0;
	unsigned int x = 0;

	assert(b != e);
	while(b != e)
	{
		x = static_cast<unsigned int>(*b++);
		ret = (ret << 7) | (x & 0x7f);
		if(!(x & 0x80))
			break;
	}

	return make_pair(ret,b);
}

nodes read_list(const node &n, const storage &store)
{
	nodes ret;
	node cur = n;

	while(cur != "nil"_rdf)
	{
		statement s = store.first(cur,"first"_rdf);

		ret.push_back(s.object);
		cur = store.first(cur,"rest"_rdf).object;
	}

	return ret;
}
