/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2015,2016  Panopticon authors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::fmt::Debug;
use std::io;
use std::io::{Seek,SeekFrom,Read};
use std::convert::From;
use num::traits::ToPrimitive;
use byteorder as bo;

use result::Result;

pub trait ElfClass {
    type Addr: ToPrimitive + Copy + Debug + PartialEq;
    type Half: ToPrimitive + Copy + Debug + PartialEq;
    type Off: ToPrimitive + Copy + Debug + PartialEq;
    type Sword: ToPrimitive + Copy + Debug + PartialEq;
    type Word: ToPrimitive + Copy + Debug + PartialEq;
    type Xword: ToPrimitive + Copy + Debug + PartialEq;
    type Yword: ToPrimitive + Copy + Debug + PartialEq;

    fn class() -> Class;
    fn read_addr<B: bo::ByteOrder, R: bo::ReadBytesExt>(fd: &mut R) -> io::Result<Self::Addr>;
    fn read_half<B: bo::ByteOrder, R: bo::ReadBytesExt>(fd: &mut R) -> io::Result<Self::Half>;
    fn read_off<B: bo::ByteOrder, R: bo::ReadBytesExt>(fd: &mut R) -> io::Result<Self::Off>;
    fn read_sword<B: bo::ByteOrder, R: bo::ReadBytesExt>(fd: &mut R) -> io::Result<Self::Sword>;
    fn read_word<B: bo::ByteOrder, R: bo::ReadBytesExt>(fd: &mut R) -> io::Result<Self::Word>;
    fn read_xword<B: bo::ByteOrder, R: bo::ReadBytesExt>(fd: &mut R) -> io::Result<Self::Xword>;
    fn read_yword<B: bo::ByteOrder, R: bo::ReadBytesExt>(fd: &mut R) -> io::Result<Self::Yword>;
}

pub struct Elf32;

impl ElfClass for Elf32 {
    type Addr = u32;
    type Half = u16;
    type Off = u32;
    type Sword = i32;
    type Word = u32;
    type Xword = u64;
    type Yword = u32;

    fn class() -> Class {
        Class::ELF32
    }

    fn read_addr<B: bo::ByteOrder, R: bo::ReadBytesExt>(fd: &mut R) -> io::Result<Self::Addr> {
        fd.read_u32::<B>()
    }

    fn read_half<B: bo::ByteOrder, R: bo::ReadBytesExt>(fd: &mut R) -> io::Result<Self::Half> {
        fd.read_u16::<B>()
    }

    fn read_off<B: bo::ByteOrder, R: bo::ReadBytesExt>(fd: &mut R) -> io::Result<Self::Off> {
        fd.read_u32::<B>()
    }

    fn read_sword<B: bo::ByteOrder, R: bo::ReadBytesExt>(fd: &mut R) -> io::Result<Self::Sword> {
        fd.read_i32::<B>()
    }

    fn read_word<B: bo::ByteOrder, R: bo::ReadBytesExt>(fd: &mut R) -> io::Result<Self::Word> {
        fd.read_u32::<B>()
    }

    fn read_xword<B: bo::ByteOrder, R: bo::ReadBytesExt>(fd: &mut R) -> io::Result<Self::Xword> {
        fd.read_u64::<B>()
    }

