macro_rules! rreil {
    ( $cg:ident : ) => {};
    ( $cg:ident : add $($cdr:tt)* ) => { rreil_binop!($cg : add # $($cdr)*); };
    ( $cg:ident : sub $($cdr:tt)* ) => { rreil_binop!($cg : sub # $($cdr)*); };
    ( $cg:ident : mul $($cdr:tt)* ) => { rreil_binop!($cg : mul # $($cdr)*); };
    ( $cg:ident : div $($cdr:tt)* ) => { rreil_binop!($cg : div # $($cdr)*); };
    ( $cg:ident : divs $($cdr:tt)* ) => { rreil_binop!($cg : divs # $($cdr)*); };
    ( $cg:ident : shl $($cdr:tt)* ) => { rreil_binop!($cg : shl # $($cdr)*); };
    ( $cg:ident : shr $($cdr:tt)* ) => { rreil_binop!($cg : shr # $($cdr)*); };
    ( $cg:ident : shrs $($cdr:tt)* ) => { rreil_binop!($cg : shrs # $($cdr)*); };
    ( $cg:ident : mod $($cdr:tt)* ) => { rreil_binop!($cg : modu # $($cdr)*); };
    ( $cg:ident : and $($cdr:tt)* ) => { rreil_binop!($cg : and # $($cdr)*); };
    ( $cg:ident : xor $($cdr:tt)* ) => { rreil_binop!($cg : xor # $($cdr)*); };
    ( $cg:ident : or $($cdr:tt)* ) => { rreil_binop!($cg : or # $($cdr)*); };

    ( $cg:ident : cmpeq $($cdr:tt)* ) => { rreil_binop!($cg : cmpeq # $($cdr)*); };
    ( $cg:ident : cmpleu $($cdr:tt)* ) => { rreil_binop!($cg : cmpleu # $($cdr)*); };
    ( $cg:ident : cmples $($cdr:tt)* ) => { rreil_binop!($cg : cmples # $($cdr)*); };
    ( $cg:ident : cmpltu $($cdr:tt)* ) => { rreil_binop!($cg : cmpltu # $($cdr)*); };
    ( $cg:ident : cmplts $($cdr:tt)* ) => { rreil_binop!($cg : cmplts # $($cdr)*); };

    ( $cg:ident : sign-extend $($cdr:tt)* ) => { rreil_unop!($cg : sign_extend # $($cdr)*); };
    ( $cg:ident : mov $($cdr:tt)* ) => { rreil_unop!($cg : mov # $($cdr)*); };
    ( $cg:ident : call $($cdr:tt)* ) => { rreil_unop!($cg : call # $($cdr)*); };
}

macro_rules! rreil_binop {
    // lit lit lit
    ($cg:ident : $o:ident # ( $a:expr ) : $a_w:tt, ( $b:expr ) : $b_w:tt, ( $c:expr ) : $c_w:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(($a),rreil_imm!($a_w),0,
                  ($b),rreil_imm!($b_w),0,
                  ($c),rreil_imm!($c_w),0);
            rreil!($cg : $($cdr)*);
        }
    };
    // lit lit off
    ($cg:ident : $o:ident # ( $a:expr ) : $a_w:tt, ( $b:expr ) : $b_w:tt, $c:tt : $c_w:tt / $c_o:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(($a),rreil_imm!($a_w),0,
                  ($b),rreil_imm!($b_w),0,
                  stringify!($c),rreil_imm!($c_w),rreil_imm!($c_o));
            rreil!($cg : $($cdr)*);
        }
    };
    // lit lit noff
    ($cg:ident : $o:ident # ( $a:expr ) : $a_w:tt, ( $b:expr ) : $b_w:tt, $c:tt : $c_w:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(($a),rreil_imm!($a_w),0,
                  ($b),rreil_imm!($b_w),0,
                  stringify!($c),rreil_imm!($c_w),0);
            rreil!($cg : $($cdr)*);
        }
    };
    // lit off lit
    ($cg:ident : $o:ident # ( $a:expr ) : $a_w:tt, $b:tt : $b_w:tt / $b_o:tt, ( $c:expr ) : $c_w:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(($a),rreil_imm!($a_w),0,
                  stringify!($b),rreil_imm!($b_w),rreil_imm!($b_o),
                  ($c),rreil_imm!($c_w),0);
            rreil!($cg : $($cdr)*);
        }
    };
    // lit off off
    ($cg:ident : $o:ident # ( $a:expr ) : $a_w:tt, $b:tt : $b_w:tt / $b_o:tt, $c:tt : $c_w:tt / $c_o:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(($a),rreil_imm!($a_w),0,
                  stringify!($b),rreil_imm!($b_w),rreil_imm!($b_o),
                  stringify!($c),rreil_imm!($c_w),rreil_imm!($c_o));
            rreil!($cg : $($cdr)*);
        }
    };
    // lit off noff
    ($cg:ident : $o:ident # ( $a:expr ) : $a_w:tt, $b:tt : $b_w:tt / $b_o:tt, $c:tt : $c_w:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(($a),rreil_imm!($a_w),0,
                  stringify!($b),rreil_imm!($b_w),rreil_imm!($b_o),
                  stringify!($c),rreil_imm!($c_w),0);
            rreil!($cg : $($cdr)*);
        }
    };
    // lit noff lit
    ($cg:ident : $o:ident # ( $a:expr ) : $a_w:tt, $b:tt : $b_w:tt, ( $c:expr ) : $c_w:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(($a),rreil_imm!($a_w),0,
                  stringify!($b),rreil_imm!($b_w),0,
                  ($c),rreil_imm!($c_w),0);
            rreil!($cg : $($cdr)*);
        }
    };
    // lit noff off
    ($cg:ident : $o:ident # ( $a:expr ) : $a_w:tt, $b:tt : $b_w:tt, $c:tt : $c_w:tt / $c_o:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(($a),rreil_imm!($a_w),0,
                  stringify!($b),rreil_imm!($b_w),0,
                  stringify!($c),rreil_imm!($c_w),rreil_imm!($c_o));
            rreil!($cg : $($cdr)*);
        }
    };
    // lit noff noff
    ($cg:ident : $o:ident # ( $a:expr ) : $a_w:tt, $b:tt : $b_w:tt, $c:tt : $c_w:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(($a),rreil_imm!($a_w),0,
                  stringify!($b),rreil_imm!($b_w),0,
                  stringify!($c),rreil_imm!($c_w),0);
            rreil!($cg : $($cdr)*);
        }
    };
    // off lit lit
    ($cg:ident : $o:ident # $a:tt : $a_w:tt / $a_o:tt, ( $b:expr ) : $b_w:tt, ( $c:expr ) : $c_w:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(stringify!($a),rreil_imm!($a_w),rreil_imm!($a_o),
                  ($b),rreil_imm!($b_w),0,
                  ($c),rreil_imm!($c_w),0);
            rreil!($cg : $($cdr)*);
        }
    };
    // off lit off
    ($cg:ident : $o:ident # $a:tt : $a_w:tt / $a_o:tt, ( $b:expr ) : $b_w:tt, $c:tt : $c_w:tt / $c_o:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(stringify!($a),rreil_imm!($a_w),rreil_imm!($a_o),
                  ($b),rreil_imm!($b_w),0,
                  stringify!($c),rreil_imm!($c_w),rreil_imm!($c_o));
            rreil!($cg : $($cdr)*);
        }
    };
    // off lit noff
    ($cg:ident : $o:ident # $a:tt : $a_w:tt / $a_o:tt, ( $b:expr ) : $b_w:tt, $c:tt : $c_w:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(stringify!($a),rreil_imm!($a_w),rreil_imm!($a_o),
                  ($b),rreil_imm!($b_w),0,
                  stringify!($c),rreil_imm!($c_w),0);
            rreil!($cg : $($cdr)*);
        }
    };
    // off off lit
    ($cg:ident : $o:ident # $a:tt : $a_w:tt / $a_o:tt, $b:tt : $b_w:tt / $b_o:tt, ( $c:expr ) : $c_w:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(stringify!($a),rreil_imm!($a_w),rreil_imm!($a_o),
                  stringify!($b),rreil_imm!($b_w),rreil_imm!($b_o),
                  ($c),rreil_imm!($c_w),0);
            rreil!($cg : $($cdr)*);
        }
    };
    // off off off
    ($cg:ident : $o:ident # $a:tt : $a_w:tt / $a_o:tt, $b:tt : $b_w:tt / $b_o:tt, $c:tt : $c_w:tt / $c_o:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(stringify!($a),rreil_imm!($a_w),rreil_imm!($a_o),
                  stringify!($b),rreil_imm!($b_w),rreil_imm!($b_o),
                  stringify!($c),rreil_imm!($c_w),rreil_imm!($c_o));
            rreil!($cg : $($cdr)*);
        }
    };
    // off off noff
    ($cg:ident : $o:ident # $a:tt : $a_w:tt / $a_o:tt, $b:tt : $b_w:tt / $b_o:tt, $c:tt : $c_w:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(stringify!($a),rreil_imm!($a_w),rreil_imm!($a_o),
                  stringify!($b),rreil_imm!($b_w),rreil_imm!($b_o),
                  stringify!($c),rreil_imm!($c_w),0);
            rreil!($cg : $($cdr)*);
        }
    };
    // off noff lit
    ($cg:ident : $o:ident # $a:tt : $a_w:tt / $a_o:tt, $b:tt : $b_w:tt, ( $c:expr ) : $c_w:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(stringify!($a),rreil_imm!($a_w),rreil_imm!($a_o),
                  stringify!($b),rreil_imm!($b_w),0,
                  ($c),rreil_imm!($c_w),0);
            rreil!($cg : $($cdr)*);
        }
    };
    // off noff off
    ($cg:ident : $o:ident # $a:tt : $a_w:tt / $a_o:tt, $b:tt : $b_w:tt, $c:tt : $c_w:tt / $c_o:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(stringify!($a),rreil_imm!($a_w),rreil_imm!($a_o),
                  stringify!($b),rreil_imm!($b_w),0,
                  stringify!($c),rreil_imm!($c_w),rreil_imm!($c_o));
            rreil!($cg : $($cdr)*);
        }
    };
    // off noff noff
    ($cg:ident : $o:ident # $a:tt : $a_w:tt / $a_o:tt, $b:tt : $b_w:tt, $c:tt : $c_w:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(stringify!($a),rreil_imm!($a_w),rreil_imm!($a_o),
                  stringify!($b),rreil_imm!($b_w),0,
                  stringify!($c),rreil_imm!($c_w),0);
            rreil!($cg : $($cdr)*);
        }
    };
    // noff lit lit
    ($cg:ident : $o:ident # $a:tt : $a_w:tt, ( $b:expr ) : $b_w:tt, ( $c:expr ) : $c_w:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(stringify!($a),rreil_imm!($a_w),0,
                  ($b),rreil_imm!($b_w),0,
                  ($c),rreil_imm!($c_w),0);
            rreil!($cg : $($cdr)*);
        }
    };
    // noff lit off
    ($cg:ident : $o:ident # $a:tt : $a_w:tt, ( $b:expr ) : $b_w:tt, $c:tt : $c_w:tt / $c_o:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(stringify!($a),rreil_imm!($a_w),0,
                  ($b),rreil_imm!($b_w),0,
                  stringify!($c),rreil_imm!($c_w),rreil_imm!($c_o));
            rreil!($cg : $($cdr)*);
        }
    };
    // noff lit noff
    ($cg:ident : $o:ident # $a:tt : $a_w:tt, ( $b:expr ) : $b_w:tt, $c:tt : $c_w:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(stringify!($a),rreil_imm!($a_w),0,
                  ($b),rreil_imm!($b_w),0,
                  stringify!($c),rreil_imm!($c_w),0);
            rreil!($cg : $($cdr)*);
        }
    };
    // noff off lit
    ($cg:ident : $o:ident # $a:tt : $a_w:tt, $b:tt : $b_w:tt / $b_o:tt, ( $c:expr ) : $c_w:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(stringify!($a),rreil_imm!($a_w),0,
                  stringify!($b),rreil_imm!($b_w),rreil_imm!($b_o),
                  ($c),rreil_imm!($c_w),0);
            rreil!($cg : $($cdr)*);
        }
    };
    // noff off off
    ($cg:ident : $o:ident # $a:tt : $a_w:tt, $b:tt : $b_w:tt / $b_o:tt, $c:tt : $c_w:tt / $c_o:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(stringify!($a),rreil_imm!($a_w),0,
                  stringify!($b),rreil_imm!($b_w),rreil_imm!($b_o),
                  stringify!($c),rreil_imm!($c_w),rreil_imm!($c_o));
            rreil!($cg : $($cdr)*);
        }
    };
    // noff off noff
    ($cg:ident : $o:ident # $a:tt : $a_w:tt, $b:tt : $b_w:tt / $b_o:tt, $c:tt : $c_w:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(stringify!($a),rreil_imm!($a_w),0,
                  stringify!($b),rreil_imm!($b_w),rreil_imm!($b_o),
                  stringify!($c),rreil_imm!($c_w),0);
            rreil!($cg : $($cdr)*);
        }
    };
    // noff noff lit
    ($cg:ident : $o:ident # $a:tt : $a_w:tt, $b:tt : $b_w:tt, ( $c:expr ) : $c_w:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(stringify!($a),rreil_imm!($a_w),0,
                  stringify!($b),rreil_imm!($b_w),0,
                  ($c),rreil_imm!($c_w),0);
            rreil!($cg : $($cdr)*);
        }
    };
    // noff noff off
    ($cg:ident : $o:ident # $a:tt : $a_w:tt, $b:tt : $b_w:tt, $c:tt : $c_w:tt / $c_o:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(stringify!($a),rreil_imm!($a_w),0,
                  stringify!($b),rreil_imm!($b_w),0,
                  stringify!($c),rreil_imm!($c_w),rreil_imm!($c_o));
            rreil!($cg : $($cdr)*);
        }
    };
    // noff noff noff
    ($cg:ident : $o:ident # $a:tt : $a_w:tt, $b:tt : $b_w:tt, $c:tt : $c_w:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(stringify!($a),rreil_imm!($a_w),0,
                  stringify!($b),rreil_imm!($b_w),0,
                  stringify!($c),rreil_imm!($c_w),0);
            rreil!($cg : $($cdr)*);
        }
    };
}

