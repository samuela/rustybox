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
















use libc::sleep;






use libc::time;




















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