    fn read_yword<B: bo::ByteOrder, R: bo::ReadBytesExt>(fd: &mut R) -> io::Result<Self::Yword> {
        fd.read_u32::<B>()
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
    type Yword = u64;

    fn class() -> Class {
        Class::ELF64
    }

    fn read_addr<B: bo::ByteOrder, R: bo::ReadBytesExt>(fd: &mut R) -> io::Result<Self::Addr> {
        fd.read_u64::<B>()
    }

    fn read_half<B: bo::ByteOrder, R: bo::ReadBytesExt>(fd: &mut R) -> io::Result<Self::Half> {
        fd.read_u16::<B>()
    }

    fn read_off<B: bo::ByteOrder, R: bo::ReadBytesExt>(fd: &mut R) -> io::Result<Self::Off> {
        fd.read_u64::<B>()
    }

    fn read_sword<B: bo::ByteOrder, R: bo::ReadBytesExt>(fd: &mut R) -> io::Result<Self::Sword> {
        fd.read_i32::<B>()
    }

    fn read_word<B: bo::ByteOrder, R: bo::ReadBytesExt>(fd: &mut R) -> io::Result<Self::Word> {
        fd.read_u32::<B>()
    }

    fn read_xword<B: bo::ByteOrder, R: bo::ReadBytesExt>(fd: &mut R) -> io::Result<Self::Xword> {
        fd.read_u64::<B>()
    }

    fn read_yword<B: bo::ByteOrder, R: bo::ReadBytesExt>(fd: &mut R) -> io::Result<Self::Yword> {
        fd.read_u64::<B>()
    }
}

#[derive(Debug)]
pub struct Ehdr {
    pub ident: Ident,
    pub file_type: Type,
    pub machine: Machine,
    pub version: usize,
    pub entry: u64,
    pub progam_headers: Vec<Phdr>,
    pub segment_headers: Vec<Shdr>,
    pub string_segment: usize,
}

impl Ehdr {
    pub fn read<R: bo::ReadBytesExt + Seek>(fd: &mut R) -> Result<Ehdr> {
        let id = try!(Ident::read(fd));

        fn read<C: ElfClass + ReadPhdr,B: bo::ByteOrder,R: bo::ReadBytesExt + Seek>(fd: &mut R, ident: Ident) -> Result<Ehdr> {
            let file_type = try!(C::read_half::<B,R>(fd));
            let machine = try!(C::read_half::<B,R>(fd));
            let version = try!(C::read_word::<B,R>(fd));
            let entry = try!(C::read_addr::<B,R>(fd));
            let phoff = try!(C::read_off::<B,R>(fd)).to_u64().unwrap();
            let shoff = try!(C::read_off::<B,R>(fd)).to_u64().unwrap();
            let flags = try!(C::read_word::<B,R>(fd));
            let ehsize = try!(C::read_half::<B,R>(fd));
            let phentsize = try!(C::read_half::<B,R>(fd)).to_u64().unwrap();
            let phnum = try!(C::read_half::<B,R>(fd)).to_u64().unwrap();
            let shentsize = try!(C::read_half::<B,R>(fd)).to_u64().unwrap();
            let shnum = try!(C::read_half::<B,R>(fd)).to_u64().unwrap();
            let shstridx = try!(C::read_half::<B,R>(fd));

            let mut phdrs = vec![];
            let mut shdrs = vec![];

            for pidx in 0..phnum {
                let off = phoff + pidx * phentsize;

                if fd.seek(SeekFrom::Start(off)).ok() == Some(off) {
                    phdrs.push(try!(<C as ReadPhdr>::read::<B,R>(fd)));

                } else {
                    return Err("Premature end of file while reading program headers".into());
                }
            }

            for sidx in 0..shnum {
                let off = shoff + sidx * shentsize;

                if fd.seek(SeekFrom::Start(off)).ok() == Some(off) {
                    shdrs.push(try!(Shdr::read::<C,B,R>(fd)));

                } else {
                    return Err("Premature end of file while reading segment headers".into());
                }
            }

            Ok(Ehdr{
                ident: ident,
                file_type: Type::new::<C>(file_type),
                machine: Machine::new::<C>(machine),
                version: version.to_usize().unwrap(),
                entry: entry.to_u64().unwrap(),
                progam_headers: phdrs,
                segment_headers: shdrs,
                string_segment: 0
            })
        }

        let maybe_ret = match (&id.class,&id.data) {
            (&Class::ELF32,&Data::LittleEndian) => read::<Elf32,bo::LittleEndian,R>(fd,id),
            (&Class::ELF64,&Data::LittleEndian) => read::<Elf64,bo::LittleEndian,R>(fd,id),
            (&Class::ELF32,&Data::BigEndian) => read::<Elf32,bo::BigEndian,R>(fd,id),
            (&Class::ELF64,&Data::BigEndian) => read::<Elf64,bo::BigEndian,R>(fd,id),
            _ => return Err("Invalid byte order and/or ELF class".into()),
        };

        match maybe_ret {
            Ok(r) => Ok(r),
            Err(_) => Err("Parsing failed".into()),
        }
    }
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
            a @ 0xfe00...0xfeff => Type::OsSpecific(a),
            a @ 0xff00...0xffff => Type::ProcessorSpecific(a),
            a => Type::Unknown(a),
        }
    }
}

