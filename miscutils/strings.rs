use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static bb_msg_standard_input: [libc::c_char; 0];
  #[no_mangle]
  static mut stdout: *mut FILE;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;
  #[no_mangle]
  fn fclose_if_not_stdin(file: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fopen_or_warn_stdin(filename: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn xatou_range(str: *const libc::c_char, l: libc::c_uint, u: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  static mut option_mask32: uint32_t;
  #[no_mangle]
  fn bb_show_usage() -> !;
}
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type off_t = __off64_t;

pub type _IO_lock_t = ();

use crate::librb::FILE;
#[no_mangle]
pub unsafe extern "C" fn strings_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut n: libc::c_int = 0;
  let mut c: libc::c_int = 0;
  let mut status: libc::c_int = 0i32;
  let mut count: libc::c_uint = 0;
  let mut offset: off_t = 0;
  let mut file: *mut FILE = 0 as *mut FILE;
  let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut fmt: *const libc::c_char = b"%s: \x00" as *const u8 as *const libc::c_char;
  let mut n_arg: *const libc::c_char = b"4\x00" as *const u8 as *const libc::c_char;
  /* default for -o */
  let mut radix: *const libc::c_char = b"o\x00" as *const u8 as *const libc::c_char;
  let mut radix_fmt: *mut libc::c_char = 0 as *mut libc::c_char;
  getopt32(
    argv,
    b"afon:t:\x00" as *const u8 as *const libc::c_char,
    &mut n_arg as *mut *const libc::c_char,
    &mut radix as *mut *const libc::c_char,
  );
  /* -a is our default behaviour */
  /*argc -= optind;*/
  argv = argv.offset(optind as isize);
  n = xatou_range(n_arg, 1i32 as libc::c_uint, 2147483647i32 as libc::c_uint) as libc::c_int;
  string = xzalloc((n + 1i32) as size_t) as *mut libc::c_char;
  n -= 1;
  if *radix.offset(0) as libc::c_int != 'd' as i32
    && *radix.offset(0) as libc::c_int != 'o' as i32
    && *radix.offset(0) as libc::c_int != 'x' as i32
    || *radix.offset(1) as libc::c_int != 0i32
  {
    bb_show_usage();
  }
  radix_fmt = xasprintf(b"%%7l%s \x00" as *const u8 as *const libc::c_char, radix);
  if (*argv).is_null() {
    fmt = b"{%s}: \x00" as *const u8 as *const libc::c_char;
    argv = argv.offset(-1);
    *argv = bb_msg_standard_input.as_ptr() as *mut libc::c_char
  }
  loop {
    file = fopen_or_warn_stdin(*argv);
    if file.is_null() {
      status = 1i32
    } else {
      offset = 0i32 as off_t;
      count = 0i32 as libc::c_uint;
      loop {
        c = getc_unlocked(file);
        if (c - 0x20i32) as libc::c_uint <= (0x7ei32 - 0x20i32) as libc::c_uint || c == '\t' as i32
        {
          if count > n as libc::c_uint {
            bb_putchar(c);
          } else {
            *string.offset(count as isize) = c as libc::c_char;
            if count == n as libc::c_uint {
              if option_mask32 & 2i32 as libc::c_uint != 0 {
                printf(fmt, *argv);
              }
              if option_mask32 & (4i32 | 16i32) as libc::c_uint != 0 {
                printf(radix_fmt, offset - n as libc::c_long);
              }
              fputs_unlocked(string, stdout);
            }
            count = count.wrapping_add(1)
          }
        } else {
          if count > n as libc::c_uint {
            bb_putchar('\n' as i32);
          }
          count = 0i32 as libc::c_uint
        }
        offset += 1;
        if !(c != -1i32) {
          break;
        }
      }
      fclose_if_not_stdin(file);
    }
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  fflush_stdout_and_exit(status);
}
