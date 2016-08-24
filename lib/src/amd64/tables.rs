use amd64::{Opcode,Operand,OperandType,AddressingMethod,Mnemonic,OpcodeOption};

macro_rules! opcode {
  ($mne:expr ; ; $opt:ident) => {
       Opcode{
            mnemonic: Mnemonic::Single($mne),
            operand_a: Operand::None,
            operand_b: Operand::None,
            operand_c: Operand::None,
            operand_d: Operand::None,
            option: OpcodeOption::$opt,
        }
    };
  (group $grp:expr; $addr1:ident / $type1:ident , $addr2:ident / $type2:ident) => {
        Opcode{
            mnemonic: Mnemonic::ModRM($grp),
            operand_a: Operand::Present(AddressingMethod::$addr1,OperandType::$type1),
            operand_b: Operand::Present(AddressingMethod::$addr2,OperandType::$type2),
            operand_c: Operand::None,
            operand_d: Operand::None,
            option: OpcodeOption::None,
        }
    };
   ($mne:expr; $addr1:ident / $type1:ident , $addr2:ident / $type2:ident; $opt:ident) => {
        Opcode{
            mnemonic: Mnemonic::Single($mne),
            operand_a: Operand::Present(AddressingMethod::$addr1,OperandType::$type1),
            operand_b: Operand::Present(AddressingMethod::$addr2,OperandType::$type2),
            operand_c: Operand::None,
            operand_d: Operand::None,
            option: OpcodeOption::$opt,
        }
    };
   ($mne:expr; $addr1:ident / $type1:ident , $addr2:ident / $type2:ident) => {
        Opcode{
            mnemonic: Mnemonic::Single($mne),
            operand_a: Operand::Present(AddressingMethod::$addr1,OperandType::$type1),
            operand_b: Operand::Present(AddressingMethod::$addr2,OperandType::$type2),
            operand_c: Operand::None,
            operand_d: Operand::None,
            option: OpcodeOption::None,
        }
    };
    ($mne:expr; $addr1:ident / $type1:ident , $addr2:ident / $type2:ident , $addr3:ident / $type3:ident) => {
        Opcode{
            mnemonic: Mnemonic::Single($mne),
            operand_a: Operand::Present(AddressingMethod::$addr1,OperandType::$type1),
            operand_b: Operand::Present(AddressingMethod::$addr2,OperandType::$type2),
            operand_c: Operand::Present(AddressingMethod::$addr3,OperandType::$type3),
            operand_d: Operand::None,
            option: OpcodeOption::None,
        }
    };
    ($mne:expr; $addr1:ident / $type1:ident , $addr2:ident / $type2:ident , $addr3:ident / $type3:ident , $addr4:ident / $type4:ident) => {
        Opcode{
            mnemonic: Mnemonic::Single($mne),
            operand_a: Operand::Present(AddressingMethod::$addr1,OperandType::$type1),
            operand_b: Operand::Present(AddressingMethod::$addr2,OperandType::$type2),
            operand_c: Operand::Present(AddressingMethod::$addr3,OperandType::$type3),
            operand_d: Operand::Present(AddressingMethod::$addr4,OperandType::$type4),
            option: OpcodeOption::None,
        }
    };
    ($mne:expr; $addr1:ident / $type1:ident , $addr2:ident / $type2:ident , $type3:ident) => {
        Opcode{
            mnemonic: Mnemonic::Single($mne),
            operand_a: Operand::Present(AddressingMethod::$addr1,OperandType::$type1),
            operand_b: Operand::Present(AddressingMethod::$addr2,OperandType::$type2),
            operand_c: Operand::Present(AddressingMethod::None,OperandType::$type3),
            operand_d: Operand::None,
            option: OpcodeOption::None,
        }
    };
    ($mne:expr; $type1:ident , $addr2:ident / $type2:ident) => {
        Opcode{
            mnemonic: Mnemonic::Single($mne),
            operand_a: Operand::Present(AddressingMethod::None,OperandType::$type1),
            operand_b: Operand::Present(AddressingMethod::$addr2,OperandType::$type2),
            operand_c: Operand::None,
            operand_d: Operand::None,
            option: OpcodeOption::None,
        }
    };
      ($mne:expr; $addr1:ident / $type1:ident , $type2:ident) => {
        Opcode{
            mnemonic: Mnemonic::Single($mne),
            operand_a: Operand::Present(AddressingMethod::$addr1,OperandType::$type1),
            operand_b: Operand::Present(AddressingMethod::None,OperandType::$type2),
            operand_c: Operand::None,
            operand_d: Operand::None,
            option: OpcodeOption::None,
        }
    };
    ($mne:expr; $type1:ident , $type2:ident) => {
        Opcode{
            mnemonic: Mnemonic::Single($mne),
            operand_a: Operand::Present(AddressingMethod::None,OperandType::$type1),
            operand_b: Operand::Present(AddressingMethod::None,OperandType::$type2),
            operand_c: Operand::None,
            operand_d: Operand::None,
            option: OpcodeOption::None,
        }
    };
    ($mne:expr; $addr1:ident / $type1:ident; $opt:ident) => {
       Opcode{
            mnemonic: Mnemonic::Single($mne),
            operand_a: Operand::Present(AddressingMethod::$addr1,OperandType::$type1),
            operand_b: Operand::None,
            operand_c: Operand::None,
            operand_d: Operand::None,
            option: OpcodeOption::$opt,
        }
    };
  ($mne:expr; $addr1:ident / $type1:ident) => {
       Opcode{
            mnemonic: Mnemonic::Single($mne),
            operand_a: Operand::Present(AddressingMethod::$addr1,OperandType::$type1),
            operand_b: Operand::None,
            operand_c: Operand::None,
            operand_d: Operand::None,
            option: OpcodeOption::None,
        }
    };
    (group $grp:expr ; $addr1:ident / $type1:ident) => {
        Opcode{
            mnemonic: Mnemonic::ModRM($grp),
            operand_a: Operand::Present(AddressingMethod::$addr1,OperandType::$type1),
            operand_b: Operand::None,
            operand_c: Operand::None,
            operand_d: Operand::None,
            option: OpcodeOption::None,
        }
    };
    (group $grp:expr; $addr1:ident / $type1:ident , $type2:ident) => {
        Opcode{
            mnemonic: Mnemonic::ModRM($grp),
            operand_a: Operand::Present(AddressingMethod::$addr1,OperandType::$type1),
            operand_b: Operand::Present(AddressingMethod::None,OperandType::$type2),
            operand_c: Operand::None,
            operand_d: Operand::None,
            option: OpcodeOption::None,
        }
    };
    ($mne:expr; $type1:ident; $opt:ident) => {
       Opcode{
            mnemonic: Mnemonic::Single($mne),
            operand_a: Operand::Present(AddressingMethod::None,OperandType::$type1),
            operand_b: Operand::None,
            operand_c: Operand::None,
            operand_d: Operand::None,
            option: OpcodeOption::$opt,
        }
    };
       ($mne:expr; $type1:ident) => {
       Opcode{
            mnemonic: Mnemonic::Single($mne),
            operand_a: Operand::Present(AddressingMethod::None,OperandType::$type1),
            operand_b: Operand::None,
            operand_c: Operand::None,
            operand_d: Operand::None,
            option: OpcodeOption::None,
        }
    };
    ($mne:expr; ) => {
       Opcode{
            mnemonic: Mnemonic::Single($mne),
            operand_a: Operand::None,
            operand_b: Operand::None,
            operand_c: Operand::None,
            operand_d: Operand::None,
            option: OpcodeOption::None,
        }
    };
    (group $grp:expr; ) => {
       Opcode{
            mnemonic: Mnemonic::ModRM($grp),
            operand_a: Operand::None,
            operand_b: Operand::None,
            operand_c: Operand::None,
            operand_d: Operand::None,
            option: OpcodeOption::None,
        }
    };
}

macro_rules! unused {
    () => {
        Opcode{
            mnemonic: Mnemonic::Undefined,
            operand_a: Operand::None,
            operand_b: Operand::None,
            operand_c: Operand::None,
            operand_d: Operand::None,
            option: OpcodeOption::None,
        }
    };
}

macro_rules! escape {
    () => {
        Opcode{
            mnemonic: Mnemonic::Escape,
            operand_a: Operand::None,
            operand_b: Operand::None,
            operand_c: Operand::None,
            operand_d: Operand::None,
            option: OpcodeOption::None,
        }
    };
}

pub static ONEBYTE_TABLE: [Opcode; 256] = [
    opcode!("add"; E/b, G/b),       // 0x00: add
    opcode!("add"; E/v, G/v),       // 0x01: add
    opcode!("add"; G/b, E/b),       // 0x02: add
    opcode!("add"; G/v, E/v),       // 0x03: add
    opcode!("add"; AL, I/b),        // 0x04: add
    opcode!("add"; rAX, I/z),       // 0x05: add
    opcode!("push"; ES; Invalid64),            // 0x06: push
    opcode!("pop"; ES; Invalid64),             // 0x07: pop
    opcode!("or"; E/b, G/b),        // 0x08: or
    opcode!("or"; E/v, G/v),        // 0x09: or
    opcode!("or"; G/b, E/b),        // 0x0a: or
    opcode!("or"; G/v, E/v),        // 0x0b: or
    opcode!("or"; AL, I/b),         // 0x0c: or
    opcode!("or"; rAX, I/z),        // 0x0d: or
    opcode!("push"; CS; Invalid64),            // 0x0e: push
    unused!(),                      // 0x0f
    opcode!("adc"; E/b, G/b),       // 0x10: adc
    opcode!("adc"; E/v, G/v),       // 0x11: adc
    opcode!("adc"; G/b, E/b),       // 0x12: adc
    opcode!("adc"; G/v, E/v),       // 0x13: adc
    opcode!("adc"; AL, I/b),        // 0x14: adc
    opcode!("adc"; rAX, I/z),       // 0x15: adc
    opcode!("push"; SS; Invalid64),            // 0x16: push
    opcode!("pop"; SS; Invalid64),             // 0x17: pop
    opcode!("sbb"; E/b, G/b),       // 0x18: sbb
    opcode!("sbb"; E/v, G/v),       // 0x19: sbb
    opcode!("sbb"; G/b, E/b),       // 0x1a: sbb
    opcode!("sbb"; G/v, E/v),       // 0x1b: sbb
    opcode!("sbb"; AL, I/b),        // 0x1c: sbb
    opcode!("sbb"; rAX, I/z),       // 0x1d: sbb
    opcode!("push"; DS; Invalid64),            // 0x1e: push
    opcode!("pop"; DS; Invalid64),             // 0x1f: pop
    opcode!("and"; E/b, G/b),       // 0x20: and
    opcode!("and"; E/v, G/v),       // 0x21: and
    opcode!("and"; G/b, E/b),       // 0x22: and
    opcode!("and"; G/v, E/v),       // 0x23: and
    opcode!("and"; AL, I/b),        // 0x24: and
    opcode!("and"; rAX, I/z),       // 0x25: and
    unused!(),                      // 0x26
    opcode!("daa"; ; Invalid64),               // 0x27: daa
    opcode!("sub"; E/b, G/b),       // 0x28: sub
    opcode!("sub"; E/v, G/v),       // 0x29: sub
    opcode!("sub"; G/b, E/b),       // 0x2a: sub
    opcode!("sub"; G/v, E/v),       // 0x2b: sub
    opcode!("sub"; AL, I/b),        // 0x2c: sub
    opcode!("sub"; rAX, I/z),       // 0x2d: sub
    unused!(),                      // 0x2e
    opcode!("das"; ; Invalid64),               // 0x2f: das
    opcode!("xor"; E/b, G/b),       // 0x30: xor
    opcode!("xor"; E/v, G/v),       // 0x31: xor
    opcode!("xor"; G/b, E/b),       // 0x32: xor
    opcode!("xor"; G/v, E/v),       // 0x33: xor
    opcode!("xor"; AL, I/b),        // 0x34: xor
    opcode!("xor"; rAX, I/z),       // 0x35: xor
    unused!(),                      // 0x36
    opcode!("aaa"; ; Invalid64),               // 0x37: aaa
    opcode!("cmp"; E/b, G/b),       // 0x38: cmp
    opcode!("cmp"; E/v, G/v),       // 0x39: cmp
    opcode!("cmp"; G/b, E/b),       // 0x3a: cmp
    opcode!("cmp"; G/v, E/v),       // 0x3b: cmp
    opcode!("cmp"; AL, I/b),        // 0x3c: cmp
    opcode!("cmp"; rAX, I/z),       // 0x3d: cmp
    unused!(),                      // 0x3e
    opcode!("aas"; ; Invalid64),               // 0x3f: aas
    opcode!("inc"; eAX; Invalid64),            // 0x40: inc
    opcode!("inc"; eCX; Invalid64),            // 0x41: inc
    opcode!("inc"; eDX; Invalid64),            // 0x42: inc
    opcode!("inc"; eBX; Invalid64),            // 0x43: inc
    opcode!("inc"; eSP; Invalid64),            // 0x44: inc
    opcode!("inc"; eBP; Invalid64),            // 0x45: inc
    opcode!("inc"; eSI; Invalid64),            // 0x46: inc
    opcode!("inc"; eDI; Invalid64),            // 0x47: inc
    opcode!("dec"; eAX; Invalid64),            // 0x48: dec
    opcode!("dec"; eCX; Invalid64),            // 0x49: dec
    opcode!("dec"; eDX; Invalid64),            // 0x4a: dec
    opcode!("dec"; eBX; Invalid64),            // 0x4b: dec
    opcode!("dec"; eSP; Invalid64),            // 0x4c: dec
    opcode!("dec"; eBP; Invalid64),            // 0x4d: dec
    opcode!("dec"; eSI; Invalid64),            // 0x4e: dec
    opcode!("dec"; eDI; Invalid64),            // 0x4f: dec
    opcode!("push"; rAX; Default64),           // 0x50: push
    opcode!("push"; rCX; Default64),           // 0x51: push
    opcode!("push"; rDX; Default64),           // 0x52: push
    opcode!("push"; rBX; Default64),           // 0x53: push
    opcode!("push"; rSP; Default64),           // 0x54: push
    opcode!("push"; rBP; Default64),           // 0x55: push
    opcode!("push"; rSI; Default64),           // 0x56: push
    opcode!("push"; rDI; Default64),           // 0x57: push
    opcode!("pop"; rAX; Default64),            // 0x58: pop
    opcode!("pop"; rCX; Default64),            // 0x59: pop
    opcode!("pop"; rDX; Default64),            // 0x5a: pop
    opcode!("pop"; rBX; Default64),            // 0x5b: pop
    opcode!("pop"; rSP; Default64),            // 0x5c: pop
    opcode!("pop"; rBP; Default64),            // 0x5d: pop
    opcode!("pop"; rSI; Default64),            // 0x5e: pop
    opcode!("pop"; rDI; Default64),            // 0x5f: pop
    opcode!("pusha"; ;Invalid64),             // 0x60: pusha
    opcode!("popa"; ;Invalid64),              // 0x61: popa
    opcode!("bound"; G/v, M/a; Invalid64),     // 0x62: bound
    opcode!("arpl"; E/w, G/w; Invalid64),      // 0x63: arpl
    unused!(),                      // 0x64
    unused!(),                      // 0x65
    unused!(),                      // 0x66
    unused!(),                      // 0x67
    opcode!("push"; I/z; Default64),           // 0x68: push
    opcode!("imul"; G/v, E/v, I/z), // 0x69: imul
    opcode!("push"; I/b; Default64),           // 0x6a: push
    opcode!("imul"; G/v, E/v, I/b), // 0x6b: imul
    opcode!("insb"; ),              // 0x6c: insb
    opcode!("insw"; ),              // 0x6d: insw
    opcode!("outsb"; ),             // 0x6e: outsb
    opcode!("outsw"; ),             // 0x6f: outsw
    opcode!("jo"; J/b; Force64),             // 0x70: jo
    opcode!("jno"; J/b; Force64),            // 0x71: jno
    opcode!("jb"; J/b; Force64),             // 0x72: jb
    opcode!("jae"; J/b; Force64),            // 0x73: jae
    opcode!("jz"; J/b; Force64),             // 0x74: jz
    opcode!("jnz"; J/b; Force64),            // 0x75: jnz
    opcode!("jbe"; J/b; Force64),            // 0x76: jbe
    opcode!("ja"; J/b; Force64),             // 0x77: ja
    opcode!("js"; J/b; Force64),             // 0x78: js
    opcode!("jns"; J/b; Force64),            // 0x79: jns
    opcode!("jp"; J/b; Force64),             // 0x7a: jp
    opcode!("jnp"; J/b; Force64),            // 0x7b: jnp
    opcode!("jl"; J/b; Force64),             // 0x7c: jl
    opcode!("jge"; J/b; Force64),            // 0x7d: jge
    opcode!("jle"; J/b; Force64),            // 0x7e: jle
    opcode!("jg"; J/b; Force64),             // 0x7f: jg
    opcode!(group 1; E/b, I/b),     // 0x80: adc
    opcode!(group 1; E/v, I/z),     // 0x81: adc
    opcode!(group 1; E/b, I/b),     // 0x82: adc
    opcode!(group 1; E/v, I/b),     // 0x83: adc
    opcode!("test"; E/b, G/b),      // 0x84: test
    opcode!("test"; E/v, G/v),      // 0x85: test
    opcode!("xchg"; E/b, G/b),      // 0x86: xchg
    opcode!("xchg"; E/v, G/v),      // 0x87: xchg
    opcode!("mov"; E/b, G/b),       // 0x88: mov
    opcode!("mov"; E/v, G/v),       // 0x89: mov
    opcode!("mov"; G/b, E/b),       // 0x8a: mov
    opcode!("mov"; G/v, E/v),       // 0x8b: mov
    opcode!("mov"; E/v, S/w),       // 0x8c: mov
    opcode!("lea"; G/v, M/None),    // 0x8d: lea
    opcode!("mov"; S/w, E/w),       // 0x8e: mov
    opcode!(group 101; E/v),          // 0x8f: group 1a
    opcode!("xchg"; rAX, rAX),      // 0x90: xchg
    opcode!("xchg"; rCX, rAX),      // 0x91: xchg
    opcode!("xchg"; rDX, rAX),      // 0x92: xchg
    opcode!("xchg"; rBX, rAX),      // 0x93: xchg
    opcode!("xchg"; rSP, rAX),      // 0x94: xchg
    opcode!("xchg"; rBP, rAX),      // 0x95: xchg
    opcode!("xchg"; rSI, rAX),      // 0x96: xchg
    opcode!("xchg"; rDI, rAX),      // 0x97: xchg
    opcode!("cbw"; ),               // 0x98: cbw
    opcode!("cwd"; ),               // 0x99: cwd
    opcode!("call"; A/v; Invalid64),           // 0x9a: call
    opcode!("wait"; ),              // 0x9b: wait
    opcode!("pushfw"; ; Default64),            // 0x9c: pushfw
    opcode!("popfw"; ; Default64),             // 0x9d: popfw
    opcode!("sahf"; ),              // 0x9e: sahf
    opcode!("lahf"; ),              // 0x9f: lahf
    opcode!("mov"; AL, O/b),        // 0xa0: mov
    opcode!("mov"; rAX, O/v),       // 0xa1: mov
    opcode!("mov"; O/b, AL),        // 0xa2: mov
    opcode!("mov"; O/v, rAX),       // 0xa3: mov
    opcode!("movsb"; ),             // 0xa4: movsb
    opcode!("movsw"; ),             // 0xa5: movsw
    opcode!("cmpsb"; ),             // 0xa6: cmpsb
    opcode!("cmpsw"; ),             // 0xa7: cmpsw
    opcode!("test"; AL, I/b),       // 0xa8: test
    opcode!("test"; rAX, I/z),      // 0xa9: test
    opcode!("stosb"; ),             // 0xaa: stosb
    opcode!("stosw"; ),             // 0xab: stosw
    opcode!("lodsb"; ),             // 0xac: lodsb
    opcode!("lodsw"; ),             // 0xad: lodsw
    opcode!("scasb"; ),             // 0xae: scasb
    opcode!("scasw"; ),             // 0xaf: scasw
    opcode!("mov"; AL, I/b),        // 0xb0: mov
    opcode!("mov"; CL, I/b),        // 0xb1: mov
    opcode!("mov"; DL, I/b),        // 0xb2: mov
    opcode!("mov"; BL, I/b),        // 0xb3: mov
    opcode!("mov"; AH, I/b),        // 0xb4: mov
    opcode!("mov"; CH, I/b),        // 0xb5: mov
    opcode!("mov"; DH, I/b),        // 0xb6: mov
    opcode!("mov"; BH, I/b),        // 0xb7: mov
    opcode!("mov"; rAX, I/v),       // 0xb8: mov
    opcode!("mov"; rCX, I/v),       // 0xb9: mov
    opcode!("mov"; rDX, I/v),       // 0xba: mov
    opcode!("mov"; rBX, I/v),       // 0xbb: mov
    opcode!("mov"; rSP, I/v),       // 0xbc: mov
    opcode!("mov"; rBP, I/v),       // 0xbd: mov
    opcode!("mov"; rSI, I/v),       // 0xbe: mov
    opcode!("mov"; rDI, I/v),       // 0xbf: mov
    opcode!(group 2; E/b, I/b),     // 0xc0: rcl
    opcode!(group 2; E/v, I/b),     // 0xc1: rcl
    opcode!("ret"; I/w; Force64),            // 0xc2: ret
    opcode!("ret"; ; Force64),               // 0xc3: ret
    opcode!("les"; G/z, M/p; Invalid64),       // 0xc4: les
    opcode!("lds"; G/z, M/p; Invalid64),       // 0xc5: lds
    opcode!(group 11; E/b, I/b),    // 0xc6: mov
    opcode!(group 11; E/v, I/z),    // 0xc7: mov
    opcode!("enter"; I/w, I/b),     // 0xc8: enter
    opcode!("leave"; ; Default64),             // 0xc9: leave
    opcode!("retf"; I/w),           // 0xca: retf
    opcode!("retf"; ),              // 0xcb: retf
    opcode!("int3"; ),              // 0xcc: int3
    opcode!("int"; I/b),            // 0xcd: int
    opcode!("into"; ; Invalid64),              // 0xce: into
    opcode!("iretw"; ),             // 0xcf: iretw
    opcode!(group 2; E/b, I/one),   // 0xd0: rcl
    opcode!(group 2; E/v, I/one),   // 0xd1: rcl
    opcode!(group 2; E/b, CL),      // 0xd2: rcl
    opcode!(group 2; E/v, CL),      // 0xd3: rcl
    opcode!("aam"; I/b; Invalid64),            // 0xd4: aam
    opcode!("aad"; I/b; Invalid64),            // 0xd5: aad
    opcode!("salc"; ),              // 0xd6: salc
    opcode!("xlatb"; ),             // 0xd7: xlatb
    escape!(),                      // 0xd8: x87
    escape!(),                      // 0xd9: x87
    escape!(),                      // 0xda: x87
    escape!(),                      // 0xdb: x87
    escape!(),                      // 0xdc: x87
    escape!(),                      // 0xdd: x87
    escape!(),                      // 0xde: x87
    escape!(),                      // 0xdf: x87
    opcode!("loopne"; J/b; Force64),         // 0xe0: loopne
    opcode!("loope"; J/b; Force64),          // 0xe1: loope
    opcode!("loop"; J/b; Force64),           // 0xe2: loop
    opcode!("jcxz"; J/b; Force64),           // 0xe3: jcxz
    opcode!("in"; AL, I/b),         // 0xe4: in
    opcode!("in"; eAX, I/b),        // 0xe5: in
    opcode!("out"; I/b, AL),        // 0xe6: out
    opcode!("out"; I/b, eAX),       // 0xe7: out
    opcode!("call"; J/z; Force64),           // 0xe8: call
    opcode!("jmp"; J/z; Force64),            // 0xe9: jmp
    opcode!("jmp"; A/p; Force64),            // 0xea: jmp
    opcode!("jmp"; J/b; Force64),            // 0xeb: jmp
    opcode!("in"; AL, DX),          // 0xec: in
    opcode!("in"; eAX, DX),         // 0xed: in
    opcode!("out"; DX, AL),         // 0xee: out
    opcode!("out"; DX, eAX),        // 0xef: out
    opcode!("lock"; ),              // 0xf0: lock
    opcode!("int1"; ),              // 0xf1: int1
    opcode!("repne"; ),             // 0xf2: repne
    opcode!("rep"; ),               // 0xf3: rep
    opcode!("hlt"; ),               // 0xf4: hlt
    opcode!("cmc"; ),               // 0xf5: cmc
    opcode!(group 3; E/b),          // 0xf6: div
    opcode!(group 3; E/v),          // 0xf7: div
    opcode!("clc"; ),               // 0xf8: clc
    opcode!("stc"; ),               // 0xf9: stc
    opcode!("cli"; ),               // 0xfa: cli
    opcode!("sti"; ),               // 0xfb: sti
    opcode!("cld"; ),               // 0xfc: cld
    opcode!("std"; ),               // 0xfd: std
    opcode!(group 4; E/b),          // 0xfe: dec
    opcode!(group 5; E/v),          // 0xff: call
];

