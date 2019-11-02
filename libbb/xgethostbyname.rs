use libc;
extern "C" {
  #[no_mangle]
  fn gethostbyname(__name: *const libc::c_char) -> *mut hostent;
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
  fn bb_simple_herror_msg_and_die(s: *const libc::c_char) -> !;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostent {
  pub h_name: *mut libc::c_char,
  pub h_aliases: *mut *mut libc::c_char,
  pub h_addrtype: libc::c_int,
  pub h_length: libc::c_int,
  pub h_addr_list: *mut *mut libc::c_char,
}
// "old" (ipv4 only) API
// users: traceroute.c hostname.c - use _list_ of all IPs
/* vi: set sw=4 ts=4: */
/*
 * Mini xgethostbyname implementation.
 *
 * Copyright (C) 2001 Matt Kraai <kraai@alumni.carnegiemellon.edu>.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn xgethostbyname(mut name: *const libc::c_char) -> *mut hostent {
  let mut retval: *mut hostent = gethostbyname(name);
  if retval.is_null() {
    bb_simple_herror_msg_and_die(name);
  }
  return retval;
}
