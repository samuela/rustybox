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










































use libc::free;



extern "C" {

}

#[no_mangle]
pub unsafe extern "C" fn auto_string(mut str: *mut libc::c_char) -> *mut libc::c_char {
  static mut saved: [*mut libc::c_char; 4] = [0 as *const libc::c_char as *mut libc::c_char; 4]; /* = 0 */
  static mut cur_saved: u8 = 0;
  free(saved[cur_saved as usize] as *mut libc::c_void);
  saved[cur_saved as usize] = str;
  cur_saved = ((cur_saved as libc::c_int + 1i32) as libc::c_uint
    & ((::std::mem::size_of::<[*mut libc::c_char; 4]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
      as libc::c_uint)
      .wrapping_sub(1i32 as libc::c_uint)) as u8;
  return str;
}