pub static TWOBYTE_TABLE: [Opcode; 256] = [
    opcode!(group 6; ),             // 0x00
    opcode!(group 7; ),             // 0x01
    opcode!("lar"; G/v, E/w),       // 0x02: lar
    opcode!("lsl"; G/v, E/w),       // 0x03: lsl
    unused!(),                      // 0x04
    opcode!("syscall"; ; Only64),           // 0x05: syscall
    opcode!("clts"; ),              // 0x06: clts
    opcode!("sysret"; ; Only64),            // 0x07: sysret
    opcode!("invd"; ),              // 0x08: invd
    opcode!("wbinvd"; ),            // 0x09: wbinvd
    unused!(),                      // 0x0a
    opcode!("ud2"; ),               // 0x0b: ud2
    unused!(),                      // 0x0c
    opcode!("prefetch"; E/v),       // 0x0d: prefetch
    unused!(),                      // 0x0e
    unused!(),                      // 0x0f
    opcode!("vmovups"; V/ps, W/ps),        // 0x10: movups
    opcode!("vmovups"; W/ps, V/ps),        // 0x11: movups
    opcode!("vmovlps"; V/q, H/q, M/q),        // 0x12: movlps
    opcode!("vmovlps"; M/q, V/q),        // 0x13: movlps
    opcode!("vunpcklps"; V/x, H/x, W/x),   // 0x14: unpcklps
    opcode!("vunpckhps"; V/x, H/x, W/x),   // 0x15: unpckhps
    opcode!("vmovhps"; V/dq, H/q, M/q),     // 0x16: movhps
    opcode!("vmovhps"; M/q, V/q),        // 0x17: movhps
    opcode!(group 16; ),       // 0x18: prefetchnta
    unused!(),              // 0x19: nop
    unused!(),              // 0x1a: nop
    unused!(),              // 0x1b: nop
    unused!(),              // 0x1c: nop
    unused!(),              // 0x1d: nop
    unused!(),              // 0x1e: nop
    opcode!("nop"; E/v),              // 0x1f: nop
    opcode!("mov"; R/d, C/d),           // 0x20: mov
    opcode!("mov"; R/d, D/d),           // 0x21: mov
    opcode!("mov"; C/d, R/d),           // 0x22: mov
    opcode!("mov"; D/d, R/d),           // 0x23: mov
    unused!(),                         // 0x24
    unused!(),                         // 0x25
    unused!(),                         // 0x26
    unused!(),                         // 0x27
    opcode!("vmovaps"; V/ps, W/ps),        // 0x28: movaps
    opcode!("vmovaps"; W/ps, V/ps),        // 0x29: movaps
    opcode!("cvtpi2ps"; V/ps, Q/pi),      // 0x2a: cvtpi2ps
    opcode!("vmovntps"; M/ps, V/ps),       // 0x2b: movntps
    opcode!("cvttps2pi"; P/pi, W/ps),     // 0x2c: cvttps2pi
    opcode!("cvtps2pi"; P/ps, W/ps),    // 0x2d: cvtps2pi
    opcode!("vucomiss"; V/ss, W/ss),       // 0x2e: ucomiss
    opcode!("vcomiss"; V/ss, W/ss),        // 0x2f: comiss
    opcode!("wrmsr"; ),             // 0x30: wrmsr
    opcode!("rdtsc"; ),             // 0x31: rdtsc
    opcode!("rdmsr"; ),             // 0x32: rdmsr
    opcode!("rdpmc"; ),             // 0x33: rdpmc
    opcode!("sysenter"; ),          // 0x34: sysenter
    opcode!("sysexit"; ),           // 0x35: sysexit
    unused!(),                         // 0x36
    opcode!("getsec"; ),            // 0x37: getsec
    unused!(),                      // 0x38
    unused!(),                         // 0x39
    unused!(),                      // 0x3a
    unused!(),                         // 0x3b
    unused!(),                         // 0x3c
    unused!(),                         // 0x3d
    unused!(),                         // 0x3e
    unused!(),                         // 0x3f
    opcode!("cmovo"; G/v, E/v),     // 0x40: cmovo
    opcode!("cmovno"; G/v, E/v),    // 0x41: cmovno
    opcode!("cmovb"; G/v, E/v),     // 0x42: cmovb
    opcode!("cmovae"; G/v, E/v),    // 0x43: cmovae
    opcode!("cmovz"; G/v, E/v),     // 0x44: cmovz
    opcode!("cmovnz"; G/v, E/v),    // 0x45: cmovnz
    opcode!("cmovbe"; G/v, E/v),    // 0x46: cmovbe
    opcode!("cmova"; G/v, E/v),     // 0x47: cmova
    opcode!("cmovs"; G/v, E/v),     // 0x48: cmovs
    opcode!("cmovns"; G/v, E/v),    // 0x49: cmovns
    opcode!("cmovp"; G/v, E/v),     // 0x4a: cmovp
    opcode!("cmovnp"; G/v, E/v),    // 0x4b: cmovnp
    opcode!("cmovl"; G/v, E/v),     // 0x4c: cmovl
    opcode!("cmovge"; G/v, E/v),    // 0x4d: cmovge
    opcode!("cmovle"; G/v, E/v),    // 0x4e: cmovle
    opcode!("cmovg"; G/v, E/v),     // 0x4f: cmovg
    opcode!("vmovmskps"; G/y, U/ps),    // 0x50: movmskps
    opcode!("vsqrtps"; V/ps, W/ps),        // 0x51: sqrtps
    opcode!("vrsqrtps"; V/ps, W/ps),       // 0x52: rsqrtps
    opcode!("vrcpps"; V/ps, W/ps),         // 0x53: rcpps
    opcode!("vandps"; V/ps, H/ps, W/ps),      // 0x54: andps
    opcode!("vandnps"; V/ps, H/ps, W/ps),     // 0x55: andnps
    opcode!("vorps"; V/ps, H/ps, W/ps),       // 0x56: orps
    opcode!("vxorps"; V/ps, H/ps, W/ps),      // 0x57: xorps
    opcode!("vaddps"; V/ps, H/ps, W/ps),      // 0x58: addps
    opcode!("vmulps"; V/ps, H/ps, W/ps),      // 0x59: mulps
    opcode!("vcvtps2pd"; V/pd, W/ps),   // 0x5a: cvtps2pd
    opcode!("vcvtdq2ps"; V/ps, W/dq),      // 0x5b: cvtdq2ps
    opcode!("vsubps"; V/ps, H/ps, W/ps),      // 0x5c: subps
    opcode!("vminps"; V/ps, H/ps, W/ps),      // 0x5d: minps
    opcode!("vdivps"; V/ps, H/ps, W/ps),      // 0x5e: divps
    opcode!("vmaxps"; V/ps, H/ps, W/ps),      // 0x5f: maxps
    opcode!("punpcklbw"; P/q, Q/d),     // 0x60: punpcklbw
    opcode!("punpcklwd"; P/q, Q/d),     // 0x61: punpcklwd
    opcode!("punpckldq"; P/q, Q/d),     // 0x62: punpckldq
    opcode!("packsswb"; P/q, Q/q),      // 0x63: packsswb
    opcode!("pcmpgtb"; P/q, Q/q),       // 0x64: pcmpgtb
    opcode!("pcmpgtw"; P/q, Q/q),       // 0x65: pcmpgtw
    opcode!("pcmpgtd"; P/q, Q/q),       // 0x66: pcmpgtd
    opcode!("packuswb"; P/q, Q/q),      // 0x67: packuswb
    opcode!("punpckhbw"; P/d, Q/q),     // 0x68: punpckhbw
    opcode!("punpckhwd"; P/d, Q/q),     // 0x69: punpckhwd
    opcode!("punpckhdq"; P/d, Q/q),     // 0x6a: punpckhdq
    opcode!("packssdw"; P/d, Q/q),      // 0x6b: packssdw
    unused!(),                         // 0x6c
    unused!(),                         // 0x6d
    opcode!("movd"; P/d, E/y),        // 0x6e: movd
    opcode!("movq"; P/q, Q/q),          // 0x6f: movq
    opcode!("pshufw"; P/q, Q/q, I/b),   // 0x70: pshufw
    opcode!(group 12; ),       // 0x71: psllw
    opcode!(group 13; ),       // 0x72: pslld
    opcode!(group 14; ),       // 0x73: psllq
    opcode!("pcmpeqb"; P/q, Q/q),       // 0x74: pcmpeqb
    opcode!("pcmpeqw"; P/q, Q/q),       // 0x75: pcmpeqw
    opcode!("pcmpeqd"; P/q, Q/q),       // 0x76: pcmpeqd
    opcode!("emms"; ),              // 0x77: emms
    opcode!("vmread"; E/y, G/y),    // 0x78: vmread
    opcode!("vmwrite"; G/y, E/y),   // 0x79: vmwrite
    unused!(),                         // 0x7a
    unused!(),                         // 0x7b
    unused!(),                         // 0x7c
    unused!(),                         // 0x7d
    opcode!("movd"; E/y, P/d),        // 0x7e: movd
    opcode!("movq"; Q/q, P/q),          // 0x7f: movq
    opcode!("jo"; J/z; Force64),             // 0x80: jo
    opcode!("jno"; J/z; Force64),            // 0x81: jno
    opcode!("jb"; J/z; Force64),             // 0x82: jb
    opcode!("jae"; J/z; Force64),            // 0x83: jae
    opcode!("jz"; J/z; Force64),             // 0x84: jz
    opcode!("jnz"; J/z; Force64),            // 0x85: jnz
    opcode!("jbe"; J/z; Force64),            // 0x86: jbe
    opcode!("ja"; J/z; Force64),             // 0x87: ja
    opcode!("js"; J/z; Force64),             // 0x88: js
    opcode!("jns"; J/z; Force64),            // 0x89: jns
    opcode!("jp"; J/z; Force64),             // 0x8a: jp
    opcode!("jnp"; J/z; Force64),            // 0x8b: jnp
    opcode!("jl"; J/z; Force64),             // 0x8c: jl
    opcode!("jge"; J/z; Force64),            // 0x8d: jge
    opcode!("jle"; J/z; Force64),            // 0x8e: jle
    opcode!("jg"; J/z; Force64),             // 0x8f: jg
    opcode!("seto"; E/b),           // 0x90: seto
    opcode!("setno"; E/b),          // 0x91: setno
    opcode!("setb"; E/b),           // 0x92: setb
    opcode!("setae"; E/b),          // 0x93: setae
    opcode!("setz"; E/b),           // 0x94: setz
    opcode!("setnz"; E/b),          // 0x95: setnz
    opcode!("setbe"; E/b),          // 0x96: setbe
    opcode!("seta"; E/b),           // 0x97: seta
    opcode!("sets"; E/b),           // 0x98: sets
    opcode!("setns"; E/b),          // 0x99: setns
    opcode!("setp"; E/b),           // 0x9a: setp
    opcode!("setnp"; E/b),          // 0x9b: setnp
    opcode!("setl"; E/b),           // 0x9c: setl
    opcode!("setge"; E/b),          // 0x9d: setge
    opcode!("setle"; E/b),          // 0x9e: setle
    opcode!("setg"; E/b),           // 0x9f: setg
    opcode!("push"; FS; Default64),            // 0xa0: push
    opcode!("pop"; FS; Default64),             // 0xa1: pop
    opcode!("cpuid"; ),             // 0xa2: cpuid
    opcode!("bt"; E/v, G/v),        // 0xa3: bt
    opcode!("shld"; E/v, G/v, I/b), // 0xa4: shld
    opcode!("shld"; E/v, G/v, CL),  // 0xa5: shld
    opcode!("montmul"; ),           // 0xa6: montmul
    opcode!("xcryptecb"; ),         // 0xa7: xcryptecb
    opcode!("push"; GS; Default64),            // 0xa8: push
    opcode!("pop"; GS; Default64),             // 0xa9: pop
    opcode!("rsm"; ),               // 0xaa: rsm
    opcode!("bts"; E/v, G/v),       // 0xab: bts
    opcode!("shrd"; E/v, G/v, I/b), // 0xac: shrd
    opcode!("shrd"; E/v, G/v, CL),  // 0xad: shrd
    opcode!(group 15; ),          // 0xae: clflush
    opcode!("imul"; G/v, E/v),      // 0xaf: imul
    opcode!("cmpxchg"; E/b, G/b),   // 0xb0: cmpxchg
    opcode!("cmpxchg"; E/v, G/v),   // 0xb1: cmpxchg
    opcode!("lss"; G/v, M/p),         // 0xb2: lss
    opcode!("btr"; E/v, G/v),       // 0xb3: btr
    opcode!("lfs"; G/z, M/p),         // 0xb4: lfs
    opcode!("lgs"; G/z, M/p),         // 0xb5: lgs
    opcode!("movzx"; G/v, E/b),     // 0xb6: movzx
    opcode!("movzx"; G/y, E/w),     // 0xb7: movzx
    unused!(),                         // 0xb8
    opcode!(group 10; ),                         // 0xb9
    opcode!(group 8; E/v, I/b),        // 0xba: bt
    opcode!("btc"; E/v, G/v),       // 0xbb: btc
    opcode!("bsf"; G/v, E/v),       // 0xbc: bsf
    opcode!("bsr"; G/v, E/v),       // 0xbd: bsr
    opcode!("movsx"; G/v, E/b),     // 0xbe: movsx
    opcode!("movsx"; G/v, E/w),     // 0xbf: movsx
    opcode!("xadd"; E/b, G/b),      // 0xc0: xadd
    opcode!("xadd"; E/v, G/v),      // 0xc1: xadd
    opcode!("vcmpps"; V/ps, H/ps, W/ps, I/b), // 0xc2: cmpps
    opcode!("movnti"; M/y, G/y),    // 0xc3: movnti
    opcode!("pinsrw"; P/q, R/y),    // 0xc4: pinsrw
    opcode!("pextrw"; G/d, N/q, I/b), // 0xc5: pextrw
    opcode!("vshufps"; V/ps, H/ps, W/ps, I/b),// 0xc6: shufps
    opcode!(group 9; ),        // 0xc7: cmpxchg8b
    opcode!("bswap"; rAX),          // 0xc8: bswap
    opcode!("bswap"; rCX),          // 0xc9: bswap
    opcode!("bswap"; rDX),          // 0xca: bswap
    opcode!("bswap"; rBX),          // 0xcb: bswap
    opcode!("bswap"; rSP),          // 0xcc: bswap
    opcode!("bswap"; rBP),          // 0xcd: bswap
    opcode!("bswap"; rSI),          // 0xce: bswap
    opcode!("bswap"; rDI),          // 0xcf: bswap
    unused!(),                      // 0xd0
    opcode!("psrlw"; P/q, Q/q),         // 0xd1: psrlw
    opcode!("psrld"; P/q, Q/q),         // 0xd2: psrld
    opcode!("psrlq"; P/q, Q/q),         // 0xd3: psrlq
    opcode!("paddq"; P/q, Q/q),         // 0xd4: paddq
    opcode!("pmullw"; P/q, Q/q),        // 0xd5: pmullw
    unused!(),                         // 0xd6
    opcode!("pmovmskb"; G/d, N/q),    // 0xd7: pmovmskb
    opcode!("psubusb"; P/q, Q/q),       // 0xd8: psubusb
    opcode!("psubusw"; P/q, Q/q),       // 0xd9: psubusw
    opcode!("pminub"; P/q, Q/q),        // 0xda: pminub
    opcode!("pand"; P/q, Q/q),          // 0xdb: pand
    opcode!("paddusb"; P/q, Q/q),       // 0xdc: paddusb
    opcode!("paddusw"; P/q, Q/q),       // 0xdd: paddusw
    opcode!("pmaxub"; P/q, Q/q),        // 0xde: pmaxub
    opcode!("pandn"; P/q, Q/q),         // 0xdf: pandn
    opcode!("pavgb"; P/q, Q/q),         // 0xe0: pavgb
    opcode!("psraw"; P/q, Q/q),         // 0xe1: psraw
    opcode!("psrad"; P/q, Q/q),         // 0xe2: psrad
    opcode!("pavgw"; P/q, Q/q),         // 0xe3: pavgw
    opcode!("pmulhuw"; P/q, Q/q),       // 0xe4: pmulhuw
    opcode!("pmulhw"; P/q, Q/q),        // 0xe5: pmulhw
    unused!(),                         // 0xe6
    opcode!("movntq"; M/q, P/q),        // 0xe7: movntq
    opcode!("psubsb"; P/q, Q/q),        // 0xe8: psubsb
    opcode!("psubsw"; P/q, Q/q),        // 0xe9: psubsw
    opcode!("pminsw"; P/q, Q/q),        // 0xea: pminsw
    opcode!("por"; P/q, Q/q),           // 0xeb: por
    opcode!("paddsb"; P/q, Q/q),        // 0xec: paddsb
    opcode!("paddsw"; P/q, Q/q),        // 0xed: paddsw
    opcode!("pmaxsw"; P/q, Q/q),        // 0xee: pmaxsw
    opcode!("pxor"; P/q, Q/q),          // 0xef: pxor
    unused!(),                         // 0xf0
    opcode!("psllw"; P/q, Q/q),         // 0xf1: psllw
    opcode!("pslld"; P/q, Q/q),         // 0xf2: pslld
    opcode!("psllq"; P/q, Q/q),         // 0xf3: psllq
    opcode!("pmuludq"; P/q, Q/q),       // 0xf4: pmuludq
    opcode!("pmaddwd"; P/q, Q/q),       // 0xf5: pmaddwd
    opcode!("psadbw"; P/q, Q/q),        // 0xf6: psadbw
    opcode!("maskmovq"; P/q, N/q),      // 0xf7: maskmovq
    opcode!("psubb"; P/q, Q/q),         // 0xf8: psubb
    opcode!("psubw"; P/q, Q/q),         // 0xf9: psubw
    opcode!("psubd"; P/q, Q/q),         // 0xfa: psubd
    opcode!("psubq"; P/q, Q/q),         // 0xfb: psubq
    opcode!("paddb"; P/q, Q/q),         // 0xfc: paddb
    opcode!("paddw"; P/q, Q/q),         // 0xfd: paddw
    opcode!("paddd"; P/q, Q/q),         // 0xfe: paddd
    unused!(),                         // 0xff:
];

pub static TWOBYTE_66_TABLE: [Opcode; 256] = [
    unused!(),                      // 0x00
    unused!(),                      // 0x01
    unused!(),                      // 0x02
    unused!(),                      // 0x03
    unused!(),                      // 0x04
    unused!(),                      // 0x05
    unused!(),                      // 0x06
    unused!(),                      // 0x07
    unused!(),                      // 0x08
    unused!(),                      // 0x09
    unused!(),                      // 0x0a
    unused!(),                      // 0x0b
    unused!(),                      // 0x0c
    unused!(),                      // 0x0d
    unused!(),                      // 0x0e
    unused!(),                      // 0x0f
    opcode!("vmovupd"; V/pd, W/pd),        // 0x10: movupd
    opcode!("vmovupd"; W/pd, V/pd),        // 0x11: movupd
    opcode!("vmovlpd"; V/q, H/q, M/q),        // 0x12: movlpd
    opcode!("vmovlpd"; M/q, V/q),        // 0x13: movlpd
    opcode!("vunpcklpd"; V/x, H/x, W/x),   // 0x14: unpcklpd
    opcode!("vunpckhpd"; V/x, H/x, W/x),   // 0x15: unpckhpd
    opcode!("vmovhpd"; V/dq, H/q, M/q),     // 0x16: movhpd
    opcode!("vmovhpd"; M/q, V/q),        // 0x17: movhpd
    unused!(),                      // 0x18
    unused!(),                      // 0x19
    unused!(),                      // 0x1a
    unused!(),                      // 0x1b
    unused!(),                      // 0x1c
    unused!(),                      // 0x1d
    unused!(),                      // 0x1e
    unused!(),                      // 0x1f
    unused!(),                      // 0x20
    unused!(),                      // 0x21
    unused!(),                      // 0x22
    unused!(),                      // 0x23
    unused!(),                      // 0x24
    unused!(),                      // 0x25
    unused!(),                      // 0x26
    unused!(),                      // 0x27
    opcode!("vmovapd"; V/pd, W/pd),        // 0x28: movapd
    opcode!("vmovapd"; W/pd, V/pd),        // 0x29: movapd
    opcode!("cvtpi2pd"; V/pd, Q/pi),      // 0x2a: cvtpi2pd
    opcode!("vmovntpd"; M/pd, V/pd),       // 0x2b: movntpd
    opcode!("cvttpd2pi"; P/pi, W/pd),     // 0x2c: cvttpd2pi
    opcode!("cvtpd2pi"; Q/pi, W/pd),      // 0x2d: cvtpd2pi
    opcode!("vucomisd"; V/sd, W/sd),       // 0x2e: ucomisd
    opcode!("vcomisd"; V/sd, W/sd),  // 0x2f: comisd
    unused!(),                      // 0x30
    unused!(),                      // 0x31
    unused!(),                      // 0x32
    unused!(),                      // 0x33
    unused!(),                      // 0x34
    unused!(),                      // 0x35
    unused!(),                      // 0x36
    unused!(),                      // 0x37
    unused!(),                      // 0x38
    unused!(),                      // 0x39
    unused!(),                      // 0x3a
    unused!(),                      // 0x3b
    unused!(),                      // 0x3c
    unused!(),                      // 0x3d
    unused!(),                      // 0x3e
    unused!(),                      // 0x3f
    unused!(),                      // 0x40
    unused!(),                      // 0x41
    unused!(),                      // 0x42
    unused!(),                      // 0x43
    unused!(),                      // 0x44
    unused!(),                      // 0x45
    unused!(),                      // 0x46
    unused!(),                      // 0x47
    unused!(),                      // 0x48
    unused!(),                      // 0x49
    unused!(),                      // 0x4a
    unused!(),                      // 0x4b
    unused!(),                      // 0x4c
    unused!(),                      // 0x4d
    unused!(),                      // 0x4e
    unused!(),                      // 0x4f
    opcode!("vmovmskpd"; G/d, U/pd),    // 0x50: movmskpd
    opcode!("vsqrtpd"; V/pd, W/pd),        // 0x51: sqrtpd
    unused!(),                      // 0x52
    unused!(),                      // 0x53
    opcode!("vandpd"; V/pd, H/pd, W/pd),      // 0x54: andpd
    opcode!("vandnpd"; V/pd, H/pd, W/pd),     // 0x55: andnpd
    opcode!("vorpd"; V/pd, H/pd, W/pd),       // 0x56: orpd
    opcode!("vxorpd"; V/pd, H/pd, W/pd),      // 0x57: xorpd
    opcode!("vaddpd"; V/pd, H/pd, W/pd),      // 0x58: addpd
    opcode!("vmulpd"; V/pd, H/pd, W/pd),      // 0x59: mulpd
    opcode!("cvtpd2ps"; V/ps, W/pd),   // 0x5a: cvtpd2ps
    opcode!("cvtps2dq"; V/dq, W/pd),      // 0x5b: cvtps2dq
    opcode!("vsubpd"; V/pd, H/pd, W/pd),      // 0x5c: subpd
    opcode!("vminpd"; V/pd, H/pd, W/pd),      // 0x5d: minpd
    opcode!("vdivpd"; V/pd, H/pd, W/pd),      // 0x5e: divpd
    opcode!("vmaxpd"; V/pd, H/pd, W/pd),      // 0x5f: maxpd
    opcode!("vpunpcklbw"; V/x, H/x, W/x),  // 0x60: punpcklbw
    opcode!("vpunpcklwd"; V/x, H/x, W/x),  // 0x61: punpcklwd
    opcode!("vpunpckldq"; V/x, H/x, W/x),  // 0x62: punpckldq
    opcode!("vpacksswb"; V/x, H/x, W/x),   // 0x63: packsswb
    opcode!("vpcmpgtb"; V/x, H/x, W/x),    // 0x64: pcmpgtb
    opcode!("vpcmpgtw"; V/x, H/x, W/x),    // 0x65: pcmpgtw
    opcode!("vpcmpgtd"; V/x, H/x, W/x),    // 0x66: pcmpgtd
    opcode!("vpackuswb"; V/x, H/x, W/x),   // 0x67: packuswb
    opcode!("punpckhbw"; V/x, H/x, W/x),  // 0x68: punpckhbw
    opcode!("punpckhwd"; V/x, H/x, W/x),  // 0x69: punpckhwd
    opcode!("punpckhdq"; V/x, H/x, W/x),  // 0x6a: punpckhdq
    opcode!("packssdw"; V/x, H/x, W/x),   // 0x6b: packssdw
    opcode!("punpcklqdq"; V/x, H/x, W/x), // 0x6c: punpcklqdq
    opcode!("punpckhqdq"; V/x, H/x, W/x), // 0x6d: punpckhqdq
    opcode!("movd"; V/y, E/y),        // 0x6e: movd
    opcode!("movdqa"; V/x, W/x),        // 0x6f: movdqa
    opcode!("pshufd"; V/x, W/x, I/b),   // 0x70: pshufd
    opcode!(group 12; ),       // 0x71: psllw
    opcode!(group 13; ),       // 0x72: pslld
    opcode!(group 14; ),       // 0x73: psllq
    opcode!("vpcmpeqb"; V/x, H/x, W/x),    // 0x74: pcmpeqb
    opcode!("vpcmpeqw"; V/x, H/x, W/x),    // 0x75: pcmpeqw
    opcode!("vpcmpeqd"; V/x, H/x, W/x),    // 0x76: pcmpeqd
    unused!(),                      // 0x77
    unused!(),                      // 0x78
    unused!(),                      // 0x79
    unused!(),                      // 0x7a
    unused!(),                      // 0x7b
    opcode!("haddpd"; V/pd, H/pd, W/pd),     // 0x7c: haddpd
    opcode!("hsubpd"; V/pd, H/pd, W/pd),     // 0x7d: hsubpd
    opcode!("movd"; E/y, V/y),        // 0x7e: movd
    opcode!("movdqa"; W/x, V/x),        // 0x7f: movdqa
    unused!(),                      // 0x80
    unused!(),                      // 0x81
    unused!(),                      // 0x82
    unused!(),                      // 0x83
    unused!(),                      // 0x84
    unused!(),                      // 0x85
    unused!(),                      // 0x86
    unused!(),                      // 0x87
    unused!(),                      // 0x88
    unused!(),                      // 0x89
    unused!(),                      // 0x8a
    unused!(),                      // 0x8b
    unused!(),                      // 0x8c
    unused!(),                      // 0x8d
    unused!(),                      // 0x8e
    unused!(),                      // 0x8f
    unused!(),                      // 0x90
    unused!(),                      // 0x91
    unused!(),                      // 0x92
    unused!(),                      // 0x93
    unused!(),                      // 0x94
    unused!(),                      // 0x95
    unused!(),                      // 0x96
    unused!(),                      // 0x97
    unused!(),                      // 0x98
    unused!(),                      // 0x99
    unused!(),                      // 0x9a
    unused!(),                      // 0x9b
    unused!(),                      // 0x9c
    unused!(),                      // 0x9d
    unused!(),                      // 0x9e
    unused!(),                      // 0x9f
    unused!(),                      // 0xa0
    unused!(),                      // 0xa1
    unused!(),                      // 0xa2
    unused!(),                      // 0xa3
    unused!(),                      // 0xa4
    unused!(),                      // 0xa5
    unused!(),                      // 0xa6
    unused!(),                      // 0xa7
    unused!(),                      // 0xa8
    unused!(),                      // 0xa9
    unused!(),                      // 0xaa
    unused!(),                      // 0xab
    unused!(),                      // 0xac
    unused!(),                      // 0xad
    unused!(),                      // 0xae
    unused!(),                      // 0xaf
    unused!(),                      // 0xb0
    unused!(),                      // 0xb1
    unused!(),                      // 0xb2
    unused!(),                      // 0xb3
    unused!(),                      // 0xb4
    unused!(),                      // 0xb5
    unused!(),                      // 0xb6
    unused!(),                      // 0xb7
    unused!(),                      // 0xb8
    unused!(),                      // 0xb9
    unused!(),                      // 0xba
    unused!(),                      // 0xbb
    unused!(),                      // 0xbc
    unused!(),                      // 0xbd
    unused!(),                      // 0xbe
    unused!(),                      // 0xbf
    unused!(),                      // 0xc0
    unused!(),                      // 0xc1
    opcode!("vcmppd"; V/pd, H/pd, W/pd, I/b), // 0xc2: cmppd
    unused!(),                      // 0xc3
    opcode!("vpinsrw"; V/pd, H/dq, I/b),// 0xc4: pinsrw
    opcode!("vpextrw"; G/d, U/dq, I/b), // 0xc5: pextrw
    opcode!("vshufpd"; V/pd, H/pd, W/pd, I/b),// 0xc6: shufpd
    opcode!(group 9; ),        // 0xc7
    unused!(),                      // 0xc8
    unused!(),                      // 0xc9
    unused!(),                      // 0xca
    unused!(),                      // 0xcb
    unused!(),                      // 0xcc
    unused!(),                      // 0xcd
    unused!(),                      // 0xce
    unused!(),                      // 0xcf
    opcode!("vaddsubpd"; V/pd, H/pd, W/pd),   // 0xd0: addsubpd
    opcode!("vpsrlw"; V/x, H/x, W/x),      // 0xd1: psrlw
    opcode!("vpsrld"; V/x, H/x, W/x),      // 0xd2: psrld
    opcode!("vpsrlq"; V/x, H/x, W/x),      // 0xd3: psrlq
    opcode!("vpaddq"; V/x, H/x, W/x),      // 0xd4: paddq
    opcode!("vpmullw"; V/x, H/x, W/x),     // 0xd5: pmullw
    opcode!("vmovq"; W/q, V/q),          // 0xd6: movq
    opcode!("vpmovmskb"; G/d, U/x),    // 0xd7: pmovmskb
    opcode!("vpsubusb"; V/x, H/x, W/x),    // 0xd8: psubusb
    opcode!("vpsubusw"; V/x, H/x, W/x),    // 0xd9: psubusw
    opcode!("vpminub"; V/x, H/x, W/x),     // 0xda: pminub
    opcode!("vpand"; V/x, H/x, W/x),       // 0xdb: pand
    opcode!("vpaddusb"; V/x, H/x, W/x),    // 0xdc: paddusb
    opcode!("vpaddusw"; V/x, H/x, W/x),    // 0xdd: paddusw
    opcode!("vpmaxub"; V/x, H/x, W/x),     // 0xde: pmaxub
    opcode!("vpandn"; V/x, H/x, W/x),      // 0xdf: pandn
    opcode!("vpavgb"; V/x, H/x, W/x),      // 0xe0: pavgb
    opcode!("vpsraw"; V/x, H/x, W/x),      // 0xe1: psraw
    opcode!("vpsrad"; V/x, H/x, W/x),      // 0xe2: psrad
    opcode!("vpavgw"; V/x, H/x, W/x),      // 0xe3: pavgw
    opcode!("vpmulhuw"; V/x, H/x, W/x),    // 0xe4: pmulhuw
    opcode!("vpmulhw"; V/x, H/x, W/x),     // 0xe5: pmulhw
    opcode!("cvttpd2dq"; V/x, W/pd),  // 0xe6: cvttpd2dq
    opcode!("movntdq"; M/x, V/x),       // 0xe7: movntdq
    opcode!("vpsubsb"; V/x, H/x, W/x),     // 0xe8: psubsb
    opcode!("vpsubsw"; V/x, H/x, W/x),     // 0xe9: psubsw
    opcode!("vpminsw"; V/x, H/x, W/x),     // 0xea: pminsw
    opcode!("vpor"; V/x, H/x, W/x),        // 0xeb: por
    opcode!("vpaddsb"; V/x, H/x, W/x),     // 0xec: paddsb
    opcode!("vpaddsw"; V/x, H/x, W/x),     // 0xed: paddsw
    opcode!("vpmaxsw"; V/x, H/x, W/x),     // 0xee: pmaxsw
    opcode!("vpxor"; V/x, H/x, W/x),       // 0xef: pxor
    unused!(),                      // 0xf0
    opcode!("vpsllw"; V/x, W/x),         // 0xf1: psllw
    opcode!("vpslld"; V/x, W/x),         // 0xf2: pslld
    opcode!("vpsllq"; V/x, W/x),         // 0xf3: psllq
    opcode!("vpmuludq"; V/x, W/x),       // 0xf4: pmuludq
    opcode!("vpmaddwd"; V/x, H/x, W/x),    // 0xf5: pmaddwd
    opcode!("vpsadbw"; V/x, H/x, W/x),     // 0xf6: psadbw
    opcode!("vmaskmovdqu"; V/dq, U/dq),    // 0xf7: maskmovdqu
    opcode!("vpsubb"; V/x, H/x, W/x),      // 0xf8: psubb
    opcode!("vpsubw"; V/x, H/x, W/x),      // 0xf9: psubw
    opcode!("vpsubd"; V/x, H/x, W/x),      // 0xfa: psubd
    opcode!("vpsubq"; V/x, H/x, W/x),      // 0xfb: psubq
    opcode!("vpaddb"; V/x, H/x, W/x),      // 0xfc: paddb
    opcode!("vpaddw"; V/x, H/x, W/x),      // 0xfd: paddw
    opcode!("vpaddd"; V/x, H/x, W/x),      // 0xfe: paddd
    unused!(),                      // 0xff
];

