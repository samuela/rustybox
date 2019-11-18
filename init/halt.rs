use crate::librb::size_t;
use crate::librb::smallint;

use crate::libbb::appletlib::applet_name;
use libc;
use libc::access;
use libc::kill;
use libc::pid_t;
use libc::sleep;
use libc::strcpy;
use libc::sync;
use libc::time;
use libc::time_t;
extern "C" {

  #[no_mangle]
  fn execlp(__file: *const libc::c_char, __arg: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn updwtmpx(__wtmpx_file: *const libc::c_char, __utmpx: *const utmpx);
  #[no_mangle]
  fn safe_strncpy(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    size: size_t,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_perror_nomsg_and_die() -> !;
  #[no_mangle]
  fn find_pid_by_name(procName: *const libc::c_char) -> *mut pid_t;

  #[no_mangle]
  static bb_path_wtmp_file: [libc::c_char; 0];
  #[no_mangle]
  fn reboot(__howto: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn uname(__name: *mut utsname) -> libc::c_int;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __exit_status {
  pub e_termination: libc::c_short,
  pub e_exit: libc::c_short,
}
use libc::utmpx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
  pub tv_sec: i32,
  pub tv_usec: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
  pub sysname: [libc::c_char; 65],
  pub nodename: [libc::c_char; 65],
  pub release: [libc::c_char; 65],
  pub version: [libc::c_char; 65],
  pub machine: [libc::c_char; 65],
  pub domainname: [libc::c_char; 65],
}
pub const initial: C2RustUnnamed_0 = 5;
pub type C2RustUnnamed_0 = libc::c_uint;

/*
 * Poweroff reboot and halt, oh my.
 *
 * Copyright 2006 by Rob Landley <rob@landley.net>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config HALT
//config:	bool "halt (4 kb)"
//config:	default y
//config:	help
//config:	Stop all processes and halt the system.
//config:
//config:config POWEROFF
//config:	bool "poweroff (4 kb)"
//config:	default y
//config:	help
//config:	Stop all processes and power off the system.
//config:
//config:config REBOOT
//config:	bool "reboot (4 kb)"
//config:	default y
//config:	help
//config:	Stop all processes and reboot the system.
//config:
//config:config FEATURE_WAIT_FOR_INIT
//config:	bool "Before signaling init, make sure it is ready for it"
//config:	default y
//config:	depends on HALT || POWEROFF || REBOOT
//config:	help
//config:	In rare cases, poweroff may be commanded by firmware to OS
//config:	even before init process exists. On Linux, this spawns
//config:	"/sbin/poweroff" very early. This option adds code
//config:	which checks that init is ready to receive poweroff
//config:	commands. Code size increase of ~80 bytes.
//config:
//config:config FEATURE_CALL_TELINIT
//config:	bool "Call telinit on shutdown and reboot"
//config:	default y
//config:	depends on (HALT || POWEROFF || REBOOT) && !INIT
//config:	help
//config:	Call an external program (normally telinit) to facilitate
//config:	a switch to a proper runlevel.
//config:
//config:	This option is only available if you selected halt and friends,
//config:	but did not select init.
//config:
//config:config TELINIT_PATH
//config:	string "Path to telinit executable"
//config:	default "/sbin/telinit"
//config:	depends on FEATURE_CALL_TELINIT
//config:	help
//config:	When busybox halt and friends have to call external telinit
//config:	to facilitate proper shutdown, this path is to be used when
//config:	locating telinit executable.
//applet:IF_HALT(APPLET(halt, BB_DIR_SBIN, BB_SUID_DROP))
//                   APPLET_ODDNAME:name      main  location     suid_type     help
//applet:IF_POWEROFF(APPLET_ODDNAME(poweroff, halt, BB_DIR_SBIN, BB_SUID_DROP, poweroff))
//applet:IF_REBOOT(  APPLET_ODDNAME(reboot,   halt, BB_DIR_SBIN, BB_SUID_DROP, reboot))
//kbuild:lib-$(CONFIG_HALT) += halt.o
//kbuild:lib-$(CONFIG_POWEROFF) += halt.o
//kbuild:lib-$(CONFIG_REBOOT) += halt.o
//usage:#define halt_trivial_usage
//usage:       "[-d DELAY] [-n] [-f]" IF_FEATURE_WTMP(" [-w]")
//usage:#define halt_full_usage "\n\n"
//usage:       "Halt the system\n"
//usage:     "\n	-d SEC	Delay interval"
//usage:     "\n	-n	Do not sync"
//usage:     "\n	-f	Force (don't go through init)"
//usage:	IF_FEATURE_WTMP(
//usage:     "\n	-w	Only write a wtmp record"
//usage:	)
//usage:
//usage:#define poweroff_trivial_usage
//usage:       "[-d DELAY] [-n] [-f]"
//usage:#define poweroff_full_usage "\n\n"
//usage:       "Halt and shut off power\n"
//usage:     "\n	-d SEC	Delay interval"
//usage:     "\n	-n	Do not sync"
//usage:     "\n	-f	Force (don't go through init)"
//usage:
//usage:#define reboot_trivial_usage
//usage:       "[-d DELAY] [-n] [-f]"
//usage:#define reboot_full_usage "\n\n"
//usage:       "Reboot the system\n"
//usage:     "\n	-d SEC	Delay interval"
//usage:     "\n	-n	Do not sync"
//usage:     "\n	-f	Force (don't go through init)"
unsafe extern "C" fn write_wtmp() {
  // TODO: use std::mem:zeroed here
  let mut utmp: utmpx = std::mem::zeroed();
  let mut uts: utsname = utsname {
    sysname: [0; 65],
    nodename: [0; 65],
    release: [0; 65],
    version: [0; 65],
    machine: [0; 65],
    domainname: [0; 65],
  };
  /* "man utmp" says wtmp file should *not* be created automagically */
  /*if (access(bb_path_wtmp_file, R_OK|W_OK) == -1) {
    close(creat(bb_path_wtmp_file, 0664));
  }*/
  memset(
    &mut utmp as *mut utmpx as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<utmpx>() as libc::c_ulong,
  ); /* it is wide enough */
  utmp.ut_tv.tv_sec = time(0 as *mut time_t) as i32; /* = strcpy(utmp.ut_id, "~~"); */
  strcpy(
    utmp.ut_user.as_mut_ptr(),
    b"shutdown\x00" as *const u8 as *const libc::c_char,
  ); /* = strcpy(utmp.ut_line, "~~"); */
  utmp.ut_type = 1i32 as libc::c_short;
  utmp.ut_id[0] = '~' as i32 as libc::c_char;
  utmp.ut_id[1] = '~' as i32 as libc::c_char;
  utmp.ut_line[0] = '~' as i32 as libc::c_char;
  utmp.ut_line[1] = '~' as i32 as libc::c_char;
  uname(&mut uts);
  safe_strncpy(
    utmp.ut_host.as_mut_ptr(),
    uts.release.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
  );
  updwtmpx(bb_path_wtmp_file.as_ptr(), &mut utmp);
}
/* In Linux, "poweroff" may be spawned even before init.
 * For example, with ACPI:
 * linux/drivers/acpi/bus.c:
 *  static void sb_notify_work(struct work_struct *dummy)
 *      orderly_poweroff(true);
 * linux/kernel/reboot.c:
 *  static int run_cmd(const char *cmd)
 *      ret = call_usermodehelper(argv[0], argv, envp, UMH_WAIT_EXEC);
 *  poweroff_cmd[] = "/sbin/poweroff";
 *  static int __orderly_poweroff(bool force)
 *      ret = run_cmd(poweroff_cmd);
 *
 * We want to make sure init exists and listens to signals.
 */
unsafe extern "C" fn init_was_not_there() -> libc::c_int {
  let mut cnt: libc::c_int = initial as libc::c_int - 1i32; /* 5 seconds should be plenty for timeout */
  /* Just existence of PID 1 does not mean it installed
   * the handlers already.
   */
  /* ... so let's wait for some evidence a usual startup event,
   * mounting of /proc, happened. By that time init should be ready
   * for signals.
   */
  while access(
    b"/proc/meminfo\x00" as *const u8 as *const libc::c_char,
    0i32,
  ) != 0i32
    && {
      cnt -= 1;
      (cnt) >= 0i32
    }
  {
    sleep(1i32 as libc::c_uint);
  }
  /* Does it look like init wasn't there? */
  return (cnt != initial as libc::c_int - 1i32) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn halt_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  static mut magic: [libc::c_int; 3] = [0xcdef0123u32 as libc::c_int, 0x4321fedci32, 0x1234567i32];
  static mut signals: [smallint; 3] = [10i32 as smallint, 12i32 as smallint, 15i32 as smallint];
  let mut delay: libc::c_int = 0i32;
  let mut which: libc::c_int = 0;
  let mut flags: libc::c_int = 0;
  let mut rc: libc::c_int = 0;
  /* Figure out which applet we're running */
  if 1i32 != 0 && 1i32 == 0 && 1i32 == 0 {
    which = 0i32
  } else if 1i32 == 0 && 1i32 != 0 && 1i32 == 0 {
    which = 1i32
  } else if 1i32 == 0 && 1i32 == 0 && 1i32 != 0 {
    which = 2i32
  } else {
    which = 0i32;
    while (*::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"hpr\x00"))[which as usize]
      as libc::c_int
      != *applet_name.offset(0) as libc::c_int
    {
      which += 1
    }
  }
  /* Parse and handle arguments */
  /* We support -w even if !ENABLE_FEATURE_WTMP,
   * in order to not break scripts.
   * -i (shut down network interfaces) is ignored.
   */
  flags = getopt32(
    argv,
    b"d:+nfwi\x00" as *const u8 as *const libc::c_char,
    &mut delay as *mut libc::c_int,
  ) as libc::c_int;
  sleep(delay as libc::c_uint);
  write_wtmp();
  if flags & 8i32 != 0 {
    /* -w */
    return 0i32;
  }
  if flags & 2i32 == 0 {
    /* no -n */
    sync();
  }
  /* Perform action. */
  rc = 1i32;
  if flags & 4i32 == 0 {
    /* no -f */
    //TODO: I tend to think that signalling linuxrc is wrong
    // pity original author didn't comment on it...
    /* talk to linuxrc */
    /* bbox init/linuxrc assumed */
    let mut pidlist: *mut pid_t =
      find_pid_by_name(b"linuxrc\x00" as *const u8 as *const libc::c_char);
    if *pidlist.offset(0) > 0i32 {
      rc = kill(*pidlist.offset(0), signals[which as usize] as libc::c_int)
    }
    if rc != 0 {
      /* talk to init */
      if 0i32 == 0 {
        /* bbox init assumed */
        rc = kill(1i32, signals[which as usize] as libc::c_int);
        if init_was_not_there() != 0 {
          rc = kill(1i32, signals[which as usize] as libc::c_int)
        }
      } else {
        /* SysV style init assumed */
        /* runlevels:
         * 0 == shutdown
         * 6 == reboot */
        execlp(
          b"\x00" as *const u8 as *const libc::c_char,
          b"\x00" as *const u8 as *const libc::c_char,
          if which == 2i32 {
            b"6\x00" as *const u8 as *const libc::c_char
          } else {
            b"0\x00" as *const u8 as *const libc::c_char
          },
          0 as *mut libc::c_void as *mut libc::c_char,
        );
        bb_perror_msg_and_die(
          b"can\'t execute \'%s\'\x00" as *const u8 as *const libc::c_char,
          b"\x00" as *const u8 as *const libc::c_char,
        );
      }
    }
  } else {
    rc = reboot(magic[which as usize])
  }
  if rc != 0 {
    bb_perror_nomsg_and_die();
  }
  return rc;
}
