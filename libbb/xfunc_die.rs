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
  static mut xfunc_error_retval: u8;
}



/* Keeping it separate allows to NOT pull in stdio for VERY small applets.
 * Try building busybox with only "true" enabled... */

#[no_mangle]
pub static mut die_func: Option<unsafe extern "C" fn() -> ()> = None;

#[no_mangle]
pub unsafe extern "C" fn xfunc_die() -> ! {
  if die_func.is_some() {
    die_func.expect("non-null function pointer")();
  }
  ::std::process::exit(xfunc_error_retval as libc::c_int);
}
