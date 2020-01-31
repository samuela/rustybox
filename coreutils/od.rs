use crate::libbb::llist::llist_t;
use crate::libbb::ptr_to_globals::bb_errno;
use crate::libbb::xfuncs_printf::xmalloc;
use crate::librb::size_t;
use crate::librb::smallint;
use libc;
use libc::fclose;
use libc::free;
use libc::fstat;
use libc::off64_t;
use libc::off_t;
use libc::printf;
use libc::putchar_unlocked;
use libc::puts;
use libc::stat;
use libc::strchr;
use libc::strcpy;
use libc::FILE;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  static mut stdin: *mut FILE;

  #[no_mangle]
  static mut stdout: *mut FILE;

  #[no_mangle]
  fn setbuf(__stream: *mut FILE, __buf: *mut libc::c_char);

  #[no_mangle]
  fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t, __stream: *mut FILE) -> size_t;

  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn fseeko(__stream: *mut FILE, __off: off64_t, __whence: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn ferror_unlocked(__stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn fileno_unlocked(__stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;

  #[no_mangle]
  static bkm_suffixes: [suffix_mult; 0];

  #[no_mangle]
  static bb_argv_dash: [*const libc::c_char; 0];

  #[no_mangle]
  static mut option_mask32: u32;

  #[no_mangle]
  static bb_msg_standard_input: [libc::c_char; 0];

  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

use crate::librb::suffix_mult;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct globals {
  pub exit_code: smallint,
  pub string_min: libc::c_uint,
  pub n_specs: libc::c_uint,
  pub spec: *mut tspec,
  pub format_address: Option<unsafe fn(_: off_t, _: libc::c_char) -> ()>,
  pub pseudo_offset: off_t,
  pub bytes_per_block: libc::c_uint,
  pub file_list: *const *const libc::c_char,
  pub in_stream: *mut FILE,
  pub not_first: bool,
  pub prev_pair_equal: bool,
  pub address_fmt: [libc::c_char; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tspec {
  pub fmt: output_format,
  pub size: size_spec,
  pub print_function:
    Option<unsafe fn(_: size_t, _: *const libc::c_char, _: *const libc::c_char) -> ()>,
  pub fmt_string: *mut libc::c_char,
  pub hexl_mode_trailer: libc::c_int,
  pub field_width: libc::c_int,
}
pub type size_spec = libc::c_uint;
pub const N_SIZE_SPECS: size_spec = 9;
pub const FLOAT_LONG_DOUBLE: size_spec = 8;
pub const FLOAT_DOUBLE: size_spec = 7;
pub const FLOAT_SINGLE: size_spec = 6;
pub const LONG_LONG: size_spec = 5;
pub const LONG: size_spec = 4;
pub const INT: size_spec = 3;
pub const SHORT: size_spec = 2;
pub const CHAR: size_spec = 1;
pub const NO_SIZE: size_spec = 0;
pub type output_format = libc::c_uint;
pub const CHARACTER: output_format = 6;
pub const NAMED_CHARACTER: output_format = 5;
pub const FLOATING_POINT: output_format = 4;
pub const HEXADECIMAL: output_format = 3;
pub const OCTAL: output_format = 2;
pub const UNSIGNED_DECIMAL: output_format = 1;
pub const SIGNED_DECIMAL: output_format = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const OPT_traditional: C2RustUnnamed = 262144;
pub const OPT_w: C2RustUnnamed = 131072;
pub const OPT_S: C2RustUnnamed = 65536;
pub const OPT_s: C2RustUnnamed = 32768;
pub const OPT_x: C2RustUnnamed = 16384;
pub const OPT_v: C2RustUnnamed = 8192;
pub const OPT_t: C2RustUnnamed = 4096;
pub const OPT_o: C2RustUnnamed = 2048;
pub const OPT_l: C2RustUnnamed = 1024;
pub const OPT_j: C2RustUnnamed = 512;
pub const OPT_i: C2RustUnnamed = 256;
pub const OPT_h: C2RustUnnamed = 128;
pub const OPT_f: C2RustUnnamed = 64;
pub const OPT_d: C2RustUnnamed = 32;
pub const OPT_c: C2RustUnnamed = 16;
pub const OPT_b: C2RustUnnamed = 8;
pub const OPT_a: C2RustUnnamed = 4;
pub const OPT_N: C2RustUnnamed = 2;
pub const OPT_A: C2RustUnnamed = 1;
pub type longdouble_t = f128::f128;
pub type ulonglong_t = libc::c_ulonglong;
static mut bytes_to_oct_digits: [u8; 17] = [
  0, 3, 6, 8, 11, 14, 16, 19, 22, 25, 27, 30, 32, 35, 38, 41, 43,
];
static mut bytes_to_signed_dec_digits: [u8; 17] = [
  1, 4, 6, 8, 11, 13, 16, 18, 20, 23, 25, 28, 30, 33, 35, 37, 40,
];
static mut bytes_to_unsigned_dec_digits: [u8; 17] = [
  0, 3, 5, 8, 10, 13, 15, 17, 20, 22, 25, 27, 29, 32, 34, 37, 39,
];
static mut bytes_to_hex_digits: [u8; 17] = [
  0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32,
];
static mut width_bytes: [libc::c_schar; 9] = [
  -1i32 as libc::c_schar,
  ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_schar,
  ::std::mem::size_of::<libc::c_short>() as libc::c_ulong as libc::c_schar,
  ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_schar,
  ::std::mem::size_of::<libc::c_long>() as libc::c_ulong as libc::c_schar,
  ::std::mem::size_of::<ulonglong_t>() as libc::c_ulong as libc::c_schar,
  ::std::mem::size_of::<libc::c_float>() as libc::c_ulong as libc::c_schar,
  ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_schar,
  ::std::mem::size_of::<longdouble_t>() as libc::c_ulong as libc::c_schar,
];
static mut integral_type_size: [libc::c_uchar; 9] = [
  0,
  CHAR as libc::c_int as libc::c_uchar,
  SHORT as libc::c_int as libc::c_uchar,
  0,
  INT as libc::c_int as libc::c_uchar,
  0,
  0,
  0,
  LONG as libc::c_int as libc::c_uchar,
];
static mut fp_type_size: [libc::c_uchar; 17] = [
  0,
  0,
  0,
  0,
  FLOAT_SINGLE as libc::c_int as libc::c_uchar,
  0,
  0,
  0,
  FLOAT_DOUBLE as libc::c_int as libc::c_uchar,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  FLOAT_LONG_DOUBLE as libc::c_int as libc::c_uchar,
];
unsafe fn gcd(mut u: libc::c_uint, mut v: libc::c_uint) -> libc::c_uint {
  let mut t: libc::c_uint = 0;
  while v != 0 {
    t = u.wrapping_rem(v);
    u = v;
    v = t
  }
  return u;
}
/* Compute the least common multiple of U and V.  */
unsafe fn lcm(mut u: libc::c_uint, mut v: libc::c_uint) -> libc::c_uint {
  let mut t: libc::c_uint = gcd(u, v);
  if t == 0 {
    return 0;
  }
  return u.wrapping_mul(v).wrapping_div(t);
}
unsafe fn print_s_char(
  mut n_bytes: size_t,
  mut block: *const libc::c_char,
  mut fmt_string: *const libc::c_char,
) {
  loop {
    let fresh0 = n_bytes;
    n_bytes = n_bytes.wrapping_sub(1);
    if !(fresh0 != 0) {
      break;
    }
    let mut tmp: libc::c_int = *(block as *mut libc::c_schar) as libc::c_int;
    printf(fmt_string, tmp);
    block = block.offset(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong as isize)
  }
}
unsafe fn print_char(
  mut n_bytes: size_t,
  mut block: *const libc::c_char,
  mut fmt_string: *const libc::c_char,
) {
  loop {
    let fresh1 = n_bytes;
    n_bytes = n_bytes.wrapping_sub(1);
    if !(fresh1 != 0) {
      break;
    }
    let mut tmp: libc::c_uint = *(block as *mut libc::c_uchar) as libc::c_uint;
    printf(fmt_string, tmp);
    block = block.offset(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong as isize)
  }
}
unsafe fn print_s_short(
  mut n_bytes: size_t,
  mut block: *const libc::c_char,
  mut fmt_string: *const libc::c_char,
) {
  n_bytes = (n_bytes as libc::c_ulong)
    .wrapping_div(::std::mem::size_of::<libc::c_short>() as libc::c_ulong) as size_t
    as size_t;
  loop {
    let fresh2 = n_bytes;
    n_bytes = n_bytes.wrapping_sub(1);
    if !(fresh2 != 0) {
      break;
    }
    let mut tmp: libc::c_int = *(block as *mut libc::c_short) as libc::c_int;
    printf(fmt_string, tmp);
    block = block.offset(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong as isize)
  }
}
unsafe fn print_short(
  mut n_bytes: size_t,
  mut block: *const libc::c_char,
  mut fmt_string: *const libc::c_char,
) {
  n_bytes = (n_bytes as libc::c_ulong)
    .wrapping_div(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong) as size_t
    as size_t;
  loop {
    let fresh3 = n_bytes;
    n_bytes = n_bytes.wrapping_sub(1);
    if !(fresh3 != 0) {
      break;
    }
    let mut tmp: libc::c_uint = *(block as *mut libc::c_ushort) as libc::c_uint;
    printf(fmt_string, tmp);
    block = block.offset(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong as isize)
  }
}
unsafe fn print_int(
  mut n_bytes: size_t,
  mut block: *const libc::c_char,
  mut fmt_string: *const libc::c_char,
) {
  n_bytes = (n_bytes as libc::c_ulong)
    .wrapping_div(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong) as size_t
    as size_t;
  loop {
    let fresh4 = n_bytes;
    n_bytes = n_bytes.wrapping_sub(1);
    if !(fresh4 != 0) {
      break;
    }
    let mut tmp: libc::c_uint = *(block as *mut libc::c_uint);
    printf(fmt_string, tmp);
    block = block.offset(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong as isize)
  }
}
unsafe fn print_long(
  mut n_bytes: size_t,
  mut block: *const libc::c_char,
  mut fmt_string: *const libc::c_char,
) {
  n_bytes = (n_bytes as libc::c_ulong)
    .wrapping_div(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong) as size_t
    as size_t;
  loop {
    let fresh5 = n_bytes;
    n_bytes = n_bytes.wrapping_sub(1);
    if !(fresh5 != 0) {
      break;
    }
    let mut tmp: libc::c_ulong = *(block as *mut libc::c_ulong);
    printf(fmt_string, tmp);
    block = block.offset(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong as isize)
  }
}
unsafe fn print_float(
  mut n_bytes: size_t,
  mut block: *const libc::c_char,
  mut fmt_string: *const libc::c_char,
) {
  n_bytes = (n_bytes as libc::c_ulong)
    .wrapping_div(::std::mem::size_of::<libc::c_float>() as libc::c_ulong) as size_t
    as size_t;
  loop {
    let fresh6 = n_bytes;
    n_bytes = n_bytes.wrapping_sub(1);
    if !(fresh6 != 0) {
      break;
    }
    let mut tmp: libc::c_float = *(block as *mut libc::c_float);
    printf(fmt_string, tmp as libc::c_double);
    block = block.offset(::std::mem::size_of::<libc::c_float>() as libc::c_ulong as isize)
  }
}
unsafe fn print_double(
  mut n_bytes: size_t,
  mut block: *const libc::c_char,
  mut fmt_string: *const libc::c_char,
) {
  n_bytes = (n_bytes as libc::c_ulong)
    .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong) as size_t
    as size_t;
  loop {
    let fresh7 = n_bytes;
    n_bytes = n_bytes.wrapping_sub(1);
    if !(fresh7 != 0) {
      break;
    }
    let mut tmp: libc::c_double = *(block as *mut libc::c_double);
    printf(fmt_string, tmp);
    block = block.offset(::std::mem::size_of::<libc::c_double>() as libc::c_ulong as isize)
  }
}
unsafe fn print_long_double(
  mut n_bytes: size_t,
  mut block: *const libc::c_char,
  mut fmt_string: *const libc::c_char,
) {
  n_bytes = (n_bytes as libc::c_ulong)
    .wrapping_div(::std::mem::size_of::<longdouble_t>() as libc::c_ulong) as size_t
    as size_t;
  loop {
    let fresh8 = n_bytes;
    n_bytes = n_bytes.wrapping_sub(1);
    if !(fresh8 != 0) {
      break;
    }
    let mut tmp: longdouble_t = *(block as *mut longdouble_t);
    printf(fmt_string, tmp);
    block = block.offset(::std::mem::size_of::<longdouble_t>() as libc::c_ulong as isize)
  }
}
/* print_[named]_ascii are optimized for speed.
 * Remember, someday you may want to pump gigabytes through this thing.
 * Saving a dozen of .text bytes here is counter-productive */
unsafe fn print_named_ascii(
  mut n_bytes: size_t,
  mut block: *const libc::c_char,
  mut _unused_fmt_string: *const libc::c_char,
) {
  /* Names for some non-printing characters.  */
  static mut charname: [[libc::c_char; 3]; 33] = [
    [110, 117, 108],
    [115, 111, 104],
    [115, 116, 120],
    [101, 116, 120],
    [101, 111, 116],
    [101, 110, 113],
    [97, 99, 107],
    [98, 101, 108],
    [32, 98, 115],
    [32, 104, 116],
    [32, 110, 108],
    [32, 118, 116],
    [32, 102, 102],
    [32, 99, 114],
    [32, 115, 111],
    [32, 115, 105],
    [100, 108, 101],
    [100, 99, 49],
    [100, 99, 50],
    [100, 99, 51],
    [100, 99, 52],
    [110, 97, 107],
    [115, 121, 110],
    [101, 116, 98],
    [99, 97, 110],
    [32, 101, 109],
    [115, 117, 98],
    [101, 115, 99],
    [32, 102, 115],
    [32, 103, 115],
    [32, 114, 115],
    [32, 117, 115],
    [32, 115, 112],
  ];
  // buf[N] pos:  01234 56789
  let mut buf: [libc::c_char; 12] =
    *::std::mem::transmute::<&[u8; 12], &mut [libc::c_char; 12]>(b"   x\x00 xxx\x00\x00\x00");
  loop
  // [12] because we take three 32bit stack slots anyway, and
  // gcc is too dumb to initialize with constant stores,
  // it copies initializer from rodata. Oh well.
  // https://gcc.gnu.org/bugzilla/show_bug.cgi?id=65410
  {
    let fresh9 = n_bytes;
    n_bytes = n_bytes.wrapping_sub(1);
    if !(fresh9 != 0) {
      break;
    }
    let fresh10 = block;
    block = block.offset(1);
    let mut masked_c: libc::c_uint = *(fresh10 as *mut libc::c_uchar) as libc::c_uint;
    masked_c &= 0x7fi32 as libc::c_uint;
    if masked_c == 0x7fi32 as libc::c_uint {
      fputs_unlocked(b" del\x00" as *const u8 as *const libc::c_char, stdout);
    } else if masked_c > ' ' as i32 as libc::c_uint {
      buf[3] = masked_c as libc::c_char;
      fputs_unlocked(buf.as_mut_ptr(), stdout);
    } else {
      /* Why? Because printf(" %3.3s") is much slower... */
      buf[6] = charname[masked_c as usize][0];
      buf[7] = charname[masked_c as usize][1];
      buf[8] = charname[masked_c as usize][2];
      fputs_unlocked(buf.as_mut_ptr().offset(5), stdout);
    }
  }
}
unsafe fn print_ascii(
  mut n_bytes: size_t,
  mut block: *const libc::c_char,
  mut _unused_fmt_string: *const libc::c_char,
) {
  // buf[N] pos:  01234 56789
  let mut buf: [libc::c_char; 12] =
    *::std::mem::transmute::<&[u8; 12], &mut [libc::c_char; 12]>(b"   x\x00 xxx\x00\x00\x00");
  loop {
    let fresh11 = n_bytes;
    n_bytes = n_bytes.wrapping_sub(1);
    if !(fresh11 != 0) {
      break;
    }
    let mut s: *const libc::c_char = std::ptr::null();
    let fresh12 = block;
    block = block.offset(1);
    let mut c: libc::c_uint = *(fresh12 as *mut libc::c_uchar) as libc::c_uint;
    if c >= ' ' as i32 as libc::c_uint && c < 0x7fi32 as libc::c_uint {
      buf[3] = c as libc::c_char;
      fputs_unlocked(buf.as_mut_ptr(), stdout);
    } else {
      match c {
        0 => s = b"  \\0\x00" as *const u8 as *const libc::c_char,
        7 => s = b"  \\a\x00" as *const u8 as *const libc::c_char,
        8 => s = b"  \\b\x00" as *const u8 as *const libc::c_char,
        12 => s = b"  \\f\x00" as *const u8 as *const libc::c_char,
        10 => s = b"  \\n\x00" as *const u8 as *const libc::c_char,
        13 => s = b"  \\r\x00" as *const u8 as *const libc::c_char,
        9 => s = b"  \\t\x00" as *const u8 as *const libc::c_char,
        11 => s = b"  \\v\x00" as *const u8 as *const libc::c_char,
        _ => {
          buf[6] = (c >> 6i32 & 3i32 as libc::c_uint).wrapping_add('0' as i32 as libc::c_uint)
            as libc::c_char;
          buf[7] = (c >> 3i32 & 7i32 as libc::c_uint).wrapping_add('0' as i32 as libc::c_uint)
            as libc::c_char;
          buf[8] =
            (c & 7i32 as libc::c_uint).wrapping_add('0' as i32 as libc::c_uint) as libc::c_char;
          s = buf.as_mut_ptr().offset(5)
        }
      }
      fputs_unlocked(s, stdout);
    }
  }
}
/* Given a list of one or more input filenames FILE_LIST, set the global
file pointer IN_STREAM and the global string INPUT_FILENAME to the
first one that can be successfully opened. Modify FILE_LIST to
reference the next filename in the list.  A file name of "-" is
interpreted as standard input.  If any file open fails, give an error
message and return nonzero.  */
unsafe fn open_next_file() {
  loop {
    if (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).file_list).is_null() {
      return;
    }
    let ref mut fresh13 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).file_list;
    let fresh14 = *fresh13;
    *fresh13 = (*fresh13).offset(1);
    let ref mut fresh15 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).in_stream;
    *fresh15 = crate::libbb::wfopen_input::fopen_or_warn_stdin(*fresh14);
    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .in_stream
      .is_null()
    {
      break;
    }
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).exit_code = 1i32 as smallint
  }
  if option_mask32 & (OPT_N as libc::c_int | OPT_S as libc::c_int) as libc::c_uint
    == OPT_N as libc::c_int as libc::c_uint
  {
    setbuf(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).in_stream,
      std::ptr::null_mut::<libc::c_char>(),
    );
  };
}
/* Test whether there have been errors on in_stream, and close it if
it is not standard input.  Return nonzero if there has been an error
on in_stream or stdout; return zero otherwise.  This function will
report more than one error only if both a read and a write error
have occurred.  IN_ERRNO, if nonzero, is the error number
corresponding to the most recent action for IN_STREAM.  */
unsafe fn check_and_close() {
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .in_stream
    .is_null()
  {
    if ferror_unlocked((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).in_stream) != 0 {
      crate::libbb::verror_msg::bb_error_msg(
        b"%s: read error\x00" as *const u8 as *const libc::c_char,
        if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).in_stream == stdin {
          bb_msg_standard_input.as_ptr()
        } else {
          *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .file_list
            .offset(-1i32 as isize)
        },
      );
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).exit_code = 1i32 as smallint
    }
    crate::libbb::fclose_nonstdin::fclose_if_not_stdin(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).in_stream,
    );
    let ref mut fresh16 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).in_stream;
    *fresh16 = std::ptr::null_mut()
  }
  if ferror_unlocked(stdout) != 0 {
    crate::libbb::verror_msg::bb_simple_error_msg_and_die(
      b"write error\x00" as *const u8 as *const libc::c_char,
    );
  };
}
/* If S points to a single valid modern od format string, put
 a description of that format in *TSPEC, return pointer to
 character following the just-decoded format.
 For example, if S were "d4afL", we will return a rtp to "afL"
 and *TSPEC would be
{
  fmt = SIGNED_DECIMAL;
  size = INT or LONG; (whichever integral_type_size[4] resolves to)
  print_function = print_int; (assuming size == INT)
  fmt_string = "%011d%c";
}
 S_ORIG is solely for reporting errors.  It should be the full format
 string argument. */
