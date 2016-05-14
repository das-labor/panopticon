#!/bin/sh

get_strings() {
   MATCH=""
   ARG=""

   case "$1" in
      lit)
         MATCH="( \$$2:expr )"
         ARG="( \$$2 )"
         ;;
      litw)
         MATCH="( \$$2:expr ) : \$$2_w:tt"
         ARG="( \$$2 ) : \$$2_w"
         ;;
      noff)
         MATCH="\$$2:tt : \$$2_w:tt"
         ARG="\$$2 : \$$2_w"
         ;;
      off)
         MATCH="\$$2:tt : \$$2_w:tt / \$$2_o:tt"
         ARG="\$$2 : \$$2_w / \$$2_o"
         ;;
      const)
         MATCH="[ \$$2:tt ] : \$$2_w:tt"
         ARG="[ \$$2 ] : \$$2_w"
         ;;
	  undef)
		 MATCH="?"
		 ARG="?"
		 ;;
   esac
}

echo "macro_rules! rreil_binop {"

for A in lit litw noff off undef
do
   get_strings $A a
   A_MATCH="$MATCH"
   A_ARG="$ARG"

   for X in noff off lit litw const undef
   do
      get_strings $X x
      X_MATCH="$MATCH"
      X_ARG="$ARG"

      for Y in noff off lit litw const undef
      do
         get_strings $Y y
         Y_MATCH="$MATCH"
         Y_ARG="$ARG"

         echo "    // $A := $X, $Y"
         echo "    (\$cg:ident : \$op:ident # $A_MATCH, $X_MATCH , $Y_MATCH ; \$(\$cdr:tt)*) => {"
         echo "        {"
         echo "            \$cg.push(\$crate::Statement{ op: $crate::Operation::\$op(rreil_rvalue!($X_ARG),rreil_rvalue!($Y_ARG)), assignee: rreil_lvalue!($A_ARG)});"
         echo "            rreil!(\$cg : \$(\$cdr)*);"
         echo "        }"
         echo "    };"
         echo ""
      done
   done
done

echo "}"
echo ""
echo "macro_rules! rreil_unop {"

for A in lit litw noff off undef
do
   get_strings $A a
   A_MATCH="$MATCH"
   A_ARG="$ARG"

   for X in noff off lit litw const undef
   do
      get_strings $X x
      X_MATCH="$MATCH"
      X_ARG="$ARG"

      echo "    // $A := $X"
      echo "    (\$cg:ident : \$op:ident # $A_MATCH, $X_MATCH ; \$(\$cdr:tt)*) => {"
      echo "        {"
      echo "            \$cg.push(\$crate::Statement{ op: $crate::Operation::\$op(rreil_rvalue!($X_ARG)), assignee: rreil_lvalue!($A_ARG)});"
      echo "            rreil!(\$cg : \$(\$cdr)*);"
      echo "        }"
      echo "    };"
      echo ""
   done
done

echo "}"
echo ""
echo "macro_rules! rreil_memop {"

for A in lit litw noff off undef
do
   get_strings $A a
   A_MATCH="$MATCH"
   A_ARG="$ARG"

   for X in noff off lit litw const undef
   do
      get_strings $X x
      X_MATCH="$MATCH"
      X_ARG="$ARG"

      echo "    // $A := $X"
      echo "    (\$cg:ident : \$op:ident # \$bank:ident # $A_MATCH, $X_MATCH ; \$(\$cdr:tt)*) => {"
      echo "        {"
      echo "            \$cg.push(\$crate::Statement{ op: $crate::Operation::\$op(::std::borrow::Cow::Borrowed(stringify!(\$bank)),rreil_rvalue!($X_ARG)), assignee: rreil_lvalue!($A_ARG)});"
      echo "            rreil!(\$cg : \$(\$cdr)*);"
      echo "        }"
      echo "    };"
      echo ""
   done
done

echo "}"
echo ""
echo "macro_rules! rreil_extop {"

for A in lit litw noff off undef
do
   get_strings $A a
   A_MATCH="$MATCH"
   A_ARG="$ARG"

   for X in noff off lit litw const undef
   do
      get_strings $X x
      X_MATCH="$MATCH"
      X_ARG="$ARG"

      echo "    // $A := $X"
      echo "    (\$cg:ident : \$op:ident # \$sz:tt # $A_MATCH, $X_MATCH ; \$(\$cdr:tt)*) => {"
      echo "        {"
      echo "            \$cg.push(\$crate::Statement{ op: $crate::Operation::\$op(rreil_imm!(\$sz),rreil_rvalue!($X_ARG)), assignee: rreil_lvalue!($A_ARG)});"
      echo "            rreil!(\$cg : \$(\$cdr)*);"
      echo "        }"
      echo "    };"
      echo ""
   done
done

echo "}"
