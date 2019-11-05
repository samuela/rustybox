use libc;

extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
  #[no_mangle]
  fn unlink(__name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  static mut stdin: *mut _IO_FILE;
  #[no_mangle]
  fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t, __stream: *mut FILE) -> size_t;
  #[no_mangle]
  fn fseeko(__stream: *mut FILE, __off: __off64_t, __whence: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn lseek(__fd: libc::c_int, __offset: __off64_t, __whence: libc::c_int) -> __off64_t;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn isqrt(N: libc::c_ulonglong) -> libc::c_ulong;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc_vector_helper(
    vector: *mut libc::c_void,
    sizeof_and_shift: libc::c_uint,
    idx: libc::c_int,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn recursive_action(
    fileName: *const libc::c_char,
    flags: libc::c_uint,
    fileAction: Option<
      unsafe extern "C" fn(
        _: *const libc::c_char,
        _: *mut stat,
        _: *mut libc::c_void,
        _: libc::c_int,
      ) -> libc::c_int,
    >,
    dirAction: Option<
      unsafe extern "C" fn(
        _: *const libc::c_char,
        _: *mut stat,
        _: *mut libc::c_void,
        _: libc::c_int,
      ) -> libc::c_int,
    >,
    userData: *mut libc::c_void,
    depth: libc::c_uint,
  ) -> libc::c_int;
  #[no_mangle]
  fn bb_copyfd_eof(fd1: libc::c_int, fd2: libc::c_int) -> off_t;
  #[no_mangle]
  fn xstat(pathname: *const libc::c_char, buf: *mut stat);
  #[no_mangle]
  fn open_or_warn(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xlseek(fd: libc::c_int, offset: off_t, whence: libc::c_int) -> off_t;
  #[no_mangle]
  fn xmkstemp(template: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn fclose_if_not_stdin(file: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn qsort_string_vector(sv: *mut *mut libc::c_char, count: libc::c_uint);
  #[no_mangle]
  static mut option_mask32: uint32_t;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> uint32_t;
  #[no_mangle]
  fn llist_pop(elm: *mut *mut llist_t) -> *mut libc::c_void;
  #[no_mangle]
  static mut xfunc_error_retval: uint8_t;
  #[no_mangle]
  fn xfunc_die() -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn concat_path_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  static ptr_to_globals: *mut globals;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type smallint = libc::c_schar;
pub type size_t = libc::c_ulong;
pub type off_t = __off64_t;
use crate::librb::stat;
use crate::librb::timespec;
use crate::librb::_IO_FILE;
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
  pub _next: *mut _IO_marker,
  pub _sbuf: *mut _IO_FILE,
  pub _pos: libc::c_int,
}
use crate::librb::FILE;
pub type __compar_fn_t =
  Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int>;
pub type C2RustUnnamed = libc::c_uint;
pub const ACTION_DANGLING_OK: C2RustUnnamed = 64;
pub const ACTION_QUIET: C2RustUnnamed = 32;
pub const ACTION_DEPTHFIRST: C2RustUnnamed = 8;
pub const ACTION_FOLLOWLINKS_L0: C2RustUnnamed = 4;
pub const ACTION_FOLLOWLINKS: C2RustUnnamed = 2;
pub const ACTION_RECURSE: C2RustUnnamed = 1;
use crate::libbb::llist::llist_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub exit_status: smallint,
  pub opt_U_context: libc::c_int,
  pub other_dir: *const libc::c_char,
  pub label: [*mut libc::c_char; 2],
  pub stb: [stat; 2],
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const COMMON_BUFSIZE: C2RustUnnamed_0 = 1024;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const STATUS_BINARY: C2RustUnnamed_1 = 2;
pub const STATUS_DIFFER: C2RustUnnamed_1 = 1;
pub const STATUS_SAME: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const FLAG_E: C2RustUnnamed_2 = 17;
pub const FLAG_B: C2RustUnnamed_2 = 16;
pub const FLAG_p: C2RustUnnamed_2 = 15;
pub const FLAG_u: C2RustUnnamed_2 = 14;
pub const FLAG_w: C2RustUnnamed_2 = 13;
pub const FLAG_U: C2RustUnnamed_2 = 12;
pub const FLAG_T: C2RustUnnamed_2 = 11;
pub const FLAG_t: C2RustUnnamed_2 = 10;
pub const FLAG_S: C2RustUnnamed_2 = 9;
pub const FLAG_s: C2RustUnnamed_2 = 8;
pub const FLAG_r: C2RustUnnamed_2 = 7;
pub const FLAG_q: C2RustUnnamed_2 = 6;
pub const FLAG_N: C2RustUnnamed_2 = 5;
pub const FLAG_L: C2RustUnnamed_2 = 4;
pub const FLAG_i: C2RustUnnamed_2 = 3;
pub const FLAG_d: C2RustUnnamed_2 = 2;
pub const FLAG_b: C2RustUnnamed_2 = 1;
pub const FLAG_a: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FILE_and_pos_t {
  pub ft_fp: *mut FILE,
  pub ft_pos: off_t,
}
pub type token_t = libc::c_int;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const CHAR_MASK: C2RustUnnamed_3 = 511;
/* 8th bit is used to distinguish EOF from 0xff */
/* used -b code, means we are skipping spaces */
pub const SHIFT_EOF: C2RustUnnamed_3 = 23;
/* we saw EOL (sticky) */
pub const TOK_SPACE: C2RustUnnamed_3 = 4096;
/* File ended */
/* Private (Only to be used by read_token() */
pub const TOK_EOL: C2RustUnnamed_3 = 2048;
/* Line fully processed, you can proceed to the next */
pub const TOK_EOF: C2RustUnnamed_3 = 1024;
/* Public */
pub const TOK_EMPTY: C2RustUnnamed_3 = 512;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cand {
  pub x: libc::c_int,
  pub y: libc::c_int,
  pub pred: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line {
  pub c2rust_unnamed: C2RustUnnamed_4,
  pub value: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
  pub serial: libc::c_uint,
  pub offset: off_t,
}
pub type vec_t = [C2RustUnnamed_5; 2];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
  pub a: libc::c_int,
  pub b: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dlist {
  pub len: size_t,
  pub s: libc::c_int,
  pub e: libc::c_int,
  pub dl: *mut *mut libc::c_char,
}
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
/* Restores full EOF from one 8th bit: */
//#define TOK2CHAR(t) (((t) << SHIFT_EOF) >> SHIFT_EOF)
/* We don't really need the above, we only need to have EOF != any_real_char: */
unsafe extern "C" fn seek_ft(mut ft: *mut FILE_and_pos_t, mut pos: off_t) {
  if (*ft).ft_pos != pos {
    (*ft).ft_pos = pos;
    fseeko((*ft).ft_fp, pos, 0i32);
  };
}
/* Reads tokens from given fp, handling -b and -w flags
 * The user must reset tok every line start
 */
unsafe extern "C" fn read_token(mut ft: *mut FILE_and_pos_t, mut tok: token_t) -> libc::c_int {
  tok |= TOK_EMPTY as libc::c_int;
  /* this one too, ignore it */
  while tok & TOK_EOL as libc::c_int == 0 {
    let mut is_space: bool = false;
    let mut t: libc::c_int = 0;
    t = getc_unlocked((*ft).ft_fp);
    if t != -1i32 {
      (*ft).ft_pos += 1
    }
    is_space = t == -1i32
      || ({
        let mut bb__isspace: libc::c_uchar = (t - 9i32) as libc::c_uchar;
        (bb__isspace as libc::c_int == ' ' as i32 - 9i32
          || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
      }) != 0;
    /* If t == EOF (-1), set both TOK_EOF and TOK_EOL */
    tok |= t & TOK_EOF as libc::c_int + TOK_EOL as libc::c_int;
    /* Only EOL? */
    if t == '\n' as i32 {
      tok |= TOK_EOL as libc::c_int
    }
    if option_mask32 & (1i32 << FLAG_i as libc::c_int) as libc::c_uint != 0 {
      /* Handcoded tolower() */
      t = if t >= 'A' as i32 && t <= 'Z' as i32 {
        (t) - ('A' as i32 - 'a' as i32)
      } else {
        t
      }
    }
    if option_mask32 & (1i32 << FLAG_w as libc::c_int) as libc::c_uint != 0
      && is_space as libc::c_int != 0
    {
      continue;
    }
    /* Trim char value to low 9 bits */
    t &= CHAR_MASK as libc::c_int;
    if option_mask32 & (1i32 << FLAG_b as libc::c_int) as libc::c_uint != 0 {
      /* Was prev char whitespace? */
      if tok & TOK_SPACE as libc::c_int != 0 {
        /* yes */
        if is_space {
          continue;
        }
        tok &= !(TOK_SPACE as libc::c_int)
      } else if is_space {
        /* 1st whitespace char.
         * Set TOK_SPACE and replace char by ' ' */
        t = TOK_SPACE as libc::c_int + ' ' as i32
      }
    }
    /* Clear EMPTY */
    tok &= !(TOK_EMPTY as libc::c_int + CHAR_MASK as libc::c_int);
    /* Assign char value (low 9 bits) and maybe set TOK_SPACE */
    tok |= t;
    break;
  }
  return tok;
}
unsafe extern "C" fn search(
  mut c: *const libc::c_int,
  mut k: libc::c_int,
  mut y: libc::c_int,
  mut list: *const cand,
) -> libc::c_int {
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  if (*list.offset(*c.offset(k as isize) as isize)).y < y {
    /* quick look for typical case */
    return k + 1i32;
  }
  i = 0i32;
  j = k + 1i32;
  loop {
    let l: libc::c_int = i + j >> 1i32;
    if l > i {
      let t: libc::c_int = (*list.offset(*c.offset(l as isize) as isize)).y;
      if t > y {
        j = l
      } else if t < y {
        i = l
      } else {
        return l;
      }
    } else {
      return l + 1i32;
    }
  }
}
unsafe extern "C" fn stone(
  mut a: *const libc::c_int,
  mut n: libc::c_int,
  mut b: *const libc::c_int,
  mut J: *mut libc::c_int,
  mut pref: libc::c_int,
) {
  let isq: libc::c_uint = isqrt(n as libc::c_ulonglong) as libc::c_uint;
  let bound: libc::c_uint = if option_mask32 & (1i32 << FLAG_d as libc::c_int) as libc::c_uint != 0
  {
    (2147483647i32 as libc::c_uint)
      .wrapping_mul(2u32)
      .wrapping_add(1u32)
  } else if 256i32 as libc::c_uint > isq {
    256i32 as libc::c_uint
  } else {
    isq
  };
  let mut clen: libc::c_int = 1i32;
  let mut clistlen: libc::c_int = 100i32;
  let mut k: libc::c_int = 0i32;
  let mut clist: *mut cand = xzalloc(
    (clistlen as libc::c_ulong).wrapping_mul(::std::mem::size_of::<cand>() as libc::c_ulong),
  ) as *mut cand;
  let mut cand: cand = cand {
    x: 0,
    y: 0,
    pred: 0,
  };
  let mut q: *mut cand = 0 as *mut cand;
  let mut klist: *mut libc::c_int = xzalloc(
    ((n + 2i32) as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
  ) as *mut libc::c_int;
  /*clist[0] = (struct cand){0}; - xzalloc did it */
  /*klist[0] = 0; */
  cand.x = 1i32;
  while cand.x <= n {
    let mut j: libc::c_int = *a.offset(cand.x as isize);
    let mut oldl: libc::c_int = 0i32;
    let mut numtries: libc::c_uint = 0i32 as libc::c_uint;
    if !(j == 0i32) {
      cand.y = -*b.offset(j as isize);
      cand.pred = *klist.offset(0);
      loop {
        let mut l: libc::c_int = 0;
        let mut tc: libc::c_int = 0;
        if !(cand.y <= (*clist.offset(cand.pred as isize)).y) {
          l = search(klist, k, cand.y, clist);
          if l != oldl + 1i32 {
            cand.pred = *klist.offset((l - 1i32) as isize)
          }
          if !(l <= k && (*clist.offset(*klist.offset(l as isize) as isize)).y <= cand.y) {
            if clen == clistlen {
              clistlen = clistlen * 11i32 / 10i32;
              clist = xrealloc(
                clist as *mut libc::c_void,
                (clistlen as libc::c_ulong)
                  .wrapping_mul(::std::mem::size_of::<cand>() as libc::c_ulong),
              ) as *mut cand
            }
            *clist.offset(clen as isize) = cand;
            tc = *klist.offset(l as isize);
            let fresh0 = clen;
            clen = clen + 1;
            *klist.offset(l as isize) = fresh0;
            if l <= k {
              cand.pred = tc;
              oldl = l;
              numtries = numtries.wrapping_add(1)
            } else {
              k += 1;
              break;
            }
          }
        }
        j += 1;
        cand.y = *b.offset(j as isize);
        if !(cand.y > 0i32 && numtries < bound) {
          break;
        }
      }
    }
    cand.x += 1
  }
  /* Unravel */
  q = clist.offset(*klist.offset(k as isize) as isize);
  while (*q).y != 0 {
    *J.offset(((*q).x + pref) as isize) = (*q).y + pref;
    q = clist.offset((*q).pred as isize)
  }
  free(klist as *mut libc::c_void);
  free(clist as *mut libc::c_void);
}
unsafe extern "C" fn equiv(
  mut a: *mut line,
  mut n: libc::c_int,
  mut b: *mut line,
  mut m: libc::c_int,
  mut c: *mut libc::c_int,
) {
  let mut i: libc::c_int = 1i32;
  let mut j: libc::c_int = 1i32;
  while i <= n && j <= m {
    if (*a.offset(i as isize)).value < (*b.offset(j as isize)).value {
      let fresh1 = i;
      i = i + 1;
      (*a.offset(fresh1 as isize)).value = 0i32 as libc::c_uint
    } else if (*a.offset(i as isize)).value == (*b.offset(j as isize)).value {
      let fresh2 = i;
      i = i + 1;
      (*a.offset(fresh2 as isize)).value = j as libc::c_uint
    } else {
      j += 1
    }
  }
  while i <= n {
    let fresh3 = i;
    i = i + 1;
    (*a.offset(fresh3 as isize)).value = 0i32 as libc::c_uint
  }
  (*b.offset((m + 1i32) as isize)).value = 0i32 as libc::c_uint;
  j = 0i32;
  loop {
    j += 1;
    if !(j <= m) {
      break;
    }
    *c.offset(j as isize) =
      (*b.offset(j as isize)).c2rust_unnamed.serial.wrapping_neg() as libc::c_int;
    while (*b.offset((j + 1i32) as isize)).value == (*b.offset(j as isize)).value {
      j += 1;
      *c.offset(j as isize) = (*b.offset(j as isize)).c2rust_unnamed.serial as libc::c_int
    }
  }
  *c.offset(j as isize) = -1i32;
}
unsafe extern "C" fn unsort(mut f: *const line, mut l: libc::c_int, mut b: *mut libc::c_int) {
  let mut i: libc::c_int = 0;
  let mut a: *mut libc::c_int = xmalloc(
    ((l + 1i32) as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
  ) as *mut libc::c_int;
  i = 1i32;
  while i <= l {
    *a.offset((*f.offset(i as isize)).c2rust_unnamed.serial as isize) =
      (*f.offset(i as isize)).value as libc::c_int;
    i += 1
  }
  i = 1i32;
  while i <= l {
    *b.offset(i as isize) = *a.offset(i as isize);
    i += 1
  }
  free(a as *mut libc::c_void);
}
unsafe extern "C" fn line_compar(
  mut a: *const libc::c_void,
  mut b: *const libc::c_void,
) -> libc::c_int {
  let mut r: libc::c_int = (*(a as *const line))
    .value
    .wrapping_sub((*(b as *const line)).value) as libc::c_int;
  if r != 0 {
    return r;
  }
  return (*(a as *const line))
    .c2rust_unnamed
    .serial
    .wrapping_sub((*(b as *const line)).c2rust_unnamed.serial) as libc::c_int;
}
unsafe extern "C" fn fetch(
  mut ft: *mut FILE_and_pos_t,
  mut ix: *const off_t,
  mut a: libc::c_int,
  mut b: libc::c_int,
  mut ch: libc::c_int,
) {
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  let mut col: libc::c_int = 0;
  i = a;
  while i <= b {
    seek_ft(ft, *ix.offset((i - 1i32) as isize));
    putchar_unlocked(ch);
    if option_mask32 & (1i32 << FLAG_T as libc::c_int) as libc::c_uint != 0 {
      putchar_unlocked('\t' as i32);
    }
    j = 0i32;
    col = 0i32;
    while (j as libc::c_long) < *ix.offset(i as isize) - *ix.offset((i - 1i32) as isize) {
      let mut c: libc::c_int = getc_unlocked((*ft).ft_fp);
      if c == -1i32 {
        puts(b"\n\\ No newline at end of file\x00" as *const u8 as *const libc::c_char);
        return;
      }
      (*ft).ft_pos += 1;
      if c == '\t' as i32 && option_mask32 & (1i32 << FLAG_t as libc::c_int) as libc::c_uint != 0 {
        loop {
          putchar_unlocked(' ' as i32);
          col += 1;
          if !(col & 7i32 != 0) {
            break;
          }
        }
      } else {
        putchar_unlocked(c);
        col += 1
      }
      j += 1
    }
    i += 1
  }
}
/* Creates the match vector J, where J[i] is the index
 * of the line in the new file corresponding to the line i
 * in the old file. Lines start at 1 instead of 0, that value
 * being used instead to denote no corresponding line.
 * This vector is dynamically allocated and must be freed by the caller.
 *
 * * fp is an input parameter, where fp[0] and fp[1] are the open
 *   old file and new file respectively.
 * * nlen is an output variable, where nlen[0] and nlen[1]
 *   gets the number of lines in the old and new file respectively.
 * * ix is an output variable, where ix[0] and ix[1] gets
 *   assigned dynamically allocated vectors of the offsets of the lines
 *   of the old and new file respectively. These must be freed by the caller.
 */
#[inline(never)]
unsafe extern "C" fn create_J(
  mut ft: *mut FILE_and_pos_t,
  mut nlen: *mut libc::c_int,
  mut ix: *mut *mut off_t,
) -> *mut libc::c_int {
  let mut J: *mut libc::c_int = 0 as *mut libc::c_int;
  let mut slen: [libc::c_int; 2] = [0; 2];
  let mut class: *mut libc::c_int = 0 as *mut libc::c_int;
  let mut member: *mut libc::c_int = 0 as *mut libc::c_int;
  let mut nfile: [*mut line; 2] = [0 as *mut line; 2];
  let mut sfile: [*mut line; 2] = [0 as *mut line; 2];
  let mut pref: libc::c_int = 0i32;
  let mut suff: libc::c_int = 0i32;
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  let mut delta: libc::c_int = 0;
  /* Lines of both files are hashed, and in the process
   * their offsets are stored in the array ix[fileno]
   * where fileno == 0 points to the old file, and
   * fileno == 1 points to the new one.
   */
  i = 0i32;
  while i < 2i32 {
    let mut hash: libc::c_uint = 0;
    let mut tok: token_t = 0;
    let mut sz: size_t = 100i32 as size_t;
    nfile[i as usize] = xmalloc(
      sz.wrapping_add(3i32 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<line>() as libc::c_ulong),
    ) as *mut line;
    /* ft gets here without the correct position, cant use seek_ft */
    (*ft.offset(i as isize)).ft_pos = 0i32 as off_t;
    fseeko((*ft.offset(i as isize)).ft_fp, 0i32 as __off64_t, 0i32);
    *nlen.offset(i as isize) = 0i32;
    /* We could zalloc nfile, but then zalloc starts showing in gprof at ~1% */
    (*nfile[i as usize].offset(0)).c2rust_unnamed.offset = 0i32 as off_t; /* saves code */
    loop {
      tok = 0i32;
      hash = tok as libc::c_uint;
      loop {
        tok = read_token(&mut *ft.offset(i as isize), tok);
        if !(tok & TOK_EMPTY as libc::c_int == 0) {
          break;
        }
        /* Hash algorithm taken from Robert Sedgewick, Algorithms in C, 3d ed., p 578. */
        /*hash = hash * 128 - hash + TOK2CHAR(tok);
         * gcc insists on optimizing above to "hash * 127 + ...", thus... */
        let mut o: libc::c_uint =
          hash.wrapping_sub((tok & CHAR_MASK as libc::c_int) as libc::c_uint); /* we want SPEED here */
        hash = hash.wrapping_mul(128i32 as libc::c_uint).wrapping_sub(o)
      }
      let ref mut fresh4 = *nlen.offset(i as isize);
      let fresh5 = *fresh4;
      *fresh4 = *fresh4 + 1;
      if fresh5 as libc::c_ulong == sz {
        sz = sz
          .wrapping_mul(3i32 as libc::c_ulong)
          .wrapping_div(2i32 as libc::c_ulong);
        nfile[i as usize] = xrealloc(
          nfile[i as usize] as *mut libc::c_void,
          sz.wrapping_add(3i32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<line>() as libc::c_ulong),
        ) as *mut line
      }
      /* line_compar needs hashes fit into positive int */
      (*nfile[i as usize].offset(*nlen.offset(i as isize) as isize)).value =
        hash & 2147483647i32 as libc::c_uint;
      /* like ftello(ft[i].ft_fp) but faster (avoids lseek syscall) */
      (*nfile[i as usize].offset(*nlen.offset(i as isize) as isize))
        .c2rust_unnamed
        .offset = (*ft.offset(i as isize)).ft_pos;
      if !(tok & TOK_EOF as libc::c_int != 0) {
        continue;
      }
      /* EOF counts as a token, so we have to adjust it here */
      let ref mut fresh6 = (*nfile[i as usize].offset(*nlen.offset(i as isize) as isize))
        .c2rust_unnamed
        .offset;
      *fresh6 += 1;
      break;
    }
    /* Exclude lone EOF line from the end of the file, to make fetch()'s job easier */
    if (*nfile[i as usize].offset(*nlen.offset(i as isize) as isize))
      .c2rust_unnamed
      .offset
      - (*nfile[i as usize].offset((*nlen.offset(i as isize) - 1i32) as isize))
        .c2rust_unnamed
        .offset
      == 1i32 as libc::c_long
    {
      let ref mut fresh7 = *nlen.offset(i as isize);
      *fresh7 -= 1
    }
    /* Now we copy the line offsets into ix */
    let ref mut fresh8 = *ix.offset(i as isize);
    *fresh8 = xmalloc(
      ((*nlen.offset(i as isize) + 2i32) as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<off_t>() as libc::c_ulong),
    ) as *mut off_t;
    j = 0i32;
    while j < *nlen.offset(i as isize) + 1i32 {
      *(*ix.offset(i as isize)).offset(j as isize) = (*nfile[i as usize].offset(j as isize))
        .c2rust_unnamed
        .offset;
      j += 1
    }
    i += 1
  }
  /* length of prefix and suffix is calculated */
  while pref < *nlen.offset(0)
    && pref < *nlen.offset(1)
    && (*nfile[0].offset((pref + 1i32) as isize)).value
      == (*nfile[1].offset((pref + 1i32) as isize)).value
  {
    pref += 1
  }
  while suff < *nlen.offset(0) - pref
    && suff < *nlen.offset(1) - pref
    && (*nfile[0].offset((*nlen.offset(0) - suff) as isize)).value
      == (*nfile[1].offset((*nlen.offset(1) - suff) as isize)).value
  {
    suff += 1
  }
  /* Arrays are pruned by the suffix and prefix length,
   * the result being sorted and stored in sfile[fileno],
   * and their sizes are stored in slen[fileno]
   */
  j = 0i32;
  while j < 2i32 {
    sfile[j as usize] = nfile[j as usize].offset(pref as isize);
    slen[j as usize] = *nlen.offset(j as isize) - pref - suff;
    i = 0i32;
    while i <= slen[j as usize] {
      (*sfile[j as usize].offset(i as isize))
        .c2rust_unnamed
        .serial = i as libc::c_uint;
      i += 1
    }
    qsort(
      sfile[j as usize].offset(1) as *mut libc::c_void,
      slen[j as usize] as size_t,
      ::std::mem::size_of::<line>() as libc::c_ulong,
      Some(
        line_compar
          as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int,
      ),
    );
    j += 1
  }
  /* nfile arrays are reused to reduce memory pressure
   * The #if zeroed out section performs the same task as the
   * one in the #else section.
   * Peak memory usage is higher, but one array copy is avoided
   * by not using unsort()
   */
  member = nfile[1] as *mut libc::c_int;
  equiv(sfile[0], slen[0], sfile[1], slen[1], member);
  member = xrealloc(
    member as *mut libc::c_void,
    ((slen[1] + 2i32) as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
  ) as *mut libc::c_int;
  class = nfile[0] as *mut libc::c_int;
  unsort(sfile[0], slen[0], nfile[0] as *mut libc::c_int);
  class = xrealloc(
    class as *mut libc::c_void,
    ((slen[0] + 2i32) as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
  ) as *mut libc::c_int;
  J = xmalloc(
    ((*nlen.offset(0) + 2i32) as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
  ) as *mut libc::c_int;
  /* The elements of J which fall inside the prefix and suffix regions
   * are marked as unchanged, while the ones which fall outside
   * are initialized with 0 (no matches), so that function stone can
   * then assign them their right values
   */
  i = 0i32;
  delta = *nlen.offset(1) - *nlen.offset(0);
  while i <= *nlen.offset(0) {
    *J.offset(i as isize) = if i <= pref {
      i
    } else if i > *nlen.offset(0) - suff {
      (i) + delta
    } else {
      0i32
    };
    i += 1
  }
  /* Here the magic is performed */
  stone(class, slen[0], member, J, pref);
  *J.offset((*nlen.offset(0) + 1i32) as isize) = *nlen.offset(1) + 1i32;
  free(class as *mut libc::c_void);
  free(member as *mut libc::c_void);
  /* Both files are rescanned, in an effort to find any lines
   * which, due to limitations intrinsic to any hashing algorithm,
   * are different but ended up confounded as the same
   */
  i = 1i32;
  while i <= *nlen.offset(0) {
    if !(*J.offset(i as isize) == 0) {
      seek_ft(
        &mut *ft.offset(0),
        *(*ix.offset(0)).offset((i - 1i32) as isize),
      );
      seek_ft(
        &mut *ft.offset(1),
        *(*ix.offset(1)).offset((*J.offset(i as isize) - 1i32) as isize),
      );
      j = *J.offset(i as isize);
      while i <= *nlen.offset(0) && *J.offset(i as isize) == j {
        let mut tok0: token_t = 0i32;
        let mut tok1: token_t = 0i32;
        loop {
          tok0 = read_token(&mut *ft.offset(0), tok0);
          tok1 = read_token(&mut *ft.offset(1), tok1);
          if (tok0 ^ tok1) & TOK_EMPTY as libc::c_int != 0i32
            || tok0 & TOK_EMPTY as libc::c_int == 0
              && tok0 & CHAR_MASK as libc::c_int != tok1 & CHAR_MASK as libc::c_int
          {
            *J.offset(i as isize) = 0i32
            /* Break the correspondence */
          }
          if !(tok0 & tok1 & TOK_EMPTY as libc::c_int == 0) {
            break;
          }
        }
        i += 1;
        j += 1
      }
    }
    i += 1
  }
  return J;
}
unsafe extern "C" fn diff(mut fp: *mut *mut FILE, mut file: *mut *mut libc::c_char) -> bool {
  let mut nlen: [libc::c_int; 2] = [0; 2];
  let mut ix: [*mut off_t; 2] = [0 as *mut off_t; 2];
  let mut ft: [FILE_and_pos_t; 2] = [FILE_and_pos_t {
    ft_fp: 0 as *mut FILE,
    ft_pos: 0,
  }; 2];
  let mut vec: *mut vec_t = 0 as *mut vec_t;
  let mut i: libc::c_int = 1i32;
  let mut j: libc::c_int = 0;
  let mut k: libc::c_int = 0;
  let mut idx: libc::c_int = -1i32;
  let mut anychange: bool = false;
  let mut J: *mut libc::c_int = 0 as *mut libc::c_int;
  ft[0].ft_fp = *fp.offset(0);
  ft[1].ft_fp = *fp.offset(1);
  /* note that ft[i].ft_pos is unintitalized, create_J()
   * must not assume otherwise */
  J = create_J(ft.as_mut_ptr(), nlen.as_mut_ptr(), ix.as_mut_ptr());
  loop {
    let mut nonempty: bool = false;
    loop {
      let mut v: vec_t = [C2RustUnnamed_5 { a: 0, b: 0 }; 2];
      v[0].a = i;
      while v[0].a <= nlen[0]
        && *J.offset(v[0].a as isize) == *J.offset((v[0].a - 1i32) as isize) + 1i32
      {
        v[0].a += 1
      }
      v[1].a = *J.offset((v[0].a - 1i32) as isize) + 1i32;
      v[0].b = v[0].a - 1i32;
      while v[0].b < nlen[0] && *J.offset((v[0].b + 1i32) as isize) == 0 {
        v[0].b += 1
      }
      v[1].b = *J.offset((v[0].b + 1i32) as isize) - 1i32;
      /*
       * Indicate that there is a difference between lines a and b of the 'from' file
       * to get to lines c to d of the 'to' file. If a is greater than b then there
       * are no lines in the 'from' file involved and this means that there were
       * lines appended (beginning at b).  If c is greater than d then there are
       * lines missing from the 'to' file.
       */
      if v[0].a <= v[0].b || v[1].a <= v[1].b {
        /*
         * If this change is more than 'context' lines from the
         * previous change, dump the record and reset it.
         */
        let mut ct: libc::c_int = 2i32 * (*ptr_to_globals).opt_U_context + 1i32;
        if idx >= 0i32
          && v[0].a > (*vec.offset(idx as isize))[0].b + ct
          && v[1].a > (*vec.offset(idx as isize))[1].b + ct
        {
          break;
        }
        j = 0i32;
        while j < 2i32 {
          k = v[j as usize].a;
          while k <= v[j as usize].b {
            nonempty = nonempty
              | (*ix[j as usize].offset(k as isize) - *ix[j as usize].offset((k - 1i32) as isize)
                != 1i32 as libc::c_long);
            k += 1
          }
          j += 1
        }
        idx += 1;
        vec = xrealloc_vector_helper(
          vec as *mut libc::c_void,
          ((::std::mem::size_of::<vec_t>() as libc::c_ulong) << 8i32)
            .wrapping_add(6i32 as libc::c_ulong) as libc::c_uint,
          idx,
        ) as *mut vec_t;
        memcpy(
          (*vec.offset(idx as isize)).as_mut_ptr() as *mut libc::c_void,
          v.as_mut_ptr() as *const libc::c_void,
          ::std::mem::size_of::<vec_t>() as libc::c_ulong,
        );
      }
      i = v[0].b + 1i32;
      if i > nlen[0] {
        break;
      }
      *J.offset(v[0].b as isize) = v[1].b
    }
    if !(idx < 0i32
      || option_mask32 & (1i32 << FLAG_B as libc::c_int) as libc::c_uint != 0 && !nonempty)
    {
      if option_mask32 & (1i32 << FLAG_q as libc::c_int) as libc::c_uint == 0 {
        let mut lowa: libc::c_int = 0;
        let mut span: vec_t = [C2RustUnnamed_5 { a: 0, b: 0 }; 2];
        let mut cvp: *mut vec_t = vec;
        if !anychange {
          /* Print the context/unidiff header first time through */
          printf(
            b"--- %s\n\x00" as *const u8 as *const libc::c_char,
            if !(*ptr_to_globals).label[0].is_null() {
              (*ptr_to_globals).label[0]
            } else {
              *file.offset(0)
            },
          );
          printf(
            b"+++ %s\n\x00" as *const u8 as *const libc::c_char,
            if !(*ptr_to_globals).label[1].is_null() {
              (*ptr_to_globals).label[1]
            } else {
              *file.offset(1)
            },
          );
        }
        printf(b"@@\x00" as *const u8 as *const libc::c_char);
        j = 0i32;
        while j < 2i32 {
          span[j as usize].a = if 1i32 > (*cvp)[j as usize].a - (*ptr_to_globals).opt_U_context {
            1i32
          } else {
            ((*cvp)[j as usize].a) - (*ptr_to_globals).opt_U_context
          };
          let mut a: libc::c_int = span[j as usize].a;
          span[j as usize].b = if nlen[j as usize]
            < (*vec.offset(idx as isize))[j as usize].b + (*ptr_to_globals).opt_U_context
          {
            nlen[j as usize]
          } else {
            ((*vec.offset(idx as isize))[j as usize].b) + (*ptr_to_globals).opt_U_context
          };
          let mut b: libc::c_int = span[j as usize].b;
          printf(
            b" %c%d\x00" as *const u8 as *const libc::c_char,
            if j != 0 { '+' as i32 } else { '-' as i32 },
            if a < b { a } else { b },
          );
          if !(a == b) {
            printf(
              b",%d\x00" as *const u8 as *const libc::c_char,
              if a < b { (b - a) + 1i32 } else { 0i32 },
            );
          }
          j += 1
        }
        puts(b" @@\x00" as *const u8 as *const libc::c_char);
        /*
         * Output changes in "unified" diff format--the old and new lines
         * are printed together.
         */
        lowa = span[0].a;
        loop {
          let mut end: bool = cvp > &mut *vec.offset(idx as isize) as *mut vec_t;
          fetch(
            &mut *ft.as_mut_ptr().offset(0),
            ix[0],
            lowa,
            if end as libc::c_int != 0 {
              span[0].b
            } else {
              ((*cvp)[0].a) - 1i32
            },
            ' ' as i32,
          );
          if end {
            break;
          }
          j = 0i32;
          while j < 2i32 {
            fetch(
              &mut *ft.as_mut_ptr().offset(j as isize),
              ix[j as usize],
              (*cvp)[j as usize].a,
              (*cvp)[j as usize].b,
              if j != 0 { '+' as i32 } else { '-' as i32 },
            );
            j += 1
          }
          let fresh9 = cvp;
          cvp = cvp.offset(1);
          lowa = (*fresh9)[0].b + 1i32
        }
      }
      anychange = 1i32 != 0
    }
    idx = -1i32;
    if !(i <= nlen[0]) {
      break;
    }
  }
  free(vec as *mut libc::c_void);
  free(ix[0] as *mut libc::c_void);
  free(ix[1] as *mut libc::c_void);
  free(J as *mut libc::c_void);
  return anychange;
}
unsafe extern "C" fn diffreg(mut file: *mut *mut libc::c_char) -> libc::c_int {
  let mut current_block: u64;
  let mut fp: [*mut FILE; 2] = [0 as *mut FILE; 2];
  let mut binary: bool = 0i32 != 0;
  let mut differ: bool = 0i32 != 0;
  let mut status: libc::c_int = STATUS_SAME as libc::c_int;
  let mut i: libc::c_int = 0;
  fp[0] = stdin;
  fp[1] = stdin;
  i = 0i32;
  loop {
    if !(i < 2i32) {
      current_block = 6669252993407410313;
      break;
    }
    let mut fd: libc::c_int = 0i32;
    if !(*(*file.offset(i as isize)).offset(0) as libc::c_int == '-' as i32
      && *(*file.offset(i as isize)).offset(1) == 0)
    {
      if option_mask32 & (1i32 << FLAG_N as libc::c_int) as libc::c_uint == 0 {
        fd = open_or_warn(*file.offset(i as isize), 0i32);
        if fd == -1i32 {
          current_block = 15027506056153281688;
          break;
        }
      } else {
        /* -N: if some file does not exist compare it like empty */
        fd = open(*file.offset(i as isize), 0i32);
        if fd == -1i32 {
          fd = xopen(b"/dev/null\x00" as *const u8 as *const libc::c_char, 0i32)
        }
      }
    }
    /* Our diff implementation is using seek.
     * When we meet non-seekable file, we must make a temp copy.
     */
    if lseek(fd, 0i32 as __off64_t, 0i32) == -1i32 as libc::c_long && *bb_errno == 29i32 {
      let mut name: [libc::c_char; 15] =
        *::std::mem::transmute::<&[u8; 15], &mut [libc::c_char; 15]>(b"/tmp/difXXXXXX\x00");
      let mut fd_tmp: libc::c_int = xmkstemp(name.as_mut_ptr());
      unlink(name.as_mut_ptr());
      if bb_copyfd_eof(fd, fd_tmp) < 0i32 as libc::c_long {
        xfunc_die();
      }
      if fd != 0i32 {
        close(fd);
      }
      fd = fd_tmp;
      xlseek(fd, 0i32 as off_t, 0i32);
    }
    fp[i as usize] = fdopen(fd, b"r\x00" as *const u8 as *const libc::c_char);
    i += 1
  }
  match current_block {
    6669252993407410313 => {
      loop {
        let sz: size_t = (COMMON_BUFSIZE as libc::c_int / 2i32) as size_t;
        let buf0: *mut libc::c_char = bb_common_bufsiz1.as_mut_ptr();
        let buf1: *mut libc::c_char = buf0.offset(sz as isize);
        let mut j: libc::c_int = 0;
        let mut k: libc::c_int = 0;
        i = fread(buf0 as *mut libc::c_void, 1i32 as size_t, sz, fp[0]) as libc::c_int;
        j = fread(buf1 as *mut libc::c_void, 1i32 as size_t, sz, fp[1]) as libc::c_int;
        if i != j {
          differ = 1i32 != 0;
          i = if i < j { i } else { j }
        }
        if i == 0i32 {
          break;
        }
        k = 0i32;
        while k < i {
          if *buf0.offset(k as isize) == 0 || *buf1.offset(k as isize) == 0 {
            binary = 1i32 != 0
          }
          if *buf0.offset(k as isize) as libc::c_int != *buf1.offset(k as isize) as libc::c_int {
            differ = 1i32 != 0
          }
          k += 1
        }
      }
      if differ {
        if binary as libc::c_int != 0
          && option_mask32 & (1i32 << FLAG_a as libc::c_int) as libc::c_uint == 0
        {
          status = STATUS_BINARY as libc::c_int
        } else if diff(fp.as_mut_ptr(), file) {
          status = STATUS_DIFFER as libc::c_int
        }
      }
      if status != STATUS_SAME as libc::c_int {
        (*ptr_to_globals).exit_status =
          ((*ptr_to_globals).exit_status as libc::c_int | 1i32) as smallint
      }
    }
    _ => {}
  }
  fclose_if_not_stdin(fp[0]);
  fclose_if_not_stdin(fp[1]);
  return status;
}
unsafe extern "C" fn print_status(mut status: libc::c_int, mut path: *mut *mut libc::c_char) {
  match status {
    2 | 1 => {
      if option_mask32 & (1i32 << FLAG_q as libc::c_int) as libc::c_uint != 0
        || status == STATUS_BINARY as libc::c_int
      {
        printf(
          b"Files %s and %s differ\n\x00" as *const u8 as *const libc::c_char,
          *path.offset(0),
          *path.offset(1),
        );
      }
    }
    0 => {
      if option_mask32 & (1i32 << FLAG_s as libc::c_int) as libc::c_uint != 0 {
        printf(
          b"Files %s and %s are identical\n\x00" as *const u8 as *const libc::c_char,
          *path.offset(0),
          *path.offset(1),
        );
      }
    }
    _ => {}
  };
}
/* This function adds a filename to dl, the directory listing. */
unsafe extern "C" fn add_to_dirlist(
  mut filename: *const libc::c_char,
  mut _sb: *mut stat,
  mut userdata: *mut libc::c_void,
  mut _depth: libc::c_int,
) -> libc::c_int {
  let l: *mut dlist = userdata as *mut dlist;
  let mut file: *const libc::c_char = filename.offset((*l).len as isize);
  while *file as libc::c_int == '/' as i32 {
    file = file.offset(1)
  }
  (*l).dl = xrealloc_vector_helper(
    (*l).dl as *mut libc::c_void,
    ((::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong) << 8i32)
      .wrapping_add(6i32 as libc::c_ulong) as libc::c_uint,
    (*l).e,
  ) as *mut *mut libc::c_char;
  let ref mut fresh10 = *(*l).dl.offset((*l).e as isize);
  *fresh10 = xstrdup(file);
  (*l).e += 1;
  return 1i32;
}
/* If recursion is not set, this function adds the directory
 * to the list and prevents recursive_action from recursing into it.
 */
unsafe extern "C" fn skip_dir(
  mut filename: *const libc::c_char,
  mut sb: *mut stat,
  mut userdata: *mut libc::c_void,
  mut depth: libc::c_int,
) -> libc::c_int {
  if option_mask32 & (1i32 << FLAG_r as libc::c_int) as libc::c_uint == 0 && depth != 0 {
    add_to_dirlist(filename, sb, userdata, depth);
    return 2i32;
  }
  if option_mask32 & (1i32 << FLAG_N as libc::c_int) as libc::c_uint == 0 {
    /* -r without -N: no need to recurse into dirs
     * which do not exist on the "other side".
     * Testcase: diff -r /tmp /
     * (it would recurse deep into /proc without this code) */
    let l: *mut dlist = userdata as *mut dlist;
    filename = filename.offset((*l).len as isize);
    if *filename.offset(0) != 0 {
      let mut osb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
          tv_sec: 0,
          tv_nsec: 0,
        },
        st_mtim: timespec {
          tv_sec: 0,
          tv_nsec: 0,
        },
        st_ctim: timespec {
          tv_sec: 0,
          tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
      };
      let mut othername: *mut libc::c_char =
        concat_path_file((*ptr_to_globals).other_dir, filename);
      let mut r: libc::c_int = stat(othername, &mut osb);
      free(othername as *mut libc::c_void);
      if r != 0i32 || !(osb.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint) {
        /* other dir doesn't have similarly named
         * directory, don't recurse; return 1 upon
         * exit, just like diffutils' diff */
        (*ptr_to_globals).exit_status =
          ((*ptr_to_globals).exit_status as libc::c_int | 1i32) as smallint;
        return 2i32;
      }
    }
  }
  return 1i32;
}
unsafe extern "C" fn diffdir(mut p: *mut *mut libc::c_char, mut s_start: *const libc::c_char) {
  let mut list: [dlist; 2] = [dlist {
    len: 0,
    s: 0,
    e: 0,
    dl: 0 as *mut *mut libc::c_char,
  }; 2];
  let mut i: libc::c_int = 0;
  memset(
    &mut list as *mut [dlist; 2] as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<[dlist; 2]>() as libc::c_ulong,
  );
  i = 0i32;
  while i < 2i32 {
    /*list[i].s = list[i].e = 0; - memset did it */
    /*list[i].dl = NULL; */
    (*ptr_to_globals).other_dir = *p.offset((1i32 - i) as isize);
    /* We need to trim root directory prefix.
     * Using list.len to specify its length,
     * add_to_dirlist will remove it. */
    list[i as usize].len = strlen(*p.offset(i as isize));
    recursive_action(
      *p.offset(i as isize),
      (ACTION_RECURSE as libc::c_int | ACTION_FOLLOWLINKS as libc::c_int) as libc::c_uint,
      Some(
        add_to_dirlist
          as unsafe extern "C" fn(
            _: *const libc::c_char,
            _: *mut stat,
            _: *mut libc::c_void,
            _: libc::c_int,
          ) -> libc::c_int,
      ),
      Some(
        skip_dir
          as unsafe extern "C" fn(
            _: *const libc::c_char,
            _: *mut stat,
            _: *mut libc::c_void,
            _: libc::c_int,
          ) -> libc::c_int,
      ),
      &mut *list.as_mut_ptr().offset(i as isize) as *mut dlist as *mut libc::c_void,
      0i32 as libc::c_uint,
    );
    /* Sort dl alphabetically.
     * GNU diff does this ignoring any number of trailing dots.
     * We don't, so for us dotted files almost always are
     * first on the list.
     */
    qsort_string_vector(list[i as usize].dl, list[i as usize].e as libc::c_uint);
    /* If -S was set, find the starting point. */
    if !s_start.is_null() {
      while list[i as usize].s < list[i as usize].e
        && strcmp(
          *list[i as usize].dl.offset(list[i as usize].s as isize),
          s_start,
        ) < 0i32
      {
        list[i as usize].s += 1
      }
    }
    i += 1
  }
  loop
  /* Now that both dirlist1 and dirlist2 contain sorted directory
   * listings, we can start to go through dirlist1. If both listings
   * contain the same file, then do a normal diff. Otherwise, behaviour
   * is determined by whether the -N flag is set. */
  {
    let mut dp: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2]; /* if -N */
    let mut pos: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    dp[0] = if list[0].s < list[0].e {
      *list[0].dl.offset(list[0].s as isize)
    } else {
      0 as *mut libc::c_char
    };
    dp[1] = if list[1].s < list[1].e {
      *list[1].dl.offset(list[1].s as isize)
    } else {
      0 as *mut libc::c_char
    };
    if dp[0].is_null() && dp[1].is_null() {
      break;
    }
    pos = if dp[0].is_null() {
      1i32
    } else if dp[1].is_null() {
      -1i32
    } else {
      strcmp(dp[0], dp[1])
    };
    k = (pos > 0i32) as libc::c_int;
    if pos != 0 && option_mask32 & (1i32 << FLAG_N as libc::c_int) as libc::c_uint == 0 {
      printf(
        b"Only in %s: %s\n\x00" as *const u8 as *const libc::c_char,
        *p.offset(k as isize),
        dp[k as usize],
      );
      (*ptr_to_globals).exit_status =
        ((*ptr_to_globals).exit_status as libc::c_int | 1i32) as smallint
    } else {
      let mut fullpath: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
      let mut path: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
      i = 0i32;
      while i < 2i32 {
        if pos == 0i32 || i == k {
          fullpath[i as usize] = concat_path_file(*p.offset(i as isize), dp[i as usize]);
          path[i as usize] = fullpath[i as usize];
          stat(
            fullpath[i as usize],
            &mut *(*ptr_to_globals).stb.as_mut_ptr().offset(i as isize),
          );
        } else {
          fullpath[i as usize] = concat_path_file(*p.offset(i as isize), dp[(1i32 - i) as usize]);
          path[i as usize] =
            b"/dev/null\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
        i += 1
      }
      if pos != 0 {
        stat(
          fullpath[k as usize],
          &mut *(*ptr_to_globals)
            .stb
            .as_mut_ptr()
            .offset((1i32 - k) as isize),
        );
      }
      if (*ptr_to_globals).stb[0].st_mode & 0o170000i32 as libc::c_uint
        == 0o40000i32 as libc::c_uint
        && (*ptr_to_globals).stb[1].st_mode & 0o170000i32 as libc::c_uint
          == 0o40000i32 as libc::c_uint
      {
        printf(
          b"Common subdirectories: %s and %s\n\x00" as *const u8 as *const libc::c_char,
          fullpath[0],
          fullpath[1],
        );
      } else if !((*ptr_to_globals).stb[0].st_mode & 0o170000i32 as libc::c_uint
        == 0o100000i32 as libc::c_uint)
        && !((*ptr_to_globals).stb[0].st_mode & 0o170000i32 as libc::c_uint
          == 0o40000i32 as libc::c_uint)
      {
        printf(
          b"File %s is not a regular file or directory and was skipped\n\x00" as *const u8
            as *const libc::c_char,
          fullpath[0],
        );
      } else if !((*ptr_to_globals).stb[1].st_mode & 0o170000i32 as libc::c_uint
        == 0o100000i32 as libc::c_uint)
        && !((*ptr_to_globals).stb[1].st_mode & 0o170000i32 as libc::c_uint
          == 0o40000i32 as libc::c_uint)
      {
        printf(
          b"File %s is not a regular file or directory and was skipped\n\x00" as *const u8
            as *const libc::c_char,
          fullpath[1],
        );
      } else if ((*ptr_to_globals).stb[0].st_mode & 0o170000i32 as libc::c_uint
        == 0o40000i32 as libc::c_uint) as libc::c_int
        != ((*ptr_to_globals).stb[1].st_mode & 0o170000i32 as libc::c_uint
          == 0o40000i32 as libc::c_uint) as libc::c_int
      {
        if (*ptr_to_globals).stb[0].st_mode & 0o170000i32 as libc::c_uint
          == 0o40000i32 as libc::c_uint
        {
          printf(
            b"File %s is a %s while file %s is a %s\n\x00" as *const u8 as *const libc::c_char,
            fullpath[0],
            b"directory\x00" as *const u8 as *const libc::c_char,
            fullpath[1],
            b"regular file\x00" as *const u8 as *const libc::c_char,
          );
        } else {
          printf(
            b"File %s is a %s while file %s is a %s\n\x00" as *const u8 as *const libc::c_char,
            fullpath[0],
            b"regular file\x00" as *const u8 as *const libc::c_char,
            fullpath[1],
            b"directory\x00" as *const u8 as *const libc::c_char,
          );
        }
      } else {
        print_status(diffreg(path.as_mut_ptr()), fullpath.as_mut_ptr());
      }
      free(fullpath[0] as *mut libc::c_void);
      free(fullpath[1] as *mut libc::c_void);
    }
    free(dp[k as usize] as *mut libc::c_void);
    list[k as usize].s += 1;
    if pos == 0i32 {
      free(dp[(1i32 - k) as usize] as *mut libc::c_void);
      list[(1i32 - k) as usize].s += 1
    }
  }
}
static mut diff_longopts: [libc::c_char; 253] = [
  105, 103, 110, 111, 114, 101, 45, 99, 97, 115, 101, 0, 0, 105, 105, 103, 110, 111, 114, 101, 45,
  116, 97, 98, 45, 101, 120, 112, 97, 110, 115, 105, 111, 110, 0, 0, 69, 105, 103, 110, 111, 114,
  101, 45, 115, 112, 97, 99, 101, 45, 99, 104, 97, 110, 103, 101, 0, 0, 98, 105, 103, 110, 111,
  114, 101, 45, 97, 108, 108, 45, 115, 112, 97, 99, 101, 0, 0, 119, 105, 103, 110, 111, 114, 101,
  45, 98, 108, 97, 110, 107, 45, 108, 105, 110, 101, 115, 0, 0, 66, 116, 101, 120, 116, 0, 0, 97,
  117, 110, 105, 102, 105, 101, 100, 0, 1, 85, 108, 97, 98, 101, 108, 0, 1, 76, 115, 104, 111, 119,
  45, 99, 45, 102, 117, 110, 99, 116, 105, 111, 110, 0, 0, 112, 98, 114, 105, 101, 102, 0, 0, 113,
  101, 120, 112, 97, 110, 100, 45, 116, 97, 98, 115, 0, 0, 116, 105, 110, 105, 116, 105, 97, 108,
  45, 116, 97, 98, 0, 0, 84, 114, 101, 99, 117, 114, 115, 105, 118, 101, 0, 0, 114, 110, 101, 119,
  45, 102, 105, 108, 101, 0, 0, 78, 114, 101, 112, 111, 114, 116, 45, 105, 100, 101, 110, 116, 105,
  99, 97, 108, 45, 102, 105, 108, 101, 115, 0, 0, 115, 115, 116, 97, 114, 116, 105, 110, 103, 45,
  102, 105, 108, 101, 0, 1, 83, 109, 105, 110, 105, 109, 97, 108, 0, 0, 100, 0,
];
#[no_mangle]
pub unsafe extern "C" fn diff_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut gotstdin: libc::c_int = 0i32;
  let mut i: libc::c_int = 0;
  let mut file: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
  let mut s_start: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut L_arg: *mut llist_t = 0 as *mut llist_t;
  let ref mut fresh11 =
    *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
      as *mut *mut globals);
  *fresh11 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  (*ptr_to_globals).opt_U_context = 3i32;
  /* exactly 2 params; collect multiple -L <label>; -U N */
  getopt32long(
    argv,
    b"^abdiL:*NqrsS:tTU:+wupBE\x00=2\x00" as *const u8 as *const libc::c_char,
    diff_longopts.as_ptr(),
    &mut L_arg as *mut *mut llist_t,
    &mut s_start as *mut *mut libc::c_char,
    &mut (*ptr_to_globals).opt_U_context as *mut libc::c_int,
  );
  argv = argv.offset(optind as isize);
  while !L_arg.is_null() {
    (*ptr_to_globals).label[!(*ptr_to_globals).label[0].is_null() as libc::c_int as usize] =
      llist_pop(&mut L_arg) as *mut libc::c_char
  }
  /* Compat: "diff file name_which_doesnt_exist" exits with 2 */
  xfunc_error_retval = 2i32 as uint8_t;
  i = 0i32;
  while i < 2i32 {
    file[i as usize] = *argv.offset(i as isize);
    if *file[i as usize].offset(0) as libc::c_int == '-' as i32 && *file[i as usize].offset(1) == 0
    {
      fstat(
        0i32,
        &mut *(*ptr_to_globals).stb.as_mut_ptr().offset(i as isize),
      );
      gotstdin += 1
    } else if option_mask32 & (1i32 << FLAG_N as libc::c_int) as libc::c_uint != 0 {
      if stat(
        file[i as usize],
        &mut *(*ptr_to_globals).stb.as_mut_ptr().offset(i as isize),
      ) != 0
      {
        xstat(
          b"/dev/null\x00" as *const u8 as *const libc::c_char,
          &mut *(*ptr_to_globals).stb.as_mut_ptr().offset(i as isize),
        );
      }
    } else {
      xstat(
        file[i as usize],
        &mut *(*ptr_to_globals).stb.as_mut_ptr().offset(i as isize),
      );
    }
    i += 1
  }
  xfunc_error_retval = 1i32 as uint8_t;
  if gotstdin != 0
    && ((*ptr_to_globals).stb[0].st_mode & 0o170000i32 as libc::c_uint
      == 0o40000i32 as libc::c_uint
      || (*ptr_to_globals).stb[1].st_mode & 0o170000i32 as libc::c_uint
        == 0o40000i32 as libc::c_uint)
  {
    bb_simple_error_msg_and_die(
      b"can\'t compare stdin to a directory\x00" as *const u8 as *const libc::c_char,
    );
  }
  /* Compare metadata to check if the files are the same physical file.
   *
   * Comment from diffutils source says:
   * POSIX says that two files are identical if st_ino and st_dev are
   * the same, but many file systems incorrectly assign the same (device,
   * inode) pair to two distinct files, including:
   * GNU/Linux NFS servers that export all local file systems as a
   * single NFS file system, if a local device number (st_dev) exceeds
   * 255, or if a local inode number (st_ino) exceeds 16777215.
   */
  if 1i32 != 0
    && (*ptr_to_globals).stb[0].st_ino == (*ptr_to_globals).stb[1].st_ino
    && (*ptr_to_globals).stb[0].st_dev == (*ptr_to_globals).stb[1].st_dev
    && (*ptr_to_globals).stb[0].st_size == (*ptr_to_globals).stb[1].st_size
    && (*ptr_to_globals).stb[0].st_mtim.tv_sec == (*ptr_to_globals).stb[1].st_mtim.tv_sec
    && (*ptr_to_globals).stb[0].st_ctim.tv_sec == (*ptr_to_globals).stb[1].st_ctim.tv_sec
    && (*ptr_to_globals).stb[0].st_mode == (*ptr_to_globals).stb[1].st_mode
    && (*ptr_to_globals).stb[0].st_nlink == (*ptr_to_globals).stb[1].st_nlink
    && (*ptr_to_globals).stb[0].st_uid == (*ptr_to_globals).stb[1].st_uid
    && (*ptr_to_globals).stb[0].st_gid == (*ptr_to_globals).stb[1].st_gid
  {
    /* files are physically the same; no need to compare them */
    return STATUS_SAME as libc::c_int;
  }
  if (*ptr_to_globals).stb[0].st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint
    && (*ptr_to_globals).stb[1].st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint
  {
    diffdir(file.as_mut_ptr(), s_start);
  } else {
    let mut dirfile: bool = (*ptr_to_globals).stb[0].st_mode & 0o170000i32 as libc::c_uint
      == 0o40000i32 as libc::c_uint
      || (*ptr_to_globals).stb[1].st_mode & 0o170000i32 as libc::c_uint
        == 0o40000i32 as libc::c_uint;
    let mut dir: bool =
      (*ptr_to_globals).stb[1].st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint;
    if dirfile {
      let mut slash: *const libc::c_char = strrchr(file[!dir as libc::c_int as usize], '/' as i32);
      file[dir as usize] = concat_path_file(
        file[dir as usize],
        if !slash.is_null() {
          slash.offset(1)
        } else {
          file[!dir as libc::c_int as usize]
        },
      );
      xstat(
        file[dir as usize],
        &mut *(*ptr_to_globals).stb.as_mut_ptr().offset(dir as isize),
      );
    }
    /* diffreg can get non-regular files here */
    print_status(
      if gotstdin > 1i32 {
        STATUS_SAME as libc::c_int
      } else {
        diffreg(file.as_mut_ptr())
      },
      file.as_mut_ptr(),
    );
    if dirfile {
      free(file[dir as usize] as *mut libc::c_void);
    }
  }
  return (*ptr_to_globals).exit_status as libc::c_int;
}