pub static TWOBYTE_F3_TABLE: [Opcode; 256] = [
    unused!(),                      // 0x00
    unused!(),                      // 0x01
    unused!(),                      // 0x02
    unused!(),                      // 0x03
    unused!(),                      // 0x04
    unused!(),                      // 0x05
    unused!(),                      // 0x06
    unused!(),                      // 0x07
    unused!(),                      // 0x08
    unused!(),                      // 0x09
    unused!(),                      // 0x0a
    unused!(),                      // 0x0b
    unused!(),                      // 0x0c
    unused!(),                      // 0x0d
    unused!(),                      // 0x0e
    unused!(),                      // 0x0f
    opcode!("vmovss"; V/x, H/x, W/ss),       // 0x10: movss
    opcode!("vmovss"; W/ss, H/x, V/ss),         // 0x11: movss
    opcode!("vmovsldup"; V/x, W/x),      // 0x12: movsldup
    unused!(),                      // 0x13
    unused!(),                      // 0x14
    unused!(),                      // 0x15
    opcode!("vmovshdup"; V/x, W/x),      // 0x16: movshdup
    unused!(),                      // 0x17
    unused!(),                      // 0x18
    unused!(),                      // 0x19
    unused!(),                      // 0x1a
    unused!(),                      // 0x1b
    unused!(),                      // 0x1c
    unused!(),                      // 0x1d
    unused!(),                      // 0x1e
    unused!(),                      // 0x1f
    unused!(),                      // 0x20
    unused!(),                      // 0x21
    unused!(),                      // 0x22
    unused!(),                      // 0x23
    unused!(),                      // 0x24
    unused!(),                      // 0x25
    unused!(),                      // 0x26
    unused!(),                      // 0x27
    unused!(),                      // 0x28
    unused!(),                      // 0x29
    opcode!("cvtsi2ss"; V/ss, H/ss, E/y), // 0x2a: cvtsi2ss
    unused!(),                      // 0x2b
    opcode!("cvttss2si"; G/y, W/ss), // 0x2c: cvttss2si
    opcode!("cvtss2si"; G/y, W/ss),  // 0x2d: cvtss2si
    unused!(),                      // 0x2e
    unused!(),                      // 0x2f
    unused!(),                      // 0x30
    unused!(),                      // 0x31
    unused!(),                      // 0x32
    unused!(),                      // 0x33
    unused!(),                      // 0x34
    unused!(),                      // 0x35
    unused!(),                      // 0x36
    unused!(),                      // 0x37
    unused!(),                      // 0x38
    unused!(),                      // 0x39
    unused!(),                      // 0x3a
    unused!(),                      // 0x3b
    unused!(),                      // 0x3c
    unused!(),                      // 0x3d
    unused!(),                      // 0x3e
    unused!(),                      // 0x3f
    unused!(),                      // 0x40
    unused!(),                      // 0x41
    unused!(),                      // 0x42
    unused!(),                      // 0x43
    unused!(),                      // 0x44
    unused!(),                      // 0x45
    unused!(),                      // 0x46
    unused!(),                      // 0x47
    unused!(),                      // 0x48
    unused!(),                      // 0x49
    unused!(),                      // 0x4a
    unused!(),                      // 0x4b
    unused!(),                      // 0x4c
    unused!(),                      // 0x4d
    unused!(),                      // 0x4e
    unused!(),                      // 0x4f
    unused!(),                      // 0x50
    opcode!("vsqrtss"; V/ss, H/ss, W/ss),     // 0x51: sqrtss
    opcode!("vrsqrtss"; V/ss, H/ss, W/ss),       // 0x52: rsqrtss
    opcode!("vrcpss"; V/ss, H/ss, W/ss),         // 0x53: rcpss
    unused!(),                      // 0x54
    unused!(),                      // 0x55
    unused!(),                      // 0x56
    unused!(),                      // 0x57
    opcode!("vaddss"; V/ss, H/ss, W/ss),      // 0x58: addss
    opcode!("vmulss"; V/ss, H/ss, W/ss),      // 0x59: mulss
    opcode!("vcvtss2sd"; V/sd, H/x, W/ss),   // 0x5a: cvtss2sd
    opcode!("vcvttps2dq"; V/dq, W/ps),     // 0x5b: cvttps2dq
    opcode!("vsubss"; V/ss, H/ss, W/ss),      // 0x5c: subss
    opcode!("vminss"; V/ss, H/ss, W/ss),    // 0x5d: minss
    opcode!("vdivss"; V/ss, H/ss, W/ss),    // 0x5e: divss
    opcode!("vmaxss"; V/ss, H/ss, W/ss),      // 0x5f: maxss
    unused!(),                      // 0x60
    unused!(),                      // 0x61
    unused!(),                      // 0x62
    unused!(),                      // 0x63
    unused!(),                      // 0x64
    unused!(),                      // 0x65
    unused!(),                      // 0x66
    unused!(),                      // 0x67
    unused!(),                      // 0x68
    unused!(),                      // 0x69
    unused!(),                      // 0x6a
    unused!(),                      // 0x6b
    unused!(),                      // 0x6c
    unused!(),                      // 0x6d
    unused!(),                      // 0x6e
    opcode!("movdqu"; V/x, W/x),        // 0x6f: movdqu
    opcode!("pshufhw"; V/x, W/x, I/b),  // 0x70: pshufhw
    unused!(),                      // 0x71
    unused!(),                      // 0x72
    unused!(),                      // 0x73
    unused!(),                      // 0x74
    unused!(),                      // 0x75
    unused!(),                      // 0x76
    unused!(),                      // 0x77
    unused!(),                      // 0x78
    unused!(),                      // 0x79
    unused!(),                      // 0x7a
    unused!(),                      // 0x7b
    unused!(),                      // 0x7c
    unused!(),                      // 0x7d
    opcode!("movq"; V/q, W/q),          // 0x7e: movq
    opcode!("movdqu"; W/x, V/x),        // 0x7f: movdqu
    unused!(),                      // 0x80
    unused!(),                      // 0x81
    unused!(),                      // 0x82
    unused!(),                      // 0x83
    unused!(),                      // 0x84
    unused!(),                      // 0x85
    unused!(),                      // 0x86
    unused!(),                      // 0x87
    unused!(),                      // 0x88
    unused!(),                      // 0x89
    unused!(),                      // 0x8a
    unused!(),                      // 0x8b
    unused!(),                      // 0x8c
    unused!(),                      // 0x8d
    unused!(),                      // 0x8e
    unused!(),                      // 0x8f
    unused!(),                      // 0x90
    unused!(),                      // 0x91
    unused!(),                      // 0x92
    unused!(),                      // 0x93
    unused!(),                      // 0x94
    unused!(),                      // 0x95
    unused!(),                      // 0x96
    unused!(),                      // 0x97
    unused!(),                      // 0x98
    unused!(),                      // 0x99
    unused!(),                      // 0x9a
    unused!(),                      // 0x9b
    unused!(),                      // 0x9c
    unused!(),                      // 0x9d
    unused!(),                      // 0x9e
    unused!(),                      // 0x9f
    unused!(),                      // 0xa0
    unused!(),                      // 0xa1
    unused!(),                      // 0xa2
    unused!(),                      // 0xa3
    unused!(),                      // 0xa4
    unused!(),                      // 0xa5
    unused!(),                      // 0xa6
    unused!(),                      // 0xa7
    unused!(),                      // 0xa8
    unused!(),                      // 0xa9
    unused!(),                      // 0xaa
    unused!(),                      // 0xab
    unused!(),                      // 0xac
    unused!(),                      // 0xad
    opcode!(group 15; ),          // 0xae: group 15
    unused!(),                      // 0xaf
    unused!(),                      // 0xb0
    unused!(),                      // 0xb1
    unused!(),                      // 0xb2
    unused!(),                      // 0xb3
    unused!(),                      // 0xb4
    unused!(),                      // 0xb5
    unused!(),                      // 0xb6
    unused!(),                      // 0xb7
    opcode!("popcnt"; G/v, E/v),    // 0xb8: popcnt
    unused!(),                      // 0xb9
    unused!(),                      // 0xba
    unused!(),                      // 0xbb
    opcode!("tzcnt"; G/v, E/v),     // 0xbc
    opcode!("lzcnt"; G/v, E/v),     // 0xbd
    unused!(),                      // 0xbe
    unused!(),                      // 0xbf
    unused!(),                      // 0xc0
    unused!(),                      // 0xc1
    opcode!("vcmpss"; V/ss, H/ss, W/ss, I/b), // 0xc2: cmpss
    unused!(),                      // 0xc3
    unused!(),                      // 0xc4
    unused!(),                      // 0xc5
    unused!(),                      // 0xc6
    opcode!(group 9; ),          // 0xc7: vmxon
    unused!(),                      // 0xc8
    unused!(),                      // 0xc9
    unused!(),                      // 0xca
    unused!(),                      // 0xcb
    unused!(),                      // 0xcc
    unused!(),                      // 0xcd
    unused!(),                      // 0xce
    unused!(),                      // 0xcf
    unused!(),                      // 0xd0
    unused!(),                      // 0xd1
    unused!(),                      // 0xd2
    unused!(),                      // 0xd3
    unused!(),                      // 0xd4
    unused!(),                      // 0xd5
    opcode!("movq2dq"; V/dq, N/q),       // 0xd6: movq2dq
    unused!(),                      // 0xd7
    unused!(),                      // 0xd8
    unused!(),                      // 0xd9
    unused!(),                      // 0xda
    unused!(),                      // 0xdb
    unused!(),                      // 0xdc
    unused!(),                      // 0xdd
    unused!(),                      // 0xde
    unused!(),                      // 0xdf
    unused!(),                      // 0xe0
    unused!(),                      // 0xe1
    unused!(),                      // 0xe2
    unused!(),                      // 0xe3
    unused!(),                      // 0xe4
    unused!(),                      // 0xe5
    opcode!("cvtdq2pd"; V/x, W/pd),   // 0xe6: cvtdq2pd
    unused!(),                      // 0xe7
    unused!(),                      // 0xe8
    unused!(),                      // 0xe9
    unused!(),                      // 0xea
    unused!(),                      // 0xeb
    unused!(),                      // 0xec
    unused!(),                      // 0xed
    unused!(),                      // 0xee
    unused!(),                      // 0xef
    unused!(),                      // 0xf0
    unused!(),                      // 0xf1
    unused!(),                      // 0xf2
    unused!(),                      // 0xf3
    unused!(),                      // 0xf4
    unused!(),                      // 0xf5
    unused!(),                      // 0xf6
    unused!(),                      // 0xf7
    unused!(),                      // 0xf8
    unused!(),                      // 0xf9
    unused!(),                      // 0xfa
    unused!(),                      // 0xfb
    unused!(),                      // 0xfc
    unused!(),                      // 0xfd
    unused!(),                      // 0xfe
    unused!(),                      // 0xff
];

pub static TWOBYTE_F2_TABLE: [Opcode; 256] = [
    unused!(),                      // 0x00
    unused!(),                      // 0x01
    unused!(),                      // 0x02
    unused!(),                      // 0x03
    unused!(),                      // 0x04
    unused!(),                      // 0x05
    unused!(),                      // 0x06
    unused!(),                      // 0x07
    unused!(),                      // 0x08
    unused!(),                      // 0x09
    unused!(),                      // 0x0a
    unused!(),                      // 0x0b
    unused!(),                      // 0x0c
    unused!(),                      // 0x0d
    unused!(),                      // 0x0e
    unused!(),                      // 0x0f
    opcode!("vmovsd"; V/x, W/x, W/sd),       // 0x10: movsd
    opcode!("vmovsd"; W/sd, H/x, V/sd),         // 0x11: movsd
    opcode!("vmovddup"; V/x, W/x),       // 0x12: movddup
    unused!(),                      // 0x13
    unused!(),                      // 0x14
    unused!(),                      // 0x15
    unused!(),                      // 0x16
    unused!(),                      // 0x17
    unused!(),                      // 0x18
    unused!(),                      // 0x19
    unused!(),                      // 0x1a
    unused!(),                      // 0x1b
    unused!(),                      // 0x1c
    unused!(),                      // 0x1d
    unused!(),                      // 0x1e
    unused!(),                      // 0x1f
    unused!(),                      // 0x20
    unused!(),                      // 0x21
    unused!(),                      // 0x22
    unused!(),                      // 0x23
    unused!(),                      // 0x24
    unused!(),                      // 0x25
    unused!(),                      // 0x26
    unused!(),                      // 0x27
    unused!(),                      // 0x28
    unused!(),                      // 0x29
    opcode!("cvtsi2sd"; V/sd, H/sd, E/y), // 0x2a: cvtsi2sd
    unused!(),                      // 0x2b
    opcode!("cvttsd2si"; G/y, W/sd), // 0x2c: cvttsd2si
    opcode!("cvtsd2si"; G/y, W/sd),  // 0x2d: cvtsd2si
    unused!(),                      // 0x2e
    unused!(),                      // 0x2f
    unused!(),                      // 0x30
    unused!(),                      // 0x31
    unused!(),                      // 0x32
    unused!(),                      // 0x33
    unused!(),                      // 0x34
    unused!(),                      // 0x35
    unused!(),                      // 0x36
    unused!(),                      // 0x37
    opcode!("crc32"; G/y, E/b),     // 0x38: crc32
    unused!(),                      // 0x39
    unused!(),                      // 0x3a
    unused!(),                      // 0x3b
    unused!(),                      // 0x3c
    unused!(),                      // 0x3d
    unused!(),                      // 0x3e
    unused!(),                      // 0x3f
    unused!(),                      // 0x40
    unused!(),                      // 0x41
    unused!(),                      // 0x42
    unused!(),                      // 0x43
    unused!(),                      // 0x44
    unused!(),                      // 0x45
    unused!(),                      // 0x46
    unused!(),                      // 0x47
    unused!(),                      // 0x48
    unused!(),                      // 0x49
    unused!(),                      // 0x4a
    unused!(),                      // 0x4b
    unused!(),                      // 0x4c
    unused!(),                      // 0x4d
    unused!(),                      // 0x4e
    unused!(),                      // 0x4f
    unused!(),                      // 0x50
    opcode!("vsqrtsd"; V/sd, H/sd, W/sd),     // 0x51: sqrtsd
    unused!(),                      // 0x52
    unused!(),                      // 0x53
    unused!(),                      // 0x54
    unused!(),                      // 0x55
    unused!(),                      // 0x56
    unused!(),                      // 0x57
    opcode!("vaddsd"; V/sd, H/sd, W/sd),      // 0x58: addsd
    opcode!("vmulsd"; V/sd, H/sd, W/sd),      // 0x59: mulsd
    opcode!("vcvtsd2ss"; V/ss, H/x, W/sd),   // 0x5a: cvtsd2ss
    unused!(),                      // 0x5b
    opcode!("vsubsd"; V/sd, H/sd, W/sd),      // 0x5c: subsd
    opcode!("vminsd"; V/sd, H/sd, W/sd),    // 0x5d: minsd
    opcode!("vdivsd"; V/sd, H/sd, W/sd),    // 0x5e: divsd
    opcode!("vmaxsd"; V/sd, H/sd, W/sd),      // 0x5f: maxsd
    unused!(),                      // 0x60
    unused!(),                      // 0x61
    unused!(),                      // 0x62
    unused!(),                      // 0x63
    unused!(),                      // 0x64
    unused!(),                      // 0x65
    unused!(),                      // 0x66
    unused!(),                      // 0x67
    unused!(),                      // 0x68
    unused!(),                      // 0x69
    unused!(),                      // 0x6a
    unused!(),                      // 0x6b
    unused!(),                      // 0x6c
    unused!(),                      // 0x6d
    unused!(),                      // 0x6e
    unused!(),                      // 0x6f
    opcode!("pshuflw"; V/x, W/x, I/b),  // 0x70: pshuflw
    unused!(),                      // 0x71
    unused!(),                      // 0x72
    unused!(),                      // 0x73
    unused!(),                      // 0x74
    unused!(),                      // 0x75
    unused!(),                      // 0x76
    unused!(),                      // 0x77
    unused!(),                      // 0x78
    unused!(),                      // 0x79
    unused!(),                      // 0x7a
    unused!(),                      // 0x7b
    opcode!("haddps"; V/ps, H/ps, W/ps),     // 0x7c: haddps
    opcode!("hsubps"; V/ps, H/ps, W/ps),     // 0x7d: hsubps
    unused!(),                      // 0x7e
    unused!(),                      // 0x7f
    unused!(),                      // 0x80
    unused!(),                      // 0x81
    unused!(),                      // 0x82
    unused!(),                      // 0x83
    unused!(),                      // 0x84
    unused!(),                      // 0x85
    unused!(),                      // 0x86
    unused!(),                      // 0x87
    unused!(),                      // 0x88
    unused!(),                      // 0x89
    unused!(),                      // 0x8a
    unused!(),                      // 0x8b
    unused!(),                      // 0x8c
    unused!(),                      // 0x8d
    unused!(),                      // 0x8e
    unused!(),                      // 0x8f
    unused!(),                      // 0x90
    unused!(),                      // 0x91
    unused!(),                      // 0x92
    unused!(),                      // 0x93
    unused!(),                      // 0x94
    unused!(),                      // 0x95
    unused!(),                      // 0x96
    unused!(),                      // 0x97
    unused!(),                      // 0x98
    unused!(),                      // 0x99
    unused!(),                      // 0x9a
    unused!(),                      // 0x9b
    unused!(),                      // 0x9c
    unused!(),                      // 0x9d
    unused!(),                      // 0x9e
    unused!(),                      // 0x9f
    unused!(),                      // 0xa0
    unused!(),                      // 0xa1
    unused!(),                      // 0xa2
    unused!(),                      // 0xa3
    unused!(),                      // 0xa4
    unused!(),                      // 0xa5
    unused!(),                      // 0xa6
    unused!(),                      // 0xa7
    unused!(),                      // 0xa8
    unused!(),                      // 0xa9
    unused!(),                      // 0xaa
    unused!(),                      // 0xab
    unused!(),                      // 0xac
    unused!(),                      // 0xad
    unused!(),                      // 0xae
    unused!(),                      // 0xaf
    unused!(),                      // 0xb0
    unused!(),                      // 0xb1
    unused!(),                      // 0xb2
    unused!(),                      // 0xb3
    unused!(),                      // 0xb4
    unused!(),                      // 0xb5
    unused!(),                      // 0xb6
    unused!(),                      // 0xb7
    unused!(),                      // 0xb8
    unused!(),                      // 0xb9
    unused!(),                      // 0xba
    unused!(),                      // 0xbb
    unused!(),                      // 0xbc
    unused!(),                      // 0xbd
    unused!(),                      // 0xbe
    unused!(),                      // 0xbf
    unused!(),                      // 0xc0
    unused!(),                      // 0xc1
    opcode!("vcmpsd"; V/sd, H/sd, W/sd, I/b), // 0xc2: cmpsd
    unused!(),                      // 0xc3
    unused!(),                      // 0xc4
    unused!(),                      // 0xc5
    unused!(),                      // 0xc6
    unused!(),                      // 0xc7
    unused!(),                      // 0xc8
    unused!(),                      // 0xc9
    unused!(),                      // 0xca
    unused!(),                      // 0xcb
    unused!(),                      // 0xcc
    unused!(),                      // 0xcd
    unused!(),                      // 0xce
    unused!(),                      // 0xcf
    opcode!("vaddsubps"; V/ps, H/ps, W/ps),   // 0xd0: addsubps
    unused!(),                      // 0xd1
    unused!(),                      // 0xd2
    unused!(),                      // 0xd3
    unused!(),                      // 0xd4
    unused!(),                      // 0xd5
    opcode!("movdq2q"; P/q, U/q),       // 0xd6: movdq2q
    unused!(),                      // 0xd7
    unused!(),                      // 0xd8
    unused!(),                      // 0xd9
    unused!(),                      // 0xda
    unused!(),                      // 0xdb
    unused!(),                      // 0xdc
    unused!(),                      // 0xdd
    unused!(),                      // 0xde
    unused!(),                      // 0xdf
    unused!(),                      // 0xe0
    unused!(),                      // 0xe1
    unused!(),                      // 0xe2
    unused!(),                      // 0xe3
    unused!(),                      // 0xe4
    unused!(),                      // 0xe5
    opcode!("cvtpd2dq"; V/x, W/pd),   // 0xe6: cvtpd2dq
    unused!(),                      // 0xe7
    unused!(),                      // 0xe8
    unused!(),                      // 0xe9
    unused!(),                      // 0xea
    unused!(),                      // 0xeb
    unused!(),                      // 0xec
    unused!(),                      // 0xed
    unused!(),                      // 0xee
    unused!(),                      // 0xef
    opcode!("lddqu"; V/x, M/x),         // 0xf0: lddqu
    unused!(),                      // 0xf1
    unused!(),                      // 0xf2
    unused!(),                      // 0xf3
    unused!(),                      // 0xf4
    unused!(),                      // 0xf5
    unused!(),                      // 0xf6
    unused!(),                      // 0xf7
    unused!(),                      // 0xf8
    unused!(),                      // 0xf9
    unused!(),                      // 0xfa
    unused!(),                      // 0xfb
    unused!(),                      // 0xfc
    unused!(),                      // 0xfd
    unused!(),                      // 0xfe
    unused!(),                      // 0xff
];

pub static THREEBYTE_38_TABLE: [Opcode; 256] = [
    opcode!("pshufb"; P/q, Q/q),        // 0x00: pshufb
    opcode!("phaddw"; P/q, Q/q),        // 0x01: phaddw
    opcode!("phaddd"; P/q, Q/q),        // 0x02: phaddd
    opcode!("phaddsw"; P/q, Q/q),       // 0x03: phaddsw
    opcode!("pmaddubsw"; P/q, Q/q),     // 0x04: pmaddubsw
    opcode!("phsubw"; P/q, Q/q),        // 0x05: phsubw
    opcode!("phsubd"; P/q, Q/q),        // 0x06: phsubd
    opcode!("phsubsw"; P/q, Q/q),       // 0x07: phsubsw
    opcode!("psignb"; P/q, Q/q),        // 0x08: psignb
    opcode!("psignw"; P/q, Q/q),        // 0x09: psignw
    opcode!("psignd"; P/q, Q/q),        // 0x0a: psignd
    opcode!("pmulhrsw"; P/q, Q/q),      // 0x0b: pmulhrsw
    unused!(),                      // 0x0c
    unused!(),                      // 0x0d
    unused!(),                      // 0x0e
    unused!(),                      // 0x0f
    unused!(),                      // 0x10
    unused!(),                      // 0x11
    unused!(),                      // 0x12
    unused!(),                      // 0x13
    unused!(),                      // 0x14
    unused!(),                      // 0x15
    unused!(),                      // 0x16
    unused!(),                      // 0x17
    unused!(),                      // 0x18
    unused!(),                      // 0x19
    unused!(),                      // 0x1a
    unused!(),                      // 0x1b
    opcode!("pabsb"; P/q, Q/q),         // 0x1c: pabsb
    opcode!("pabsw"; P/q, Q/q),         // 0x1d: pabsw
    opcode!("pabsd"; P/q, Q/q),         // 0x1e: pabsd
    unused!(),                      // 0x1f
    unused!(),                      // 0x20
    unused!(),                      // 0x21
    unused!(),                      // 0x22
    unused!(),                      // 0x23
    unused!(),                      // 0x24
    unused!(),                      // 0x25
    unused!(),                      // 0x26
    unused!(),                      // 0x27
    unused!(),                      // 0x28
    unused!(),                      // 0x29
    unused!(),                      // 0x2a
    unused!(),                      // 0x2b
    unused!(),                      // 0x2c
    unused!(),                      // 0x2d
    unused!(),                      // 0x2e
    unused!(),                      // 0x2f
    unused!(),                      // 0x30
    unused!(),                      // 0x31
    unused!(),                      // 0x32
    unused!(),                      // 0x33
    unused!(),                      // 0x34
    unused!(),                      // 0x35
    unused!(),                      // 0x36
    unused!(),                      // 0x37
    unused!(),                      // 0x38
    unused!(),                      // 0x39
    unused!(),                      // 0x3a
    unused!(),                      // 0x3b
    unused!(),                      // 0x3c
    unused!(),                      // 0x3d
    unused!(),                      // 0x3e
    unused!(),                      // 0x3f
    unused!(),                      // 0x40
    unused!(),                      // 0x41
    unused!(),                      // 0x42
    unused!(),                      // 0x43
    unused!(),                      // 0x44
    unused!(),                      // 0x45
    unused!(),                      // 0x46
    unused!(),                      // 0x47
    unused!(),                      // 0x48
    unused!(),                      // 0x49
    unused!(),                      // 0x4a
    unused!(),                      // 0x4b
    unused!(),                      // 0x4c
    unused!(),                      // 0x4d
    unused!(),                      // 0x4e
    unused!(),                      // 0x4f
    unused!(),                      // 0x50
    unused!(),                      // 0x51
    unused!(),                      // 0x52
    unused!(),                      // 0x53
    unused!(),                      // 0x54
    unused!(),                      // 0x55
    unused!(),                      // 0x56
    unused!(),                      // 0x57
    unused!(),                      // 0x58
    unused!(),                      // 0x59
    unused!(),                      // 0x5a
    unused!(),                      // 0x5b
    unused!(),                      // 0x5c
    unused!(),                      // 0x5d
    unused!(),                      // 0x5e
    unused!(),                      // 0x5f
    unused!(),                      // 0x60
    unused!(),                      // 0x61
    unused!(),                      // 0x62
    unused!(),                      // 0x63
    unused!(),                      // 0x64
    unused!(),                      // 0x65
    unused!(),                      // 0x66
    unused!(),                      // 0x67
    unused!(),                      // 0x68
    unused!(),                      // 0x69
    unused!(),                      // 0x6a
    unused!(),                      // 0x6b
    unused!(),                      // 0x6c
    unused!(),                      // 0x6d
    unused!(),                      // 0x6e
    unused!(),                      // 0x6f
    unused!(),                      // 0x70
    unused!(),                      // 0x71
    unused!(),                      // 0x72
    unused!(),                      // 0x73
    unused!(),                      // 0x74
    unused!(),                      // 0x75
    unused!(),                      // 0x76
    unused!(),                      // 0x77
    unused!(),                      // 0x78
    unused!(),                      // 0x79
    unused!(),                      // 0x7a
    unused!(),                      // 0x7b
    unused!(),                      // 0x7c
    unused!(),                      // 0x7d
    unused!(),                      // 0x7e
    unused!(),                      // 0x7f
    unused!(),                      // 0x80
    unused!(),                      // 0x81
    unused!(),                      // 0x82
    unused!(),                      // 0x83
    unused!(),                      // 0x84
    unused!(),                      // 0x85
    unused!(),                      // 0x86
    unused!(),                      // 0x87
    unused!(),                      // 0x88
    unused!(),                      // 0x89
    unused!(),                      // 0x8a
    unused!(),                      // 0x8b
    unused!(),                      // 0x8c
    unused!(),                      // 0x8d
    unused!(),                      // 0x8e
    unused!(),                      // 0x8f
    unused!(),                      // 0x90
    unused!(),                      // 0x91
    unused!(),                      // 0x92
    unused!(),                      // 0x93
    unused!(),                      // 0x94
    unused!(),                      // 0x95
    unused!(),                      // 0x96
    unused!(),                      // 0x97
    unused!(),                      // 0x98
    unused!(),                      // 0x99
    unused!(),                      // 0x9a
    unused!(),                      // 0x9b
    unused!(),                      // 0x9c
    unused!(),                      // 0x9d
    unused!(),                      // 0x9e
    unused!(),                      // 0x9f
    unused!(),                      // 0xa0
    unused!(),                      // 0xa1
    unused!(),                      // 0xa2
    unused!(),                      // 0xa3
    unused!(),                      // 0xa4
    unused!(),                      // 0xa5
    unused!(),                      // 0xa6
    unused!(),                      // 0xa7
    unused!(),                      // 0xa8
    unused!(),                      // 0xa9
    unused!(),                      // 0xaa
    unused!(),                      // 0xab
    unused!(),                      // 0xac
    unused!(),                      // 0xad
    unused!(),                      // 0xae
    unused!(),                      // 0xaf
    unused!(),                      // 0xb0
    unused!(),                      // 0xb1
    unused!(),                      // 0xb2
    unused!(),                      // 0xb3
    unused!(),                      // 0xb4
    unused!(),                      // 0xb5
    unused!(),                      // 0xb6
    unused!(),                      // 0xb7
    unused!(),                      // 0xb8
    unused!(),                      // 0xb9
    unused!(),                      // 0xba
    unused!(),                      // 0xbb
    unused!(),                      // 0xbc
    unused!(),                      // 0xbd
    unused!(),                      // 0xbe
    unused!(),                      // 0xbf
    unused!(),                      // 0xc0
    unused!(),                      // 0xc1
    unused!(),                      // 0xc2
    unused!(),                      // 0xc3
    unused!(),                      // 0xc4
    unused!(),                      // 0xc5
    unused!(),                      // 0xc6
    unused!(),                      // 0xc7
    opcode!("sha1nexte"; V/dq, W/dq),                      // 0xc8
    opcode!("sha1msg1"; V/dq, W/dq),                      // 0xc9
    opcode!("sha1msg2"; V/dq, W/dq),                      // 0xca
    opcode!("sha256rnds2"; V/dq, W/dq),                      // 0xcb
    opcode!("sha256msg1"; V/dq, W/dq),                      // 0xcc
    opcode!("sha256msg2"; V/dq, W/dq),                      // 0xcd
    unused!(),                      // 0xce
    unused!(),                      // 0xcf
    unused!(),                      // 0xd0
    unused!(),                      // 0xd1
    unused!(),                      // 0xd2
    unused!(),                      // 0xd3
    unused!(),                      // 0xd4
    unused!(),                      // 0xd5
    unused!(),                      // 0xd6
    unused!(),                      // 0xd7
    unused!(),                      // 0xd8
    unused!(),                      // 0xd9
    unused!(),                      // 0xda
    unused!(),                      // 0xdb
    unused!(),                      // 0xdc
    unused!(),                      // 0xdd
    unused!(),                      // 0xde
    unused!(),                      // 0xdf
    unused!(),                      // 0xe0
    unused!(),                      // 0xe1
    unused!(),                      // 0xe2
    unused!(),                      // 0xe3
    unused!(),                      // 0xe4
    unused!(),                      // 0xe5
    unused!(),                      // 0xe6
    unused!(),                      // 0xe7
    unused!(),                      // 0xe8
    unused!(),                      // 0xe9
    unused!(),                      // 0xea
    unused!(),                      // 0xeb
    unused!(),                      // 0xec
    unused!(),                      // 0xed
    unused!(),                      // 0xee
    unused!(),                      // 0xef
    opcode!("movbe"; G/y, M/y),     // 0xf0: movbe
    opcode!("movbe"; M/y, G/y),     // 0xf1: movbe
    opcode!("andn"; G/y, B/y, E/y),                      // 0xf2
    unused!(),                      // 0xf3
    unused!(),                      // 0xf4
    opcode!("bzhi"; G/y, E/y, B/y),                      // 0xf5
    unused!(),                      // 0xf6
    opcode!("bextr"; G/y, E/y, B/y),                      // 0xf7
    unused!(),                      // 0xf8
    unused!(),                      // 0xf9
    unused!(),                      // 0xfa
    unused!(),                      // 0xfb
    unused!(),                      // 0xfc
    unused!(),                      // 0xfd
    unused!(),                      // 0xfe
    unused!(),                      // 0xff
];

