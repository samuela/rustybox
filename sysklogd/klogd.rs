use libc;
extern "C" {
  #[no_mangle]
  fn strtoul(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_ulong;
  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn klogctl(type_0: libc::c_int, b: *mut libc::c_char, len: libc::c_int) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn bb_signals_recursive_norestart(
    sigs: libc::c_int,
    f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
  );
  #[no_mangle]
  fn kill_myself_with_sig(sig: libc::c_int) -> !;
  #[no_mangle]
  static mut bb_got_signal: smallint;
  #[no_mangle]
  fn record_signo(signo: libc::c_int);
  #[no_mangle]
  fn overlapping_strcpy(dst: *mut libc::c_char, src: *const libc::c_char);
  #[no_mangle]
  fn xatou_range(str: *const libc::c_char, l: libc::c_uint, u: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn bb_daemonize_or_rexec(flags: libc::c_int);
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn write_pidfile_std_path_and_ext(path: *const libc::c_char);
  #[no_mangle]
  fn remove_pidfile_std_path_and_ext(path: *const libc::c_char);
  #[no_mangle]
  static mut logmode: smallint;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  static bb_banner: [libc::c_char; 0];
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
  #[no_mangle]
  fn openlog(__ident: *const libc::c_char, __option: libc::c_int, __facility: libc::c_int);
  #[no_mangle]
  fn syslog(__pri: libc::c_int, __fmt: *const libc::c_char, _: ...);
}

use crate::librb::uint32_t;
use crate::librb::smallint;
use crate::librb::signal::__sighandler_t;
pub type C2RustUnnamed = libc::c_uint;
pub const BB_FATAL_SIGS: C2RustUnnamed = 117503054;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const DAEMON_ONLY_SANITIZE: C2RustUnnamed_0 = 8;
pub const DAEMON_CLOSE_EXTRA_FDS: C2RustUnnamed_0 = 4;
pub const DAEMON_DEVNULL_STDIO: C2RustUnnamed_0 = 2;
pub const DAEMON_CHDIR_ROOT: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const LOGMODE_BOTH: C2RustUnnamed_1 = 3;
pub const LOGMODE_SYSLOG: C2RustUnnamed_1 = 2;
pub const LOGMODE_STDIO: C2RustUnnamed_1 = 1;
pub const LOGMODE_NONE: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const OPT_FOREGROUND: C2RustUnnamed_2 = 2;
pub const OPT_LEVEL: C2RustUnnamed_2 = 1;
pub const KLOGD_LOGBUF_SIZE: C2RustUnnamed_2 = 1024;

/*
 * Mini klogd implementation for busybox
 *
 * Copyright (C) 2001 by Gennady Feldman <gfeldman@gena01.com>.
 * Changes: Made this a standalone busybox module which uses standalone
 * syslog() client interface.
 *
 * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
 *
 * Copyright (C) 2000 by Karl M. Hegbloom <karlheg@debian.org>
 *
 * "circular buffer" Copyright (C) 2000 by Gennady Feldman <gfeldman@gena01.com>
 *
 * Maintainer: Gennady Feldman <gfeldman@gena01.com> as of Mar 12, 2001
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config KLOGD
//config:	bool "klogd (5.7 kb)"
//config:	default y
//config:	help
//config:	klogd is a utility which intercepts and logs all
//config:	messages from the Linux kernel and sends the messages
//config:	out to the 'syslogd' utility so they can be logged. If
//config:	you wish to record the messages produced by the kernel,
//config:	you should enable this option.
//config:
//config:comment "klogd should not be used together with syslog to kernel printk buffer"
//config:	depends on KLOGD && FEATURE_KMSG_SYSLOG
//config:
//config:config FEATURE_KLOGD_KLOGCTL
//config:	bool "Use the klogctl() interface"
//config:	default y
//config:	depends on KLOGD
//config:	select PLATFORM_LINUX
//config:	help
//config:	The klogd applet supports two interfaces for reading
//config:	kernel messages. Linux provides the klogctl() interface
//config:	which allows reading messages from the kernel ring buffer
//config:	independently from the file system.
//config:
//config:	If you answer 'N' here, klogd will use the more portable
//config:	approach of reading them from /proc or a device node.
//config:	However, this method requires the file to be available.
//config:
//config:	If in doubt, say 'Y'.
//applet:IF_KLOGD(APPLET(klogd, BB_DIR_SBIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_KLOGD) += klogd.o
//usage:#define klogd_trivial_usage
//usage:       "[-c N] [-n]"
//usage:#define klogd_full_usage "\n\n"
//usage:       "Log kernel messages to syslog\n"
//usage:     "\n	-c N	Print to console messages more urgent than prio N (1-8)"
//usage:     "\n	-n	Run in foreground"
/* The Linux-specific klogctl(3) interface does not rely on the filesystem and
 * allows us to change the console loglevel. Alternatively, we read the
 * messages from _PATH_KLOG. */
unsafe extern "C" fn klogd_open() {
  /* "Open the log. Currently a NOP" */
  klogctl(1i32, 0 as *mut libc::c_char, 0i32);
}
unsafe extern "C" fn klogd_setloglevel(mut lvl: libc::c_int) {
  /* "printk() prints a message on the console only if it has a loglevel
   * less than console_loglevel". Here we set console_loglevel = lvl. */
  klogctl(8i32, 0 as *mut libc::c_char, lvl);
}
unsafe extern "C" fn klogd_read(mut bufp: *mut libc::c_char, mut len: libc::c_int) -> libc::c_int {
  /* "2 -- Read from the log." */
  return klogctl(2i32, bufp, len);
}
unsafe extern "C" fn klogd_close() {
  /* FYI: cmd 7 is equivalent to setting console_loglevel to 7
   * via klogctl(8, NULL, 7). */
  klogctl(7i32, 0 as *mut libc::c_char, 0i32); /* "7 -- Enable printk's to console" */
  klogctl(0i32, 0 as *mut libc::c_char, 0i32);
  /* "0 -- Close the log. Currently a NOP" */
}
/* TODO: glibc openlog(LOG_KERN) reverts to LOG_USER instead,
 * because that's how they interpret word "default"
 * in the openlog() manpage:
 *      LOG_USER (default)
 *              generic user-level messages
 * and the fact that LOG_KERN is a constant 0.
 * glibc interprets it as "0 in openlog() call means 'use default'".
 * I think it means "if openlog wasn't called before syslog() is called,
 * use default".
 * Convincing glibc maintainers otherwise is, as usual, nearly impossible.
 * Should we open-code syslog() here to use correct facility?
 */
#[no_mangle]
pub unsafe extern "C" fn klogd_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut i: libc::c_int = 0i32;
  let mut opt_c: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut opt: libc::c_int = 0;
  let mut used: libc::c_int = 0;
  opt = getopt32(
    argv,
    b"c:n\x00" as *const u8 as *const libc::c_char,
    &mut opt_c as *mut *mut libc::c_char,
  ) as libc::c_int;
  if opt & OPT_LEVEL as libc::c_int != 0 {
    /* Valid levels are between 1 and 8 */
    i = xatou_range(opt_c, 1i32 as libc::c_uint, 8i32 as libc::c_uint) as libc::c_int
  }
  if opt & OPT_FOREGROUND as libc::c_int == 0 {
    bb_daemonize_or_rexec(DAEMON_CHDIR_ROOT as libc::c_int);
  }
  logmode = LOGMODE_SYSLOG as libc::c_int as smallint;
  /* klogd_open() before openlog(), since it might use fixed fd 3,
   * and openlog() also may use the same fd 3 if we swap them:
   */
  klogd_open();
  openlog(
    b"kernel\x00" as *const u8 as *const libc::c_char,
    0i32,
    0i32 << 3i32,
  );
  /*
   * glibc problem: for some reason, glibc changes LOG_KERN to LOG_USER
   * above. The logic behind this is that standard
   * http://pubs.opengroup.org/onlinepubs/9699919799/functions/syslog.html
   * says the following about openlog and syslog:
   * "LOG_USER
   *  Messages generated by arbitrary processes.
   *  This is the default facility identifier if none is specified."
   *
   * I believe glibc misinterpreted this text as "if openlog's
   * third parameter is 0 (=LOG_KERN), treat it as LOG_USER".
   * Whereas it was meant to say "if *syslog* is called with facility
   * 0 in its 1st parameter without prior call to openlog, then perform
   * implicit openlog(LOG_USER)".
   *
   * As a result of this, eh, feature, standard klogd was forced
   * to open-code its own openlog and syslog implementation (!).
   *
   * Note that prohibiting openlog(LOG_KERN) on libc level does not
   * add any security: any process can open a socket to "/dev/log"
   * and write a string "<0>Voila, a LOG_KERN + LOG_EMERG message"
   *
   * Google code search tells me there is no widespread use of
   * openlog("foo", 0, 0), thus fixing glibc won't break userspace.
   *
   * The bug against glibc was filed:
   * bugzilla.redhat.com/show_bug.cgi?id=547000
   */
  if i != 0 {
    klogd_setloglevel(i);
  }
  signal(
    1i32,
    ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
  );
  /* We want klogd_read to not be restarted, thus _norestart: */
  bb_signals_recursive_norestart(
    BB_FATAL_SIGS as libc::c_int,
    Some(record_signo as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  syslog(
    5i32,
    b"klogd started: %s\x00" as *const u8 as *const libc::c_char,
    bb_banner.as_ptr(),
  );
  write_pidfile_std_path_and_ext(b"klogd\x00" as *const u8 as *const libc::c_char);
  used = 0i32;
  while bb_got_signal == 0 {
    let mut n: libc::c_int = 0;
    let mut priority: libc::c_int = 0;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    start = bb_common_bufsiz1.as_mut_ptr().offset(used as isize);
    n = klogd_read(start, KLOGD_LOGBUF_SIZE as libc::c_int - 1i32 - used);
    if n < 0i32 {
      if *bb_errno == 4i32 {
        continue;
      }
      bb_simple_perror_msg(b"klogctl(2) error\x00" as *const u8 as *const libc::c_char);
      break;
    } else {
      *start.offset(n as isize) = '\u{0}' as i32 as libc::c_char;
      /* Process each newline-terminated line in the buffer */
      start = bb_common_bufsiz1.as_mut_ptr();
      loop {
        let mut newline: *mut libc::c_char = strchrnul(start, '\n' as i32);
        if *newline as libc::c_int == '\u{0}' as i32 {
          /* This line is incomplete */
          /* move it to the front of the buffer */
          overlapping_strcpy(bb_common_bufsiz1.as_mut_ptr(), start);
          used = newline.wrapping_offset_from(start) as libc::c_long as libc::c_int;
          if used < KLOGD_LOGBUF_SIZE as libc::c_int - 1i32 {
            break;
          }
          /* buffer is full, log it anyway */
          used = 0i32;
          newline = 0 as *mut libc::c_char
        } else {
          let fresh0 = newline;
          newline = newline.offset(1);
          *fresh0 = '\u{0}' as i32 as libc::c_char
        }
        /* Extract the priority */
        priority = 6i32;
        if *start as libc::c_int == '<' as i32 {
          start = start.offset(1);
          if *start != 0 {
            let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
            priority = strtoul(start, &mut end, 10i32) as libc::c_int;
            if *end as libc::c_int == '>' as i32 {
              end = end.offset(1)
            }
            start = end
          }
        }
        /* Log (only non-empty lines) */
        if *start != 0 {
          syslog(
            priority,
            b"%s\x00" as *const u8 as *const libc::c_char,
            start,
          );
        }
        if newline.is_null() {
          break;
        }
        start = newline
      }
    }
  }
  klogd_close();
  syslog(
    5i32,
    b"klogd: exiting\x00" as *const u8 as *const libc::c_char,
  );
  remove_pidfile_std_path_and_ext(b"klogd\x00" as *const u8 as *const libc::c_char);
  if bb_got_signal != 0 {
    kill_myself_with_sig(bb_got_signal as libc::c_int);
  }
  return 1i32;
}
