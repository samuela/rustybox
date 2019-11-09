use libc;
use libc::free;
extern "C" {
  #[no_mangle]
  fn xchroot(path: *const libc::c_char);
  #[no_mangle]
  fn BB_EXECVP_or_die(argv: *mut *mut libc::c_char) -> !;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn get_shell_name() -> *const libc::c_char;
}

/*
 * Mini chroot implementation for busybox
 *
 * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config CHROOT
//config:	bool "chroot (3.7 kb)"
//config:	default y
//config:	help
//config:	chroot is used to change the root directory and run a command.
//config:	The default command is '/bin/sh'.
//applet:IF_CHROOT(APPLET_NOEXEC(chroot, chroot, BB_DIR_USR_SBIN, BB_SUID_DROP, chroot))
//kbuild:lib-$(CONFIG_CHROOT) += chroot.o
/* BB_AUDIT SUSv3 N/A -- Matches GNU behavior. */
//usage:#define chroot_trivial_usage
//usage:       "NEWROOT [PROG ARGS]"
//usage:#define chroot_full_usage "\n\n"
//usage:       "Run PROG with root directory set to NEWROOT"
//usage:
//usage:#define chroot_example_usage
//usage:       "$ ls -l /bin/ls\n"
//usage:       "lrwxrwxrwx    1 root     root          12 Apr 13 00:46 /bin/ls -> /BusyBox\n"
//usage:       "# mount /dev/hdc1 /mnt -t minix\n"
//usage:       "# chroot /mnt\n"
//usage:       "# ls -l /bin/ls\n"
//usage:       "-rwxr-xr-x    1 root     root        40816 Feb  5 07:45 /bin/ls*\n"
#[no_mangle]
pub unsafe extern "C" fn chroot_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  argv = argv.offset(1);
  if (*argv).is_null() {
    bb_show_usage();
  }
  xchroot(*argv);
  argv = argv.offset(1);
  if (*argv).is_null() {
    /* no 2nd param (PROG), use shell */
    argv = argv.offset(-2);
    let ref mut fresh0 = *argv.offset(0);
    *fresh0 = get_shell_name() as *mut libc::c_char;
    let ref mut fresh1 = *argv.offset(1);
    *fresh1 = b"-i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    /* GNU coreutils 8.4 compat */
    /*argv[2] = NULL; - already is */
  }
  BB_EXECVP_or_die(argv);
}
