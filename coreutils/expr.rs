use c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;
use libc;
extern "C" {
  #[no_mangle]
  fn strtoll(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_longlong;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xstrndup(s: *const libc::c_char, n: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;
  #[no_mangle]
  static mut xfunc_error_retval: uint8_t;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn index_in_strings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
  #[no_mangle]
  fn regexec(
    __preg: *const regex_t,
    __string: *const libc::c_char,
    __nmatch: size_t,
    __pmatch: *mut regmatch_t,
    __eflags: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn regfree(__preg: *mut regex_t);
  #[no_mangle]
  fn xregcomp(preg: *mut regex_t, regex: *const libc::c_char, cflags: libc::c_int);
}

pub type __int64_t = libc::c_long;
pub type int64_t = __int64_t;
use crate::librb::size_t;
use crate::librb::smallint;
 use libc::uint8_t;
//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub args: *mut *mut libc::c_char,
}
pub type reg_syntax_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_int;
pub const REG_ERPAREN: C2RustUnnamed = 16;
pub const REG_ESIZE: C2RustUnnamed = 15;
pub const REG_EEND: C2RustUnnamed = 14;
pub const REG_BADRPT: C2RustUnnamed = 13;
pub const REG_ESPACE: C2RustUnnamed = 12;
pub const REG_ERANGE: C2RustUnnamed = 11;
pub const REG_BADBR: C2RustUnnamed = 10;
pub const REG_EBRACE: C2RustUnnamed = 9;
pub const REG_EPAREN: C2RustUnnamed = 8;
pub const REG_EBRACK: C2RustUnnamed = 7;
pub const REG_ESUBREG: C2RustUnnamed = 6;
pub const REG_EESCAPE: C2RustUnnamed = 5;
pub const REG_ECTYPE: C2RustUnnamed = 4;
pub const REG_ECOLLATE: C2RustUnnamed = 3;
pub const REG_BADPAT: C2RustUnnamed = 2;
pub const REG_NOMATCH: C2RustUnnamed = 1;
pub const REG_NOERROR: C2RustUnnamed = 0;
pub const REG_ENOSYS: C2RustUnnamed = -1;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
  pub buffer: *mut libc::c_uchar,
  pub allocated: libc::c_ulong,
  pub used: libc::c_ulong,
  pub syntax: reg_syntax_t,
  pub fastmap: *mut libc::c_char,
  pub translate: *mut libc::c_uchar,
  pub re_nsub: size_t,
  #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
  #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
  #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
  #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
  #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
  #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
  #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
  pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
  #[bitfield(padding)]
  pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
  pub rm_so: regoff_t,
  pub rm_eo: regoff_t,
}
pub type arith_t = int64_t;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const STRING: C2RustUnnamed_0 = 1;
pub const INTEGER: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct valinfo {
  pub type_0: smallint,
  pub u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
  pub i: arith_t,
  pub s: *mut libc::c_char,
}
pub type VALUE = valinfo;
pub const NMATCH: C2RustUnnamed_2 = 2;
pub type C2RustUnnamed_2 = libc::c_uint;
/* Return a VALUE for I.  */
unsafe extern "C" fn int_value(mut i: arith_t) -> *mut VALUE {
  let mut v: *mut VALUE = 0 as *mut VALUE;
  v = xzalloc(::std::mem::size_of::<VALUE>() as libc::c_ulong) as *mut VALUE;
  if INTEGER as libc::c_int != 0 {
    /* otherwise xzalloc did it already */
    (*v).type_0 = INTEGER as libc::c_int as smallint
  }
  (*v).u.i = i;
  return v;
}
/* Return a VALUE for S.  */
unsafe extern "C" fn str_value(mut s: *const libc::c_char) -> *mut VALUE {
  let mut v: *mut VALUE = 0 as *mut VALUE;
  v = xzalloc(::std::mem::size_of::<VALUE>() as libc::c_ulong) as *mut VALUE;
  if STRING as libc::c_int != 0 {
    /* otherwise xzalloc did it already */
    (*v).type_0 = STRING as libc::c_int as smallint
  }
  (*v).u.s = xstrdup(s);
  return v;
}
/* Free VALUE V, including structure components.  */
unsafe extern "C" fn freev(mut v: *mut VALUE) {
  if (*v).type_0 as libc::c_int == STRING as libc::c_int {
    free((*v).u.s as *mut libc::c_void);
  }
  free(v as *mut libc::c_void);
}
/* Return nonzero if V is a null-string or zero-number.  */
unsafe extern "C" fn null(mut v: *mut VALUE) -> libc::c_int {
  if (*v).type_0 as libc::c_int == INTEGER as libc::c_int {
    return ((*v).u.i == 0i32 as libc::c_long) as libc::c_int;
  }
  /* STRING: */
  return (*(*v).u.s.offset(0) as libc::c_int == '\u{0}' as i32
    || *(*v).u.s.offset(0) as libc::c_int == '0' as i32 && *(*v).u.s.offset(1) == 0)
    as libc::c_int;
}
/* Coerce V to a STRING value (can't fail).  */
unsafe extern "C" fn tostring(mut v: *mut VALUE) {
  if (*v).type_0 as libc::c_int == INTEGER as libc::c_int {
    (*v).u.s = xasprintf(
      b"%lld\x00" as *const u8 as *const libc::c_char,
      (*v).u.i as libc::c_longlong,
    );
    (*v).type_0 = STRING as libc::c_int as smallint
  };
}
/* Coerce V to an INTEGER value.  Return 1 on success, 0 on failure.  */
unsafe extern "C" fn toarith(mut v: *mut VALUE) -> bool {
  if (*v).type_0 as libc::c_int == STRING as libc::c_int {
    let mut i: arith_t = 0;
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Don't interpret the empty string as an integer.  */
    /* Currently does not worry about overflow or int/long differences. */
    i = strtoll((*v).u.s, &mut e, 10i32) as arith_t;
    if (*v).u.s == e || *e as libc::c_int != 0 {
      return 0i32 != 0;
    }
    free((*v).u.s as *mut libc::c_void);
    (*v).u.i = i;
    (*v).type_0 = INTEGER as libc::c_int as smallint
  }
  return 1i32 != 0;
}
/* Return str[0]+str[1] if the next token matches STR exactly.
STR must not be NULL.  */
unsafe extern "C" fn nextarg(mut str: *const libc::c_char) -> libc::c_int {
  if (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args).is_null()
    || strcmp(
      *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args,
      str,
    ) != 0i32
  {
    return 0i32;
  }
  return *str.offset(0) as libc::c_uchar as libc::c_int
    + *str.offset(1) as libc::c_uchar as libc::c_int;
}
/* The comparison operator handling functions.  */
unsafe extern "C" fn cmp_common(
  mut l: *mut VALUE,
  mut r: *mut VALUE,
  mut op: libc::c_int,
) -> libc::c_int {
  let mut ll: arith_t = 0;
  let mut rr: arith_t = 0;
  ll = (*l).u.i;
  rr = (*r).u.i;
  if (*l).type_0 as libc::c_int == STRING as libc::c_int
    || (*r).type_0 as libc::c_int == STRING as libc::c_int
  {
    tostring(l);
    tostring(r);
    ll = strcmp((*l).u.s, (*r).u.s) as arith_t;
    rr = 0i32 as arith_t
  }
  /* calculating ll - rr and checking the result is prone to overflows.
   * We'll do it differently: */
  if op == '<' as i32 {
    return (ll < rr) as libc::c_int;
  }
  if op == '<' as i32 + '=' as i32 {
    return (ll <= rr) as libc::c_int;
  }
  if op == '=' as i32 || op == '=' as i32 + '=' as i32 {
    return (ll == rr) as libc::c_int;
  }
  if op == '!' as i32 + '=' as i32 {
    return (ll != rr) as libc::c_int;
  }
  if op == '>' as i32 {
    return (ll > rr) as libc::c_int;
  }
  /* >= */
  return (ll >= rr) as libc::c_int;
}
/* The arithmetic operator handling functions.  */
unsafe extern "C" fn arithmetic_common(
  mut l: *mut VALUE,
  mut r: *mut VALUE,
  mut op: libc::c_int,
) -> arith_t {
  let mut li: arith_t = 0;
  let mut ri: arith_t = 0;
  if !toarith(l) || !toarith(r) {
    bb_simple_error_msg_and_die(b"non-numeric argument\x00" as *const u8 as *const libc::c_char);
  }
  li = (*l).u.i;
  ri = (*r).u.i;
  if op == '+' as i32 {
    return li + ri;
  }
  if op == '-' as i32 {
    return li - ri;
  }
  if op == '*' as i32 {
    return li * ri;
  }
  if ri == 0i32 as libc::c_long {
    bb_simple_error_msg_and_die(b"division by zero\x00" as *const u8 as *const libc::c_char);
  }
  if op == '/' as i32 {
    return li / ri;
  }
  return li % ri;
}
/* Do the : operator.
SV is the VALUE for the lhs (the string),
PV is the VALUE for the rhs (the pattern).  */
unsafe extern "C" fn docolon(mut sv: *mut VALUE, mut pv: *mut VALUE) -> *mut VALUE {
  let mut v: *mut VALUE = 0 as *mut VALUE;
  let mut re_buffer: regex_t = regex_t {
    buffer: 0 as *mut libc::c_uchar,
    allocated: 0,
    used: 0,
    syntax: 0,
    fastmap: 0 as *mut libc::c_char,
    translate: 0 as *mut libc::c_uchar,
    re_nsub: 0,
    can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
    c2rust_padding: [0; 7],
  };
  let mut re_regs: [regmatch_t; 2] = [regmatch_t { rm_so: 0, rm_eo: 0 }; 2];
  tostring(sv);
  tostring(pv);
  if *(*pv).u.s.offset(0) as libc::c_int == '^' as i32 {
    bb_error_msg(b"warning: \'%s\': using \'^\' as the first character\nof a basic regular expression is not portable; it is ignored\x00"
                         as *const u8 as *const libc::c_char, (*pv).u.s);
  }
  memset(
    &mut re_buffer as *mut regex_t as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<regex_t>() as libc::c_ulong,
  );
  memset(
    re_regs.as_mut_ptr() as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<[regmatch_t; 2]>() as libc::c_ulong,
  );
  xregcomp(&mut re_buffer, (*pv).u.s, 0i32);
  /* expr uses an anchored pattern match, so check that there was a
   * match and that the match starts at offset 0. */
  if regexec(
    &mut re_buffer,
    (*sv).u.s,
    NMATCH as libc::c_int as size_t,
    re_regs.as_mut_ptr(),
    0i32,
  ) != REG_NOMATCH as libc::c_int
    && re_regs[0].rm_so == 0i32
  {
    /* Were \(...\) used? */
    if re_buffer.re_nsub > 0i32 as libc::c_ulong && re_regs[1].rm_so >= 0i32 {
      *(*sv).u.s.offset(re_regs[1].rm_eo as isize) = '\u{0}' as i32 as libc::c_char;
      v = str_value((*sv).u.s.offset(re_regs[1].rm_so as isize))
    } else {
      v = int_value(re_regs[0].rm_eo as arith_t)
    }
  } else if re_buffer.re_nsub > 0i32 as libc::c_ulong {
    v = str_value(b"\x00" as *const u8 as *const libc::c_char)
  } else {
    v = int_value(0i32 as arith_t)
  }
  regfree(&mut re_buffer);
  return v;
}
/* Match failed -- return the right kind of null.  */
/* Handle bare operands and ( expr ) syntax.  */
unsafe extern "C" fn eval7() -> *mut VALUE {
  let mut v: *mut VALUE = 0 as *mut VALUE;
  if (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args).is_null() {
    bb_simple_error_msg_and_die(b"syntax error\x00" as *const u8 as *const libc::c_char);
  }
  if nextarg(b"(\x00" as *const u8 as *const libc::c_char) != 0 {
    let ref mut fresh0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args;
    *fresh0 = (*fresh0).offset(1);
    v = eval();
    if nextarg(b")\x00" as *const u8 as *const libc::c_char) == 0 {
      bb_simple_error_msg_and_die(b"syntax error\x00" as *const u8 as *const libc::c_char);
    }
    let ref mut fresh1 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args;
    *fresh1 = (*fresh1).offset(1);
    return v;
  }
  if nextarg(b")\x00" as *const u8 as *const libc::c_char) != 0 {
    bb_simple_error_msg_and_die(b"syntax error\x00" as *const u8 as *const libc::c_char);
  }
  let ref mut fresh2 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args;
  let fresh3 = *fresh2;
  *fresh2 = (*fresh2).offset(1);
  return str_value(*fresh3);
}
/* Handle match, substr, index, length, and quote keywords.  */
unsafe extern "C" fn eval6() -> *mut VALUE {
  static mut keywords: [libc::c_char; 33] = [
    113, 117, 111, 116, 101, 0, 108, 101, 110, 103, 116, 104, 0, 109, 97, 116, 99, 104, 0, 105,
    110, 100, 101, 120, 0, 115, 117, 98, 115, 116, 114, 0, 0,
  ]; /* silence gcc */
  let mut r: *mut VALUE = 0 as *mut VALUE; /* silence gcc */
  let mut i1: *mut VALUE = 0 as *mut VALUE;
  let mut i2: *mut VALUE = 0 as *mut VALUE;
  let mut l: *mut VALUE = 0 as *mut VALUE;
  l = l;
  let mut v: *mut VALUE = 0 as *mut VALUE;
  v = v;
  let mut key: libc::c_int =
    if !(*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args).is_null() {
      (index_in_strings(
        keywords.as_ptr(),
        *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args,
      )) + 1i32
    } else {
      0i32
    };
  if key == 0i32 {
    /* not a keyword */
    return eval7();
  } /* We have a valid token, so get the next argument.  */
  let ref mut fresh4 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args;
  *fresh4 = (*fresh4).offset(1);
  if key == 1i32 {
    /* quote */
    if (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args).is_null() {
      bb_simple_error_msg_and_die(b"syntax error\x00" as *const u8 as *const libc::c_char);
    }
    let ref mut fresh5 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args;
    let fresh6 = *fresh5;
    *fresh5 = (*fresh5).offset(1);
    return str_value(*fresh6);
  }
  if key == 2i32 {
    /* length */
    r = eval6();
    tostring(r);
    v = int_value(strlen((*r).u.s) as arith_t);
    freev(r);
  } else {
    l = eval6()
  }
  if key == 3i32 {
    /* match */
    r = eval6();
    v = docolon(l, r);
    freev(l);
    freev(r);
  }
  if key == 4i32 {
    /* index */
    r = eval6();
    tostring(l);
    tostring(r);
    v = int_value(strcspn((*l).u.s, (*r).u.s).wrapping_add(1i32 as libc::c_ulong) as arith_t);
    if (*v).u.i == strlen((*l).u.s) as arith_t + 1i32 as libc::c_long {
      (*v).u.i = 0i32 as arith_t
    }
    freev(l);
    freev(r);
  }
  if key == 5i32 {
    /* substr */
    i1 = eval6();
    i2 = eval6();
    tostring(l);
    if !toarith(i1)
      || !toarith(i2)
      || (*i1).u.i > strlen((*l).u.s) as arith_t
      || (*i1).u.i <= 0i32 as libc::c_long
      || (*i2).u.i <= 0i32 as libc::c_long
    {
      v = str_value(b"\x00" as *const u8 as *const libc::c_char)
    } else {
      v = xmalloc(::std::mem::size_of::<VALUE>() as libc::c_ulong) as *mut VALUE;
      (*v).type_0 = STRING as libc::c_int as smallint;
      (*v).u.s = xstrndup(
        (*l).u.s.offset((*i1).u.i as isize).offset(-1),
        (*i2).u.i as libc::c_int,
      )
    }
    freev(l);
    freev(i1);
    freev(i2);
  }
  return v;
}
/* Handle : operator (pattern matching).
Calls docolon to do the real work.  */
unsafe extern "C" fn eval5() -> *mut VALUE {
  let mut l: *mut VALUE = 0 as *mut VALUE;
  let mut r: *mut VALUE = 0 as *mut VALUE;
  let mut v: *mut VALUE = 0 as *mut VALUE;
  l = eval6();
  while nextarg(b":\x00" as *const u8 as *const libc::c_char) != 0 {
    let ref mut fresh7 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args;
    *fresh7 = (*fresh7).offset(1);
    r = eval6();
    v = docolon(l, r);
    freev(l);
    freev(r);
    l = v
  }
  return l;
}
/* Handle *, /, % operators.  */
unsafe extern "C" fn eval4() -> *mut VALUE {
  let mut l: *mut VALUE = 0 as *mut VALUE;
  let mut r: *mut VALUE = 0 as *mut VALUE;
  let mut op: libc::c_int = 0;
  let mut val: arith_t = 0;
  l = eval5();
  loop {
    op = nextarg(b"*\x00" as *const u8 as *const libc::c_char);
    if op == 0 {
      op = nextarg(b"/\x00" as *const u8 as *const libc::c_char);
      if op == 0 {
        op = nextarg(b"%\x00" as *const u8 as *const libc::c_char);
        if op == 0 {
          return l;
        }
      }
    }
    let ref mut fresh8 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args;
    *fresh8 = (*fresh8).offset(1);
    r = eval5();
    val = arithmetic_common(l, r, op);
    freev(l);
    freev(r);
    l = int_value(val)
  }
}
/* Handle +, - operators.  */
unsafe extern "C" fn eval3() -> *mut VALUE {
  let mut l: *mut VALUE = 0 as *mut VALUE;
  let mut r: *mut VALUE = 0 as *mut VALUE;
  let mut op: libc::c_int = 0;
  let mut val: arith_t = 0;
  l = eval4();
  loop {
    op = nextarg(b"+\x00" as *const u8 as *const libc::c_char);
    if op == 0 {
      op = nextarg(b"-\x00" as *const u8 as *const libc::c_char);
      if op == 0 {
        return l;
      }
    }
    let ref mut fresh9 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args;
    *fresh9 = (*fresh9).offset(1);
    r = eval4();
    val = arithmetic_common(l, r, op);
    freev(l);
    freev(r);
    l = int_value(val)
  }
}
/* Handle comparisons.  */
unsafe extern "C" fn eval2() -> *mut VALUE {
  let mut l: *mut VALUE = 0 as *mut VALUE;
  let mut r: *mut VALUE = 0 as *mut VALUE;
  let mut op: libc::c_int = 0;
  let mut val: arith_t = 0;
  l = eval3();
  loop {
    op = nextarg(b"<\x00" as *const u8 as *const libc::c_char);
    if op == 0 {
      op = nextarg(b"<=\x00" as *const u8 as *const libc::c_char);
      if op == 0 {
        op = nextarg(b"=\x00" as *const u8 as *const libc::c_char);
        if op == 0 {
          op = nextarg(b"==\x00" as *const u8 as *const libc::c_char);
          if op == 0 {
            op = nextarg(b"!=\x00" as *const u8 as *const libc::c_char);
            if op == 0 {
              op = nextarg(b">=\x00" as *const u8 as *const libc::c_char);
              if op == 0 {
                op = nextarg(b">\x00" as *const u8 as *const libc::c_char);
                if op == 0 {
                  return l;
                }
              }
            }
          }
        }
      }
    }
    let ref mut fresh10 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args;
    *fresh10 = (*fresh10).offset(1);
    r = eval3();
    toarith(l);
    toarith(r);
    val = cmp_common(l, r, op) as arith_t;
    freev(l);
    freev(r);
    l = int_value(val)
  }
}
/* Handle &.  */
unsafe extern "C" fn eval1() -> *mut VALUE {
  let mut l: *mut VALUE = 0 as *mut VALUE;
  let mut r: *mut VALUE = 0 as *mut VALUE;
  l = eval2();
  while nextarg(b"&\x00" as *const u8 as *const libc::c_char) != 0 {
    let ref mut fresh11 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args;
    *fresh11 = (*fresh11).offset(1);
    r = eval2();
    if null(l) != 0 || null(r) != 0 {
      freev(l);
      freev(r);
      l = int_value(0i32 as arith_t)
    } else {
      freev(r);
    }
  }
  return l;
}
/* NB: noexec applet - globals not zeroed */
/* forward declarations */
/* Handle |.  */
unsafe extern "C" fn eval() -> *mut VALUE {
  let mut l: *mut VALUE = 0 as *mut VALUE; /* coreutils compat */
  let mut r: *mut VALUE = 0 as *mut VALUE;
  l = eval1();
  while nextarg(b"|\x00" as *const u8 as *const libc::c_char) != 0 {
    let ref mut fresh12 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args;
    *fresh12 = (*fresh12).offset(1);
    r = eval1();
    if null(l) != 0 {
      freev(l);
      l = r
    } else {
      freev(r);
    }
  }
  return l;
}
#[no_mangle]
pub unsafe extern "C" fn expr_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut v: *mut VALUE = 0 as *mut VALUE;
  xfunc_error_retval = 2i32 as uint8_t;
  let ref mut fresh13 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args;
  *fresh13 = argv.offset(1);
  if (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args).is_null() {
    bb_simple_error_msg_and_die(b"too few arguments\x00" as *const u8 as *const libc::c_char);
  }
  v = eval();
  if !(*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).args).is_null() {
    bb_simple_error_msg_and_die(b"syntax error\x00" as *const u8 as *const libc::c_char);
  }
  if (*v).type_0 as libc::c_int == INTEGER as libc::c_int {
    printf(
      b"%lld\n\x00" as *const u8 as *const libc::c_char,
      (*v).u.i as libc::c_longlong,
    );
  } else {
    puts((*v).u.s);
  }
  fflush_stdout_and_exit(null(v));
}
