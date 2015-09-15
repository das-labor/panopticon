use std::path::Path;
use std::mem;
use std::fs::File;
use std::io::{Read,Seek,SeekFrom};

use project::Project;
use region::Region;
use mnemonic::Bound;
use layer::Layer;

#[repr(C,packed)]
struct Mz {
    signature: u16,
    extra_bytes: u16,
    pages: u16,
    reloc_items: u16,
    header_size: u16,
    min_alloc: u16,
    max_alloc: u16,
    initial_ss: u16,
    initial_sp: u16,
    checksum: u16,
    initial_ip: u16,
    initial_cs: u16,
    reloc_table: u16,
    overlay: u16,
    reserved1: [u16; 4],
    oem_id: u16,
    oem_info: u16,
    reserved2: [u16; 10],
    e_lfanew: u32,
}

const MZ_MAGIC: u16 = 0x5a4d;

#[repr(C,packed)]
struct MzReloc {
    offset: u16,
    segment: u16,
}

#[repr(C,packed)]
struct Pe {
    magic: u32,
    machine: u16,
    num_section: u16,
    timestamp: u32,
    symbol_table: u32,
    num_symbols: u32,
    opthdr_size: u16,
    characteristics: u16,
}

#[repr(C,packed)]
struct PeOptional32 {
    magic: u16,
    major: u8,
    minor: u8,
    text_size: u32,
    data_size: u32,
    bss_size: u32,
    entry_point: u32,
    text_base: u32,
    data_base: u32, // w
    image_base: u32, // w
    section_align: u32,
    file_align: u32,
    os_major: u16,
    os_minor: u16,
    imgae_major: u16,
    image_minor: u16,
    subsys_major: u16,
    subsys_minor: u16,
    win32_ver: u32,
    image_size: u32,
    header_size: u32,
    checksum: u32,
    subsys: u16,
    dll_flags: u16,
    stack_reserve: u32, // w
    stack_commit: u32, // w
    heap_reserve: u32, // w
    heap_commit: u32, // w
    loader_flags: u32,
    datadir_entries: u32,
}

#[repr(C,packed)]
struct PeOptional64 {
    magic: u16,
    major: u8,
    minor: u8,
    text_size: u32,
    data_size: u32,
    bss_size: u32,
    entry_point: u32,
    text_base: u32,
    image_base: u64, // w
    section_align: u32,
    file_align: u32,
    os_major: u16,
    os_minor: u16,
    image_major: u16,
    image_minor: u16,
    subsys_major: u16,
    subsys_minor: u16,
    win32_ver: u32,
    image_size: u32,
    header_size: u32,
    checksum: u32,
    subsys: u16,
    dll_flags: u16,
    stack_reserve: u64, // w
    stack_commit: u64, // w
    heap_reserve: u64, // w
    heap_commit: u64, // w
    loader_flags: u32,
    datadir_entries: u32,
}

#[repr(C,packed)]
struct PeDataDirectory {
    rva: i32,
    size: u32,
}

#[repr(C,packed)]
struct PeSection {
    name: [u8; 8],
    virt_sz_or_phy_addr: u32,
    virt_address: u32,
    raw_sz: u32,
    raw_ptr: u32,
    reloc_ptr: u32,
    linenr_ptr: u32,
    reloc_count: u16,
    linenr_cout: u16,
    flags: u32,
}