pub static THREEBYTE_3866_TABLE: [Opcode; 256] = [
    opcode!("vpshufb"; V/x, H/x, W/x),     // 0x00: pshufb
    opcode!("vphaddw"; V/x, H/x, W/x),     // 0x01: phaddw
    opcode!("vphaddd"; V/x, H/x, W/x),     // 0x02: phaddd
    opcode!("vphaddsw"; V/x, H/x, W/x),    // 0x03: phaddsw
    opcode!("pmaddubsw"; V/x, H/x, W/x),  // 0x04: pmaddubsw
    opcode!("vphsubw"; V/x, H/x, W/x),     // 0x05: phsubw
    opcode!("vphsubd"; V/x, H/x, W/x),     // 0x06: phsubd
    opcode!("vphsubsw"; V/x, H/x, W/x),    // 0x07: phsubsw
    opcode!("vpsignb"; V/x, H/x, W/x),     // 0x08: psignb
    opcode!("vpsignw"; V/x, H/x, W/x),     // 0x09: psignw
    opcode!("vpsignd"; V/x, H/x, W/x),     // 0x0a: psignd
    opcode!("vpmulhrsw"; V/x, H/x, W/x),   // 0x0b: pmulhrsw
    opcode!("vpermilp"; V/x, H/x, W/x),                      // 0x0c
    opcode!("vpermilpd"; V/x, H/x, W/x),                      // 0x0d
    opcode!("vtestps"; V/x, W/x),                      // 0x0e
    opcode!("vtestpd"; V/x, W/x),                      // 0x0f
    opcode!("pblendvb"; V/dq, W/dq),      // 0x10: pblendvb
    unused!(),                      // 0x11
    unused!(),                      // 0x12
    opcode!("vcvtph2ps"; V/x, W/x, I/b),                      // 0x13
    opcode!("blendvps"; V/dq, W/dq),      // 0x14: blendvps
    opcode!("blendvpd"; V/dq, W/dq),      // 0x15: blendvpd
    unused!(),                      // 0x16
    opcode!("ptest"; V/x, W/x),         // 0x17: ptest
    opcode!("vbroadcastss"; V/x, W/d),                      // 0x18
    opcode!("vbroadcastsd"; W/q),                      // 0x19
    opcode!("vbroadcastf128"; M/dq),                      // 0x1a
    unused!(),                      // 0x1b
    opcode!("vpabsb"; V/x, W/x),         // 0x1c: pabsb
    opcode!("vpabsw"; V/x, W/x),         // 0x1d: pabsw
    opcode!("vpabsd"; V/x, W/x),         // 0x1e: pabsd
    unused!(),                      // 0x1f
    opcode!("pmovsxbw"; V/x, M/q),    // 0x20: pmovsxbw
    opcode!("pmovsxbd"; V/x, M/d),    // 0x21: pmovsxbd
    opcode!("pmovsxbq"; V/x, M/w),    // 0x22: pmovsxbq
    opcode!("pmovsxwd"; V/x, M/q),    // 0x23: pmovsxwd
    opcode!("pmovsxwq"; V/x, M/d),    // 0x24: pmovsxwq
    opcode!("pmovsxdq"; V/x, M/q),    // 0x25: pmovsxdq
    unused!(),                      // 0x26
    unused!(),                      // 0x27
    opcode!("vpmuldq"; V/x, H/x, W/x),     // 0x28: pmuldq
    opcode!("vpcmpeqq"; V/x, H/x, W/x),    // 0x29: pcmpeqq
    opcode!("vmovntdqa"; V/x, M/dq),      // 0x2a: movntdqa
    opcode!("vpackusdw"; V/x, H/x, W/x),   // 0x2b: packusdw
    opcode!("vmaskmovps"; V/x, H/x, M/x),                      // 0x2c
    opcode!("vmaskmovpd"; V/x, H/x, M/x),                      // 0x2d
    opcode!("vmaskmovps"; M/x, H/x, V/x),                      // 0x2e
    opcode!("vmaskmovpd"; M/x, H/x, V/x),                      // 0x2f
    opcode!("pmovzxbw"; V/x, M/q),    // 0x30: pmovzxbw
    opcode!("pmovzxbd"; V/x, M/d),    // 0x31: pmovzxbd
    opcode!("pmovzxbq"; V/x, M/w),    // 0x32: pmovzxbq
    opcode!("pmovzxwd"; V/x, M/q),    // 0x33: pmovzxwd
    opcode!("pmovzxwq"; V/x, M/d),    // 0x34: pmovzxwq
    opcode!("pmovzxdq"; V/x, M/q),    // 0x35: pmovzxdq
    opcode!("vpermd"; V/qq, H/qq, W/qq),                      // 0x36
    opcode!("pcmpgtq"; V/x, H/x, W/x),    // 0x37: pcmpgtq
    opcode!("vpminsb"; V/x, H/x, W/x),     // 0x38: pminsb
    opcode!("vpminsd"; V/x, H/x, W/x),     // 0x39: pminsd
    opcode!("vpminuw"; V/x, H/x, W/x),     // 0x3a: pminuw
    opcode!("vpminud"; V/x, H/x, W/x),     // 0x3b: pminud
    opcode!("vpmaxsb"; V/x, H/x, W/x),     // 0x3c: pmaxsb
    opcode!("vpmaxsd"; V/x, H/x, W/x),     // 0x3d: pmaxsd
    opcode!("vpmaxuw"; V/x, H/x, W/x),     // 0x3e: pmaxuw
    opcode!("vpmaxud"; V/x, H/x, W/x),     // 0x3f: pmaxud
    opcode!("pmulld"; V/x, H/x, W/x),     // 0x40: pmulld
    opcode!("phminposuw"; V/dq, W/dq),    // 0x41: phminposuw
    unused!(),                      // 0x42
    unused!(),                      // 0x43
    unused!(),                      // 0x44
    unused!(),                      // 0x45
    unused!(),                      // 0x46
    unused!(),                      // 0x47
    unused!(),                      // 0x48
    unused!(),                      // 0x49
    unused!(),                      // 0x4a
    unused!(),                      // 0x4b
    unused!(),                      // 0x4c
    unused!(),                      // 0x4d
    unused!(),                      // 0x4e
    unused!(),                      // 0x4f
    unused!(),                      // 0x50
    unused!(),                      // 0x51
    unused!(),                      // 0x52
    unused!(),                      // 0x53
    unused!(),                      // 0x54
    unused!(),                      // 0x55
    unused!(),                      // 0x56
    unused!(),                      // 0x57
    opcode!("vpbroadcastd"; V/x, W/x),                      // 0x58
    opcode!("vpbroadcastq"; V/x, W/x),                      // 0x59
    opcode!("vbroadcasti128"; V/dq, W/dq),                      // 0x5a
    unused!(),                      // 0x5b
    unused!(),                      // 0x5c
    unused!(),                      // 0x5d
    unused!(),                      // 0x5e
    unused!(),                      // 0x5f
    unused!(),                      // 0x60
    unused!(),                      // 0x61
    unused!(),                      // 0x62
    unused!(),                      // 0x63
    unused!(),                      // 0x64
    unused!(),                      // 0x65
    unused!(),                      // 0x66
    unused!(),                      // 0x67
    unused!(),                      // 0x68
    unused!(),                      // 0x69
    unused!(),                      // 0x6a
    unused!(),                      // 0x6b
    unused!(),                      // 0x6c
    unused!(),                      // 0x6d
    unused!(),                      // 0x6e
    unused!(),                      // 0x6f
    unused!(),                      // 0x70
    unused!(),                      // 0x71
    unused!(),                      // 0x72
    unused!(),                      // 0x73
    unused!(),                      // 0x74
    unused!(),                      // 0x75
    unused!(),                      // 0x76
    unused!(),                      // 0x77
    opcode!("vpbroadcastb"; V/x, W/x),                      // 0x78
    opcode!("vpboradcastw"; V/x, W/x),                      // 0x79
    unused!(),                      // 0x7a
    unused!(),                      // 0x7b
    unused!(),                      // 0x7c
    unused!(),                      // 0x7d
    unused!(),                      // 0x7e
    unused!(),                      // 0x7f
    opcode!("invept"; G/y, M/dq),    // 0x80: invept
    opcode!("invvpid"; G/y, M/dq),   // 0x81: invvpid
    opcode!("invpcid"; G/y, M/dq),    // 0x82
    unused!(),                      // 0x83
    unused!(),                      // 0x84
    unused!(),                      // 0x85
    unused!(),                      // 0x86
    unused!(),                      // 0x87
    unused!(),                      // 0x88
    unused!(),                      // 0x89
    unused!(),                      // 0x8a
    unused!(),                      // 0x8b
    opcode!("vpmaskmovd"; V/x, H/x, M/x),                      // 0x8c
    unused!(),                      // 0x8d
    opcode!("vpmaskmovd"; M/x, V/x, H/x),                      // 0x8e
    unused!(),                      // 0x8f
    opcode!("vgatherdd"; V/x, H/x, W/x),                      // 0x90
    opcode!("vgatherqd"; V/x, H/x, W/x),                      // 0x91
    opcode!("vgatherdps"; V/x, H/x, W/x),                      // 0x92
    opcode!("vgatherqps"; V/x, H/x, W/x),                      // 0x93
    unused!(),                      // 0x94
    unused!(),                      // 0x95
    opcode!("vfmaddsub132ps"; V/x, H/x, W/x),                      // 0x96
    opcode!("vfmsubadd132ps"; V/x, H/x, W/x),                      // 0x97
    opcode!("vfmadd132ps"; V/x, H/x, W/x),                      // 0x98
    opcode!("vfmadd132ss"; V/x, H/x, W/x),                      // 0x99
    opcode!("vfmsub132ps"; V/x, H/x, W/x),                      // 0x9a
    opcode!("vfmsub132ss"; V/x, H/x, W/x),                      // 0x9b
    opcode!("vfmnadd132ps"; V/x, H/x, W/x),                      // 0x9c
    opcode!("vfmnadd132ps"; V/x, H/x, W/x),                      // 0x9d
    opcode!("vfmnsub132ps"; V/x, H/x, W/x),                      // 0x9e
    opcode!("vfmnsub132ps"; V/x, H/x, W/x),                      // 0x9f
    unused!(),                      // 0xa0
    unused!(),                      // 0xa1
    unused!(),                      // 0xa2
    unused!(),                      // 0xa3
    unused!(),                      // 0xa4
    unused!(),                      // 0xa5
    opcode!("vfmaddsub232ps"; V/x, H/x, W/x),                      // 0xa6
    opcode!("vfmsubadd232ps"; V/x, H/x, W/x),                      // 0xa7
    opcode!("vfmadd213ps"; V/x, H/x, W/x),                      // 0xa8
    opcode!("vfmadd213ss"; V/x, H/x, W/x),                      // 0xa9
    opcode!("vfmsub213ps"; V/x, H/x, W/x),                      // 0xaa
    opcode!("vfmsub213ss"; V/x, H/x, W/x),                      // 0xab
    opcode!("vfnmadd213ps"; V/x, H/x, W/x),                      // 0xac
    opcode!("vfnmadd213ss"; V/x, H/x, W/x),                      // 0xad
    opcode!("vfnmsub213ps"; V/x, H/x, W/x),                      // 0xae
    opcode!("vfnmsub213ss"; V/x, H/x, W/x),                      // 0xaf
    unused!(),                      // 0xb0
    unused!(),                      // 0xb1
    unused!(),                      // 0xb2
    unused!(),                      // 0xb3
    unused!(),                      // 0xb4
    unused!(),                      // 0xb5
    opcode!("vfmaddsub231ps"; V/x, H/x, W/x),                      // 0xa6
    opcode!("vfmsubadd231ps"; V/x, H/x, W/x),                      // 0xa7
    opcode!("vfmadd231ps"; V/x, H/x, W/x),                      // 0xb8
    opcode!("vfmadd231ss"; V/x, H/x, W/x),                      // 0xb9
    opcode!("vfmsub231ps"; V/x, H/x, W/x),                      // 0xba
    opcode!("vfmsub231ss"; V/x, H/x, W/x),                      // 0xbb
    opcode!("vfnmadd231ps"; V/x, H/x, W/x),                      // 0xbc
    opcode!("vfnmadd231ss"; V/x, H/x, W/x),                      // 0xbd
    opcode!("vfnmsub231ps"; V/x, H/x, W/x),                      // 0xbe
    opcode!("vfnmsub231ss"; V/x, H/x, W/x),                      // 0xbf
    unused!(),                      // 0xc0
    unused!(),                      // 0xc1
    unused!(),                      // 0xc2
    unused!(),                      // 0xc3
    unused!(),                      // 0xc4
    unused!(),                      // 0xc5
    unused!(),                      // 0xc6
    unused!(),                      // 0xc7
    unused!(),                      // 0xc8
    unused!(),                      // 0xc9
    unused!(),                      // 0xca
    unused!(),                      // 0xcb
    unused!(),                      // 0xcc
    unused!(),                      // 0xcd
    unused!(),                      // 0xce
    unused!(),                      // 0xcf
    unused!(),                      // 0xd0
    unused!(),                      // 0xd1
    unused!(),                      // 0xd2
    unused!(),                      // 0xd3
    unused!(),                      // 0xd4
    unused!(),                      // 0xd5
    unused!(),                      // 0xd6
    unused!(),                      // 0xd7
    unused!(),                      // 0xd8
    unused!(),                      // 0xd9
    unused!(),                      // 0xda
    opcode!("aesimc"; V/dq, W/dq),        // 0xdb: aesimc
    opcode!("aesenc"; V/dq, W/dq),        // 0xdc: aesenc
    opcode!("aesenclast"; V/dq, H/dq, W/dq), // 0xdd: aesenclast
    opcode!("aesdec"; V/dq, H/dq, W/dq),     // 0xde: aesdec
    opcode!("aesdeclast"; V/dq, W/dq),    // 0xdf: aesdeclast
    unused!(),                      // 0xe0
    unused!(),                      // 0xe1
    unused!(),                      // 0xe2
    unused!(),                      // 0xe3
    unused!(),                      // 0xe4
    unused!(),                      // 0xe5
    unused!(),                      // 0xe6
    unused!(),                      // 0xe7
    unused!(),                      // 0xe8
    unused!(),                      // 0xe9
    unused!(),                      // 0xea
    unused!(),                      // 0xeb
    unused!(),                      // 0xec
    unused!(),                      // 0xed
    unused!(),                      // 0xee
    unused!(),                      // 0xef
    opcode!("movbe"; G/w, M/w),                      // 0xf0
    opcode!("movbe"; G/w, M/w),                      // 0xf1
    unused!(),                      // 0xf2
    opcode!(group 17; ),                      // 0xf3
    unused!(),                      // 0xf4
    unused!(),                      // 0xf5
    opcode!("adcx"; G/y, E/y),                      // 0xf6
    opcode!("shlx"; G/y, E/y, B/y),                      // 0xf7
    unused!(),                      // 0xf8
    unused!(),                      // 0xf9
    unused!(),                      // 0xfa
    unused!(),                      // 0xfb
    unused!(),                      // 0xfc
    unused!(),                      // 0xfd
    unused!(),                      // 0xfe
    unused!(),                      // 0xff
];

pub static THREEBYTE_38F2_TABLE: [Opcode; 256] = [
    unused!(),                      // 0x00
    unused!(),                      // 0x01
    unused!(),                      // 0x02
    unused!(),                      // 0x03
    unused!(),                      // 0x04
    unused!(),                      // 0x05
    unused!(),                      // 0x06
    unused!(),                      // 0x07
    unused!(),                      // 0x08
    unused!(),                      // 0x09
    unused!(),                      // 0x0a
    unused!(),                      // 0x0b
    unused!(),                      // 0x0c
    unused!(),                      // 0x0d
    unused!(),                      // 0x0e
    unused!(),                      // 0x0f
    unused!(),                      // 0x10
    unused!(),                      // 0x11
    unused!(),                      // 0x12
    unused!(),                      // 0x13
    unused!(),                      // 0x14
    unused!(),                      // 0x15
    unused!(),                      // 0x16
    unused!(),                      // 0x17
    unused!(),                      // 0x18
    unused!(),                      // 0x19
    unused!(),                      // 0x1a
    unused!(),                      // 0x1b
    unused!(),                      // 0x1c
    unused!(),                      // 0x1d
    unused!(),                      // 0x1e
    unused!(),                      // 0x1f
    unused!(),                      // 0x20
    unused!(),                      // 0x21
    unused!(),                      // 0x22
    unused!(),                      // 0x23
    unused!(),                      // 0x24
    unused!(),                      // 0x25
    unused!(),                      // 0x26
    unused!(),                      // 0x27
    unused!(),                      // 0x28
    unused!(),                      // 0x29
    unused!(),                      // 0x2a
    unused!(),                      // 0x2b
    unused!(),                      // 0x2c
    unused!(),                      // 0x2d
    unused!(),                      // 0x2e
    unused!(),                      // 0x2f
    unused!(),                      // 0x30
    unused!(),                      // 0x31
    unused!(),                      // 0x32
    unused!(),                      // 0x33
    unused!(),                      // 0x34
    unused!(),                      // 0x35
    unused!(),                      // 0x36
    unused!(),                      // 0x37
    unused!(),                      // 0x38
    unused!(),                      // 0x39
    unused!(),                      // 0x3a
    unused!(),                      // 0x3b
    unused!(),                      // 0x3c
    unused!(),                      // 0x3d
    unused!(),                      // 0x3e
    unused!(),                      // 0x3f
    unused!(),                      // 0x40
    unused!(),                      // 0x41
    unused!(),                      // 0x42
    unused!(),                      // 0x43
    unused!(),                      // 0x44
    unused!(),                      // 0x45
    unused!(),                      // 0x46
    unused!(),                      // 0x47
    unused!(),                      // 0x48
    unused!(),                      // 0x49
    unused!(),                      // 0x4a
    unused!(),                      // 0x4b
    unused!(),                      // 0x4c
    unused!(),                      // 0x4d
    unused!(),                      // 0x4e
    unused!(),                      // 0x4f
    unused!(),                      // 0x50
    unused!(),                      // 0x51
    unused!(),                      // 0x52
    unused!(),                      // 0x53
    unused!(),                      // 0x54
    unused!(),                      // 0x55
    unused!(),                      // 0x56
    unused!(),                      // 0x57
    unused!(),                      // 0x58
    unused!(),                      // 0x59
    unused!(),                      // 0x5a
    unused!(),                      // 0x5b
    unused!(),                      // 0x5c
    unused!(),                      // 0x5d
    unused!(),                      // 0x5e
    unused!(),                      // 0x5f
    unused!(),                      // 0x60
    unused!(),                      // 0x61
    unused!(),                      // 0x62
    unused!(),                      // 0x63
    unused!(),                      // 0x64
    unused!(),                      // 0x65
    unused!(),                      // 0x66
    unused!(),                      // 0x67
    unused!(),                      // 0x68
    unused!(),                      // 0x69
    unused!(),                      // 0x6a
    unused!(),                      // 0x6b
    unused!(),                      // 0x6c
    unused!(),                      // 0x6d
    unused!(),                      // 0x6e
    unused!(),                      // 0x6f
    unused!(),                      // 0x70
    unused!(),                      // 0x71
    unused!(),                      // 0x72
    unused!(),                      // 0x73
    unused!(),                      // 0x74
    unused!(),                      // 0x75
    unused!(),                      // 0x76
    unused!(),                      // 0x77
    unused!(),                      // 0x78
    unused!(),                      // 0x79
    unused!(),                      // 0x7a
    unused!(),                      // 0x7b
    unused!(),                      // 0x7c
    unused!(),                      // 0x7d
    unused!(),                      // 0x7e
    unused!(),                      // 0x7f
    unused!(),                      // 0x80
    unused!(),                      // 0x81
    unused!(),                      // 0x82
    unused!(),                      // 0x83
    unused!(),                      // 0x84
    unused!(),                      // 0x85
    unused!(),                      // 0x86
    unused!(),                      // 0x87
    unused!(),                      // 0x88
    unused!(),                      // 0x89
    unused!(),                      // 0x8a
    unused!(),                      // 0x8b
    unused!(),                      // 0x8c
    unused!(),                      // 0x8d
    unused!(),                      // 0x8e
    unused!(),                      // 0x8f
    unused!(),                      // 0x90
    unused!(),                      // 0x91
    unused!(),                      // 0x92
    unused!(),                      // 0x93
    unused!(),                      // 0x94
    unused!(),                      // 0x95
    unused!(),                      // 0x96
    unused!(),                      // 0x97
    unused!(),                      // 0x98
    unused!(),                      // 0x99
    unused!(),                      // 0x9a
    unused!(),                      // 0x9b
    unused!(),                      // 0x9c
    unused!(),                      // 0x9d
    unused!(),                      // 0x9e
    unused!(),                      // 0x9f
    unused!(),                      // 0xa0
    unused!(),                      // 0xa1
    unused!(),                      // 0xa2
    unused!(),                      // 0xa3
    unused!(),                      // 0xa4
    unused!(),                      // 0xa5
    unused!(),                      // 0xa6
    unused!(),                      // 0xa7
    unused!(),                      // 0xa8
    unused!(),                      // 0xa9
    unused!(),                      // 0xaa
    unused!(),                      // 0xab
    unused!(),                      // 0xac
    unused!(),                      // 0xad
    unused!(),                      // 0xae
    unused!(),                      // 0xaf
    unused!(),                      // 0xb0
    unused!(),                      // 0xb1
    unused!(),                      // 0xb2
    unused!(),                      // 0xb3
    unused!(),                      // 0xb4
    unused!(),                      // 0xb5
    unused!(),                      // 0xb6
    unused!(),                      // 0xb7
    unused!(),                      // 0xb8
    unused!(),                      // 0xb9
    unused!(),                      // 0xba
    unused!(),                      // 0xbb
    unused!(),                      // 0xbc
    unused!(),                      // 0xbd
    unused!(),                      // 0xbe
    unused!(),                      // 0xbf
    unused!(),                      // 0xc0
    unused!(),                      // 0xc1
    unused!(),                      // 0xc2
    unused!(),                      // 0xc3
    unused!(),                      // 0xc4
    unused!(),                      // 0xc5
    unused!(),                      // 0xc6
    unused!(),                      // 0xc7
    unused!(),                      // 0xc8
    unused!(),                      // 0xc9
    unused!(),                      // 0xca
    unused!(),                      // 0xcb
    unused!(),                      // 0xcc
    unused!(),                      // 0xcd
    unused!(),                      // 0xce
    unused!(),                      // 0xcf
    unused!(),                      // 0xd0
    unused!(),                      // 0xd1
    unused!(),                      // 0xd2
    unused!(),                      // 0xd3
    unused!(),                      // 0xd4
    unused!(),                      // 0xd5
    unused!(),                      // 0xd6
    unused!(),                      // 0xd7
    unused!(),                      // 0xd8
    unused!(),                      // 0xd9
    unused!(),                      // 0xda
    unused!(),                      // 0xdb
    unused!(),                      // 0xdc
    unused!(),                      // 0xdd
    unused!(),                      // 0xde
    unused!(),                      // 0xdf
    unused!(),                      // 0xe0
    unused!(),                      // 0xe1
    unused!(),                      // 0xe2
    unused!(),                      // 0xe3
    unused!(),                      // 0xe4
    unused!(),                      // 0xe5
    unused!(),                      // 0xe6
    unused!(),                      // 0xe7
    unused!(),                      // 0xe8
    unused!(),                      // 0xe9
    unused!(),                      // 0xea
    unused!(),                      // 0xeb
    unused!(),                      // 0xec
    unused!(),                      // 0xed
    unused!(),                      // 0xee
    unused!(),                      // 0xef
    opcode!("crc32"; G/y, E/b),     // 0xf0: crc32
    opcode!("crc32"; G/y, E/v),     // 0xf1: crc32
    unused!(),                      // 0xf2
    unused!(),                      // 0xf3
    unused!(),                      // 0xf4
    unused!(),                      // 0xf5
    unused!(),                      // 0xf6
    unused!(),                      // 0xf7
    unused!(),                      // 0xf8
    unused!(),                      // 0xf9
    unused!(),                      // 0xfa
    unused!(),                      // 0xfb
    unused!(),                      // 0xfc
    unused!(),                      // 0xfd
    unused!(),                      // 0xfe
    unused!(),                      // 0xff
];

