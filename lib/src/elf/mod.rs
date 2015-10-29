use std::mem::{size_of,transmute};
use std::borrow::Cow;
use std::fmt::Debug;
use num::traits::ToPrimitive;
use project::Project;

pub trait ElfClass {
    type Addr: ToPrimitive + Copy + Debug + PartialEq;
    type Half: ToPrimitive + Copy + Debug + PartialEq;
    type Off: ToPrimitive + Copy + Debug + PartialEq;
    type Sword: ToPrimitive + Copy + Debug + PartialEq;
    type Word: ToPrimitive + Copy + Debug + PartialEq;
    type Xword: ToPrimitive + Copy + Debug + PartialEq;

    fn class() -> Class;
}

pub struct Elf32;

impl ElfClass for Elf32 {
    type Addr = u32;
    type Half = u16;
    type Off = u32;
    type Sword = i32;
    type Word = u32;
    type Xword = u64;

    fn class() -> Class {
        Class::ELF32
    }
}

pub struct Elf64;

impl ElfClass for Elf64 {
    type Addr = u64;
    type Half = u16;
    type Off = u64;
    type Sword = i32;
    type Word = u32;
    type Xword = u64;

    fn class() -> Class {
        Class::ELF64
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Ehdr<C: ElfClass> {
    e_ident: [u8; 16],
    e_type: C::Half,
    e_machine: C::Half,
    e_version: C::Word,
    e_entry: C::Addr,
    e_phoff: C::Off,
    e_shoff: C::Off,
    e_flags: C::Word,
    e_ehsize: C::Half,
    e_phentsize: C::Half,
    e_phnum: C::Half,
    e_shentsize: C::Half,
    e_shnum: C::Half,
    e_shstrndx: C::Half,
}

const EI_CLASS: usize = 4;
const EI_DATA: usize = 5;
const EI_VERSION: usize = 6;
const EI_OSABI: usize = 7;
const EI_ABIVERSION: usize = 8;
const EI_PAD: usize = 9;

const MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];

#[derive(Debug,PartialEq,Eq)]
pub enum Class {
    None,
    ELF32,
    ELF64,
    Unknown(u8),
}

impl Class {
    pub fn new(v: u8) -> Class {
        match v {
            0 => Class::None,
            1 => Class::ELF32,
            2 => Class::ELF64,
            a => Class::Unknown(a),
        }
    }
}

#[derive(Debug)]
pub enum Data {
    None,
    LittleEndian,
    BigEndian,
    Unknown(u8),
}

impl Data {
    pub fn new(v: u8) -> Data {
        match v {
            0 => Data::None,
            1 => Data::LittleEndian,
            2 => Data::BigEndian,
            a => Data::Unknown(a),
        }
    }
}

#[derive(Debug)]
pub enum ABI {
    SystemV,
    HPUX,
    NetBSD,
    Linux,
    Solaris,
    AIX,
    Irix,
    FreeBSD,
    Tru64,
    Modesto,
    OpenBSD,
    OpenVMS,
    ARM_EABI,
    ARM,
    Standalone,
    Unknown(u8),
}

impl ABI {
    pub fn new(v: u8) -> ABI {
        match v {
            0 => ABI::SystemV,
            1 => ABI::HPUX,
            2 => ABI::NetBSD,
            3 => ABI::Linux,
            6 => ABI::Solaris,
            7 => ABI::AIX,
            8 => ABI::Irix,
            9 => ABI::FreeBSD,
            10 => ABI::Tru64,
            11 => ABI::Modesto,
            12 => ABI::OpenBSD,
            13 => ABI::OpenVMS,
            64 => ABI::ARM_EABI,
            97 => ABI::ARM,
            255 => ABI::Standalone,
            a => ABI::Unknown(a),
        }
    }
}

#[derive(Debug,PartialEq,Eq)]
pub enum Type {
    None,
    Relocatable,
    Executable,
    Shared,
    Core,
    OsSpecific(u16),
    ProcessorSpecific(u16),
    Unknown(u16),
}

impl Type {
    pub fn new<C: ElfClass>(v: C::Half) -> Type {
        match v.to_u16().unwrap() {
            0 => Type::None,
            1 => Type::Relocatable,
            2 => Type::Executable,
            3 => Type::Shared,
            4 => Type::Core,
            0xfe00...0xff00 => Type::OsSpecific(v.to_u16().unwrap()),
            0xff00...0x10000 => Type::ProcessorSpecific(v.to_u16().unwrap()),
            a => Type::Unknown(a),
        }
    }
}

#[derive(Debug)]
pub enum Machine {
    None,
    M32,
    Sparc,
    i386,
    m68k,
    m88k,
    i860,
    Mips,
    S370,
    MipsLe,
    PARISC,
    VPP500,
    Sparc32Plus,
    i960,
    PPC,
    PPC64,
    S390,
    V800,
    FR20,
    RH32,
    RCE,
    ARM,
    FakeAlpha,
    SH,
    SPARCV9,
    TRICORE,
    ARC,
    H8_300,
    H8_300H,
    H8S,
    H8_500,
    IA_64,
    MIPS_X,
    COLDFIRE,
    HC12,
    MMA,
    PCP,
    NCPU,
    NDR1,
    STARCORE,
    ME16,
    ST100,
    TINYJ,
    X86_64,
    PDSP,
    FX66,
    ST9PLUS,
    ST7,
    HC16,
    HC11,
    HC08,
    HC05,
    SVX,
    ST19,
    VAX,
    CRIS,
    JAVELIN,
    FIREPATH,
    ZSP,
    MMIX,
    HUANY,
    PRISM,
    AVR,
    FR30,
    D10V,
    D30V,
    V850,
    M32R,
    MN10300,
    MN10200,
    PJ,
    OPENRISC,
    ARC_A5,
    XTENSA,
    AARCH64,
    TILEPRO,
    MICROBLAZE,
    TILEGX,
    Unknown(u16),
}

impl Machine {
    pub fn new<C: ElfClass>(v: C::Half) -> Machine {
        match v.to_u16().unwrap() {
            0 => Machine::None,
            1 => Machine::M32,
            2 => Machine::Sparc,
            3 => Machine::i386,
            4 => Machine::m68k,
            5 => Machine::m88k,
            7 => Machine::i860,
            8 => Machine::Mips,
            9 => Machine::S370,
            10 => Machine::MipsLe,
            15 => Machine::PARISC,
            17 => Machine::VPP500,
            18 => Machine::Sparc32Plus,
            19 => Machine::i960,
            20 => Machine::PPC,
            21 => Machine::PPC64,
            22 => Machine::S390,
            36 => Machine::V800,
            37 => Machine::FR20,
            38 => Machine::RH32,
            39 => Machine::RCE,
            40 => Machine::ARM,
            41 => Machine::FakeAlpha,
            42 => Machine::SH,
            43 => Machine::SPARCV9,
            44 => Machine::TRICORE,
            45 => Machine::ARC,
            46 => Machine::H8_300,
            47 => Machine::H8_300H,
            48 => Machine::H8S,
            49 => Machine::H8_500,
            50 => Machine::IA_64,
            51 => Machine::MIPS_X,
            52 => Machine::COLDFIRE,
            53 => Machine::HC12,
            54 => Machine::MMA,
            55 => Machine::PCP,
            56 => Machine::NCPU,
            57 => Machine::NDR1,
            58 => Machine::STARCORE,
            59 => Machine::ME16,
            60 => Machine::ST100,
            61 => Machine::TINYJ,
            62 => Machine::X86_64,
            63 => Machine::PDSP,
            66 => Machine::FX66,
            67 => Machine::ST9PLUS,
            68 => Machine::ST7,
            69 => Machine::HC16,
            70 => Machine::HC11,
            71 => Machine::HC08,
            72 => Machine::HC05,
            73 => Machine::SVX,
            74 => Machine::ST19,
            75 => Machine::VAX,
            76 => Machine::CRIS,
            77 => Machine::JAVELIN,
            78 => Machine::FIREPATH,
            79 => Machine::ZSP,
            80 => Machine::MMIX,
            81 => Machine::HUANY,
            82 => Machine::PRISM,
            83 => Machine::AVR,
            84 => Machine::FR30,
            85 => Machine::D10V,
            86 => Machine::D30V,
            87 => Machine::V850,
            88 => Machine::M32R,
            89 => Machine::MN10300,
            90 => Machine::MN10200,
            91 => Machine::PJ,
            92 => Machine::OPENRISC,
            93 => Machine::ARC_A5,
            94 => Machine::XTENSA,
            183 => Machine::AARCH64,
            188 => Machine::TILEPRO,
            189 => Machine::MICROBLAZE,
            191 => Machine::TILEGX,
            a => Machine::Unknown(a),
        }
    }
}

#[derive(Debug)]
enum SegmentType {
    Null,
    Load,
    Dynamic,
    Interp,
    Note,
    ShLib,
    Phdr,
    TLS,
    OsSpecific(u32),
    GnuEhFrame,
    GnuStack,
    GnuRelro,
    SunWBss,
    SunWStack,
    ProcessorSpecific(u32),
    Unknown(u32),
}

impl SegmentType {
    pub fn new<C: ElfClass>(v: C::Word) -> SegmentType {
        match v.to_u32().unwrap() {
            0 => SegmentType::Null,
            1 => SegmentType::Load,
            2 => SegmentType::Dynamic,
            3 => SegmentType::Interp,
            4 => SegmentType::Note,
            5 => SegmentType::ShLib,
            6 => SegmentType::Phdr,
            7 => SegmentType::TLS,
            0x6474e550 => SegmentType::Null,
            0x6474e551 => SegmentType::Null,
            0x6474e552 => SegmentType::Null,
            0x6ffffffa => SegmentType::Null,
            0x6ffffffb => SegmentType::Null,
            a => match a {
                0x60000000...0x70000000 => SegmentType::OsSpecific(a),
                0x70000000...0x80000000 => SegmentType::ProcessorSpecific(a),
                _ => SegmentType::Unknown(a),
            }
        }
    }
}

impl<C: ElfClass> Ehdr<C> {
    pub fn is_valid(&self) -> Result<(Class,Data,ABI,usize,Type,Machine),&'static str> {
        let i = try!(ident(&self.e_ident));
        if i.0 != C::class() {
            return Err("Wrong ELF class");
        }