#[allow(non_camel_case_types)]
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
pub struct Ident {
    pub magic: [u8; 4],
    pub class: Class,
    pub data: Data,
    pub version: usize,
    pub abi: ABI,
    pub abi_ver: usize,
    pub pad: [u8; 7],
}

impl Ident {
    pub fn read<R: Read>(strm: &mut R) -> Result<Ident> {
        let mut e_ident = [0u8; 16];

        if let Err(_) = strm.read(&mut e_ident) {
            return Err("Failed to read ident".into());
        }

        if e_ident[0..4] != MAGIC {
            return Err("Invalid magic number".into());
        }

        if e_ident[EI_PAD..16].iter().any(|&x| x != 0) {
            return Err("Invalid padding".into());
        }

        if e_ident[EI_VERSION] != 1 {
            return Err("Invalid ELF version".into());
        }

        let cls = Class::new(e_ident[EI_CLASS]);
        if let Class::Unknown(_) = cls {
            return Err("Invalid file class".into());
        }

        let dat = Data::new(e_ident[EI_DATA]);
        if let Data::Unknown(_) = dat {
            return Err("Invalid data encoding".into());
        }

        let abi = ABI::new(e_ident[EI_OSABI]);
        if let ABI::Unknown(_) = abi {
            return Err("Invalid ABI".into());
        }

        Ok(Ident{
            magic: [e_ident[0],e_ident[1],e_ident[2],e_ident[3]],
            class: cls,
            data: dat,
            version: e_ident[EI_VERSION] as usize,
            abi: abi,
            abi_ver: e_ident[EI_ABIVERSION] as usize,
            pad: [
                  e_ident[EI_PAD+0],
                  e_ident[EI_PAD+1],
                  e_ident[EI_PAD+2],
                  e_ident[EI_PAD+3],
                  e_ident[EI_PAD+4],
                  e_ident[EI_PAD+5],
                  e_ident[EI_PAD+6]
            ],
        })
    }
}

