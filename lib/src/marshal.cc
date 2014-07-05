#include <iostream>
#include <sstream>
#include <algorithm>

extern "C" {
#include <archive.h>
#include <archive_entry.h>

// stat(), open(), mmap()
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <sys/mman.h>
#include <stdio.h>
}

#include <boost/uuid/uuid_generators.hpp>
#include <boost/uuid/uuid_io.hpp>

#include <panopticon/marshal.hh>

using namespace po;
using namespace std;
using namespace rdf;
using namespace boost;
using namespace filesystem;

std::mt19937 uuid::prng;
boost::uuids::basic_random_generator<std::mt19937> uuid::generator(&uuid::prng);

mapped_file::mapped_file(const boost::filesystem::path& p, const uuid& t)
: _size(file_size(p)), _fd(open(p.string().c_str(),O_RDONLY)), _data(nullptr), _tag(t), _path(p), _reference(new std::atomic<unsigned long long>())
{
	if(_fd < 0)
		throw std::runtime_error("Can't create mapping for " + p.string());
	_data = (char*)mmap(NULL,_size,PROT_READ,MAP_PRIVATE,_fd,0);
	if(!_data)
		throw std::runtime_error("Can't create mapping for " + p.string());

	++(*_reference);
}

mapped_file::mapped_file(const mapped_file& f)
: _size(f._size), _fd(f._fd), _data(f._data), _tag(f._tag), _path(f._path), _reference(f._reference)
{
	++(*_reference);
}

mapped_file::~mapped_file(void)
{
	if(--(*_reference) == 0)
	{
		munmap(_data,_size);
		close(_fd);
		delete _reference;
	}
}

marshal_exception::marshal_exception(const string &w)
: runtime_error(w)
{}

iri::iri(const std::string& i) : _iri(i) {}
iri::iri(const uuid& u) : _iri("urn:uuid:" + to_string(u)) {}

bool iri::operator==(const iri& i) const { return _iri == i._iri; }
bool iri::operator!=(const iri& i) const { return _iri != i._iri; }
bool iri::operator<(const iri& i) const { return _iri < i._iri; }

bool iri::is_uuid(void) const { return _iri.substr(0,9) == "urn:uuid:"; }

uuid iri::as_uuid(void) const
{
	if(!is_uuid())
		throw marshal_exception("not a uuid");
	return uuids::string_generator{}(_iri.substr(9));
}

const std::string& iri::as_string(void) const { if(is_uuid()) throw std::runtime_error("BUG: thats a uuid!"); return _iri; }
const std::string& iri::raw(void) const { return _iri; }

node node::blank(void) { return node(uuid::generator()); }

node::node(const iri& n) : _inner(n) {}
node::node(const string& s, const iri& t) : _inner(make_pair(s,t)) {}
node::node(const uuid& u) : _inner(u) {}

bool node::is_iri(void) const { return !!get<iri>(&_inner); }
bool node::is_literal(void) const { return !!get<pair<string,iri>>(&_inner); }
bool node::is_blank(void) const { return !!get<uuid>(&_inner); }

const iri& node::as_iri(void) const { return get<iri>(_inner); }
std::string node::as_literal(void) const { return get<pair<string,iri>>(_inner).first; }
const iri& node::literal_type(void) const { return get<pair<string,iri>>(_inner).second; }
const uuid& node::blank_id(void) const { return get<uuid>(_inner); }

bool node::operator==(const node& n) const
{
	return _inner == n._inner;
}

bool node::operator<(const node& n) const
{
	return _inner < n._inner;
}

