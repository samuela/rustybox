use libc;
extern "C" {
  #[no_mangle]
  fn rmdir(__path: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn dirname(__path: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> uint32_t;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
}
use crate::librb::__uint32_t;
use crate::librb::uint32_t;
#[no_mangle]
pub unsafe extern "C" fn rmdir_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut status: libc::c_int = 0i32; /* Match gnu rmdir msg. */
  let mut flags: libc::c_int = 0;
  let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
  flags = getopt32long(
    argv,
    b"pv\x00" as *const u8 as *const libc::c_char,
    b"parents\x00\x00pignore-fail-on-non-empty\x00\x00\xffverbose\x00\x00v\x00" as *const u8
      as *const libc::c_char,
  ) as libc::c_int;
  argv = argv.offset(optind as isize);
  if (*argv).is_null() {
    bb_show_usage();
  }
  loop {
    path = *argv;
    loop {
      if flags & (1i32 << 1i32) * 1i32 != 0 {
        printf(
          b"rmdir: removing directory, \'%s\'\n\x00" as *const u8 as *const libc::c_char,
          path,
        );
      }
      if rmdir(path) < 0i32 {
        if flags & (1i32 << 2i32) * 1i32 != 0 && *bb_errno == 39i32 {
          break;
        }
        bb_perror_msg(b"\'%s\'\x00" as *const u8 as *const libc::c_char, path);
        status = 1i32;
        break;
      } else {
        if !(flags & 1i32 << 0i32 != 0) {
          break;
        }
        /* Note: path was not "" since rmdir succeeded. */
        path = dirname(path);
        /* Path is now just the parent component.  Dirname
         * returns "." if there are no parents.
         */
        if !(*path.offset(0) as libc::c_int != '.' as i32 || *path.offset(1) as libc::c_int != 0) {
          break;
        }
      }
    }
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  return status;
}