        let ty = Type::new::<C>(self.e_type);
        if let Type::Unknown(_) = ty {
            return Err("Invalid file type");
        }

        let em = Machine::new::<C>(self.e_machine);
        if let Machine::Unknown(_) = em {
            return Err("Invalid machine type");
        }

        Ok((i.0,i.1,i.2,i.3,ty,em))
    }
}

fn ident(e_ident: &[u8]) -> Result<(Class,Data,ABI,usize),&'static str> {
    if e_ident[0..4] != MAGIC {
        return Err("Invalid magic number");
    }

    if e_ident[EI_PAD..16].iter().any(|&x| x != 0) {
        return Err("Invalid padding");
    }

    if e_ident[EI_VERSION] != 1 {
        return Err("Invalid ELF version");
    }

    let cls = Class::new(e_ident[EI_CLASS]);
    if let Class::Unknown(_) = cls {
        return Err("Invalid file class");
    }

    let dat = Data::new(e_ident[EI_DATA]);
    if let Data::Unknown(_) = dat {
        return Err("Invalid data encoding");
    }

    let abi = ABI::new(e_ident[EI_OSABI]);
    if let ABI::Unknown(_) = abi {
        return Err("Invalid ABI");
    }

    Ok((cls,dat,abi,e_ident[EI_ABIVERSION] as usize))
}

