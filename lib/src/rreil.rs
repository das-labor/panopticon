macro_rules! rreil_binop {
    // lit := noff, noff
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := noff, off
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := noff, lit
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := noff, litw
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := noff, const
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := noff, undef
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := off, noff
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := off, off
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := off, lit
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := off, litw
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := off, const
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := off, undef
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := lit, noff
    ( $op:ident # ( $a:expr ), ( $x:expr ) , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := lit, off
    ( $op:ident # ( $a:expr ), ( $x:expr ) , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := lit, lit
    ( $op:ident # ( $a:expr ), ( $x:expr ) , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := lit, litw
    ( $op:ident # ( $a:expr ), ( $x:expr ) , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := lit, const
    ( $op:ident # ( $a:expr ), ( $x:expr ) , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := lit, undef
    ( $op:ident # ( $a:expr ), ( $x:expr ) , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := litw, noff
    ( $op:ident # ( $a:expr ), ( $x:expr ) : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := litw, off
    ( $op:ident # ( $a:expr ), ( $x:expr ) : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := litw, lit
    ( $op:ident # ( $a:expr ), ( $x:expr ) : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := litw, litw
    ( $op:ident # ( $a:expr ), ( $x:expr ) : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := litw, const
    ( $op:ident # ( $a:expr ), ( $x:expr ) : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := litw, undef
    ( $op:ident # ( $a:expr ), ( $x:expr ) : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := const, noff
    ( $op:ident # ( $a:expr ), [ $x:tt ] : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := const, off
    ( $op:ident # ( $a:expr ), [ $x:tt ] : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := const, lit
    ( $op:ident # ( $a:expr ), [ $x:tt ] : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := const, litw
    ( $op:ident # ( $a:expr ), [ $x:tt ] : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := const, const
    ( $op:ident # ( $a:expr ), [ $x:tt ] : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := const, undef
    ( $op:ident # ( $a:expr ), [ $x:tt ] : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := undef, noff
    ( $op:ident # ( $a:expr ), ? , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := undef, off
    ( $op:ident # ( $a:expr ), ? , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := undef, lit
    ( $op:ident # ( $a:expr ), ? , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := undef, litw
    ( $op:ident # ( $a:expr ), ? , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := undef, const
    ( $op:ident # ( $a:expr ), ? , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // lit := undef, undef
    ( $op:ident # ( $a:expr ), ? , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := noff, noff
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := noff, off
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := noff, lit
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := noff, litw
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := noff, const
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := noff, undef
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := off, noff
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := off, off
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := off, lit
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := off, litw
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := off, const
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := off, undef
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := lit, noff
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := lit, off
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := lit, lit
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := lit, litw
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := lit, const
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := lit, undef
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := litw, noff
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := litw, off
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := litw, lit
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := litw, litw
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := litw, const
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := litw, undef
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := const, noff
    ( $op:ident # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := const, off
    ( $op:ident # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := const, lit
    ( $op:ident # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := const, litw
    ( $op:ident # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := const, const
    ( $op:ident # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := const, undef
    ( $op:ident # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := undef, noff
    ( $op:ident # ( $a:expr ) : $a_w:tt, ? , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := undef, off
    ( $op:ident # ( $a:expr ) : $a_w:tt, ? , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := undef, lit
    ( $op:ident # ( $a:expr ) : $a_w:tt, ? , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := undef, litw
    ( $op:ident # ( $a:expr ) : $a_w:tt, ? , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := undef, const
    ( $op:ident # ( $a:expr ) : $a_w:tt, ? , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // litw := undef, undef
    ( $op:ident # ( $a:expr ) : $a_w:tt, ? , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := noff, noff
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := noff, off
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := noff, lit
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := noff, litw
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := noff, const
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := noff, undef
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := off, noff
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := off, off
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := off, lit
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := off, litw
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := off, const
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := off, undef
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := lit, noff
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := lit, off
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := lit, lit
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := lit, litw
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := lit, const
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := lit, undef
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := litw, noff
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := litw, off
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := litw, lit
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := litw, litw
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := litw, const
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := litw, undef
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := const, noff
    ( $op:ident # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := const, off
    ( $op:ident # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := const, lit
    ( $op:ident # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := const, litw
    ( $op:ident # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := const, const
    ( $op:ident # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := const, undef
    ( $op:ident # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := undef, noff
    ( $op:ident # $a:tt : $a_w:tt, ? , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := undef, off
    ( $op:ident # $a:tt : $a_w:tt, ? , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := undef, lit
    ( $op:ident # $a:tt : $a_w:tt, ? , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := undef, litw
    ( $op:ident # $a:tt : $a_w:tt, ? , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := undef, const
    ( $op:ident # $a:tt : $a_w:tt, ? , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // noff := undef, undef
    ( $op:ident # $a:tt : $a_w:tt, ? , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := noff, noff
    ( $op:ident # ?, $x:tt : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := noff, off
    ( $op:ident # ?, $x:tt : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := noff, lit
    ( $op:ident # ?, $x:tt : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := noff, litw
    ( $op:ident # ?, $x:tt : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := noff, const
    ( $op:ident # ?, $x:tt : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := noff, undef
    ( $op:ident # ?, $x:tt : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := off, noff
    ( $op:ident # ?, $x:tt : $x_w:tt / $x_o:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := off, off
    ( $op:ident # ?, $x:tt : $x_w:tt / $x_o:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := off, lit
    ( $op:ident # ?, $x:tt : $x_w:tt / $x_o:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := off, litw
    ( $op:ident # ?, $x:tt : $x_w:tt / $x_o:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := off, const
    ( $op:ident # ?, $x:tt : $x_w:tt / $x_o:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := off, undef
    ( $op:ident # ?, $x:tt : $x_w:tt / $x_o:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := lit, noff
    ( $op:ident # ?, ( $x:expr ) , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := lit, off
    ( $op:ident # ?, ( $x:expr ) , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := lit, lit
    ( $op:ident # ?, ( $x:expr ) , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := lit, litw
    ( $op:ident # ?, ( $x:expr ) , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := lit, const
    ( $op:ident # ?, ( $x:expr ) , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := lit, undef
    ( $op:ident # ?, ( $x:expr ) , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := litw, noff
    ( $op:ident # ?, ( $x:expr ) : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := litw, off
    ( $op:ident # ?, ( $x:expr ) : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := litw, lit
    ( $op:ident # ?, ( $x:expr ) : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := litw, litw
    ( $op:ident # ?, ( $x:expr ) : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := litw, const
    ( $op:ident # ?, ( $x:expr ) : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := litw, undef
    ( $op:ident # ?, ( $x:expr ) : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := const, noff
    ( $op:ident # ?, [ $x:tt ] : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := const, off
    ( $op:ident # ?, [ $x:tt ] : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := const, lit
    ( $op:ident # ?, [ $x:tt ] : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := const, litw
    ( $op:ident # ?, [ $x:tt ] : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := const, const
    ( $op:ident # ?, [ $x:tt ] : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := const, undef
    ( $op:ident # ?, [ $x:tt ] : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := undef, noff
    ( $op:ident # ?, ? , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := undef, off
    ( $op:ident # ?, ? , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := undef, lit
    ( $op:ident # ?, ? , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := undef, litw
    ( $op:ident # ?, ? , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := undef, const
    ( $op:ident # ?, ? , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

    // undef := undef, undef
    ( $op:ident # ?, ? , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!($($cdr)*));
            stmt
        }
    };

}

macro_rules! rreil_unop {
    // lit := noff
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // lit := off
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // lit := lit
    ( $op:ident # ( $a:expr ), ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // lit := litw
    ( $op:ident # ( $a:expr ), ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // lit := const
    ( $op:ident # ( $a:expr ), [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // lit := undef
    ( $op:ident # ( $a:expr ), ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // litw := noff
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // litw := off
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // litw := lit
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // litw := litw
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // litw := const
    ( $op:ident # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // litw := undef
    ( $op:ident # ( $a:expr ) : $a_w:tt, ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // noff := noff
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // noff := off
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // noff := lit
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ))), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // noff := litw
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // noff := const
    ( $op:ident # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // noff := undef
    ( $op:ident # $a:tt : $a_w:tt, ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // undef := noff
    ( $op:ident # ?, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // undef := off
    ( $op:ident # ?, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // undef := lit
    ( $op:ident # ?, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // undef := litw
    ( $op:ident # ?, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // undef := const
    ( $op:ident # ?, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // undef := undef
    ( $op:ident # ?, ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

}

macro_rules! rreil_memop {
    // lit := noff
    ( $op:ident # $bank:ident # ( $a:expr ), $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // lit := off
    ( $op:ident # $bank:ident # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // lit := lit
    ( $op:ident # $bank:ident # ( $a:expr ), ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // lit := litw
    ( $op:ident # $bank:ident # ( $a:expr ), ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // lit := const
    ( $op:ident # $bank:ident # ( $a:expr ), [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // lit := undef
    ( $op:ident # $bank:ident # ( $a:expr ), ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // litw := noff
    ( $op:ident # $bank:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // litw := off
    ( $op:ident # $bank:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // litw := lit
    ( $op:ident # $bank:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // litw := litw
    ( $op:ident # $bank:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // litw := const
    ( $op:ident # $bank:ident # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // litw := undef
    ( $op:ident # $bank:ident # ( $a:expr ) : $a_w:tt, ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // noff := noff
    ( $op:ident # $bank:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // noff := off
    ( $op:ident # $bank:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // noff := lit
    ( $op:ident # $bank:ident # $a:tt : $a_w:tt, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // noff := litw
    ( $op:ident # $bank:ident # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // noff := const
    ( $op:ident # $bank:ident # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // noff := undef
    ( $op:ident # $bank:ident # $a:tt : $a_w:tt, ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // undef := noff
    ( $op:ident # $bank:ident # ?, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // undef := off
    ( $op:ident # $bank:ident # ?, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // undef := lit
    ( $op:ident # $bank:ident # ?, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // undef := litw
    ( $op:ident # $bank:ident # ?, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // undef := const
    ( $op:ident # $bank:ident # ?, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // undef := undef
    ( $op:ident # $bank:ident # ?, ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

}

macro_rules! rreil_extop {
    // lit := noff
    ( $op:ident # $sz:tt # ( $a:expr ), $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // lit := off
    ( $op:ident # $sz:tt # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // lit := lit
    ( $op:ident # $sz:tt # ( $a:expr ), ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // lit := litw
    ( $op:ident # $sz:tt # ( $a:expr ), ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // lit := const
    ( $op:ident # $sz:tt # ( $a:expr ), [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // lit := undef
    ( $op:ident # $sz:tt # ( $a:expr ), ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // litw := noff
    ( $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // litw := off
    ( $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // litw := lit
    ( $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // litw := litw
    ( $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // litw := const
    ( $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // litw := undef
    ( $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // noff := noff
    ( $op:ident # $sz:tt # $a:tt : $a_w:tt, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // noff := off
    ( $op:ident # $sz:tt # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // noff := lit
    ( $op:ident # $sz:tt # $a:tt : $a_w:tt, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // noff := litw
    ( $op:ident # $sz:tt # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // noff := const
    ( $op:ident # $sz:tt # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // noff := undef
    ( $op:ident # $sz:tt # $a:tt : $a_w:tt, ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // undef := noff
    ( $op:ident # $sz:tt # ?, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // undef := off
    ( $op:ident # $sz:tt # ?, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // undef := lit
    ( $op:ident # $sz:tt # ?, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // undef := litw
    ( $op:ident # $sz:tt # ?, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // undef := const
    ( $op:ident # $sz:tt # ?, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // undef := undef
    ( $op:ident # $sz:tt # ?, ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

}

macro_rules! rreil_selop {
    // lit := noff
    ( $op:ident # $sz:tt # ( $a:expr ), $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a )),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // lit := off
    ( $op:ident # $sz:tt # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a )),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // lit := lit
    ( $op:ident # $sz:tt # ( $a:expr ), ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a )),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // lit := litw
    ( $op:ident # $sz:tt # ( $a:expr ), ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a )),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // lit := const
    ( $op:ident # $sz:tt # ( $a:expr ), [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a )),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // lit := undef
    ( $op:ident # $sz:tt # ( $a:expr ), ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a )),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // litw := noff
    ( $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a ) : $a_w),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // litw := off
    ( $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a ) : $a_w),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // litw := lit
    ( $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a ) : $a_w),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // litw := litw
    ( $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a ) : $a_w),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // litw := const
    ( $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a ) : $a_w),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // litw := undef
    ( $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a ) : $a_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // noff := noff
    ( $op:ident # $sz:tt # $a:tt : $a_w:tt, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($a : $a_w),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // noff := off
    ( $op:ident # $sz:tt # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($a : $a_w),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // noff := lit
    ( $op:ident # $sz:tt # $a:tt : $a_w:tt, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($a : $a_w),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // noff := litw
    ( $op:ident # $sz:tt # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($a : $a_w),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // noff := const
    ( $op:ident # $sz:tt # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($a : $a_w),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // noff := undef
    ( $op:ident # $sz:tt # $a:tt : $a_w:tt, ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($a : $a_w),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // undef := noff
    ( $op:ident # $sz:tt # ?, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // undef := off
    ( $op:ident # $sz:tt # ?, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // undef := lit
    ( $op:ident # $sz:tt # ?, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // undef := litw
    ( $op:ident # $sz:tt # ?, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // undef := const
    ( $op:ident # $sz:tt # ?, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

    // undef := undef
    ( $op:ident # $sz:tt # ?, ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)}];
            let _ = try!(stmt[0].sanity_check());
            stmt.append(&mut rreil!( $($cdr)*));
            stmt
        }
    };

}

