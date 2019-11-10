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











































use libc::gid_t;

use crate::librb::size_t;

extern "C" {
  #[no_mangle]
  fn getgroups(__size: libc::c_int, __list: *mut gid_t) -> libc::c_int;

  #[no_mangle]
  static bb_errno: *mut libc::c_int;

  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
}

#[no_mangle]
pub unsafe extern "C" fn bb_getgroups(
  mut ngroups: *mut libc::c_int,
  mut group_array: *mut gid_t,
) -> *mut gid_t {
  let mut n: libc::c_int = if !ngroups.is_null() { *ngroups } else { 0i32 };
  /* getgroups may be a bit expensive, try to use it only once */
  if n < 32i32 {
    n = 32i32
  }
  loop {
    // FIXME: ash tries so hard to not die on OOM (when we are called from test),
    // and we spoil it with just one xrealloc here
    group_array = xrealloc(
      group_array as *mut libc::c_void,
      ((n + 1i32) as libc::c_ulong).wrapping_mul(::std::mem::size_of::<gid_t>() as libc::c_ulong),
    ) as *mut gid_t;
    n = getgroups(n, group_array);
    /*
     * If buffer is too small, kernel does not return new_n > n.
     * It returns -1 and EINVAL:
     */
    if n >= 0i32 {
      /* Terminator for bb_getgroups(NULL, NULL) usage */
      *group_array.offset(n as isize) = -1i32 as gid_t;
      break;
    } else if *bb_errno == 22i32 {
      /* too small? */
      /* This is the way to ask kernel how big the array is */
      n = getgroups(0i32, group_array)
    } else {
      /* Some other error (should never happen on Linux) */
      bb_simple_perror_msg_and_die(b"getgroups\x00" as *const u8 as *const libc::c_char);
    }
  }
  if !ngroups.is_null() {
    *ngroups = n
  }
  return group_array;
}
