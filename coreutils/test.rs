use crate::librb::size_t;
use crate::libbb::ptr_to_globals::bb_errno;
use crate::libbb::appletlib::applet_name;
use libc;
use libc::free;
use libc::getegid;
use libc::geteuid;
use libc::getgid;
use libc::gid_t;
use libc::isatty;
use libc::lstat;
use libc::sigset_t;
use libc::stat;
use libc::strcmp;
extern "C" {
  #[no_mangle]
  fn strtoll(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_longlong;

  #[no_mangle]
  fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
  #[no_mangle]
  fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;


  #[no_mangle]
  fn skip_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn bb_basename(name: *const libc::c_char) -> *const libc::c_char;
  #[no_mangle]
  fn bb_getgroups(ngroups: *mut libc::c_int, group_array: *mut gid_t) -> *mut gid_t;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn index_in_strings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  /* See test_ptr_hack.c */
  #[no_mangle]
  static test_ptr_to_statics: *mut test_statics;
}
pub type __int64_t = libc::c_long;
pub type int64_t = __int64_t;

pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
  pub __jmpbuf: __jmp_buf,
  pub __mask_was_saved: libc::c_int,
  pub __saved_mask: sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
/* We try to minimize both static and stack usage. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct test_statics {
  pub args: *mut *mut libc::c_char,
  pub last_operator: *const operator_t,
  pub group_array: *mut gid_t,
  pub ngroups: libc::c_int,
  pub leaving: jmp_buf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct operator_t {
  pub op_num: libc::c_uchar,
  pub op_type: libc::c_uchar,
}
pub type number_t = int64_t;

/*
 * test implementation for busybox
 *
 * Copyright (c) by a whole pile of folks:
 *
 *     test(1); version 7-like  --  author Erik Baalbergen
 *     modified by Eric Gisin to be used as built-in.
 *     modified by Arnold Robbins to add SVR3 compatibility
 *     (-x -c -b -p -u -g -k) plus Korn's -L -nt -ot -ef and new -S (socket).
 *     modified by J.T. Conklin for NetBSD.
 *     modified by Herbert Xu to be used as built-in in ash.
 *     modified by Erik Andersen <andersen@codepoet.org> to be used
 *     in busybox.
 *     modified by Bernhard Reutner-Fischer to be useable (i.e. a bit less bloaty).
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 *
 * Original copyright notice states:
 *     "This program is in the Public Domain."
 */
//config:config TEST
//config:	bool "test (4.1 kb)"
//config:	default y
//config:	help
//config:	test is used to check file types and compare values,
//config:	returning an appropriate exit code. The bash shell
//config:	has test built in, ash can build it in optionally.
//config:
//config:config TEST1
//config:	bool "test as ["
//config:	default y
//config:	help
//config:	Provide test command in the "[ EXPR ]" form
//config:
//config:config TEST2
//config:	bool "test as [["
//config:	default y
//config:	help
//config:	Provide test command in the "[[ EXPR ]]" form
//config:
//config:config FEATURE_TEST_64
//config:	bool "Extend test to 64 bit"
//config:	default y
//config:	depends on TEST || TEST1 || TEST2 || ASH_TEST || HUSH_TEST
//config:	help
//config:	Enable 64-bit support in test.
//applet:IF_TEST(APPLET_NOFORK(test, test, BB_DIR_USR_BIN, BB_SUID_DROP, test))
//applet:IF_TEST1(APPLET_NOFORK([,   test, BB_DIR_USR_BIN, BB_SUID_DROP, test))
//applet:IF_TEST2(APPLET_NOFORK([[,  test, BB_DIR_USR_BIN, BB_SUID_DROP, test))
//kbuild:lib-$(CONFIG_TEST)  += test.o test_ptr_hack.o
//kbuild:lib-$(CONFIG_TEST1) += test.o test_ptr_hack.o
//kbuild:lib-$(CONFIG_TEST2) += test.o test_ptr_hack.o
//kbuild:lib-$(CONFIG_ASH_TEST)  += test.o test_ptr_hack.o
//kbuild:lib-$(CONFIG_HUSH_TEST) += test.o test_ptr_hack.o
/* "test --help" is special-cased to ignore --help */
//usage:#define test_trivial_usage NOUSAGE_STR
//usage:#define test_full_usage ""
//usage:
//usage:#define test_example_usage
//usage:       "$ test 1 -eq 2\n"
//usage:       "$ echo $?\n"
//usage:       "1\n"
//usage:       "$ test 1 -eq 1\n"
//usage:       "$ echo $?\n"
//usage:       "0\n"
//usage:       "$ [ -d /etc ]\n"
//usage:       "$ echo $?\n"
//usage:       "0\n"
//usage:       "$ [ -d /junk ]\n"
//usage:       "$ echo $?\n"
//usage:       "1\n"
/* This is a NOFORK applet. Be very careful! */
/* test_main() is called from shells, and we need to be extra careful here.
 * This is true regardless of PREFER_APPLETS and SH_STANDALONE
 * state. */
/* test(1) accepts the following grammar:
  oexpr   ::= aexpr | aexpr "-o" oexpr ;
  aexpr   ::= nexpr | nexpr "-a" aexpr ;
  nexpr   ::= primary | "!" primary
  primary ::= unary-operator operand
    | operand binary-operator operand
    | operand
    | "(" oexpr ")"
    ;
  unary-operator ::= "-r"|"-w"|"-x"|"-f"|"-d"|"-c"|"-b"|"-p"|
    "-u"|"-g"|"-k"|"-s"|"-t"|"-z"|"-n"|"-o"|"-O"|"-G"|"-L"|"-S";

  binary-operator ::= "="|"=="|"!="|"-eq"|"-ne"|"-ge"|"-gt"|"-le"|"-lt"|
      "-nt"|"-ot"|"-ef";
  operand ::= <any legal UNIX file name>
*/
/* TODO: handle [[ expr ]] bashism bash-compatibly.
 * [[ ]] is meant to be a "better [ ]", with less weird syntax
 * and without the risk of variables and quoted strings misinterpreted
 * as operators.
 * This will require support from shells - we need to know quote status
 * of each parameter (see below).
 *
 * Word splitting and pathname expansion should NOT be performed:
 *      # a="a b"; [[ $a = "a b" ]] && echo YES
 *      YES
 *      # [[ /bin/m* ]] && echo YES
 *      YES
 *
 * =~ should do regexp match
 * = and == should do pattern match against right side:
 *      # [[ *a* == bab ]] && echo YES
 *      # [[ bab == *a* ]] && echo YES
 *      YES
 * != does the negated == (i.e., also with pattern matching).
 * Pattern matching is quotation-sensitive:
 *      # [[ bab == "b"a* ]] && echo YES
 *      YES
 *      # [[ bab == b"a*" ]] && echo YES
 *
 * Conditional operators such as -f must be unquoted literals to be recognized:
 *      # [[ -e /bin ]] && echo YES
 *      YES
 *      # [[ '-e' /bin ]] && echo YES
 *      bash: conditional binary operator expected...
 *      # A='-e'; [[ $A /bin ]] && echo YES
 *      bash: conditional binary operator expected...
 *
 * || and && should work as -o and -a work in [ ]
 * -a and -o aren't recognized (&& and || are to be used instead)
 * ( and ) do not need to be quoted unlike in [ ]:
 *      # [[ ( abc ) && '' ]] && echo YES
 *      # [[ ( abc ) || '' ]] && echo YES
 *      YES
 *      # [[ ( abc ) -o '' ]] && echo YES
 *      bash: syntax error in conditional expression...
 *
 * Apart from the above, [[ expr ]] should work as [ expr ]
 */
pub type token = libc::c_uint;
pub const OPERAND: token = 39;
pub const RPAREN: token = 38;
pub const LPAREN: token = 37;
pub const BOR: token = 36;
pub const BAND: token = 35;
pub const UNOT: token = 34;
pub const INTLT: token = 33;
pub const INTLE: token = 32;
pub const INTGT: token = 31;
pub const INTGE: token = 30;
/* int ops */
pub const INTNE: token = 29;
pub const INTEQ: token = 28;
pub const STRGT: token = 27;
pub const STRLT: token = 26;
pub const STRNE: token = 25;
pub const STREQ: token = 24;
/* str ops */
pub const STRNZ: token = 23;
pub const STREZ: token = 22;
pub const FILGID: token = 21;
pub const FILUID: token = 20;
pub const FILEQ: token = 19;
/* file ops */
pub const FILOT: token = 18;
pub const FILNT: token = 17;
pub const FILSTCK: token = 16;
/* file bit */
pub const FILSGID: token = 15;
pub const FILSUID: token = 14;
pub const FILTT: token = 13;
pub const FILGZ: token = 12;
pub const FILSYM: token = 11;
pub const FILSOCK: token = 10;
pub const FILFIFO: token = 9;
pub const FILBDEV: token = 8;
pub const FILCDEV: token = 7;
/* file type */
pub const FILDIR: token = 6;
pub const FILREG: token = 5;
pub const FILEXIST: token = 4;
pub const FILEX: token = 3;
/* file access */
pub const FILWR: token = 2;
pub const FILRD: token = 1;
pub const EOI: token = 0;
pub const PAREN: C2RustUnnamed = 4;
pub const BBINOP: C2RustUnnamed = 3;
pub const BUNOP: C2RustUnnamed = 2;
pub const BINOP: C2RustUnnamed = 1;
pub const UNOP: C2RustUnnamed = 0;
pub type C2RustUnnamed = libc::c_uint;
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
static mut ops_table: [operator_t; 40] = [
  {
    let mut init = operator_t {
      op_num: FILRD as libc::c_int as libc::c_uchar,
      op_type: UNOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: FILWR as libc::c_int as libc::c_uchar,
      op_type: UNOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: FILEX as libc::c_int as libc::c_uchar,
      op_type: UNOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: FILEXIST as libc::c_int as libc::c_uchar,
      op_type: UNOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: FILREG as libc::c_int as libc::c_uchar,
      op_type: UNOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: FILDIR as libc::c_int as libc::c_uchar,
      op_type: UNOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: FILCDEV as libc::c_int as libc::c_uchar,
      op_type: UNOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: FILBDEV as libc::c_int as libc::c_uchar,
      op_type: UNOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: FILFIFO as libc::c_int as libc::c_uchar,
      op_type: UNOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: FILSUID as libc::c_int as libc::c_uchar,
      op_type: UNOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: FILSGID as libc::c_int as libc::c_uchar,
      op_type: UNOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: FILSTCK as libc::c_int as libc::c_uchar,
      op_type: UNOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: FILGZ as libc::c_int as libc::c_uchar,
      op_type: UNOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: FILTT as libc::c_int as libc::c_uchar,
      op_type: UNOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: STREZ as libc::c_int as libc::c_uchar,
      op_type: UNOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: STRNZ as libc::c_int as libc::c_uchar,
      op_type: UNOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: FILSYM as libc::c_int as libc::c_uchar,
      op_type: UNOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: FILUID as libc::c_int as libc::c_uchar,
      op_type: UNOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: FILGID as libc::c_int as libc::c_uchar,
      op_type: UNOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: FILSYM as libc::c_int as libc::c_uchar,
      op_type: UNOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: FILSOCK as libc::c_int as libc::c_uchar,
      op_type: UNOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: STREQ as libc::c_int as libc::c_uchar,
      op_type: BINOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: STREQ as libc::c_int as libc::c_uchar,
      op_type: BINOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: STRNE as libc::c_int as libc::c_uchar,
      op_type: BINOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: STRLT as libc::c_int as libc::c_uchar,
      op_type: BINOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: STRGT as libc::c_int as libc::c_uchar,
      op_type: BINOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: INTEQ as libc::c_int as libc::c_uchar,
      op_type: BINOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: INTNE as libc::c_int as libc::c_uchar,
      op_type: BINOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: INTGE as libc::c_int as libc::c_uchar,
      op_type: BINOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: INTGT as libc::c_int as libc::c_uchar,
      op_type: BINOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: INTLE as libc::c_int as libc::c_uchar,
      op_type: BINOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: INTLT as libc::c_int as libc::c_uchar,
      op_type: BINOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: FILNT as libc::c_int as libc::c_uchar,
      op_type: BINOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: FILOT as libc::c_int as libc::c_uchar,
      op_type: BINOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: FILEQ as libc::c_int as libc::c_uchar,
      op_type: BINOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: UNOT as libc::c_int as libc::c_uchar,
      op_type: BUNOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: BAND as libc::c_int as libc::c_uchar,
      op_type: BBINOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: BOR as libc::c_int as libc::c_uchar,
      op_type: BBINOP as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: LPAREN as libc::c_int as libc::c_uchar,
      op_type: PAREN as libc::c_int as libc::c_uchar,
    };
    init
  },
  {
    let mut init = operator_t {
      op_num: RPAREN as libc::c_int as libc::c_uchar,
      op_type: PAREN as libc::c_int as libc::c_uchar,
    };
    init
  },
];
/* Please keep these two tables in sync */
static mut ops_texts: [libc::c_char; 124] = [
  45, 114, 0, 45, 119, 0, 45, 120, 0, 45, 101, 0, 45, 102, 0, 45, 100, 0, 45, 99, 0, 45, 98, 0, 45,
  112, 0, 45, 117, 0, 45, 103, 0, 45, 107, 0, 45, 115, 0, 45, 116, 0, 45, 122, 0, 45, 110, 0, 45,
  104, 0, 45, 79, 0, 45, 71, 0, 45, 76, 0, 45, 83, 0, 61, 0, 61, 61, 0, 33, 61, 0, 60, 0, 62, 0,
  45, 101, 113, 0, 45, 110, 101, 0, 45, 103, 101, 0, 45, 103, 116, 0, 45, 108, 101, 0, 45, 108,
  116, 0, 45, 110, 116, 0, 45, 111, 116, 0, 45, 101, 102, 0, 33, 0, 45, 97, 0, 45, 111, 0, 40, 0,
  41, 0, 0,
];
unsafe extern "C" fn syntax(mut op: *const libc::c_char, mut msg: *const libc::c_char) -> ! {
  if !op.is_null() && *op as libc::c_int != 0 {
    bb_error_msg(b"%s: %s\x00" as *const u8 as *const libc::c_char, op, msg);
  } else {
    bb_error_msg(
      (b"%s: %s\x00" as *const u8 as *const libc::c_char).offset(4),
      msg,
    );
  }
  longjmp((*test_ptr_to_statics).leaving.as_mut_ptr(), 2i32);
}
/* atoi with error detection */
//XXX: FIXME: duplicate of existing libbb function?
unsafe extern "C" fn getn(mut s: *const libc::c_char) -> number_t {
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut r: libc::c_longlong = 0;
  *bb_errno = 0i32;
  r = strtoll(s, &mut p, 10i32);
  if *bb_errno != 0i32 {
    syntax(s, b"out of range\x00" as *const u8 as *const libc::c_char);
  }
  if p == s as *mut libc::c_char || *skip_whitespace(p) as libc::c_int != '\u{0}' as i32 {
    syntax(s, b"bad number\x00" as *const u8 as *const libc::c_char);
  }
  return r as number_t;
}
/* UNUSED
static int newerf(const char *f1, const char *f2)
{
  struct stat b1, b2;

  return (stat(f1, &b1) == 0 &&
      stat(f2, &b2) == 0 && b1.st_mtimee > b2.st_mtimee);
}

static int olderf(const char *f1, const char *f2)
{
  struct stat b1, b2;

  return (stat(f1, &b1) == 0 &&
      stat(f2, &b2) == 0 && b1.st_mtimee < b2.st_mtimee);
}

static int equalf(const char *f1, const char *f2)
{
  struct stat b1, b2;

  return (stat(f1, &b1) == 0 &&
      stat(f2, &b2) == 0 &&
      b1.st_dev == b2.st_dev && b1.st_ino == b2.st_ino);
}
*/
unsafe extern "C" fn check_operator(mut s: *const libc::c_char) -> token {
  static mut no_op: operator_t = {
    let mut init = operator_t {
      op_num: -1i32 as libc::c_uchar,
      op_type: -1i32 as libc::c_uchar,
    };
    init
  };
  let mut n: libc::c_int = 0;
  (*test_ptr_to_statics).last_operator = &no_op;
  if s.is_null() {
    return EOI;
  }
  n = index_in_strings(ops_texts.as_ptr(), s);
  if n < 0i32 {
    return OPERAND;
  }
  (*test_ptr_to_statics).last_operator =
    &*ops_table.as_ptr().offset(n as isize) as *const operator_t;
  return ops_table[n as usize].op_num as token;
}
unsafe extern "C" fn binop() -> libc::c_int {
  let mut opnd1: *const libc::c_char = 0 as *const libc::c_char;
  let mut opnd2: *const libc::c_char = 0 as *const libc::c_char;
  let mut op: *const operator_t = 0 as *const operator_t;
  let mut val1: number_t = 0;
  let mut val2: number_t = 0;
  opnd1 = *(*test_ptr_to_statics).args;
  (*test_ptr_to_statics).args = (*test_ptr_to_statics).args.offset(1);
  check_operator(*(*test_ptr_to_statics).args);
  op = (*test_ptr_to_statics).last_operator;
  (*test_ptr_to_statics).args = (*test_ptr_to_statics).args.offset(1);
  opnd2 = *(*test_ptr_to_statics).args;
  if opnd2.is_null() {
    syntax(
      *(*test_ptr_to_statics).args.offset(-1i32 as isize),
      b"argument expected\x00" as *const u8 as *const libc::c_char,
    );
  }
  if ((*op).op_num as libc::c_int - INTEQ as libc::c_int) as libc::c_uchar as libc::c_int <= 5i32 {
    val1 = getn(opnd1);
    val2 = getn(opnd2);
    if (*op).op_num as libc::c_int == INTEQ as libc::c_int {
      return (val1 == val2) as libc::c_int;
    }
    if (*op).op_num as libc::c_int == INTNE as libc::c_int {
      return (val1 != val2) as libc::c_int;
    }
    if (*op).op_num as libc::c_int == INTGE as libc::c_int {
      return (val1 >= val2) as libc::c_int;
    }
    if (*op).op_num as libc::c_int == INTGT as libc::c_int {
      return (val1 > val2) as libc::c_int;
    }
    if (*op).op_num as libc::c_int == INTLE as libc::c_int {
      return (val1 <= val2) as libc::c_int;
    }
    /*if (op->op_num == INTLT)*/
    return (val1 < val2) as libc::c_int;
  }
  if ((*op).op_num as libc::c_int - STREZ as libc::c_int) as libc::c_uchar as libc::c_int <= 5i32 {
    val1 = strcmp(opnd1, opnd2) as number_t;
    if (*op).op_num as libc::c_int == STREQ as libc::c_int {
      return (val1 == 0) as libc::c_int;
    }
    if (*op).op_num as libc::c_int == STRNE as libc::c_int {
      return (val1 != 0) as libc::c_int;
    }
    if (*op).op_num as libc::c_int == STRLT as libc::c_int {
      return (val1 < 0) as libc::c_int;
    }
    /*if (op->op_num == STRGT)*/
    return (val1 > 0) as libc::c_int;
  }
  /* We are sure that these three are by now the only binops we didn't check
   * yet, so we do not check if the class is correct:
   */
  /*	if (is_file_op(op->op_num)) */
  let mut b1: stat = std::mem::zeroed(); /* false, since at least one stat failed */
  let mut b2: stat = std::mem::zeroed();
  if stat(opnd1, &mut b1) != 0 || stat(opnd2, &mut b2) != 0 {
    return 0i32;
  }
  if (*op).op_num as libc::c_int == FILNT as libc::c_int {
    return (b1.st_mtime > b2.st_mtime) as libc::c_int;
  }
  if (*op).op_num as libc::c_int == FILOT as libc::c_int {
    return (b1.st_mtime < b2.st_mtime) as libc::c_int;
  }
  /*if (op->op_num == FILEQ)*/
  return (b1.st_dev == b2.st_dev && b1.st_ino == b2.st_ino) as libc::c_int;
  /*return 1; - NOTREACHED */
}
unsafe extern "C" fn initialize_group_array() {
  (*test_ptr_to_statics).group_array =
    bb_getgroups(&mut (*test_ptr_to_statics).ngroups, 0 as *mut gid_t);
}
/* Return non-zero if GID is one that we have in our groups list. */
//XXX: FIXME: duplicate of existing libbb function?
// see toplevel TODO file:
// possible code duplication ingroup() and is_a_group_member()
unsafe extern "C" fn is_a_group_member(mut gid: gid_t) -> libc::c_int {
  let mut i: libc::c_int = 0;
  /* Short-circuit if possible, maybe saving a call to getgroups(). */
  if gid == getgid() || gid == getegid() {
    return 1i32;
  }
  if (*test_ptr_to_statics).ngroups == 0i32 {
    initialize_group_array();
  }
  /* Search through the list looking for GID. */
  i = 0i32;
  while i < (*test_ptr_to_statics).ngroups {
    if gid == *(*test_ptr_to_statics).group_array.offset(i as isize) {
      return 1i32;
    }
    i += 1
  }
  return 0i32;
}
/* Do the same thing access(2) does, but use the effective uid and gid,
and don't make the mistake of telling root that any file is
executable. */
unsafe extern "C" fn test_eaccess(mut st: *mut stat, mut mode: libc::c_int) -> libc::c_int {
  let mut euid: libc::c_uint = geteuid();
  if euid == 0i32 as libc::c_uint {
    /* Root can read or write any file. */
    if mode != 1i32 {
      return 0i32;
    }
    /* Root can execute any file that has any one of the execute
     * bits set. */
    if (*st).st_mode & (0o100i32 | 0o100i32 >> 3i32 | 0o100i32 >> 3i32 >> 3i32) as libc::c_uint != 0
    {
      return 0i32;
    }
  }
  if (*st).st_uid == euid {
    /* owner */
    mode <<= 6i32
  } else if is_a_group_member((*st).st_gid) != 0 {
    mode <<= 3i32
  } /* gcc 3.x thinks it can be used uninitialized */
  if (*st).st_mode & mode as libc::c_uint != 0 {
    return 0i32;
  }
  return -1i32;
}
unsafe extern "C" fn filstat(mut nm: *mut libc::c_char, mut mode: token) -> libc::c_int {
  let mut s: stat = std::mem::zeroed();
  let mut i: libc::c_uint = 0;
  i = i;
  if mode as libc::c_uint == FILSYM as libc::c_int as libc::c_uint {
    if lstat(nm, &mut s) == 0i32 {
      i = 0o120000i32 as libc::c_uint
    } else {
      return 0i32;
    }
  } else {
    if stat(nm, &mut s) != 0i32 {
      return 0i32;
    }
    if mode as libc::c_uint == FILEXIST as libc::c_int as libc::c_uint {
      return 1i32;
    }
    if (mode as libc::c_uint).wrapping_sub(FILRD as libc::c_int as libc::c_uint) as libc::c_uchar
      as libc::c_int
      <= 2i32
    {
      if mode as libc::c_uint == FILRD as libc::c_int as libc::c_uint {
        i = 4i32 as libc::c_uint
      }
      if mode as libc::c_uint == FILWR as libc::c_int as libc::c_uint {
        i = 2i32 as libc::c_uint
      }
      if mode as libc::c_uint == FILEX as libc::c_int as libc::c_uint {
        i = 1i32 as libc::c_uint
      }
      return (test_eaccess(&mut s, i as libc::c_int) == 0i32) as libc::c_int;
    }
    if (mode as libc::c_uint).wrapping_sub(FILREG as libc::c_int as libc::c_uint) as libc::c_uchar
      as libc::c_int
      <= 5i32
    {
      if mode as libc::c_uint == FILREG as libc::c_int as libc::c_uint {
        i = 0o100000i32 as libc::c_uint
      }
      if mode as libc::c_uint == FILDIR as libc::c_int as libc::c_uint {
        i = 0o40000i32 as libc::c_uint
      }
      if mode as libc::c_uint == FILCDEV as libc::c_int as libc::c_uint {
        i = 0o20000i32 as libc::c_uint
      }
      if mode as libc::c_uint == FILBDEV as libc::c_int as libc::c_uint {
        i = 0o60000i32 as libc::c_uint
      }
      if mode as libc::c_uint == FILFIFO as libc::c_int as libc::c_uint {
        i = 0o10000i32 as libc::c_uint
      }
      if mode as libc::c_uint == FILSOCK as libc::c_int as libc::c_uint {
        i = 0o140000i32 as libc::c_uint
      }
    } else {
      if (mode as libc::c_uint).wrapping_sub(FILSUID as libc::c_int as libc::c_uint)
        as libc::c_uchar as libc::c_int
        <= 2i32
      {
        if mode as libc::c_uint == FILSUID as libc::c_int as libc::c_uint {
          i = 0o4000i32 as libc::c_uint
        }
        if mode as libc::c_uint == FILSGID as libc::c_int as libc::c_uint {
          i = 0o2000i32 as libc::c_uint
        }
        if mode as libc::c_uint == FILSTCK as libc::c_int as libc::c_uint {
          i = 0o1000i32 as libc::c_uint
        }
        return (s.st_mode & i != 0i32 as libc::c_uint) as libc::c_int;
      }
      if mode as libc::c_uint == FILGZ as libc::c_int as libc::c_uint {
        return (s.st_size > 0i64) as libc::c_int;
      }
      if mode as libc::c_uint == FILUID as libc::c_int as libc::c_uint {
        return (s.st_uid == geteuid()) as libc::c_int;
      }
      if mode as libc::c_uint == FILGID as libc::c_int as libc::c_uint {
        return (s.st_gid == getegid()) as libc::c_int;
      }
      return 1i32;
    }
  }
  return (s.st_mode & 0o170000i32 as libc::c_uint == i) as libc::c_int;
  /* NOTREACHED */
}
unsafe extern "C" fn nexpr(mut n: token) -> number_t {
  let mut res: number_t = 0;
  if n as libc::c_uint == UNOT as libc::c_int as libc::c_uint {
    (*test_ptr_to_statics).args = (*test_ptr_to_statics).args.offset(1);
    n = check_operator(*(*test_ptr_to_statics).args);
    if n as libc::c_uint == EOI as libc::c_int as libc::c_uint {
      /* special case: [ ! ], [ a -a ! ] are valid */
      /* IOW, "! ARG" may miss ARG */
      (*test_ptr_to_statics).args = (*test_ptr_to_statics).args.offset(-1);
      return 1i32 as number_t;
    }
    res = (nexpr(n) == 0) as libc::c_int as number_t;
    return res;
  }
  res = primary(n);
  return res;
}
unsafe extern "C" fn aexpr(mut n: token) -> number_t {
  let mut res: number_t = 0;
  res = nexpr(n);
  (*test_ptr_to_statics).args = (*test_ptr_to_statics).args.offset(1);
  if check_operator(*(*test_ptr_to_statics).args) as libc::c_uint
    == BAND as libc::c_int as libc::c_uint
  {
    (*test_ptr_to_statics).args = (*test_ptr_to_statics).args.offset(1);
    res = (aexpr(check_operator(*(*test_ptr_to_statics).args)) != 0 && res != 0) as libc::c_int
      as number_t;
    return res;
  }
  (*test_ptr_to_statics).args = (*test_ptr_to_statics).args.offset(-1);
  return res;
}
unsafe extern "C" fn oexpr(mut n: token) -> number_t {
  let mut res: number_t = 0;
  res = aexpr(n);
  (*test_ptr_to_statics).args = (*test_ptr_to_statics).args.offset(1);
  if check_operator(*(*test_ptr_to_statics).args) as libc::c_uint
    == BOR as libc::c_int as libc::c_uint
  {
    (*test_ptr_to_statics).args = (*test_ptr_to_statics).args.offset(1);
    res = (oexpr(check_operator(*(*test_ptr_to_statics).args)) != 0 || res != 0) as libc::c_int
      as number_t;
    return res;
  }
  (*test_ptr_to_statics).args = (*test_ptr_to_statics).args.offset(-1);
  return res;
}
unsafe extern "C" fn primary(mut n: token) -> number_t {
  let mut res: number_t = 0;
  let mut args0_op: *const operator_t = 0 as *const operator_t;
  if n as libc::c_uint == EOI as libc::c_int as libc::c_uint {
    syntax(
      0 as *const libc::c_char,
      b"argument expected\x00" as *const u8 as *const libc::c_char,
    );
  }
  if n as libc::c_uint == LPAREN as libc::c_int as libc::c_uint {
    (*test_ptr_to_statics).args = (*test_ptr_to_statics).args.offset(1);
    res = oexpr(check_operator(*(*test_ptr_to_statics).args));
    (*test_ptr_to_statics).args = (*test_ptr_to_statics).args.offset(1);
    if check_operator(*(*test_ptr_to_statics).args) as libc::c_uint
      != RPAREN as libc::c_int as libc::c_uint
    {
      syntax(
        0 as *const libc::c_char,
        b"closing paren expected\x00" as *const u8 as *const libc::c_char,
      );
    }
    return res;
  }
  /* coreutils 6.9 checks "is args[1] binop and args[2] exist?" first,
   * do the same */
  args0_op = (*test_ptr_to_statics).last_operator;
  /* last_operator = operator at args[1] */
  if check_operator(*(*test_ptr_to_statics).args.offset(1)) as libc::c_uint
    != EOI as libc::c_int as libc::c_uint
  {
    /* if args[1] != NULL */
    if !(*(*test_ptr_to_statics).args.offset(2)).is_null() {
      // coreutils also does this:
      // if (args[3] && args[0]="-l" && args[2] is BINOP)
      //	return binop(1 /* prepended by -l */);
      if (*(*test_ptr_to_statics).last_operator).op_type as libc::c_int == BINOP as libc::c_int {
        return binop() as number_t;
      }
    }
  }
  /* check "is args[0] unop?" second */
  if (*args0_op).op_type as libc::c_int == UNOP as libc::c_int {
    /* unary expression */
    if !(*(*test_ptr_to_statics).args.offset(1)).is_null() {
      (*test_ptr_to_statics).args = (*test_ptr_to_statics).args.offset(1);
      if n as libc::c_uint == STREZ as libc::c_int as libc::c_uint {
        return (*(*(*test_ptr_to_statics).args.offset(0)).offset(0) as libc::c_int
          == '\u{0}' as i32) as libc::c_int as number_t;
      }
      if n as libc::c_uint == STRNZ as libc::c_int as libc::c_uint {
        return (*(*(*test_ptr_to_statics).args.offset(0)).offset(0) as libc::c_int
          != '\u{0}' as i32) as libc::c_int as number_t;
      }
      if n as libc::c_uint == FILTT as libc::c_int as libc::c_uint {
        return isatty(getn(*(*test_ptr_to_statics).args) as libc::c_int) as number_t;
      }
      return filstat(*(*test_ptr_to_statics).args, n) as number_t;
    }
  } else if (*(*test_ptr_to_statics).last_operator).op_type as libc::c_int == BINOP as libc::c_int {
    /*check_operator(args[1]); - already done */
    /* args[2] is known to be NULL, isn't it bound to fail? */
    return binop() as number_t;
  }
  //			syntax(args0_op->op_text, "argument expected");
  return (*(*(*test_ptr_to_statics).args.offset(0)).offset(0) as libc::c_int != '\u{0}' as i32)
    as libc::c_int as number_t;
}

/*
 * Busybox main internal header file
 *
 * Based in part on code from sash, Copyright (c) 1999 by David I. Bell
 * Permission has been granted to redistribute this code under GPL.
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* TODO: and glibc? */
/* There are two incompatible basename's, let's not use them! */
/* See the dirname/basename man page for details */
/* dirname,basename */
/* Don't do this here:
 * #include <sys/sysinfo.h>
 * Some linux/ includes pull in conflicting definition
 * of struct sysinfo (only in some toolchanins), which breaks build.
 * Include sys/sysinfo.h only in those files which need it.
 */
/* Just in case libc doesn't define some of these... */
//This breaks on bionic:
//# if !defined(__socklen_t_defined) && !defined(_SOCKLEN_T_DECLARED)
// /* We #define socklen_t *after* includes, otherwise we get
// * typedef redefinition errors from system headers
// * (in case "is it defined already" detection above failed)
// */
//#  define socklen_t bb_socklen_t
//   typedef unsigned socklen_t;
//# endif
//if this is still needed, add a fix along the lines of
//  ifdef SPECIFIC_BROKEN_LIBC_CHECK / typedef socklen_t / endif
//in platform.h instead!
/*
 * Use '%m' to append error string on platforms that support it,
 * '%s' and strerror() on those that don't.
 */
/*nothing*/
/* Some libc's forget to declare these, do it ourself */
/* klogctl is in libc's klog.h, but we cheat and not #include that */
/* Busybox does not use threads, we can speed up stdio. */
/* Above functions are required by POSIX.1-2008, below ones are extensions */
/* musl <= 1.1.15 does not support fflush_unlocked(NULL) */
//# undef  fflush
//# define fflush(stream) fflush_unlocked(stream)
/* Make all declarations hidden (-fvisibility flag only affects definitions) */
/* (don't include system headers after this until corresponding pop!) */
/* Tested to work correctly with all int types (IIRC :]) */
/* Large file support */
/* Note that CONFIG_LFS=y forces bbox to be built with all common ops
 * (stat, lseek etc) mapped to "largefile" variants by libc.
 * Practically it means that open() automatically has O_LARGEFILE added
 * and all filesize/file_offset parameters and struct members are "large"
 * (in today's world - signed 64bit). For full support of large files,
 * we need a few helper #defines (below) and careful use of off_t
 * instead of int/ssize_t. No lseek64(), O_LARGEFILE etc necessary */
/* CONFIG_LFS is on */
/* "long" is long enough on this system */
/* usage: sz = BB_STRTOOFF(s, NULL, 10); if (errno || sz < 0) die(); */
/* usage: printf("size: %"OFF_FMT"d (%"OFF_FMT"x)\n", sz, sz); */
/* scary. better ideas? (but do *test* them first!) */
/* Users report bionic to use 32-bit off_t even if LARGEFILE support is requested.
 * We misdetected that. Don't let it build:
 */
/* Some useful definitions */
/* Macros for min/max.  */
/* buffer allocation schemes */
/* glibc uses __errno_location() to get a ptr to errno */
/* We can just memorize it once - no multithreading in busybox :) */
/* dmalloc will redefine these to it's own implementation. It is safe
 * to have the prototypes here unconditionally.  */
/* After v = xrealloc_vector(v, SHIFT, idx) it's ok to use
 * at least v[idx] and v[idx+1], for all idx values.
 * SHIFT specifies how many new elements are added (1:2, 2:4, ..., 8:256...)
 * when all elements are used up. New elements are zeroed out.
 * xrealloc_vector(v, SHIFT, idx) *MUST* be called with consecutive IDXs -
 * skipping an index is a bad bug - it may miss a realloc!
 */
//TODO: supply a pointer to char[11] buffer (avoid statics)?
/* cp.c, mv.c, install.c depend on these values. CAREFUL when changing them! */
/* -p */
/* !-d */
/* -R */
/* -f */
/* -i */
/* -l */
/* -s */
/* -L */
/* -H */
/* -a = -pdR (mapped in cp.c) */
/* -r = -dR  (mapped in cp.c) */
/* -P = -d   (mapped in cp.c) */
/* -v */
/* -u */
/* -T */
/* --remove-destination */
/* bit 17 skipped for "cp --parents" */
/* cp --reflink=auto */
/* cp --reflink[=always] */
/*
 * Hole. cp may have some bits set here,
 * they should not affect remove_file()/copy_file()
 */
/* NB: without FILEUTILS_RECUR in flags, it will basically "cat"
 * the source, not copy (unless "source" is a directory).
 * This makes "cp /dev/null file" and "install /dev/null file" (!!!)
 * work coreutils-compatibly. */
/*ACTION_REVERSE      = (1 << 4), - unused */
/* more than enough for "/dev/ttyXXX" */
/* bb_copyfd_XX print read/write errors and return -1 if they occur */
/* "short" copy can be detected by return value < size */
/* this helper yells "short read!" if param is not -1 */
/* xxxx_strip version can modify its parameter:
 * "/"        -> "/"
 * "abc"      -> "abc"
 * "abc/def"  -> "def"
 * "abc/def/" -> "def" !!
 */
/* "abc/def/" -> "" and it never modifies 'path' */
/* Simpler version: does not special case "/" string */
/* NB: can violate const-ness (similarly to strchr) */
/* !RETURNS_MALLOC: it's a realloc-like function */
/* bb_signals(BB_FATAL_SIGS, handler) catches all signals which
 * otherwise would kill us, except for those resulting from bugs:
 * SIGSEGV, SIGILL, SIGFPE.
 * Other fatal signals not included (TODO?):
 * SIGBUS   Bus error (bad memory access)
 * SIGPOLL  Pollable event. Synonym of SIGIO
 * SIGPROF  Profiling timer expired
 * SIGSYS   Bad argument to routine
 * SIGTRAP  Trace/breakpoint trap
 *
 * The only known arch with some of these sigs not fitting
 * into 32 bits is parisc (SIGXCPU=33, SIGXFSZ=34, SIGSTKFLT=36).
 * Dance around with long long to guard against that...
 */
// Write to pipe with no readers
// Quit from keyboard
// Abort signal from abort(3)
// Timer signal from alarm(2)
// Virtual alarm clock
// CPU time limit exceeded
// File size limit exceeded
// Yes kids, these are also fatal!
/* Unlike signal() and bb_signals, sets handler with sigaction()
 * and in a way that while signal handler is run, no other signals
 * will be blocked; syscalls will not be restarted: */
/* syscalls like read() will be interrupted with EINTR: */
/* syscalls like read() won't be interrupted (though select/poll will be): */
/* Will do sigaction(signum, act, NULL): */
/* SIG_BLOCK/SIG_UNBLOCK all signals: */
/* Return old set in the same set: */
/* Standard handler which just records signo */
/* not FAST_FUNC! */
/* In this form code with pipes is much more readable */
/* Useful for having small structure members/global variables */
/* | AF_DECnet */
/* | AF_IPX */
/* SO_REUSEADDR allows a server to rebind to an address that is already
 * "in use" by old connections to e.g. previous server instance which is
 * killed or crashed. Without it bind will fail until all such connections
 * time out. Linux does not allow multiple live binds on same ip:port
 * regardless of SO_REUSEADDR (unlike some other flavors of Unix).
 * Turn it on before you call bind(). */
/* On Linux this never fails. */
/* NB: returns port in host byte order */
/* Create stream socket, and allocate suitable lsa.
 * (lsa of correct size and lsa->sa.sa_family (AF_INET/AF_INET6))
 * af == AF_UNSPEC will result in trying to create IPv6 socket,
 * and if kernel doesn't support it, fall back to IPv4.
 * This is useful if you plan to bind to resulting local lsa.
 */
/* Create server socket bound to bindaddr:port. bindaddr can be NULL,
 * numeric IP ("N.N.N.N") or numeric IPv6 address,
 * and can have ":PORT" suffix (for IPv6 use "[X:X:...:X]:PORT").
 * Only if there is no suffix, port argument is used */
/* NB: these set SO_REUSEADDR before bind */
/* Create client TCP socket connected to peer:port. Peer cannot be NULL.
 * Peer can be numeric IP ("N.N.N.N"), numeric IPv6 address or hostname,
 * and can have ":PORT" suffix (for IPv6 use "[X:X:...:X]:PORT").
 * If there is no suffix, port argument is used */
/* Connect to peer identified by lsa */
/* Get local address of bound or accepted socket */
/* Get remote address of connected or accepted socket */
/* Return malloc'ed len_and_sockaddr with socket address of host:port
 * Currently will return IPv4 or IPv6 sockaddrs only
 * (depending on host), but in theory nothing prevents e.g.
 * UNIX socket address being returned, IPX sockaddr etc...
 * On error does bb_error_msg and returns NULL */
/* Version which dies on error */
/* Same, useful if you want to force family (e.g. IPv6) */
/* Assign sin[6]_port member if the socket is an AF_INET[6] one,
 * otherwise no-op. Useful for ftp.
 * NB: does NOT do htons() internally, just direct assignment. */
/* Retrieve sin[6]_port or return -1 for non-INET[6] lsa's */
/* Reverse DNS. Returns NULL on failure. */
/* This one doesn't append :PORTNUM */
/* This one also doesn't fall back to dotted IP (returns NULL) */
/* inet_[ap]ton on steroids */
// "old" (ipv4 only) API
// users: traceroute.c hostname.c - use _list_ of all IPs
// Also mount.c and inetd.c are using gethostbyname(),
// + inet_common.c has additional IPv4-only stuff
/* opaque */
// RFC 5246
// sequence number
//   Each connection state contains a sequence number, which is
//   maintained separately for read and write states.  The sequence
//   number MUST be set to zero whenever a connection state is made the
//   active state.  Sequence numbers are of type uint64 and may not
//   exceed 2^64-1.
/*uint64_t read_seq64_be;*/
/*u8 *server_write_MAC_key;*/
//used by AES_GCM
/* 0 if argv[0] is NULL: */
/* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
 * But potentially slow, don't use in one-billion-times loops */
/* Note: does not use stdio, writes to fd 2 directly */
// gcc-4.1.1 still isn't good enough at optimizing it
// (+200 bytes compared to macro)
//static ALWAYS_INLINE
//int LONE_DASH(const char *s) { return s[0] == '-' && !s[1]; }
//static ALWAYS_INLINE
//int NOT_LONE_DASH(const char *s) { return s[0] != '-' || s[1]; }
/* Returns a string with unprintable chars replaced by '?' or
 * SUBST_WCHAR. This function is unicode-aware. */
/* Prints unprintable char ch as ^C or M-c to file
 * (M-c is used only if ch is ORed with PRINTABLE_META),
 * else it is printed as-is (except for ch = 0x9b) */
/* Return a string that is the printable representation of character ch.
 * Buffer must hold at least four characters. */
// NB: will return short read on error, not -1,
// if some data was read before error occurred
// Reads one line a-la fgets (but doesn't save terminating '\n').
// Reads byte-by-byte. Useful when it is important to not read ahead.
// Bytes are appended to pfx (which must be malloced, or NULL).
/* Reads block up to *maxsz_p (default: INT_MAX - 4095) */
/* Returns NULL if file can't be opened (default max size: INT_MAX - 4095) */
/* Never returns NULL */
/* Else use variable one (a bit more expensive) */
/* Autodetects gzip/bzip2 formats. fd may be in the middle of the file! */
/* Autodetects .gz etc */
/* lzma has no signature, need a little helper. NB: exist only for ENABLE_FEATURE_SEAMLESS_LZMA=y */
// NB: will return short write on error, not -1,
// if some data was written before error occurred
/* Close fd, but check for failures (some types of write errors) */
/* Reads and prints to stdout till eof, then closes FILE. Exits on error: */
/* Reads a line from a text file, up to a newline or NUL byte, inclusive.
 * Returns malloc'ed char*. If end is NULL '\n' isn't considered
 * end of line. If end isn't NULL, length of the chunk is stored in it.
 * Returns NULL if EOF/error.
 */
/* Reads up to (and including) TERMINATING_STRING: */
/* Same, with limited max size, and returns the length (excluding NUL): */
/* Chops off TERMINATING_STRING from the end: */
/* Reads up to (and including) "\n" or NUL byte: */
/* Chops off '\n' from the end, unlike fgets: */
/* Same, but doesn't try to conserve space (may have some slack after the end) */
/* extern char *xmalloc_fgetline_fast(FILE *file) FAST_FUNC RETURNS_MALLOC; */
/* Prints warning to stderr and returns NULL on failure: */
/* "Opens" stdin if filename is special, else just opens file: */
/* not FAST_FUNC! */
/* Wrapper which restarts poll on EINTR or ENOMEM.
 * On other errors complains [perror("poll")] and returns.
 * Warning! May take (much) longer than timeout_ms to return!
 * If this is a problem, use bare poll and open-code EINTR/ENOMEM handling */
/* Convert each alpha char in str to lower-case */
/* Returns a pointer past the formatted number, does NOT null-terminate */
/* Intelligent formatters of bignums */
/* If block_size == 0, display size without fractional part,
 * else display (size * block_size) with one decimal digit.
 * If display_unit == 0, show value no bigger than 1024 with suffix (K,M,G...),
 * else divide by display_unit and do not use suffix. */
/* "1024.0G" */
//TODO: provide pointer to buf (avoid statics)?
/* Put a string of hex bytes ("1b2e66fe"...), return advanced pointer */
/* Reverse */
/* Generate a UUID */
/* Last element is marked by mult == 0 */
/* Specialized: */
/* Using xatoi() instead of naive atoi() is not always convenient -
 * in many places people want *non-negative* values, but store them
 * in signed int. Therefore we need this one:
 * dies if input is not in [0, INT_MAX] range. Also will reject '-0' etc.
 * It should really be named xatoi_nonnegative (since it allows 0),
 * but that would be too long.
 */
/* Useful for reading port numbers */
/* These parse entries in /etc/passwd and /etc/group.  This is desirable
 * for BusyBox since we want to avoid using the glibc NSS stuff, which
 * increases target size and is often not needed on embedded systems.  */
/* wrapper: allows string to contain numeric uid or gid */
/* always sets uid and gid; returns 0 on failure */
/* always sets uid and gid; exits on failure */
/* chown-like handling of "user[:[group]" */
/* versions which cache results (useful for ps, ls etc) */
/* internally usernames are saved in fixed-sized char[] buffers */
/*
 * Returns (-1) terminated malloced result of getgroups().
 * Reallocs group_array (useful for repeated calls).
 * ngroups is an initial size of array. It is rounded up to 32 for realloc.
 * ngroups is updated on return.
 * ngroups can be NULL: bb_getgroups(NULL, NULL) is valid usage.
 * Dies on errors (on Linux, only xrealloc can cause this, not internal getgroups call).
 */
/* BB_EXECxx always execs (it's not doing NOFORK/NOEXEC stuff),
 * but it may exec busybox and call applet instead of searching PATH.
 */
/* xvfork() can't be a _function_, return after vfork in child mangles stack
 * in the parent. It must be a macro. */
/* NOMMU friendy fork+exec: */
/* wait4pid: unlike waitpid, waits ONLY for one process.
 * Returns sig + 0x180 if child is killed by signal.
 * It's safe to pass negative 'pids' from failed [v]fork -
 * wait4pid will return -1 (and will not clobber [v]fork's errno).
 * IOW: rc = wait4pid(spawn(argv));
 *      if (rc < 0) bb_perror_msg("%s", argv[0]);
 *      if (rc > 0) bb_error_msg("exit code: %d", rc & 0xff);
 */
/* ***********************************************************************/
/* spawn_and_wait/run_nofork_applet/run_applet_no_and_exit need to work */
/* carefully together to reinit some global state while not disturbing  */
/* other. Be careful if you change them. Consult docs/nofork_noexec.txt */
/* ***********************************************************************/
/* Same as wait4pid(spawn(argv)), but with NOFORK/NOEXEC if configured: */
/* Does NOT check that applet is NOFORK, just blindly runs it */
/* Helpers for daemonization.
 *
 * bb_daemonize(flags) = daemonize, does not compile on NOMMU
 *
 * bb_daemonize_or_rexec(flags, argv) = daemonizes on MMU (and ignores argv),
 *      rexec's itself on NOMMU with argv passed as command line.
 * Thus bb_daemonize_or_rexec may cause your <applet>_main() to be re-executed
 * from the start. (It will detect it and not reexec again second time).
 * You have to audit carefully that you don't do something twice as a result
 * (opening files/sockets, parsing config files etc...)!
 *
 * Both of the above will redirect fd 0,1,2 to /dev/null and drop ctty
 * (will do setsid()).
 *
 * fork_or_rexec(argv) = bare-bones fork on MMU,
 *      "vfork + re-exec ourself" on NOMMU. No fd redirection, no setsid().
 *      On MMU ignores argv.
 *
 * Helper for network daemons in foreground mode:
 *
 * bb_sanitize_stdio() = make sure that fd 0,1,2 are opened by opening them
 * to /dev/null if they are not.
 */
/* internal use */
//DAEMON_DOUBLE_FORK     = 1 << 4, /* double fork to avoid controlling tty */
/* Clear dangerous stuff, set PATH. Return 1 if was run by different user. */
/* For top, ps. Some argv[i] are replaced by malloced "-opt" strings */
/* { "-", NULL } */
/* BSD-derived getopt() functions require that optind be set to 1 in
 * order to reset getopt() state.  This used to be generally accepted
 * way of resetting getopt().  However, glibc's getopt()
 * has additional getopt() state beyond optind (specifically, glibc
 * extensions such as '+' and '-' at the start of the string), and requires
 * that optind be set to zero to reset its state.  BSD-derived versions
 * of getopt() misbehaved if optind is set to 0 in order to reset getopt(),
 * and glibc's getopt() used to coredump if optind is set 1 in order
 * to reset getopt().
 * Then BSD introduced additional variable "optreset" which should be
 * set to 1 in order to reset getopt().  Sigh.  Standards, anyone?
 *
 * By ~2008, OpenBSD 3.4 was changed to survive glibc-like optind = 0
 * (to interpret it as if optreset was set).
 */
/*def __GLIBC__*/
/* BSD style */
/* Having next pointer as a first member allows easy creation
 * of "llist-compatible" structs, and using llist_FOO functions
 * on them.
 */
/* BTW, surprisingly, changing API to
 *   llist_t *llist_add_to(llist_t *old_head, void *data)
 * etc does not result in smaller code... */
/* start_stop_daemon and udhcpc are special - they want
 * to create pidfiles regardless of FEATURE_PIDFILE */
/* True only if we created pidfile which is *file*, not /dev/null etc */
/* We need to export XXX_main from libbusybox
 * only if we build "individual" binaries
 */
/* Embedded script support */
/* Applets which are useful from another applets */
/* If shell needs them, they exist even if not enabled as applets */
#[no_mangle]
pub unsafe extern "C" fn test_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64; /* assuming "[[" */
  let mut res: libc::c_int = 0;
  let mut arg0: *const libc::c_char = 0 as *const libc::c_char;
  arg0 = bb_basename(*argv.offset(0));
  if (1i32 != 0 || 1i32 != 0 || 1i32 != 0 || 1i32 != 0)
    && *arg0.offset(0) as libc::c_int == '[' as i32
  {
    argc -= 1;
    if *arg0.offset(1) == 0 {
      /* "[" ? */
      if *(*argv.offset(argc as isize)).offset(0) as libc::c_int != ']' as i32
        || *(*argv.offset(argc as isize)).offset(1) as libc::c_int != 0
      {
        bb_simple_error_msg(b"missing ]\x00" as *const u8 as *const libc::c_char);
        return 2i32;
      }
    } else if strcmp(
      *argv.offset(argc as isize),
      b"]]\x00" as *const u8 as *const libc::c_char,
    ) != 0i32
    {
      bb_simple_error_msg(b"missing ]]\x00" as *const u8 as *const libc::c_char);
      return 2i32;
    }
    let ref mut fresh0 = *argv.offset(argc as isize);
    *fresh0 = 0 as *mut libc::c_char
  }
  /* argc is unused after this point */
  /* We must do DEINIT_S() prior to returning */
  let ref mut fresh1 =
    *(not_const_pp(&test_ptr_to_statics as *const *mut test_statics as *const libc::c_void)
      as *mut *mut test_statics);
  *fresh1 = xzalloc(::std::mem::size_of::<test_statics>() as libc::c_ulong) as *mut test_statics;
  asm!("" : : : "memory" : "volatile");
  res = _setjmp((*test_ptr_to_statics).leaving.as_mut_ptr());
  if !(res != 0) {
    /* resetting ngroups is probably unnecessary.  it will
     * force a new call to getgroups(), which prevents using
     * group data fetched during a previous call.  but the
     * only way the group data could be stale is if there's
     * been an intervening call to setgroups(), and this
     * isn't likely in the case of a shell.  paranoia
     * prevails...
     */
    /*ngroups = 0; - done by INIT_S() */
    argv = argv.offset(1);
    (*test_ptr_to_statics).args = argv;
    /* Implement special cases from POSIX.2, section 4.62.4.
     * Testcase: "test '(' = '('"
     * The general parser would misinterpret '(' as group start.
     */
    let mut negate: libc::c_int = 0i32;
    loop {
      if (*argv.offset(0)).is_null() {
        /* "test" */
        res = 1i32;
        current_block = 9086154867617767870;
        break;
      } else if (*argv.offset(1)).is_null() {
        /* "test [!] arg" */
        res = (*(*argv.offset(0)).offset(0) as libc::c_int == '\u{0}' as i32) as libc::c_int;
        current_block = 9086154867617767870;
        break;
      } else {
        if !(*argv.offset(2)).is_null() {
          if (*argv.offset(3)).is_null() {
            /*
             * http://pubs.opengroup.org/onlinepubs/009695399/utilities/test.html
             * """ 3 arguments:
             * If $2 is a binary primary, perform the binary test of $1 and $3.
             * """
             */
            check_operator(*argv.offset(1));
            if (*(*test_ptr_to_statics).last_operator).op_type as libc::c_int
              == BINOP as libc::c_int
            {
              /* "test [!] arg1 <binary_op> arg2" */
              (*test_ptr_to_statics).args = argv;
              res = (binop() == 0i32) as libc::c_int;
              current_block = 9086154867617767870;
              break;
            }
          } else if (*argv.offset(4)).is_null() {
            /* argv[3] exists (at least 4 args), is it exactly 4 args? */
            /*
             * """ 4 arguments:
             * If $1 is '!', negate the three-argument test of $2, $3, and $4.
             * If $1 is '(' and $4 is ')', perform the two-argument test of $2 and $3.
             * """
             * Example why code below is necessary: test '(' ! -e ')'
             */
            if *(*argv.offset(0)).offset(0) as libc::c_int == '(' as i32
              && *(*argv.offset(0)).offset(1) == 0
              && (*(*argv.offset(3)).offset(0) as libc::c_int == ')' as i32
                && *(*argv.offset(3)).offset(1) == 0)
            {
              /* "test [!] ( x y )" */
              let ref mut fresh2 = *argv.offset(3);
              *fresh2 = 0 as *mut libc::c_char;
              argv = argv.offset(1)
            }
          }
        }
        /* """If $1 is '(' and $3 is ')', perform the unary test of $2."""
         * Looks like this works without additional coding.
         */
        if *(*argv.offset(0)).offset(0) as libc::c_int == '!' as i32
          && *(*argv.offset(0)).offset(1) == 0
        {
          argv = argv.offset(1);
          negate ^= 1i32
        } else {
          res = (oexpr(check_operator(*(*test_ptr_to_statics).args)) == 0) as libc::c_int;
          if !(*(*test_ptr_to_statics).args).is_null() && {
            (*test_ptr_to_statics).args = (*test_ptr_to_statics).args.offset(1);
            !(*(*test_ptr_to_statics).args).is_null()
          } {
            /* Examples:
             * test 3 -lt 5 6
             * test -t 1 2
             */
            bb_error_msg(
              b"%s: unknown operand\x00" as *const u8 as *const libc::c_char,
              *(*test_ptr_to_statics).args,
            );
            res = 2i32
          }
          current_block = 15881415538116575152;
          break;
        }
      }
    }
    match current_block {
      15881415538116575152 => {}
      _ => {
        /* If there was leading "!" op... */
        res ^= negate
      }
    }
  }
  free((*test_ptr_to_statics).group_array as *mut libc::c_void);
  free(test_ptr_to_statics as *mut libc::c_void);
  return res;
}
