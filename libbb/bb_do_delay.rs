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





use libc::time_t;

extern "C" {



}

#[no_mangle]
pub unsafe extern "C" fn bb_do_delay(mut seconds: libc::c_int) {
  let mut start: time_t = 0;
  let mut now: time_t = 0;
  start = time(0 as *mut time_t);
  loop {
    sleep(seconds as libc::c_uint);
    now = time(0 as *mut time_t);
    if !(now - start < seconds as libc::c_long) {
      break;
    }
  }
}