pub static THREEBYTE_38F3_TABLE: [Opcode; 256] = [
    unused!(),                      // 0x00
    unused!(),                      // 0x01
    unused!(),                      // 0x02
    unused!(),                      // 0x03
    unused!(),                      // 0x04
    unused!(),                      // 0x05
    unused!(),                      // 0x06
    unused!(),                      // 0x07
    unused!(),                      // 0x08
    unused!(),                      // 0x09
    unused!(),                      // 0x0a
    unused!(),                      // 0x0b
    unused!(),                      // 0x0c
    unused!(),                      // 0x0d
    unused!(),                      // 0x0e
    unused!(),                      // 0x0f
    unused!(),                      // 0x10
    unused!(),                      // 0x11
    unused!(),                      // 0x12
    unused!(),                      // 0x13
    unused!(),                      // 0x14
    unused!(),                      // 0x15
    unused!(),                      // 0x16
    unused!(),                      // 0x17
    unused!(),                      // 0x18
    unused!(),                      // 0x19
    unused!(),                      // 0x1a
    unused!(),                      // 0x1b
    unused!(),                      // 0x1c
    unused!(),                      // 0x1d
    unused!(),                      // 0x1e
    unused!(),                      // 0x1f
    unused!(),                      // 0x20
    unused!(),                      // 0x21
    unused!(),                      // 0x22
    unused!(),                      // 0x23
    unused!(),                      // 0x24
    unused!(),                      // 0x25
    unused!(),                      // 0x26
    unused!(),                      // 0x27
    unused!(),                      // 0x28
    unused!(),                      // 0x29
    unused!(),                      // 0x2a
    unused!(),                      // 0x2b
    unused!(),                      // 0x2c
    unused!(),                      // 0x2d
    unused!(),                      // 0x2e
    unused!(),                      // 0x2f
    unused!(),                      // 0x30
    unused!(),                      // 0x31
    unused!(),                      // 0x32
    unused!(),                      // 0x33
    unused!(),                      // 0x34
    unused!(),                      // 0x35
    unused!(),                      // 0x36
    unused!(),                      // 0x37
    unused!(),                      // 0x38
    unused!(),                      // 0x39
    unused!(),                      // 0x3a
    unused!(),                      // 0x3b
    unused!(),                      // 0x3c
    unused!(),                      // 0x3d
    unused!(),                      // 0x3e
    unused!(),                      // 0x3f
    unused!(),                      // 0x40
    unused!(),                      // 0x41
    unused!(),                      // 0x42
    unused!(),                      // 0x43
    unused!(),                      // 0x44
    unused!(),                      // 0x45
    unused!(),                      // 0x46
    unused!(),                      // 0x47
    unused!(),                      // 0x48
    unused!(),                      // 0x49
    unused!(),                      // 0x4a
    unused!(),                      // 0x4b
    unused!(),                      // 0x4c
    unused!(),                      // 0x4d
    unused!(),                      // 0x4e
    unused!(),                      // 0x4f
    unused!(),                      // 0x50
    unused!(),                      // 0x51
    unused!(),                      // 0x52
    unused!(),                      // 0x53
    unused!(),                      // 0x54
    unused!(),                      // 0x55
    unused!(),                      // 0x56
    unused!(),                      // 0x57
    unused!(),                      // 0x58
    unused!(),                      // 0x59
    unused!(),                      // 0x5a
    unused!(),                      // 0x5b
    unused!(),                      // 0x5c
    unused!(),                      // 0x5d
    unused!(),                      // 0x5e
    unused!(),                      // 0x5f
    unused!(),                      // 0x60
    unused!(),                      // 0x61
    unused!(),                      // 0x62
    unused!(),                      // 0x63
    unused!(),                      // 0x64
    unused!(),                      // 0x65
    unused!(),                      // 0x66
    unused!(),                      // 0x67
    unused!(),                      // 0x68
    unused!(),                      // 0x69
    unused!(),                      // 0x6a
    unused!(),                      // 0x6b
    unused!(),                      // 0x6c
    unused!(),                      // 0x6d
    unused!(),                      // 0x6e
    unused!(),                      // 0x6f
    unused!(),                      // 0x70
    unused!(),                      // 0x71
    unused!(),                      // 0x72
    unused!(),                      // 0x73
    unused!(),                      // 0x74
    unused!(),                      // 0x75
    unused!(),                      // 0x76
    unused!(),                      // 0x77
    unused!(),                      // 0x78
    unused!(),                      // 0x79
    unused!(),                      // 0x7a
    unused!(),                      // 0x7b
    unused!(),                      // 0x7c
    unused!(),                      // 0x7d
    unused!(),                      // 0x7e
    unused!(),                      // 0x7f
    unused!(),                      // 0x80
    unused!(),                      // 0x81
    unused!(),                      // 0x82
    unused!(),                      // 0x83
    unused!(),                      // 0x84
    unused!(),                      // 0x85
    unused!(),                      // 0x86
    unused!(),                      // 0x87
    unused!(),                      // 0x88
    unused!(),                      // 0x89
    unused!(),                      // 0x8a
    unused!(),                      // 0x8b
    unused!(),                      // 0x8c
    unused!(),                      // 0x8d
    unused!(),                      // 0x8e
    unused!(),                      // 0x8f
    unused!(),                      // 0x90
    unused!(),                      // 0x91
    unused!(),                      // 0x92
    unused!(),                      // 0x93
    unused!(),                      // 0x94
    unused!(),                      // 0x95
    unused!(),                      // 0x96
    unused!(),                      // 0x97
    unused!(),                      // 0x98
    unused!(),                      // 0x99
    unused!(),                      // 0x9a
    unused!(),                      // 0x9b
    unused!(),                      // 0x9c
    unused!(),                      // 0x9d
    unused!(),                      // 0x9e
    unused!(),                      // 0x9f
    unused!(),                      // 0xa0
    unused!(),                      // 0xa1
    unused!(),                      // 0xa2
    unused!(),                      // 0xa3
    unused!(),                      // 0xa4
    unused!(),                      // 0xa5
    unused!(),                      // 0xa6
    unused!(),                      // 0xa7
    unused!(),                      // 0xa8
    unused!(),                      // 0xa9
    unused!(),                      // 0xaa
    unused!(),                      // 0xab
    unused!(),                      // 0xac
    unused!(),                      // 0xad
    unused!(),                      // 0xae
    unused!(),                      // 0xaf
    unused!(),                      // 0xb0
    unused!(),                      // 0xb1
    unused!(),                      // 0xb2
    unused!(),                      // 0xb3
    unused!(),                      // 0xb4
    unused!(),                      // 0xb5
    unused!(),                      // 0xb6
    unused!(),                      // 0xb7
    unused!(),                      // 0xb8
    unused!(),                      // 0xb9
    unused!(),                      // 0xba
    unused!(),                      // 0xbb
    unused!(),                      // 0xbc
    unused!(),                      // 0xbd
    unused!(),                      // 0xbe
    unused!(),                      // 0xbf
    unused!(),                      // 0xc0
    unused!(),                      // 0xc1
    unused!(),                      // 0xc2
    unused!(),                      // 0xc3
    unused!(),                      // 0xc4
    unused!(),                      // 0xc5
    unused!(),                      // 0xc6
    unused!(),                      // 0xc7
    unused!(),                      // 0xc8
    unused!(),                      // 0xc9
    unused!(),                      // 0xca
    unused!(),                      // 0xcb
    unused!(),                      // 0xcc
    unused!(),                      // 0xcd
    unused!(),                      // 0xce
    unused!(),                      // 0xcf
    unused!(),                      // 0xd0
    unused!(),                      // 0xd1
    unused!(),                      // 0xd2
    unused!(),                      // 0xd3
    unused!(),                      // 0xd4
    unused!(),                      // 0xd5
    unused!(),                      // 0xd6
    unused!(),                      // 0xd7
    unused!(),                      // 0xd8
    unused!(),                      // 0xd9
    unused!(),                      // 0xda
    unused!(),                      // 0xdb
    unused!(),                      // 0xdc
    unused!(),                      // 0xdd
    unused!(),                      // 0xde
    unused!(),                      // 0xdf
    unused!(),                      // 0xe0
    unused!(),                      // 0xe1
    unused!(),                      // 0xe2
    unused!(),                      // 0xe3
    unused!(),                      // 0xe4
    unused!(),                      // 0xe5
    unused!(),                      // 0xe6
    unused!(),                      // 0xe7
    unused!(),                      // 0xe8
    unused!(),                      // 0xe9
    unused!(),                      // 0xea
    unused!(),                      // 0xeb
    unused!(),                      // 0xec
    unused!(),                      // 0xed
    unused!(),                      // 0xee
    unused!(),                      // 0xef
    unused!(),                      // 0xf0
    unused!(),                      // 0xf1
    unused!(),                      // 0xf2
    unused!(),                      // 0xf3
    unused!(),                      // 0xf4
    unused!(),                      // 0xf5
    unused!(),                      // 0xf6
    unused!(),                      // 0xf7
    unused!(),                      // 0xf8
    unused!(),                      // 0xf9
    unused!(),                      // 0xfa
    unused!(),                      // 0xfb
    unused!(),                      // 0xfc
    unused!(),                      // 0xfd
    unused!(),                      // 0xfe
    unused!(),                      // 0xff
];

pub static THREEBYTE_3A_TABLE: [Opcode; 256] = [
    unused!(),                      // 0x00
    unused!(),                      // 0x01
    unused!(),                      // 0x02
    unused!(),                      // 0x03
    unused!(),                      // 0x04
    unused!(),                      // 0x05
    unused!(),                      // 0x06
    unused!(),                      // 0x07
    unused!(),                      // 0x08
    unused!(),                      // 0x09
    unused!(),                      // 0x0a
    unused!(),                      // 0x0b
    unused!(),                      // 0x0c
    unused!(),                      // 0x0d
    unused!(),                      // 0x0e
    opcode!("palignr"; P/q, Q/q, I/b),  // 0x0f: palignr
    unused!(),                      // 0x10
    unused!(),                      // 0x11
    unused!(),                      // 0x12
    unused!(),                      // 0x13
    unused!(),                      // 0x14
    unused!(),                      // 0x15
    unused!(),                      // 0x16
    unused!(),                      // 0x17
    unused!(),                      // 0x18
    unused!(),                      // 0x19
    unused!(),                      // 0x1a
    unused!(),                      // 0x1b
    unused!(),                      // 0x1c
    unused!(),                      // 0x1d
    unused!(),                      // 0x1e
    unused!(),                      // 0x1f
    unused!(),                      // 0x20
    unused!(),                      // 0x21
    unused!(),                      // 0x22
    unused!(),                      // 0x23
    unused!(),                      // 0x24
    unused!(),                      // 0x25
    unused!(),                      // 0x26
    unused!(),                      // 0x27
    unused!(),                      // 0x28
    unused!(),                      // 0x29
    unused!(),                      // 0x2a
    unused!(),                      // 0x2b
    unused!(),                      // 0x2c
    unused!(),                      // 0x2d
    unused!(),                      // 0x2e
    unused!(),                      // 0x2f
    unused!(),                      // 0x30
    unused!(),                      // 0x31
    unused!(),                      // 0x32
    unused!(),                      // 0x33
    unused!(),                      // 0x34
    unused!(),                      // 0x35
    unused!(),                      // 0x36
    unused!(),                      // 0x37
    unused!(),                      // 0x38
    unused!(),                      // 0x39
    unused!(),                      // 0x3a
    unused!(),                      // 0x3b
    unused!(),                      // 0x3c
    unused!(),                      // 0x3d
    unused!(),                      // 0x3e
    unused!(),                      // 0x3f
    unused!(),                      // 0x40
    unused!(),                      // 0x41
    unused!(),                      // 0x42
    unused!(),                      // 0x43
    unused!(),                      // 0x44
    unused!(),                      // 0x45
    unused!(),                      // 0x46
    unused!(),                      // 0x47
    unused!(),                      // 0x48
    unused!(),                      // 0x49
    unused!(),                      // 0x4a
    unused!(),                      // 0x4b
    unused!(),                      // 0x4c
    unused!(),                      // 0x4d
    unused!(),                      // 0x4e
    unused!(),                      // 0x4f
    unused!(),                      // 0x50
    unused!(),                      // 0x51
    unused!(),                      // 0x52
    unused!(),                      // 0x53
    unused!(),                      // 0x54
    unused!(),                      // 0x55
    unused!(),                      // 0x56
    unused!(),                      // 0x57
    unused!(),                      // 0x58
    unused!(),                      // 0x59
    unused!(),                      // 0x5a
    unused!(),                      // 0x5b
    unused!(),                      // 0x5c
    unused!(),                      // 0x5d
    unused!(),                      // 0x5e
    unused!(),                      // 0x5f
    unused!(),                      // 0x60
    unused!(),                      // 0x61
    unused!(),                      // 0x62
    unused!(),                      // 0x63
    unused!(),                      // 0x64
    unused!(),                      // 0x65
    unused!(),                      // 0x66
    unused!(),                      // 0x67
    unused!(),                      // 0x68
    unused!(),                      // 0x69
    unused!(),                      // 0x6a
    unused!(),                      // 0x6b
    unused!(),                      // 0x6c
    unused!(),                      // 0x6d
    unused!(),                      // 0x6e
    unused!(),                      // 0x6f
    unused!(),                      // 0x70
    unused!(),                      // 0x71
    unused!(),                      // 0x72
    unused!(),                      // 0x73
    unused!(),                      // 0x74
    unused!(),                      // 0x75
    unused!(),                      // 0x76
    unused!(),                      // 0x77
    unused!(),                      // 0x78
    unused!(),                      // 0x79
    unused!(),                      // 0x7a
    unused!(),                      // 0x7b
    unused!(),                      // 0x7c
    unused!(),                      // 0x7d
    unused!(),                      // 0x7e
    unused!(),                      // 0x7f
    unused!(),                      // 0x80
    unused!(),                      // 0x81
    unused!(),                      // 0x82
    unused!(),                      // 0x83
    unused!(),                      // 0x84
    unused!(),                      // 0x85
    unused!(),                      // 0x86
    unused!(),                      // 0x87
    unused!(),                      // 0x88
    unused!(),                      // 0x89
    unused!(),                      // 0x8a
    unused!(),                      // 0x8b
    unused!(),                      // 0x8c
    unused!(),                      // 0x8d
    unused!(),                      // 0x8e
    unused!(),                      // 0x8f
    unused!(),                      // 0x90
    unused!(),                      // 0x91
    unused!(),                      // 0x92
    unused!(),                      // 0x93
    unused!(),                      // 0x94
    unused!(),                      // 0x95
    unused!(),                      // 0x96
    unused!(),                      // 0x97
    unused!(),                      // 0x98
    unused!(),                      // 0x99
    unused!(),                      // 0x9a
    unused!(),                      // 0x9b
    unused!(),                      // 0x9c
    unused!(),                      // 0x9d
    unused!(),                      // 0x9e
    unused!(),                      // 0x9f
    unused!(),                      // 0xa0
    unused!(),                      // 0xa1
    unused!(),                      // 0xa2
    unused!(),                      // 0xa3
    unused!(),                      // 0xa4
    unused!(),                      // 0xa5
    unused!(),                      // 0xa6
    unused!(),                      // 0xa7
    unused!(),                      // 0xa8
    unused!(),                      // 0xa9
    unused!(),                      // 0xaa
    unused!(),                      // 0xab
    unused!(),                      // 0xac
    unused!(),                      // 0xad
    unused!(),                      // 0xae
    unused!(),                      // 0xaf
    unused!(),                      // 0xb0
    unused!(),                      // 0xb1
    unused!(),                      // 0xb2
    unused!(),                      // 0xb3
    unused!(),                      // 0xb4
    unused!(),                      // 0xb5
    unused!(),                      // 0xb6
    unused!(),                      // 0xb7
    unused!(),                      // 0xb8
    unused!(),                      // 0xb9
    unused!(),                      // 0xba
    unused!(),                      // 0xbb
    unused!(),                      // 0xbc
    unused!(),                      // 0xbd
    unused!(),                      // 0xbe
    unused!(),                      // 0xbf
    unused!(),                      // 0xc0
    unused!(),                      // 0xc1
    unused!(),                      // 0xc2
    unused!(),                      // 0xc3
    unused!(),                      // 0xc4
    unused!(),                      // 0xc5
    unused!(),                      // 0xc6
    unused!(),                      // 0xc7
    unused!(),                      // 0xc8
    unused!(),                      // 0xc9
    unused!(),                      // 0xca
    unused!(),                      // 0xcb
    opcode!("sha1rnds4"; V/dq, W/dq, I/b),                      // 0xcc
    unused!(),                      // 0xcd
    unused!(),                      // 0xce
    unused!(),                      // 0xcf
    unused!(),                      // 0xd0
    unused!(),                      // 0xd1
    unused!(),                      // 0xd2
    unused!(),                      // 0xd3
    unused!(),                      // 0xd4
    unused!(),                      // 0xd5
    unused!(),                      // 0xd6
    unused!(),                      // 0xd7
    unused!(),                      // 0xd8
    unused!(),                      // 0xd9
    unused!(),                      // 0xda
    unused!(),                      // 0xdb
    unused!(),                      // 0xdc
    unused!(),                      // 0xdd
    unused!(),                      // 0xde
    unused!(),                      // 0xdf
    unused!(),                      // 0xe0
    unused!(),                      // 0xe1
    unused!(),                      // 0xe2
    unused!(),                      // 0xe3
    unused!(),                      // 0xe4
    unused!(),                      // 0xe5
    unused!(),                      // 0xe6
    unused!(),                      // 0xe7
    unused!(),                      // 0xe8
    unused!(),                      // 0xe9
    unused!(),                      // 0xea
    unused!(),                      // 0xeb
    unused!(),                      // 0xec
    unused!(),                      // 0xed
    unused!(),                      // 0xee
    unused!(),                      // 0xef
    unused!(),                      // 0xf0
    unused!(),                      // 0xf1
    unused!(),                      // 0xf2
    unused!(),                      // 0xf3
    unused!(),                      // 0xf4
    unused!(),                      // 0xf5
    unused!(),                      // 0xf6
    unused!(),                      // 0xf7
    unused!(),                      // 0xf8
    unused!(),                      // 0xf9
    unused!(),                      // 0xfa
    unused!(),                      // 0xfb
    unused!(),                      // 0xfc
    unused!(),                      // 0xfd
    unused!(),                      // 0xfe
    unused!(),                      // 0xff
];

pub static THREEBYTE_3A66_TABLE: [Opcode; 256] = [
    opcode!("vpermq"; V/qq, W/qq, I/b),                      // 0x00
    opcode!("vpermpd"; V/qq, W/qq, I/b),                      // 0x01
    opcode!("vblendd"; V/x, H/x, W/x, I/b),                      // 0x02
    unused!(),                      // 0x03
    opcode!("vpermilps"; V/x, W/x, I/b),                      // 0x04
    opcode!("vpermilpd"; V/x, W/x, I/b),                      // 0x05
    opcode!("vperm2f128"; V/qq, H/qq, W/qq, I/b),                      // 0x06
    unused!(),                      // 0x07
    opcode!("vroundps"; V/x, W/x, I/b),  // 0x08: roundps
    opcode!("vroundpd"; V/x, W/x, I/b),  // 0x09: roundpd
    opcode!("vroundss"; V/ss, W/ss, I/b),// 0x0a: roundss
    opcode!("vroundsd"; V/sd, W/sd, I/b),// 0x0b: roundsd
    opcode!("vblendps"; V/x, H/x, W/x, I/b),// 0x0c: blendps
    opcode!("vblendpd"; V/x, H/x, W/x, I/b),// 0x0d: blendpd
    opcode!("vpblendw"; V/x, H/x, W/x, I/b),// 0x0e: pblendw
    opcode!("vpalignr"; V/x, H/x, W/x, I/b),// 0x0f: palignr
    unused!(),                      // 0x10
    unused!(),                      // 0x11
    unused!(),                      // 0x12
    unused!(),                      // 0x13
    opcode!("vpextrb"; M/b, V/dq, I/b),// 0x14: pextrb
    opcode!("vpextrw"; M/w, V/dq, I/b),// 0x15: pextrw
    opcode!("vpextrd"; E/y, V/dq, I/b), // 0x16: pextrd
    opcode!("vextractps"; E/d, V/dq, I/b),// 0x17: extractps
    opcode!("vinserti128"; V/qq, H/qq, W/qq, I/b),                      // 0x18
    opcode!("vextracti128"; W/dq, V/dq, I/b),                      // 0x19
    unused!(),                      // 0x1a
    unused!(),                      // 0x1b
    unused!(),                      // 0x1c
    unused!(),                      // 0x1d
    unused!(),                      // 0x1e
    unused!(),                      // 0x1f
    opcode!("vpinsrb"; V/dq, H/dq, M/d, I/b),// 0x20: pinsrb
    opcode!("vinsertps"; V/dq, H/dq, M/d, I/b),// 0x21: insertps
    opcode!("vpinsrd"; V/dq, H/dq, E/y, I/b), // 0x22: pinsrd
    unused!(),                      // 0x23
    unused!(),                      // 0x24
    unused!(),                      // 0x25
    unused!(),                      // 0x26
    unused!(),                      // 0x27
    unused!(),                      // 0x28
    unused!(),                      // 0x29
    unused!(),                      // 0x2a
    unused!(),                      // 0x2b
    unused!(),                      // 0x2c
    unused!(),                      // 0x2d
    unused!(),                      // 0x2e
    unused!(),                      // 0x2f
    unused!(),                      // 0x30
    unused!(),                      // 0x31
    unused!(),                      // 0x32
    unused!(),                      // 0x33
    unused!(),                      // 0x34
    unused!(),                      // 0x35
    unused!(),                      // 0x36
    unused!(),                      // 0x37
    unused!(),                      // 0x38
    unused!(),                      // 0x39
    unused!(),                      // 0x3a
    unused!(),                      // 0x3b
    unused!(),                      // 0x3c
    unused!(),                      // 0x3d
    unused!(),                      // 0x3e
    unused!(),                      // 0x3f
    opcode!("vdpps"; V/x, H/x, W/x, I/b),  // 0x40: dpps
    opcode!("vdppd"; V/dq, H/dq, W/dq, I/b),  // 0x41: dppd
    opcode!("vmpsadbw"; V/x, H/x, W/x, I/b),// 0x42: mpsadbw
    unused!(),                      // 0x43
    opcode!("vpclmulqdq"; V/dq, H/dq, W/dq, I/b),// 0x44: pclmulqdq
    unused!(),                      // 0x45
    opcode!("vperm2i128"; V/qq, H/qq, W/qq, I/b),                      // 0x46
    unused!(),                      // 0x47
    unused!(),                      // 0x48
    unused!(),                      // 0x49
    opcode!("vblendvps"; V/x, H/x, W/x, L/x),                      // 0x4a
    opcode!("vblendvpd"; V/x, H/x, W/x, L/x),                      // 0x4b
    opcode!("vblendvb"; V/x, H/x, W/x, L/x),                      // 0x4c
    unused!(),                      // 0x4d
    unused!(),                      // 0x4e
    unused!(),                      // 0x4f
    unused!(),                      // 0x50
    unused!(),                      // 0x51
    unused!(),                      // 0x52
    unused!(),                      // 0x53
    unused!(),                      // 0x54
    unused!(),                      // 0x55
    unused!(),                      // 0x56
    unused!(),                      // 0x57
    unused!(),                      // 0x58
    unused!(),                      // 0x59
    unused!(),                      // 0x5a
    unused!(),                      // 0x5b
    unused!(),                      // 0x5c
    unused!(),                      // 0x5d
    unused!(),                      // 0x5e
    unused!(),                      // 0x5f
    opcode!("vpcmpestrm"; V/dq, W/dq, I/b),// 0x60: pcmpestrm
    opcode!("vpcmpestri"; V/dq, W/dq, I/b),// 0x61: pcmpestri
    opcode!("vpcmpistrm"; V/dq, W/dq, I/b),// 0x62: pcmpistrm
    opcode!("vpcmpistri"; V/dq, W/dq, I/b),// 0x63: pcmpistri
    unused!(),                      // 0x64
    unused!(),                      // 0x65
    unused!(),                      // 0x66
    unused!(),                      // 0x67
    unused!(),                      // 0x68
    unused!(),                      // 0x69
    unused!(),                      // 0x6a
    unused!(),                      // 0x6b
    unused!(),                      // 0x6c
    unused!(),                      // 0x6d
    unused!(),                      // 0x6e
    unused!(),                      // 0x6f
    unused!(),                      // 0x70
    unused!(),                      // 0x71
    unused!(),                      // 0x72
    unused!(),                      // 0x73
    unused!(),                      // 0x74
    unused!(),                      // 0x75
    unused!(),                      // 0x76
    unused!(),                      // 0x77
    unused!(),                      // 0x78
    unused!(),                      // 0x79
    unused!(),                      // 0x7a
    unused!(),                      // 0x7b
    unused!(),                      // 0x7c
    unused!(),                      // 0x7d
    unused!(),                      // 0x7e
    unused!(),                      // 0x7f
    unused!(),                      // 0x80
    unused!(),                      // 0x81
    unused!(),                      // 0x82
    unused!(),                      // 0x83
    unused!(),                      // 0x84
    unused!(),                      // 0x85
    unused!(),                      // 0x86
    unused!(),                      // 0x87
    unused!(),                      // 0x88
    unused!(),                      // 0x89
    unused!(),                      // 0x8a
    unused!(),                      // 0x8b
    unused!(),                      // 0x8c
    unused!(),                      // 0x8d
    unused!(),                      // 0x8e
    unused!(),                      // 0x8f
    unused!(),                      // 0x90
    unused!(),                      // 0x91
    unused!(),                      // 0x92
    unused!(),                      // 0x93
    unused!(),                      // 0x94
    unused!(),                      // 0x95
    unused!(),                      // 0x96
    unused!(),                      // 0x97
    unused!(),                      // 0x98
    unused!(),                      // 0x99
    unused!(),                      // 0x9a
    unused!(),                      // 0x9b
    unused!(),                      // 0x9c
    unused!(),                      // 0x9d
    unused!(),                      // 0x9e
    unused!(),                      // 0x9f
    unused!(),                      // 0xa0
    unused!(),                      // 0xa1
    unused!(),                      // 0xa2
    unused!(),                      // 0xa3
    unused!(),                      // 0xa4
    unused!(),                      // 0xa5
    unused!(),                      // 0xa6
    unused!(),                      // 0xa7
    unused!(),                      // 0xa8
    unused!(),                      // 0xa9
    unused!(),                      // 0xaa
    unused!(),                      // 0xab
    unused!(),                      // 0xac
    unused!(),                      // 0xad
    unused!(),                      // 0xae
    unused!(),                      // 0xaf
    unused!(),                      // 0xb0
    unused!(),                      // 0xb1
    unused!(),                      // 0xb2
    unused!(),                      // 0xb3
    unused!(),                      // 0xb4
    unused!(),                      // 0xb5
    unused!(),                      // 0xb6
    unused!(),                      // 0xb7
    unused!(),                      // 0xb8
    unused!(),                      // 0xb9
    unused!(),                      // 0xba
    unused!(),                      // 0xbb
    unused!(),                      // 0xbc
    unused!(),                      // 0xbd
    unused!(),                      // 0xbe
    unused!(),                      // 0xbf
    unused!(),                      // 0xc0
    unused!(),                      // 0xc1
    unused!(),                      // 0xc2
    unused!(),                      // 0xc3
    unused!(),                      // 0xc4
    unused!(),                      // 0xc5
    unused!(),                      // 0xc6
    unused!(),                      // 0xc7
    unused!(),                      // 0xc8
    unused!(),                      // 0xc9
    unused!(),                      // 0xca
    unused!(),                      // 0xcb
    unused!(),                      // 0xcc
    unused!(),                      // 0xcd
    unused!(),                      // 0xce
    unused!(),                      // 0xcf
    unused!(),                      // 0xd0
    unused!(),                      // 0xd1
    unused!(),                      // 0xd2
    unused!(),                      // 0xd3
    unused!(),                      // 0xd4
    unused!(),                      // 0xd5
    unused!(),                      // 0xd6
    unused!(),                      // 0xd7
    unused!(),                      // 0xd8
    unused!(),                      // 0xd9
    unused!(),                      // 0xda
    unused!(),                      // 0xdb
    unused!(),                      // 0xdc
    unused!(),                      // 0xdd
    unused!(),                      // 0xde
    opcode!("aeskeygenassist"; V/dq, W/dq, I/b),// 0xdf: aeskeygenassist
    unused!(),                      // 0xe0
    unused!(),                      // 0xe1
    unused!(),                      // 0xe2
    unused!(),                      // 0xe3
    unused!(),                      // 0xe4
    unused!(),                      // 0xe5
    unused!(),                      // 0xe6
    unused!(),                      // 0xe7
    unused!(),                      // 0xe8
    unused!(),                      // 0xe9
    unused!(),                      // 0xea
    unused!(),                      // 0xeb
    unused!(),                      // 0xec
    unused!(),                      // 0xed
    unused!(),                      // 0xee
    unused!(),                      // 0xef
    unused!(),                      // 0xf0
    unused!(),                      // 0xf1
    unused!(),                      // 0xf2
    unused!(),                      // 0xf3
    unused!(),                      // 0xf4
    unused!(),                      // 0xf5
    unused!(),                      // 0xf6
    unused!(),                      // 0xf7
    unused!(),                      // 0xf8
    unused!(),                      // 0xf9
    unused!(),                      // 0xfa
    unused!(),                      // 0xfb
    unused!(),                      // 0xfc
    unused!(),                      // 0xfd
    unused!(),                      // 0xfe
    unused!(),                      // 0xff
];