#[derive(Debug)]
pub enum SegmentType {
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

#[derive(Debug)]
pub struct Phdr {
    pub seg_type: SegmentType,
    pub offset: u64,
    pub vaddr: u64,
    pub paddr: u64,
    pub filesz: u64,
    pub flags: usize,
    pub align: usize,
    pub memsz: u64,
}

trait ReadPhdr {
    fn read<B: bo::ByteOrder,R: bo::ReadBytesExt>(fd: &mut R) -> Result<Phdr>;
}

impl ReadPhdr for Elf32 {
    fn read<B: bo::ByteOrder,R: bo::ReadBytesExt>(fd: &mut R) -> Result<Phdr> {
        let segtype = try!(Elf32::read_word::<B,R>(fd));
        let offset = try!(Elf32::read_off::<B,R>(fd));
        let vaddr = try!(Elf32::read_addr::<B,R>(fd));
        let paddr = try!(Elf32::read_addr::<B,R>(fd));
        let filesz = try!(Elf32::read_word::<B,R>(fd));
        let memsz = try!(Elf32::read_word::<B,R>(fd));
        let flags = try!(Elf32::read_word::<B,R>(fd));
        let align= try!(Elf32::read_word::<B,R>(fd));

        Ok(Phdr{
            seg_type: SegmentType::new::<Elf32>(segtype),
            offset: offset.to_u64().unwrap(),
            vaddr: vaddr.to_u64().unwrap(),
            paddr: paddr.to_u64().unwrap(),
            filesz: filesz.to_u64().unwrap(),
            flags: flags.to_usize().unwrap(),
            align: align.to_usize().unwrap(),
            memsz: memsz.to_u64().unwrap(),
        })
    }
}

impl ReadPhdr for Elf64 {
    fn read<B: bo::ByteOrder,R: bo::ReadBytesExt>(fd: &mut R) -> Result<Phdr> {
        let segtype = try!(Elf64::read_word::<B,R>(fd));
        let flags = try!(Elf64::read_word::<B,R>(fd));
        let offset = try!(Elf64::read_off::<B,R>(fd));
        let vaddr = try!(Elf64::read_addr::<B,R>(fd));
        let paddr = try!(Elf64::read_addr::<B,R>(fd));
        let filesz = try!(Elf64::read_xword::<B,R>(fd));
        let memsz = try!(Elf64::read_xword::<B,R>(fd));
        let align= try!(Elf64::read_xword::<B,R>(fd));

        Ok(Phdr{
            seg_type: SegmentType::new::<Elf32>(segtype),
            offset: offset.to_u64().unwrap(),
            vaddr: vaddr.to_u64().unwrap(),
            paddr: paddr.to_u64().unwrap(),
            filesz: filesz.to_u64().unwrap(),
            flags: flags.to_usize().unwrap(),
            align: align.to_usize().unwrap(),
            memsz: memsz.to_u64().unwrap(),
        })
    }
}

#[derive(Debug)]
enum SectionType {
    Null,
    Progbits,
    Symtab,
    Strtab,
    Rela,
    Hash,
    Dynamic,
    Note,
    Nobits,
    Rel,
    Shlib,
    Dynsym,
    InitArray,
    FiniArray,
    PreinitArray,
    Group,
    SymtabIndices,
    OsSpecific(u32),
    ProcessorSpecific(u32),
    ApplicationSpecific(u32),
    Unknown(u32),
}

impl SectionType {
    pub fn new<C: ElfClass>(v: C::Word) -> SectionType {
        match v.to_u32().unwrap() {
            0 => SectionType::Null,
            1 => SectionType::Progbits,
            2 => SectionType::Symtab,
            3 => SectionType::Strtab,
            4 => SectionType::Rela,
            5 => SectionType::Hash,
            6 => SectionType::Dynamic,
            7 => SectionType::Note,
            8 => SectionType::Nobits,
            9 => SectionType::Rel,
            10 => SectionType::Shlib,
            11 => SectionType::Dynsym,
            14 => SectionType::InitArray,
            15 => SectionType::FiniArray,
            16 => SectionType::PreinitArray,
            17 => SectionType::Group,
            18 => SectionType::SymtabIndices,
            a => match a {
                0x60000000...0x70000000 => SectionType::OsSpecific(a),
                0x70000000...0x80000000 => SectionType::ProcessorSpecific(a),
                0x80000000...0x90000000 => SectionType::ApplicationSpecific(a),
                _ => SectionType::Unknown(a),
            }
        }
    }
}

#[derive(Debug)]
pub struct Shdr {
    name: String,
    sec_type: SectionType,
    flags: usize,
    addr: u64,
    offset: u64,
    size: u64,
    link: usize,
    info: usize,
    entsize: u64,
}

impl Shdr {
    pub fn read<C: ElfClass,B: bo::ByteOrder,R: bo::ReadBytesExt>(fd: &mut R) -> Result<Shdr> {
        let name = try!(C::read_word::<B,R>(fd));
        let sectype = try!(C::read_word::<B,R>(fd));
        let flags = try!(C::read_yword::<B,R>(fd));
        let addr = try!(C::read_addr::<B,R>(fd));
        let offset = try!(C::read_off::<B,R>(fd));
        let size = try!(C::read_yword::<B,R>(fd));
        let link = try!(C::read_word::<B,R>(fd));
        let info = try!(C::read_word::<B,R>(fd));
        let align = try!(C::read_yword::<B,R>(fd));
        let entsize = try!(C::read_yword::<B,R>(fd));

        Ok(Shdr{
            name: "TODO".to_string(),
            sec_type: SectionType::new::<C>(sectype),
            flags: flags.to_usize().unwrap(),
            addr: addr.to_u64().unwrap(),
            offset: offset.to_u64().unwrap(),
            size: size.to_u64().unwrap(),
            link: link.to_usize().unwrap(),
            info: info.to_usize().unwrap(),
            entsize: entsize.to_u64().unwrap(),
        })
    }
}