std::ostream& po::rdf::operator<<(std::ostream& os, const node& n)
{
	if(n.is_literal())
		os << "\"" << n.as_literal() << "\"^^" << n.literal_type();
	else if(n.is_iri())
		os << n.as_iri().raw();
	else
		os << "Blank(" << to_string(n.blank_id()) << ")";
	return os;
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

std::ostream& po::rdf::operator<<(std::ostream& os, const statement& st)
{
	os << st.subject << " " << st.predicate << " " << st.object;
	return os;
}

storage::storage(void)
: _meta(), _tempdir(unique_path(temp_directory_path() / std::string("panop-%%%%-%%%%-%%%%-%%%%")))
{
	if(!filesystem::create_directory(_tempdir) ||
		 !_meta.open((_tempdir / filesystem::path("meta.kct")).string(),PolyDB::OWRITER | PolyDB::OCREATE))
		throw marshal_exception("can't open database");
}

storage::storage(const filesystem::path& p)
: _meta(), _tempdir(unique_path(temp_directory_path() / std::string("panop-%%%%-%%%%-%%%%-%%%%")))
{
	uuids::string_generator sg;

	if(!filesystem::create_directory(_tempdir))
		throw marshal_exception("can't create temp directory " + _tempdir.string());

	// open target zip
	archive *ar = archive_read_new();
	if(ar == NULL)
		throw marshal_exception("can't allocate archive struct");

	try
	{
		if(archive_read_support_format_cpio(ar) != ARCHIVE_OK)
			throw marshal_exception("can't set archive format");

		if(archive_read_support_filter_lzma(ar) != ARCHIVE_OK)
			throw marshal_exception("can't set compression algorithm");

		if(archive_read_open_filename(ar,p.string().c_str(),4096) != ARCHIVE_OK)
			throw marshal_exception("can't open " + p.string());

		bool found_meta = false;

		// copy database files to tempdir
		struct archive_entry *ae;

		while(archive_read_next_header(ar,&ae) == ARCHIVE_OK)
		{
			filesystem::path pathName(archive_entry_pathname(ae));
			filesystem::path tmpName = _tempdir / pathName;

			ofstream of(tmpName.string(), ios_base::binary | ios_base::trunc | ios_base::out);
			char buf[4096];
			size_t len;

			if(!of)
				throw marshal_exception("can't open " + pathName.string());

			while((len = archive_read_data(ar,buf,4096)) > 0)
				of.write(buf,len);

			of.close();


			if(tmpName.filename() == filesystem::path("meta.kct"))
				found_meta = true;
			else
			{
				std::cerr << tmpName.filename().string() << std::endl;
				register_blob(mapped_file(tmpName,sg(tmpName.filename().string())));
			}
		}

		if(!(found_meta))
			throw marshal_exception("can't open " + p.string() + ": no graph database in file");
	}
	catch(...)
	{
		archive_read_free(ar);
		throw;
	}

	if(archive_read_free(ar) != ARCHIVE_OK)
		throw marshal_exception("can't open " + p.string());

	if(!_meta.open((_tempdir / filesystem::path("meta.kct")).string(),PolyDB::OWRITER | PolyDB::OCREATE))
		throw marshal_exception("can't open database");
}

storage::storage(const storage& st)
: _meta(), _tempdir(unique_path(temp_directory_path() / std::string("panop-%%%%-%%%%-%%%%-%%%%")))
{
	if(!filesystem::create_directory(_tempdir))
		throw marshal_exception("can't create temporary directory");

	st._meta.copy((_tempdir / filesystem::path("meta.kct")).string());
	if(!_meta.open((_tempdir / filesystem::path("meta.kct")).string(),PolyDB::OWRITER | PolyDB::OCREATE))
		throw marshal_exception("can't open database");
}

storage::~storage(void)
{
	_meta.close();

	if(!_tempdir.empty())
		filesystem::remove_all(_tempdir);
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

list<statement> storage::all(void) const
{
	list<statement> ret;

	kyotocabinet::DB::Cursor* cur = _meta.cursor();
	ensure(cur);
	cur->jump();

	string k,v;
  while (cur->get(&k,&v,true))
		ret.push_back(decode_key(k.begin(),k.end()).first);
  delete cur;

	return ret;
}

statement storage::first(const node &s, const node &p) const
{
	statements st = find(s,p);

	if(st.size() > 0)
		return st.front();
	else
	{
		std::stringstream ss;
		ss << "no statement found: " << s << " " << p << " *";
		throw marshal_exception(ss.str());
	}
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

void storage::snapshot(const filesystem::path& p) const
{
	if(p.empty())
		throw marshal_exception("can't save to empty path");

	// delete existing `path'
	filesystem::remove(p);

	// sync db
	if(!_meta.synchronize(false))
		throw marshal_exception("can't sync triple store");

	// open target zip
	struct archive *ar = archive_write_new();
	if(ar == NULL)
		throw marshal_exception("can't save to " + p.string() + ": failed to allocate archive struct");

	try
	{
		if(archive_write_add_filter_lzma(ar) != ARCHIVE_OK)
			throw marshal_exception("can't save to " + p.string() + ": failed setting compression algorithm");

		// save into *.cpio.lzma
		if(archive_write_set_format_cpio(ar) != ARCHIVE_OK)
			throw marshal_exception("can't save to " + p.string() + ": failed setting archive format");

		if(archive_write_open_filename(ar,p.string().c_str()) != ARCHIVE_OK)
			throw marshal_exception("can't save to " + p.string() + ": failed to open");

		// save database files
		char buf[4096];
		struct archive_entry *ae = archive_entry_new();

		if(!ae)
			throw marshal_exception("can't save to " + p.string() + ": failed to allocate archive entry struct");

		try
		{
			std::function<void(const filesystem::path&,const std::string&)> add_to_archive = [&](const filesystem::path& entPath, const std::string& n)
			{
				ifstream fi(entPath.string().c_str(),ios_base::binary | ios_base::in);
				if(!fi)
					throw marshal_exception("can't save to " + p.string() + ": " + strerror(errno) + " while opening " + entPath.string());

				struct stat st;
				stat(entPath.string().c_str(),&st);
				archive_entry_clear(ae);
				archive_entry_copy_pathname(ae,n.c_str());
				archive_entry_copy_stat(ae,&st);

				if(archive_write_header(ar,ae) != ARCHIVE_OK)
					throw marshal_exception("can't save to " + p.string() + ": failed to write header");

				while(!fi.eof())
				{
					fi.read(buf,4096);

					if(fi.gcount() && archive_write_data(ar,buf,fi.gcount()) != fi.gcount())
						throw marshal_exception("can't save to " + p.string() + ": error while reading " + entPath.string());
				}
			};

			add_to_archive(_tempdir / "meta.kct","meta.kct");

			for(auto mf: _blobs)
				add_to_archive(mf.path(),to_string(mf.tag()));
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
		archive_write_free(ar);
		throw;
	}

	if(archive_write_free(ar) != ARCHIVE_OK)
		throw marshal_exception("can't save to " + p.string() + ": failed to close directory");
}

bool storage::register_blob(const mapped_file& mf)
{
	auto i = std::find_if(_blobs.begin(),_blobs.end(),[&](const mapped_file& m) { return m.tag() == mf.tag(); });
	if(i == _blobs.end())
	{
		_blobs.push_back(mf);
		return true;
	}
	else
		return false;
}

po::mapped_file storage::fetch_blob(const uuid& u) const
{
	auto i = std::find_if(_blobs.begin(),_blobs.end(),[&](const mapped_file& mf) { return mf.tag() == u; });

	if(i == _blobs.end())
	{
		boost::filesystem::directory_iterator ent(_tempdir);

		auto j = std::find_if(ent,boost::filesystem::directory_iterator(),[&](boost::filesystem::directory_entry e) { return e.path().filename() == to_string(u); });

		if(j == boost::filesystem::directory_iterator())
			throw marshal_exception("no blob \"" + to_string(u) + "\"");

		mapped_file mf(j->path(),u);
		_blobs.push_back(mf);

		return mf;
	}
	else
		return *i;
}

string storage::encode_node(const node& n)
{
	if(n.is_iri())
		return string(1,Named) + n.as_iri().raw();
	else if(n.is_literal())
		return string(1,Literal) + encode_varint(n.as_literal().size()) + n.as_literal() + n.literal_type().raw();
	else if(n.is_blank())
		return string(1,Blank) + to_string(n.blank_id());
	else
		throw marshal_exception("unknown node type");
}

std::pair<node,storage::iter> storage::decode_node(iter b, iter e)
{
	switch(static_cast<node_type>(*b))
	{
		case Named:
			return make_pair(node(iri(string(std::next(b),e))),e);
		case Literal:
		{
			pair<size_t,iter> len = decode_varint(std::next(b),e);
			string lit(len.second,next(len.second,len.first));
			string ty(next(len.second,len.first),e);
			return make_pair(node(lit,ty),e);
		}
		case Blank:
		{
			boost::uuids::string_generator s;
			return make_pair(node(s(string(std::next(b),e))),e);
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

	if(!sz)
		return string(1,0);

	while(sz)
	{
		tmp.push_back(sz & 0x7f);
		sz >>= 7;
	}

	string ret;
	auto i = tmp.rbegin();
	while(i != tmp.rend())
	{
		ret.push_back(*i | (std::next(i) == tmp.rend() ? 0 : 0x80));
		++i;
	}

	return ret;
}

pair<size_t,storage::iter> storage::decode_varint(iter b, iter e)
{
	size_t ret = 0;
	unsigned int x = 0;

	ensure(b != e);
	while(b != e)
	{
		x = static_cast<unsigned int>(*b++);
		ret = (ret << 7) | (x & 0x7f);
		if(!(x & 0x80))
			break;
	}

	return make_pair(ret,b);
}

nodes po::rdf::read_list(const node &n, const storage &store)
{
	nodes ret;
	node cur = n;

	while(cur != rdf::ns_rdf("nil"))
	{
		statement s = store.first(cur,rdf::ns_rdf("first"));

		ret.push_back(s.object);
		cur = store.first(cur,rdf::ns_rdf("rest")).object;
	}

	return ret;
}