macro_rules! rreil_unop {
    // lit lit
    ($cg:ident : $o:ident # ( $a:expr ) : $a_w:tt, ( $b:expr ) : $b_w:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(($a),rreil_imm!($a_w),0,
                  ($b),rreil_imm!($b_w),0);
            rreil!($cg : $($cdr)*);
        }
    };
    // lit off
    ($cg:ident : $o:ident # ( $a:expr ) : $a_w:tt, $b:tt : $b_w:tt / $b_o:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(($a),rreil_imm!($a_w),0,
                  stringify!($b),rreil_imm!($b_w),rreil_imm!($b_o));
            rreil!($cg : $($cdr)*);
        }
    };
    // lit noff
    ($cg:ident : $o:ident # ( $a:expr ) : $a_w:tt, $b:tt : $b_w:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(($a),rreil_imm!($a_w),0,
                  stringify!($b),rreil_imm!($b_w),0);
            rreil!($cg : $($cdr)*);
        }
    };
    // off lit
    ($cg:ident : $o:ident # $a:tt : $a_w:tt / $a_o:tt, ( $b:expr ) : $b_w:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(stringify!($a),rreil_imm!($a_w),rreil_imm!($a_o),
                  ($b),rreil_imm!($b_w),0);
            rreil!($cg : $($cdr)*);
        }
    };
    // off off
    ($cg:ident : $o:ident # $a:tt : $a_w:tt / $a_o:tt, $b:tt : $b_w:tt / $b_o:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(stringify!($a),rreil_imm!($a_w),rreil_imm!($a_o),
                  stringify!($b),rreil_imm!($b_w),rreil_imm!($b_o));
            rreil!($cg : $($cdr)*);
        }
    };
    // off noff
    ($cg:ident : $o:ident # $a:tt : $a_w:tt / $a_o:tt, $b:tt : $b_w:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(stringify!($a),rreil_imm!($a_w),rreil_imm!($a_o),
                  stringify!($b),rreil_imm!($b_w),0);
            rreil!($cg : $($cdr)*);
        }
    };
    // noff lit
    ($cg:ident : $o:ident # $a:tt : $a_w:tt, ( $b:expr ) : $b_w:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(stringify!($a),rreil_imm!($a_w),0,
                  ($b),rreil_imm!($b_w),0);
            rreil!($cg : $($cdr)*);
        }
    };
    // noff off
    ($cg:ident : $o:ident # $a:tt : $a_w:tt, $b:tt : $b_w:tt / $b_o:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(stringify!($a),rreil_imm!($a_w),0,
                  stringify!($b),rreil_imm!($b_w),rreil_imm!($b_o));
            rreil!($cg : $($cdr)*);
        }
    };
    // noff noff
    ($cg:ident : $o:ident # $a:tt : $a_w:tt, $b:tt : $b_w:tt ; $($cdr:tt)*) => {
        {
            $cg.$o(stringify!($a),rreil_imm!($a_w),0,
                  stringify!($b),rreil_imm!($b_w),0);
            rreil!($cg : $($cdr)*);
        }
    };
}