#[repr(C)]
#[derive(Debug)]
pub struct Phdr32 {
    p_type: <Elf32 as ElfClass>::Word,
    p_offset: <Elf32 as ElfClass>::Off,
    p_vaddr: <Elf32 as ElfClass>::Addr,
    p_paddr: <Elf32 as ElfClass>::Addr,
    p_filesz: <Elf32 as ElfClass>::Word,
    p_memsz: <Elf32 as ElfClass>::Word,
    p_flags: <Elf32 as ElfClass>::Word,
    p_align: <Elf32 as ElfClass>::Word,
}

#[repr(C)]
#[derive(Debug)]
pub struct Phdr64 {
    p_type: <Elf64 as ElfClass>::Word,
    p_flags: <Elf64 as ElfClass>::Word,
    p_offset: <Elf64 as ElfClass>::Off,
    p_vaddr: <Elf64 as ElfClass>::Addr,
    p_paddr: <Elf64 as ElfClass>::Addr,
    p_filesz: <Elf64 as ElfClass>::Xword,
    p_memsz: <Elf64 as ElfClass>::Xword,
    p_align: <Elf64 as ElfClass>::Xword,
}

fn load_reloc_file(_: &[u8]) -> Project {
    unimplemented!()
}

fn load_exec_file(_: &[u8]) -> Project {
    unimplemented!()
}

