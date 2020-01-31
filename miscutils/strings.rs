use libc;
use libc::printf;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static bb_msg_standard_input: [libc::c_char; 0];
  #[no_mangle]
  static mut stdout: *mut FILE;

  #[no_mangle]
  fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  static mut option_mask32: u32;

}

use crate::librb::size_t;
use libc::off_t;
use libc::FILE;
pub unsafe fn strings_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut n: libc::c_int = 0;
  let mut c: libc::c_int = 0;
  let mut status: libc::c_int = 0;
  let mut count: libc::c_uint = 0;
  let mut offset: off_t = 0;
  let mut file: *mut FILE = std::ptr::null_mut();
  let mut string: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut fmt: *const libc::c_char = b"%s: \x00" as *const u8 as *const libc::c_char;
  let mut n_arg: *const libc::c_char = b"4\x00" as *const u8 as *const libc::c_char;
  /* default for -o */
  let mut radix: *const libc::c_char = b"o\x00" as *const u8 as *const libc::c_char;
  let mut radix_fmt: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  crate::libbb::getopt32::getopt32(
    argv,
    b"afon:t:\x00" as *const u8 as *const libc::c_char,
    &mut n_arg as *mut *const libc::c_char,
    &mut radix as *mut *const libc::c_char,
  );
  /* -a is our default behaviour */
  /*argc -= optind;*/
  argv = argv.offset(optind as isize);
  n = crate::libbb::xatonum::xatou_range(n_arg, 1i32 as libc::c_uint, 2147483647i32 as libc::c_uint)
    as libc::c_int;
  string = crate::libbb::xfuncs_printf::xzalloc((n + 1i32) as size_t) as *mut libc::c_char;
  n -= 1;
  if *radix.offset(0) as libc::c_int != 'd' as i32
    && *radix.offset(0) as libc::c_int != 'o' as i32
    && *radix.offset(0) as libc::c_int != 'x' as i32
    || *radix.offset(1) as libc::c_int != 0
  {
    crate::libbb::appletlib::bb_show_usage();
  }
  radix_fmt = crate::libbb::xfuncs_printf::xasprintf(
    b"%%7l%s \x00" as *const u8 as *const libc::c_char,
    radix,
  );
  if (*argv).is_null() {
    fmt = b"{%s}: \x00" as *const u8 as *const libc::c_char;
    argv = argv.offset(-1);
    *argv = bb_msg_standard_input.as_ptr() as *mut libc::c_char
  }
  loop {
    file = crate::libbb::wfopen_input::fopen_or_warn_stdin(*argv);
    if file.is_null() {
      status = 1i32
    } else {
      offset = 0 as off_t;
      count = 0 as libc::c_uint;
      loop {
        c = getc_unlocked(file);
        if (c - 0x20i32) as libc::c_uint <= (0x7ei32 - 0x20i32) as libc::c_uint || c == '\t' as i32
        {
          if count > n as libc::c_uint {
            crate::libbb::xfuncs_printf::bb_putchar(c);
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
            crate::libbb::xfuncs_printf::bb_putchar('\n' as i32);
          }
          count = 0 as libc::c_uint
        }
        offset += 1;
        if !(c != -1i32) {
          break;
        }
      }
      crate::libbb::fclose_nonstdin::fclose_if_not_stdin(file);
    }
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  crate::libbb::fflush_stdout_and_exit::fflush_stdout_and_exit(status);
}
