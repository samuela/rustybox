use crate::libbb::ptr_to_globals::bb_errno;
use libc;
use libc::open;
extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn ftruncate(__fd: libc::c_int, __length: off_t) -> libc::c_int;

  /* Close fd, but check for failures (some types of write errors) */

  #[no_mangle]
  static cwbkMG_suffixes: [suffix_mult; 0];

}

use libc::off_t;
/* Last element is marked by mult == 0 */

use crate::librb::suffix_mult;

pub const OPT_NOCREATE: C2RustUnnamed = 1;
pub type C2RustUnnamed = libc::c_uint;
pub const OPT_SIZE: C2RustUnnamed = 2;
/* This is a NOFORK applet. Be very careful! */

pub unsafe fn truncate_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opts: libc::c_uint = 0;
  let mut flags: libc::c_int = 0o1i32 | 0o4000i32;
  let mut ret: libc::c_int = 0;
  let mut size_str: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut size: off_t = 0;
  opts = crate::libbb::getopt32::getopt32(
    argv,
    b"^cs:\x00s:-1\x00" as *const u8 as *const libc::c_char,
    &mut size_str as *mut *mut libc::c_char,
  );
  if opts & OPT_NOCREATE as libc::c_int as libc::c_uint == 0 {
    flags |= 0o100i32
  }
  // TODO: coreutils 8.17 also support "m" (lowercase) suffix
  // with truncate, but not with dd!
  // We share kMG_suffixes[], so we can't make both tools
  // compatible at once...
  size = crate::libbb::xatonum::xatoull_sfx(size_str, cwbkMG_suffixes.as_ptr().offset(3)) as off_t;
  argv = argv.offset(optind as isize);
  while !(*argv).is_null() {
    let mut fd: libc::c_int = open(*argv, flags, 0o666i32);
    if fd < 0 {
      if *bb_errno != 2i32 || opts & OPT_NOCREATE as libc::c_int as libc::c_uint == 0 {
        crate::libbb::perror_msg::bb_perror_msg(
          b"%s: open\x00" as *const u8 as *const libc::c_char,
          *argv,
        );
        ret = 1i32
      }
    /* else: ENOENT && OPT_NOCREATE:
     * do not report error, exitcode is also 0.
     */
    } else {
      if ftruncate(fd, size) == -1i32 {
        crate::libbb::perror_msg::bb_perror_msg(
          b"%s: truncate\x00" as *const u8 as *const libc::c_char,
          *argv,
        );
        ret = 1i32
      }
      crate::libbb::xfuncs_printf::xclose(fd);
    }
    argv = argv.offset(1)
  }
  return ret;
}
