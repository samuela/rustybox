/* Print string that matches bit masked flags */

use crate::libbb::appletlib::applet_name;
use libc;
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
