use libc;
use libc::free;
extern "C" {
  #[no_mangle]
  fn strtoull(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_ulonglong;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn skip_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn endofname(name: *const libc::c_char) -> *const libc::c_char;
  #[no_mangle]
  fn safe_strncpy(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    size: size_t,
  ) -> *mut libc::c_char;
}
use crate::librb::size_t;
/* math.h - interface to shell math "library" -- this allows shells to share
 *          the implementation of arithmetic $((...)) expansions.
 *
 * This aims to be a POSIX shell math library as documented here:
 *	http://www.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#tag_18_06_04
 *
 * See math.c for internal documentation.
 */
/* The math library has just one function:
 *
 * arith_t arith(arith_state_t *state, const char *expr);
 *
 * The expr argument is the math string to parse.  All normal expansions must
 * be done already.  i.e. no dollar symbols should be present.
 *
 * The state argument is a pointer to a struct of hooks for your shell (see below),
 * and an error message string (NULL if no error).
 *
 * The function returns the answer to the expression.  So if you called it
 * with the expression:
 * "1 + 2 + 3"
 * you would obviously get back 6.
 */
/* To add support to a shell, you need to implement three functions:
 *
 * lookupvar() - look up and return the value of a variable
 *
 *	If the shell does:
 *		foo=123
 *	Then the code:
 *		const char *val = lookupvar("foo");
 *	will result in val pointing to "123"
 *
 * setvar() - set a variable to some value
 *
 *	If the arithmetic expansion does something like:
 *		$(( i = 1))
 *	then the math code will make a call like so:
 *		setvar("i", "1", 0);
 *	The storage for the first two parameters are not allocated, so your
 *	shell implementation will most likely need to strdup() them to save.
 *
 * endofname() - return the end of a variable name from input
 *
 *	The arithmetic code does not know about variable naming conventions.
 *	So when it is given an experession, it knows something is not numeric,
 *	but it is up to the shell to dictate what is a valid identifiers.
 *	So when it encounters something like:
 *		$(( some_var + 123 ))
 *	It will make a call like so:
 *		end = endofname("some_var + 123");
 *	So the shell needs to scan the input string and return a pointer to the
 *	first non-identifier string.  In this case, it should return the input
 *	pointer with an offset pointing to the first space.  The typical
 *	implementation will return the offset of first char that does not match
 *	the regex (in C locale): ^[a-zA-Z_][a-zA-Z_0-9]*
 */
pub type arith_t = libc::c_longlong;
pub type arith_var_lookup_t =
  Option<unsafe extern "C" fn(_: *const libc::c_char) -> *const libc::c_char>;
pub type arith_var_set_t =
  Option<unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arith_state_t {
  pub errmsg: *const libc::c_char,
  pub lookupvar: arith_var_lookup_t,
  pub setvar: arith_var_set_t,
  pub list_of_recursed_names: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct var_or_num_t {
  pub val: arith_t,
  pub second_val: arith_t,
  pub second_val_present: libc::c_char,
  pub var: *mut libc::c_char,
}
/*
 * Arithmetic code ripped out of ash shell for code sharing.
 *
 * This code is derived from software contributed to Berkeley by
 * Kenneth Almquist.
 *
 * Original BSD copyright notice is retained at the end of this file.
 *
 * Copyright (c) 1989, 1991, 1993, 1994
 *      The Regents of the University of California.  All rights reserved.
 *
 * Copyright (c) 1997-2005 Herbert Xu <herbert@gondor.apana.org.au>
 * was re-ported from NetBSD and debianized.
 *
 * rewrite arith.y to micro stack based cryptic algorithm by
 * Copyright (c) 2001 Aaron Lehmann <aaronl@vitelus.com>
 *
 * Modified by Paul Mundt <lethal@linux-sh.org> (c) 2004 to support
 * dynamic variables.
 *
 * Modified by Vladimir Oleynik <dzo@simtreas.ru> (c) 2001-2005 to be
 * used in busybox and size optimizations,
 * rewrote arith (see notes to this), added locale support,
 * rewrote dynamic variables.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Copyright (c) 2001 Aaron Lehmann <aaronl@vitelus.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining
 * a copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sublicense, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 * IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
 * CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
 * TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
 * SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */
/* This is my infix parser/evaluator. It is optimized for size, intended
 * as a replacement for yacc-based parsers. However, it may well be faster
 * than a comparable parser written in yacc. The supported operators are
 * listed in #defines below. Parens, order of operations, and error handling
 * are supported. This code is thread safe. The exact expression format should
 * be that which POSIX specifies for shells.
 *
 * The code uses a simple two-stack algorithm. See
 * http://www.onthenet.com.au/~grahamis/int2008/week02/lect02.html
 * for a detailed explanation of the infix-to-postfix algorithm on which
 * this is based (this code differs in that it applies operators immediately
 * to the stack instead of adding them to a queue to end up with an
 * expression).
 */
/*
 * Aug 24, 2001              Manuel Novoa III
 *
 * Reduced the generated code size by about 30% (i386) and fixed several bugs.
 *
 * 1) In arith_apply():
 *    a) Cached values of *numptr and &(numptr[-1]).
 *    b) Removed redundant test for zero denominator.
 *
 * 2) In arith():
 *    a) Eliminated redundant code for processing operator tokens by moving
 *       to a table-based implementation.  Also folded handling of parens
 *       into the table.
 *    b) Combined all 3 loops which called arith_apply to reduce generated
 *       code size at the cost of speed.
 *
 * 3) The following expressions were treated as valid by the original code:
 *       1()  ,    0!  ,    1 ( *3 )   .
 *    These bugs have been fixed by internally enclosing the expression in
 *    parens and then checking that all binary ops and right parens are
 *    preceded by a valid expression (NUM_TOKEN).
 *
 * Note: It may be desirable to replace Aaron's test for whitespace with
 * ctype's isspace() if it is used by another busybox applet or if additional
 * whitespace chars should be considered.  Look below the "#include"s for a
 * precompiler test.
 */
/*
 * Aug 26, 2001              Manuel Novoa III
 *
 * Return 0 for null expressions.  Pointed out by Vladimir Oleynik.
 *
 * Merge in Aaron's comments previously posted to the busybox list,
 * modified slightly to take account of my changes to the code.
 *
 */
/*
 *  (C) 2003 Vladimir Oleynik <dzo@simtreas.ru>
 *
 * - allow access to variable,
 *   use recursive value indirection: c="2*2"; a="c"; echo $((a+=2)) produce 6
 * - implement assign syntax (VAR=expr, +=, *= etc)
 * - implement exponentiation (** operator)
 * - implement comma separated - expr, expr
 * - implement ++expr --expr expr++ expr--
 * - implement expr ? expr : expr (but second expr is always calculated)
 * - allow hexadecimal and octal numbers
 * - restore lost XOR operator
 * - protect $((num num)) as true zero expr (Manuel's error)
 * - always use special isspace(), see comment from bash ;-)
 */
//#define endofname (math_state->endofname)
pub type operator = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct remembered_name {
  pub next: *mut remembered_name,
  pub var: *const libc::c_char,
}
unsafe extern "C" fn is_assign_op(mut op: operator) -> libc::c_int {
  let mut prec: operator = (op as libc::c_int & 0x1fi32) as operator;
  if prec as libc::c_int == 3i32 {
    prec = 2i32 as operator
  }
  return (prec as libc::c_int == (0i32 << 5i32 | 2i32) & 0x1fi32
    || prec as libc::c_int == 16i32 + 2i32
    || prec as libc::c_int == 16i32 + 3i32) as libc::c_int;
}
unsafe extern "C" fn is_right_associative(mut prec: operator) -> libc::c_int {
  return (prec as libc::c_int == (0i32 << 5i32 | 2i32) & 0x1fi32
    || prec as libc::c_int == (1i32 << 5i32 | 15i32) & 0x1fi32
    || prec as libc::c_int == (0i32 << 5i32 | 4i32) & 0x1fi32) as libc::c_int;
}
unsafe extern "C" fn arith_lookup_val(
  mut math_state: *mut arith_state_t,
  mut t: *mut var_or_num_t,
) -> *const libc::c_char {
  if !(*t).var.is_null() {
    let mut p: *const libc::c_char =
      (*math_state).lookupvar.expect("non-null function pointer")((*t).var);
    if !p.is_null() {
      let mut cur: *mut remembered_name = 0 as *mut remembered_name;
      let mut cur_save: remembered_name = remembered_name {
        next: 0 as *mut remembered_name,
        var: 0 as *const libc::c_char,
      };
      /* did we already see this name?
       * testcase: a=b; b=a; echo $((a))
       */
      cur = (*math_state).list_of_recursed_names as *mut remembered_name;
      while !cur.is_null() {
        if strcmp((*cur).var, (*t).var) == 0i32 {
          /* Yes */
          return b"expression recursion loop detected\x00" as *const u8 as *const libc::c_char;
        }
        cur = (*cur).next
      }
      /* push current var name */
      cur = (*math_state).list_of_recursed_names as *mut remembered_name;
      cur_save.var = (*t).var;
      cur_save.next = cur;
      (*math_state).list_of_recursed_names =
        &mut cur_save as *mut remembered_name as *mut libc::c_void;
      /* recursively evaluate p as expression */
      (*t).val = evaluate_string(math_state, p);
      /* pop current var name */
      (*math_state).list_of_recursed_names = cur as *mut libc::c_void;
      return (*math_state).errmsg;
    }
    /* treat undefined var as 0 */
    (*t).val = 0i32 as arith_t
  }
  return 0 as *const libc::c_char;
}
/* "Applying" a token means performing it on the top elements on the integer
 * stack. For an unary operator it will only change the top element, but a
 * binary operator will pop two arguments and push the result */
#[inline(never)]
unsafe extern "C" fn arith_apply(
  mut math_state: *mut arith_state_t,
  mut op: operator,
  mut numstack: *mut var_or_num_t,
  mut numstackptr: *mut *mut var_or_num_t,
) -> *const libc::c_char {
  let mut current_block: u64;
  let mut top_of_stack: *mut var_or_num_t = 0 as *mut var_or_num_t;
  let mut rez: arith_t = 0;
  let mut err: *const libc::c_char = 0 as *const libc::c_char;
  /* There is no operator that can work without arguments */
  if !(*numstackptr == numstack) {
    top_of_stack = (*numstackptr).offset(-1);
    /* Resolve name to value, if needed */
    err = arith_lookup_val(math_state, top_of_stack);
    if !err.is_null() {
      return err;
    }
    rez = (*top_of_stack).val;
    if op as libc::c_int == 0i32 << 5i32 | 16i32 + 1i32 {
      rez = -rez;
      current_block = 12705158477165241210;
    } else if op as libc::c_int == 1i32 << 5i32 | 16i32 {
      rez = (rez == 0) as libc::c_int as arith_t;
      current_block = 12705158477165241210;
    } else if op as libc::c_int == 0i32 << 5i32 | 16i32 {
      rez = !rez;
      current_block = 12705158477165241210;
    } else if op as libc::c_int == 0i32 << 5i32 | 16i32 + 3i32
      || op as libc::c_int == 0i32 << 5i32 | 16i32 + 2i32
    {
      rez += 1;
      current_block = 12705158477165241210;
    } else if op as libc::c_int == 1i32 << 5i32 | 16i32 + 3i32
      || op as libc::c_int == 1i32 << 5i32 | 16i32 + 2i32
    {
      rez -= 1;
      current_block = 12705158477165241210;
    } else if op as libc::c_int != 1i32 << 5i32 | 16i32 + 1i32 {
      /* Binary operators */
      let mut right_side_val: arith_t = 0;
      let mut bad_second_val: libc::c_char = 0;
      /* Binary operators need two arguments */
      if top_of_stack == numstack {
        current_block = 11780103142822320374;
      } else {
        /* ...and they pop one */
        *numstackptr = top_of_stack; /* this decrements NUMPTR */
        bad_second_val = (*top_of_stack).second_val_present;
        if op as libc::c_int == 0i32 << 5i32 | 4i32 {
          /* ? operation */
          /* Make next if (...) protect against
           * $((expr1 ? expr2)) - that is, missing ": expr" */
          bad_second_val = (bad_second_val == 0) as libc::c_int as libc::c_char
        }
        if bad_second_val != 0 {
          /* Protect against $((expr <not_?_op> expr1 : expr2)) */
          return b"malformed ?: operator\x00" as *const u8 as *const libc::c_char;
        } /* now points to left side */
        top_of_stack = top_of_stack.offset(-1);
        if op as libc::c_int != 0i32 << 5i32 | 2i32 {
          /* Resolve left side value (unless the op is '=') */
          err = arith_lookup_val(math_state, top_of_stack);
          if !err.is_null() {
            return err;
          }
        }
        right_side_val = rez;
        rez = (*top_of_stack).val;
        if op as libc::c_int == 0i32 << 5i32 | 4i32 {
          /* ? operation */
          rez = if rez != 0 {
            right_side_val
          } else {
            (*top_of_stack.offset(1)).second_val
          }
        } else if op as libc::c_int == 1i32 << 5i32 | 4i32 {
          /* : operation */
          if top_of_stack == numstack {
            /* Protect against $((expr : expr)) */
            return b"malformed ?: operator\x00" as *const u8 as *const libc::c_char;
          }
          (*top_of_stack).second_val_present = op as libc::c_char;
          (*top_of_stack).second_val = right_side_val
        } else if op as libc::c_int == 0i32 << 5i32 | 7i32
          || op as libc::c_int == 2i32 << 5i32 | 2i32
        {
          rez |= right_side_val
        } else if op as libc::c_int == 0i32 << 5i32 | 5i32 {
          rez = (right_side_val != 0 || rez != 0) as libc::c_int as arith_t
        } else if op as libc::c_int == 0i32 << 5i32 | 9i32
          || op as libc::c_int == 1i32 << 5i32 | 2i32
        {
          rez &= right_side_val
        } else if op as libc::c_int == 0i32 << 5i32 | 8i32
          || op as libc::c_int == 3i32 << 5i32 | 2i32
        {
          rez ^= right_side_val
        } else if op as libc::c_int == 0i32 << 5i32 | 6i32 {
          rez = (rez != 0 && right_side_val != 0) as libc::c_int as arith_t
        } else if op as libc::c_int == 0i32 << 5i32 | 10i32 {
          rez = (rez == right_side_val) as libc::c_int as arith_t
        } else if op as libc::c_int == 1i32 << 5i32 | 10i32 {
          rez = (rez != right_side_val) as libc::c_int as arith_t
        } else if op as libc::c_int == 2i32 << 5i32 | 11i32 {
          rez = (rez >= right_side_val) as libc::c_int as arith_t
        } else if op as libc::c_int == 1i32 << 5i32 | 12i32
          || op as libc::c_int == 7i32 << 5i32 | 2i32
        {
          rez >>= right_side_val
        } else if op as libc::c_int == 0i32 << 5i32 | 12i32
          || op as libc::c_int == 6i32 << 5i32 | 2i32
        {
          rez <<= right_side_val
        } else if op as libc::c_int == 1i32 << 5i32 | 11i32 {
          rez = (rez > right_side_val) as libc::c_int as arith_t
        } else if op as libc::c_int == 0i32 << 5i32 | 11i32 {
          rez = (rez < right_side_val) as libc::c_int as arith_t
        } else if op as libc::c_int == 3i32 << 5i32 | 11i32 {
          rez = (rez <= right_side_val) as libc::c_int as arith_t
        } else if op as libc::c_int == 0i32 << 5i32 | 14i32
          || op as libc::c_int == 0i32 << 5i32 | 3i32
        {
          rez *= right_side_val
        } else if op as libc::c_int == 0i32 << 5i32 | 13i32
          || op as libc::c_int == 4i32 << 5i32 | 2i32
        {
          rez += right_side_val
        } else if op as libc::c_int == 1i32 << 5i32 | 13i32
          || op as libc::c_int == 5i32 << 5i32 | 2i32
        {
          rez -= right_side_val
        } else if op as libc::c_int == 0i32 << 5i32 | 2i32
          || op as libc::c_int == 0i32 << 5i32 | 1i32
        {
          rez = right_side_val
        } else if op as libc::c_int == 1i32 << 5i32 | 15i32 {
          let mut c: arith_t = 0;
          if right_side_val < 0i32 as libc::c_longlong {
            return b"exponent less than 0\x00" as *const u8 as *const libc::c_char;
          }
          c = 1i32 as arith_t;
          loop {
            right_side_val -= 1;
            if !(right_side_val >= 0i32 as libc::c_longlong) {
              break;
            }
            c *= rez
          }
          rez = c
        } else if right_side_val == 0i32 as libc::c_longlong {
          return b"divide by zero\x00" as *const u8 as *const libc::c_char;
        } else {
          if op as libc::c_int == 1i32 << 5i32 | 14i32
            || op as libc::c_int == 1i32 << 5i32 | 3i32
            || op as libc::c_int == 2i32 << 5i32 | 14i32
            || op as libc::c_int == 2i32 << 5i32 | 3i32
          {
            /*
             * bash 4.2.45 x86 64bit: SEGV on 'echo $((2**63 / -1))'
             *
             * MAX_NEGATIVE_INT / -1 = MAX_POSITIVE_INT+1
             * and thus is not representable.
             * Some CPUs segfault trying such op.
             * Others overflow MAX_POSITIVE_INT+1 to
             * MAX_NEGATIVE_INT (0x7fff+1 = 0x8000).
             * Make sure to at least not SEGV here:
             */
            if right_side_val == -1i32 as libc::c_longlong
              && rez << 1i32 == 0i32 as libc::c_longlong
            {
              /* MAX_NEGATIVE_INT or 0 */
              right_side_val = 1i32 as arith_t
            }
            if op as libc::c_int == 1i32 << 5i32 | 14i32 || op as libc::c_int == 1i32 << 5i32 | 3i32
            {
              rez /= right_side_val
            } else {
              rez %= right_side_val
            }
          }
        }
        current_block = 12705158477165241210;
      }
    } else {
      current_block = 12705158477165241210;
    }
    match current_block {
      11780103142822320374 => {}
      _ => {
        if is_assign_op(op) != 0 {
          let mut buf: [libc::c_char; 26] = [0; 26];
          if (*top_of_stack).var.is_null() {
            current_block = 11780103142822320374;
          } else {
            /* Save to shell variable */
            sprintf(
              buf.as_mut_ptr(),
              b"%lld\x00" as *const u8 as *const libc::c_char,
              rez,
            );
            (*math_state).setvar.expect("non-null function pointer")(
              (*top_of_stack).var,
              buf.as_mut_ptr(),
            );
            /* After saving, make previous value for v++ or v-- */
            if op as libc::c_int == 0i32 << 5i32 | 16i32 + 3i32 {
              rez -= 1
            } else if op as libc::c_int == 1i32 << 5i32 | 16i32 + 3i32 {
              rez += 1
            }
            current_block = 6712462580143783635;
          }
        } else {
          current_block = 6712462580143783635;
        }
        match current_block {
          11780103142822320374 => {}
          _ => {
            (*top_of_stack).val = rez;
            /* Erase var name, it is just a number now */
            (*top_of_stack).var = 0 as *mut libc::c_char;
            return 0 as *const libc::c_char;
          }
        }
      }
    }
  }
  /* Hmm, 1=2 ? */
  //TODO: actually, bash allows ++7 but for some reason it evals to 7, not 8
  return b"arithmetic syntax error\x00" as *const u8 as *const libc::c_char;
}
/* longest must be first */
static mut op_tokens: [libc::c_char; 141] = [
  '<' as i32 as libc::c_char,
  '<' as i32 as libc::c_char,
  '=' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (6i32 << 5i32 | 2i32) as libc::c_char,
  '>' as i32 as libc::c_char,
  '>' as i32 as libc::c_char,
  '=' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (7i32 << 5i32 | 2i32) as libc::c_char,
  '<' as i32 as libc::c_char,
  '<' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (0i32 << 5i32 | 12i32) as libc::c_char,
  '>' as i32 as libc::c_char,
  '>' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (1i32 << 5i32 | 12i32) as libc::c_char,
  '|' as i32 as libc::c_char,
  '|' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (0i32 << 5i32 | 5i32) as libc::c_char,
  '&' as i32 as libc::c_char,
  '&' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (0i32 << 5i32 | 6i32) as libc::c_char,
  '!' as i32 as libc::c_char,
  '=' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (1i32 << 5i32 | 10i32) as libc::c_char,
  '<' as i32 as libc::c_char,
  '=' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (3i32 << 5i32 | 11i32) as libc::c_char,
  '>' as i32 as libc::c_char,
  '=' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (2i32 << 5i32 | 11i32) as libc::c_char,
  '=' as i32 as libc::c_char,
  '=' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (0i32 << 5i32 | 10i32) as libc::c_char,
  '|' as i32 as libc::c_char,
  '=' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (2i32 << 5i32 | 2i32) as libc::c_char,
  '&' as i32 as libc::c_char,
  '=' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (1i32 << 5i32 | 2i32) as libc::c_char,
  '*' as i32 as libc::c_char,
  '=' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (0i32 << 5i32 | 3i32) as libc::c_char,
  '/' as i32 as libc::c_char,
  '=' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (1i32 << 5i32 | 3i32) as libc::c_char,
  '%' as i32 as libc::c_char,
  '=' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (2i32 << 5i32 | 3i32) as libc::c_char,
  '+' as i32 as libc::c_char,
  '=' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (4i32 << 5i32 | 2i32) as libc::c_char,
  '-' as i32 as libc::c_char,
  '=' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (5i32 << 5i32 | 2i32) as libc::c_char,
  '-' as i32 as libc::c_char,
  '-' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (1i32 << 5i32 | 16i32 + 3i32) as libc::c_char,
  '^' as i32 as libc::c_char,
  '=' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (3i32 << 5i32 | 2i32) as libc::c_char,
  '+' as i32 as libc::c_char,
  '+' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (0i32 << 5i32 | 16i32 + 3i32) as libc::c_char,
  '*' as i32 as libc::c_char,
  '*' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (1i32 << 5i32 | 15i32) as libc::c_char,
  '!' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (1i32 << 5i32 | 16i32) as libc::c_char,
  '<' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (0i32 << 5i32 | 11i32) as libc::c_char,
  '>' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (1i32 << 5i32 | 11i32) as libc::c_char,
  '=' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (0i32 << 5i32 | 2i32) as libc::c_char,
  '|' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (0i32 << 5i32 | 7i32) as libc::c_char,
  '&' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (0i32 << 5i32 | 9i32) as libc::c_char,
  '*' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (0i32 << 5i32 | 14i32) as libc::c_char,
  '/' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (1i32 << 5i32 | 14i32) as libc::c_char,
  '%' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (2i32 << 5i32 | 14i32) as libc::c_char,
  '+' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (0i32 << 5i32 | 13i32) as libc::c_char,
  '-' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (1i32 << 5i32 | 13i32) as libc::c_char,
  '^' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (0i32 << 5i32 | 8i32) as libc::c_char,
  '~' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (0i32 << 5i32 | 16i32) as libc::c_char,
  ',' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (0i32 << 5i32 | 1i32) as libc::c_char,
  '?' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (0i32 << 5i32 | 4i32) as libc::c_char,
  ':' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (1i32 << 5i32 | 4i32) as libc::c_char,
  ')' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (1i32 << 5i32 | 16i32 + 4i32) as libc::c_char,
  '(' as i32 as libc::c_char,
  0i32 as libc::c_char,
  (0i32 << 5i32 | 0i32) as libc::c_char,
  0i32 as libc::c_char,
];
unsafe extern "C" fn strto_arith_t(
  mut nptr: *const libc::c_char,
  mut endptr: *mut *mut libc::c_char,
) -> arith_t {
  let mut base: libc::c_uint = 0;
  let mut n: arith_t = 0;
  n = strtoull(nptr, endptr, 0i32) as arith_t;
  if **endptr as libc::c_int != '#' as i32
    || ((*nptr as libc::c_int) < '1' as i32 || *nptr as libc::c_int > '9' as i32)
    || (n < 2i32 as libc::c_longlong || n > 64i32 as libc::c_longlong)
  {
    return n;
  }
  /* It's "N#nnnn" or "NN#nnnn" syntax, NN can't start with 0,
   * NN is in 2..64 range.
   */
  base = n as libc::c_uint;
  n = 0i32 as arith_t;
  nptr = (*endptr).offset(1);
  loop {
    let mut digit: libc::c_uint = (*nptr as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint);
    if digit >= 10i32 as libc::c_uint && digit <= ('z' as i32 - '0' as i32) as libc::c_uint {
      /* needed to reject e.g. $((64#~)) */
      /* in bases up to 36, case does not matter for a-z */
      digit = ((*nptr as libc::c_int | 0x20i32) as libc::c_uint)
        .wrapping_sub(('a' as i32 - 10i32) as libc::c_uint);
      if base > 36i32 as libc::c_uint && *nptr as libc::c_int <= '_' as i32 {
        /* otherwise, A-Z,@,_ are 36-61,62,63 */
        if *nptr as libc::c_int == '_' as i32 {
          digit = 63i32 as libc::c_uint
        } else if *nptr as libc::c_int == '@' as i32 {
          digit = 62i32 as libc::c_uint
        } else {
          if !(digit < 36i32 as libc::c_uint) {
            break;
          }
          /* A-Z */
          digit = digit.wrapping_add((36i32 - 10i32) as libc::c_uint)
        }
        /* error: one of [\]^ */
      }
      //bb_error_msg("ch:'%c'%d digit:%u", *nptr, *nptr, digit);
      //if (digit < 10) - example where we need this?
      //	break;
    }
    if digit >= base {
      break;
    }
    /* bash does not check for overflows */
    n = n * base as libc::c_longlong + digit as libc::c_longlong;
    nptr = nptr.offset(1)
  }
  /* Note: we do not set errno on bad chars, we just set a pointer
   * to the first invalid char. For example, this allows
   * "N#" (empty "nnnn" part): 64#+1 is a valid expression,
   * it means 64# + 1, whereas 64#~... is not, since ~ is not a valid
   * operator.
   */
  *endptr = nptr as *mut libc::c_char;
  return n;
}
/* !ENABLE_FEATURE_SH_MATH_BASE */
unsafe extern "C" fn evaluate_string(
  mut math_state: *mut arith_state_t,
  mut expr: *const libc::c_char,
) -> arith_t {
  let mut var_name_size: size_t = 0;
  let mut current_block: u64;
  let mut lasttok: operator = 0;
  let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
  expr = skip_whitespace(expr);
  let mut start_expr: *const libc::c_char = expr;
  let mut expr_len: libc::c_uint = strlen(expr).wrapping_add(2i32 as libc::c_ulong) as libc::c_uint;
  /* Stack of integers */
  /* The proof that there can be no more than strlen(startbuf)/2+1
   * integers in any given correct or incorrect expression
   * is left as an exercise to the reader. */
  let mut fresh0 = ::std::vec::from_elem(
    0,
    (expr_len.wrapping_div(2i32 as libc::c_uint) as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<var_or_num_t>() as libc::c_ulong) as usize,
  );
  let numstack: *mut var_or_num_t = fresh0.as_mut_ptr() as *mut var_or_num_t;
  let mut numstackptr: *mut var_or_num_t = numstack;
  /* Stack of operator tokens */
  let mut fresh1 = ::std::vec::from_elem(
    0,
    (expr_len as libc::c_ulong).wrapping_mul(::std::mem::size_of::<operator>() as libc::c_ulong)
      as usize,
  );
  let stack: *mut operator = fresh1.as_mut_ptr() as *mut operator;
  let mut stackptr: *mut operator = stack;
  /* Start with a left paren */
  lasttok = (0i32 << 5i32 | 0i32) as operator; /* while (1) */
  let fresh2 = stackptr;
  stackptr = stackptr.offset(1);
  *fresh2 = lasttok;
  errmsg = 0 as *const libc::c_char;
  's_37: loop {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut op: operator = 0;
    let mut prec: operator = 0;
    let mut arithval: libc::c_char = 0;
    expr = skip_whitespace(expr);
    arithval = *expr;
    if arithval as libc::c_int == '\u{0}' as i32 {
      if expr == start_expr {
        /* Null expression */
        (*numstack).val = 0i32 as arith_t;
        current_block = 11386481267603146021;
        break;
      } else if expr
        != (&*op_tokens.as_ptr().offset(
          (::std::mem::size_of::<[libc::c_char; 141]>() as libc::c_ulong)
            .wrapping_sub(7i32 as libc::c_ulong) as isize,
        ) as *const libc::c_char)
          .offset(1)
      {
        /* This is only reached after all tokens have been extracted from the
         * input stream. If there are still tokens on the operator stack, they
         * are to be applied in order. At the end, there should be a final
         * result on the integer stack */
        /* If we haven't done so already,
         * append a closing right paren
         * and let the loop process it */
        expr = &*op_tokens.as_ptr().offset(
          (::std::mem::size_of::<[libc::c_char; 141]>() as libc::c_ulong)
            .wrapping_sub(7i32 as libc::c_ulong) as isize,
        ) as *const libc::c_char
      } else if numstackptr != numstack.offset(1) {
        /* At this point, we're done with the expression */
        /* ...but if there isn't, it's bad */
        current_block = 4372395669998863707;
        break;
      } else {
        if !(*numstack).var.is_null() {
          /* expression is $((var)) only, lookup now */
          errmsg = arith_lookup_val(math_state, numstack)
        }
        current_block = 11386481267603146021;
        break;
      }
    } else {
      p = endofname(expr);
      if p != expr {
        /* Name */
        var_name_size =
          (p.wrapping_offset_from(expr) as libc::c_long + 1) as size_t; /* +1 for NUL */
        let mut fresh3 = ::std::vec::from_elem(0, var_name_size as usize);
        (*numstackptr).var = fresh3.as_mut_ptr() as *mut libc::c_char;
        safe_strncpy((*numstackptr).var, expr, var_name_size);
        expr = p
      } else if (arithval as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
        /* Number */
        (*numstackptr).var = 0 as *mut libc::c_char; /* bash compat */
        *bb_errno = 0i32;
        (*numstackptr).val = strto_arith_t(
          expr,
          &mut expr as *mut *const libc::c_char as *mut *mut libc::c_char,
        );
        if *bb_errno != 0 {
          (*numstackptr).val = 0i32 as arith_t
        }
      } else {
        /* Should be an operator */
        /* Special case: NUM-- and NUM++ are not recognized if NUM
         * is a literal number, not a variable. IOW:
         * "a+++v" is a++ + v.
         * "7+++v" is 7 + ++v, not 7++ + v.
         */
        if lasttok as libc::c_int == 0i32 << 5i32 | 16i32 + 4i32
          && (*numstackptr.offset(-1i32 as isize)).var.is_null()
          && (*expr.offset(0) as libc::c_int == '+' as i32
            || *expr.offset(0) as libc::c_int == '-' as i32)
          && *expr.offset(1) as libc::c_int == *expr.offset(0) as libc::c_int
        {
          //bb_error_msg("special %c%c", expr[0], expr[0]);
          op = if *expr.offset(0) as libc::c_int == '+' as i32 {
            (0i32 << 5i32) | 13i32
          } else {
            (1i32 << 5i32) | 13i32
          } as operator;
          expr = expr.offset(1)
        } else {
          p = op_tokens.as_ptr();
          's_213: loop
          /* Compare expr to current op_tokens[] element */
          {
            let mut e: *const libc::c_char = expr;
            loop {
              if *p as libc::c_int == '\u{0}' as i32 {
                /* Match: operator is found */
                expr = e;
                break 's_213;
              } else {
                if *p as libc::c_int != *e as libc::c_int {
                  break;
                }
                p = p.offset(1);
                e = e.offset(1)
              }
            }
            /* No match, go to next element of op_tokens[] */
            while *p != 0 {
              p = p.offset(1)
            } /* skip NUL and TOK_foo bytes */
            p = p.offset(2);
            if !(*p as libc::c_int == '\u{0}' as i32) {
              continue;
            }
            /* No next element, operator not found */
            //math_state->syntax_error_at = expr;
            current_block = 4372395669998863707; /* fetch TOK_foo value */
            break 's_37;
          }
          op = *p.offset(1) as operator
        }
        /* NB: expr now points past the operator */
        /* post grammar: a++ reduce to num */
        if lasttok as libc::c_int == 0i32 << 5i32 | 16i32 + 3i32
          || lasttok as libc::c_int == 1i32 << 5i32 | 16i32 + 3i32
        {
          lasttok = (0i32 << 5i32 | 16i32 + 4i32) as operator
        }
        /* Plus and minus are binary (not unary) _only_ if the last
         * token was a number, or a right paren (which pretends to be
         * a number, since it evaluates to one). Think about it.
         * It makes sense. */
        if lasttok as libc::c_int != 0i32 << 5i32 | 16i32 + 4i32 {
          match op as libc::c_int {
            13 => op = (1i32 << 5i32 | 16i32 + 1i32) as operator,
            45 => op = (0i32 << 5i32 | 16i32 + 1i32) as operator,
            19 => op = (0i32 << 5i32 | 16i32 + 2i32) as operator,
            51 => op = (1i32 << 5i32 | 16i32 + 2i32) as operator,
            _ => {}
          }
        }
        /* We don't want an unary operator to cause recursive descent on the
         * stack, because there can be many in a row and it could cause an
         * operator to be evaluated before its argument is pushed onto the
         * integer stack.
         * But for binary operators, "apply" everything on the operator
         * stack until we find an operator with a lesser priority than the
         * one we have just extracted. If op is right-associative,
         * then stop "applying" on the equal priority too.
         * Left paren is given the lowest priority so it will never be
         * "applied" in this way.
         */
        prec = (op as libc::c_int & 0x1fi32) as operator;
        if prec as libc::c_int > 0i32 && (prec as libc::c_int) < 16i32
          || prec as libc::c_int == 16i32 + 4i32
        {
          /* not left paren or unary */
          if lasttok as libc::c_int != 0i32 << 5i32 | 16i32 + 4i32 {
            current_block = 4372395669998863707;
            break;
          }
          while stackptr != stack {
            stackptr = stackptr.offset(-1);
            let mut prev_op: operator = *stackptr;
            if op as libc::c_int == 1i32 << 5i32 | 16i32 + 4i32 {
              /* The algorithm employed here is simple: while we don't
               * hit an open paren nor the bottom of the stack, pop
               * tokens and apply them */
              if prev_op as libc::c_int == 0i32 << 5i32 | 0i32 {
                /* Any operator directly after a
                 * close paren should consider itself binary */
                lasttok = (0i32 << 5i32 | 16i32 + 4i32) as operator;
                continue 's_37;
              }
            } else {
              let mut prev_prec: operator = (prev_op as libc::c_int & 0x1fi32) as operator;
              if prec as libc::c_int == 3i32 {
                prec = 2i32 as operator
              }
              if prev_prec as libc::c_int == 3i32 {
                prev_prec = 2i32 as operator
              }
              if (prev_prec as libc::c_int) < prec as libc::c_int
                || prev_prec as libc::c_int == prec as libc::c_int
                  && is_right_associative(prec) != 0
              {
                stackptr = stackptr.offset(1);
                break;
              }
            }
            errmsg = arith_apply(math_state, prev_op, numstack, &mut numstackptr);
            if !errmsg.is_null() {
              current_block = 8978805484006108354;
              break 's_37;
            }
          }
          if op as libc::c_int == 1i32 << 5i32 | 16i32 + 4i32 {
            current_block = 4372395669998863707;
            break;
          }
        }
        /* Push this operator to the stack and remember it */
        lasttok = op;
        let fresh4 = stackptr;
        stackptr = stackptr.offset(1);
        *fresh4 = lasttok;
        continue;
      }
      (*numstackptr).second_val_present = 0i32 as libc::c_char;
      numstackptr = numstackptr.offset(1);
      lasttok = (0i32 << 5i32 | 16i32 + 4i32) as operator
    }
  }
  match current_block {
    4372395669998863707 =>
    /* binary op must be preceded by a num */
    {
      errmsg = b"arithmetic syntax error\x00" as *const u8 as *const libc::c_char;
      current_block = 8978805484006108354;
    }
    _ => {}
  }
  match current_block {
    8978805484006108354 => (*numstack).val = -1i32 as arith_t,
    _ => {}
  }
  (*math_state).errmsg = errmsg;
  return (*numstack).val;
}
#[no_mangle]
pub unsafe extern "C" fn arith(
  mut math_state: *mut arith_state_t,
  mut expr: *const libc::c_char,
) -> arith_t {
  (*math_state).errmsg = 0 as *const libc::c_char;
  (*math_state).list_of_recursed_names = 0 as *mut libc::c_void;
  return evaluate_string(math_state, expr);
}
/*
 * Copyright (c) 1989, 1991, 1993, 1994
 *      The Regents of the University of California.  All rights reserved.
 *
 * This code is derived from software contributed to Berkeley by
 * Kenneth Almquist.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ''AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */
