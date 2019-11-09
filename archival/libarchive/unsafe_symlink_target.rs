use libc;
use libc::close;


extern "C" {
  #[no_mangle]
  fn link(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn symlink(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;

  #[no_mangle]
  fn llist_add_to_end(list_head: *mut *mut llist_t, data: *mut libc::c_void);

  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;

  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
}

use crate::libbb::llist::llist_t;
use crate::librb::size_t;

/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn create_or_remember_link(
  mut link_placeholders: *mut *mut llist_t,
  mut target: *const libc::c_char,
  mut linkname: *const libc::c_char,
  mut hard_link: libc::c_int,
) {
  if hard_link != 0
    || *target.offset(0) as libc::c_int == '/' as i32
    || !strstr(target, b"..\x00" as *const u8 as *const libc::c_char).is_null()
  {
    llist_add_to_end(
      link_placeholders,
      xasprintf(
        b"%c%s%c%s\x00" as *const u8 as *const libc::c_char,
        hard_link,
        linkname,
        '\u{0}' as i32,
        target,
      ) as *mut libc::c_void,
    );
    return;
  }
  if symlink(target, linkname) != 0i32 {
    /* shared message */
    bb_perror_msg_and_die(
      b"can\'t create %slink \'%s\' to \'%s\'\x00" as *const u8 as *const libc::c_char,
      b"sym\x00" as *const u8 as *const libc::c_char,
      linkname,
      target,
    );
  };
}
#[no_mangle]
pub unsafe extern "C" fn create_links_from_list(mut list: *mut llist_t) {
  while !list.is_null() {
    let mut target: *mut libc::c_char = 0 as *mut libc::c_char;
    target = (*list)
      .data
      .offset(1)
      .offset(strlen((*list).data.offset(1)) as isize)
      .offset(1);
    if if *(*list).data as libc::c_int != 0 {
      Some(
        link as unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int,
      )
    } else {
      Some(
        symlink
          as unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int,
      )
    }
    .expect("non-null function pointer")(target, (*list).data.offset(1))
      != 0
    {
      /* shared message */
      bb_error_msg_and_die(
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