pub static THREEBYTE_3AF2_TABLE: [Opcode; 256] = [
    unused!(),                      // 0x00
    unused!(),                      // 0x01
    unused!(),                      // 0x02
    unused!(),                      // 0x03
    unused!(),                      // 0x04
    unused!(),                      // 0x05
    unused!(),                      // 0x06
    unused!(),                      // 0x07
    unused!(),                      // 0x08
    unused!(),                      // 0x09
    unused!(),                      // 0x0a
    unused!(),                      // 0x0b
    unused!(),                      // 0x0c
    unused!(),                      // 0x0d
    unused!(),                      // 0x0e
    unused!(),                      // 0x0f
    unused!(),                      // 0x10
    unused!(),                      // 0x11
    unused!(),                      // 0x12
    unused!(),                      // 0x13
    unused!(),                      // 0x14
    unused!(),                      // 0x15
    unused!(),                      // 0x16
    unused!(),                      // 0x17
    unused!(),                      // 0x18
    unused!(),                      // 0x19
    unused!(),                      // 0x1a
    unused!(),                      // 0x1b
    unused!(),                      // 0x1c
    unused!(),                      // 0x1d
    unused!(),                      // 0x1e
    unused!(),                      // 0x1f
    unused!(),                      // 0x20
    unused!(),                      // 0x21
    unused!(),                      // 0x22
    unused!(),                      // 0x23
    unused!(),                      // 0x24
    unused!(),                      // 0x25
    unused!(),                      // 0x26
    unused!(),                      // 0x27
    unused!(),                      // 0x28
    unused!(),                      // 0x29
    unused!(),                      // 0x2a
    unused!(),                      // 0x2b
    unused!(),                      // 0x2c
    unused!(),                      // 0x2d
    unused!(),                      // 0x2e
    unused!(),                      // 0x2f
    unused!(),                      // 0x30
    unused!(),                      // 0x31
    unused!(),                      // 0x32
    unused!(),                      // 0x33
    unused!(),                      // 0x34
    unused!(),                      // 0x35
    unused!(),                      // 0x36
    unused!(),                      // 0x37
    unused!(),                      // 0x38
    unused!(),                      // 0x39
    unused!(),                      // 0x3a
    unused!(),                      // 0x3b
    unused!(),                      // 0x3c
    unused!(),                      // 0x3d
    unused!(),                      // 0x3e
    unused!(),                      // 0x3f
    unused!(),                      // 0x40
    unused!(),                      // 0x41
    unused!(),                      // 0x42
    unused!(),                      // 0x43
    unused!(),                      // 0x44
    unused!(),                      // 0x45
    unused!(),                      // 0x46
    unused!(),                      // 0x47
    unused!(),                      // 0x48
    unused!(),                      // 0x49
    unused!(),                      // 0x4a
    unused!(),                      // 0x4b
    unused!(),                      // 0x4c
    unused!(),                      // 0x4d
    unused!(),                      // 0x4e
    unused!(),                      // 0x4f
    unused!(),                      // 0x50
    unused!(),                      // 0x51
    unused!(),                      // 0x52
    unused!(),                      // 0x53
    unused!(),                      // 0x54
    unused!(),                      // 0x55
    unused!(),                      // 0x56
    unused!(),                      // 0x57
    unused!(),                      // 0x58
    unused!(),                      // 0x59
    unused!(),                      // 0x5a
    unused!(),                      // 0x5b
    unused!(),                      // 0x5c
    unused!(),                      // 0x5d
    unused!(),                      // 0x5e
    unused!(),                      // 0x5f
    unused!(),                      // 0x60
    unused!(),                      // 0x61
    unused!(),                      // 0x62
    unused!(),                      // 0x63
    unused!(),                      // 0x64
    unused!(),                      // 0x65
    unused!(),                      // 0x66
    unused!(),                      // 0x67
    unused!(),                      // 0x68
    unused!(),                      // 0x69
    unused!(),                      // 0x6a
    unused!(),                      // 0x6b
    unused!(),                      // 0x6c
    unused!(),                      // 0x6d
    unused!(),                      // 0x6e
    unused!(),                      // 0x6f
    unused!(),                      // 0x70
    unused!(),                      // 0x71
    unused!(),                      // 0x72
    unused!(),                      // 0x73
    unused!(),                      // 0x74
    unused!(),                      // 0x75
    unused!(),                      // 0x76
    unused!(),                      // 0x77
    unused!(),                      // 0x78
    unused!(),                      // 0x79
    unused!(),                      // 0x7a
    unused!(),                      // 0x7b
    unused!(),                      // 0x7c
    unused!(),                      // 0x7d
    unused!(),                      // 0x7e
    unused!(),                      // 0x7f
    unused!(),                      // 0x80
    unused!(),                      // 0x81
    unused!(),                      // 0x82
    unused!(),                      // 0x83
    unused!(),                      // 0x84
    unused!(),                      // 0x85
    unused!(),                      // 0x86
    unused!(),                      // 0x87
    unused!(),                      // 0x88
    unused!(),                      // 0x89
    unused!(),                      // 0x8a
    unused!(),                      // 0x8b
    unused!(),                      // 0x8c
    unused!(),                      // 0x8d
    unused!(),                      // 0x8e
    unused!(),                      // 0x8f
    unused!(),                      // 0x90
    unused!(),                      // 0x91
    unused!(),                      // 0x92
    unused!(),                      // 0x93
    unused!(),                      // 0x94
    unused!(),                      // 0x95
    unused!(),                      // 0x96
    unused!(),                      // 0x97
    unused!(),                      // 0x98
    unused!(),                      // 0x99
    unused!(),                      // 0x9a
    unused!(),                      // 0x9b
    unused!(),                      // 0x9c
    unused!(),                      // 0x9d
    unused!(),                      // 0x9e
    unused!(),                      // 0x9f
    unused!(),                      // 0xa0
    unused!(),                      // 0xa1
    unused!(),                      // 0xa2
    unused!(),                      // 0xa3
    unused!(),                      // 0xa4
    unused!(),                      // 0xa5
    unused!(),                      // 0xa6
    unused!(),                      // 0xa7
    unused!(),                      // 0xa8
    unused!(),                      // 0xa9
    unused!(),                      // 0xaa
    unused!(),                      // 0xab
    unused!(),                      // 0xac
    unused!(),                      // 0xad
    unused!(),                      // 0xae
    unused!(),                      // 0xaf
    unused!(),                      // 0xb0
    unused!(),                      // 0xb1
    unused!(),                      // 0xb2
    unused!(),                      // 0xb3
    unused!(),                      // 0xb4
    unused!(),                      // 0xb5
    unused!(),                      // 0xb6
    unused!(),                      // 0xb7
    unused!(),                      // 0xb8
    unused!(),                      // 0xb9
    unused!(),                      // 0xba
    unused!(),                      // 0xbb
    unused!(),                      // 0xbc
    unused!(),                      // 0xbd
    unused!(),                      // 0xbe
    unused!(),                      // 0xbf
    unused!(),                      // 0xc0
    unused!(),                      // 0xc1
    unused!(),                      // 0xc2
    unused!(),                      // 0xc3
    unused!(),                      // 0xc4
    unused!(),                      // 0xc5
    unused!(),                      // 0xc6
    unused!(),                      // 0xc7
    unused!(),                      // 0xc8
    unused!(),                      // 0xc9
    unused!(),                      // 0xca
    unused!(),                      // 0xcb
    unused!(),                      // 0xcc
    unused!(),                      // 0xcd
    unused!(),                      // 0xce
    unused!(),                      // 0xcf
    unused!(),                      // 0xd0
    unused!(),                      // 0xd1
    unused!(),                      // 0xd2
    unused!(),                      // 0xd3
    unused!(),                      // 0xd4
    unused!(),                      // 0xd5
    unused!(),                      // 0xd6
    unused!(),                      // 0xd7
    unused!(),                      // 0xd8
    unused!(),                      // 0xd9
    unused!(),                      // 0xda
    unused!(),                      // 0xdb
    unused!(),                      // 0xdc
    unused!(),                      // 0xdd
    unused!(),                      // 0xde
    unused!(),                      // 0xdf
    unused!(),                      // 0xe0
    unused!(),                      // 0xe1
    unused!(),                      // 0xe2
    unused!(),                      // 0xe3
    unused!(),                      // 0xe4
    unused!(),                      // 0xe5
    unused!(),                      // 0xe6
    unused!(),                      // 0xe7
    unused!(),                      // 0xe8
    unused!(),                      // 0xe9
    unused!(),                      // 0xea
    unused!(),                      // 0xeb
    unused!(),                      // 0xec
    unused!(),                      // 0xed
    unused!(),                      // 0xee
    unused!(),                      // 0xef
    unused!(),                      // 0xf0
    unused!(),                      // 0xf1
    unused!(),                      // 0xf2
    unused!(),                      // 0xf3
    unused!(),                      // 0xf4
    unused!(),                      // 0xf5
    unused!(),                      // 0xf6
    unused!(),                      // 0xf7
    unused!(),                      // 0xf8
    unused!(),                      // 0xf9
    unused!(),                      // 0xfa
    unused!(),                      // 0xfb
    unused!(),                      // 0xfc
    unused!(),                      // 0xfd
    unused!(),                      // 0xfe
    unused!(),                      // 0xff
];

pub static THREEDNOW_TABLE: [Opcode; 256] = [
    unused!(),                      // 0x00
    unused!(),                      // 0x01
    unused!(),                      // 0x02
    unused!(),                      // 0x03
    unused!(),                      // 0x04
    unused!(),                      // 0x05
    unused!(),                      // 0x06
    unused!(),                      // 0x07
    unused!(),                      // 0x08
    unused!(),                      // 0x09
    unused!(),                      // 0x0a
    unused!(),                      // 0x0b
    opcode!("pi2fw"; P/q, Q/q),         // 0x0c: pi2fw
    opcode!("pi2fd"; P/q, Q/q),         // 0x0d: pi2fd
    unused!(),                      // 0x0e
    unused!(),                      // 0x0f
    unused!(),                      // 0x10
    unused!(),                      // 0x11
    unused!(),                      // 0x12
    unused!(),                      // 0x13
    unused!(),                      // 0x14
    unused!(),                      // 0x15
    unused!(),                      // 0x16
    unused!(),                      // 0x17
    unused!(),                      // 0x18
    unused!(),                      // 0x19
    unused!(),                      // 0x1a
    unused!(),                      // 0x1b
    opcode!("pf2iw"; P/q, Q/q),         // 0x1c: pf2iw
    opcode!("pf2id"; P/q, Q/q),         // 0x1d: pf2id
    unused!(),                      // 0x1e
    unused!(),                      // 0x1f
    unused!(),                      // 0x20
    unused!(),                      // 0x21
    unused!(),                      // 0x22
    unused!(),                      // 0x23
    unused!(),                      // 0x24
    unused!(),                      // 0x25
    unused!(),                      // 0x26
    unused!(),                      // 0x27
    unused!(),                      // 0x28
    unused!(),                      // 0x29
    unused!(),                      // 0x2a
    unused!(),                      // 0x2b
    unused!(),                      // 0x2c
    unused!(),                      // 0x2d
    unused!(),                      // 0x2e
    unused!(),                      // 0x2f
    unused!(),                      // 0x30
    unused!(),                      // 0x31
    unused!(),                      // 0x32
    unused!(),                      // 0x33
    unused!(),                      // 0x34
    unused!(),                      // 0x35
    unused!(),                      // 0x36
    unused!(),                      // 0x37
    unused!(),                      // 0x38
    unused!(),                      // 0x39
    unused!(),                      // 0x3a
    unused!(),                      // 0x3b
    unused!(),                      // 0x3c
    unused!(),                      // 0x3d
    unused!(),                      // 0x3e
    unused!(),                      // 0x3f
    unused!(),                      // 0x40
    unused!(),                      // 0x41
    unused!(),                      // 0x42
    unused!(),                      // 0x43
    unused!(),                      // 0x44
    unused!(),                      // 0x45
    unused!(),                      // 0x46
    unused!(),                      // 0x47
    unused!(),                      // 0x48
    unused!(),                      // 0x49
    unused!(),                      // 0x4a
    unused!(),                      // 0x4b
    unused!(),                      // 0x4c
    unused!(),                      // 0x4d
    unused!(),                      // 0x4e
    unused!(),                      // 0x4f
    unused!(),                      // 0x50
    unused!(),                      // 0x51
    unused!(),                      // 0x52
    unused!(),                      // 0x53
    unused!(),                      // 0x54
    unused!(),                      // 0x55
    unused!(),                      // 0x56
    unused!(),                      // 0x57
    unused!(),                      // 0x58
    unused!(),                      // 0x59
    unused!(),                      // 0x5a
    unused!(),                      // 0x5b
    unused!(),                      // 0x5c
    unused!(),                      // 0x5d
    unused!(),                      // 0x5e
    unused!(),                      // 0x5f
    unused!(),                      // 0x60
    unused!(),                      // 0x61
    unused!(),                      // 0x62
    unused!(),                      // 0x63
    unused!(),                      // 0x64
    unused!(),                      // 0x65
    unused!(),                      // 0x66
    unused!(),                      // 0x67
    unused!(),                      // 0x68
    unused!(),                      // 0x69
    unused!(),                      // 0x6a
    unused!(),                      // 0x6b
    unused!(),                      // 0x6c
    unused!(),                      // 0x6d
    unused!(),                      // 0x6e
    unused!(),                      // 0x6f
    unused!(),                      // 0x70
    unused!(),                      // 0x71
    unused!(),                      // 0x72
    unused!(),                      // 0x73
    unused!(),                      // 0x74
    unused!(),                      // 0x75
    unused!(),                      // 0x76
    unused!(),                      // 0x77
    unused!(),                      // 0x78
    unused!(),                      // 0x79
    unused!(),                      // 0x7a
    unused!(),                      // 0x7b
    unused!(),                      // 0x7c
    unused!(),                      // 0x7d
    unused!(),                      // 0x7e
    unused!(),                      // 0x7f
    unused!(),                      // 0x80
    unused!(),                      // 0x81
    unused!(),                      // 0x82
    unused!(),                      // 0x83
    unused!(),                      // 0x84
    unused!(),                      // 0x85
    unused!(),                      // 0x86
    unused!(),                      // 0x87
    unused!(),                      // 0x88
    unused!(),                      // 0x89
    opcode!("pfnacc"; P/q, Q/q),        // 0x8a: pfnacc
    unused!(),                      // 0x8b
    unused!(),                      // 0x8c
    unused!(),                      // 0x8d
    opcode!("pfpnacc"; P/q, Q/q),       // 0x8e: pfpnacc
    unused!(),                      // 0x8f
    opcode!("pfcmpge"; P/q, Q/q),       // 0x90: pfcmpge
    unused!(),                      // 0x91
    unused!(),                      // 0x92
    unused!(),                      // 0x93
    opcode!("pfmin"; P/q, Q/q),         // 0x94: pfmin
    unused!(),                      // 0x95
    opcode!("pfrcp"; P/q, Q/q),         // 0x96: pfrcp
    opcode!("pfrsqrt"; P/q, Q/q),       // 0x97: pfrsqrt
    unused!(),                      // 0x98
    unused!(),                      // 0x99
    opcode!("pfsub"; P/q, Q/q),         // 0x9a: pfsub
    unused!(),                      // 0x9b
    unused!(),                      // 0x9c
    unused!(),                      // 0x9d
    opcode!("pfadd"; P/q, Q/q),         // 0x9e: pfadd
    unused!(),                      // 0x9f
    opcode!("pfcmpgt"; P/q, Q/q),       // 0xa0: pfcmpgt
    unused!(),                      // 0xa1
    unused!(),                      // 0xa2
    unused!(),                      // 0xa3
    opcode!("pfmax"; P/q, Q/q),         // 0xa4: pfmax
    unused!(),                      // 0xa5
    opcode!("pfrcpit1"; P/q, Q/q),      // 0xa6: pfrcpit1
    opcode!("pfrsqit1"; P/q, Q/q),      // 0xa7: pfrsqit1
    unused!(),                      // 0xa8
    unused!(),                      // 0xa9
    opcode!("pfsubr"; P/q, Q/q),        // 0xaa: pfsubr
    unused!(),                      // 0xab
    unused!(),                      // 0xac
    unused!(),                      // 0xad
    opcode!("pfacc"; P/q, Q/q),         // 0xae: pfacc
    unused!(),                      // 0xaf
    opcode!("pfcmpeq"; P/q, Q/q),       // 0xb0: pfcmpeq
    unused!(),                      // 0xb1
    unused!(),                      // 0xb2
    unused!(),                      // 0xb3
    opcode!("pfmul"; P/q, Q/q),         // 0xb4: pfmul
    unused!(),                      // 0xb5
    opcode!("pfrcpit2"; P/q, Q/q),      // 0xb6: pfrcpit2
    opcode!("pmulhrw"; P/q, Q/q),       // 0xb7: pmulhrw
    unused!(),                      // 0xb8
    unused!(),                      // 0xb9
    unused!(),                      // 0xba
    opcode!("pswapd"; P/q, Q/q),        // 0xbb: pswapd
    unused!(),                      // 0xbc
    unused!(),                      // 0xbd
    unused!(),                      // 0xbe
    opcode!("pavgusb"; P/q, Q/q),       // 0xbf: pavgusb
    unused!(),                      // 0xc0
    unused!(),                      // 0xc1
    unused!(),                      // 0xc2
    unused!(),                      // 0xc3
    unused!(),                      // 0xc4
    unused!(),                      // 0xc5
    unused!(),                      // 0xc6
    unused!(),                      // 0xc7
    unused!(),                      // 0xc8
    unused!(),                      // 0xc9
    unused!(),                      // 0xca
    unused!(),                      // 0xcb
    unused!(),                      // 0xcc
    unused!(),                      // 0xcd
    unused!(),                      // 0xce
    unused!(),                      // 0xcf
    unused!(),                      // 0xd0
    unused!(),                      // 0xd1
    unused!(),                      // 0xd2
    unused!(),                      // 0xd3
    unused!(),                      // 0xd4
    unused!(),                      // 0xd5
    unused!(),                      // 0xd6
    unused!(),                      // 0xd7
    unused!(),                      // 0xd8
    unused!(),                      // 0xd9
    unused!(),                      // 0xda
    unused!(),                      // 0xdb
    unused!(),                      // 0xdc
    unused!(),                      // 0xdd
    unused!(),                      // 0xde
    unused!(),                      // 0xdf
    unused!(),                      // 0xe0
    unused!(),                      // 0xe1
    unused!(),                      // 0xe2
    unused!(),                      // 0xe3
    unused!(),                      // 0xe4
    unused!(),                      // 0xe5
    unused!(),                      // 0xe6
    unused!(),                      // 0xe7
    unused!(),                      // 0xe8
    unused!(),                      // 0xe9
    unused!(),                      // 0xea
    unused!(),                      // 0xeb
    unused!(),                      // 0xec
    unused!(),                      // 0xed
    unused!(),                      // 0xee
    unused!(),                      // 0xef
    unused!(),                      // 0xf0
    unused!(),                      // 0xf1
    unused!(),                      // 0xf2
    unused!(),                      // 0xf3
    unused!(),                      // 0xf4
    unused!(),                      // 0xf5
    unused!(),                      // 0xf6
    unused!(),                      // 0xf7
    unused!(),                      // 0xf8
    unused!(),                      // 0xf9
    unused!(),                      // 0xfa
    unused!(),                      // 0xfb
    unused!(),                      // 0xfc
    unused!(),                      // 0xfd
    unused!(),                      // 0xfe
    unused!(),                      // 0xff
];

pub static XOP8_TABLE: [Opcode; 256] = [
    unused!(),                      // 0x00
    unused!(),                      // 0x01
    unused!(),                      // 0x02
    unused!(),                      // 0x03
    unused!(),                      // 0x04
    unused!(),                      // 0x05
    unused!(),                      // 0x06
    unused!(),                      // 0x07
    unused!(),                      // 0x08
    unused!(),                      // 0x09
    unused!(),                      // 0x0a
    unused!(),                      // 0x0b
    unused!(),                      // 0x0c
    unused!(),                      // 0x0d
    unused!(),                      // 0x0e
    unused!(),                      // 0x0f
    unused!(),                      // 0x10
    unused!(),                      // 0x11
    unused!(),                      // 0x12
    unused!(),                      // 0x13
    unused!(),                      // 0x14
    unused!(),                      // 0x15
    unused!(),                      // 0x16
    unused!(),                      // 0x17
    unused!(),                      // 0x18
    unused!(),                      // 0x19
    unused!(),                      // 0x1a
    unused!(),                      // 0x1b
    unused!(),                      // 0x1c
    unused!(),                      // 0x1d
    unused!(),                      // 0x1e
    unused!(),                      // 0x1f
    unused!(),                      // 0x20
    unused!(),                      // 0x21
    unused!(),                      // 0x22
    unused!(),                      // 0x23
    unused!(),                      // 0x24
    unused!(),                      // 0x25
    unused!(),                      // 0x26
    unused!(),                      // 0x27
    unused!(),                      // 0x28
    unused!(),                      // 0x29
    unused!(),                      // 0x2a
    unused!(),                      // 0x2b
    unused!(),                      // 0x2c
    unused!(),                      // 0x2d
    unused!(),                      // 0x2e
    unused!(),                      // 0x2f
    unused!(),                      // 0x30
    unused!(),                      // 0x31
    unused!(),                      // 0x32
    unused!(),                      // 0x33
    unused!(),                      // 0x34
    unused!(),                      // 0x35
    unused!(),                      // 0x36
    unused!(),                      // 0x37
    unused!(),                      // 0x38
    unused!(),                      // 0x39
    unused!(),                      // 0x3a
    unused!(),                      // 0x3b
    unused!(),                      // 0x3c
    unused!(),                      // 0x3d
    unused!(),                      // 0x3e
    unused!(),                      // 0x3f
    unused!(),                      // 0x40
    unused!(),                      // 0x41
    unused!(),                      // 0x42
    unused!(),                      // 0x43
    unused!(),                      // 0x44
    unused!(),                      // 0x45
    unused!(),                      // 0x46
    unused!(),                      // 0x47
    unused!(),                      // 0x48
    unused!(),                      // 0x49
    unused!(),                      // 0x4a
    unused!(),                      // 0x4b
    unused!(),                      // 0x4c
    unused!(),                      // 0x4d
    unused!(),                      // 0x4e
    unused!(),                      // 0x4f
    unused!(),                      // 0x50
    unused!(),                      // 0x51
    unused!(),                      // 0x52
    unused!(),                      // 0x53
    unused!(),                      // 0x54
    unused!(),                      // 0x55
    unused!(),                      // 0x56
    unused!(),                      // 0x57
    unused!(),                      // 0x58
    unused!(),                      // 0x59
    unused!(),                      // 0x5a
    unused!(),                      // 0x5b
    unused!(),                      // 0x5c
    unused!(),                      // 0x5d
    unused!(),                      // 0x5e
    unused!(),                      // 0x5f
    unused!(),                      // 0x60
    unused!(),                      // 0x61
    unused!(),                      // 0x62
    unused!(),                      // 0x63
    unused!(),                      // 0x64
    unused!(),                      // 0x65
    unused!(),                      // 0x66
    unused!(),                      // 0x67
    unused!(),                      // 0x68
    unused!(),                      // 0x69
    unused!(),                      // 0x6a
    unused!(),                      // 0x6b
    unused!(),                      // 0x6c
    unused!(),                      // 0x6d
    unused!(),                      // 0x6e
    unused!(),                      // 0x6f
    unused!(),                      // 0x70
    unused!(),                      // 0x71
    unused!(),                      // 0x72
    unused!(),                      // 0x73
    unused!(),                      // 0x74
    unused!(),                      // 0x75
    unused!(),                      // 0x76
    unused!(),                      // 0x77
    unused!(),                      // 0x78
    unused!(),                      // 0x79
    unused!(),                      // 0x7a
    unused!(),                      // 0x7b
    unused!(),                      // 0x7c
    unused!(),                      // 0x7d
    unused!(),                      // 0x7e
    unused!(),                      // 0x7f
    unused!(),                      // 0x80
    unused!(),                      // 0x81
    unused!(),                      // 0x82
    unused!(),                      // 0x83
    unused!(),                      // 0x84
    unused!(),                      // 0x85
    unused!(),                      // 0x86
    unused!(),                      // 0x87
    unused!(),                      // 0x88
    unused!(),                      // 0x89
    unused!(),                      // 0x8a
    unused!(),                      // 0x8b
    unused!(),                      // 0x8c
    unused!(),                      // 0x8d
    unused!(),                      // 0x8e
    unused!(),                      // 0x8f
    unused!(),                      // 0x90
    unused!(),                      // 0x91
    unused!(),                      // 0x92
    unused!(),                      // 0x93
    unused!(),                      // 0x94
    unused!(),                      // 0x95
    unused!(),                      // 0x96
    unused!(),                      // 0x97
    unused!(),                      // 0x98
    unused!(),                      // 0x99
    unused!(),                      // 0x9a
    unused!(),                      // 0x9b
    unused!(),                      // 0x9c
    unused!(),                      // 0x9d
    unused!(),                      // 0x9e
    unused!(),                      // 0x9f
    unused!(),                      // 0xa0
    unused!(),                      // 0xa1
    unused!(),                      // 0xa2
    unused!(),                      // 0xa3
    unused!(),                      // 0xa4
    unused!(),                      // 0xa5
    unused!(),                      // 0xa6
    unused!(),                      // 0xa7
    unused!(),                      // 0xa8
    unused!(),                      // 0xa9
    unused!(),                      // 0xaa
    unused!(),                      // 0xab
    unused!(),                      // 0xac
    unused!(),                      // 0xad
    unused!(),                      // 0xae
    unused!(),                      // 0xaf
    unused!(),                      // 0xb0
    unused!(),                      // 0xb1
    unused!(),                      // 0xb2
    unused!(),                      // 0xb3
    unused!(),                      // 0xb4
    unused!(),                      // 0xb5
    unused!(),                      // 0xb6
    unused!(),                      // 0xb7
    unused!(),                      // 0xb8
    unused!(),                      // 0xb9
    unused!(),                      // 0xba
    unused!(),                      // 0xbb
    unused!(),                      // 0xbc
    unused!(),                      // 0xbd
    unused!(),                      // 0xbe
    unused!(),                      // 0xbf
    unused!(),                      // 0xc0
    unused!(),                      // 0xc1
    unused!(),                      // 0xc2
    unused!(),                      // 0xc3
    unused!(),                      // 0xc4
    unused!(),                      // 0xc5
    unused!(),                      // 0xc6
    unused!(),                      // 0xc7
    unused!(),                      // 0xc8
    unused!(),                      // 0xc9
    unused!(),                      // 0xca
    unused!(),                      // 0xcb
    unused!(),                      // 0xcc
    unused!(),                      // 0xcd
    unused!(),                      // 0xce
    unused!(),                      // 0xcf
    unused!(),                      // 0xd0
    unused!(),                      // 0xd1
    unused!(),                      // 0xd2
    unused!(),                      // 0xd3
    unused!(),                      // 0xd4
    unused!(),                      // 0xd5
    unused!(),                      // 0xd6
    unused!(),                      // 0xd7
    unused!(),                      // 0xd8
    unused!(),                      // 0xd9
    unused!(),                      // 0xda
    unused!(),                      // 0xdb
    unused!(),                      // 0xdc
    unused!(),                      // 0xdd
    unused!(),                      // 0xde
    unused!(),                      // 0xdf
    unused!(),                      // 0xe0
    unused!(),                      // 0xe1
    unused!(),                      // 0xe2
    unused!(),                      // 0xe3
    unused!(),                      // 0xe4
    unused!(),                      // 0xe5
    unused!(),                      // 0xe6
    unused!(),                      // 0xe7
    unused!(),                      // 0xe8
    unused!(),                      // 0xe9
    unused!(),                      // 0xea
    unused!(),                      // 0xeb
    unused!(),                      // 0xec
    unused!(),                      // 0xed
    unused!(),                      // 0xee
    unused!(),                      // 0xef
    unused!(),                      // 0xf0
    unused!(),                      // 0xf1
    unused!(),                      // 0xf2
    unused!(),                      // 0xf3
    unused!(),                      // 0xf4
    unused!(),                      // 0xf5
    unused!(),                      // 0xf6
    unused!(),                      // 0xf7
    unused!(),                      // 0xf8
    unused!(),                      // 0xf9
    unused!(),                      // 0xfa
    unused!(),                      // 0xfb
    unused!(),                      // 0xfc
    unused!(),                      // 0xfd
    unused!(),                      // 0xfe
    unused!(),                      // 0xff
];

