use libc;
extern "C" {
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

/*
 * Mini dumpkmap implementation for busybox
 *
 * Copyright (C) Arne Bernin <arne@matrix.loopback.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config DUMPKMAP
//config:	bool "dumpkmap (1.6 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	This program dumps the kernel's keyboard translation table to
//config:	stdout, in binary format. You can then use loadkmap to load it.
//applet:IF_DUMPKMAP(APPLET_NOEXEC(dumpkmap, dumpkmap, BB_DIR_BIN, SUID_DROP, dumpkmap))
/* bb_common_bufsiz1 usage here is safe wrt NOEXEC: not expecting it to be zeroed. */
//kbuild:lib-$(CONFIG_DUMPKMAP) += dumpkmap.o
//usage:#define dumpkmap_trivial_usage
//usage:       "> keymap"
//usage:#define dumpkmap_full_usage "\n\n"
//usage:       "Print a binary keyboard translation table to stdout"
//usage:
//usage:#define dumpkmap_example_usage
//usage:       "$ dumpkmap > keymap\n"
/* From <linux/kd.h> */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kbentry {
  pub kb_table: libc::c_uchar,
  pub kb_index: libc::c_uchar,
  pub kb_value: libc::c_ushort,
}
pub unsafe fn dumpkmap_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut ke: kbentry = kbentry {
    kb_table: 0,
    kb_index: 0,
    kb_value: 0,
  };
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  let mut fd: libc::c_int = 0;
  /* When user accidentally runs "dumpkmap FILE"
   * instead of "dumpkmap >FILE", we'd dump binary stuff to tty.
   * Let's prevent it:
   */
  if !(*argv.offset(1)).is_null() {
    crate::libbb::appletlib::bb_show_usage();
  }
  /*	bb_warn_ignoring_args(argv[1]);*/
  fd = crate::libbb::get_console::get_console_fd_or_die();
  /*                     0 1 2 3 4 5 6 7 8 9 a b c=12 */
  memcpy(
    bb_common_bufsiz1.as_mut_ptr() as *mut libc::c_void,
    b"bkeymap\x01\x01\x01\x00\x01\x01\x01\x00\x01\x01\x01\x00\x01\x00" as *const u8
      as *const libc::c_char as *const libc::c_void,
    ::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong,
  );
  libc::write(
    1i32,
    bb_common_bufsiz1.as_mut_ptr() as *const libc::c_void,
    7 + 256,
  );
  i = 0;
  while i < 13i32 {
    if *bb_common_bufsiz1.as_mut_ptr().offset(7).offset(i as isize) != 0 {
      j = 0;
      while j < 128i32 {
        ke.kb_index = j as libc::c_uchar;
        ke.kb_table = i as libc::c_uchar;
        if crate::libbb::xfuncs_printf::ioctl_or_perror(
          fd,
          0x4b46i32 as libc::c_uint,
          &mut ke as *mut kbentry as *mut libc::c_void,
          b"ioctl(KDGKBENT{%d,%d}) failed\x00" as *const u8 as *const libc::c_char,
          j,
          i,
        ) == 0
        {
          libc::write(
            1,
            &mut ke.kb_value as *mut libc::c_ushort as *const libc::c_void,
            2,
          );
        }
        j += 1
      }
    }
    i += 1
  }
  return 0;
}