macro_rules! rreil_imm {
    ($x:expr) => ($x as u64);
}

struct CG;

impl CG {
    pub fn add(&mut self, a: &str, aw: u64, ao: u64, b: &str, bw: u64, bo: u64, c: &str, cw: u64, co: u64) {
        unimplemented!()
    }

    pub fn and(&mut self, a: &str, aw: u64, ao: u64, b: &str, bw: u64, bo: u64, c: &str, cw: u64, co: u64) {
        unimplemented!()
    }

    pub fn sub(&mut self, a: &str, aw: u64, ao: u64, b: &str, bw: u64, bo: u64, c: &str, cw: u64, co: u64) {
        unimplemented!()
    }

    pub fn shr(&mut self, a: &str, aw: u64, ao: u64, b: &str, bw: u64, bo: u64, c: &str, cw: u64, co: u64) {
        unimplemented!()
    }

    pub fn xor(&mut self, a: &str, aw: u64, ao: u64, b: &str, bw: u64, bo: u64, c: &str, cw: u64, co: u64) {
        unimplemented!()
    }

    pub fn mov(&mut self, a: &str, aw: u64, ao: u64, b: &str, bw: u64, bo: u64) {
        unimplemented!()
    }
}

fn main () {
    let mut cg = CG;
    rreil!{
        cg:
        add ("t0") : 32 , ("2147483648") : 32, ("eax") : 32;
        and t0 : 32 , 2147483648 : 32, eax : 32;
        and t1 : 32 , 2147483648 : 32, ebx : 32;
        sub t2 : 64 , ebx : 32 , eax : 32;
        and t3 : 32 , 2147483648 : 32, t2 : 64;
        shr SF : 8 , 31 : 32 , t3 : 32;
        xor t4 : 32 , t1 : 32 , t0 : 32;
        xor t5 : 32 , t3 : 32 , t0 : 32;
        and t6 : 32 , t5 : 32 , t4 : 32;
        shr OF : 8 , 31 : 32 , t6 : 32;
        and t7 : 64 , 4294967296 : 64, t2 : 64;
        shr CF : 8 , 32 : 32 , t7 : 64;
        and t8 : 32 , 4294967295 : 64, t2 : 64;
        xor t9 : 8 , OF : 8 , SF : 8;
    }
}