pub static XOP9_TABLE: [Opcode; 256] = [
    unused!(),                      // 0x00
    unused!(),                      // 0x01
    unused!(),                      // 0x02
    unused!(),                      // 0x03
    unused!(),                      // 0x04
    unused!(),                      // 0x05
    unused!(),                      // 0x06
    unused!(),                      // 0x07
    unused!(),                      // 0x08
    unused!(),                      // 0x09
    unused!(),                      // 0x0a
    unused!(),                      // 0x0b
    unused!(),                      // 0x0c
    unused!(),                      // 0x0d
    unused!(),                      // 0x0e
    unused!(),                      // 0x0f
    unused!(),                      // 0x10
    unused!(),                      // 0x11
    unused!(),                      // 0x12
    unused!(),                      // 0x13
    unused!(),                      // 0x14
    unused!(),                      // 0x15
    unused!(),                      // 0x16
    unused!(),                      // 0x17
    unused!(),                      // 0x18
    unused!(),                      // 0x19
    unused!(),                      // 0x1a
    unused!(),                      // 0x1b
    unused!(),                      // 0x1c
    unused!(),                      // 0x1d
    unused!(),                      // 0x1e
    unused!(),                      // 0x1f
    unused!(),                      // 0x20
    unused!(),                      // 0x21
    unused!(),                      // 0x22
    unused!(),                      // 0x23
    unused!(),                      // 0x24
    unused!(),                      // 0x25
    unused!(),                      // 0x26
    unused!(),                      // 0x27
    unused!(),                      // 0x28
    unused!(),                      // 0x29
    unused!(),                      // 0x2a
    unused!(),                      // 0x2b
    unused!(),                      // 0x2c
    unused!(),                      // 0x2d
    unused!(),                      // 0x2e
    unused!(),                      // 0x2f
    unused!(),                      // 0x30
    unused!(),                      // 0x31
    unused!(),                      // 0x32
    unused!(),                      // 0x33
    unused!(),                      // 0x34
    unused!(),                      // 0x35
    unused!(),                      // 0x36
    unused!(),                      // 0x37
    unused!(),                      // 0x38
    unused!(),                      // 0x39
    unused!(),                      // 0x3a
    unused!(),                      // 0x3b
    unused!(),                      // 0x3c
    unused!(),                      // 0x3d
    unused!(),                      // 0x3e
    unused!(),                      // 0x3f
    unused!(),                      // 0x40
    unused!(),                      // 0x41
    unused!(),                      // 0x42
    unused!(),                      // 0x43
    unused!(),                      // 0x44
    unused!(),                      // 0x45
    unused!(),                      // 0x46
    unused!(),                      // 0x47
    unused!(),                      // 0x48
    unused!(),                      // 0x49
    unused!(),                      // 0x4a
    unused!(),                      // 0x4b
    unused!(),                      // 0x4c
    unused!(),                      // 0x4d
    unused!(),                      // 0x4e
    unused!(),                      // 0x4f
    unused!(),                      // 0x50
    unused!(),                      // 0x51
    unused!(),                      // 0x52
    unused!(),                      // 0x53
    unused!(),                      // 0x54
    unused!(),                      // 0x55
    unused!(),                      // 0x56
    unused!(),                      // 0x57
    unused!(),                      // 0x58
    unused!(),                      // 0x59
    unused!(),                      // 0x5a
    unused!(),                      // 0x5b
    unused!(),                      // 0x5c
    unused!(),                      // 0x5d
    unused!(),                      // 0x5e
    unused!(),                      // 0x5f
    unused!(),                      // 0x60
    unused!(),                      // 0x61
    unused!(),                      // 0x62
    unused!(),                      // 0x63
    unused!(),                      // 0x64
    unused!(),                      // 0x65
    unused!(),                      // 0x66
    unused!(),                      // 0x67
    unused!(),                      // 0x68
    unused!(),                      // 0x69
    unused!(),                      // 0x6a
    unused!(),                      // 0x6b
    unused!(),                      // 0x6c
    unused!(),                      // 0x6d
    unused!(),                      // 0x6e
    unused!(),                      // 0x6f
    unused!(),                      // 0x70
    unused!(),                      // 0x71
    unused!(),                      // 0x72
    unused!(),                      // 0x73
    unused!(),                      // 0x74
    unused!(),                      // 0x75
    unused!(),                      // 0x76
    unused!(),                      // 0x77
    unused!(),                      // 0x78
    unused!(),                      // 0x79
    unused!(),                      // 0x7a
    unused!(),                      // 0x7b
    unused!(),                      // 0x7c
    unused!(),                      // 0x7d
    unused!(),                      // 0x7e
    unused!(),                      // 0x7f
    unused!(),                      // 0x80
    unused!(),                      // 0x81
    unused!(),                      // 0x82
    unused!(),                      // 0x83
    unused!(),                      // 0x84
    unused!(),                      // 0x85
    unused!(),                      // 0x86
    unused!(),                      // 0x87
    unused!(),                      // 0x88
    unused!(),                      // 0x89
    unused!(),                      // 0x8a
    unused!(),                      // 0x8b
    unused!(),                      // 0x8c
    unused!(),                      // 0x8d
    unused!(),                      // 0x8e
    unused!(),                      // 0x8f
    unused!(),                      // 0x90
    unused!(),                      // 0x91
    unused!(),                      // 0x92
    unused!(),                      // 0x93
    unused!(),                      // 0x94
    unused!(),                      // 0x95
    unused!(),                      // 0x96
    unused!(),                      // 0x97
    unused!(),                      // 0x98
    unused!(),                      // 0x99
    unused!(),                      // 0x9a
    unused!(),                      // 0x9b
    unused!(),                      // 0x9c
    unused!(),                      // 0x9d
    unused!(),                      // 0x9e
    unused!(),                      // 0x9f
    unused!(),                      // 0xa0
    unused!(),                      // 0xa1
    unused!(),                      // 0xa2
    unused!(),                      // 0xa3
    unused!(),                      // 0xa4
    unused!(),                      // 0xa5
    unused!(),                      // 0xa6
    unused!(),                      // 0xa7
    unused!(),                      // 0xa8
    unused!(),                      // 0xa9
    unused!(),                      // 0xaa
    unused!(),                      // 0xab
    unused!(),                      // 0xac
    unused!(),                      // 0xad
    unused!(),                      // 0xae
    unused!(),                      // 0xaf
    unused!(),                      // 0xb0
    unused!(),                      // 0xb1
    unused!(),                      // 0xb2
    unused!(),                      // 0xb3
    unused!(),                      // 0xb4
    unused!(),                      // 0xb5
    unused!(),                      // 0xb6
    unused!(),                      // 0xb7
    unused!(),                      // 0xb8
    unused!(),                      // 0xb9
    unused!(),                      // 0xba
    unused!(),                      // 0xbb
    unused!(),                      // 0xbc
    unused!(),                      // 0xbd
    unused!(),                      // 0xbe
    unused!(),                      // 0xbf
    unused!(),                      // 0xc0
    unused!(),                      // 0xc1
    unused!(),                      // 0xc2
    unused!(),                      // 0xc3
    unused!(),                      // 0xc4
    unused!(),                      // 0xc5
    unused!(),                      // 0xc6
    unused!(),                      // 0xc7
    unused!(),                      // 0xc8
    unused!(),                      // 0xc9
    unused!(),                      // 0xca
    unused!(),                      // 0xcb
    unused!(),                      // 0xcc
    unused!(),                      // 0xcd
    unused!(),                      // 0xce
    unused!(),                      // 0xcf
    unused!(),                      // 0xd0
    unused!(),                      // 0xd1
    unused!(),                      // 0xd2
    unused!(),                      // 0xd3
    unused!(),                      // 0xd4
    unused!(),                      // 0xd5
    unused!(),                      // 0xd6
    unused!(),                      // 0xd7
    unused!(),                      // 0xd8
    unused!(),                      // 0xd9
    unused!(),                      // 0xda
    unused!(),                      // 0xdb
    unused!(),                      // 0xdc
    unused!(),                      // 0xdd
    unused!(),                      // 0xde
    unused!(),                      // 0xdf
    unused!(),                      // 0xe0
    unused!(),                      // 0xe1
    unused!(),                      // 0xe2
    unused!(),                      // 0xe3
    unused!(),                      // 0xe4
    unused!(),                      // 0xe5
    unused!(),                      // 0xe6
    unused!(),                      // 0xe7
    unused!(),                      // 0xe8
    unused!(),                      // 0xe9
    unused!(),                      // 0xea
    unused!(),                      // 0xeb
    unused!(),                      // 0xec
    unused!(),                      // 0xed
    unused!(),                      // 0xee
    unused!(),                      // 0xef
    unused!(),                      // 0xf0
    unused!(),                      // 0xf1
    unused!(),                      // 0xf2
    unused!(),                      // 0xf3
    unused!(),                      // 0xf4
    unused!(),                      // 0xf5
    unused!(),                      // 0xf6
    unused!(),                      // 0xf7
    unused!(),                      // 0xf8
    unused!(),                      // 0xf9
    unused!(),                      // 0xfa
    unused!(),                      // 0xfb
    unused!(),                      // 0xfc
    unused!(),                      // 0xfd
    unused!(),                      // 0xfe
    unused!(),                      // 0xff
];

pub static XOPA_TABLE: [Opcode; 256] = [
    unused!(),                      // 0x00
    unused!(),                      // 0x01
    unused!(),                      // 0x02
    unused!(),                      // 0x03
    unused!(),                      // 0x04
    unused!(),                      // 0x05
    unused!(),                      // 0x06
    unused!(),                      // 0x07
    unused!(),                      // 0x08
    unused!(),                      // 0x09
    unused!(),                      // 0x0a
    unused!(),                      // 0x0b
    unused!(),                      // 0x0c
    unused!(),                      // 0x0d
    unused!(),                      // 0x0e
    unused!(),                      // 0x0f
    unused!(),                      // 0x10
    unused!(),                      // 0x11
    unused!(),                      // 0x12
    unused!(),                      // 0x13
    unused!(),                      // 0x14
    unused!(),                      // 0x15
    unused!(),                      // 0x16
    unused!(),                      // 0x17
    unused!(),                      // 0x18
    unused!(),                      // 0x19
    unused!(),                      // 0x1a
    unused!(),                      // 0x1b
    unused!(),                      // 0x1c
    unused!(),                      // 0x1d
    unused!(),                      // 0x1e
    unused!(),                      // 0x1f
    unused!(),                      // 0x20
    unused!(),                      // 0x21
    unused!(),                      // 0x22
    unused!(),                      // 0x23
    unused!(),                      // 0x24
    unused!(),                      // 0x25
    unused!(),                      // 0x26
    unused!(),                      // 0x27
    unused!(),                      // 0x28
    unused!(),                      // 0x29
    unused!(),                      // 0x2a
    unused!(),                      // 0x2b
    unused!(),                      // 0x2c
    unused!(),                      // 0x2d
    unused!(),                      // 0x2e
    unused!(),                      // 0x2f
    unused!(),                      // 0x30
    unused!(),                      // 0x31
    unused!(),                      // 0x32
    unused!(),                      // 0x33
    unused!(),                      // 0x34
    unused!(),                      // 0x35
    unused!(),                      // 0x36
    unused!(),                      // 0x37
    unused!(),                      // 0x38
    unused!(),                      // 0x39
    unused!(),                      // 0x3a
    unused!(),                      // 0x3b
    unused!(),                      // 0x3c
    unused!(),                      // 0x3d
    unused!(),                      // 0x3e
    unused!(),                      // 0x3f
    unused!(),                      // 0x40
    unused!(),                      // 0x41
    unused!(),                      // 0x42
    unused!(),                      // 0x43
    unused!(),                      // 0x44
    unused!(),                      // 0x45
    unused!(),                      // 0x46
    unused!(),                      // 0x47
    unused!(),                      // 0x48
    unused!(),                      // 0x49
    unused!(),                      // 0x4a
    unused!(),                      // 0x4b
    unused!(),                      // 0x4c
    unused!(),                      // 0x4d
    unused!(),                      // 0x4e
    unused!(),                      // 0x4f
    unused!(),                      // 0x50
    unused!(),                      // 0x51
    unused!(),                      // 0x52
    unused!(),                      // 0x53
    unused!(),                      // 0x54
    unused!(),                      // 0x55
    unused!(),                      // 0x56
    unused!(),                      // 0x57
    unused!(),                      // 0x58
    unused!(),                      // 0x59
    unused!(),                      // 0x5a
    unused!(),                      // 0x5b
    unused!(),                      // 0x5c
    unused!(),                      // 0x5d
    unused!(),                      // 0x5e
    unused!(),                      // 0x5f
    unused!(),                      // 0x60
    unused!(),                      // 0x61
    unused!(),                      // 0x62
    unused!(),                      // 0x63
    unused!(),                      // 0x64
    unused!(),                      // 0x65
    unused!(),                      // 0x66
    unused!(),                      // 0x67
    unused!(),                      // 0x68
    unused!(),                      // 0x69
    unused!(),                      // 0x6a
    unused!(),                      // 0x6b
    unused!(),                      // 0x6c
    unused!(),                      // 0x6d
    unused!(),                      // 0x6e
    unused!(),                      // 0x6f
    unused!(),                      // 0x70
    unused!(),                      // 0x71
    unused!(),                      // 0x72
    unused!(),                      // 0x73
    unused!(),                      // 0x74
    unused!(),                      // 0x75
    unused!(),                      // 0x76
    unused!(),                      // 0x77
    unused!(),                      // 0x78
    unused!(),                      // 0x79
    unused!(),                      // 0x7a
    unused!(),                      // 0x7b
    unused!(),                      // 0x7c
    unused!(),                      // 0x7d
    unused!(),                      // 0x7e
    unused!(),                      // 0x7f
    unused!(),                      // 0x80
    unused!(),                      // 0x81
    unused!(),                      // 0x82
    unused!(),                      // 0x83
    unused!(),                      // 0x84
    unused!(),                      // 0x85
    unused!(),                      // 0x86
    unused!(),                      // 0x87
    unused!(),                      // 0x88
    unused!(),                      // 0x89
    unused!(),                      // 0x8a
    unused!(),                      // 0x8b
    unused!(),                      // 0x8c
    unused!(),                      // 0x8d
    unused!(),                      // 0x8e
    unused!(),                      // 0x8f
    unused!(),                      // 0x90
    unused!(),                      // 0x91
    unused!(),                      // 0x92
    unused!(),                      // 0x93
    unused!(),                      // 0x94
    unused!(),                      // 0x95
    unused!(),                      // 0x96
    unused!(),                      // 0x97
    unused!(),                      // 0x98
    unused!(),                      // 0x99
    unused!(),                      // 0x9a
    unused!(),                      // 0x9b
    unused!(),                      // 0x9c
    unused!(),                      // 0x9d
    unused!(),                      // 0x9e
    unused!(),                      // 0x9f
    unused!(),                      // 0xa0
    unused!(),                      // 0xa1
    unused!(),                      // 0xa2
    unused!(),                      // 0xa3
    unused!(),                      // 0xa4
    unused!(),                      // 0xa5
    unused!(),                      // 0xa6
    unused!(),                      // 0xa7
    unused!(),                      // 0xa8
    unused!(),                      // 0xa9
    unused!(),                      // 0xaa
    unused!(),                      // 0xab
    unused!(),                      // 0xac
    unused!(),                      // 0xad
    unused!(),                      // 0xae
    unused!(),                      // 0xaf
    unused!(),                      // 0xb0
    unused!(),                      // 0xb1
    unused!(),                      // 0xb2
    unused!(),                      // 0xb3
    unused!(),                      // 0xb4
    unused!(),                      // 0xb5
    unused!(),                      // 0xb6
    unused!(),                      // 0xb7
    unused!(),                      // 0xb8
    unused!(),                      // 0xb9
    unused!(),                      // 0xba
    unused!(),                      // 0xbb
    unused!(),                      // 0xbc
    unused!(),                      // 0xbd
    unused!(),                      // 0xbe
    unused!(),                      // 0xbf
    unused!(),                      // 0xc0
    unused!(),                      // 0xc1
    unused!(),                      // 0xc2
    unused!(),                      // 0xc3
    unused!(),                      // 0xc4
    unused!(),                      // 0xc5
    unused!(),                      // 0xc6
    unused!(),                      // 0xc7
    unused!(),                      // 0xc8
    unused!(),                      // 0xc9
    unused!(),                      // 0xca
    unused!(),                      // 0xcb
    unused!(),                      // 0xcc
    unused!(),                      // 0xcd
    unused!(),                      // 0xce
    unused!(),                      // 0xcf
    unused!(),                      // 0xd0
    unused!(),                      // 0xd1
    unused!(),                      // 0xd2
    unused!(),                      // 0xd3
    unused!(),                      // 0xd4
    unused!(),                      // 0xd5
    unused!(),                      // 0xd6
    unused!(),                      // 0xd7
    unused!(),                      // 0xd8
    unused!(),                      // 0xd9
    unused!(),                      // 0xda
    unused!(),                      // 0xdb
    unused!(),                      // 0xdc
    unused!(),                      // 0xdd
    unused!(),                      // 0xde
    unused!(),                      // 0xdf
    unused!(),                      // 0xe0
    unused!(),                      // 0xe1
    unused!(),                      // 0xe2
    unused!(),                      // 0xe3
    unused!(),                      // 0xe4
    unused!(),                      // 0xe5
    unused!(),                      // 0xe6
    unused!(),                      // 0xe7
    unused!(),                      // 0xe8
    unused!(),                      // 0xe9
    unused!(),                      // 0xea
    unused!(),                      // 0xeb
    unused!(),                      // 0xec
    unused!(),                      // 0xed
    unused!(),                      // 0xee
    unused!(),                      // 0xef
    unused!(),                      // 0xf0
    unused!(),                      // 0xf1
    unused!(),                      // 0xf2
    unused!(),                      // 0xf3
    unused!(),                      // 0xf4
    unused!(),                      // 0xf5
    unused!(),                      // 0xf6
    unused!(),                      // 0xf7
    unused!(),                      // 0xf8
    unused!(),                      // 0xf9
    unused!(),                      // 0xfa
    unused!(),                      // 0xfb
    unused!(),                      // 0xfc
    unused!(),                      // 0xfd
    unused!(),                      // 0xfe
    unused!(),                      // 0xff
];

pub static GROUP1_OPC80: [Opcode; 8] = [
    opcode!("add"; E/b, I/b),       // 0x00: add
    opcode!("or"; E/b, I/b),        // 0x01: or
    opcode!("adc"; E/b, I/b),       // 0x02: adc
    opcode!("sbb"; E/b, I/b),       // 0x03: sbb
    opcode!("and"; E/b, I/b),       // 0x04: and
    opcode!("sub"; E/b, I/b),       // 0x05: sub
    opcode!("xor"; E/b, I/b),       // 0x06: xor
    opcode!("cmp"; E/b, I/b),       // 0x07: cmp
];

pub static GROUP1_OPC81: [Opcode; 8] = [
    opcode!("add"; E/v, I/z),       // 0x00: add
    opcode!("or"; E/v, I/z),        // 0x01: or
    opcode!("adc"; E/v, I/z),       // 0x02: adc
    opcode!("sbb"; E/v, I/z),       // 0x03: sbb
    opcode!("and"; E/v, I/z),       // 0x04: and
    opcode!("sub"; E/v, I/z),       // 0x05: sub
    opcode!("xor"; E/v, I/z),       // 0x06: xor
    opcode!("cmp"; E/v, I/z),       // 0x07: cmp
];

pub static GROUP1_OPC82: [Opcode; 8] = [
    opcode!("add"; E/b, I/b),       // 0x00: add
    opcode!("or"; E/b, I/b),        // 0x01: or
    opcode!("adc"; E/b, I/b),       // 0x02: adc
    opcode!("sbb"; E/b, I/b),       // 0x03: sbb
    opcode!("and"; E/b, I/b),       // 0x04: and
    opcode!("sub"; E/b, I/b),       // 0x05: sub
    opcode!("xor"; E/b, I/b),       // 0x06: xor
    opcode!("cmp"; E/b, I/b),       // 0x07: cmp
];
pub static GROUP1_OPC83: [Opcode; 8] = [
    opcode!("add"; E/v, I/b),       // 0x00: add
    opcode!("or"; E/v, I/b),        // 0x01: or
    opcode!("adc"; E/v, I/b),       // 0x02: adc
    opcode!("sbb"; E/v, I/b),       // 0x03: sbb
    opcode!("and"; E/v, I/b),       // 0x04: and
    opcode!("sub"; E/v, I/b),       // 0x05: sub
    opcode!("xor"; E/v, I/b),       // 0x06: xor
    opcode!("cmp"; E/v, I/b),       // 0x07: cmp
];

// GROUP1A_OPC8F
pub static GROUP101_OPC8F: [Opcode; 8] = [
    opcode!("pop"; E/v),       // 0x00: pop
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
];

pub static GROUP2_OPCC0: [Opcode; 8] = [
    opcode!("rol"; E/b, I/b),   // 0x00: rol
    opcode!("ror"; E/b, I/b),   // 0x01: ror
    opcode!("rcl"; E/b, I/b),   // 0x02: rcl
    opcode!("rcr"; E/b, I/b),   // 0x03: rcr
    opcode!("shl"; E/b, I/b),   // 0x04: shl
    opcode!("shr"; E/b, I/b),   // 0x05: shr
    opcode!("shl"; E/b, I/b),   // 0x06: shl
    opcode!("sar"; E/b, I/b),   // 0x07: sar
];

pub static GROUP2_OPCC1: [Opcode; 8] = [
    opcode!("rol"; E/v, I/b),   // 0x00: rol
    opcode!("ror"; E/v, I/b),   // 0x01: ror
    opcode!("rcl"; E/v, I/b),   // 0x02: rcl
    opcode!("rcr"; E/v, I/b),   // 0x03: rcr
    opcode!("shl"; E/v, I/b),   // 0x04: shl
    opcode!("shr"; E/v, I/b),   // 0x05: shr
    opcode!("shl"; E/v, I/b),   // 0x06: shl
    opcode!("sar"; E/v, I/b),   // 0x07: sar
];

pub static GROUP2_OPCD0: [Opcode; 8] = [
    opcode!("rol"; E/b, I/one),    // 0x00: rol
    opcode!("ror"; E/b, I/one),    // 0x01: ror
    opcode!("rcl"; E/b, I/one),    // 0x02: rcl
    opcode!("rcr"; E/b, I/one),    // 0x03: rcr
    opcode!("shl"; E/b, I/one),    // 0x04: shl
    opcode!("shr"; E/b, I/one),    // 0x05: shr
    opcode!("shl"; E/b, I/one),    // 0x06: shl
    opcode!("sar"; E/b, I/one),    // 0x07: sar
];

pub static GROUP2_OPCD1: [Opcode; 8] = [
    opcode!("rol"; E/v, I/one),    // 0x00: rol
    opcode!("ror"; E/v, I/one),    // 0x01: ror
    opcode!("rcl"; E/v, I/one),    // 0x02: rcl
    opcode!("rcr"; E/v, I/one),    // 0x03: rcr
    opcode!("shl"; E/v, I/one),    // 0x04: shl
    opcode!("shr"; E/v, I/one),    // 0x05: shr
    opcode!("shl"; E/v, I/one),    // 0x06: shl
    opcode!("sar"; E/v, I/one),    // 0x07: sar
];

pub static GROUP2_OPCD2: [Opcode; 8] = [
    opcode!("rol"; E/b, CL),    // 0x00: rol
    opcode!("ror"; E/b, CL),    // 0x01: ror
    opcode!("rcl"; E/b, CL),    // 0x02: rcl
    opcode!("rcr"; E/b, CL),    // 0x03: rcr
    opcode!("shl"; E/b, CL),    // 0x04: shl
    opcode!("shr"; E/b, CL),    // 0x05: shr
    opcode!("shl"; E/b, CL),    // 0x06: shl
    opcode!("sar"; E/b, CL),    // 0x07: sar
];

pub static GROUP2_OPCD3: [Opcode; 8] = [
    opcode!("rol"; E/v, CL),    // 0x00: rol
    opcode!("ror"; E/v, CL),    // 0x01: ror
    opcode!("rcl"; E/v, CL),    // 0x02: rcl
    opcode!("rcr"; E/v, CL),    // 0x03: rcr
    opcode!("shl"; E/v, CL),    // 0x04: shl
    opcode!("shr"; E/v, CL),    // 0x05: shr
    opcode!("shl"; E/v, CL),    // 0x06: shl
    opcode!("sar"; E/v, CL),    // 0x07: sar
];

pub static GROUP3_OPCF6: [Opcode; 8] = [
    opcode!("test"; E/b, I/b),  // 0x00: test
    opcode!("test"; E/b, I/b),  // 0x01: test
    opcode!("not"; E/b),        // 0x02: not
    opcode!("neg"; E/b),        // 0x03: neg
    opcode!("mul"; E/b),        // 0x04: mul
    opcode!("imul"; E/b),       // 0x05: imul
    opcode!("div"; E/b),        // 0x06: div
    opcode!("idiv"; E/b),       // 0x07: idiv
];

pub static GROUP3_OPCF7: [Opcode; 8] = [
    opcode!("test"; E/v, I/z),  // 0x00: test
    opcode!("test"; E/v, I/z),  // 0x01: test
    opcode!("not"; E/v),        // 0x02: not
    opcode!("neg"; E/v),        // 0x03: neg
    opcode!("mul"; E/v),        // 0x04: mul
    opcode!("imul"; E/v),       // 0x05: imul
    opcode!("div"; E/v),        // 0x06: div
    opcode!("idiv"; E/v),       // 0x07: idiv
];

