use libc;
extern "C" {
  #[no_mangle]
  fn strtoul(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_ulong;
  #[no_mangle]
  fn strtoull(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_ulonglong;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  /* These parse entries in /etc/passwd and /etc/group.  This is desirable
   * for BusyBox since we want to avoid using the glibc NSS stuff, which
   * increases target size and is often not needed on embedded systems.  */
  /* wrapper: allows string to contain numeric uid or gid */
  /* always sets uid and gid; returns 0 on failure */
  /* always sets uid and gid; exits on failure */
  /* chown-like handling of "user[:[group]" */
  /* versions which cache results (useful for ps, ls etc) */
  /* internally usernames are saved in fixed-sized char[] buffers */
  /*
   * Returns (-1) terminated malloced result of getgroups().
   * Reallocs group_array (useful for repeated calls).
   * ngroups is an initial size of array. It is rounded up to 32 for realloc.
   * ngroups is updated on return.
   * ngroups can be NULL: bb_getgroups(NULL, NULL) is valid usage.
   * Dies on errors (on Linux, only xrealloc can cause this, not internal getgroups call).
   */
  /* BB_EXECxx always execs (it's not doing NOFORK/NOEXEC stuff),
   * but it may exec busybox and call applet instead of searching PATH.
   */
  /* xvfork() can't be a _function_, return after vfork in child mangles stack
   * in the parent. It must be a macro. */
  /* NOMMU friendy fork+exec: */
  /* wait4pid: unlike waitpid, waits ONLY for one process.
   * Returns sig + 0x180 if child is killed by signal.
   * It's safe to pass negative 'pids' from failed [v]fork -
   * wait4pid will return -1 (and will not clobber [v]fork's errno).
   * IOW: rc = wait4pid(spawn(argv));
   *      if (rc < 0) bb_perror_msg("%s", argv[0]);
   *      if (rc > 0) bb_error_msg("exit code: %d", rc & 0xff);
   */
  /* ***********************************************************************/
  /* spawn_and_wait/run_nofork_applet/run_applet_no_and_exit need to work */
  /* carefully together to reinit some global state while not disturbing  */
  /* other. Be careful if you change them. Consult docs/nofork_noexec.txt */
  /* ***********************************************************************/
  /* Same as wait4pid(spawn(argv)), but with NOFORK/NOEXEC if configured: */
  /* Does NOT check that applet is NOFORK, just blindly runs it */
  /* Helpers for daemonization.
   *
   * bb_daemonize(flags) = daemonize, does not compile on NOMMU
   *
   * bb_daemonize_or_rexec(flags, argv) = daemonizes on MMU (and ignores argv),
   *      rexec's itself on NOMMU with argv passed as command line.
   * Thus bb_daemonize_or_rexec may cause your <applet>_main() to be re-executed
   * from the start. (It will detect it and not reexec again second time).
   * You have to audit carefully that you don't do something twice as a result
   * (opening files/sockets, parsing config files etc...)!
   *
   * Both of the above will redirect fd 0,1,2 to /dev/null and drop ctty
   * (will do setsid()).
   *
   * fork_or_rexec(argv) = bare-bones fork on MMU,
   *      "vfork + re-exec ourself" on NOMMU. No fd redirection, no setsid().
   *      On MMU ignores argv.
   *
   * Helper for network daemons in foreground mode:
   *
   * bb_sanitize_stdio() = make sure that fd 0,1,2 are opened by opening them
   * to /dev/null if they are not.
   */
  /* internal use */
  //DAEMON_DOUBLE_FORK     = 1 << 4, /* double fork to avoid controlling tty */
  /* Clear dangerous stuff, set PATH. Return 1 if was run by different user. */
  /* For top, ps. Some argv[i] are replaced by malloced "-opt" strings */
  /* { "-", NULL } */
  /* BSD-derived getopt() functions require that optind be set to 1 in
   * order to reset getopt() state.  This used to be generally accepted
   * way of resetting getopt().  However, glibc's getopt()
   * has additional getopt() state beyond optind (specifically, glibc
   * extensions such as '+' and '-' at the start of the string), and requires
   * that optind be set to zero to reset its state.  BSD-derived versions
   * of getopt() misbehaved if optind is set to 0 in order to reset getopt(),
   * and glibc's getopt() used to coredump if optind is set 1 in order
   * to reset getopt().
   * Then BSD introduced additional variable "optreset" which should be
   * set to 1 in order to reset getopt().  Sigh.  Standards, anyone?
   *
   * By ~2008, OpenBSD 3.4 was changed to survive glibc-like optind = 0
   * (to interpret it as if optreset was set).
   */
  /*def __GLIBC__*/
  /* BSD style */
  /* Having next pointer as a first member allows easy creation
   * of "llist-compatible" structs, and using llist_FOO functions
   * on them.
   */
  /* BTW, surprisingly, changing API to
   *   llist_t *llist_add_to(llist_t *old_head, void *data)
   * etc does not result in smaller code... */
  /* start_stop_daemon and udhcpc are special - they want
   * to create pidfiles regardless of FEATURE_PIDFILE */
  /* True only if we created pidfile which is *file*, not /dev/null etc */
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
}
pub type __uint16_t = libc::c_ushort;
pub type uint16_t = __uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct suffix_mult {
  pub suffix: [libc::c_char; 4],
  pub mult: libc::c_uint,
}
/*
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/*
You need to define the following (example):

#define type long
#define xstrtou(rest) xstrtoul##rest
#define xstrto(rest) xstrtol##rest
#define xatou(rest) xatoul##rest
#define xato(rest) xatol##rest
#define XSTR_UTYPE_MAX ULONG_MAX
#define XSTR_TYPE_MAX LONG_MAX
#define XSTR_TYPE_MIN LONG_MIN
#define XSTR_STRTOU strtoul
*/
/*
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/*
You need to define the following (example):

#define type long
#define xstrtou(rest) xstrtoul##rest
#define xstrto(rest) xstrtol##rest
#define xatou(rest) xatoul##rest
#define xato(rest) xatol##rest
#define XSTR_UTYPE_MAX ULONG_MAX
#define XSTR_TYPE_MAX LONG_MAX
#define XSTR_TYPE_MIN LONG_MIN
#define XSTR_STRTOU strtoul
*/
#[no_mangle]
pub unsafe extern "C" fn xstrtoull_range_sfx(
  mut numstr: *const libc::c_char,
  mut base: libc::c_int,
  mut lower: libc::c_ulonglong,
  mut upper: libc::c_ulonglong,
  mut suffixes: *const suffix_mult,
) -> libc::c_ulonglong {
  let mut current_block: u64;
  let mut r: libc::c_ulonglong = 0;
  let mut old_errno: libc::c_int = 0;
  let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
  /* Disallow '-' and any leading whitespace. */
  /* Disallow '-' and any leading whitespace. */
  if !(*numstr as libc::c_int == '-' as i32
    || *numstr as libc::c_int == '+' as i32
    || ({
      let mut bb__isspace: libc::c_uchar = (*numstr as libc::c_int - 9i32) as libc::c_uchar;
      (bb__isspace as libc::c_int == ' ' as i32 - 9i32
        || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
    }) != 0)
  {
    /* Since this is a lib function, we're not allowed to reset errno to 0.
     * Doing so could break an app that is deferring checking of errno.
     * So, save the old value so that we can restore it if successful. */
    /* Since this is a lib function, we're not allowed to reset errno to 0.
     * Doing so could break an app that is deferring checking of errno.
     * So, save the old value so that we can restore it if successful. */
    old_errno = *bb_errno;
    *bb_errno = 0i32;
    r = strtoull(numstr, &mut e, base);
    /* Do the initial validity check.  Note: The standards do not
     * guarantee that errno is set if no digits were found.  So we
     * must test for this explicitly. */
    /* Do the initial validity check.  Note: The standards do not
     * guarantee that errno is set if no digits were found.  So we
     * must test for this explicitly. */
    if !(*bb_errno != 0 || numstr == e) {
      /* error / no digits / illegal trailing chars */
      /* error / no digits / illegal trailing chars */
      *bb_errno = old_errno;
      /* Ok.  So restore errno. */
      /* Ok.  So restore errno. */
      /* Do optional suffix parsing.  Allow 'empty' suffix tables.
       * Note that we also allow nul suffixes with associated multipliers,
       * to allow for scaling of the numstr by some default multiplier. */
      /* Do optional suffix parsing.  Allow 'empty' suffix tables.
       * Note that we also allow nul suffixes with associated multipliers,
       * to allow for scaling of the numstr by some default multiplier. */
      if !suffixes.is_null() {
        loop {
          if !((*suffixes).mult != 0) {
            current_block = 7976072742316086414;
            break;
          }
          if strcmp((*suffixes).suffix.as_ptr(), e) == 0i32 {
            if (9223372036854775807i64 as libc::c_ulonglong)
              .wrapping_mul(2u64)
              .wrapping_add(1u64)
              .wrapping_div((*suffixes).mult as libc::c_ulonglong)
              < r
            {
              current_block = 11445230999298041061;
              break;
            }
            /* overflow! */
            /* overflow! */
            r = r.wrapping_mul((*suffixes).mult as libc::c_ulonglong);
            current_block = 867545450848386021;
            break;
          } else {
            suffixes = suffixes.offset(1)
          }
        }
      } else {
        current_block = 7976072742316086414;
      }
      match current_block {
        7976072742316086414 =>
        /* Note: trailing space is an error.
         * It would be easy enough to allow though if desired. */
        /* Note: trailing space is an error.
         * It would be easy enough to allow though if desired. */
        {
          if *e != 0 {
            current_block = 11475201832830405258;
          } else {
            current_block = 867545450848386021;
          }
        }
        _ => {}
      }
      match current_block {
        11475201832830405258 => {}
        _ => {
          match current_block {
            867545450848386021 => {
              /* Finally, check for range limits. */
              /* Finally, check for range limits. */
              if r >= lower && r <= upper {
                return r;
              }
            }
            _ => {}
          }
          bb_error_msg_and_die(
            b"number %s is not in %llu..%llu range\x00" as *const u8 as *const libc::c_char,
            numstr,
            lower,
            upper,
          );
        }
      }
    }
  }
  bb_error_msg_and_die(
    b"invalid number \'%s\'\x00" as *const u8 as *const libc::c_char,
    numstr,
  );
}
#[no_mangle]
pub unsafe extern "C" fn xstrtou_range_sfx(
  mut numstr: *const libc::c_char,
  mut base: libc::c_int,
  mut lower: libc::c_uint,
  mut upper: libc::c_uint,
  mut suffixes: *const suffix_mult,
) -> libc::c_uint {
  let mut current_block: u64;
  let mut r: libc::c_uint = 0;
  let mut old_errno: libc::c_int = 0;
  let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
  if !(*numstr as libc::c_int == '-' as i32
    || *numstr as libc::c_int == '+' as i32
    || ({
      let mut bb__isspace: libc::c_uchar = (*numstr as libc::c_int - 9i32) as libc::c_uchar;
      (bb__isspace as libc::c_int == ' ' as i32 - 9i32
        || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
    }) != 0)
  {
    old_errno = *bb_errno;
    *bb_errno = 0i32;
    r = bb_strtoui(numstr, &mut e, base);
    if !(*bb_errno != 0 || numstr == e) {
      *bb_errno = old_errno;
      if !suffixes.is_null() {
        loop {
          if !((*suffixes).mult != 0) {
            current_block = 7976072742316086414;
            break;
          }
          if strcmp((*suffixes).suffix.as_ptr(), e) == 0i32 {
            if (2147483647i32 as libc::c_uint)
              .wrapping_mul(2u32)
              .wrapping_add(1u32)
              .wrapping_div((*suffixes).mult)
              < r
            {
              current_block = 1299544639425101402;
              break;
            }
            r = r.wrapping_mul((*suffixes).mult);
            current_block = 13950620942190400307;
            break;
          } else {
            suffixes = suffixes.offset(1)
          }
        }
      } else {
        current_block = 7976072742316086414;
      }
      match current_block {
        7976072742316086414 => {
          if *e != 0 {
            current_block = 7295379926513587625;
          } else {
            current_block = 13950620942190400307;
          }
        }
        _ => {}
      }
      match current_block {
        7295379926513587625 => {}
        _ => {
          match current_block {
            13950620942190400307 => {
              if r >= lower && r <= upper {
                return r;
              }
            }
            _ => {}
          }
          bb_error_msg_and_die(
            b"number %s is not in %llu..%llu range\x00" as *const u8 as *const libc::c_char,
            numstr,
            lower as libc::c_ulonglong,
            upper as libc::c_ulonglong,
          );
        }
      }
    }
  }
  bb_error_msg_and_die(
    b"invalid number \'%s\'\x00" as *const u8 as *const libc::c_char,
    numstr,
  );
}
#[no_mangle]
pub unsafe extern "C" fn xstrtou_range(
  mut numstr: *const libc::c_char,
  mut base: libc::c_int,
  mut lower: libc::c_uint,
  mut upper: libc::c_uint,
) -> libc::c_uint {
  return xstrtou_range_sfx(numstr, base, lower, upper, 0 as *const suffix_mult);
}
#[no_mangle]
pub unsafe extern "C" fn xstrtoull_range(
  mut numstr: *const libc::c_char,
  mut base: libc::c_int,
  mut lower: libc::c_ulonglong,
  mut upper: libc::c_ulonglong,
) -> libc::c_ulonglong {
  return xstrtoull_range_sfx(numstr, base, lower, upper, 0 as *const suffix_mult);
}
#[no_mangle]
pub unsafe extern "C" fn xstrtoull_sfx(
  mut numstr: *const libc::c_char,
  mut base: libc::c_int,
  mut suffixes: *const suffix_mult,
) -> libc::c_ulonglong {
  return xstrtoull_range_sfx(
    numstr,
    base,
    0i32 as libc::c_ulonglong,
    (9223372036854775807i64 as libc::c_ulonglong)
      .wrapping_mul(2u64)
      .wrapping_add(1u64),
    suffixes,
  );
}
#[no_mangle]
pub unsafe extern "C" fn xstrtou_sfx(
  mut numstr: *const libc::c_char,
  mut base: libc::c_int,
  mut suffixes: *const suffix_mult,
) -> libc::c_uint {
  return xstrtou_range_sfx(
    numstr,
    base,
    0i32 as libc::c_uint,
    (2147483647i32 as libc::c_uint)
      .wrapping_mul(2u32)
      .wrapping_add(1u32),
    suffixes,
  );
}
#[no_mangle]
pub unsafe extern "C" fn xstrtou(
  mut numstr: *const libc::c_char,
  mut base: libc::c_int,
) -> libc::c_uint {
  return xstrtou_range_sfx(
    numstr,
    base,
    0i32 as libc::c_uint,
    (2147483647i32 as libc::c_uint)
      .wrapping_mul(2u32)
      .wrapping_add(1u32),
    0 as *const suffix_mult,
  );
}
#[no_mangle]
pub unsafe extern "C" fn xstrtoull(
  mut numstr: *const libc::c_char,
  mut base: libc::c_int,
) -> libc::c_ulonglong {
  return xstrtoull_range_sfx(
    numstr,
    base,
    0i32 as libc::c_ulonglong,
    (9223372036854775807i64 as libc::c_ulonglong)
      .wrapping_mul(2u64)
      .wrapping_add(1u64),
    0 as *const suffix_mult,
  );
}
#[no_mangle]
pub unsafe extern "C" fn xatou_range_sfx(
  mut numstr: *const libc::c_char,
  mut lower: libc::c_uint,
  mut upper: libc::c_uint,
  mut suffixes: *const suffix_mult,
) -> libc::c_uint {
  return xstrtou_range_sfx(numstr, 10i32, lower, upper, suffixes);
}
#[no_mangle]
pub unsafe extern "C" fn xatoull_range_sfx(
  mut numstr: *const libc::c_char,
  mut lower: libc::c_ulonglong,
  mut upper: libc::c_ulonglong,
  mut suffixes: *const suffix_mult,
) -> libc::c_ulonglong {
  return xstrtoull_range_sfx(numstr, 10i32, lower, upper, suffixes);
}
#[no_mangle]
pub unsafe extern "C" fn xatou_range(
  mut numstr: *const libc::c_char,
  mut lower: libc::c_uint,
  mut upper: libc::c_uint,
) -> libc::c_uint {
  return xstrtou_range_sfx(numstr, 10i32, lower, upper, 0 as *const suffix_mult);
}
#[no_mangle]
pub unsafe extern "C" fn xatoull_range(
  mut numstr: *const libc::c_char,
  mut lower: libc::c_ulonglong,
  mut upper: libc::c_ulonglong,
) -> libc::c_ulonglong {
  return xstrtoull_range_sfx(numstr, 10i32, lower, upper, 0 as *const suffix_mult);
}
#[no_mangle]
pub unsafe extern "C" fn xatou_sfx(
  mut numstr: *const libc::c_char,
  mut suffixes: *const suffix_mult,
) -> libc::c_uint {
  return xstrtou_range_sfx(
    numstr,
    10i32,
    0i32 as libc::c_uint,
    (2147483647i32 as libc::c_uint)
      .wrapping_mul(2u32)
      .wrapping_add(1u32),
    suffixes,
  );
}
#[no_mangle]
pub unsafe extern "C" fn xatoull_sfx(
  mut numstr: *const libc::c_char,
  mut suffixes: *const suffix_mult,
) -> libc::c_ulonglong {
  return xstrtoull_range_sfx(
    numstr,
    10i32,
    0i32 as libc::c_ulonglong,
    (9223372036854775807i64 as libc::c_ulonglong)
      .wrapping_mul(2u64)
      .wrapping_add(1u64),
    suffixes,
  );
}
#[no_mangle]
pub unsafe extern "C" fn xatoull(mut numstr: *const libc::c_char) -> libc::c_ulonglong {
  return xatoull_sfx(numstr, 0 as *const suffix_mult);
}
#[no_mangle]
pub unsafe extern "C" fn xatou(mut numstr: *const libc::c_char) -> libc::c_uint {
  return xatou_sfx(numstr, 0 as *const suffix_mult);
}
/* Signed ones */
/* Signed ones */
#[no_mangle]
pub unsafe extern "C" fn xstrtoi_range_sfx(
  mut numstr: *const libc::c_char,
  mut base: libc::c_int,
  mut lower: libc::c_int,
  mut upper: libc::c_int,
  mut suffixes: *const suffix_mult,
) -> libc::c_int {
  let mut u: libc::c_uint = 2147483647i32 as libc::c_uint;
  let mut r: libc::c_int = 0;
  let mut p: *const libc::c_char = numstr;
  /* NB: if you'll decide to disallow '+':
   * at least renice applet needs to allow it */
  /* NB: if you'll decide to disallow '+':
   * at least renice applet needs to allow it */
  if *p.offset(0) as libc::c_int == '+' as i32 || *p.offset(0) as libc::c_int == '-' as i32 {
    p = p.offset(1);
    if *p.offset(0) as libc::c_int == '-' as i32 {
      u = u.wrapping_add(1)
    }
    /* = <type>_MIN (01111... + 1 == 10000...) */
    /* = <type>_MIN (01111... + 1 == 10000...) */
  }
  r = xstrtou_range_sfx(p, base, 0i32 as libc::c_uint, u, suffixes) as libc::c_int;
  if *numstr as libc::c_int == '-' as i32 {
    r = -r
  }
  if r < lower || r > upper {
    bb_error_msg_and_die(
      b"number %s is not in %lld..%lld range\x00" as *const u8 as *const libc::c_char,
      numstr,
      lower as libc::c_longlong,
      upper as libc::c_longlong,
    );
  }
  return r;
}
#[no_mangle]
pub unsafe extern "C" fn xstrtoll_range_sfx(
  mut numstr: *const libc::c_char,
  mut base: libc::c_int,
  mut lower: libc::c_longlong,
  mut upper: libc::c_longlong,
  mut suffixes: *const suffix_mult,
) -> libc::c_longlong {
  let mut u: libc::c_ulonglong = 9223372036854775807i64 as libc::c_ulonglong;
  let mut r: libc::c_longlong = 0;
  let mut p: *const libc::c_char = numstr;
  if *p.offset(0) as libc::c_int == '+' as i32 || *p.offset(0) as libc::c_int == '-' as i32 {
    p = p.offset(1);
    if *p.offset(0) as libc::c_int == '-' as i32 {
      u = u.wrapping_add(1)
    }
  }
  r = xstrtoull_range_sfx(p, base, 0i32 as libc::c_ulonglong, u, suffixes) as libc::c_longlong;
  if *numstr as libc::c_int == '-' as i32 {
    r = -r
  }
  if r < lower || r > upper {
    bb_error_msg_and_die(
      b"number %s is not in %lld..%lld range\x00" as *const u8 as *const libc::c_char,
      numstr,
      lower,
      upper,
    );
  }
  return r;
}
#[no_mangle]
pub unsafe extern "C" fn xstrtoi_range(
  mut numstr: *const libc::c_char,
  mut base: libc::c_int,
  mut lower: libc::c_int,
  mut upper: libc::c_int,
) -> libc::c_int {
  return xstrtoi_range_sfx(numstr, base, lower, upper, 0 as *const suffix_mult);
}
#[no_mangle]
pub unsafe extern "C" fn xstrtoll_range(
  mut numstr: *const libc::c_char,
  mut base: libc::c_int,
  mut lower: libc::c_longlong,
  mut upper: libc::c_longlong,
) -> libc::c_longlong {
  return xstrtoll_range_sfx(numstr, base, lower, upper, 0 as *const suffix_mult);
}
#[no_mangle]
pub unsafe extern "C" fn xstrtoi(
  mut numstr: *const libc::c_char,
  mut base: libc::c_int,
) -> libc::c_int {
  return xstrtoi_range_sfx(
    numstr,
    base,
    -2147483647i32 - 1i32,
    2147483647i32,
    0 as *const suffix_mult,
  );
}
#[no_mangle]
pub unsafe extern "C" fn xstrtoll(
  mut numstr: *const libc::c_char,
  mut base: libc::c_int,
) -> libc::c_longlong {
  return xstrtoll_range_sfx(
    numstr,
    base,
    -9223372036854775807i64 - 1i64,
    9223372036854775807i64,
    0 as *const suffix_mult,
  );
}
#[no_mangle]
pub unsafe extern "C" fn xatoll_range_sfx(
  mut numstr: *const libc::c_char,
  mut lower: libc::c_longlong,
  mut upper: libc::c_longlong,
  mut suffixes: *const suffix_mult,
) -> libc::c_longlong {
  return xstrtoll_range_sfx(numstr, 10i32, lower, upper, suffixes);
}
#[no_mangle]
pub unsafe extern "C" fn xatoi_range_sfx(
  mut numstr: *const libc::c_char,
  mut lower: libc::c_int,
  mut upper: libc::c_int,
  mut suffixes: *const suffix_mult,
) -> libc::c_int {
  return xstrtoi_range_sfx(numstr, 10i32, lower, upper, suffixes);
}
#[no_mangle]
pub unsafe extern "C" fn xatoll_range(
  mut numstr: *const libc::c_char,
  mut lower: libc::c_longlong,
  mut upper: libc::c_longlong,
) -> libc::c_longlong {
  return xstrtoll_range_sfx(numstr, 10i32, lower, upper, 0 as *const suffix_mult);
}
#[no_mangle]
pub unsafe extern "C" fn xatoi_range(
  mut numstr: *const libc::c_char,
  mut lower: libc::c_int,
  mut upper: libc::c_int,
) -> libc::c_int {
  return xstrtoi_range_sfx(numstr, 10i32, lower, upper, 0 as *const suffix_mult);
}
#[no_mangle]
pub unsafe extern "C" fn xatoll_sfx(
  mut numstr: *const libc::c_char,
  mut suffixes: *const suffix_mult,
) -> libc::c_longlong {
  return xstrtoll_range_sfx(
    numstr,
    10i32,
    -9223372036854775807i64 - 1i64,
    9223372036854775807i64,
    suffixes,
  );
}
#[no_mangle]
pub unsafe extern "C" fn xatoi_sfx(
  mut numstr: *const libc::c_char,
  mut suffixes: *const suffix_mult,
) -> libc::c_int {
  return xstrtoi_range_sfx(
    numstr,
    10i32,
    -2147483647i32 - 1i32,
    2147483647i32,
    suffixes,
  );
}
#[no_mangle]
pub unsafe extern "C" fn xatoll(mut numstr: *const libc::c_char) -> libc::c_longlong {
  return xstrtoll_range_sfx(
    numstr,
    10i32,
    -9223372036854775807i64 - 1i64,
    9223372036854775807i64,
    0 as *const suffix_mult,
  );
}

/*
 * ascii-to-numbers implementations for busybox
 *
 * Copyright (C) 2003  Manuel Novoa III  <mjn3@codepoet.org>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* Provides extern declarations of functions */
/* Unsigned long long functions always exist */
/* Provides inline definitions of functions */
/* (useful for mapping them to the type of the same width) */
/* If long == long long, then just map them one-to-one */
/* Same for int -> [long] long */
#[no_mangle]
pub unsafe extern "C" fn xatoi(mut numstr: *const libc::c_char) -> libc::c_int {
  return xstrtoi_range_sfx(
    numstr,
    10i32,
    -2147483647i32 - 1i32,
    2147483647i32,
    0 as *const suffix_mult,
  );
}
#[inline(always)]
unsafe extern "C" fn bb_strtoui(
  mut str: *const libc::c_char,
  mut end: *mut *mut libc::c_char,
  mut b: libc::c_int,
) -> libc::c_uint {
  let mut v: libc::c_ulong = strtoul(str, end, b);
  if v
    > (2147483647i32 as libc::c_uint)
      .wrapping_mul(2u32)
      .wrapping_add(1u32) as libc::c_ulong
  {
    *bb_errno = 34i32;
    return (2147483647i32 as libc::c_uint)
      .wrapping_mul(2u32)
      .wrapping_add(1u32);
  }
  return v as libc::c_uint;
}
/* A few special cases */
#[no_mangle]
pub unsafe extern "C" fn xatoi_positive(mut numstr: *const libc::c_char) -> libc::c_int {
  return xatou_range(numstr, 0i32 as libc::c_uint, 2147483647i32 as libc::c_uint) as libc::c_int;
}

/*
 * Busybox main internal header file
 *
 * Based in part on code from sash, Copyright (c) 1999 by David I. Bell
 * Permission has been granted to redistribute this code under GPL.
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* TODO: and glibc? */
/* There are two incompatible basename's, let's not use them! */
/* See the dirname/basename man page for details */
/* dirname,basename */
/* Don't do this here:
 * #include <sys/sysinfo.h>
 * Some linux/ includes pull in conflicting definition
 * of struct sysinfo (only in some toolchanins), which breaks build.
 * Include sys/sysinfo.h only in those files which need it.
 */
/* Just in case libc doesn't define some of these... */
//This breaks on bionic:
//# if !defined(__socklen_t_defined) && !defined(_SOCKLEN_T_DECLARED)
// /* We #define socklen_t *after* includes, otherwise we get
// * typedef redefinition errors from system headers
// * (in case "is it defined already" detection above failed)
// */
//#  define socklen_t bb_socklen_t
//   typedef unsigned socklen_t;
//# endif
//if this is still needed, add a fix along the lines of
//  ifdef SPECIFIC_BROKEN_LIBC_CHECK / typedef socklen_t / endif
//in platform.h instead!
/*
 * Use '%m' to append error string on platforms that support it,
 * '%s' and strerror() on those that don't.
 */
/*nothing*/
/* Some libc's forget to declare these, do it ourself */
/* klogctl is in libc's klog.h, but we cheat and not #include that */
/* Busybox does not use threads, we can speed up stdio. */
/* Above functions are required by POSIX.1-2008, below ones are extensions */
/* musl <= 1.1.15 does not support fflush_unlocked(NULL) */
//# undef  fflush
//# define fflush(stream) fflush_unlocked(stream)
/* Make all declarations hidden (-fvisibility flag only affects definitions) */
/* (don't include system headers after this until corresponding pop!) */
/* Tested to work correctly with all int types (IIRC :]) */
/* Large file support */
/* Note that CONFIG_LFS=y forces bbox to be built with all common ops
 * (stat, lseek etc) mapped to "largefile" variants by libc.
 * Practically it means that open() automatically has O_LARGEFILE added
 * and all filesize/file_offset parameters and struct members are "large"
 * (in today's world - signed 64bit). For full support of large files,
 * we need a few helper #defines (below) and careful use of off_t
 * instead of int/ssize_t. No lseek64(), O_LARGEFILE etc necessary */
/* CONFIG_LFS is on */
/* "long" is long enough on this system */
/* usage: sz = BB_STRTOOFF(s, NULL, 10); if (errno || sz < 0) die(); */
/* usage: printf("size: %"OFF_FMT"d (%"OFF_FMT"x)\n", sz, sz); */
/* scary. better ideas? (but do *test* them first!) */
/* Users report bionic to use 32-bit off_t even if LARGEFILE support is requested.
 * We misdetected that. Don't let it build:
 */
/* Some useful definitions */
/* Macros for min/max.  */
/* buffer allocation schemes */
/* glibc uses __errno_location() to get a ptr to errno */
/* We can just memorize it once - no multithreading in busybox :) */
/* dmalloc will redefine these to it's own implementation. It is safe
 * to have the prototypes here unconditionally.  */
/* After v = xrealloc_vector(v, SHIFT, idx) it's ok to use
 * at least v[idx] and v[idx+1], for all idx values.
 * SHIFT specifies how many new elements are added (1:2, 2:4, ..., 8:256...)
 * when all elements are used up. New elements are zeroed out.
 * xrealloc_vector(v, SHIFT, idx) *MUST* be called with consecutive IDXs -
 * skipping an index is a bad bug - it may miss a realloc!
 */
//TODO: supply a pointer to char[11] buffer (avoid statics)?
/* cp.c, mv.c, install.c depend on these values. CAREFUL when changing them! */
/* -p */
/* !-d */
/* -R */
/* -f */
/* -i */
/* -l */
/* -s */
/* -L */
/* -H */
/* -a = -pdR (mapped in cp.c) */
/* -r = -dR  (mapped in cp.c) */
/* -P = -d   (mapped in cp.c) */
/* -v */
/* -u */
/* -T */
/* --remove-destination */
/* bit 17 skipped for "cp --parents" */
/* cp --reflink=auto */
/* cp --reflink[=always] */
/*
 * Hole. cp may have some bits set here,
 * they should not affect remove_file()/copy_file()
 */
/* NB: without FILEUTILS_RECUR in flags, it will basically "cat"
 * the source, not copy (unless "source" is a directory).
 * This makes "cp /dev/null file" and "install /dev/null file" (!!!)
 * work coreutils-compatibly. */
/*ACTION_REVERSE      = (1 << 4), - unused */
/* more than enough for "/dev/ttyXXX" */
/* bb_copyfd_XX print read/write errors and return -1 if they occur */
/* "short" copy can be detected by return value < size */
/* this helper yells "short read!" if param is not -1 */
/* xxxx_strip version can modify its parameter:
 * "/"        -> "/"
 * "abc"      -> "abc"
 * "abc/def"  -> "def"
 * "abc/def/" -> "def" !!
 */
/* "abc/def/" -> "" and it never modifies 'path' */
/* Simpler version: does not special case "/" string */
/* NB: can violate const-ness (similarly to strchr) */
/* !RETURNS_MALLOC: it's a realloc-like function */
/* bb_signals(BB_FATAL_SIGS, handler) catches all signals which
 * otherwise would kill us, except for those resulting from bugs:
 * SIGSEGV, SIGILL, SIGFPE.
 * Other fatal signals not included (TODO?):
 * SIGBUS   Bus error (bad memory access)
 * SIGPOLL  Pollable event. Synonym of SIGIO
 * SIGPROF  Profiling timer expired
 * SIGSYS   Bad argument to routine
 * SIGTRAP  Trace/breakpoint trap
 *
 * The only known arch with some of these sigs not fitting
 * into 32 bits is parisc (SIGXCPU=33, SIGXFSZ=34, SIGSTKFLT=36).
 * Dance around with long long to guard against that...
 */
// Write to pipe with no readers
// Quit from keyboard
// Abort signal from abort(3)
// Timer signal from alarm(2)
// Virtual alarm clock
// CPU time limit exceeded
// File size limit exceeded
// Yes kids, these are also fatal!
/* Unlike signal() and bb_signals, sets handler with sigaction()
 * and in a way that while signal handler is run, no other signals
 * will be blocked; syscalls will not be restarted: */
/* syscalls like read() will be interrupted with EINTR: */
/* syscalls like read() won't be interrupted (though select/poll will be): */
/* Will do sigaction(signum, act, NULL): */
/* SIG_BLOCK/SIG_UNBLOCK all signals: */
/* Return old set in the same set: */
/* Standard handler which just records signo */
/* not FAST_FUNC! */
/* In this form code with pipes is much more readable */
/* Useful for having small structure members/global variables */
/* | AF_DECnet */
/* | AF_IPX */
/* SO_REUSEADDR allows a server to rebind to an address that is already
 * "in use" by old connections to e.g. previous server instance which is
 * killed or crashed. Without it bind will fail until all such connections
 * time out. Linux does not allow multiple live binds on same ip:port
 * regardless of SO_REUSEADDR (unlike some other flavors of Unix).
 * Turn it on before you call bind(). */
/* On Linux this never fails. */
/* NB: returns port in host byte order */
/* Create stream socket, and allocate suitable lsa.
 * (lsa of correct size and lsa->sa.sa_family (AF_INET/AF_INET6))
 * af == AF_UNSPEC will result in trying to create IPv6 socket,
 * and if kernel doesn't support it, fall back to IPv4.
 * This is useful if you plan to bind to resulting local lsa.
 */
/* Create server socket bound to bindaddr:port. bindaddr can be NULL,
 * numeric IP ("N.N.N.N") or numeric IPv6 address,
 * and can have ":PORT" suffix (for IPv6 use "[X:X:...:X]:PORT").
 * Only if there is no suffix, port argument is used */
/* NB: these set SO_REUSEADDR before bind */
/* Create client TCP socket connected to peer:port. Peer cannot be NULL.
 * Peer can be numeric IP ("N.N.N.N"), numeric IPv6 address or hostname,
 * and can have ":PORT" suffix (for IPv6 use "[X:X:...:X]:PORT").
 * If there is no suffix, port argument is used */
/* Connect to peer identified by lsa */
/* Get local address of bound or accepted socket */
/* Get remote address of connected or accepted socket */
/* Return malloc'ed len_and_sockaddr with socket address of host:port
 * Currently will return IPv4 or IPv6 sockaddrs only
 * (depending on host), but in theory nothing prevents e.g.
 * UNIX socket address being returned, IPX sockaddr etc...
 * On error does bb_error_msg and returns NULL */
/* Version which dies on error */
/* Same, useful if you want to force family (e.g. IPv6) */
/* Assign sin[6]_port member if the socket is an AF_INET[6] one,
 * otherwise no-op. Useful for ftp.
 * NB: does NOT do htons() internally, just direct assignment. */
/* Retrieve sin[6]_port or return -1 for non-INET[6] lsa's */
/* Reverse DNS. Returns NULL on failure. */
/* This one doesn't append :PORTNUM */
/* This one also doesn't fall back to dotted IP (returns NULL) */
/* inet_[ap]ton on steroids */
// "old" (ipv4 only) API
// users: traceroute.c hostname.c - use _list_ of all IPs
// Also mount.c and inetd.c are using gethostbyname(),
// + inet_common.c has additional IPv4-only stuff
/* opaque */
// RFC 5246
// sequence number
//   Each connection state contains a sequence number, which is
//   maintained separately for read and write states.  The sequence
//   number MUST be set to zero whenever a connection state is made the
//   active state.  Sequence numbers are of type uint64 and may not
//   exceed 2^64-1.
/*uint64_t read_seq64_be;*/
/*uint8_t *server_write_MAC_key;*/
//used by AES_GCM
/* 0 if argv[0] is NULL: */
/* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
 * But potentially slow, don't use in one-billion-times loops */
/* Note: does not use stdio, writes to fd 2 directly */
// gcc-4.1.1 still isn't good enough at optimizing it
// (+200 bytes compared to macro)
//static ALWAYS_INLINE
//int LONE_DASH(const char *s) { return s[0] == '-' && !s[1]; }
//static ALWAYS_INLINE
//int NOT_LONE_DASH(const char *s) { return s[0] != '-' || s[1]; }
/* Returns a string with unprintable chars replaced by '?' or
 * SUBST_WCHAR. This function is unicode-aware. */
/* Prints unprintable char ch as ^C or M-c to file
 * (M-c is used only if ch is ORed with PRINTABLE_META),
 * else it is printed as-is (except for ch = 0x9b) */
/* Return a string that is the printable representation of character ch.
 * Buffer must hold at least four characters. */
// NB: will return short read on error, not -1,
// if some data was read before error occurred
// Reads one line a-la fgets (but doesn't save terminating '\n').
// Reads byte-by-byte. Useful when it is important to not read ahead.
// Bytes are appended to pfx (which must be malloced, or NULL).
/* Reads block up to *maxsz_p (default: INT_MAX - 4095) */
/* Returns NULL if file can't be opened (default max size: INT_MAX - 4095) */
/* Never returns NULL */
/* Else use variable one (a bit more expensive) */
/* Autodetects gzip/bzip2 formats. fd may be in the middle of the file! */
/* Autodetects .gz etc */
/* lzma has no signature, need a little helper. NB: exist only for ENABLE_FEATURE_SEAMLESS_LZMA=y */
// NB: will return short write on error, not -1,
// if some data was written before error occurred
/* Close fd, but check for failures (some types of write errors) */
/* Reads and prints to stdout till eof, then closes FILE. Exits on error: */
/* Reads a line from a text file, up to a newline or NUL byte, inclusive.
 * Returns malloc'ed char*. If end is NULL '\n' isn't considered
 * end of line. If end isn't NULL, length of the chunk is stored in it.
 * Returns NULL if EOF/error.
 */
/* Reads up to (and including) TERMINATING_STRING: */
/* Same, with limited max size, and returns the length (excluding NUL): */
/* Chops off TERMINATING_STRING from the end: */
/* Reads up to (and including) "\n" or NUL byte: */
/* Chops off '\n' from the end, unlike fgets: */
/* Same, but doesn't try to conserve space (may have some slack after the end) */
/* extern char *xmalloc_fgetline_fast(FILE *file) FAST_FUNC RETURNS_MALLOC; */
/* Prints warning to stderr and returns NULL on failure: */
/* "Opens" stdin if filename is special, else just opens file: */
/* not FAST_FUNC! */
/* Wrapper which restarts poll on EINTR or ENOMEM.
 * On other errors complains [perror("poll")] and returns.
 * Warning! May take (much) longer than timeout_ms to return!
 * If this is a problem, use bare poll and open-code EINTR/ENOMEM handling */
/* Convert each alpha char in str to lower-case */
/* Returns a pointer past the formatted number, does NOT null-terminate */
/* Intelligent formatters of bignums */
/* If block_size == 0, display size without fractional part,
 * else display (size * block_size) with one decimal digit.
 * If display_unit == 0, show value no bigger than 1024 with suffix (K,M,G...),
 * else divide by display_unit and do not use suffix. */
/* "1024.0G" */
//TODO: provide pointer to buf (avoid statics)?
/* Put a string of hex bytes ("1b2e66fe"...), return advanced pointer */
/* Reverse */
/* Generate a UUID */
/* Last element is marked by mult == 0 */
/* Specialized: */
/* Using xatoi() instead of naive atoi() is not always convenient -
 * in many places people want *non-negative* values, but store them
 * in signed int. Therefore we need this one:
 * dies if input is not in [0, INT_MAX] range. Also will reject '-0' etc.
 * It should really be named xatoi_nonnegative (since it allows 0),
 * but that would be too long.
 */
/* Useful for reading port numbers */
#[no_mangle]
pub unsafe extern "C" fn xatou16(mut numstr: *const libc::c_char) -> uint16_t {
  return xatou_range(numstr, 0i32 as libc::c_uint, 0xffffi32 as libc::c_uint) as uint16_t;
}
#[no_mangle]
pub static mut bkm_suffixes: [suffix_mult; 4] = [
  {
    let mut init = suffix_mult {
      suffix: [98, 0, 0, 0],
      mult: 512i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [107, 0, 0, 0],
      mult: 1024i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [109, 0, 0, 0],
      mult: (1024i32 * 1024i32) as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [0, 0, 0, 0],
      mult: 0i32 as libc::c_uint,
    };
    init
  },
];
#[no_mangle]
pub static mut cwbkMG_suffixes: [suffix_mult; 16] = [
  {
    let mut init = suffix_mult {
      suffix: [99, 0, 0, 0],
      mult: 1i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [119, 0, 0, 0],
      mult: 2i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [98, 0, 0, 0],
      mult: 512i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [107, 66, 0, 0],
      mult: 1000i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [107, 68, 0, 0],
      mult: 1000i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [107, 0, 0, 0],
      mult: 1024i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [75, 66, 0, 0],
      mult: 1000i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [75, 68, 0, 0],
      mult: 1000i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [75, 0, 0, 0],
      mult: 1024i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [77, 66, 0, 0],
      mult: 1000000i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [77, 68, 0, 0],
      mult: 1000000i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [77, 0, 0, 0],
      mult: (1024i32 * 1024i32) as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [71, 66, 0, 0],
      mult: 1000000000i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [71, 68, 0, 0],
      mult: 1000000000i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [71, 0, 0, 0],
      mult: (1024i32 * 1024i32 * 1024i32) as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [0, 0, 0, 0],
      mult: 0i32 as libc::c_uint,
    };
    init
  },
];
#[no_mangle]
pub static mut kmg_i_suffixes: [suffix_mult; 16] = [
  {
    let mut init = suffix_mult {
      suffix: [75, 105, 66, 0],
      mult: 1024i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [107, 105, 66, 0],
      mult: 1024i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [75, 0, 0, 0],
      mult: 1024i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [107, 0, 0, 0],
      mult: 1024i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [77, 105, 66, 0],
      mult: 1048576i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [109, 105, 66, 0],
      mult: 1048576i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [77, 0, 0, 0],
      mult: 1048576i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [109, 0, 0, 0],
      mult: 1048576i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [71, 105, 66, 0],
      mult: 1073741824i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [103, 105, 66, 0],
      mult: 1073741824i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [71, 0, 0, 0],
      mult: 1073741824i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [103, 0, 0, 0],
      mult: 1073741824i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [75, 66, 0, 0],
      mult: 1000i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [77, 66, 0, 0],
      mult: 1000000i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [71, 66, 0, 0],
      mult: 1000000000i32 as libc::c_uint,
    };
    init
  },
  {
    let mut init = suffix_mult {
      suffix: [0, 0, 0, 0],
      mult: 0i32 as libc::c_uint,
    };
    init
  },
];
