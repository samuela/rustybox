use crate::libbb::ptr_to_globals::bb_errno;
use libc;
use libc::printf;
use libc::rmdir;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn dirname(__path: *mut libc::c_char) -> *mut libc::c_char;

}

pub unsafe fn rmdir_main(mut _argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut status: libc::c_int = 0; /* Match gnu rmdir msg. */
  let mut flags: libc::c_int = 0;
  let mut path: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  flags = crate::libbb::getopt32::getopt32long(
    argv,
    b"pv\x00" as *const u8 as *const libc::c_char,
    b"parents\x00\x00pignore-fail-on-non-empty\x00\x00\xffverbose\x00\x00v\x00" as *const u8
      as *const libc::c_char,
  ) as libc::c_int;
  argv = argv.offset(optind as isize);
  if (*argv).is_null() {
    crate::libbb::appletlib::bb_show_usage();
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
      if rmdir(path) < 0 {
        if flags & (1i32 << 2i32) * 1i32 != 0 && *bb_errno == 39i32 {
          break;
        }
        crate::libbb::perror_msg::bb_perror_msg(
          b"\'%s\'\x00" as *const u8 as *const libc::c_char,
          path,
        );
        status = 1i32;
        break;
      } else {
        if !(flags & 1i32 << 0 != 0) {
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
