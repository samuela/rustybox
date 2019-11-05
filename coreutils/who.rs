use libc;
extern "C" {
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn time(__timer: *mut time_t) -> time_t;
  #[no_mangle]
  fn ctime(__timer: *const time_t) -> *mut libc::c_char;
  #[no_mangle]
  fn setutxent();
  #[no_mangle]
  fn endutxent();
  #[no_mangle]
  fn getutxent() -> *mut utmpx;
  #[no_mangle]
  fn safe_strncpy(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    size: size_t,
  ) -> *mut libc::c_char;
  /* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
   * But potentially slow, don't use in one-billion-times loops */
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
}
pub type __int32_t = libc::c_int;

pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
use crate::librb::__pid_t;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
use crate::librb::size_t;
use crate::librb::stat;
use crate::librb::time_t;
use crate::librb::timespec;
use crate::librb::uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __exit_status {
  pub e_termination: libc::c_short,
  pub e_exit: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utmpx {
  pub ut_type: libc::c_short,
  pub ut_pid: __pid_t,
  pub ut_line: [libc::c_char; 32],
  pub ut_id: [libc::c_char; 4],
  pub ut_user: [libc::c_char; 32],
  pub ut_host: [libc::c_char; 256],
  pub ut_exit: __exit_status,
  pub ut_session: __int32_t,
  pub ut_tv: C2RustUnnamed,
  pub ut_addr_v6: [__int32_t; 4],
  pub __glibc_reserved: [libc::c_char; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
  pub tv_sec: __int32_t,
  pub tv_usec: __int32_t,
}

/*
 * Mini who is used to display user name, login time,
 * idle time and host name.
 *
 * Author: Da Chen  <dchen@ayrnetworks.com>
 *
 * This is a free document; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation:
 *    http://www.gnu.org/copyleft/gpl.html
 *
 * Copyright (c) 2002 AYR Networks, Inc.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config WHO
//config:	bool "who (3.9 kb)"
//config:	default y
//config:	depends on FEATURE_UTMP
//config:	help
//config:	Print users currently logged on.
//config:
// procps-ng has this variation of "who":
//config:config W
//config:	bool "w (3.8 kb)"
//config:	default y
//config:	depends on FEATURE_UTMP
//config:	help
//config:	Print users currently logged on.
//config:
//config:config USERS
//config:	bool "users (3.4 kb)"
//config:	default y
//config:	depends on FEATURE_UTMP
//config:	help
//config:	Print users currently logged on.
//                APPLET_NOEXEC:name   main location        suid_type     help
//applet:IF_USERS(APPLET_NOEXEC(users, who, BB_DIR_USR_BIN, BB_SUID_DROP, users))
//applet:IF_W(    APPLET_NOEXEC(w,     who, BB_DIR_USR_BIN, BB_SUID_DROP, w))
//applet:IF_WHO(  APPLET_NOEXEC(who,   who, BB_DIR_USR_BIN, BB_SUID_DROP, who))
//kbuild:lib-$(CONFIG_USERS) += who.o
//kbuild:lib-$(CONFIG_W)     += who.o
//kbuild:lib-$(CONFIG_WHO)   += who.o
/* BB_AUDIT SUSv3 _NOT_ compliant -- missing options -b, -d, -l, -m, -p, -q, -r, -s, -t, -T, -u; Missing argument 'file'.  */
//usage:#define users_trivial_usage
//usage:       ""
//usage:#define users_full_usage "\n\n"
//usage:       "Print the users currently logged on"
//usage:#define w_trivial_usage
//usage:       ""
//usage:#define w_full_usage "\n\n"
//usage:       "Show who is logged on"
//
// procps-ng 3.3.10:
//           "\n	-h, --no-header"
//           "\n	-u, --no-current"
//	Ignores the username while figuring out the current process
//	and cpu times.  To demonstrate this, do a "su" and do a "w" and a "w -u".
//           "\n	-s, --short"
//	Short format.  Don't print the login time, JCPU or PCPU times.
//           "\n	-f, --from"
//	Toggle printing the from (remote hostname) field.
//	The default is for the from field to not be printed
//           "\n	-i, --ip-addr"
//	Display IP address instead of hostname for from field.
//           "\n	-o, --old-style"
//	Old style output. Prints blank space for idle times less than one minute.
// Example output:
//  17:28:00 up 4 days, 22:41,  4 users,  load average: 0.84, 0.97, 0.90
// USER     TTY        LOGIN@   IDLE   JCPU   PCPU WHAT
// root     tty1      Thu18    4days  4:33m  0.07s /bin/sh /etc/xdg/xfce4/xinitrc -- vt
// root     pts/1     Mon13    3:24m  1:01   0.01s w
//usage:#define who_trivial_usage
//usage:       "[-a]"
//usage:#define who_full_usage "\n\n"
//usage:       "Show who is logged on\n"
//usage:     "\n	-a	Show all"
//usage:     "\n	-H	Print column headers"
unsafe extern "C" fn idle_string(mut str6: *mut libc::c_char, mut t: time_t) {
  t = time(0 as *mut time_t) - t;
  /*if (t < 60) {
    str6[0] = '.';
    str6[1] = '\0';
    return;
  }*/
  if t >= 0i32 as libc::c_long && t < (24i32 * 60i32 * 60i32) as libc::c_long {
    sprintf(
      str6,
      b"%02d:%02d\x00" as *const u8 as *const libc::c_char,
      (t / (60i32 * 60i32) as libc::c_long) as libc::c_int,
      (t % (60i32 * 60i32) as libc::c_long / 60i32 as libc::c_long) as libc::c_int,
    );
    return;
  }
  strcpy(str6, b"old\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn who_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut do_users: libc::c_int = (1i32 != 0
    && (1i32 + 1i32 + 1i32 == 1i32 || *applet_name.offset(0) as libc::c_int == 'u' as i32))
    as libc::c_int;
  let mut do_w: libc::c_int = (1i32 != 0
    && (1i32 + 1i32 + 1i32 == 1i32 || *applet_name.offset(1) as libc::c_int == '\u{0}' as i32))
    as libc::c_int;
  let mut do_who: libc::c_int = (1i32 != 0
    && (1i32 + 1i32 + 1i32 == 1i32 || *applet_name.offset(1) as libc::c_int == 'h' as i32))
    as libc::c_int;
  let mut ut: *mut utmpx = 0 as *mut utmpx;
  let mut opt: libc::c_uint = 0;
  let mut fmt: *const libc::c_char = b"%s\x00" as *const u8 as *const libc::c_char;
  opt = getopt32(
    argv,
    if do_who != 0 {
      b"^aH\x00=0\x00" as *const u8 as *const libc::c_char
    } else {
      b"^\x00=0\x00" as *const u8 as *const libc::c_char
    },
  );
  if opt & 2i32 as libc::c_uint != 0 || do_w != 0 {
    /* -H or we are w */
    puts(b"USER\t\tTTY\t\tIDLE\tTIME\t\t HOST\x00" as *const u8 as *const libc::c_char);
  }
  setutxent();
  loop {
    ut = getutxent();
    if ut.is_null() {
      break;
    }
    if (*ut).ut_user[0] as libc::c_int != 0
      && (opt & 1i32 as libc::c_uint != 0 || (*ut).ut_type as libc::c_int == 7i32)
    {
      if do_users == 0 {
        let mut str6: [libc::c_char; 6] = [0; 6];
        let mut name: [libc::c_char; 39] = [0; 39];
        let mut st: stat = stat {
          st_dev: 0,
          st_ino: 0,
          st_nlink: 0,
          st_mode: 0,
          st_uid: 0,
          st_gid: 0,
          __pad0: 0,
          st_rdev: 0,
          st_size: 0,
          st_blksize: 0,
          st_blocks: 0,
          st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
          },
          st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
          },
          st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
          },
          __glibc_reserved: [0; 3],
        };
        let mut seconds: time_t = 0;
        str6[0] = '?' as i32 as libc::c_char;
        str6[1] = '\u{0}' as i32 as libc::c_char;
        strcpy(
          name.as_mut_ptr(),
          b"/dev/\x00" as *const u8 as *const libc::c_char,
        );
        safe_strncpy(
          if (*ut).ut_line[0] as libc::c_int == '/' as i32 {
            name.as_mut_ptr()
          } else {
            name
              .as_mut_ptr()
              .offset(::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as isize)
              .offset(-1)
          },
          (*ut).ut_line.as_mut_ptr(),
          (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
            .wrapping_add(1i32 as libc::c_ulong),
        );
        if stat(name.as_mut_ptr(), &mut st) == 0i32 {
          idle_string(str6.as_mut_ptr(), st.st_atim.tv_sec);
        }
        /* manpages say ut_tv.tv_sec *is* time_t,
         * but some systems have it wrong */
        seconds = (*ut).ut_tv.tv_sec as time_t;
        /* How wide time field can be?
         * "Nov 10 19:33:20": 15 chars
         * "2010-11-10 19:33": 16 chars
         */
        printf(
          b"%-15.*s %-15.*s %-7s %-16.16s %.*s\n\x00" as *const u8 as *const libc::c_char,
          ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
          (*ut).ut_user.as_mut_ptr(),
          ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
          (*ut).ut_line.as_mut_ptr(),
          str6.as_mut_ptr(),
          ctime(&mut seconds).offset(4),
          ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
          (*ut).ut_host.as_mut_ptr(),
        );
      } else {
        printf(fmt, (*ut).ut_user.as_mut_ptr());
        fmt = b" %s\x00" as *const u8 as *const libc::c_char
      }
    }
  }
  if do_users != 0 {
    bb_putchar('\n' as i32);
  }
  return 0i32;
}
