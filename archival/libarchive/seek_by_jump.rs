use libc;
use libc::chdir;
use libc::chmod;
use libc::chown;
use libc::closelog;
use libc::dup2;
use libc::fstat;
use libc::getenv;
use libc::geteuid;
use libc::getopt;
use libc::getpid;
use libc::isatty;
use libc::kill;
use libc::openlog;
use libc::sigaddset;
use libc::sigemptyset;
use libc::sigprocmask;
use libc::sleep;
use libc::sscanf;
use libc::strcasecmp;
use libc::strcpy;
use libc::symlink;
use libc::syscall;
use libc::syslog;
use libc::time;
use libc::access;
use libc::atoi;
use libc::fclose;
use libc::fprintf;
use libc::lstat;
use libc::printf;
use libc::puts;
use libc::rename;
use libc::rmdir;
use libc::sprintf;
use libc::strchr;
use libc::strcmp;
use libc::strrchr;
use libc::strstr;
use libc::system;





extern "C" {
  #[no_mangle]
  fn lseek(__fd: libc::c_int, __offset: off64_t, __whence: libc::c_int) -> off64_t;

  #[no_mangle]
  static bb_errno: *mut libc::c_int;

  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;

  #[no_mangle]
  fn seek_by_read(fd: libc::c_int, amount: off_t);
}

use libc::off64_t;
use libc::off_t;

#[no_mangle]
pub unsafe extern "C" fn seek_by_jump(mut fd: libc::c_int, mut amount: off_t) {
  if amount != 0 && lseek(fd, amount, 1i32) == -1i32 as off_t {
    if *bb_errno == 29i32 {
      seek_by_read(fd, amount);
    } else {
      bb_simple_perror_msg_and_die(b"seek failure\x00" as *const u8 as *const libc::c_char);
    }
  };
}
