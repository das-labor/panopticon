use disassembler::*;
use program::{Program,DisassembleEvent};
use layer::LayerIter;
use value::{Lvalue,Rvalue,Endianess,ToRvalue};
use codegen::CodeGen;
use guard::Guard;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::rc::Rc;
use std::num::Wrapping;
use super::*;

#[allow(overflowing_literals)]
pub fn disassembler() -> Rc<Disassembler<Avr>> {
    let skip = new_disassembler!(Avr =>
        // SBRC
        [ "1111 110 sr@..... 0 sb@..." ] = |st: &mut State<Avr>| {
            let _b = st.get_group("sb") as u64;
            let b = Rvalue::Constant(_b);
            let mask = Rvalue::Constant(1 << _b);
            let rr = reg(st,"sr");
            let fallthru = st.configuration.wrap(st.address + 2);
            let skip = st.configuration.wrap(st.address + 4);
            let r = new_temp(8);
            let g = Guard::eq(&r,&1);

            st.mnemonic(2,"sbrc","{8}, {3}",vec!(rr.to_rv(),b.to_rv()),&|cg: &mut CodeGen<Avr>| {
                cg.div_i(&r,&rr,&mask);
                cg.mod_i(&r,&r,&2);
            });

            if st.tokens.len() == 1 {
                st.jump(skip,g.negation());
            } else {
                st.configuration.skip = Some((g.negation(),st.address));
            }

            st.jump(fallthru,g);
            true
        },
        // SBRS
        [ "1111 111 sr@..... 0 sb@..." ] = |st: &mut State<Avr>| {
            let _b = st.get_group("sb") as u64;
            let b = Rvalue::Constant(_b);
            let mask = Rvalue::Constant(1 << _b);
            let rr = reg(st,"sr");
            let fallthru = st.configuration.wrap(st.address + 2);
            let skip = st.configuration.wrap(st.address + 4);
            let r = new_temp(8);
            let g = Guard::eq(&r,&0);

            st.mnemonic(2,"sbrs","{8}, {3}",vec!(rr.to_rv(),b.to_rv()),&|cg: &mut CodeGen<Avr>| {
                cg.div_i(&r,&rr,&mask);
                cg.mod_i(&r,&r,&2);
            });

            if st.tokens.len() == 1 {
                st.jump(skip,g.negation());
            } else {
                st.configuration.skip = Some((g.negation(),st.address));
            }

            st.jump(fallthru,g);
            true
        },
        // CPSE
        [ "000100 cr@. cd@..... cr@...." ] = |st: &mut State<Avr>| {
            let rr = reg(st,"cr");
            let rd = reg(st,"cd");
            let fallthru = st.configuration.wrap(st.address + 2);
            let skip = st.configuration.wrap(st.address + 4);
            let g = Guard::eq(&*Z,&1);

            st.mnemonic(2,"cpse","{8}, {8}",vec!(rr.to_rv(),rd.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let r = new_temp(8);
                cg.sub_i(&r,&rr,&rd);

                let half_rr = new_temp(8);
                let half_rd = new_temp(8);

                cg.mod_i(&half_rd,&rd,&0x10);
                cg.mod_i(&half_rr,&rr,&0x10);
                cg.less_i(&*H,&half_rd,&half_rr);

                cg.less_i(&*C,&rd,&rr);
                cg.equal_i(&*Z,&r,&0);
                cg.less_i(&*N,&0x7f,&r);
                cg.not_b(&*V,&*C);
                cg.xor_b(&*S,&*N,&*V);
            });

            if st.tokens.len() == 1 {
                st.jump(skip,g.negation());
            } else {
                st.configuration.skip = Some((g.negation(),st.address));
            }

            st.jump(fallthru,g);
            true
        },
        // SBIC
        [ "1001 1001 sA@..... sb@..." ] = |st: &mut State<Avr>| {
            let _b = st.get_group("sb") as u64;
            let b = Rvalue::Constant(_b);
            let mask = Rvalue::Constant(1 << _b);
            let a = ioreg(st,"sA");
            let fallthru = st.configuration.wrap(st.address + 2);
            let skip = st.configuration.wrap(st.address + 4);
            let r = new_temp(8);
            let g = Guard::eq(&r,&1);

            st.mnemonic(2,"sbic","{8}, {3}",vec!(a.to_rv(),b.to_rv()),&|cg: &mut CodeGen<Avr>| {
                cg.div_i(&r,&a,&mask);
                cg.mod_i(&r,&r,&2);
            });

            if st.tokens.len() == 1 {
                st.jump(skip,g.negation());
            } else {
                st.configuration.skip = Some((g.negation(),st.address));
            }

            st.jump(fallthru,g);
            true
        },
        // SBIS
        [ "1001 1011 sA@..... sb@..." ] = |st: &mut State<Avr>| {
            let _b = st.get_group("sb") as u64;
            let b = Rvalue::Constant(_b);
            let mask = Rvalue::Constant(1 << _b);
            let a = ioreg(st,"sA");
            let fallthru = st.configuration.wrap(st.address + 2);
            let skip = st.configuration.wrap(st.address + 4);
            let r = new_temp(8);
            let g = Guard::eq(&r,&0);

            st.mnemonic(2,"sbis","{8}, {3}",vec!(a.to_rv(),b.to_rv()),&|cg: &mut CodeGen<Avr>| {
                cg.div_i(&r,&a,&mask);
                cg.mod_i(&r,&r,&2);
            });

            if st.tokens.len() == 1 {
                st.jump(skip,g.negation());
            } else {
                st.configuration.skip = Some((g.negation(),st.address));
            }

            st.jump(fallthru,g);
            true
        }
    );

    let main = new_disassembler!(Avr =>
        // MOV
        [ opt!(skip), "001011 r@. d@..... r@...." ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let rr = reg(st,"r");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"mov","{8}, {8}",vec!(rd.to_rv(),rr.to_rv()),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&rd,&rr);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // MOVW
        [ opt!(skip), "00000001 d@.... r@...." ] = |st: &mut State<Avr>| {
            let rd1 = reg(st,"d"); let rd2 = resolv(st.get_group("d") * 2 + 1);
            let rr1 = reg(st,"r"); let rr2 = resolv(st.get_group("r") * 2 + 1);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"mov","{8}, {8}",vec!(rd1.to_rv(),rr1.to_rv()),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&rd1,&rr1);
                cg.assign(&rd2,&rr2);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // IN
        [ opt!(skip), "10110 A@.. d@..... A@...." ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let io = ioreg(st,"A");
            let name = if let Lvalue::Variable{ name: n,..} = io { n } else { "(noname)".to_string() };
            let off = Rvalue::Constant(st.get_group("d") as u64);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"in",&format!("{{8}}, {{8::{}}}",name),vec!(rd.to_rv(),off.to_rv()),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&rd,&sram(&off));
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // OUT
        [ opt!(skip), "10111 A@.. r@..... A@...." ] = |st: &mut State<Avr>| {
            let rr = reg(st,"r");
            let io = ioreg(st,"A");
            let name = if let Lvalue::Variable{ name: n,..} = io.clone() { n } else { "(noname)".to_string() };
            let off = Rvalue::Constant(st.get_group("r") as u64);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"out",&format!("{{8::{}}}, {{8}}",name),vec!(io.to_rv(),rr.to_rv()),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&sram(&off),&rr);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // POP
        [ opt!(skip), "1001000 d@..... 1111" ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"pop","{{8}}",vec!(rd.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let sp = get_sp(cg);
                cg.sub_i(&sp,&sp,&1);
                cg.assign(&rd,&sram(&sp));
                set_sp(&sp,cg);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // PUSH
        [ opt!(skip), "1001001 d@..... 1111" ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"push","{{8}}",vec!(rd.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let sp = get_sp(cg);
                cg.sub_i(&sp,&sp,&1);
                cg.assign(&sram(&sp),&rd);
                set_sp(&sp,cg);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // SWAP
        [ opt!(skip), "1001010 d@..... 0010" ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"swap","{8}",vec!(rd.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let lower = new_temp(8);
                let higher = new_temp(8);

                cg.div_i(&higher,&rd,&128);
                cg.mod_i(&lower,&rd,&127);

                let shifted = new_temp(8);
                cg.mul_i(&shifted,&lower,&128);

                cg.add_i(&rd,&shifted,&higher);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // XCH
        [ opt!(skip), "1001001 r@..... 0100" ] = |st: &mut State<Avr>| {
            let rr = reg(st,"r");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"xch","{8}",vec!(rr.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let z = new_temp(16);
                cg.mul_i(&z,&*R30,&0x100);
                cg.add_i(&z,&*R31,&z);

                let tmp = new_temp(8);
                cg.assign(&tmp,&sram(&z));
                cg.assign(&sram(&z),&rr);
                cg.assign(&rr,&tmp);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // SER
        [ opt!(skip), "11101111 d@.... 1111" ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"ser","{8}",vec!(rd.to_rv()),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&rd,&0xff);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // LDI
        [ opt!(skip), "1110 K@.... d@.... K@...." ] = |st: &mut State<Avr>| {
            let rd = resolv(st.get_group("d") + 16);
            let k = st.get_group("K");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"ldi",&format!("{{8}}, {{::{}}}",k),vec!(rd.to_rv(),Rvalue::Constant(k)),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&rd,&(k as u64));
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // LAC
        [ opt!(skip), "1001001 r@..... 0110" ] = |st: &mut State<Avr>| {
            let rr = reg(st,"r");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"lac","{{8}}",vec!(rr.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let z = new_temp(16);
                cg.mul_i(&z,&*R30,&0x100);
                cg.add_i(&z,&*R31,&z);

                let comp = new_temp(8);
                cg.sub_i(&comp,&0xff,&sram(&z));

                cg.and_i(&sram(&z),&rr,&comp);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // LAS
        [ opt!(skip), "1001001 r@..... 0101" ] = |st: &mut State<Avr>| {
            let rr = reg(st,"r");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"las","{{8}}",vec!(rr.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let z = new_temp(16);
                cg.mul_i(&z,&*R30,&0x100);
                cg.add_i(&z,&*R31,&z);

                let tmp = new_temp(8);
                cg.assign(&tmp,&sram(&z));

                cg.or_i(&sram(&z),&rr,&tmp);
                cg.assign(&rr,&tmp);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // LAT
        [ opt!(skip), "1001001 r@..... 0111" ] = |st: &mut State<Avr>| {
            let rr = reg(st,"r");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"lat","{{8}}",vec!(rr.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let z = new_temp(16);
                cg.mul_i(&z,&*R30,&0x100);
                cg.add_i(&z,&*R31,&z);

                let tmp = new_temp(8);
                cg.assign(&tmp,&sram(&z));

                cg.xor_i(&sram(&z),&rr,&tmp);
                cg.assign(&rr,&tmp);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // LDS
        [ opt!(skip), "1001000 d@..... 0000", "k@................" ] = |st: &mut State<Avr>| {
            let k = Rvalue::Constant(st.get_group("k") as u64);
            let rd = reg(st,"d");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(4,"lds","{{8}}, {{8}}",vec!(rd.to_rv(),k.to_rv()),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&rd,&sram(&k));
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // LDS
        [ opt!(skip), "10100 k@... d@.... k@...." ] = |st: &mut State<Avr>| {
            let k_ = st.get_group("k");
            let k = Rvalue::Constant(((!k_ & 16) | (k_ & 16) | (k_ & 64) | (k_ & 32) | (k_ & 15)) as u64);
            let rd = reg(st,"d");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"lds","{{8}}, {{8}}",vec!(rd.to_rv(),k.to_rv()),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&rd,&sram(&k));
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // LPM
        [ opt!(skip), 0x95c8 ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"lpm","",vec!(),&|cg: &mut CodeGen<Avr>| {
                let z = new_temp(16);
                cg.mul_i(&z,&*R30,&0x100);
                cg.add_i(&z,&*R31,&z);

                cg.assign(&*R0,&flash(&z));
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // LPM
        [ opt!(skip), "1001 000 d@..... 0100" ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);
            let rd = reg(st,"d");

            st.mnemonic_dynargs(2,"lpm","{8}, {16::Z}",&|cg: &mut CodeGen<Avr>| {
                let z = new_temp(16);
                cg.mul_i(&z,&*R30,&0x100);
                cg.add_i(&z,&*R31,&z);
                cg.assign(&rd,&flash(&z));

                vec!(rd.to_rv(),z.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // LPM
        [ opt!(skip), "1001 000 d@..... 0101" ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);
            let rd = reg(st,"d");

            st.mnemonic_dynargs(2,"lpm","{8}, {16::Z+}",&|cg: &mut CodeGen<Avr>| {
                let z = new_temp(16);
                cg.mul_i(&z,&*R30,&0x100);
                cg.add_i(&z,&*R31,&z);
                cg.assign(&rd,&flash(&z));
                cg.add_i(&z,&z,&1);
                cg.mod_i(&*R31,&z,&0x100);
                cg.div_i(&*R30,&z,&0x100);

                vec!(rd.to_rv(),z.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // SPM
        [ opt!(skip), 0x95e8 ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic_dynargs(2,"spm","{{16::X}}",&|cg: &mut CodeGen<Avr>| {
                let z = new_temp(16);
                cg.mul_i(&z,&*R30,&0x100);
                cg.add_i(&z,&*R31,&z);

                cg.assign(&flash(&z),&*R1);
                vec!(z.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // SPM
        [ opt!(skip), 0x95f8 ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic_dynargs(2,"spm","{{16::X+}}",&|cg: &mut CodeGen<Avr>| {
                let z = new_temp(16);
                cg.mul_i(&z,&*R30,&0x100);
                cg.add_i(&z,&*R31,&z);

                cg.assign(&flash(&z),&*R1);
                cg.add_i(&z,&z,&1);

                cg.div_i(&*R30,&z,&0x100);
                cg.mod_i(&*R31,&z,&0x100);

                vec!(z.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // STS
        [ opt!(skip), "1001001 d@..... 0000", "k@................" ] = |st: &mut State<Avr>| {
            let k = Rvalue::Constant(st.get_group("k") as u64);
            let rd = reg(st,"d");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(4,"sts","{16}, {8}",vec!(k.clone(),rd.to_rv()),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&sram(&k),&rd);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // STS
        [ opt!(skip), "10101 k@... d@.... k@...." ] = |st: &mut State<Avr>| {
            let k_ = st.get_group("k");
            let k = Rvalue::Constant(((!k_ & 16) | (k_ & 16) | (k_ & 64) | (k_ & 32) | (k_ & 15)) as u64);
            let rd = reg(st,"d");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"sts","{16}, {8}",vec!(k.clone(),rd.to_rv()),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&sram(&k),&rd);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // SBI
        [ opt!(skip), "1001 1010 A@..... b@..." ] = |st: &mut State<Avr>| {
            let a = Rvalue::Constant(st.get_group("A") as u64);
            let b = Rvalue::Constant(st.get_group("b") as u64);
            let mask = Rvalue::Constant(1 << (st.get_group("b")));
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"sbi","{{8}}, {{8}}",vec!(a.to_rv(),b.to_rv()),&|cg: &mut CodeGen<Avr>| {
                cg.or_i(&sram(&a),&sram(&a),&mask);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // CBI
        [ opt!(skip), "1001 1000 A@..... b@..." ] = |st: &mut State<Avr>| {
            let a = Rvalue::Constant(st.get_group("A") as u64);
            let b = Rvalue::Constant(st.get_group("b") as u64);
            let mask = Rvalue::Constant(((!(1 << (st.get_group("b")))) & 0xff) as u64);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"sbi","{{8}}, {{8}}",vec!(a.to_rv(),b.to_rv()),&|cg: &mut CodeGen<Avr>| {
                cg.and_i(&sram(&a),&sram(&a),&mask);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // SEC
        [ opt!(skip), 0x9408 ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"sec","",vec!(),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&*C,&1);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // SEH
        [ opt!(skip), 0x9458 ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"seh","",vec!(),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&*H,&1);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // SEI
        [ opt!(skip), 0x9478 ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"sei","",vec!(),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&*I,&1);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // SEN
        [ opt!(skip), 0x9428 ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"sen","",vec!(),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&*N,&1);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // SES
        [ opt!(skip), 0x9448 ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"ses","",vec!(),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&*S,&1);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // SET
        [ opt!(skip), 0x9468 ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"set","",vec!(),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&*T,&1);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // SEV
        [ opt!(skip), 0x9438 ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"sev","",vec!(),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&*V,&1);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // SEZ
        [ opt!(skip), 0x9418 ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"sez","",vec!(),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&*Z,&1);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // CLC
        [ opt!(skip), 0x9488 ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"clc","",vec!(),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&*C,&0);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // CLH
        [ opt!(skip), 0x94d8 ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"clh","",vec!(),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&*H,&0);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // CLI
        [ opt!(skip), 0x94f8 ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"cli","",vec!(),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&*I,&0);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // CLN
        [ opt!(skip), 0x94a8 ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"cln","",vec!(),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&*N,&0);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // CLS
        [ opt!(skip), 0x94c8 ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"cls","",vec!(),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&*S,&0);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // CLT
        [ opt!(skip), 0x94e8 ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"clt","",vec!(),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&*T,&0);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // CLV
        [ opt!(skip), 0x94b8 ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"clv","",vec!(),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&*V,&0);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // CLZ
        [ opt!(skip), 0x9498 ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"clz","",vec!(),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&*Z,&0);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // CP
        [ opt!(skip), "0001 01 r@. d@..... r@...." ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);
            let rr = reg(st,"r");
            let rd = reg(st,"d");

            st.mnemonic(2,"cp","{8}, {8}",vec!(rd.to_rv(),rr.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let r = new_temp(8);
                cg.sub_i(&r,&rd,&rr);

                let half_rd = new_temp(4);
                let half_rr = new_temp(4);

                cg.mod_i(&half_rd,&rd,&0x10);
                cg.mod_i(&half_rr,&rr,&0x10);
                cg.less_i(&*H,&half_rd,&half_rr);

                cg.less_i(&*C,&rd,&rr);
                cg.equal_i(&*Z,&r,&0);
                cg.less_i(&*N,&0x7f,&r);
                cg.not_b(&*V,&*C);
                cg.xor_b(&*S,&*N,&*V);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // CPC
        [ opt!(skip), "000001 r@. d@..... r@...." ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);
            let rr = reg(st,"r");
            let rd = reg(st,"d");

            st.mnemonic(2,"cpc","{{8}}, {{8}}",vec!(rd.to_rv(),rr.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let r = new_temp(8);
                let cr = new_temp(8);

                cg.lift_b(&cr,&*C);
                cg.sub_i(&r,&rd,&rr);
                cg.sub_i(&r,&r,&cr);

                let half_rd = new_temp(4);
                let half_rr = new_temp(4);

                cg.mod_i(&half_rd,&rd,&0x10);
                cg.mod_i(&half_rr,&rr,&0x10);
                cg.less_i(&*H,&half_rd,&half_rr);

                cg.less_i(&*C,&rd,&rr);
                cg.equal_i(&*Z,&r,&0);
                cg.less_i(&*N,&0x7f,&r);
                cg.not_b(&*V,&*C);
                cg.xor_b(&*S,&*N,&*V);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // CPI
        [ opt!(skip), "0011 K@.... d@.... K@...." ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);
            let k = st.get_group("K") as u64;
            let rd = resolv(st.get_group("d") + 16);

            st.mnemonic(2,"cpi","{{8}}, {{8}}",vec!(rd.to_rv(),Rvalue::Constant(k)),&|cg: &mut CodeGen<Avr>| {
                let r = new_temp(8);
                cg.sub_i(&r,&rd,&k);

                let half_k = new_temp(4);
                let half_rd = new_temp(4);

                cg.mod_i(&half_k,&k,&0x10);
                cg.mod_i(&half_rd,&rd,&0x10);
                cg.less_i(&*H,&half_k,&half_rd);

                cg.less_i(&*C,&k,&rd);
                cg.equal_i(&*Z,&r,&0);
                cg.less_i(&*N,&0x7f,&r);
                cg.not_b(&*V,&*C);
                cg.xor_b(&*S,&*N,&*V);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // LSR
        [ opt!(skip), "1001010 d@..... 0110" ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);
            let rd = reg(st,"d");

            st.mnemonic(2,"lsr","",vec!(rd.to_rv()),&|cg: &mut CodeGen<Avr>| {
                cg.mod_i(&*C,&rd,&2);
                cg.rshift_i(&rd,&rd,&1);
                cg.xor_b(&*S,&*V,&*N);
                cg.xor_b(&*V,&*N,&*C);
                cg.assign(&*N,&0);
                cg.equal_i(&*Z,&rd,&0);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // ADC
        [ opt!(skip), "000111 r@. d@..... r@...." ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);
            let rd = reg(st,"d");
            let rr = reg(st,"r");

            st.mnemonic(2,"adc","{{8}}, {{8}}",vec!(rd.to_rv(),rr.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let cr = new_temp(1);
                let r = new_temp(9);

                cg.lift_b(&cr,&*C);
                cg.add_i(&r,&rd,&rr);
                cg.add_i(&r,&r,&cr);

                let half_rd = new_temp(4);
                let half_rr = new_temp(4);

                cg.mod_i(&half_rd,&rd,&0x10);
                cg.mod_i(&half_rr,&rr,&0x10);
                cg.less_i(&*H,&half_rd,&half_rr);

                cg.less_i(&*C,&r,&0x100);
                cg.less_i(&*N,&r,&0x7f);
                cg.equal_i(&*Z,&0,&r);
                cg.xor_b(&*S,&*V,&*N);
                cg.mod_i(&rd,&r,&0x100);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // ADD
        [ opt!(skip), "0000 11 r@. d@..... r@...." ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);
            let rd = reg(st,"d");
            let rr = reg(st,"r");

            st.mnemonic(2,"add","{{8}}, {{8}}",vec!(rd.to_rv(),rr.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let cr = new_temp(1);
                let r = new_temp(9);

                cg.lift_b(&cr,&*C);
                cg.add_i(&r,&rd,&rr);
                cg.add_i(&r,&r,&cr);

                let half_rd = new_temp(4);
                let half_rr = new_temp(4);

                cg.mod_i(&half_rd,&rd,&0x10);
                cg.mod_i(&half_rr,&rr,&0x10);
                cg.less_i(&*H,&half_rd,&half_rr);

                cg.less_i(&*C,&r,&0x100);
                cg.less_i(&*N,&r,&0x7f);
                cg.equal_i(&*Z,&0,&r);
                cg.xor_b(&*S,&*V,&*N);
                cg.mod_i(&rd,&r,&0x100);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // AND
        [ opt!(skip), "0010 00 r@. d@..... r@...." ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let rr = reg(st,"r");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"and","{8}, {8}",vec!(rd.to_rv(),rr.to_rv()),&|cg: &mut CodeGen<Avr>| {
                cg.and_i(&rd,&rd,&rr);
                cg.assign(&*V,&0);
                cg.equal_i(&*Z,&0,&rd);
                cg.less_i(&*N,&rd,&0x80);
                cg.not_b(&*N,&*N);
                cg.xor_b(&*S,&*N,&*V);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // ANDI
        [ opt!(skip), "0111 K@.... d@.... K@...." ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let _k = st.get_group("K");
            let k = Rvalue::Constant(_k as u64);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"andi","{8}, {8}",vec!(rd.to_rv(),k.clone()),&|cg: &mut CodeGen<Avr>| {
                cg.and_i(&rd,&rd,&k);
                cg.assign(&*V,&0);
                cg.equal_i(&*Z,&0,&rd);
                cg.less_i(&*N,&rd,&0x80);
                cg.not_b(&*N,&*N);
                cg.xor_b(&*S,&*N,&*V);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // SUB
        [ opt!(skip), "000110 r@. d@..... r@...." ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let rr = reg(st,"r");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"sub","{8}, {8}",vec!(rd.to_rv(),rr.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let r = new_temp(8);

                cg.sub_i(&r,&rd,&rr);

                let half_rr = new_temp(8);
                let half_rd = new_temp(8);

                cg.mod_i(&half_rd,&rd,&0x10);
                cg.mod_i(&half_rr,&rr,&0x10);
                cg.less_i(&*H,&half_rd,&half_rr);

                cg.less_i(&*C,&rd,&rr);
                cg.equal_i(&*Z,&r,&0);
                cg.less_i(&*N,&0x7f,&r);
                cg.not_b(&*V,&*C);
                cg.xor_b(&*S,&*N,&*V);
                cg.assign(&rd,&r);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // SUBI
        [ opt!(skip), "0101 K@.... d@.... K@...." ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let k = st.get_group("K");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"subi","{8}, {8}",vec!(rd.to_rv(),Rvalue::Constant(k as u64)),&|cg: &mut CodeGen<Avr>| {
                let r = new_temp(8);
                let cr = new_temp(8);

                cg.lift_b(&cr,&*C);

                cg.sub_i(&r,&rd,&(k as u64));
                cg.sub_i(&r,&r,&cr);

                let half_rd = new_temp(8);

                cg.mod_i(&half_rd,&rd,&0x10);
                cg.less_i(&*H,&half_rd,&((k % 0x10) as u64));

                cg.less_i(&*C,&rd,&(k as u64));
                cg.equal_i(&*Z,&r,&0);
                cg.less_i(&*N,&0x7f,&r);
                cg.not_b(&*V,&*C);
                cg.xor_b(&*S,&*N,&*V);
                cg.assign(&rd,&r);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // ASR
        [ opt!(skip), "1001010 d@..... 0101" ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"asr","{8}",vec!(rd.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let low = new_temp(8);

                cg.mod_i(&low,&rd,&2);
                cg.xor_b(&*S,&*N,&*V);
                cg.equal_i(&*C,&low,&0);

                let r = new_temp(8);
                cg.div_i(&r,&rd,&2);

                cg.less_i(&*N,&0x7f,&r);
                cg.equal_i(&*Z,&r,&0);
                cg.assign(&rd,&r);
                cg.xor_b(&*V,&*N,&*C);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // BST
        [ opt!(skip), "1111 101 d@..... 0 b@..." ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let b = st.get_group("b");
            let mask = 1 << (b as u64);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"bst","{8}, {8}",vec!(rd.to_rv(),Rvalue::Constant(b as u64)),&|cg: &mut CodeGen<Avr>| {
                let t = new_temp(8);

                cg.div_i(&t,&rd,&mask);
                cg.mod_i(&t,&t,&2);
                cg.equal_i(&*T,&t,&0);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // BLD
        [ opt!(skip), "1111 100 d@..... 0 b@..." ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let b = st.get_group("b");
            let mask = 1 << (b as u64);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"bld","{8}, {8}",vec!(rd.to_rv(),Rvalue::Constant(b as u64)),&|cg: &mut CodeGen<Avr>| {
                let t = new_temp(8);

                cg.lift_b(&t,&*T);
                cg.mul_i(&t,&t,&mask);
                cg.or_i(&rd,&rd,&t);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },

        // ROL
        [ opt!(skip), "000111 d@.........." ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"rol","{8}",vec!(rd.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let c = new_temp(1);

                cg.div_i(&c,&rd,&0x80);
                cg.mul_i(&rd,&rd,&2);
                cg.add_i(&rd,&rd,&*C);
                cg.assign(&*C,&c);

                let half_rd = new_temp(8);
                cg.div_i(&half_rd,&rd,&0x10);
                cg.mod_i(&half_rd,&half_rd,&2);
                cg.equal_i(&*H,&half_rd,&1);
                cg.xor_b(&*S,&*N,&*V);
                cg.less_i(&*N,&0x7f,&rd);
                cg.equal_i(&*Z,&rd,&0);
                cg.xor_b(&*V,&*N,&*C);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // ROR
        [ opt!(skip), "1001010 d@..... 0111" ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"ror","{8}",vec!(rd.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let c = new_temp(1);

                cg.mod_i(&c,&rd,&2);
                cg.div_i(&rd,&rd,&2);

                let t = new_temp(8);
                cg.mul_i(&t,&*C,&0x80);
                cg.add_i(&rd,&rd,&t);
                cg.assign(&*C,&c);

                cg.xor_b(&*S,&*N,&*V);
                cg.less_i(&*N,&0x7f,&rd);
                cg.equal_i(&*Z,&rd,&0);
                cg.xor_b(&*V,&*N,&*C);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // DEC
        [ opt!(skip), "1001010 d@..... 1010" ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"dec","{8}",vec!(rd.to_rv()),&|cg: &mut CodeGen<Avr>| {
                cg.sub_i(&rd,&rd,&1);
                cg.equal_i(&*C,&rd,&0xff);

                cg.less_i(&*N,&0x7f,&rd);
                cg.equal_i(&*Z,&rd,&0);
                cg.equal_i(&*V,&rd,&0x80);
                cg.xor_b(&*S,&*N,&*V);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // INC
        [ opt!(skip), "1001010 d@..... 0011" ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"inc","{8}",vec!(rd.to_rv()),&|cg: &mut CodeGen<Avr>| {
                cg.add_i(&rd,&rd,&1);
                cg.equal_i(&*C,&rd,&0);

                cg.less_i(&*N,&0x7f,&rd);
                cg.equal_i(&*Z,&rd,&0);
                cg.equal_i(&*V,&rd,&0x80);
                cg.xor_b(&*S,&*N,&*V);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // SBC
        [ opt!(skip), "000010 r@. d@..... r@...." ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let rr = reg(st,"r");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"sbc","{8}, {8}",vec!(rd.to_rv(),rr.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let r = new_temp(8);
                let cr = new_temp(8);

                cg.lift_b(&cr,&*C);

                cg.sub_i(&r,&rd,&rr);
                cg.sub_i(&r,&r,&cr);

                let half_rd = new_temp(8);
                let half_rr = new_temp(8);

                cg.mod_i(&half_rd,&rd,&0x10);
                cg.mod_i(&half_rr,&rr,&0x10);

                cg.less_i(&*C,&rd,&rr);
                cg.equal_i(&*Z,&r,&0);
                cg.less_i(&*N,&0x7f,&r);
                cg.not_b(&*V,&*C);
                cg.xor_b(&*S,&*N,&*V);
                cg.assign(&rd,&r);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // SBCI
        [ opt!(skip), "0100 K@.... d@.... K@...." ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let _k = st.get_group("K");
            let k = Rvalue::Constant(_k as u64);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"sbci","{8}, {8}",vec!(rd.to_rv(),k.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let r = new_temp(8);
                let cr = new_temp(8);

                cg.lift_b(&cr,&*C);

                cg.sub_i(&r,&rd,&k);
                cg.sub_i(&r,&r,&cr);

                let half_rd = new_temp(8);

                cg.mod_i(&half_rd,&rd,&0x10);
                cg.less_i(&*H,&half_rd,&Rvalue::Constant((_k % 0x10) as u64));

                cg.less_i(&*C,&rd,&k);
                cg.equal_i(&*Z,&r,&0);
                cg.less_i(&*N,&0x7f,&r);
                cg.not_b(&*V,&*C);
                cg.xor_b(&*S,&*N,&*V);
                cg.assign(&rd,&r);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // COM
        [ opt!(skip), "1001010 d@..... 0000" ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"com","{8}",vec!(rd.to_rv()),&|cg: &mut CodeGen<Avr>| {
                cg.sub_i(&rd,&0xff,&rd);

                cg.assign(&*C,&0);
                cg.equal_i(&*Z,&rd,&0);
                cg.less_i(&*N,&0x7f,&rd);
                cg.assign(&*V,&0);
                cg.xor_b(&*S,&*N,&*V);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // ADIW
        [ opt!(skip), "10010110 K@.. d@.. K@...." ] = |st: &mut State<Avr>| {
            let d = st.get_group("d") * 2 + 24;
            let k = Rvalue::Constant(st.get_group("K") as u64);
            let rd1 = resolv(d);
            let rd2 = resolv(d + 1);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"adiw","{8}:{8}, {8}",vec!(rd1.to_rv(),rd2.to_rv(),k.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let r = new_temp(16);

                cg.assign(&r,&rd2);
                cg.mul_i(&r,&r,&0x100);
                cg.add_i(&r,&r,&rd1);
                cg.add_i(&r,&r,&k);

                let v1 = new_temp(1);
                let v2 = new_temp(1);
                cg.less_i(&v1,&rd2,&0x80);
                cg.less_i(&v2,&r,&0x8000);
                cg.not_b(&v1,&v1);
                cg.and_b(&*V,&v1,&v2);

                cg.less_i(&*N,&r,&0x8000);
                cg.equal_i(&*Z,&0,&r);

                let c1 = new_temp(1);
                let c2 = new_temp(1);
                cg.less_i(&c1,&rd2,&0x80);
                cg.less_i(&c2,&r,&0x8000);
                cg.not_b(&c2,&c2);
                cg.and_b(&*C,&c1,&c2);

                cg.xor_b(&*S,&*N,&*V);

                cg.div_i(&rd2,&r,&0x100);
                cg.mod_i(&rd1,&r,&0x100);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // SBIW
        [ opt!(skip), "10010111 K@.. d@.. K@...." ] = |st: &mut State<Avr>| {
            let d = st.get_group("d") * 2 + 24;
            let k = Rvalue::Constant(st.get_group("K") as u64);
            let rd1 = resolv(d);
            let rd2 = resolv(d + 1);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"sbiw","{8}:{8}, {8}",vec!(rd1.to_rv(),rd2.to_rv(),k.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let r = new_temp(16);

                cg.assign(&r,&rd2);
                cg.mul_i(&r,&r,&0x100);
                cg.add_i(&r,&r,&rd1);
                cg.sub_i(&r,&r,&k);

                let v1 = new_temp(1);
                let v2 = new_temp(1);
                cg.less_i(&v1,&rd2,&0x80);
                cg.less_i(&v2,&r,&0x8000);
                cg.not_b(&v2,&v2);
                cg.and_b(&*V,&v1,&v2);

                cg.less_i(&*N,&r,&0x8000);
                cg.equal_i(&*Z,&0,&r);

                let c1 = new_temp(1);
                let c2 = new_temp(1);
                cg.less_i(&c1,&rd2,&0x80);
                cg.less_i(&c2,&r,&0x8000);
                cg.not_b(&c1,&c1);
                cg.and_b(&*C,&c1,&c2);

                cg.xor_b(&*S,&*N,&*V);

                cg.div_i(&rd2,&r,&0x100);
                cg.mod_i(&rd1,&r,&0x100);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // MULS
        [ opt!(skip), "0000 0010 d@.... r@...." ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let rr = reg(st,"r");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"muls","{8}, {8}",vec!(rd.to_rv(),rr.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let r = new_temp(17);

                cg.mul_i(&r,&rd,&rr);
                cg.less_i(&*C,&r,&0x8000);
                cg.equal_i(&*Z,&0,&r);

                cg.div_i(&*R1,&r,&0x100);
                cg.mod_i(&*R0,&r,&0x100);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // MULSU
        [ opt!(skip), "0000 0011 0 d@... 0 r@..." ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let rr = reg(st,"r");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"mulsu","{8}, {8}",vec!(rd.to_rv(),rr.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let r = new_temp(17);

                cg.mul_i(&r,&rd,&rr);
                cg.less_i(&*C,&r,&0x8000);
                cg.equal_i(&*Z,&0,&r);

                cg.div_i(&*R1,&r,&0x100);
                cg.mod_i(&*R0,&r,&0x100);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // NEG
        [ opt!(skip), "1001 010 d@..... 0001" ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"neg","{8}",vec!(rd.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let r = new_temp(8);

                cg.sub_i(&r,&0,&rd);

                let half_rd = new_temp(8);

                cg.mod_i(&half_rd,&rd,&0x10);
                cg.less_i(&*H,&0x7,&r);

                cg.less_i(&*C,&0,&r);
                cg.equal_i(&*Z,&r,&0);
                cg.less_i(&*N,&0x7f,&r);
                cg.equal_i(&*V,&r,&0x80);
                cg.xor_b(&*S,&*N,&*V);
                cg.assign(&rd,&r);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // MUL
        [ opt!(skip), "1001 11 r@. d@..... r@...." ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let rr = reg(st,"r");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"mul","{8}, {8}",vec!(rd.to_rv(),rr.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let r = new_temp(17);

                cg.mul_i(&r,&rd,&rr);
                cg.less_i(&*C,&r,&0x8000);
                cg.equal_i(&*Z,&0,&r);

                cg.div_i(&*R1,&r,&0x100);
                cg.mod_i(&*R0,&r,&0x100);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // BRCx
        [ opt!(skip), "11110 x@. k@....... 000" ] = |st: &mut State<Avr>| {
            let _k = st.get_group("k") as i16; // 6 bits
            let pc_mod = (1 << (st.configuration.pc_bits - 1)) as i16;
            let k = (((if _k & 0x20 != 0 {
                (Wrapping(_k) | Wrapping(0xFFE0)).0
            } else {
                _k
            } * 2 + 2 + (st.address as i16)) % pc_mod) + pc_mod) % pc_mod;
            let fallthru = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);
            let g = Guard::eq(&*C,&0);
            let skip = st.configuration.wrap(k as u64);

            optional_skip(fallthru.clone(),st);
            if st.get_group("x") == 0 {
                st.mnemonic(2,"brcs","{8}",vec!(Rvalue::Constant(k as u64)),&|_: &mut CodeGen<Avr>| {});
                st.jump(skip,g.negation());
                st.jump(fallthru,g);
            } else {
                st.mnemonic(2,"brcc","{8}",vec!(Rvalue::Constant(k as u64)),&|_: &mut CodeGen<Avr>| {});
                st.jump(fallthru,g.negation());
                st.jump(skip,g);
            }
            true
        },
        // BREQ/BRNE
        [ opt!(skip), "11110 x@. k@....... 001" ] = |st: &mut State<Avr>| {
            let _k = st.get_group("k") as i16; // 6 bits
            let pc_mod = (1 << (st.configuration.pc_bits - 1)) as i16;
            let k = (((if _k & 0x20 != 0 {
                (Wrapping(_k) | Wrapping(0xFFE0)).0
            } else {
                _k
            } * 2 + 2 + (st.address as i16)) % pc_mod) + pc_mod) % pc_mod;
            let fallthru = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);
            let g = Guard::eq(&*Z,&0);
            let skip = st.configuration.wrap(k as u64);

            optional_skip(fallthru.clone(),st);
            if st.get_group("x") == 0 {
                st.mnemonic(2,"breq","{8}",vec!(Rvalue::Constant(k as u64)),&|_: &mut CodeGen<Avr>| {});
                st.jump(skip,g.negation());
                st.jump(fallthru,g);
            } else {
                st.mnemonic(2,"brne","{8}",vec!(Rvalue::Constant(k as u64)),&|_: &mut CodeGen<Avr>| {});
                st.jump(fallthru,g.negation());
                st.jump(skip,g);
            }
            true
        },
        // BRNx
        [ opt!(skip), "11110 x@. k@....... 010" ] = |st: &mut State<Avr>| {
            let _k = st.get_group("k") as i16; // 6 bits
            let pc_mod = (1 << (st.configuration.pc_bits - 1)) as i16;
            let k = (((if _k & 0x20 != 0 {
                (Wrapping(_k) | Wrapping(0xFFE0)).0
            } else {
                _k
            } * 2 + 2 + (st.address as i16)) % pc_mod) + pc_mod) % pc_mod;
            let fallthru = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);
            let g = Guard::eq(&*N,&0);
            let skip = st.configuration.wrap(k as u64);

            optional_skip(fallthru.clone(),st);
            if st.get_group("x") == 0 {
                st.mnemonic(2,"brns","{8}",vec!(Rvalue::Constant(k as u64)),&|_: &mut CodeGen<Avr>| {});
                st.jump(skip,g.negation());
                st.jump(fallthru,g);
            } else {
                st.mnemonic(2,"brnc","{8}",vec!(Rvalue::Constant(k as u64)),&|_: &mut CodeGen<Avr>| {});
                st.jump(fallthru,g.negation());
                st.jump(skip,g);
            }
            true
        },
        // BRVx
        [ opt!(skip), "11110 x@. k@....... 011" ] = |st: &mut State<Avr>| {
            let _k = st.get_group("k") as i16; // 6 bits
            let pc_mod = (1 << (st.configuration.pc_bits - 1)) as i16;
            let k = (((if _k & 0x20 != 0 {
                (Wrapping(_k) | Wrapping(0xFFE0)).0
            } else {
                _k
            } * 2 + 2 + (st.address as i16)) % pc_mod) + pc_mod) % pc_mod;
            let fallthru = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);
            let g = Guard::eq(&*V,&0);
            let skip = st.configuration.wrap(k as u64);

            optional_skip(fallthru.clone(),st);
            if st.get_group("x") == 0 {
                st.mnemonic(2,"brvs","{8}",vec!(Rvalue::Constant(k as u64)),&|_: &mut CodeGen<Avr>| {});
                st.jump(skip,g.negation());
                st.jump(fallthru,g);
            } else {
                st.mnemonic(2,"brvc","{8}",vec!(Rvalue::Constant(k as u64)),&|_: &mut CodeGen<Avr>| {});
                st.jump(fallthru,g.negation());
                st.jump(skip,g);
            }
            true
        },
        // BRGE/BTLT
        [ opt!(skip), "11110 x@. k@....... 100" ] = |st: &mut State<Avr>| {
            let _k = st.get_group("k") as i16; // 6 bits
            let pc_mod = (1 << (st.configuration.pc_bits - 1)) as i16;
            let k = (((if _k & 0x20 != 0 {
                (Wrapping(_k) | Wrapping(0xFFE0)).0
            } else {
                _k
            } * 2 + 2 + (st.address as i16)) % pc_mod) + pc_mod) % pc_mod;
            let fallthru = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);
            let g = Guard::eq(&*S,&0);
            let skip = st.configuration.wrap(k as u64);

            optional_skip(fallthru.clone(),st);
            if st.get_group("x") == 0 {
                st.mnemonic(2,"brlt","{8}",vec!(Rvalue::Constant(k as u64)),&|_: &mut CodeGen<Avr>| {});
                st.jump(skip,g.negation());
                st.jump(fallthru,g);
            } else {
                st.mnemonic(2,"brge","{8}",vec!(Rvalue::Constant(k as u64)),&|_: &mut CodeGen<Avr>| {});
                st.jump(fallthru,g.negation());
                st.jump(skip,g);
            }
            true
        },
        // BRHx
        [ opt!(skip), "11110 x@. k@....... 101" ] = |st: &mut State<Avr>| {
            let _k = st.get_group("k") as i16; // 6 bits
            let pc_mod = (1 << (st.configuration.pc_bits - 1)) as i16;
            let k = (((if _k & 0x20 != 0 {
                (Wrapping(_k) | Wrapping(0xFFE0)).0
            } else {
                _k
            } * 2 + 2 + (st.address as i16)) % pc_mod) + pc_mod) % pc_mod;
            let fallthru = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);
            let g = Guard::eq(&*H,&0);
            let skip = st.configuration.wrap(k as u64);

            optional_skip(fallthru.clone(),st);
            if st.get_group("x") == 0 {
                st.mnemonic(2,"brhs","{8}",vec!(Rvalue::Constant(k as u64)),&|_: &mut CodeGen<Avr>| {});
                st.jump(skip,g.negation());
                st.jump(fallthru,g);
            } else {
                st.mnemonic(2,"brhc","{8}",vec!(Rvalue::Constant(k as u64)),&|_: &mut CodeGen<Avr>| {});
                st.jump(fallthru,g.negation());
                st.jump(skip,g);
            }
            true
        },
        // BRTx
        [ opt!(skip), "11110 x@. k@....... 110" ] = |st: &mut State<Avr>| {
            let _k = st.get_group("k") as i16; // 6 bits
            let pc_mod = (1 << (st.configuration.pc_bits - 1)) as i16;
            let k = (((if _k & 0x20 != 0 {
                (Wrapping(_k) | Wrapping(0xFFE0)).0
            } else {
                _k
            } * 2 + 2 + (st.address as i16)) % pc_mod) + pc_mod) % pc_mod;
            let fallthru = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);
            let g = Guard::eq(&*T,&0);
            let skip = st.configuration.wrap(k as u64);

            optional_skip(fallthru.clone(),st);
            if st.get_group("x") == 0 {
                st.mnemonic(2,"brts","{8}",vec!(Rvalue::Constant(k as u64)),&|_: &mut CodeGen<Avr>| {});
                st.jump(skip,g.negation());
                st.jump(fallthru,g);
            } else {
                st.mnemonic(2,"brtc","{8}",vec!(Rvalue::Constant(k as u64)),&|_: &mut CodeGen<Avr>| {});
                st.jump(fallthru,g.negation());
                st.jump(skip,g);
            }
            true
        },
        // BRIx
        [ opt!(skip), "11110 x@. k@....... 111" ] = |st: &mut State<Avr>| {
            let _k = st.get_group("k") as i16; // 6 bits
            let pc_mod = (1 << (st.configuration.pc_bits - 1)) as i16;
            let k = (((if _k & 0x20 != 0 {
                (Wrapping(_k) | Wrapping(0xFFE0)).0
            } else {
                _k
            } * 2 + 2 + (st.address as i16)) % pc_mod) + pc_mod) % pc_mod;
            let fallthru = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);
            let g = Guard::eq(&*I,&0);
            let skip = st.configuration.wrap(k as u64);

            optional_skip(fallthru.clone(),st);
            if st.get_group("x") == 0 {
                st.mnemonic(2,"brie","{8}",vec!(Rvalue::Constant(k as u64)),&|_: &mut CodeGen<Avr>| {});
                st.jump(skip,g.negation());
                st.jump(fallthru,g);
            } else {
                st.mnemonic(2,"brid","{8}",vec!(Rvalue::Constant(k as u64)),&|_: &mut CodeGen<Avr>| {});
                st.jump(fallthru,g.negation());
                st.jump(skip,g);
            }
            true
        },
        // CALL
        [ opt!(skip), "1001010 k@..... 111 k@.", "k@................" ] = |st: &mut State<Avr>| {
            let pc_mod = 1 << st.configuration.pc_bits;
            let _k = (st.get_group("k") % pc_mod) * 2;
            let k = Rvalue::Constant(_k);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(4,"call","{26}",vec!(k.to_rv()),&|cg: &mut CodeGen<Avr>| {
                cg.call_i(&Lvalue::Undefined,&k);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // JMP
        [ opt!(skip), "1001010 k@..... 110 k@.", "k@................" ] = |st: &mut State<Avr>| {
            let pc_mod = 1 << st.configuration.pc_bits;
            let _k = (st.get_group("k") % pc_mod) * 2;
            let k = Rvalue::Constant(_k);

            st.mnemonic(4,"jmp","{26}",vec!(k.to_rv()),&|_: &mut CodeGen<Avr>| {});
            optional_skip(st.configuration.wrap(st.address + st.tokens.len() as u64 * 2),st);
            st.jump(k,Guard::always());
            true
        },
        // RCALL
        [ opt!(skip), "1101 k@............" ] = |st: &mut State<Avr>| {
            let _k = st.get_group("k") as i64;
            let k = st.configuration.wrap_signed(if _k & 0x0800 != 0 {
                (Wrapping(_k) | Wrapping(0xFFFFFFFFFFFFF000)).0
            } else {
                _k
            } * 2 + 2 + (st.address as i64));
            let next =  st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"rcall","{26}",vec!(k.clone()),&|cg: &mut CodeGen<Avr>| {
                cg.call_i(&Lvalue::Undefined,&k);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // RJMP
        [ opt!(skip), "1100 k@............" ] = |st: &mut State<Avr>| {
            let _k = st.get_group("k") as i64;
            let k = if _k & 0x0800 != 0 {
                (Wrapping(_k) | Wrapping(0xFFFFFFFFFFFFF000)).0
            } else {
                _k
            } * 2 + 2 + (st.address as i64);
            let skip = st.configuration.wrap_signed(k);

            st.mnemonic(2,"rjmp","{26}",vec!(Rvalue::Constant(k as u64)),&|_: &mut CodeGen<Avr>| {});
            optional_skip(st.configuration.wrap(st.address + st.tokens.len() as u64 * 2),st);
            st.jump(skip,Guard::always());
            true
        },
        // RET
        [ opt!(skip), 0x9508 ] = |st: &mut State<Avr>| {
            st.mnemonic(2,"ret","",vec!(),&|_: &mut CodeGen<Avr>| {});
            optional_skip(st.configuration.wrap(st.address + st.tokens.len() as u64 * 2),st);
            true
        },
        // RETI
        [ opt!(skip), 0x9518 ] = |st: &mut State<Avr>| {
            st.mnemonic(2,"reti","",vec!(),&|_: &mut CodeGen<Avr>| {});
            optional_skip(st.configuration.wrap(st.address + st.tokens.len() as u64 * 2),st);
            true
        },
        // IJMP
        [ opt!(skip), 0x9409 ] = |st: &mut State<Avr>| {
            let z = new_temp(16);
            st.mnemonic_dynargs(2,"ijmp","{16::Z}",&|cg: &mut CodeGen<Avr>| {
                cg.mul_i(&z,&*R30,&0x100);
                cg.add_i(&z,&*R31,&z);
                vec!(z.to_rv())
            });
            optional_skip(st.configuration.wrap(st.address + st.tokens.len() as u64 * 2),st);
            st.jump(z.to_rv(),Guard::always());
            true
        },
        // ICALL
        [ opt!(skip), 0x9509 ] = |st: &mut State<Avr>| {
            let z = new_temp(16);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic_dynargs(2,"icall","{16::Z}",&|cg: &mut CodeGen<Avr>| {
                cg.mul_i(&z,&*R30,&0x100);
                cg.add_i(&z,&*R31,&z);

                cg.call_i(&Lvalue::Undefined,&z);
                vec!(z.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // ST X
        [ opt!(skip), "1001 001 r@. r@.... 1100" ] = |st: &mut State<Avr>| {
            let rr = reg(st,"r");
            let x = new_temp(16);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic_dynargs(2,"st","{16::X}",&|cg: &mut CodeGen<Avr>| {
                cg.mul_i(&x,&*R26,&0x100);
                cg.add_i(&x,&*R27,&x);
                cg.assign(&sram(&x),&rr);
                vec!(x.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // ST -X
        [ opt!(skip), "1001 001 r@. r@.... 1110" ] = |st: &mut State<Avr>| {
            let rr = reg(st,"r");
            let x = new_temp(16);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic_dynargs(2,"st","{16::-X}",&|cg: &mut CodeGen<Avr>| {
                cg.mul_i(&x,&*R26,&0x100);
                cg.add_i(&x,&*R27,&x);
                cg.sub_i(&x,&x,&1);
                cg.assign(&sram(&x),&rr);
                vec!(x.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // ST X+
        [ opt!(skip), "1001 001 r@. r@.... 1101" ] = |st: &mut State<Avr>| {
            let rr = reg(st,"r");
            let x = new_temp(16);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic_dynargs(2,"st","{16::X+}",&|cg: &mut CodeGen<Avr>| {
                cg.mul_i(&x,&*R26,&0x100);
                cg.add_i(&x,&*R27,&x);
                cg.assign(&sram(&x),&rr);
                cg.add_i(&x,&x,&1);
                vec!(x.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // ST Y
        [ opt!(skip), "1001 001 r@. r@.... 1000" ] = |st: &mut State<Avr>| {
            let rr = reg(st,"r");
            let x = new_temp(16);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic_dynargs(2,"st","{16::Y}",&|cg: &mut CodeGen<Avr>| {
                cg.mul_i(&x,&*R28,&0x100);
                cg.add_i(&x,&*R29,&x);
                    cg.sub_i(&x,&x,&1);
                cg.assign(&sram(&x),&rr);
                    cg.add_i(&x,&x,&1);
                vec!(x.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // ST -Y
        [ opt!(skip), "1001 001 r@. r@.... 1010" ] = |st: &mut State<Avr>| {
            let rr = reg(st,"r");
            let x = new_temp(16);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic_dynargs(2,"st","{16::-Y}",&|cg: &mut CodeGen<Avr>| {
                cg.mul_i(&x,&*R28,&0x100);
                cg.add_i(&x,&*R29,&x);
                cg.sub_i(&x,&x,&1);
                cg.assign(&sram(&x),&rr);
                vec!(x.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // ST Y+
        [ opt!(skip), "1001 001 r@. r@.... 1001" ] = |st: &mut State<Avr>| {
            let rr = reg(st,"r");
            let x = new_temp(16);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic_dynargs(2,"st","{16::Y+}",&|cg: &mut CodeGen<Avr>| {
                cg.mul_i(&x,&*R28,&0x100);
                cg.add_i(&x,&*R29,&x);
                cg.assign(&sram(&x),&rr);
                cg.add_i(&x,&x,&1);
                vec!(x.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // ST Z
        [ opt!(skip), "1001 001 r@. r@.... 0000" ] = |st: &mut State<Avr>| {
            let rr = reg(st,"r");
            let x = new_temp(16);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic_dynargs(2,"st","{16::Z}",&|cg: &mut CodeGen<Avr>| {
                cg.mul_i(&x,&*R30,&0x100);
                cg.add_i(&x,&*R31,&x);
                cg.assign(&sram(&x),&rr);
                vec!(x.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // ST -Z
        [ opt!(skip), "1001 001 r@. r@.... 0010" ] = |st: &mut State<Avr>| {
            let rr = reg(st,"r");
            let x = new_temp(16);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic_dynargs(2,"st","{16::-Z}",&|cg: &mut CodeGen<Avr>| {
                cg.mul_i(&x,&*R30,&0x100);
                cg.add_i(&x,&*R31,&x);
                cg.sub_i(&x,&x,&1);
                cg.assign(&sram(&x),&rr);
                vec!(x.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // ST Z+
        [ opt!(skip), "1001 001 r@. r@.... 0001" ] = |st: &mut State<Avr>| {
            let rr = reg(st,"r");
            let x = new_temp(16);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic_dynargs(2,"st","{16::Z+}",&|cg: &mut CodeGen<Avr>| {
                cg.mul_i(&x,&*R30,&0x100);
                cg.add_i(&x,&*R31,&x);
                cg.assign(&sram(&x),&rr);
                cg.add_i(&x,&x,&1);
                vec!(x.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // STD *+q
        [ opt!(skip), "10 q@. 0 q@.. 1 r@..... x@. q@..." ] = |st: &mut State<Avr>| {
            let rr = reg(st,"r");
            let reg = st.get_group("x") == 1;
            let q = st.get_group("q");
            let x = new_temp(16);
            let t = if reg { "Y" } else { "Z" };
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic_dynargs(2,"std",&format!("{{16::{}+{}}}",t,q),&|cg: &mut CodeGen<Avr>| {
                if reg {
                    cg.mul_i(&x,&*R28,&0x100);
                    cg.add_i(&x,&*R29,&x);
                } else {
                    cg.mul_i(&x,&*R30,&0x100);
                    cg.add_i(&x,&*R31,&x);
                }

                cg.assign(&sram(&x),&rr);
                cg.add_i(&x,&x,&(q as u64));
                vec!(x.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // LD X
        [ opt!(skip), "1001 000 d@..... 1100" ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let x = new_temp(16);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic_dynargs(2,"ld","{8}, {16::X}",&|cg: &mut CodeGen<Avr>| {
                cg.mul_i(&x,&*R26,&0x100);
                cg.add_i(&x,&*R27,&x);
                cg.assign(&rd,&sram(&x).to_rv());
                vec!(rd.to_rv(),x.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // LD -X
        [ opt!(skip), "1001 000 d@..... 1110" ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let x = new_temp(16);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic_dynargs(2,"ld","{8}, {16::-X}",&|cg: &mut CodeGen<Avr>| {
                cg.mul_i(&x,&*R26,&0x100);
                cg.add_i(&x,&*R27,&x);
                cg.sub_i(&x,&x,&1);
                cg.assign(&rd,&sram(&x).to_rv());
                cg.mod_i(&*R26,&x,&0x100);
                cg.div_i(&*R27,&x,&0x100);

                vec!(rd.to_rv(),x.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // LD X+
        [ opt!(skip), "1001 000 d@..... 1101" ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let x = new_temp(16);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic_dynargs(2,"ld","{8}, {16::X+}",&|cg: &mut CodeGen<Avr>| {
                cg.mul_i(&x,&*R26,&0x100);
                cg.add_i(&x,&*R27,&x);
                cg.assign(&rd,&sram(&x).to_rv());
                cg.add_i(&x,&x,&1);
                cg.mod_i(&*R26,&x,&0x100);
                cg.div_i(&*R27,&x,&0x100);

                vec!(rd.to_rv(),x.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // LD Y
        [ opt!(skip), "1000 000 d@..... 1000" ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let x = new_temp(16);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic_dynargs(2,"ld","{8}, {16::Y}",&|cg: &mut CodeGen<Avr>| {
                cg.mul_i(&x,&*R28,&0x100);
                cg.add_i(&x,&*R29,&x);
                cg.assign(&rd,&sram(&x).to_rv());
                vec!(rd.to_rv(),x.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // LD -Y
        [ opt!(skip), "1001 000 d@..... 1010" ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let x = new_temp(16);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic_dynargs(2,"ld","{8}, {16::-Y}",&|cg: &mut CodeGen<Avr>| {
                cg.mul_i(&x,&*R28,&0x100);
                cg.add_i(&x,&*R29,&x);
                cg.sub_i(&x,&x,&1);
                cg.assign(&rd,&sram(&x).to_rv());
                cg.mod_i(&*R28,&x,&0x100);
                cg.div_i(&*R29,&x,&0x100);

                vec!(rd.to_rv(),x.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // LD Y+
        [ opt!(skip), "1001 000 d@..... 1001" ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let x = new_temp(16);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic_dynargs(2,"ld","{8}, {16::Y+}",&|cg: &mut CodeGen<Avr>| {
                cg.mul_i(&x,&*R28,&0x100);
                cg.add_i(&x,&*R29,&x);
                cg.assign(&rd,&sram(&x).to_rv());
                cg.add_i(&x,&x,&1);
                cg.mod_i(&*R28,&x,&0x100);
                cg.div_i(&*R29,&x,&0x100);

                vec!(rd.to_rv(),x.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // LD Z
        [ opt!(skip), "1000 000 d@..... 0000" ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let x = new_temp(16);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic_dynargs(2,"ld","{8}, {16::Z}",&|cg: &mut CodeGen<Avr>| {
                cg.mul_i(&x,&*R30,&0x100);
                cg.add_i(&x,&*R31,&x);
                cg.assign(&rd,&sram(&x).to_rv());
                vec!(rd.to_rv(),x.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // LD -Z
        [ opt!(skip), "1001 000 d@..... 0010" ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let x = new_temp(16);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic_dynargs(2,"ld","{8}, {16::-Z}",&|cg: &mut CodeGen<Avr>| {
                cg.mul_i(&x,&*R30,&0x100);
                cg.add_i(&x,&*R31,&x);
                cg.sub_i(&x,&x,&1);
                cg.assign(&rd,&sram(&x).to_rv());
                cg.mod_i(&*R30,&x,&0x100);
                cg.div_i(&*R31,&x,&0x100);

                vec!(rd.to_rv(),x.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // LD Z+
        [ opt!(skip), "1001 000 d@..... 0001" ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let x = new_temp(16);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic_dynargs(2,"ld","{8}, {16::Z+}",&|cg: &mut CodeGen<Avr>| {
                cg.mul_i(&x,&*R30,&0x100);
                cg.add_i(&x,&*R31,&x);
                cg.assign(&rd,&sram(&x).to_rv());
                cg.add_i(&x,&x,&1);
                cg.mod_i(&*R30,&x,&0x100);
                cg.div_i(&*R31,&x,&0x100);

                vec!(rd.to_rv(),x.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // LDD Y+q
        [ opt!(skip), "10 q@. 0 q@.. 0 d@..... 1 q@..." ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let _q = st.get_group("q");
            let q = Rvalue::Constant(_q as u64);
            let x = new_temp(16);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic_dynargs(2,"ldd",&format!("{{8}}, {{16::Y+{}}}",_q),&|cg: &mut CodeGen<Avr>| {
                cg.mul_i(&x,&*R28,&0x100);
                cg.add_i(&x,&*R29,&x);
                cg.assign(&rd,&sram(&x).to_rv());
                cg.add_i(&x,&x,&q);
                cg.mod_i(&*R28,&x,&0x100);
                cg.div_i(&*R29,&x,&0x100);

                vec!(rd.to_rv(),x.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // LDD Z+q
        [ opt!(skip), "10 q@. 0 q@.. 0 d@..... 0 q@..." ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let _q = st.get_group("q");
            let q = Rvalue::Constant(_q as u64);
            let x = new_temp(16);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic_dynargs(2,"ldd",&format!("{{8}}, {{16::Z+{}}}",_q),&|cg: &mut CodeGen<Avr>| {
                cg.mul_i(&x,&*R30,&0x100);
                cg.add_i(&x,&*R31,&x);
                cg.assign(&rd,&sram(&x).to_rv());
                cg.add_i(&x,&x,&q);
                cg.mod_i(&*R30,&x,&0x100);
                cg.div_i(&*R31,&x,&0x100);

                vec!(rd.to_rv(),x.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // BREAK
        [ opt!(skip), 0x9598 ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"break","",vec!(),&|_: &mut CodeGen<Avr>| {});
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // DES
        [ opt!(skip), "10010100 K@.... 1011" ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);
            let k = Rvalue::Constant(st.get_group("K") as u64);

            st.mnemonic(2,"des","{{4}}",vec!(k),&|cg: &mut CodeGen<Avr>| {
                cg.assign(&*R0,&Rvalue::Undefined);
                cg.assign(&*R1,&Rvalue::Undefined);
                cg.assign(&*R2,&Rvalue::Undefined);
                cg.assign(&*R3,&Rvalue::Undefined);
                cg.assign(&*R4,&Rvalue::Undefined);
                cg.assign(&*R5,&Rvalue::Undefined);
                cg.assign(&*R6,&Rvalue::Undefined);
                cg.assign(&*R7,&Rvalue::Undefined);
                cg.assign(&*R8,&Rvalue::Undefined);
                cg.assign(&*R9,&Rvalue::Undefined);
                cg.assign(&*R10,&Rvalue::Undefined);
                cg.assign(&*R11,&Rvalue::Undefined);
                cg.assign(&*R12,&Rvalue::Undefined);
                cg.assign(&*R13,&Rvalue::Undefined);
                cg.assign(&*R14,&Rvalue::Undefined);
                cg.assign(&*R15,&Rvalue::Undefined);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // EICALL
        [ opt!(skip), "1001 0101 0001 1001" ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);
            let z = new_temp(22);

            st.mnemonic_dynargs(2,"eicall","{16::Z}",&|cg: &mut CodeGen<Avr>| {
                let t = new_temp(22);

                cg.mul_i(&t,&*EIND,&0x10000);
                cg.mul_i(&z,&*R30,&0x100);
                cg.add_i(&z,&*R31,&z);
                cg.add_i(&z,&t,&z);

                cg.call_i(&Lvalue::Undefined,&z);
                vec!(z.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // EIJMP
        [ opt!(skip), "1001 0100 0001 1001" ] = |st: &mut State<Avr>| {
            let z = new_temp(22);
            st.mnemonic_dynargs(2,"eijmp","{22::Z:EIND}",&|cg: &mut CodeGen<Avr>| {
                let t = new_temp(22);

                cg.mul_i(&t,&*EIND,&0x10000);
                cg.mul_i(&z,&*R30,&0x100);
                cg.add_i(&z,&*R31,&z);
                cg.add_i(&z,&t,&z);
                vec!(z.to_rv())
            });
            optional_skip(st.configuration.wrap(st.address + st.tokens.len() as u64 * 2),st);
            st.jump(z.to_rv(),Guard::always());
            true
        },
        // ELPM
        [ opt!(skip), "1001 0101 1101 1000" ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"elpm","",vec!(),&|cg: &mut CodeGen<Avr>| {
                let z = new_temp(24);
                let t = new_temp(24);

                cg.mul_i(&t,&*RAMPZ,&0x10000);
                cg.mul_i(&z,&*R30,&0x100);
                cg.add_i(&z,&*R31,&z);
                cg.add_i(&z,&t,&z);

                cg.assign(&*R0,&flash(&z));
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // ELPM
        [ opt!(skip), "1001 000 d@..... 0110" ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic_dynargs(2,"elpm","{8}, {24::Z}",&|cg: &mut CodeGen<Avr>| {
                let z = new_temp(24);
                let t = new_temp(24);

                cg.mul_i(&t,&*RAMPZ,&0x10000);
                cg.mul_i(&z,&*R30,&0x100);
                cg.add_i(&z,&*R31,&z);
                cg.add_i(&z,&t,&z);

                cg.assign(&rd,&flash(&z));

                vec!(rd.to_rv(),z.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // ELPM
        [ opt!(skip), "1001 000 d@..... 0111" ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

             st.mnemonic_dynargs(2,"elpm","{8}, {24::Z+}",&|cg: &mut CodeGen<Avr>| {
                let z = new_temp(24);
                let t = new_temp(24);

                cg.mul_i(&t,&*RAMPZ,&0x10000);
                cg.mul_i(&z,&*R30,&0x100);
                cg.add_i(&z,&*R31,&z);
                cg.add_i(&z,&t,&z);

                cg.assign(&rd,&flash(&z));
                cg.add_i(&z,&z,&1);
                cg.mod_i(&*R31,&z,&0x100);
                cg.div_i(&*R30,&z,&0x100);
                cg.div_i(&*RAMPZ,&z,&0x10000);

                vec!(rd.to_rv(),z.to_rv())
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // EOR
        [ opt!(skip), "0010 01 r@. d@..... r@...." ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let rr = reg(st,"r");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"eor","{8}, {8}",vec!(rd.to_rv(),rr.to_rv()),&|cg: &mut CodeGen<Avr>| {
                cg.xor_i(&rd,&rd,&rr);
                cg.assign(&*V,&0);
                cg.equal_i(&*Z,&0,&rd);
                cg.less_i(&*N,&rd,&0x80);
                cg.not_b(&*N,&*N);
                cg.xor_b(&*S,&*N,&*V);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // FMUL
        [ opt!(skip), "0000 0011 0 d@... 1 r@..." ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let rr = reg(st,"r");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"fmul","{8}, {8}",vec!(rd.to_rv(),rr.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let r = new_temp(17);

                cg.mul_i(&r,&rd,&rr);
                cg.mul_i(&r,&r,&2);
                cg.less_i(&*C,&r,&0x8000);
                cg.equal_i(&*Z,&0,&r);

                cg.div_i(&*R1,&r,&0x100);
                cg.mod_i(&*R0,&r,&0x100);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // FMULS
        [ opt!(skip), "0000 0011 1 d@... 0 r@..." ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let rr = reg(st,"r");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"fmuls","{8}, {8}",vec!(rd.to_rv(),rr.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let r = new_temp(17);

                cg.mul_i(&r,&rd,&rr);
                cg.mul_i(&r,&r,&2);
                cg.less_i(&*C,&r,&0x8000);
                cg.equal_i(&*Z,&0,&r);

                cg.div_i(&*R1,&r,&0x100);
                cg.mod_i(&*R0,&r,&0x100);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // FMULSU
        [ opt!(skip), "0000 0011 1 d@... 1 r@..." ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let rr = reg(st,"r");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"fmulsu","{8}, {8}",vec!(rd.to_rv(),rr.to_rv()),&|cg: &mut CodeGen<Avr>| {
                let r = new_temp(17);

                cg.mul_i(&r,&rd,&rr);
                cg.mul_i(&r,&r,&2);
                cg.less_i(&*C,&r,&0x8000);
                cg.equal_i(&*Z,&0,&r);

                cg.div_i(&*R1,&r,&0x100);
                cg.mod_i(&*R0,&r,&0x100);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // NOP
        [ opt!(skip), 0x0 ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);
            st.mnemonic(2,"nop","",vec!(),&|_: &mut CodeGen<Avr>| {});
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // OR
        [ opt!(skip), "0010 10 r@. d@..... r@...." ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let rr = reg(st,"r");
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"or","{8}, {8}",vec!(rd.to_rv(),rr.to_rv()),&|cg: &mut CodeGen<Avr>| {
                cg.or_i(&rd,&rd,&rr);
                cg.assign(&*V,&0);
                cg.equal_i(&*Z,&0,&rd);
                cg.less_i(&*N,&rd,&0x80);
                cg.not_b(&*N,&*N);
                cg.xor_b(&*S,&*N,&*V);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // ORI
        [ opt!(skip), "0110 K@.... d@.... K@...." ] = |st: &mut State<Avr>| {
            let rd = reg(st,"d");
            let _k = st.get_group("K");
            let k = Rvalue::Constant(_k as u64);
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"ori","{8}, {8}",vec!(rd.to_rv(),k.clone()),&|cg: &mut CodeGen<Avr>| {
                cg.or_i(&rd,&rd,&k);
                cg.assign(&*V,&0);
                cg.equal_i(&*Z,&0,&rd);
                cg.less_i(&*N,&rd,&0x80);
                cg.not_b(&*N,&*N);
                cg.xor_b(&*S,&*N,&*V);
            });
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // SLEEP
        [ opt!(skip), 0x9588 ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"sleep","",vec!(),&|_: &mut CodeGen<Avr>| {});
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // WDR
        [ opt!(skip), 0x95a8 ] = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(2,"wdr","",vec!(),&|_: &mut CodeGen<Avr>| {});
            optional_skip(next.clone(),st);
            st.jump(next,Guard::always());
            true
        },
        // catch all
        _ = |st: &mut State<Avr>| {
            let next = st.configuration.wrap(st.address + st.tokens.len() as u64 * 2);

            st.mnemonic(1,"unk","",vec!(),&|_: &mut CodeGen<Avr>| {});
            st.jump(next,Guard::always());
            true
        }
    );

    new_disassembler!(Avr =>
        [ main ] = |_: &mut State<Avr>| { true },
        [ skip ] = |_: &mut State<Avr>| { true }
    )
}
