#[macro_export]
macro_rules! rreil_binop {
    // lit := noff, noff
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := noff, off
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := noff, lit
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := noff, litw
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := noff, const
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := noff, undef
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := off, noff
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := off, off
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := off, lit
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := off, litw
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := off, const
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := off, undef
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := lit, noff
    ( $op:ident # ( $a:expr ), ( $x:expr ) , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := lit, off
    ( $op:ident # ( $a:expr ), ( $x:expr ) , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := lit, lit
    ( $op:ident # ( $a:expr ), ( $x:expr ) , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := lit, litw
    ( $op:ident # ( $a:expr ), ( $x:expr ) , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := lit, const
    ( $op:ident # ( $a:expr ), ( $x:expr ) , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := lit, undef
    ( $op:ident # ( $a:expr ), ( $x:expr ) , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := litw, noff
    ( $op:ident # ( $a:expr ), ( $x:expr ) : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := litw, off
    ( $op:ident # ( $a:expr ), ( $x:expr ) : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := litw, lit
    ( $op:ident # ( $a:expr ), ( $x:expr ) : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := litw, litw
    ( $op:ident # ( $a:expr ), ( $x:expr ) : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := litw, const
    ( $op:ident # ( $a:expr ), ( $x:expr ) : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := litw, undef
    ( $op:ident # ( $a:expr ), ( $x:expr ) : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := const, noff
    ( $op:ident # ( $a:expr ), [ $x:tt ] : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := const, off
    ( $op:ident # ( $a:expr ), [ $x:tt ] : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := const, lit
    ( $op:ident # ( $a:expr ), [ $x:tt ] : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := const, litw
    ( $op:ident # ( $a:expr ), [ $x:tt ] : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := const, const
    ( $op:ident # ( $a:expr ), [ $x:tt ] : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := const, undef
    ( $op:ident # ( $a:expr ), [ $x:tt ] : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := undef, noff
    ( $op:ident # ( $a:expr ), ? , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := undef, off
    ( $op:ident # ( $a:expr ), ? , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := undef, lit
    ( $op:ident # ( $a:expr ), ? , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := undef, litw
    ( $op:ident # ( $a:expr ), ? , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := undef, const
    ( $op:ident # ( $a:expr ), ? , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := undef, undef
    ( $op:ident # ( $a:expr ), ? , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := noff, noff
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := noff, off
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := noff, lit
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := noff, litw
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := noff, const
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := noff, undef
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := off, noff
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := off, off
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := off, lit
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := off, litw
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := off, const
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := off, undef
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := lit, noff
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := lit, off
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := lit, lit
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := lit, litw
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := lit, const
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := lit, undef
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := litw, noff
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := litw, off
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := litw, lit
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := litw, litw
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := litw, const
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := litw, undef
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := const, noff
    ( $op:ident # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := const, off
    ( $op:ident # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := const, lit
    ( $op:ident # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := const, litw
    ( $op:ident # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := const, const
    ( $op:ident # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := const, undef
    ( $op:ident # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := undef, noff
    ( $op:ident # ( $a:expr ) : $a_w:tt, ? , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := undef, off
    ( $op:ident # ( $a:expr ) : $a_w:tt, ? , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := undef, lit
    ( $op:ident # ( $a:expr ) : $a_w:tt, ? , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := undef, litw
    ( $op:ident # ( $a:expr ) : $a_w:tt, ? , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := undef, const
    ( $op:ident # ( $a:expr ) : $a_w:tt, ? , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := undef, undef
    ( $op:ident # ( $a:expr ) : $a_w:tt, ? , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := noff, noff
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := noff, off
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := noff, lit
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := noff, litw
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := noff, const
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := noff, undef
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := off, noff
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := off, off
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := off, lit
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := off, litw
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := off, const
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := off, undef
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := lit, noff
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := lit, off
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := lit, lit
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := lit, litw
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := lit, const
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := lit, undef
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := litw, noff
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := litw, off
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := litw, lit
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := litw, litw
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := litw, const
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := litw, undef
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := const, noff
    ( $op:ident # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := const, off
    ( $op:ident # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := const, lit
    ( $op:ident # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := const, litw
    ( $op:ident # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := const, const
    ( $op:ident # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := const, undef
    ( $op:ident # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := undef, noff
    ( $op:ident # $a:tt : $a_w:tt, ? , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := undef, off
    ( $op:ident # $a:tt : $a_w:tt, ? , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := undef, lit
    ( $op:ident # $a:tt : $a_w:tt, ? , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := undef, litw
    ( $op:ident # $a:tt : $a_w:tt, ? , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := undef, const
    ( $op:ident # $a:tt : $a_w:tt, ? , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := undef, undef
    ( $op:ident # $a:tt : $a_w:tt, ? , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := noff, noff
    ( $op:ident # ?, $x:tt : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := noff, off
    ( $op:ident # ?, $x:tt : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := noff, lit
    ( $op:ident # ?, $x:tt : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := noff, litw
    ( $op:ident # ?, $x:tt : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := noff, const
    ( $op:ident # ?, $x:tt : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := noff, undef
    ( $op:ident # ?, $x:tt : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := off, noff
    ( $op:ident # ?, $x:tt : $x_w:tt / $x_o:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := off, off
    ( $op:ident # ?, $x:tt : $x_w:tt / $x_o:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := off, lit
    ( $op:ident # ?, $x:tt : $x_w:tt / $x_o:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := off, litw
    ( $op:ident # ?, $x:tt : $x_w:tt / $x_o:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := off, const
    ( $op:ident # ?, $x:tt : $x_w:tt / $x_o:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := off, undef
    ( $op:ident # ?, $x:tt : $x_w:tt / $x_o:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := lit, noff
    ( $op:ident # ?, ( $x:expr ) , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := lit, off
    ( $op:ident # ?, ( $x:expr ) , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := lit, lit
    ( $op:ident # ?, ( $x:expr ) , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := lit, litw
    ( $op:ident # ?, ( $x:expr ) , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := lit, const
    ( $op:ident # ?, ( $x:expr ) , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := lit, undef
    ( $op:ident # ?, ( $x:expr ) , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x )),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := litw, noff
    ( $op:ident # ?, ( $x:expr ) : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := litw, off
    ( $op:ident # ?, ( $x:expr ) : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := litw, lit
    ( $op:ident # ?, ( $x:expr ) : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := litw, litw
    ( $op:ident # ?, ( $x:expr ) : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := litw, const
    ( $op:ident # ?, ( $x:expr ) : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := litw, undef
    ( $op:ident # ?, ( $x:expr ) : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := const, noff
    ( $op:ident # ?, [ $x:tt ] : $x_w:tt , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := const, off
    ( $op:ident # ?, [ $x:tt ] : $x_w:tt , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := const, lit
    ( $op:ident # ?, [ $x:tt ] : $x_w:tt , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := const, litw
    ( $op:ident # ?, [ $x:tt ] : $x_w:tt , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := const, const
    ( $op:ident # ?, [ $x:tt ] : $x_w:tt , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := const, undef
    ( $op:ident # ?, [ $x:tt ] : $x_w:tt , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := undef, noff
    ( $op:ident # ?, ? , $y:tt : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?),rreil_rvalue!($y : $y_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := undef, off
    ( $op:ident # ?, ? , $y:tt : $y_w:tt / $y_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?),rreil_rvalue!($y : $y_w / $y_o)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := undef, lit
    ( $op:ident # ?, ? , ( $y:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(( $y ))), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := undef, litw
    ( $op:ident # ?, ? , ( $y:expr ) : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(( $y ) : $y_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := undef, const
    ( $op:ident # ?, ? , [ $y:tt ] : $y_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?),rreil_rvalue!([ $y ] : $y_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := undef, undef
    ( $op:ident # ?, ? , ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

}

#[macro_export]
macro_rules! rreil_unop {
    // lit := noff
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := off
    ( $op:ident # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := lit
    ( $op:ident # ( $a:expr ), ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := litw
    ( $op:ident # ( $a:expr ), ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := const
    ( $op:ident # ( $a:expr ), [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := undef
    ( $op:ident # ( $a:expr ), ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := noff
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := off
    ( $op:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := lit
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := litw
    ( $op:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := const
    ( $op:ident # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := undef
    ( $op:ident # ( $a:expr ) : $a_w:tt, ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := noff
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := off
    ( $op:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := lit
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ))), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := litw
    ( $op:ident # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := const
    ( $op:ident # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := undef
    ( $op:ident # $a:tt : $a_w:tt, ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := noff
    ( $op:ident # ?, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := off
    ( $op:ident # ?, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := lit
    ( $op:ident # ?, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := litw
    ( $op:ident # ?, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := const
    ( $op:ident # ?, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := undef
    ( $op:ident # ?, ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_rvalue!(?)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

}

#[macro_export]
macro_rules! rreil_memop {
    // lit := noff
    ( $op:ident # $bank:ident # ( $a:expr ), $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := off
    ( $op:ident # $bank:ident # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := lit
    ( $op:ident # $bank:ident # ( $a:expr ), ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := litw
    ( $op:ident # $bank:ident # ( $a:expr ), ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := const
    ( $op:ident # $bank:ident # ( $a:expr ), [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := undef
    ( $op:ident # $bank:ident # ( $a:expr ), ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := noff
    ( $op:ident # $bank:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := off
    ( $op:ident # $bank:ident # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := lit
    ( $op:ident # $bank:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := litw
    ( $op:ident # $bank:ident # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := const
    ( $op:ident # $bank:ident # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := undef
    ( $op:ident # $bank:ident # ( $a:expr ) : $a_w:tt, ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := noff
    ( $op:ident # $bank:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := off
    ( $op:ident # $bank:ident # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := lit
    ( $op:ident # $bank:ident # $a:tt : $a_w:tt, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := litw
    ( $op:ident # $bank:ident # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := const
    ( $op:ident # $bank:ident # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := undef
    ( $op:ident # $bank:ident # $a:tt : $a_w:tt, ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := noff
    ( $op:ident # $bank:ident # ?, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := off
    ( $op:ident # $bank:ident # ?, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := lit
    ( $op:ident # $bank:ident # ?, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := litw
    ( $op:ident # $bank:ident # ?, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := const
    ( $op:ident # $bank:ident # ?, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := undef
    ( $op:ident # $bank:ident # ?, ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(::std::borrow::Cow::Borrowed(stringify!($bank)),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

}

#[macro_export]
macro_rules! rreil_extop {
    // lit := noff
    ( $op:ident # $sz:tt # ( $a:expr ), $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := off
    ( $op:ident # $sz:tt # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := lit
    ( $op:ident # $sz:tt # ( $a:expr ), ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := litw
    ( $op:ident # $sz:tt # ( $a:expr ), ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := const
    ( $op:ident # $sz:tt # ( $a:expr ), [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := undef
    ( $op:ident # $sz:tt # ( $a:expr ), ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := noff
    ( $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := off
    ( $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := lit
    ( $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := litw
    ( $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := const
    ( $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := undef
    ( $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := noff
    ( $op:ident # $sz:tt # $a:tt : $a_w:tt, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := off
    ( $op:ident # $sz:tt # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := lit
    ( $op:ident # $sz:tt # $a:tt : $a_w:tt, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := litw
    ( $op:ident # $sz:tt # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := const
    ( $op:ident # $sz:tt # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := undef
    ( $op:ident # $sz:tt # $a:tt : $a_w:tt, ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := noff
    ( $op:ident # $sz:tt # ?, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := off
    ( $op:ident # $sz:tt # ?, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := lit
    ( $op:ident # $sz:tt # ?, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := litw
    ( $op:ident # $sz:tt # ?, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := const
    ( $op:ident # $sz:tt # ?, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := undef
    ( $op:ident # $sz:tt # ?, ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

}

#[macro_export]
macro_rules! rreil_selop {
    // lit := noff
    ( $op:ident # $sz:tt # ( $a:expr ), $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a )),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := off
    ( $op:ident # $sz:tt # ( $a:expr ), $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a )),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := lit
    ( $op:ident # $sz:tt # ( $a:expr ), ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a )),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := litw
    ( $op:ident # $sz:tt # ( $a:expr ), ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a )),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := const
    ( $op:ident # $sz:tt # ( $a:expr ), [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a )),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // lit := undef
    ( $op:ident # $sz:tt # ( $a:expr ), ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a )),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ))}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := noff
    ( $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a ) : $a_w),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := off
    ( $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a ) : $a_w),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := lit
    ( $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a ) : $a_w),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := litw
    ( $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a ) : $a_w),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := const
    ( $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a ) : $a_w),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // litw := undef
    ( $op:ident # $sz:tt # ( $a:expr ) : $a_w:tt, ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(( $a ) : $a_w),rreil_rvalue!(?)), assignee: rreil_lvalue!(( $a ) : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := noff
    ( $op:ident # $sz:tt # $a:tt : $a_w:tt, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!($a : $a_w),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := off
    ( $op:ident # $sz:tt # $a:tt : $a_w:tt, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!($a : $a_w),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := lit
    ( $op:ident # $sz:tt # $a:tt : $a_w:tt, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!($a : $a_w),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := litw
    ( $op:ident # $sz:tt # $a:tt : $a_w:tt, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!($a : $a_w),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := const
    ( $op:ident # $sz:tt # $a:tt : $a_w:tt, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!($a : $a_w),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // noff := undef
    ( $op:ident # $sz:tt # $a:tt : $a_w:tt, ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!($a : $a_w),rreil_rvalue!(?)), assignee: rreil_lvalue!($a : $a_w)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := noff
    ( $op:ident # $sz:tt # ?, $x:tt : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?),rreil_rvalue!($x : $x_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := off
    ( $op:ident # $sz:tt # ?, $x:tt : $x_w:tt / $x_o:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?),rreil_rvalue!($x : $x_w / $x_o)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := lit
    ( $op:ident # $sz:tt # ?, ( $x:expr ) ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?),rreil_rvalue!(( $x ))), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := litw
    ( $op:ident # $sz:tt # ?, ( $x:expr ) : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?),rreil_rvalue!(( $x ) : $x_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := const
    ( $op:ident # $sz:tt # ?, [ $x:tt ] : $x_w:tt ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?),rreil_rvalue!([ $x ] : $x_w)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

    // undef := undef
    ( $op:ident # $sz:tt # ?, ? ; $($cdr:tt)*) => {
        {
            let mut stmt = vec![$crate::Statement{ op: $crate::Operation::$op(rreil_imm!($sz),rreil_rvalue!(?),rreil_rvalue!(?)), assignee: rreil_lvalue!(?)}];
let ret: $crate::result::Result<Vec<$crate::il::Statement>> = match stmt[0].sanity_check() {
	Ok(()) => {
    let mut tail: $crate::result::Result<Vec<$crate::il::Statement>> = { rreil!( $($cdr)*) };
    match tail {
		  Ok(ref mut other) => {
			  stmt.extend(other.drain(..));
		  	Ok(stmt)
		  }
		  Err(e) => Err(e),
	  }
  },
	Err(e) => Err(e).into(),
}; ret
        }
    };

}

