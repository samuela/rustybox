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
  fn get_console_fd_or_die() -> libc::c_int;

  #[no_mangle]
  fn console_make_active(fd: libc::c_int, vt_num: libc::c_int);

  #[no_mangle]
  fn xatou_range(str: *const libc::c_char, l: libc::c_uint, u: libc::c_uint) -> libc::c_uint;

  #[no_mangle]
  fn single_argv(argv: *mut *mut libc::c_char) -> *mut libc::c_char;
}

#[no_mangle]
pub unsafe extern "C" fn chvt_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut num: libc::c_int = xatou_range(single_argv(argv), 1, 63) as libc::c_int;
  console_make_active(get_console_fd_or_die(), num);
  return 0;
}