fn load_so_file(_: &[u8]) -> Project {
    unimplemented!()
}

fn load_core_file(_: &[u8]) -> Project {
    unimplemented!()
}

pub fn executable(file: &[u8]) -> Result<Project,Cow<str>> {
    let mag = try!(ident(file));

    match mag.0 {
        Class::ELF64 => executable_class::<Elf64>(file),
        Class::ELF32 => executable_class::<Elf32>(file),
        _ => Err(Cow::Borrowed("Unknown file class"))
    }
}

pub fn executable_class<C: ElfClass>(file: &[u8]) -> Result<Project,Cow<str>> {
    let p = try!(peek_class::<C>(file));

    if p.4 != Type::Shared {
        return Err(Cow::Borrowed("Wrong file type"));
    }

    let ehdr: &Ehdr<C> = unsafe { &*(file[0..size_of::<Ehdr<C>>()].as_ptr() as *const Ehdr<C>) };

    iterate_program_headers_64(ehdr.e_phnum.to_usize().unwrap(),
                               ehdr.e_phoff.to_usize().unwrap(),
                               ehdr.e_phentsize.to_usize().unwrap(),
                               file)
}

pub fn iterate_program_headers_64(phoff: usize, phnum: usize, phsz: usize, file: &[u8]) -> Result<Project,Cow<str>> {
    if phsz != size_of::<Phdr64>() {
        return Err(Cow::Borrowed("Wrong program header table entry size"));
    }

    for phidx in (0..phnum) {
        let phoff = phoff + phidx * phsz;
        let phdr: &Phdr64 = unsafe { &*(file[phoff..(phoff + phsz)].as_ptr() as *const Phdr64) };

        let ty = SegmentType::new::<Elf64>(phdr.p_type);
        if let SegmentType::Unknown(st) = ty {
            return Err(Cow::Owned(format!("Unknown segment type 0x{:x}",st)));
        }

        println!("{}, p_type: {:?}",phidx,ty);
    }
    unimplemented!()
}

pub fn peek(file: &[u8]) -> Result<(Class,Data,ABI,usize,Type,Machine),&'static str> {
    let mag = try!(ident(file));

    match mag.0 {
        Class::ELF64 => peek_class::<Elf64>(file),
        Class::ELF32 => peek_class::<Elf32>(file),
        _ => Err("Unknown file class")
    }
}

pub fn peek_class<C: ElfClass>(file: &[u8]) -> Result<(Class,Data,ABI,usize,Type,Machine),&'static str> {
    let mag = try!(ident(file));
    if mag.0 != C::class() {
        return Err("Wrong file class");
    }

    let ehdr: &Ehdr<C> = unsafe { &*(file[0..size_of::<Ehdr<C>>()].as_ptr() as *const Ehdr<C>) };
    let id = try!(ehdr.is_valid());
    let ty = Type::new::<C>(ehdr.e_type);
    let em = Machine::new::<C>(ehdr.e_machine);

    if let Type::Unknown(_) = ty {
        Err("Unknown file type")
    } else if let Machine::Unknown(_) = em {
        Err("Unknown machine type")
    } else {
        Ok((id.0,id.1,id.2,id.3,ty,em))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use memmap::{Mmap, Protection};
    use std::path::Path;

    #[test]
    fn elf_load_self() {
        let mmap = Mmap::open_path(Path::new("target/debug/qtpanopticon"),Protection::Read).ok().unwrap();
        let buf = unsafe { mmap.as_slice() };

        if let Err(e) = executable(buf) {
            panic!("{:?}",e);
        }
    }
}
