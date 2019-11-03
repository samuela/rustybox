use libc;
extern "C" {
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_show_usage() -> !;
  /* Returns:
   * 0: no UUID= or LABEL= prefix found
   * 1: UUID= or LABEL= prefix found. In this case,
   *    *fsname is replaced if device with such UUID or LABEL is found
   */
  #[no_mangle]
  fn resolve_mount_spec(fsname: *mut *mut libc::c_char) -> libc::c_int;
}
/* vi: set sw=4 ts=4: */
/*
 * Support functions for mounting devices by label/uuid
 *
 * Copyright (C) 2006 by Jason Schoon <floydpink@gmail.com>
 * Some portions cribbed from e2fsprogs, util-linux, dosfstools
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config FINDFS
//config:	bool "findfs (12 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	select VOLUMEID
//config:	help
//config:	Prints the name of a filesystem with given label or UUID.
/* Benefits from suid root: better access to /dev/BLOCKDEVs: */
//applet:IF_FINDFS(APPLET(findfs, BB_DIR_SBIN, BB_SUID_MAYBE))
//kbuild:lib-$(CONFIG_FINDFS) += findfs.o
//usage:#define findfs_trivial_usage
//usage:       "LABEL=label or UUID=uuid"
//usage:#define findfs_full_usage "\n\n"
//usage:       "Find a filesystem device based on a label or UUID"
//usage:
//usage:#define findfs_example_usage
//usage:       "$ findfs LABEL=MyDevice"
#[no_mangle]
pub unsafe extern "C" fn findfs_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  argv = argv.offset(1);
  let mut dev: *mut libc::c_char = *argv;
  if dev.is_null() {
    bb_show_usage();
  }
  if !is_prefixed_with(dev, b"/dev/\x00" as *const u8 as *const libc::c_char).is_null() {
    /* Just pass any /dev/xxx name right through.
     * This might aid in some scripts being able
     * to call this unconditionally */
    dev = 0 as *mut libc::c_char
  } else if resolve_mount_spec(argv) == 0 {
    bb_show_usage();
  }
  if *argv != dev {
    puts(*argv);
    return 0i32;
  }
  return 1i32;
}
/* Otherwise, handle LABEL=xxx and UUID=xxx,
 * fail on anything else */
