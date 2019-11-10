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
  fn bb_show_usage() -> !;
}

#[no_mangle]
pub unsafe extern "C" fn single_argv(mut argv: *mut *mut libc::c_char) -> *mut libc::c_char {
  if !(*argv.offset(1)).is_null()
    && strcmp(
      *argv.offset(1),
      b"--\x00" as *const u8 as *const libc::c_char,
    ) == 0
  {
    argv = argv.offset(1)
  }
  if (*argv.offset(1)).is_null() || !(*argv.offset(2)).is_null() {
    bb_show_usage();
  }
  return *argv.offset(1);
}
