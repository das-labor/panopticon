macro_rules! rreil_binop {
    // lit := noff, noff
    ($cg:ident : $op:ident # ( $a:expr ), $x:tt : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := noff, off
    ($cg:ident : $op:ident # ( $a:expr ), $x:tt : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := noff, lit
    ($cg:ident : $op:ident # ( $a:expr ), $x:tt : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := noff, litw
    ($cg:ident : $op:ident # ( $a:expr ), $x:tt : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := noff, const
    ($cg:ident : $op:ident # ( $a:expr ), $x:tt : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := noff, undef
    ($cg:ident : $op:ident # ( $a:expr ), $x:tt : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := off, noff
    ($cg:ident : $op:ident # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := off, off
    ($cg:ident : $op:ident # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := off, lit
    ($cg:ident : $op:ident # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := off, litw
    ($cg:ident : $op:ident # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := off, const
    ($cg:ident : $op:ident # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := off, undef
    ($cg:ident : $op:ident # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt , ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := lit, noff
    ($cg:ident : $op:ident # ( $a:expr ), ( $x:expr ) , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := lit, off
    ($cg:ident : $op:ident # ( $a:expr ), ( $x:expr ) , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := lit, lit
    ($cg:ident : $op:ident # ( $a:expr ), ( $x:expr ) , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := lit, litw
    ($cg:ident : $op:ident # ( $a:expr ), ( $x:expr ) , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := lit, const
    ($cg:ident : $op:ident # ( $a:expr ), ( $x:expr ) , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := lit, undef
    ($cg:ident : $op:ident # ( $a:expr ), ( $x:expr ) , ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := litw, noff
    ($cg:ident : $op:ident # ( $a:expr ), ( $x:expr ) : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := litw, off
    ($cg:ident : $op:ident # ( $a:expr ), ( $x:expr ) : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := litw, lit
    ($cg:ident : $op:ident # ( $a:expr ), ( $x:expr ) : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := litw, litw
    ($cg:ident : $op:ident # ( $a:expr ), ( $x:expr ) : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := litw, const
    ($cg:ident : $op:ident # ( $a:expr ), ( $x:expr ) : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := litw, undef
    ($cg:ident : $op:ident # ( $a:expr ), ( $x:expr ) : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := const, noff
    ($cg:ident : $op:ident # ( $a:expr ), [ $x:tt ] : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := const, off
    ($cg:ident : $op:ident # ( $a:expr ), [ $x:tt ] : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := const, lit
    ($cg:ident : $op:ident # ( $a:expr ), [ $x:tt ] : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := const, litw
    ($cg:ident : $op:ident # ( $a:expr ), [ $x:tt ] : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := const, const
    ($cg:ident : $op:ident # ( $a:expr ), [ $x:tt ] : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := const, undef
    ($cg:ident : $op:ident # ( $a:expr ), [ $x:tt ] : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := undef, noff
    ($cg:ident : $op:ident # ( $a:expr ), ? , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := undef, off
    ($cg:ident : $op:ident # ( $a:expr ), ? , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := undef, lit
    ($cg:ident : $op:ident # ( $a:expr ), ? , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := undef, litw
    ($cg:ident : $op:ident # ( $a:expr ), ? , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := undef, const
    ($cg:ident : $op:ident # ( $a:expr ), ? , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := undef, undef
    ($cg:ident : $op:ident # ( $a:expr ), ? , ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := noff, noff
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := noff, off
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := noff, lit
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := noff, litw
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := noff, const
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := noff, undef
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := off, noff
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := off, off
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := off, lit
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := off, litw
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := off, const
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := off, undef
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := lit, noff
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := lit, off
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := lit, lit
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := lit, litw
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := lit, const
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := lit, undef
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) , ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := litw, noff
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := litw, off
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := litw, lit
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := litw, litw
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := litw, const
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := litw, undef
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := const, noff
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := const, off
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := const, lit
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := const, litw
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := const, const
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := const, undef
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := undef, noff
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, ? , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := undef, off
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, ? , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := undef, lit
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, ? , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := undef, litw
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, ? , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := undef, const
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, ? , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := undef, undef
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, ? , ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := noff, noff
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := noff, off
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := noff, lit
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := noff, litw
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := noff, const
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := noff, undef
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := off, noff
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := off, off
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := off, lit
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := off, litw
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := off, const
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := off, undef
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := lit, noff
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, ( $x:expr ) , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := lit, off
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, ( $x:expr ) , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := lit, lit
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, ( $x:expr ) , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := lit, litw
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, ( $x:expr ) , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := lit, const
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, ( $x:expr ) , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := lit, undef
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, ( $x:expr ) , ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := litw, noff
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := litw, off
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := litw, lit
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := litw, litw
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := litw, const
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := litw, undef
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := const, noff
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := const, off
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := const, lit
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := const, litw
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := const, const
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := const, undef
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := undef, noff
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, ? , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := undef, off
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, ? , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := undef, lit
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, ? , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := undef, litw
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, ? , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := undef, const
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, ? , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := undef, undef
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, ? , ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := noff, noff
    ($cg:ident : $op:ident # ?, $x:tt : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := noff, off
    ($cg:ident : $op:ident # ?, $x:tt : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := noff, lit
    ($cg:ident : $op:ident # ?, $x:tt : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := noff, litw
    ($cg:ident : $op:ident # ?, $x:tt : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := noff, const
    ($cg:ident : $op:ident # ?, $x:tt : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := noff, undef
    ($cg:ident : $op:ident # ?, $x:tt : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := off, noff
    ($cg:ident : $op:ident # ?, $x:tt : $x_w:tt / $x_o:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := off, off
    ($cg:ident : $op:ident # ?, $x:tt : $x_w:tt / $x_o:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := off, lit
    ($cg:ident : $op:ident # ?, $x:tt : $x_w:tt / $x_o:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := off, litw
    ($cg:ident : $op:ident # ?, $x:tt : $x_w:tt / $x_o:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := off, const
    ($cg:ident : $op:ident # ?, $x:tt : $x_w:tt / $x_o:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := off, undef
    ($cg:ident : $op:ident # ?, $x:tt : $x_w:tt / $x_o:tt , ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := lit, noff
    ($cg:ident : $op:ident # ?, ( $x:expr ) , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := lit, off
    ($cg:ident : $op:ident # ?, ( $x:expr ) , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := lit, lit
    ($cg:ident : $op:ident # ?, ( $x:expr ) , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := lit, litw
    ($cg:ident : $op:ident # ?, ( $x:expr ) , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := lit, const
    ($cg:ident : $op:ident # ?, ( $x:expr ) , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := lit, undef
    ($cg:ident : $op:ident # ?, ( $x:expr ) , ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := litw, noff
    ($cg:ident : $op:ident # ?, ( $x:expr ) : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := litw, off
    ($cg:ident : $op:ident # ?, ( $x:expr ) : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := litw, lit
    ($cg:ident : $op:ident # ?, ( $x:expr ) : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := litw, litw
    ($cg:ident : $op:ident # ?, ( $x:expr ) : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := litw, const
    ($cg:ident : $op:ident # ?, ( $x:expr ) : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := litw, undef
    ($cg:ident : $op:ident # ?, ( $x:expr ) : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := const, noff
    ($cg:ident : $op:ident # ?, [ $x:tt ] : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := const, off
    ($cg:ident : $op:ident # ?, [ $x:tt ] : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := const, lit
    ($cg:ident : $op:ident # ?, [ $x:tt ] : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := const, litw
    ($cg:ident : $op:ident # ?, [ $x:tt ] : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := const, const
    ($cg:ident : $op:ident # ?, [ $x:tt ] : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := const, undef
    ($cg:ident : $op:ident # ?, [ $x:tt ] : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := undef, noff
    ($cg:ident : $op:ident # ?, ? , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := undef, off
    ($cg:ident : $op:ident # ?, ? , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := undef, lit
    ($cg:ident : $op:ident # ?, ? , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := undef, litw
    ($cg:ident : $op:ident # ?, ? , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := undef, const
    ($cg:ident : $op:ident # ?, ? , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := undef, undef
    ($cg:ident : $op:ident # ?, ? , ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

}

macro_rules! rreil_unop {
    // lit := noff
    ($cg:ident : $op:ident # ( $a:expr ), $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := off
    ($cg:ident : $op:ident # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := lit
    ($cg:ident : $op:ident # ( $a:expr ), ( $x:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := litw
    ($cg:ident : $op:ident # ( $a:expr ), ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := const
    ($cg:ident : $op:ident # ( $a:expr ), [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := undef
    ($cg:ident : $op:ident # ( $a:expr ), ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := noff
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := off
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := lit
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := litw
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := const
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := undef
    ($cg:ident : $op:ident # ( $a:expr ) : $a_w:tt, ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := noff
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := off
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := lit
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ))), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := litw
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := const
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := undef
    ($cg:ident : $op:ident # $a:tt : $a_w:tt, ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := noff
    ($cg:ident : $op:ident # ?, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := off
    ($cg:ident : $op:ident # ?, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := lit
    ($cg:ident : $op:ident # ?, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := litw
    ($cg:ident : $op:ident # ?, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := const
    ($cg:ident : $op:ident # ?, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := undef
    ($cg:ident : $op:ident # ?, ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_rvalue!(?)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

}

macro_rules! rreil_memop {
    // lit := noff
    ($cg:ident : $op:ident # $bank:ident # ( $a:expr ), $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := off
    ($cg:ident : $op:ident # $bank:ident # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := lit
    ($cg:ident : $op:ident # $bank:ident # ( $a:expr ), ( $x:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := litw
    ($cg:ident : $op:ident # $bank:ident # ( $a:expr ), ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := const
    ($cg:ident : $op:ident # $bank:ident # ( $a:expr ), [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := undef
    ($cg:ident : $op:ident # $bank:ident # ( $a:expr ), ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := noff
    ($cg:ident : $op:ident # $bank:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := off
    ($cg:ident : $op:ident # $bank:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := lit
    ($cg:ident : $op:ident # $bank:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := litw
    ($cg:ident : $op:ident # $bank:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := const
    ($cg:ident : $op:ident # $bank:ident # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := undef
    ($cg:ident : $op:ident # $bank:ident # ( $a:expr ) : $a_w:tt, ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := noff
    ($cg:ident : $op:ident # $bank:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := off
    ($cg:ident : $op:ident # $bank:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := lit
    ($cg:ident : $op:ident # $bank:ident # $a:tt : $a_w:tt, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := litw
    ($cg:ident : $op:ident # $bank:ident # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := const
    ($cg:ident : $op:ident # $bank:ident # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := undef
    ($cg:ident : $op:ident # $bank:ident # $a:tt : $a_w:tt, ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := noff
    ($cg:ident : $op:ident # $bank:ident # ?, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := off
    ($cg:ident : $op:ident # $bank:ident # ?, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := lit
    ($cg:ident : $op:ident # $bank:ident # ?, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := litw
    ($cg:ident : $op:ident # $bank:ident # ?, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := const
    ($cg:ident : $op:ident # $bank:ident # ?, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := undef
    ($cg:ident : $op:ident # $bank:ident # ?, ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

}

macro_rules! rreil_extop {
    // lit := noff
    ($cg:ident : $op:ident # $sz:tt # ( $a:expr ), $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := off
    ($cg:ident : $op:ident # $sz:tt # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := lit
    ($cg:ident : $op:ident # $sz:tt # ( $a:expr ), ( $x:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := litw
    ($cg:ident : $op:ident # $sz:tt # ( $a:expr ), ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := const
    ($cg:ident : $op:ident # $sz:tt # ( $a:expr ), [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := undef
    ($cg:ident : $op:ident # $sz:tt # ( $a:expr ), ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := noff
    ($cg:ident : $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := off
    ($cg:ident : $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := lit
    ($cg:ident : $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := litw
    ($cg:ident : $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := const
    ($cg:ident : $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := undef
    ($cg:ident : $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := noff
    ($cg:ident : $op:ident # $sz:tt # $a:tt : $a_w:tt, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := off
    ($cg:ident : $op:ident # $sz:tt # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := lit
    ($cg:ident : $op:ident # $sz:tt # $a:tt : $a_w:tt, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := litw
    ($cg:ident : $op:ident # $sz:tt # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := const
    ($cg:ident : $op:ident # $sz:tt # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := undef
    ($cg:ident : $op:ident # $sz:tt # $a:tt : $a_w:tt, ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := noff
    ($cg:ident : $op:ident # $sz:tt # ?, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := off
    ($cg:ident : $op:ident # $sz:tt # ?, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := lit
    ($cg:ident : $op:ident # $sz:tt # ?, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := litw
    ($cg:ident : $op:ident # $sz:tt # ?, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := const
    ($cg:ident : $op:ident # $sz:tt # ?, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := undef
    ($cg:ident : $op:ident # $sz:tt # ?, ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

}

macro_rules! rreil_selop {
    // lit := noff
    ($cg:ident : $op:ident # $sz:tt # ( $a:expr ), $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a )),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := off
    ($cg:ident : $op:ident # $sz:tt # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a )),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := lit
    ($cg:ident : $op:ident # $sz:tt # ( $a:expr ), ( $x:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a )),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := litw
    ($cg:ident : $op:ident # $sz:tt # ( $a:expr ), ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a )),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := const
    ($cg:ident : $op:ident # $sz:tt # ( $a:expr ), [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a )),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // lit := undef
    ($cg:ident : $op:ident # $sz:tt # ( $a:expr ), ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a )),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := noff
    ($cg:ident : $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a ) : $a_w),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := off
    ($cg:ident : $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a ) : $a_w),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := lit
    ($cg:ident : $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a ) : $a_w),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := litw
    ($cg:ident : $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a ) : $a_w),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := const
    ($cg:ident : $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a ) : $a_w),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // litw := undef
    ($cg:ident : $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a ) : $a_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := noff
    ($cg:ident : $op:ident # $sz:tt # $a:tt : $a_w:tt, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($a : $a_w),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := off
    ($cg:ident : $op:ident # $sz:tt # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($a : $a_w),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := lit
    ($cg:ident : $op:ident # $sz:tt # $a:tt : $a_w:tt, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($a : $a_w),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := litw
    ($cg:ident : $op:ident # $sz:tt # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($a : $a_w),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := const
    ($cg:ident : $op:ident # $sz:tt # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($a : $a_w),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // noff := undef
    ($cg:ident : $op:ident # $sz:tt # $a:tt : $a_w:tt, ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!($a : $a_w),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := noff
    ($cg:ident : $op:ident # $sz:tt # ?, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := off
    ($cg:ident : $op:ident # $sz:tt # ?, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := lit
    ($cg:ident : $op:ident # $sz:tt # ?, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := litw
    ($cg:ident : $op:ident # $sz:tt # ?, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := const
    ($cg:ident : $op:ident # $sz:tt # ?, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

    // undef := undef
    ($cg:ident : $op:ident # $sz:tt # ?, ? ; $($cdr:tt)*) => {
        {
            $cg.push($crate::Statement{ op: ::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)});
            rreil!($cg : $($cdr)*);
        }
    };

}

