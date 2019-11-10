use libc;
use libc::chdir;
use libc::chmod;
use libc::chown;
use libc::closelog;
use libc::dup2;
use libc::fstat;
use libc::getenv;
use libc::geteuid;
use libc::getopt;
use libc::getpid;
use libc::isatty;
use libc::kill;
use libc::openlog;
use libc::sigaddset;
use libc::sigemptyset;
use libc::sigprocmask;
use libc::sleep;
use libc::sscanf;
use libc::strcasecmp;
use libc::strcpy;
use libc::symlink;
use libc::syscall;
use libc::syslog;
use libc::time;
use libc::access;
use libc::atoi;
use libc::fclose;
use libc::fprintf;
use libc::lstat;
use libc::printf;
use libc::puts;
use libc::rename;
use libc::rmdir;
use libc::sprintf;
use libc::strchr;
use libc::strcmp;
use libc::strrchr;
use libc::strstr;
use libc::system;




extern "C" {
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn xatou_range(str: *const libc::c_char, l: libc::c_uint, u: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn bb_xioctl(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
  pub fn_0: libc::c_char,
  pub subarg: libc::c_char,
}

/*
 * setlogcons: Send kernel messages to the current console or to console N
 *
 * Copyright (C) 2006 by Jan Kiszka <jan.kiszka@web.de>
 *
 * Based on setlogcons (kbd-1.12) by Andries E. Brouwer
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config SETLOGCONS
//config:	bool "setlogcons (1.8 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	This program redirects the output console of kernel messages.
//applet:IF_SETLOGCONS(APPLET_NOEXEC(setlogcons, setlogcons, BB_DIR_USR_SBIN, BB_SUID_DROP, setlogcons))
//kbuild:lib-$(CONFIG_SETLOGCONS) += setlogcons.o
//usage:#define setlogcons_trivial_usage
//usage:       "[N]"
//usage:#define setlogcons_full_usage "\n\n"
//usage:       "Pin kernel output to VT console N. Default:0 (do not pin)"
// Comment from kernel source:
/* ...
 * By default, the kernel messages are always printed on the current virtual
 * console. However, the user may modify that default with the
 * TIOCL_SETKMSGREDIRECT ioctl call.
 *
 * This function sets the kernel message console to be @new. It returns the old
 * virtual console number. The virtual terminal number 0 (both as parameter and
 * return value) means no redirection (i.e. always printed on the currently
 * active console).
 */
#[no_mangle]
pub unsafe extern "C" fn setlogcons_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut devname: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut arg: C2RustUnnamed = {
    let mut init = C2RustUnnamed {
      fn_0: 11i32 as libc::c_char,
      subarg: 0i32 as libc::c_char,
    };
    init
  };
  if !(*argv.offset(1)).is_null() {
    arg.subarg =
      xatou_range(*argv.offset(1), 0i32 as libc::c_uint, 63i32 as libc::c_uint) as libc::c_char
  }
  /* Can just call it on "/dev/tty1" always, but...
   * in my testing, inactive (never opened) VTs are not
   * redirected to, despite ioctl not failing.
   *
   * By using "/dev/ttyN", ensure it is activated.
   */
  devname = xasprintf(
    b"/dev/tty%u\x00" as *const u8 as *const libc::c_char,
    arg.subarg as libc::c_int,
  );
  bb_xioctl(
    xopen(devname, 0i32),
    0x541ci32 as libc::c_uint,
    &mut arg as *mut C2RustUnnamed as *mut libc::c_void,
    b"TIOCLINUX\x00" as *const u8 as *const libc::c_char,
  );
  return 0i32;
}
