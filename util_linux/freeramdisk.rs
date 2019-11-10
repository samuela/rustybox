use libc;
use libc::putenv;
use libc::umask;
use libc::mknod;
use libc::fchmod;
use libc::fscanf;
use libc::alarm;
use libc::sync;
use libc::setsid;
use libc::ioctl;
use libc::statfs;
use libc::mount;
use libc::prctl;
use libc::opendir;
use libc::closedir;
use libc::readdir;
use libc::strtok;
use libc::putchar_unlocked;
use libc::endmntent;
use libc::setmntent;
use libc::umount2;
use libc::getegid;
use libc::getuid;
use libc::getgid;
use libc::setutxent;
use libc::endutxent;











































extern "C" {

  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn single_argv(argv: *mut *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn ioctl_or_perror_and_die(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    fmt: *const libc::c_char,
    _: ...
  ) -> libc::c_int;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn freeramdisk_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut fd: libc::c_int = 0;
  fd = xopen(single_argv(argv), 0o2i32);
  // Act like freeramdisk, fdflush, or both depending on configuration.
  ioctl_or_perror_and_die(
    fd,
    if 1i32 != 0 && *applet_name.offset(1) as libc::c_int == 'r' as i32 || 1i32 == 0 {
      (0u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (0x12i32 << 0i32 + 8i32) as libc::c_uint
        | (97i32 << 0i32) as libc::c_uint)
        | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint
    } else {
      (0u32 << 0i32 + 8i32 + 8i32 + 14i32
        | (2i32 << 0i32 + 8i32) as libc::c_uint
        | (0x4bi32 << 0i32) as libc::c_uint)
        | (0i32 << 0i32 + 8i32 + 8i32) as libc::c_uint
    },
    0 as *mut libc::c_void,
    b"%s\x00" as *const u8 as *const libc::c_char,
    *argv.offset(1),
  );
  return 0i32;
}