#[inline(never)]
unsafe fn decode_one_format(
  mut s_orig: *const libc::c_char,
  mut s: *const libc::c_char,
  mut tspec: *mut tspec,
) -> *const libc::c_char {
  let mut size_spec: size_spec = NO_SIZE;
  let mut size: libc::c_uint = 0;
  let mut fmt: output_format = SIGNED_DECIMAL;
  let mut p: *const libc::c_char = std::ptr::null();
  let mut end: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut fmt_string: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut print_function: Option<
    unsafe fn(_: size_t, _: *const libc::c_char, _: *const libc::c_char) -> (),
  > = None;
  let mut c: libc::c_uint = 0;
  let mut field_width: libc::c_uint = 0 as libc::c_uint;
  let mut pos: libc::c_int = 0;
  match *s as libc::c_int {
    100 | 111 | 117 | 120 => {
      static mut CSIL: [libc::c_char; 5] = [67, 83, 73, 76, 0];
      let fresh17 = s;
      s = s.offset(1);
      c = *fresh17 as libc::c_uint;
      p = strchr(CSIL.as_ptr(), *s as libc::c_int);
      /* if *s == NUL, p != NULL! Testcase: "od -tx" */
      if p.is_null() || *p as libc::c_int == '\u{0}' as i32 {
        size = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint;
        if (*s.offset(0) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
          size = crate::libbb::bb_strtonum::bb_strtou(s, &mut end, 0);
          if *bb_errno == 34i32
            || (::std::mem::size_of::<ulonglong_t>() as libc::c_ulong) < size as libc::c_ulong
            || integral_type_size[size as usize] as libc::c_int == NO_SIZE as libc::c_int
          {
            crate::libbb::verror_msg::bb_error_msg_and_die(
              b"invalid type string \'%s\'; %u-byte %s type is not supported\x00" as *const u8
                as *const libc::c_char,
              s_orig,
              size,
              b"integral\x00" as *const u8 as *const libc::c_char,
            );
          }
          s = end
        }
      } else {
        static mut CSIL_sizeof: [u8; 4] = [
          ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as u8,
          ::std::mem::size_of::<libc::c_short>() as libc::c_ulong as u8,
          ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as u8,
          ::std::mem::size_of::<libc::c_long>() as libc::c_ulong as u8,
        ];
        size = CSIL_sizeof[p.wrapping_offset_from(CSIL.as_ptr()) as libc::c_long as usize]
          as libc::c_uint;
        s = s.offset(1)
        /* skip C/S/I/L */
      }
      size_spec = integral_type_size[size as usize] as size_spec;
      static mut doux: [libc::c_char; 5] = [100, 111, 117, 120, 0];
      static mut doux_fmt_letter: [[libc::c_char; 4]; 4] = [
        [108, 108, 100, 0],
        [108, 108, 111, 0],
        [108, 108, 117, 0],
        [108, 108, 120, 0],
      ];
      static mut doux_fmt: [output_format; 4] =
        [SIGNED_DECIMAL, OCTAL, UNSIGNED_DECIMAL, HEXADECIMAL];
      static mut doux_bytes_to_XXX: [*const u8; 4] = unsafe {
        [
          bytes_to_signed_dec_digits.as_ptr(),
          bytes_to_oct_digits.as_ptr(),
          bytes_to_unsigned_dec_digits.as_ptr(),
          bytes_to_hex_digits.as_ptr(),
        ]
      };
      static mut doux_fmtstring: [[libc::c_char; 9]; 4] = [
        [32, 37, 37, 37, 117, 37, 115, 0, 0],
        [32, 37, 37, 48, 37, 117, 37, 115, 0],
        [32, 37, 37, 37, 117, 37, 115, 0, 0],
        [32, 37, 37, 48, 37, 117, 37, 115, 0],
      ];
      pos = strchr(doux.as_ptr(), c as libc::c_int).wrapping_offset_from(doux.as_ptr())
        as libc::c_long as libc::c_int;
      fmt = doux_fmt[pos as usize];
      field_width = *doux_bytes_to_XXX[pos as usize].offset(size as isize) as libc::c_uint;
      p = doux_fmt_letter[pos as usize].as_ptr().offset(2);
      if size_spec as libc::c_uint == LONG as libc::c_int as libc::c_uint {
        p = p.offset(-1)
      }
      if size_spec as libc::c_uint == LONG_LONG as libc::c_int as libc::c_uint {
        p = p.offset(-2)
      }
      fmt_string = crate::libbb::xfuncs_printf::xasprintf(
        doux_fmtstring[pos as usize].as_ptr(),
        field_width,
        p,
      );
      match size_spec as libc::c_uint {
        1 => {
          print_function = if fmt as libc::c_uint == SIGNED_DECIMAL as libc::c_int as libc::c_uint {
            Some(
              print_s_char
                as unsafe fn(_: size_t, _: *const libc::c_char, _: *const libc::c_char) -> (),
            )
          } else {
            Some(
              print_char
                as unsafe fn(_: size_t, _: *const libc::c_char, _: *const libc::c_char) -> (),
            )
          }
        }
        2 => {
          print_function = if fmt as libc::c_uint == SIGNED_DECIMAL as libc::c_int as libc::c_uint {
            Some(
              print_s_short
                as unsafe fn(_: size_t, _: *const libc::c_char, _: *const libc::c_char) -> (),
            )
          } else {
            Some(
              print_short
                as unsafe fn(_: size_t, _: *const libc::c_char, _: *const libc::c_char) -> (),
            )
          }
        }
        3 => {
          print_function = Some(
            print_int as unsafe fn(_: size_t, _: *const libc::c_char, _: *const libc::c_char) -> (),
          )
        }
        4 => {
          print_function = Some(
            print_long
              as unsafe fn(_: size_t, _: *const libc::c_char, _: *const libc::c_char) -> (),
          )
        }
        _ => {
          /* case LONG_LONG: */
          print_function = Some(
            print_long
              as unsafe fn(_: size_t, _: *const libc::c_char, _: *const libc::c_char) -> (),
          )
        }
      }
    }
    102 => {
      static mut FDL: [libc::c_char; 4] = [70, 68, 76, 0];
      fmt = FLOATING_POINT;
      s = s.offset(1);
      p = strchr(FDL.as_ptr(), *s as libc::c_int);
      if p.is_null() || *p as libc::c_int == '\u{0}' as i32 {
        size = ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_uint;
        if (*s.offset(0) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
          size = crate::libbb::bb_strtonum::bb_strtou(s, &mut end, 0);
          if *bb_errno == 34i32
            || size as libc::c_ulong > ::std::mem::size_of::<longdouble_t>() as libc::c_ulong
            || fp_type_size[size as usize] as libc::c_int == NO_SIZE as libc::c_int
          {
            crate::libbb::verror_msg::bb_error_msg_and_die(
              b"invalid type string \'%s\'; %u-byte %s type is not supported\x00" as *const u8
                as *const libc::c_char,
              s_orig,
              size,
              b"floating point\x00" as *const u8 as *const libc::c_char,
            );
          }
          s = end
        }
      } else {
        static mut FDL_sizeof: [u8; 3] = [
          ::std::mem::size_of::<libc::c_float>() as libc::c_ulong as u8,
          ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as u8,
          ::std::mem::size_of::<longdouble_t>() as libc::c_ulong as u8,
        ];
        size =
          FDL_sizeof[p.wrapping_offset_from(FDL.as_ptr()) as libc::c_long as usize] as libc::c_uint;
        s = s.offset(1)
        /* skip F/D/L */
      }
      size_spec = fp_type_size[size as usize] as size_spec;
      match size_spec as libc::c_uint {
        6 => {
          print_function = Some(
            print_float
              as unsafe fn(_: size_t, _: *const libc::c_char, _: *const libc::c_char) -> (),
          );
          field_width = (7i32 + 8i32) as libc::c_uint;
          /* Don't use %#e; not all systems support it.  */
          fmt_string = crate::libbb::xfuncs_printf::xasprintf(
            b" %%%d.%de\x00" as *const u8 as *const libc::c_char,
            field_width,
            7i32,
          )
        }
        7 => {
          print_function = Some(
            print_double
              as unsafe fn(_: size_t, _: *const libc::c_char, _: *const libc::c_char) -> (),
          );
          field_width = (15i32 + 8i32) as libc::c_uint;
          fmt_string = crate::libbb::xfuncs_printf::xasprintf(
            b" %%%d.%de\x00" as *const u8 as *const libc::c_char,
            field_width,
            15i32,
          )
        }
        _ => {
          /* case FLOAT_LONG_DOUBLE: */
          print_function = Some(
            print_long_double
              as unsafe fn(_: size_t, _: *const libc::c_char, _: *const libc::c_char) -> (),
          );
          field_width = (15i32 + 8i32) as libc::c_uint;
          fmt_string = crate::libbb::xfuncs_printf::xasprintf(
            b" %%%d.%dLe\x00" as *const u8 as *const libc::c_char,
            field_width,
            15i32,
          )
        }
      }
    }
    97 => {
      s = s.offset(1);
      fmt = NAMED_CHARACTER;
      size_spec = CHAR;
      print_function = Some(
        print_named_ascii
          as unsafe fn(_: size_t, _: *const libc::c_char, _: *const libc::c_char) -> (),
      );
      field_width = 3i32 as libc::c_uint
    }
    99 => {
      s = s.offset(1);
      fmt = CHARACTER;
      size_spec = CHAR;
      print_function = Some(
        print_ascii as unsafe fn(_: size_t, _: *const libc::c_char, _: *const libc::c_char) -> (),
      );
      field_width = 3i32 as libc::c_uint
    }
    _ => {
      crate::libbb::verror_msg::bb_error_msg_and_die(
        b"invalid character \'%c\' in type string \'%s\'\x00" as *const u8 as *const libc::c_char,
        *s as libc::c_int,
        s_orig,
      );
    }
  }
  (*tspec).size = size_spec;
  (*tspec).fmt = fmt;
  (*tspec).print_function = print_function;
  (*tspec).fmt_string = fmt_string;
  (*tspec).field_width = field_width as libc::c_int;
  (*tspec).hexl_mode_trailer = (*s as libc::c_int == 'z' as i32) as libc::c_int;
  if (*tspec).hexl_mode_trailer != 0 {
    s = s.offset(1)
  }
  return s;
}
/* Decode the modern od format string S.  Append the decoded
representation to the global array SPEC, reallocating SPEC if
necessary.  */
/* Given a list of one or more input filenames FILE_LIST, set the global
file pointer IN_STREAM to position N_SKIP in the concatenation of
those files.  If any file operation fails or if there are fewer than
N_SKIP bytes in the combined input, give an error message and return
nonzero.  When possible, use seek rather than read operations to
advance IN_STREAM.  */
unsafe fn skip(mut n_skip: off_t) {
  if n_skip == 0 {
    return;
  }
  while !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .in_stream
    .is_null()
  {
    /* !EOF */
    let mut file_stats: stat = std::mem::zeroed();
    /* First try seeking.  For large offsets, this extra work is
     worthwhile.  If the offset is below some threshold it may be
     more efficient to move the pointer by reading.  There are two
     issues when trying to seek:
    - the file must be seekable.
    - before seeking to the specified position, make sure
      that the new position is in the current file.
      Try to do that by getting file's size using fstat.
      But that will work only for regular files.  */
    /* The st_size field is valid only for regular files
    (and for symbolic links, which cannot occur here).
    If the number of bytes left to skip is at least
    as large as the size of the current file, we can
    decrement n_skip and go on to the next file.  */
    if fstat(
      fileno_unlocked((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).in_stream),
      &mut file_stats,
    ) == 0
      && file_stats.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint
      && file_stats.st_size > 0
    {
      if file_stats.st_size < n_skip {
        n_skip -= file_stats.st_size
      /* take "check & close / open_next" route */
      } else {
        if fseeko(
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).in_stream,
          n_skip,
          1i32,
        ) != 0
        {
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).exit_code = 1i32 as smallint
        }
        return;
      }
    } else {
      /* If it's not a regular file with positive size,
      position the file pointer by reading.  */
      let mut buf: [libc::c_char; 1024] = [0; 1024];
      let mut n_bytes_to_read: size_t = 1024i32 as size_t;
      let mut n_bytes_read: size_t = 0;
      while n_skip > 0 {
        if (n_skip as libc::c_ulong) < n_bytes_to_read {
          n_bytes_to_read = n_skip as size_t
        }
        n_bytes_read = fread(
          buf.as_mut_ptr() as *mut libc::c_void,
          1i32 as size_t,
          n_bytes_to_read,
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).in_stream,
        );
        n_skip = (n_skip as libc::c_ulong).wrapping_sub(n_bytes_read) as off_t as off_t;
        if n_bytes_read != n_bytes_to_read {
          break;
        }
        /* EOF on this file or error */
      }
    }
    if n_skip == 0 {
      return;
    }
    check_and_close();
    open_next_file();
  }
  if n_skip != 0 {
    crate::libbb::verror_msg::bb_simple_error_msg_and_die(
      b"can\'t skip past end of combined input\x00" as *const u8 as *const libc::c_char,
    );
  };
}
unsafe fn format_address_none(mut _address: off_t, mut _c: libc::c_char) {}
unsafe fn decode_format_string(mut s: *const libc::c_char) {
  let mut s_orig: *const libc::c_char = s;
  while *s as libc::c_int != '\u{0}' as i32 {
    let mut tspec: tspec = tspec {
      fmt: SIGNED_DECIMAL,
      size: NO_SIZE,
      print_function: None,
      fmt_string: std::ptr::null_mut::<libc::c_char>(),
      hexl_mode_trailer: 0,
      field_width: 0,
    };
    let mut next: *const libc::c_char = std::ptr::null();
    next = decode_one_format(s_orig, s, &mut tspec);
    s = next;
    let ref mut fresh18 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).spec;
    *fresh18 = crate::libbb::xrealloc_vector::xrealloc_vector_helper(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).spec as *mut libc::c_void,
      ((::std::mem::size_of::<tspec>() as libc::c_ulong) << 8i32)
        .wrapping_add(4i32 as libc::c_ulong) as libc::c_uint,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).n_specs as libc::c_int,
    ) as *mut tspec;
    memcpy(
      &mut *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .spec
        .offset((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).n_specs as isize)
        as *mut tspec as *mut libc::c_void,
      &mut tspec as *mut tspec as *const libc::c_void,
      ::std::mem::size_of::<tspec>() as libc::c_ulong,
    );
    let ref mut fresh19 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).n_specs;
    *fresh19 = (*fresh19).wrapping_add(1)
  }
}
unsafe fn format_address_std(mut address: off_t, mut c: libc::c_char) {
  /* Corresponds to 'c' */
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).address_fmt[(::std::mem::size_of::<
    [libc::c_char; 7],
  >() as libc::c_ulong)
    .wrapping_sub(2i32 as libc::c_ulong)
    as usize] = c;
  printf(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .address_fmt
      .as_mut_ptr(),
    address,
  );
}
/* only used with --traditional */
unsafe fn format_address_paren(mut address: off_t, mut c: libc::c_char) {
  putchar_unlocked('(' as i32);
  format_address_std(address, ')' as i32 as libc::c_char);
  if c != 0 {
    putchar_unlocked(c as libc::c_int);
  };
}
unsafe fn format_address_label(mut address: off_t, mut c: libc::c_char) {
  format_address_std(address, ' ' as i32 as libc::c_char);
  format_address_paren(
    address + (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pseudo_offset,
    c,
  );
}
unsafe fn dump_hexl_mode_trailer(mut n_bytes: size_t, mut block: *const libc::c_char) {
  fputs_unlocked(b"  >\x00" as *const u8 as *const libc::c_char, stdout);
  loop {
    let fresh20 = n_bytes;
    n_bytes = n_bytes.wrapping_sub(1);
    if !(fresh20 != 0) {
      break;
    }
    let fresh21 = block;
    block = block.offset(1);
    let mut c: libc::c_uint = *(fresh21 as *mut libc::c_uchar) as libc::c_uint;
    c = if c >= ' ' as i32 as libc::c_uint && c < 0x7fi32 as libc::c_uint {
      c
    } else {
      '.' as i32 as libc::c_uint
    };
    putchar_unlocked(c as libc::c_int);
  }
  putchar_unlocked('<' as i32);
}
/* Write N_BYTES bytes from CURR_BLOCK to standard output once for each
of the N_SPEC format specs.  CURRENT_OFFSET is the byte address of
CURR_BLOCK in the concatenation of input files, and it is printed
(optionally) only before the output line associated with the first
format spec.  When duplicate blocks are being abbreviated, the output
for a sequence of identical input blocks is the output for the first
block followed by an asterisk alone on a line.  It is valid to compare
the blocks PREV_BLOCK and CURR_BLOCK only when N_BYTES == BYTES_PER_BLOCK.
That condition may be false only for the last input block -- and then
only when it has not been padded to length BYTES_PER_BLOCK.  */
unsafe fn write_block(
  mut current_offset: off_t,
  mut n_bytes: size_t,
  mut prev_block: *const libc::c_char,
  mut curr_block: *const libc::c_char,
) {
  let mut i: libc::c_uint = 0;
  if option_mask32 & OPT_v as libc::c_int as libc::c_uint == 0
    && (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).not_first as libc::c_int != 0
    && n_bytes
      == (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).bytes_per_block as libc::c_ulong
    && memcmp(
      prev_block as *const libc::c_void,
      curr_block as *const libc::c_void,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).bytes_per_block as libc::c_ulong,
    ) == 0
  {
    if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).prev_pair_equal {
      puts(b"*\x00" as *const u8 as *const libc::c_char);
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).prev_pair_equal = 1i32 != 0
    }
  } else {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).not_first = 1i32 != 0;
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).prev_pair_equal = 0 != 0;
    i = 0 as libc::c_uint;
    while i < (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).n_specs {
      if i == 0 as libc::c_uint {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .format_address
          .expect("non-null function pointer")(
          current_offset, '\u{0}' as i32 as libc::c_char
        );
      } else {
        printf(
          b"%*s\x00" as *const u8 as *const libc::c_char,
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).address_fmt[2] as libc::c_int
            - '0' as i32,
          b"\x00" as *const u8 as *const libc::c_char,
        );
      }
      Some(
        (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .spec
          .offset(i as isize))
        .print_function
        .expect("non-null function pointer"),
      )
      .expect("non-null function pointer")(
        n_bytes,
        curr_block,
        (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .spec
          .offset(i as isize))
        .fmt_string,
      );
      if (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .spec
        .offset(i as isize))
      .hexl_mode_trailer
        != 0
      {
        /* space-pad out to full line width, then dump the trailer */
        let mut datum_width: libc::c_uint = width_bytes[(*(*(bb_common_bufsiz1.as_mut_ptr()
          as *mut globals))
          .spec
          .offset(i as isize))
        .size as usize] as libc::c_uint;
        let mut blank_fields: libc::c_uint =
          ((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).bytes_per_block as libc::c_ulong)
            .wrapping_sub(n_bytes)
            .wrapping_div(datum_width as libc::c_ulong) as libc::c_uint;
        let mut field_width: libc::c_uint = ((*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .spec
          .offset(i as isize))
        .field_width
          + 1i32) as libc::c_uint;
        printf(
          b"%*s\x00" as *const u8 as *const libc::c_char,
          blank_fields.wrapping_mul(field_width),
          b"\x00" as *const u8 as *const libc::c_char,
        );
        dump_hexl_mode_trailer(n_bytes, curr_block);
      }
      putchar_unlocked('\n' as i32);
      i = i.wrapping_add(1)
    }
  };
}
unsafe fn read_block(
  mut n: size_t,
  mut block: *mut libc::c_char,
  mut n_bytes_in_buffer: *mut size_t,
) {
  *n_bytes_in_buffer = 0 as size_t;
  if n == 0 as libc::c_ulong {
    return;
  }
  while !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .in_stream
    .is_null()
  {
    /* EOF.  */
    let mut n_needed: size_t = 0;
    let mut n_read: size_t = 0;
    n_needed = n.wrapping_sub(*n_bytes_in_buffer);
    n_read = fread(
      block.offset(*n_bytes_in_buffer as isize) as *mut libc::c_void,
      1i32 as size_t,
      n_needed,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).in_stream,
    );
    *n_bytes_in_buffer =
      (*n_bytes_in_buffer as libc::c_ulong).wrapping_add(n_read) as size_t as size_t;
    if n_read == n_needed {
      break;
    }
    /* error check is done in check_and_close */
    check_and_close();
    open_next_file();
  }
}
/* Return the least common multiple of the sizes associated
with the format specs.  */
unsafe fn get_lcm() -> libc::c_int {
  let mut i: size_t = 0;
  let mut l_c_m: libc::c_int = 1i32;
  i = 0 as size_t;
  while i < (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).n_specs as libc::c_ulong {
    l_c_m = lcm(
      l_c_m as libc::c_uint,
      width_bytes[(*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .spec
        .offset(i as isize))
      .size as libc::c_int as usize] as libc::c_uint,
    ) as libc::c_int;
    i = i.wrapping_add(1)
  }
  return l_c_m;
}
/* Read a chunk of size BYTES_PER_BLOCK from the input files, write the
formatted block to standard output, and repeat until the specified
maximum number of bytes has been read or until all input has been
processed.  If the last block read is smaller than BYTES_PER_BLOCK
and its size is not a multiple of the size associated with a format
spec, extend the input block with zero bytes until its length is a
multiple of all format spec sizes.  Write the final block.  Finally,
write on a line by itself the offset of the byte after the last byte
read.  */
unsafe fn dump(mut current_offset: off_t, mut end_offset: off_t) {
  let mut block: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
  let mut idx: libc::c_int = 0;
  let mut n_bytes_read: size_t = 0;
  block[0] = xmalloc(
    (2i32 as libc::c_uint)
      .wrapping_mul((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).bytes_per_block)
      as size_t,
  ) as *mut libc::c_char;
  block[1] =
    block[0].offset((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).bytes_per_block as isize);
  idx = 0;
  if option_mask32 & OPT_N as libc::c_int as libc::c_uint != 0 {
    loop {
      let mut n_needed: size_t = 0;
      if current_offset >= end_offset {
        n_bytes_read = 0 as size_t;
        break;
      } else {
        n_needed = if end_offset - current_offset
          < (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).bytes_per_block as off_t
        {
          (end_offset) - current_offset
        } else {
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).bytes_per_block as off_t
        } as size_t;
        read_block(n_needed, block[idx as usize], &mut n_bytes_read);
        if n_bytes_read
          < (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).bytes_per_block as libc::c_ulong
        {
          break;
        }
        write_block(
          current_offset,
          n_bytes_read,
          block[(idx ^ 1i32) as usize],
          block[idx as usize],
        );
        current_offset =
          (current_offset as libc::c_ulong).wrapping_add(n_bytes_read) as off_t as off_t;
        idx ^= 1i32
      }
    }
  } else {
    loop {
      read_block(
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).bytes_per_block as size_t,
        block[idx as usize],
        &mut n_bytes_read,
      );
      if n_bytes_read
        < (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).bytes_per_block as libc::c_ulong
      {
        break;
      }
      write_block(
        current_offset,
        n_bytes_read,
        block[(idx ^ 1i32) as usize],
        block[idx as usize],
      );
      current_offset =
        (current_offset as libc::c_ulong).wrapping_add(n_bytes_read) as off_t as off_t;
      idx ^= 1i32
    }
  }
  if n_bytes_read > 0 as libc::c_ulong {
    let mut l_c_m: libc::c_int = 0;
    let mut bytes_to_write: size_t = 0;
    l_c_m = get_lcm();
    /* Make bytes_to_write the smallest multiple of l_c_m that
    is at least as large as n_bytes_read.  */
    bytes_to_write = (l_c_m as libc::c_ulong).wrapping_mul(
      n_bytes_read
        .wrapping_add(l_c_m as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
        .wrapping_div(l_c_m as libc::c_ulong),
    );
    memset(
      block[idx as usize].offset(n_bytes_read as isize) as *mut libc::c_void,
      0,
      bytes_to_write.wrapping_sub(n_bytes_read),
    );
    write_block(
      current_offset,
      bytes_to_write,
      block[(idx ^ 1i32) as usize],
      block[idx as usize],
    );
    current_offset = (current_offset as libc::c_ulong).wrapping_add(n_bytes_read) as off_t as off_t
  }
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .format_address
    .expect("non-null function pointer")(current_offset, '\n' as i32 as libc::c_char);
  if option_mask32 & OPT_N as libc::c_int as libc::c_uint != 0 && current_offset >= end_offset {
    check_and_close();
  }
  free(block[0] as *mut libc::c_void);
}
/* Read N bytes into BLOCK from the concatenation of the input files
named in the global array FILE_LIST.  On the first call to this
function, the global variable IN_STREAM is expected to be an open
stream associated with the input file INPUT_FILENAME.  If all N
bytes cannot be read from IN_STREAM, close IN_STREAM and update
the global variables IN_STREAM and INPUT_FILENAME.  Then try to
read the remaining bytes from the newly opened file.  Repeat if
necessary until EOF is reached for the last file in FILE_LIST.
On subsequent calls, don't modify BLOCK and return zero.  Set
*N_BYTES_IN_BUFFER to the number of bytes read.  If an error occurs,
it will be detected through ferror when the stream is about to be
closed.  If there is an error, give a message but continue reading
as usual and return nonzero.  Otherwise return zero.  */
/* STRINGS mode.  Find each "string constant" in the input.
A string constant is a run of at least 'string_min' ASCII
graphic (or formatting) characters terminated by a null.
Based on a function written by Richard Stallman for a
traditional version of od.  */
unsafe fn dump_strings(mut address: off_t, mut end_offset: off_t) {
  let mut current_block: u64;
  let mut bufsize: libc::c_uint =
    if 100i32 as libc::c_uint > (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).string_min {
      100i32 as libc::c_uint
    } else {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).string_min
    };
  let mut buf: *mut libc::c_uchar = xmalloc(bufsize as size_t) as *mut libc::c_uchar;
  's_11: loop {
    let mut i: size_t = 0;
    let mut c: libc::c_int = 0;
    /* See if the next 'G.string_min' chars are all printing chars.  */
    'c_12120: loop
    /* Too short! */
    {
      if option_mask32 & OPT_N as libc::c_int as libc::c_uint != 0
        && end_offset
          - (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).string_min as libc::c_long
          <= address
      {
        current_block = 2480299350034459858;
        break 's_11;
      }
      i = 0 as size_t;
      while option_mask32 & OPT_N as libc::c_int as libc::c_uint == 0 || address < end_offset {
        if i == bufsize as libc::c_ulong {
          bufsize = bufsize.wrapping_add(bufsize.wrapping_div(8i32 as libc::c_uint));
          buf = crate::libbb::xfuncs_printf::xrealloc(buf as *mut libc::c_void, bufsize as size_t)
            as *mut libc::c_uchar
        }
        loop {
          if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .in_stream
            .is_null()
          {
            current_block = 1715091323959057865;
            break 's_11;
          }
          /* String continues; store it all.  */
          /* !EOF */
          c = getc_unlocked((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).in_stream); /* It isn't; give up on this string.  */
          if c != -1i32 {
            break;
          }
          check_and_close();
          open_next_file();
        }
        address += 1;
        if c == 0 {
          break;
        }
        if !(c >= ' ' as i32 && c < 0x7fi32) {
          continue 'c_12120;
        }
        let fresh22 = i;
        i = i.wrapping_add(1);
        *buf.offset(fresh22 as isize) = c as libc::c_uchar
      }
      if !(i < (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).string_min as libc::c_ulong) {
        break;
      }
    }
    /* If we get here, the string is all printable and NUL-terminated */
    *buf.offset(i as isize) = 0 as libc::c_uchar;
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .format_address
      .expect("non-null function pointer")(
      (address as libc::c_ulong)
        .wrapping_sub(i)
        .wrapping_sub(1i32 as libc::c_ulong) as off_t,
      ' ' as i32 as libc::c_char,
    );
    i = 0 as size_t;
    loop {
      c = *buf.offset(i as isize) as libc::c_int;
      if !(c != 0) {
        break;
      }
      match c {
        7 => {
          fputs_unlocked(b"\\a\x00" as *const u8 as *const libc::c_char, stdout);
        }
        8 => {
          fputs_unlocked(b"\\b\x00" as *const u8 as *const libc::c_char, stdout);
        }
        12 => {
          fputs_unlocked(b"\\f\x00" as *const u8 as *const libc::c_char, stdout);
        }
        10 => {
          fputs_unlocked(b"\\n\x00" as *const u8 as *const libc::c_char, stdout);
        }
        13 => {
          fputs_unlocked(b"\\r\x00" as *const u8 as *const libc::c_char, stdout);
        }
        9 => {
          fputs_unlocked(b"\\t\x00" as *const u8 as *const libc::c_char, stdout);
        }
        11 => {
          fputs_unlocked(b"\\v\x00" as *const u8 as *const libc::c_char, stdout);
        }
        _ => {
          putchar_unlocked(c);
        }
      }
      i = i.wrapping_add(1)
    }
    putchar_unlocked('\n' as i32);
  }
  match current_block {
    2480299350034459858 => {
      /* We reach this point only if we search through
      (max_bytes_to_format - G.string_min) bytes before reaching EOF.  */
      check_and_close();
    }
    _ => {}
  }
  /* EOF */
  free(buf as *mut libc::c_void);
}
/* If S is a valid traditional offset specification with an optional
leading '+' return nonzero and set *OFFSET to the offset it denotes.  */
unsafe fn parse_old_offset(
  mut s: *const libc::c_char,
  mut offset: *mut off_t,
) -> libc::c_int {
  static mut Bb: [suffix_mult; 3] = [
    {
      let mut init = suffix_mult {
        suffix: [66, 0, 0, 0],
        mult: 1024i32 as libc::c_uint,
      };
      init
    },
    {
      let mut init = suffix_mult {
        suffix: [98, 0, 0, 0],
        mult: 512i32 as libc::c_uint,
      };
      init
    },
    {
      let mut init = suffix_mult {
        suffix: [0, 0, 0, 0],
        mult: 0 as libc::c_uint,
      };
      init
    },
  ];
  let mut p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut radix: libc::c_int = 0;
  /* Skip over any leading '+'. */
  if *s.offset(0) as libc::c_int == '+' as i32 {
    s = s.offset(1)
  } /* not a number */
  if !((*s.offset(0) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32) {
    return 0;
  }
  /* Determine the radix we'll use to interpret S.  If there is a '.',
   * it's decimal, otherwise, if the string begins with '0X'or '0x',
   * it's hexadecimal, else octal.  */
  p = strchr(s, '.' as i32); /* cheating */
  radix = 8i32;
  if !p.is_null() {
    *p.offset(0) = '\u{0}' as i32 as libc::c_char;
    radix = 10i32
  } else if *s.offset(0) as libc::c_int == '0' as i32
    && (*s.offset(1) as libc::c_int == 'x' as i32 || *s.offset(1) as libc::c_int == 'X' as i32)
  {
    radix = 16i32
  }
  *offset = crate::libbb::xatonum::xstrtoull_sfx(s, radix, Bb.as_ptr()) as off_t;
  if !p.is_null() {
    *p.offset(0) = '.' as i32 as libc::c_char
  }
  return (*offset >= 0) as libc::c_int;
}

pub unsafe fn od_main(mut _argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
  static mut od_longopts: [libc::c_char; 104] = [
    115, 107, 105, 112, 45, 98, 121, 116, 101, 115, 0, 1, 106, 97, 100, 100, 114, 101, 115, 115,
    45, 114, 97, 100, 105, 120, 0, 1, 65, 114, 101, 97, 100, 45, 98, 121, 116, 101, 115, 0, 1, 78,
    102, 111, 114, 109, 97, 116, 0, 1, 116, 111, 117, 116, 112, 117, 116, 45, 100, 117, 112, 108,
    105, 99, 97, 116, 101, 115, 0, 0, 118, 115, 116, 114, 105, 110, 103, 115, 0, 2, 83, 119, 105,
    100, 116, 104, 0, 2, 119, 116, 114, 97, 100, 105, 116, 105, 111, 110, 97, 108, 0, 0, -1, 0,
  ];
  let mut str_A: *const libc::c_char = std::ptr::null();
  let mut str_N: *const libc::c_char = std::ptr::null();
  let mut str_j: *const libc::c_char = std::ptr::null();
  let mut str_S: *const libc::c_char = b"3\x00" as *const u8 as *const libc::c_char;
  let mut lst_t: *mut llist_t = std::ptr::null_mut();
  let mut opt: libc::c_uint = 0;
  let mut l_c_m: libc::c_int = 0;
  /* The number of input bytes to skip before formatting and writing.  */
  let mut n_bytes_to_skip: off_t = 0 as off_t;
  /* The offset of the first byte after the last byte to be formatted.  */
  let mut end_offset: off_t = 0 as off_t;
  /* The maximum number of bytes that will be formatted.  */
  let mut max_bytes_to_format: off_t = 0 as off_t;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).bytes_per_block = 32i32 as libc::c_uint;
  strcpy(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .address_fmt
      .as_mut_ptr(),
    b"%0nlxc\x00" as *const u8 as *const libc::c_char,
  );
  /*G.spec = NULL; - already is */
  let ref mut fresh23 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).format_address;
  *fresh23 = Some(format_address_std as unsafe fn(_: off_t, _: libc::c_char) -> ());
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).address_fmt[(::std::mem::size_of::<
    [libc::c_char; 7],
  >() as libc::c_ulong)
    .wrapping_sub(3i32 as libc::c_ulong)
    as usize] = 'o' as i32 as libc::c_char;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).address_fmt[2] = '7' as i32 as libc::c_char;
  /* Parse command line */
  opt = crate::libbb::getopt32::getopt32long(
    argv,
    b"A:N:abcdfhij:lot:*vxsS:w:+:\x00" as *const u8 as *const libc::c_char,
    od_longopts.as_ptr(),
    &mut str_A as *mut *const libc::c_char,
    &mut str_N as *mut *const libc::c_char,
    &mut str_j as *mut *const libc::c_char,
    &mut lst_t as *mut *mut llist_t,
    &mut str_S as *mut *const libc::c_char,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).bytes_per_block as *mut libc::c_uint,
  );
  argv = argv.offset(optind as isize);
  if opt & OPT_A as libc::c_int as libc::c_uint != 0 {
    static mut doxn: [libc::c_char; 5] = [100, 111, 120, 110, 0];
    static mut doxn_address_base_char: [libc::c_char; 3] = [
      'u' as i32 as libc::c_char,
      'o' as i32 as libc::c_char,
      'x' as i32 as libc::c_char,
    ];
    static mut doxn_address_pad_len_char: [u8; 3] =
      ['7' as i32 as u8, '7' as i32 as u8, '6' as i32 as u8];
    let mut p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut pos: libc::c_int = 0;
    p = strchr(doxn.as_ptr(), *str_A.offset(0) as libc::c_int);
    if p.is_null() {
      crate::libbb::verror_msg::bb_error_msg_and_die(
        b"bad output address radix \'%c\' (must be [doxn])\x00" as *const u8 as *const libc::c_char,
        *str_A.offset(0) as libc::c_int,
      );
    }
    pos = p.wrapping_offset_from(doxn.as_ptr()) as libc::c_long as libc::c_int;
    if pos == 3i32 {
      let ref mut fresh24 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).format_address;
      *fresh24 = Some(format_address_none as unsafe fn(_: off_t, _: libc::c_char) -> ())
    }
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).address_fmt[(::std::mem::size_of::<
      [libc::c_char; 7],
    >() as libc::c_ulong)
      .wrapping_sub(3i32 as libc::c_ulong)
      as usize] = doxn_address_base_char[pos as usize];
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).address_fmt[2] =
      doxn_address_pad_len_char[pos as usize] as libc::c_char
  }
  if opt & OPT_N as libc::c_int as libc::c_uint != 0 {
    max_bytes_to_format =
      crate::libbb::xatonum::xstrtoull_sfx(str_N, 0, bkm_suffixes.as_ptr()) as off_t
  }
  if opt & OPT_a as libc::c_int as libc::c_uint != 0 {
    decode_format_string(b"a\x00" as *const u8 as *const libc::c_char);
  }
  if opt & OPT_b as libc::c_int as libc::c_uint != 0 {
    decode_format_string(b"oC\x00" as *const u8 as *const libc::c_char);
  }
  if opt & OPT_c as libc::c_int as libc::c_uint != 0 {
    decode_format_string(b"c\x00" as *const u8 as *const libc::c_char);
  }
  if opt & OPT_d as libc::c_int as libc::c_uint != 0 {
    decode_format_string(b"u2\x00" as *const u8 as *const libc::c_char);
  }
  if opt & OPT_f as libc::c_int as libc::c_uint != 0 {
    decode_format_string(b"fF\x00" as *const u8 as *const libc::c_char);
  }
  if opt & OPT_h as libc::c_int as libc::c_uint != 0 {
    decode_format_string(b"x2\x00" as *const u8 as *const libc::c_char);
  }
  if opt & OPT_i as libc::c_int as libc::c_uint != 0 {
    decode_format_string(b"d2\x00" as *const u8 as *const libc::c_char);
  }
  if opt & OPT_j as libc::c_int as libc::c_uint != 0 {
    n_bytes_to_skip = crate::libbb::xatonum::xstrtoull_sfx(str_j, 0, bkm_suffixes.as_ptr()) as off_t
  }
  if opt & OPT_l as libc::c_int as libc::c_uint != 0 {
    decode_format_string(b"d4\x00" as *const u8 as *const libc::c_char);
  }
  if opt & OPT_o as libc::c_int as libc::c_uint != 0 {
    decode_format_string(b"o2\x00" as *const u8 as *const libc::c_char);
  }
  while !lst_t.is_null() {
    decode_format_string(crate::libbb::llist::llist_pop(&mut lst_t) as *const libc::c_char);
  }
  if opt & OPT_x as libc::c_int as libc::c_uint != 0 {
    decode_format_string(b"x2\x00" as *const u8 as *const libc::c_char);
  }
  if opt & OPT_s as libc::c_int as libc::c_uint != 0 {
    decode_format_string(b"d2\x00" as *const u8 as *const libc::c_char);
  }
  if opt & OPT_S as libc::c_int as libc::c_uint != 0 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).string_min =
      crate::libbb::xatonum::xstrtou_sfx(str_S, 0, bkm_suffixes.as_ptr())
  }
  // Bloat:
  //if ((option_mask32 & OPT_S) && G.n_specs > 0)
  //	bb_error_msg_and_die("no type may be specified when dumping strings");
  /* If the --traditional option is used, there may be from
   * 0 to 3 remaining command line arguments;  handle each case
   * separately.
   * od [FILE] [[+]OFFSET[.][b] [[+]LABEL[.][b]]]
   * The offset and pseudo_start have the same syntax.
   *
   * FIXME: POSIX 1003.1-2001 with XSI requires support for the
   * traditional syntax even if --traditional is not given.  */
  if opt & OPT_traditional as libc::c_int as libc::c_uint != 0 {
    if !(*argv.offset(0)).is_null() {
      let mut pseudo_start: off_t = -1i32 as off_t;
      let mut o1: off_t = 0;
      let mut o2: off_t = 0;
      if (*argv.offset(1)).is_null() {
        /* one arg */
        if parse_old_offset(*argv.offset(0), &mut o1) != 0 {
          /* od --traditional OFFSET */
          n_bytes_to_skip = o1;
          argv = argv.offset(1)
        }
      /* od --traditional FILE */
      } else if (*argv.offset(2)).is_null() {
        /* two args */
        if parse_old_offset(*argv.offset(0), &mut o1) != 0
          && parse_old_offset(*argv.offset(1), &mut o2) != 0
        {
          /* od --traditional OFFSET LABEL */
          n_bytes_to_skip = o1;
          pseudo_start = o2;
          argv = argv.offset(2)
        } else if parse_old_offset(*argv.offset(1), &mut o2) != 0 {
          /* od --traditional FILE OFFSET */
          n_bytes_to_skip = o2; /* >3 args */
          let ref mut fresh25 = *argv.offset(1);
          *fresh25 = std::ptr::null_mut::<libc::c_char>()
        } else {
          crate::libbb::verror_msg::bb_error_msg_and_die(
            b"invalid second argument \'%s\'\x00" as *const u8 as *const libc::c_char,
            *argv.offset(1),
          );
        }
      } else if (*argv.offset(3)).is_null() {
        /* three args */
        if parse_old_offset(*argv.offset(1), &mut o1) != 0
          && parse_old_offset(*argv.offset(2), &mut o2) != 0
        {
          /* od --traditional FILE OFFSET LABEL */
          n_bytes_to_skip = o1;
          pseudo_start = o2;
          let ref mut fresh26 = *argv.offset(1);
          *fresh26 = std::ptr::null_mut::<libc::c_char>()
        } else {
          crate::libbb::verror_msg::bb_simple_error_msg_and_die(
            b"the last two arguments must be offsets\x00" as *const u8 as *const libc::c_char,
          );
        }
      } else {
        crate::libbb::verror_msg::bb_simple_error_msg_and_die(
          b"too many arguments\x00" as *const u8 as *const libc::c_char,
        );
      }
      if pseudo_start >= 0 {
        if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).format_address
          == Some(format_address_none as unsafe fn(_: off_t, _: libc::c_char) -> ())
        {
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).address_fmt[(::std::mem::size_of::<
            [libc::c_char; 7],
          >()
            as libc::c_ulong)
            .wrapping_sub(3i32 as libc::c_ulong)
            as usize] = 'o' as i32 as libc::c_char;
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).address_fmt[2] =
            '7' as i32 as libc::c_char;
          let ref mut fresh27 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).format_address;
          *fresh27 = Some(format_address_paren as unsafe fn(_: off_t, _: libc::c_char) -> ())
        } else {
          let ref mut fresh28 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).format_address;
          *fresh28 = Some(format_address_label as unsafe fn(_: off_t, _: libc::c_char) -> ())
        }
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pseudo_offset =
          pseudo_start - n_bytes_to_skip
      }
    }
    /* else: od --traditional (without args) */
  }
  if option_mask32 & OPT_N as libc::c_int as libc::c_uint != 0 {
    end_offset = n_bytes_to_skip + max_bytes_to_format;
    if end_offset < n_bytes_to_skip {
      crate::libbb::verror_msg::bb_simple_error_msg_and_die(
        b"SKIP + SIZE is too large\x00" as *const u8 as *const libc::c_char,
      );
    }
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).n_specs == 0 as libc::c_uint {
    decode_format_string(b"o2\x00" as *const u8 as *const libc::c_char);
    /*G.n_specs = 1; - done by decode_format_string */
  }
  /* If no files were listed on the command line,
  set the global pointer FILE_LIST so that it
  references the null-terminated list of one name: "-".  */
  let ref mut fresh29 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).file_list;
  *fresh29 = bb_argv_dash.as_ptr();
  if !(*argv.offset(0)).is_null() {
    /* Set the global pointer FILE_LIST so that it
    references the first file-argument on the command-line.  */
    let ref mut fresh30 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).file_list;
    *fresh30 = argv as *const *const libc::c_char
  }
  /* Open the first input file */
  open_next_file();
  /* Skip over any unwanted header bytes */
  skip(n_bytes_to_skip);
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .in_stream
    .is_null()
  {
    return 1i32;
  }
  /* Compute output block length */
  l_c_m = get_lcm();
  if opt & OPT_w as libc::c_int as libc::c_uint != 0 {
    /* -w: width */
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).bytes_per_block == 0
      || (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .bytes_per_block
        .wrapping_rem(l_c_m as libc::c_uint)
        != 0 as libc::c_uint
    {
      crate::libbb::verror_msg::bb_error_msg(
        b"warning: invalid width %u; using %d instead\x00" as *const u8 as *const libc::c_char,
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).bytes_per_block,
        l_c_m,
      );
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).bytes_per_block = l_c_m as libc::c_uint
    }
  } else {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).bytes_per_block = l_c_m as libc::c_uint;
    if l_c_m < 16i32 {
      let ref mut fresh31 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).bytes_per_block;
      *fresh31 = (*fresh31).wrapping_mul((16i32 / l_c_m) as libc::c_uint)
    }
  }
  if option_mask32 & OPT_S as libc::c_int as libc::c_uint != 0 {
    dump_strings(n_bytes_to_skip, end_offset);
  } else {
    dump(n_bytes_to_skip, end_offset);
  }
  if fclose(stdin) != 0 {
    crate::libbb::perror_msg::bb_simple_perror_msg_and_die(bb_msg_standard_input.as_ptr());
  }
  return (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).exit_code as libc::c_int;
}
/*-
 * Copyright (c) 1990 The Regents of the University of California.
 * All rights reserved.
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
/* ENABLE_DESKTOP */
/* This one provides -t (busybox's own build script needs it) */
//usage:#if !ENABLE_DESKTOP
//usage:#define od_trivial_usage
//usage:       "[-aBbcDdeFfHhIiLlOovXx] [FILE]"
//usage:#define od_full_usage "\n\n"
//usage:       "Print FILE (or stdin) unambiguously, as octal bytes by default"
//usage:#endif
//kbuild:lib-$(CONFIG_OD) += od.o
//applet:IF_OD(APPLET(od, BB_DIR_USR_BIN, SUID_DROP))

/*
 * od implementation for busybox
 * Based on code from util-linux v 2.11l
 *
 * Copyright (c) 1990
 * The Regents of the University of California.  All rights reserved.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 *
 * Original copyright notice is retained at the end of this file.
 */
//config:config OD
//config:	bool "od (11 kb)"
//config:	default y
//config:	help
//config:	od is used to dump binary files in octal and other formats.
