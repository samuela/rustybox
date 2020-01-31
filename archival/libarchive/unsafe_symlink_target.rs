use crate::libbb::llist::llist_t;
use crate::librb::size_t;
use libc;
use libc::strstr;
use libc::symlink;
extern "C" {
  #[no_mangle]
  fn link(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

}

/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
pub unsafe fn create_or_remember_link(
  mut link_placeholders: *mut *mut llist_t,
  mut target: *const libc::c_char,
  mut linkname: *const libc::c_char,
  mut hard_link: libc::c_int,
) {
  if hard_link != 0
    || *target.offset(0) as libc::c_int == '/' as i32
    || !strstr(target, b"..\x00" as *const u8 as *const libc::c_char).is_null()
  {
    crate::libbb::llist::llist_add_to_end(
      link_placeholders,
      crate::libbb::xfuncs_printf::xasprintf(
        b"%c%s%c%s\x00" as *const u8 as *const libc::c_char,
        hard_link,
        linkname,
        '\u{0}' as i32,
        target,
      ) as *mut libc::c_void,
    );
    return;
  }
  if symlink(target, linkname) != 0 {
    /* shared message */
    crate::libbb::perror_msg::bb_perror_msg_and_die(
      b"can\'t create %slink \'%s\' to \'%s\'\x00" as *const u8 as *const libc::c_char,
      b"sym\x00" as *const u8 as *const libc::c_char,
      linkname,
      target,
    );
  };
}
pub unsafe fn create_links_from_list(mut list: *mut llist_t) {
  while !list.is_null() {
    let mut target: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    target = (*list)
      .data
      .offset(1)
      .offset(strlen((*list).data.offset(1)) as isize)
      .offset(1);
    if (if *(*list).data as libc::c_int != 0 {
      link
    } else {
      symlink
    })(target, (*list).data.offset(1))
      != 0
    {
      /* shared message */
      crate::libbb::verror_msg::bb_error_msg_and_die(
        b"can\'t create %slink \'%s\' to \'%s\'\x00" as *const u8 as *const libc::c_char,
        if *(*list).data as libc::c_int != 0 {
          b"hard\x00" as *const u8 as *const libc::c_char
        } else {
          b"sym\x00" as *const u8 as *const libc::c_char
        },
        (*list).data.offset(1),
        target,
      );
    }
    list = (*list).link
  }
}