pub static GROUP4_OPCFE: [Opcode; 8] = [
    opcode!("inc"; E/b),        // 0x00: inc
    opcode!("dec"; E/b),        // 0x01: dec
    unused!(),                  // 0x02
    unused!(),                  // 0x03
    unused!(),                  // 0x04
    unused!(),                  // 0x05
    unused!(),                  // 0x06
    unused!(),                  // 0x07
];

pub static GROUP5_OPCFF: [Opcode; 8] = [
    opcode!("inc"; E/v),        // 0x00: inc
    opcode!("dec"; E/v),        // 0x01: dec
    opcode!("call"; E/v),       // 0x02: call
    opcode!("call"; M/p),       // 0x03: call
    opcode!("jmp"; E/v),        // 0x04: jmp
    opcode!("jmp"; M/p),        // 0x05: jmp
    opcode!("push"; E/v),       // 0x06: push
    unused!(),                  // 0x07
];

pub static GROUP6_OPC00: [Opcode; 8] = [
    opcode!("sldt"; M/w),      // 0x00: sldt
    opcode!("str"; M/w),       // 0x01: str
    opcode!("lldt"; E/w),       // 0x02: lldt
    opcode!("ltr"; E/w),        // 0x03: ltr
    opcode!("verr"; E/w),       // 0x04: verr
    opcode!("verw"; E/w),       // 0x05: verw
    unused!(),                  // 0x06
    unused!(),                  // 0x07
];

pub static GROUP7_OPC01_MEM: [Opcode; 8] = [
    opcode!("sgdt"; M/s),         // 0x00: sgdt
    opcode!("sidt"; M/s),       // 0x01: monitor
    opcode!("lgdt"; M/s),         // 0x02: lgdt
    opcode!("lidt"; M/s),          // 0x03: clgi
    opcode!("smsw"; M/w),      // 0x04: smsw
    unused!(),                  // 0x05
    opcode!("lmsw"; E/w),       // 0x06: lmsw
    opcode!("invlpg"; M/b),       // 0x07: invlpg
];

pub static GROUP8_OPCBA: [Opcode; 8] = [
    unused!(),                  // 0x00
    unused!(),                  // 0x01
    unused!(),                  // 0x02
    unused!(),                  // 0x03
    opcode!("bt"; E/v, I/b),    // 0x04: bt
    opcode!("bts"; E/v, I/b),   // 0x05: bts
    opcode!("btr"; E/v, I/b),   // 0x06: btr
    opcode!("btc"; E/v, I/b),   // 0x07: btc
];

// GROUP9_C7

pub static GROUP10_OPCB9: [Opcode; 8] = [
    opcode!("ud1"; ),                  // 0x00
    opcode!("ud1"; ),                  // 0x01
    opcode!("ud1"; ),                  // 0x02
    opcode!("ud1"; ),                  // 0x03
    opcode!("ud1"; ),                  // 0x04
    opcode!("ud1"; ),                  // 0x05
    opcode!("ud1"; ),                  // 0x06
    opcode!("ud1"; ),                  // 0x07
];

pub static GROUP11_OPCC6: [Opcode; 8] = [
    opcode!("mov"; E/b, I/b),   // 0x00: mov
    unused!(),                  // 0x01
    unused!(),                  // 0x02
    unused!(),                  // 0x03
    unused!(),                  // 0x04
    unused!(),                  // 0x05
    unused!(),                  // 0x06
    unused!(),                  // 0x07
];

pub static GROUP11_OPCC7: [Opcode; 8] = [
    opcode!("mov"; E/v, I/z),   // 0x00: mov
    unused!(),                  // 0x01
    unused!(),                  // 0x02
    unused!(),                  // 0x03
    unused!(),                  // 0x04
    unused!(),                  // 0x05
    unused!(),                  // 0x06
    unused!(),                  // 0x07
];


pub static GROUP12_OPC71: [Opcode; 8] = [
    unused!(),                  // 0x00
    unused!(),                  // 0x01
    opcode!("psrlw"; N/q, I/b),   // 0x02: psrlw
    unused!(),                  // 0x03
    opcode!("psraw"; N/q, I/b),   // 0x04: psraw
    unused!(),                  // 0x05
    opcode!("psllw"; N/q, I/b),   // 0x06: psllw
    unused!(),                  // 0x07
];

pub static GROUP12_OPC6671: [Opcode; 8] = [
    unused!(),                  // 0x00
    unused!(),                  // 0x01
    opcode!("psrlw"; H/x, U/x, I/b),// 0x02: psrlw
    unused!(),                  // 0x03
    opcode!("psraw"; H/x, U/x, I/b),// 0x04: psraw
    unused!(),                  // 0x05
    opcode!("psllw"; H/x, U/x, I/b),   // 0x06: psllw
    unused!(),                  // 0x07
];

pub static GROUP13_OPC72: [Opcode; 8] = [
    unused!(),                  // 0x00
    unused!(),                  // 0x01
    opcode!("psrld"; N/q, I/b),   // 0x02: psrld
    unused!(),                  // 0x03
    opcode!("psrad"; N/q, I/b),   // 0x04: psrad
    unused!(),                  // 0x05
    opcode!("pslld"; N/q, I/b),   // 0x06: pslld
    unused!(),                  // 0x07
];

pub static GROUP13_OPC6672: [Opcode; 8] = [
    unused!(),                  // 0x00
    unused!(),                  // 0x01
    opcode!("psrld"; H/x, U/x, I/b),// 0x02: psrld
    unused!(),                  // 0x03
    opcode!("psrad"; H/x, U/x, I/b),// 0x04: psrad
    unused!(),                  // 0x05
    opcode!("pslld"; H/x, U/x, I/b),   // 0x06: pslld
    unused!(),                  // 0x07
];

pub static GROUP14_OPC73: [Opcode; 8] = [
    unused!(),                  // 0x00
    unused!(),                  // 0x01
    opcode!("psrlq"; N/q, I/b),   // 0x02: psrlq
    unused!(),                  // 0x03
    unused!(),                  // 0x04
    unused!(),                  // 0x05
    opcode!("psllq"; N/q, I/b),   // 0x06: psllq
    unused!(),                  // 0x07
];

pub static GROUP14_OPC6673: [Opcode; 8] = [
    unused!(),                  // 0x00
    unused!(),                  // 0x01
    opcode!("psrlq"; H/x, U/x, I/b),// 0x02: psrlq
    opcode!("psrldq"; H/x, U/x, I/b),// 0x03: psrldq
    unused!(),                  // 0x04
    unused!(),                  // 0x05
    opcode!("psllq"; H/x, U/x, I/b),   // 0x06: psllq
    opcode!("pslldq"; H/x, U/x, I/b),// 0x07: pslldq
];

// GROUP15_OPCAE

// GROUP16_OPC18

// GROUP16_OPCF3

// GROUPX
pub static GROUP102_OPC01: [Opcode; 8] = [
    opcode!("sgdt"; M/x),         // 0x00: sgdt
    opcode!("monitor"; ),       // 0x01: monitor
    opcode!("lgdt"; M/x),         // 0x02: lgdt
    opcode!("clgi"; ),          // 0x03: clgi
    opcode!("smsw"; M/w),      // 0x04: smsw
    unused!(),                  // 0x05
    opcode!("lmsw"; E/w),       // 0x06: lmsw
    opcode!("invlpg"; M/x),       // 0x07: invlpg
];

pub static X87_D8_TABLE: [Opcode; 64] = [
    opcode!("fadd"; ST0, ST0),
    opcode!("fadd"; ST0, ST1),
    opcode!("fadd"; ST0, ST2),
    opcode!("fadd"; ST0, ST3),
    opcode!("fadd"; ST0, ST4),
    opcode!("fadd"; ST0, ST5),
    opcode!("fadd"; ST0, ST6),
    opcode!("fadd"; ST0, ST7),
    opcode!("fmul"; ST0, ST0),
    opcode!("fmul"; ST0, ST1),
    opcode!("fmul"; ST0, ST2),
    opcode!("fmul"; ST0, ST3),
    opcode!("fmul"; ST0, ST4),
    opcode!("fmul"; ST0, ST5),
    opcode!("fmul"; ST0, ST6),
    opcode!("fmul"; ST0, ST7),
    opcode!("fcom"; ST0, ST0),
    opcode!("fcom"; ST0, ST1),
    opcode!("fcom"; ST0, ST2),
    opcode!("fcom"; ST0, ST3),
    opcode!("fcom"; ST0, ST4),
    opcode!("fcom"; ST0, ST5),
    opcode!("fcom"; ST0, ST6),
    opcode!("fcom"; ST0, ST7),
    opcode!("fcomp"; ST0, ST0),
    opcode!("fcomp"; ST0, ST1),
    opcode!("fcomp"; ST0, ST2),
    opcode!("fcomp"; ST0, ST3),
    opcode!("fcomp"; ST0, ST4),
    opcode!("fcomp"; ST0, ST5),
    opcode!("fcomp"; ST0, ST6),
    opcode!("fcomp"; ST0, ST7),
    opcode!("fsub"; ST0, ST0),
    opcode!("fsub"; ST0, ST1),
    opcode!("fsub"; ST0, ST2),
    opcode!("fsub"; ST0, ST3),
    opcode!("fsub"; ST0, ST4),
    opcode!("fsub"; ST0, ST5),
    opcode!("fsub"; ST0, ST6),
    opcode!("fsub"; ST0, ST7),
    opcode!("fsubr"; ST0, ST0),
    opcode!("fsubr"; ST0, ST1),
    opcode!("fsubr"; ST0, ST2),
    opcode!("fsubr"; ST0, ST3),
    opcode!("fsubr"; ST0, ST4),
    opcode!("fsubr"; ST0, ST5),
    opcode!("fsubr"; ST0, ST6),
    opcode!("fsubr"; ST0, ST7),
    opcode!("fdiv"; ST0, ST0),
    opcode!("fdiv"; ST0, ST1),
    opcode!("fdiv"; ST0, ST2),
    opcode!("fdiv"; ST0, ST3),
    opcode!("fdiv"; ST0, ST4),
    opcode!("fdiv"; ST0, ST5),
    opcode!("fdiv"; ST0, ST6),
    opcode!("fdiv"; ST0, ST7),
    opcode!("fdivr"; ST0, ST0),
    opcode!("fdivr"; ST0, ST1),
    opcode!("fdivr"; ST0, ST2),
    opcode!("fdivr"; ST0, ST3),
    opcode!("fdivr"; ST0, ST4),
    opcode!("fdivr"; ST0, ST5),
    opcode!("fdivr"; ST0, ST6),
    opcode!("fdivr"; ST0, ST7),
];

pub static X87_D9_TABLE: [Opcode; 64] = [
    opcode!("fld"; ST0, ST0),
    opcode!("fld"; ST0, ST1),
    opcode!("fld"; ST0, ST2),
    opcode!("fld"; ST0, ST3),
    opcode!("fld"; ST0, ST4),
    opcode!("fld"; ST0, ST5),
    opcode!("fld"; ST0, ST6),
    opcode!("fld"; ST0, ST7),
    opcode!("fxch"; ST0, ST0),
    opcode!("fxch"; ST0, ST1),
    opcode!("fxch"; ST0, ST2),
    opcode!("fxch"; ST0, ST3),
    opcode!("fxch"; ST0, ST4),
    opcode!("fxch"; ST0, ST5),
    opcode!("fxch"; ST0, ST6),
    opcode!("fxch"; ST0, ST7),
    opcode!("fnop"; ),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    opcode!("fchs"; ),
    opcode!("fabs"; ),
    unused!(),
    unused!(),
    opcode!("ftst"; ),
    opcode!("fxam"; ),
    unused!(),
    unused!(),
    opcode!("fld1"; ),
    opcode!("fldl2t"; ),
    opcode!("fldl2e"; ),
    opcode!("fldpi"; ),
    opcode!("fldl2g"; ),
    opcode!("fldln2"; ),
    opcode!("fldz"; ),
    unused!(),
    opcode!("f2xm1"; ),
    opcode!("fyl2x"; ),
    opcode!("fptan"; ),
    opcode!("fpatan"; ),
    opcode!("fxtract"; ),
    opcode!("fperm1"; ),
    opcode!("fdecstp"; ),
    opcode!("fincstp"; ),
    opcode!("fperm"; ),
    opcode!("fyl2xp1"; ),
    opcode!("fsqrt"; ),
    opcode!("fsincos"; ),
    opcode!("frndintx"; ),
    opcode!("fscale"; ),
    opcode!("fsin"; ),
    opcode!("fcos"; ),
];

pub static X87_DA_TABLE: [Opcode; 64] = [
    opcode!("fcmovb"; ST0, ST0),
    opcode!("fcmovb"; ST0, ST1),
    opcode!("fcmovb"; ST0, ST2),
    opcode!("fcmovb"; ST0, ST3),
    opcode!("fcmovb"; ST0, ST4),
    opcode!("fcmovb"; ST0, ST5),
    opcode!("fcmovb"; ST0, ST6),
    opcode!("fcmovb"; ST0, ST7),
    opcode!("fcmove"; ST0, ST0),
    opcode!("fcmove"; ST0, ST1),
    opcode!("fcmove"; ST0, ST2),
    opcode!("fcmove"; ST0, ST3),
    opcode!("fcmove"; ST0, ST4),
    opcode!("fcmove"; ST0, ST5),
    opcode!("fcmove"; ST0, ST6),
    opcode!("fcmove"; ST0, ST7),
    opcode!("fcmovbe"; ST0, ST0),
    opcode!("fcmovbe"; ST0, ST1),
    opcode!("fcmovbe"; ST0, ST2),
    opcode!("fcmovbe"; ST0, ST3),
    opcode!("fcmovbe"; ST0, ST4),
    opcode!("fcmovbe"; ST0, ST5),
    opcode!("fcmovbe"; ST0, ST6),
    opcode!("fcmovbe"; ST0, ST7),
    opcode!("fcmovu"; ST0, ST0),
    opcode!("fcmovu"; ST0, ST1),
    opcode!("fcmovu"; ST0, ST2),
    opcode!("fcmovu"; ST0, ST3),
    opcode!("fcmovu"; ST0, ST4),
    opcode!("fcmovu"; ST0, ST5),
    opcode!("fcmovu"; ST0, ST6),
    opcode!("fcmovu"; ST0, ST7),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    opcode!("fucompp"; ),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
];

pub static X87_DB_TABLE: [Opcode; 64] = [
    opcode!("fcmovnb"; ST0, ST0),
    opcode!("fcmovnb"; ST0, ST1),
    opcode!("fcmovnb"; ST0, ST2),
    opcode!("fcmovnb"; ST0, ST3),
    opcode!("fcmovnb"; ST0, ST4),
    opcode!("fcmovnb"; ST0, ST5),
    opcode!("fcmovnb"; ST0, ST6),
    opcode!("fcmovnb"; ST0, ST7),
    opcode!("fcmovne"; ST0, ST0),
    opcode!("fcmovne"; ST0, ST1),
    opcode!("fcmovne"; ST0, ST2),
    opcode!("fcmovne"; ST0, ST3),
    opcode!("fcmovne"; ST0, ST4),
    opcode!("fcmovne"; ST0, ST5),
    opcode!("fcmovne"; ST0, ST6),
    opcode!("fcmovne"; ST0, ST7),
    opcode!("fcmovnbe"; ST0, ST0),
    opcode!("fcmovnbe"; ST0, ST1),
    opcode!("fcmovnbe"; ST0, ST2),
    opcode!("fcmovnbe"; ST0, ST3),
    opcode!("fcmovnbe"; ST0, ST4),
    opcode!("fcmovnbe"; ST0, ST5),
    opcode!("fcmovnbe"; ST0, ST6),
    opcode!("fcmovnbe"; ST0, ST7),
    opcode!("fcmovnu"; ST0, ST0),
    opcode!("fcmovnu"; ST0, ST1),
    opcode!("fcmovnu"; ST0, ST2),
    opcode!("fcmovnu"; ST0, ST3),
    opcode!("fcmovnu"; ST0, ST4),
    opcode!("fcmovnu"; ST0, ST5),
    opcode!("fcmovnu"; ST0, ST6),
    opcode!("fcmovnu"; ST0, ST7),
    unused!(),
    unused!(),
    opcode!("fclex"; ),
    opcode!("finit"; ),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    opcode!("fucomi"; ST0, ST0),
    opcode!("fucomi"; ST0, ST1),
    opcode!("fucomi"; ST0, ST2),
    opcode!("fucomi"; ST0, ST3),
    opcode!("fucomi"; ST0, ST4),
    opcode!("fucomi"; ST0, ST5),
    opcode!("fucomi"; ST0, ST6),
    opcode!("fucomi"; ST0, ST7),
    opcode!("fcomi"; ST0, ST0),
    opcode!("fcomi"; ST0, ST1),
    opcode!("fcomi"; ST0, ST2),
    opcode!("fcomi"; ST0, ST3),
    opcode!("fcomi"; ST0, ST4),
    opcode!("fcomi"; ST0, ST5),
    opcode!("fcomi"; ST0, ST6),
    opcode!("fcomi"; ST0, ST7),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
];

pub static X87_DC_TABLE: [Opcode; 64] = [
    opcode!("fadd"; ST0, ST0),
    opcode!("fadd"; ST1, ST0),
    opcode!("fadd"; ST2, ST0),
    opcode!("fadd"; ST3, ST0),
    opcode!("fadd"; ST4, ST0),
    opcode!("fadd"; ST5, ST0),
    opcode!("fadd"; ST6, ST0),
    opcode!("fadd"; ST7, ST0),
    opcode!("fmul"; ST0, ST0),
    opcode!("fmul"; ST1, ST0),
    opcode!("fmul"; ST2, ST0),
    opcode!("fmul"; ST3, ST0),
    opcode!("fmul"; ST4, ST0),
    opcode!("fmul"; ST5, ST0),
    opcode!("fmul"; ST6, ST0),
    opcode!("fmul"; ST7, ST0),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    opcode!("fsubr"; ST0, ST0),
    opcode!("fsubr"; ST1, ST0),
    opcode!("fsubr"; ST2, ST0),
    opcode!("fsubr"; ST3, ST0),
    opcode!("fsubr"; ST4, ST0),
    opcode!("fsubr"; ST5, ST0),
    opcode!("fsubr"; ST6, ST0),
    opcode!("fsubr"; ST7, ST0),
    opcode!("fsub"; ST0, ST0),
    opcode!("fsub"; ST1, ST0),
    opcode!("fsub"; ST2, ST0),
    opcode!("fsub"; ST3, ST0),
    opcode!("fsub"; ST4, ST0),
    opcode!("fsub"; ST5, ST0),
    opcode!("fsub"; ST6, ST0),
    opcode!("fsub"; ST7, ST0),
    opcode!("fdivr"; ST0, ST0),
    opcode!("fdivr"; ST1, ST0),
    opcode!("fdivr"; ST2, ST0),
    opcode!("fdivr"; ST3, ST0),
    opcode!("fdivr"; ST4, ST0),
    opcode!("fdivr"; ST5, ST0),
    opcode!("fdivr"; ST6, ST0),
    opcode!("fdivr"; ST7, ST0),
    opcode!("fdiv"; ST0, ST0),
    opcode!("fdiv"; ST1, ST0),
    opcode!("fdiv"; ST2, ST0),
    opcode!("fdiv"; ST3, ST0),
    opcode!("fdiv"; ST4, ST0),
    opcode!("fdiv"; ST5, ST0),
    opcode!("fdiv"; ST6, ST0),
    opcode!("fdiv"; ST7, ST0),
];

pub static X87_DD_TABLE: [Opcode; 64] = [
    opcode!("ffree"; ST0),
    opcode!("ffree"; ST1),
    opcode!("ffree"; ST2),
    opcode!("ffree"; ST3),
    opcode!("ffree"; ST4),
    opcode!("ffree"; ST5),
    opcode!("ffree"; ST6),
    opcode!("ffree"; ST7),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    opcode!("fst"; ST0),
    opcode!("fst"; ST1),
    opcode!("fst"; ST2),
    opcode!("fst"; ST3),
    opcode!("fst"; ST4),
    opcode!("fst"; ST5),
    opcode!("fst"; ST6),
    opcode!("fst"; ST7),
    opcode!("fstp"; ST0),
    opcode!("fstp"; ST1),
    opcode!("fstp"; ST2),
    opcode!("fstp"; ST3),
    opcode!("fstp"; ST4),
    opcode!("fstp"; ST5),
    opcode!("fstp"; ST6),
    opcode!("fstp"; ST7),
    opcode!("fucom"; ST0, ST0),
    opcode!("fucom"; ST1, ST0),
    opcode!("fucom"; ST2, ST0),
    opcode!("fucom"; ST3, ST0),
    opcode!("fucom"; ST4, ST0),
    opcode!("fucom"; ST5, ST0),
    opcode!("fucom"; ST6, ST0),
    opcode!("fucom"; ST7, ST0),
    opcode!("fucomp"; ST0, ST0),
    opcode!("fucomp"; ST1, ST0),
    opcode!("fucomp"; ST2, ST0),
    opcode!("fucomp"; ST3, ST0),
    opcode!("fucomp"; ST4, ST0),
    opcode!("fucomp"; ST5, ST0),
    opcode!("fucomp"; ST6, ST0),
    opcode!("fucomp"; ST7, ST0),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!()
];

pub static X87_DE_TABLE: [Opcode; 64] = [
    opcode!("faddp"; ST0, ST0),
    opcode!("faddp"; ST1, ST0),
    opcode!("faddp"; ST2, ST0),
    opcode!("faddp"; ST3, ST0),
    opcode!("faddp"; ST4, ST0),
    opcode!("faddp"; ST5, ST0),
    opcode!("faddp"; ST6, ST0),
    opcode!("faddp"; ST7, ST0),
    opcode!("fmulp"; ST0, ST0),
    opcode!("fmulp"; ST1, ST0),
    opcode!("fmulp"; ST2, ST0),
    opcode!("fmulp"; ST3, ST0),
    opcode!("fmulp"; ST4, ST0),
    opcode!("fmulp"; ST5, ST0),
    opcode!("fmulp"; ST6, ST0),
    opcode!("fmulp"; ST7, ST0),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    opcode!("fcompp"; ),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    opcode!("fsubrp"; ST0, ST0),
    opcode!("fsubrp"; ST1, ST0),
    opcode!("fsubrp"; ST2, ST0),
    opcode!("fsubrp"; ST3, ST0),
    opcode!("fsubrp"; ST4, ST0),
    opcode!("fsubrp"; ST5, ST0),
    opcode!("fsubrp"; ST6, ST0),
    opcode!("fsubrp"; ST7, ST0),
    opcode!("fsubp"; ST0, ST0),
    opcode!("fsubp"; ST1, ST0),
    opcode!("fsubp"; ST2, ST0),
    opcode!("fsubp"; ST3, ST0),
    opcode!("fsubp"; ST4, ST0),
    opcode!("fsubp"; ST5, ST0),
    opcode!("fsubp"; ST6, ST0),
    opcode!("fsubp"; ST7, ST0),
    opcode!("fdivrp"; ST0, ST0),
    opcode!("fdivrp"; ST1, ST0),
    opcode!("fdivrp"; ST2, ST0),
    opcode!("fdivrp"; ST3, ST0),
    opcode!("fdivrp"; ST4, ST0),
    opcode!("fdivrp"; ST5, ST0),
    opcode!("fdivrp"; ST6, ST0),
    opcode!("fdivrp"; ST7, ST0),
    opcode!("fdivp"; ST0, ST0),
    opcode!("fdivp"; ST1, ST0),
    opcode!("fdivp"; ST2, ST0),
    opcode!("fdivp"; ST3, ST0),
    opcode!("fdivp"; ST4, ST0),
    opcode!("fdivp"; ST5, ST0),
    opcode!("fdivp"; ST6, ST0),
    opcode!("fdivp"; ST7, ST0),
];

pub static X87_DF_TABLE: [Opcode; 64] = [
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    opcode!("fstsw"; AX),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    opcode!("fucomip"; ST0, ST0),
    opcode!("fucomip"; ST0, ST1),
    opcode!("fucomip"; ST0, ST2),
    opcode!("fucomip"; ST0, ST3),
    opcode!("fucomip"; ST0, ST4),
    opcode!("fucomip"; ST0, ST5),
    opcode!("fucomip"; ST0, ST6),
    opcode!("fucomip"; ST0, ST7),
    opcode!("fcomip"; ST0, ST0),
    opcode!("fcomip"; ST0, ST1),
    opcode!("fcomip"; ST0, ST2),
    opcode!("fcomip"; ST0, ST3),
    opcode!("fcomip"; ST0, ST4),
    opcode!("fcomip"; ST0, ST5),
    opcode!("fcomip"; ST0, ST6),
    opcode!("fcomip"; ST0, ST7),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
    unused!(),
];

pub static X87_D8_TABLE2: [Opcode; 8] = [
    opcode!("fadd"; E/d, G/d),
    opcode!("fmul"; E/d, G/d),
    opcode!("fcom"; E/d, G/d),
    opcode!("fcomp"; E/d, G/d),
    opcode!("fsub"; E/d, G/d),
    opcode!("fsubr"; E/d, G/d),
    opcode!("fdiv"; E/d, G/d),
    opcode!("fdivr"; E/d, G/d),
];

pub static X87_D9_TABLE2: [Opcode; 8] = [
    opcode!("fld"; E/d, G/d),
    unused!(),
    opcode!("fst"; E/d, G/d),
    opcode!("fstp"; E/d, G/d),
    opcode!("fldenv"; E/d, G/d),
    opcode!("fldcw"; E/d, G/d),
    opcode!("fstenv"; E/d, G/d),
    opcode!("fstcw"; E/d, G/d),
];

pub static X87_DA_TABLE2: [Opcode; 8] = [
    opcode!("fiadd"; E/d, G/d),
    opcode!("fimul"; E/d, G/d),
    opcode!("ficom"; E/d, G/d),
    opcode!("ficomp"; E/d, G/d),
    opcode!("fisub"; E/d, G/d),
    opcode!("fisubr"; E/d, G/d),
    opcode!("fidiv"; E/d, G/d),
    opcode!("fidivr"; E/d, G/d),
];

pub static X87_DB_TABLE2: [Opcode; 8] = [
    opcode!("fild"; E/d, G/d),
    opcode!("fisttp"; E/d, G/d),
    opcode!("fist"; E/d, G/d),
    opcode!("fistp"; E/d, G/d),
    unused!(),
    opcode!("fld"; E/dq, G/dq),
    unused!(),
    opcode!("fstp"; E/dq, G/dq),
];

pub static X87_DC_TABLE2: [Opcode; 8] = [
    opcode!("fadd"; E/dq, G/dq),
    opcode!("fmul"; E/dq, G/dq),
    opcode!("fcom"; E/dq, G/dq),
    opcode!("fcomp"; E/dq, G/dq),
    opcode!("fsub"; E/dq, G/dq),
    opcode!("fsubr"; E/dq, G/dq),
    opcode!("fdiv"; E/dq, G/dq),
    opcode!("fdivr"; E/dq, G/dq),
];

pub static X87_DD_TABLE2: [Opcode; 8] = [
    opcode!("fld"; E/dq, G/dq),
    opcode!("fisttp"; E/d, G/d),
    opcode!("fst"; E/dq, G/dq),
    opcode!("fstp"; E/dq, G/dq),
    opcode!("frstor"; E/dq, G/dq),
    unused!(),
    opcode!("fsave"; E/dq, G/dq),
    opcode!("fstsw"; E/dq, G/dq),
];

pub static X87_DE_TABLE2: [Opcode; 8] = [
    opcode!("fiadd"; E/w, G/w),
    opcode!("fimul"; E/w, G/w),
    opcode!("ficom"; E/w, G/w),
    opcode!("ficomp"; E/w, G/w),
    opcode!("fisub"; E/w, G/w),
    opcode!("fisubr"; E/w, G/w),
    opcode!("fidiv"; E/w, G/w),
    opcode!("fidivr"; E/w, G/w),
];

pub static X87_DF_TABLE2: [Opcode; 8] = [
    opcode!("fild"; E/w, G/w),
    opcode!("fisttp"; E/w, G/w),
    opcode!("fist"; E/w, G/w),
    opcode!("fistp"; E/w, G/w),
    opcode!("fbld"; E/w, G/w),
    opcode!("fild"; E/dq, G/dq),
    opcode!("fbstp"; E/w, G/w),
    opcode!("fistp"; E/dq, G/dq),
];
