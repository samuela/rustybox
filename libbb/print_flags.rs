/* Print string that matches bit masked flags */
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





























use libc::printf;














extern "C" {


  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> libc::c_ulong;
}

// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct masks_labels_t {
// 	pub labels: *const libc::c_char,
// 	pub masks: [libc::c_int; 0],
// }

/* returns a set with the flags not printed */
#[no_mangle]
pub unsafe extern "C" fn print_flags_separated(
  mut masks: *const libc::c_int,
  mut labels: *const libc::c_char,
  mut flags: libc::c_int,
  mut separator: *const libc::c_char,
) -> libc::c_int {
  let mut need_separator: *const libc::c_char = 0 as *const libc::c_char;
  while *labels != 0 {
    if flags & *masks != 0 {
      printf(
        b"%s%s\x00" as *const u8 as *const libc::c_char,
        if !need_separator.is_null() {
          need_separator
        } else {
          b"\x00" as *const u8 as *const libc::c_char
        },
        labels,
      );
      need_separator = separator;
      flags &= !*masks
    }
    masks = masks.offset(1);
    labels = labels.offset(strlen(labels).wrapping_add(1i32 as libc::c_ulong) as isize)
  }
  return flags;
}

// #[no_mangle]
// pub unsafe extern "C" fn print_flags(
// 	mut ml: *const masks_labels_t,
// 	mut flags: libc::c_int,
// ) -> libc::c_int {
// 	return print_flags_separated(
// 		(*ml).masks.as_ptr(),
// 		(*ml).labels,
// 		flags,
// 		0 as *const libc::c_char,
// 	);
// }