pub fn pe(p: &Path) -> Option<Project> {
    let name = p.file_name().and_then(|x| x.to_str()).or(p.to_str()).unwrap_or("unknown pe");

    if let Some(mut fd) = File::open(p).ok() {
        // read MZ header
        const MZ_SIZE: usize = 64;
        assert_eq!(MZ_SIZE, mem::size_of::<Mz>());
        let mut mz_raw = [0; MZ_SIZE];

        if Some(MZ_SIZE) != fd.read(&mut mz_raw).ok() {
            return None;
        }

        let mz: Mz = unsafe { mem::transmute(mz_raw) };

        // check MZ signature
        if mz.signature != MZ_MAGIC {
            return None;
        }

        // read PE header
        const PE_SIZE: usize = 24;
        assert_eq!(PE_SIZE, mem::size_of::<Pe>());
        let mut pe_raw = [0; PE_SIZE];

        if Some(mz.e_lfanew as u64) != fd.seek(SeekFrom::Start(mz.e_lfanew as u64)).ok() {
            return None;
        }

        if Some(PE_SIZE) != fd.read(&mut pe_raw).ok() {
            return None;
        }

        let pe: Pe = unsafe { mem::transmute(pe_raw) };

        if pe.magic != 0x00004550 {
            return None;
        }

        match pe.machine {
            0x8664 => println!("AMD64"),
            0x014c => println!("IA32"),
            _ => return None
        }

        if pe.characteristics & 2 == 0 {
            return None;
        }

        // read optional PE header
        let mut opt_magic = [0; 2];

        if Some(2) != fd.read(&mut opt_magic).ok() {
            return None;
        }

        const PE32_SIZE: usize = 96;
        const PE64_SIZE: usize = 112;

        assert_eq!(PE32_SIZE, mem::size_of::<PeOptional32>());
        assert_eq!(PE64_SIZE, mem::size_of::<PeOptional64>());

        let (img_base,datadir_entries) = if opt_magic == [0xb,0x1] {
            let mut peopt_raw = [0; PE32_SIZE];

            if Some(PE32_SIZE) != fd.read(&mut peopt_raw).ok() {
                return None;
            }

            let peopt: PeOptional32 = unsafe { mem::transmute(peopt_raw) };
            (peopt.image_base as u64,peopt.datadir_entries)
        } else if opt_magic == [0xb,0x2] {
            let mut peopt_raw = [0; PE64_SIZE];

            if Some(PE64_SIZE) != fd.read(&mut peopt_raw).ok() {
                return None;
            }

            let peopt: PeOptional64 = unsafe { mem::transmute(peopt_raw) };
            (peopt.image_base,peopt.datadir_entries)
        } else {
            return None
        };

        // XXX: data directory

        // read sections
        const PESEC_SIZE: usize = 40;
        assert_eq!(PESEC_SIZE, mem::size_of::<PeSection>());
        let mut ram = Region::undefined("ram".to_string(),0xc0000000);

        for i in 0..pe.num_section {
            let sec_off = (mz.e_lfanew as u64) + (PE_SIZE as u64) + (PESEC_SIZE as u64) * (i as u64) + (pe.opthdr_size as u64);
            let mut sec_raw = [0; PESEC_SIZE];

            if Some(sec_off) != fd.seek(SeekFrom::Start(sec_off)).ok() {
                return None;
            }

            if Some(PESEC_SIZE) != fd.read(&mut sec_raw).ok() {
                return None;
            }

            let sec: PeSection = unsafe { mem::transmute(sec_raw) };
            let name = String::from_utf8_lossy(&sec.name);

            println!("{}",name);

            let l = if sec.raw_sz > 0 {
                let mut buf = vec![0; sec.raw_sz as usize];

                if Some(sec.raw_ptr as u64) != fd.seek(SeekFrom::Start(sec.raw_ptr as u64)).ok() {
                    return None;
                }

                if Some(sec.raw_sz as usize) != fd.read(&mut buf).ok() {
                    return None;
                }

                println!("mapped '{}'",name);
                Layer::wrap(buf.to_vec())
            } else {
                println!("not mapped '{}'",name);
                Layer::undefined(sec.virt_sz_or_phy_addr as u64)
            };

            if !ram.cover(Bound::new(img_base + (sec.virt_address as u64),img_base + (sec.virt_address as u64) + (sec.raw_sz as u64)),l) {
                return None;
            }
        }

        Some(Project::new(name.to_string(),ram))
    } else {
        None
    }
}
/*
session po::pe(const string& p)
{
	dbase_loc db(new database());

	blob file(p);
	region_loc ram = region::undefined("base",0xc0000000ull);

	if(file.size() < 2)
		throw runtime_error("file too short");

	std::cout << "file magic: " << file.data()[0] << file.data()[1] << std::endl;

	if(file.data()[0] != 'M' || file.data()[1] != 'Z')
		throw runtime_error("unknown magic");

	if(file.size() < 0x3c)
		throw runtime_error("file too short");

	size_t pe_off = *(uint8_t*)(file.data() + 0x3c);

	std::cout << "pe header at " << pe_off << std::endl;


	if(file.size() < pe_off + sizeof(pe_hdr))
		throw runtime_error("file too short");

	pe_hdr* hdr = (pe_hdr*)(file.data() + pe_off);

	std::cout << "magic1: " << hdr->magic1 << std::endl;
	std::cout << "magic2: " << hdr->magic2 << std::endl;
	std::cout << "magic3: " << hdr->magic3 << std::endl;
	std::cout << "magic4: " << hdr->magic4 << std::endl;
	std::cout << "machine: " << hdr->machine << std::endl;
	std::cout << "num_sections: " << hdr->num_sections << std::endl;
	std::cout << "timestamp: " << hdr->timestamp << std::endl;
	std::cout << "symtab: " << hdr->symtab << std::endl;
	std::cout << "num_symbols: " << hdr->num_symbols << std::endl;
	std::cout << "opthdr_size: " << std::dec << hdr->opthdr_size << std::endl;
	std::cout << "flags: " << hdr->flags << std::endl;

	if(hdr->magic1 != 'P' || hdr->magic2 != 'E' || hdr->magic3 != 0 || hdr->magic4 != 0)
		throw runtime_error("wrong magic");

	if(hdr->machine == 0x14c || hdr->machine == 0x864)
		std::cout << "ia32/amd64" << std::endl;
	else
		throw std::runtime_error("unsupported machine type");

	if(!(hdr->flags & 2))
		throw std::runtime_error("image not executable");

	struct opt_hdr
	{
		uint8_t major;
		uint8_t minor;
		uint32_t text_size;
		uint32_t data_size;
		uint32_t bss_size;
		uint32_t entry_point;
		uint32_t text_base;

		union
		{
			struct narrow
			{
				uint32_t data_base; // w
				uint32_t image_base; // w
				uint32_t section_align;
				uint32_t file_align;
				uint16_t os_major;
				uint16_t os_minor;
				uint16_t imgae_major;
				uint16_t image_minor;
				uint16_t subsys_major;
				uint16_t subsys_minor;
				uint32_t win32_ver;
				uint32_t image_size;
				uint32_t header_size;
				uint32_t checksum;
				uint16_t subsys;
				uint16_t dll_flags;
				uint32_t stack_reserve; // w
				uint32_t stack_commit; // w
				uint32_t heap_reserve; // w
				uint32_t heap_commit; // w
				uint32_t loader_flags;
				uint32_t datadir_entries;
			} pe;

			struct wide
			{
				uint64_t image_base; // w
				uint32_t section_align;
				uint32_t file_align;
				uint16_t os_major;
				uint16_t os_minor;
				uint16_t imgae_major;
				uint16_t image_minor;
				uint16_t subsys_major;
				uint16_t subsys_minor;
				uint32_t win32_ver;
				uint32_t image_size;
				uint32_t header_size;
				uint32_t checksum;
				uint16_t subsys;
				uint16_t dll_flags;
				uint64_t stack_reserve; // w
				uint64_t stack_commit; // w
				uint64_t heap_reserve; // w
				uint64_t heap_commit; // w
				uint32_t loader_flags;
				uint32_t datadir_entries;
			} pe_plus;
		} u;
	};

	if(file.size() < pe_off + sizeof(pe_hdr) + 2)
		throw runtime_error("file too short");

	opt_hdr *opt = (opt_hdr*)(file.data() + pe_off + sizeof(pe_hdr));

	std::cout << "=== Optional Header === " << std::dec << sizeof(opt_hdr) << std::endl;

	size_t ddir_cnt = 0;
	uint64_t image_base = 0;

	if(opt->magic == 0x10b)
	{
		std::cout << "magic: " << std::hex << opt->magic << std::dec << " (PE)" << std::endl;
		std::cout << "entry: " << std::hex << opt->entry_point << std::dec << std::endl;
		std::cout << "image base: " << std::hex << opt->u.pe.image_base << std::endl;
		std::cout << "section alignment: " << opt->u.pe.section_align << std::endl;
		std::cout << "file alignment: " << opt->u.pe.file_align << std::endl;
		std::cout << "subsystem ver: " << opt->u.pe.subsys_major << "." << opt->u.pe.subsys_minor << std::endl;
		std::cout << "image size: " << opt->u.pe.image_size << std::endl;
		std::cout << "hdr size: " << opt->u.pe.header_size << std::endl;
		std::cout << "subsys: " << opt->u.pe.subsys << std::endl;
		ddir_cnt = opt->u.pe.datadir_entries;
		image_base = opt->u.pe.image_base;
	}
	else if(opt->magic == 0x20b)
	{
		std::cout << "magic: " << std::hex << opt->magic << std::dec << " (PE+)" << std::endl;
		std::cout << "entry: " << std::hex << opt->entry_point << std::dec << std::dec << std::endl;
		std::cout << "image base: " << std::hex << opt->u.pe_plus.image_base << std::dec << std::endl;
		std::cout << "section alignment: " << opt->u.pe_plus.section_align << std::endl;
		std::cout << "file alignment: " << opt->u.pe_plus.file_align << std::endl;
		std::cout << "subsystem ver: " << opt->u.pe_plus.subsys_major << "." << opt->u.pe_plus.subsys_minor << std::endl;
		std::cout << "image size: " << opt->u.pe_plus.image_size << std::endl;
		std::cout << "hdr size: " << opt->u.pe_plus.header_size << std::endl;
		std::cout << "subsys: " << opt->u.pe_plus.subsys << std::endl;
		ddir_cnt = opt->u.pe_plus.datadir_entries;
		image_base = opt->u.pe_plus.image_base;
	}
	else
		throw std::runtime_error("unknown optional header magic");


	struct ddir_entry
	{
		int32_t rva;
		uint32_t size;
	};

	std::cout << "data directory offset: " << pe_off + sizeof(pe_hdr) + hdr->opthdr_size - sizeof(ddir_entry) * ddir_cnt << std::endl;
	ddir_entry *ddir = (ddir_entry*)(file.data() + pe_off + sizeof(pe_hdr) - sizeof(ddir_entry) * ddir_cnt + hdr->opthdr_size);
	size_t ddir_idx = 0;

	while(ddir_idx < ddir_cnt)
	{
		std::cout << "data dir entry #" << ddir_idx << std::endl;
		std::cout << "rva: " << ddir[ddir_idx].rva << " (" << ddir[ddir_idx].rva + image_base << ")" << std::endl;
		std::cout << "size: " << ddir[ddir_idx].size << std::endl;

		++ddir_idx;
	}

	struct section
	{
		char name0;
		char name1;
		char name2;
		char name3;
		char name4;
		char name5;
		char name6;
		char name7;

		uint32_t virt_sz_or_phy_addr;
		uint32_t virt_address;

		uint32_t raw_sz;
		uint32_t raw_ptr;
		uint32_t reloc_ptr;
		uint32_t linenr_ptr;
		uint16_t reloc_count;
		uint16_t linenr_cout;
		uint32_t flags;
	};

	section* sec_ptr = (section *)(file.data() + pe_off + sizeof(pe_hdr) + hdr->opthdr_size);
	size_t sec_idx = 0;

	while(sec_idx < hdr->num_sections)
	{
		section& s = sec_ptr[sec_idx++];
		std::string n = {s.name0, s.name1, s.name2, s.name3, s.name4, s.name5, s.name6, s.name7};

		std::cout << n << ": ";

		if(s.raw_sz)
		{
			layer_loc l(new layer(n,(uint8_t *)(file.data() + s.raw_ptr),s.raw_sz));
			ram.write().add(po::bound(image_base + s.virt_address,image_base + s.virt_address + s.raw_sz),l);

			std::cout << "mapped" << std::endl;
		}
		else
		{
			std::cout << "not mapped" << std::endl;

			if(s.virt_sz_or_phy_addr)
			{
				layer_loc l(new layer(n,s.virt_sz_or_phy_addr));
				ram.write().add(po::bound(image_base + s.virt_address,image_base + s.virt_address + s.virt_sz_or_phy_addr),l);
			}
		}
	}

	insert_vertex(ram,db.write().data);
	return session{db,std::make_shared<rdf::storage>()};
}*/
