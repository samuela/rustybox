use crate::libbb::ptr_to_globals::bb_errno;

use libc;
use libc::close;
use libc::ioctl;
use libc::open;
use libc::ptrdiff_t;
use libc::setsid;
use libc::sprintf;
extern "C" {

  #[no_mangle]
  fn vfork() -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vt_stat {
  pub v_active: libc::c_ushort,
  pub v_signal: libc::c_ushort,
  pub v_state: libc::c_ushort,
}

pub type C2RustUnnamed = libc::c_uint;
pub const DAEMON_ONLY_SANITIZE: C2RustUnnamed = 8;
pub const DAEMON_CLOSE_EXTRA_FDS: C2RustUnnamed = 4;
pub const DAEMON_DEVNULL_STDIO: C2RustUnnamed = 2;
pub const DAEMON_CHDIR_ROOT: C2RustUnnamed = 1;
pub const OPT_s: C2RustUnnamed_0 = 4;
pub const OPT_w: C2RustUnnamed_0 = 2;
pub const OPT_c: C2RustUnnamed_0 = 1;

pub type C2RustUnnamed_0 = libc::c_uint;
pub const OPT_v: C2RustUnnamed_0 = 32;
pub const OPT_f: C2RustUnnamed_0 = 16;
pub const OPT_l: C2RustUnnamed_0 = 8;

/*
 *  openvt.c - open a vt to run a command.
 *
 *  busyboxed by Quy Tonthat <quy@signal3.com>
 *  hacked by Tito <farmatito@tiscali.it>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config OPENVT
//config:	bool "openvt (7.2 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	This program is used to start a command on an unused
//config:	virtual terminal.
//applet:IF_OPENVT(APPLET(openvt, BB_DIR_USR_BIN, SUID_DROP))
//kbuild:lib-$(CONFIG_OPENVT) += openvt.o
//usage:#define openvt_trivial_usage
//usage:       "[-c N] [-sw] [PROG ARGS]"
//usage:#define openvt_full_usage "\n\n"
//usage:       "Start PROG on a new virtual terminal\n"
//usage:     "\n	-c N	Use specified VT"
//usage:     "\n	-s	Switch to the VT"
/* //usage:     "\n	-l	Run PROG as login shell (by prepending '-')" */
//usage:     "\n	-w	Wait for PROG to exit"
//usage:
//usage:#define openvt_example_usage
//usage:       "openvt 2 /bin/ash\n"
/* "Standard" openvt's man page (we do not support all of this):

openvt [-c NUM] [-fsulv] [--] [command [args]]

Find the first available VT, and run command on it. Stdio is directed
to that VT. If no command is specified then $SHELL is used.

-c NUM
    Use the given VT number, not the first free one.
-f
    Force opening a VT: don't try to check if VT is already in use.
-s
    Switch to the new VT when starting the command.
    The VT of the new command will be made the new current VT.
-u
    Figure out the owner of the current VT, and run login as that user.
    Suitable to be called by init. Shouldn't be used with -c or -l.
-l
    Make the command a login shell: a "-" is prepended to the argv[0]
    when command is executed.
-v
    Verbose.
-w
    Wait for command to complete. If -w and -s are used together,
    switch back to the controlling terminal when the command completes.

bbox:
-u: not implemented
-f: always in effect
-l: not implemented, ignored
-v: ignored
-ws: does NOT switch back
*/
/* Helper: does this fd understand VT_xxx? */
unsafe fn not_vt_fd(mut fd: libc::c_int) -> libc::c_int {
  let mut vtstat: vt_stat = vt_stat {
    v_active: 0,
    v_signal: 0,
    v_state: 0,
  };
  return ioctl(fd, 0x5603i32 as libc::c_ulong, &mut vtstat as *mut vt_stat);
  /* !0: error, it's not VT fd */
}
/* Helper: get a fd suitable for VT_xxx */
unsafe fn get_vt_fd() -> libc::c_int {
  let mut fd: libc::c_int = 0;
  /* Do we, by chance, already have it? */
  fd = 0;
  while fd < 3i32 {
    if not_vt_fd(fd) == 0 {
      return fd;
    }
    fd += 1
  }
  fd = open(
    b"/dev/console\x00" as *const u8 as *const libc::c_char,
    0 | 0o4000i32,
  );
  if fd >= 0 && not_vt_fd(fd) == 0 {
    return fd;
  }
  crate::libbb::verror_msg::bb_simple_error_msg_and_die(
    b"can\'t find open VT\x00" as *const u8 as *const libc::c_char,
  );
}
unsafe fn find_free_vtno() -> libc::c_int {
  let mut vtno: libc::c_int = 0;
  let mut fd: libc::c_int = get_vt_fd();
  *bb_errno = 0;
  /*xfunc_error_retval = 3; - do we need compat? */
  if ioctl(
    fd,
    0x5600i32 as libc::c_ulong,
    &mut vtno as *mut libc::c_int,
  ) != 0
    || vtno <= 0
  {
    crate::libbb::perror_msg::bb_simple_perror_msg_and_die(
      b"can\'t find open VT\x00" as *const u8 as *const libc::c_char,
    );
  }
  // Not really needed, grep for DAEMON_CLOSE_EXTRA_FDS
  //	if (fd > 2)
  //		close(fd);
  return vtno;
}
/* vfork scares gcc, it generates bigger code.
 * Keep it away from main program.
 * TODO: move to libbb; or adapt existing libbb's spawn().
 */
#[inline(never)]
unsafe fn vfork_child(mut argv: *mut *mut libc::c_char) {
  if vfork() == 0 {
    /* CHILD */
    /* Try to make this VT our controlling tty */
    setsid(); /* lose old ctty */
    ioctl(0i32, 0x540ei32 as libc::c_ulong, 0);
    //bb_error_msg("our sid %d", getsid(0));
    //bb_error_msg("our pgrp %d", getpgrp());
    //bb_error_msg("VT's sid %d", tcgetsid(0));
    //bb_error_msg("VT's pgrp %d", tcgetpgrp(0));
    crate::libbb::executable::BB_EXECVP_or_die(argv);
  };
}
pub unsafe fn openvt_main(mut _argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut vtname: [libc::c_char; 23] = [0; 23];
  let mut vtstat: vt_stat = vt_stat {
    v_active: 0,
    v_signal: 0,
    v_state: 0,
  };
  let mut str_c: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut vtno: libc::c_int = 0;
  let mut flags: libc::c_int = 0;
  /* "+" - stop on first non-option */
  flags = crate::libbb::getopt32::getopt32(
    argv,
    b"+c:wslfv\x00" as *const u8 as *const libc::c_char,
    &mut str_c as *mut *mut libc::c_char,
  ) as libc::c_int;
  argv = argv.offset(optind as isize);
  if flags & OPT_c as libc::c_int != 0 {
    /* Check for illegal vt number: < 1 or > 63 */
    vtno = crate::libbb::xatonum::xatou_range(str_c, 1i32 as libc::c_uint, 63i32 as libc::c_uint)
      as libc::c_int
  } else {
    vtno = find_free_vtno()
  }
  /* Grab new VT */
  sprintf(
    vtname.as_mut_ptr(),
    b"/dev/tty%d\x00" as *const u8 as *const libc::c_char,
    vtno,
  );
  /* (Try to) clean up stray open fds above fd 2 */
  crate::libbb::vfork_daemon_rexec::bb_daemonize_or_rexec(
    DAEMON_CLOSE_EXTRA_FDS as libc::c_int | DAEMON_ONLY_SANITIZE as libc::c_int,
  );
  close(0i32);
  /*setsid(); - BAD IDEA: after we exit, child is SIGHUPed... */
  crate::libbb::xfuncs_printf::xopen(vtname.as_mut_ptr(), 0o2i32);
  crate::libbb::xfuncs_printf::bb_xioctl(
    0,
    0x5603i32 as libc::c_uint,
    &mut vtstat as *mut vt_stat as *mut libc::c_void,
    b"VT_GETSTATE\x00" as *const u8 as *const libc::c_char,
  );
  if flags & OPT_s as libc::c_int != 0 {
    crate::libbb::get_console::console_make_active(0i32, vtno);
  }
  if (*argv.offset(0)).is_null() {
    argv = argv.offset(-1);
    let ref mut fresh0 = *argv.offset(0);
    *fresh0 = crate::libbb::get_shell_name::get_shell_name() as *mut libc::c_char
    /*argv[1] = NULL; - already is */
  }
  crate::libbb::xfuncs_printf::xdup2(0i32, 1i32);
  crate::libbb::xfuncs_printf::xdup2(0i32, 2i32);
  vfork_child(argv);
  if flags & OPT_w as libc::c_int != 0 {
    /* We have only one child, wait for it */
    crate::libbb::xfuncs::safe_waitpid(-1i32, 0 as *mut libc::c_int, 0); /* loops on EINTR */
    if flags & OPT_s as libc::c_int != 0 {
      crate::libbb::get_console::console_make_active(0i32, vtstat.v_active as libc::c_int);
      // Compat: even with -c N (try to) disallocate:
      // # /usr/app/kbd-1.12/bin/openvt -f -c 9 -ws sleep 5
      // openvt: could not deallocate console 9
      crate::libbb::xfuncs_printf::bb_xioctl(
        0,
        0x5608i32 as libc::c_uint,
        vtno as ptrdiff_t as *mut libc::c_void,
        b"VT_DISALLOCATE\x00" as *const u8 as *const libc::c_char,
      );
    }
  }
  return 0;
}
