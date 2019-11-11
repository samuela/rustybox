

use libc;
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
/*u8 *server_write_MAC_key;*/
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
/* We need to export XXX_main from libbusybox
 * only if we build "individual" binaries
 */
/* Embedded script support */
/* Applets which are useful from another applets */
/* If shell needs them, they exist even if not enabled as applets */
/* Similar, but used by chgrp, not shell */
/* Used by ftpd */
/* Don't need IF_xxx() guard for these */
/* Networking */
/* This structure defines protocol families and their handlers. */
/*int type,*/
/* may modify src */
/* This structure defines hardware protocols and their handlers. */
/*
 * If *devname is not NULL, use that name, otherwise try to find free one,
 * malloc and return it in *devname.
 * return value is the opened fd to the loop device, or < on error
 */
/* These constants match linux/loop.h (without BB_ prefix): */
/* Returns malloced str */
/* Like bb_ask_noecho, but asks on stdin with no timeout.  */
/* Returns -1 if input is invalid. current_mode is a base for e.g. "u+rw" */
/*
 * Config file parser
 */
// treat consecutive delimiters as one
// trim leading and trailing delimiters
// TODO: COLLAPSE and TRIM seem to always go in pair
// last token takes entire remainder of the line
// die if < min tokens found
// keep a copy of current line
// comments are recognized even if they aren't the first char
// delim[0] and delim[1] are two different allowed comment chars
// (so far, delim[0] will only work as comment char for full-line comment)
// (IOW: it works as if PARSE_EOL_COMMENTS is not set. sysctl applet is okay with this)
// comments are recognized even if there is whitespace before
// ("line start><space><tab><space>#comment" is also comment, not only "line start>#comment")
// NORMAL is:
// * remove leading and trailing delimiters and collapse
//   multiple delimiters into one
// * warn and continue if less than mintokens delimiters found
// * grab everything into last token
// * comments are recognized even if they aren't the first char
/* delims[0] is a comment char (use '\0' to disable), the rest are token delimiters */
/* Concatenate path and filename to new allocated buffer.
 * Add "/" only as needed (no duplicate "//" are produced).
 * If path is NULL, it is assumed to be "/".
 * filename should not be NULL. */
/* Returns NULL on . and .. */
/* Returns ptr to NUL */
/* Returns $SHELL, getpwuid(getuid())->pw_shell, or DEFAULT_SHELL.
 * Note that getpwuid result might need xstrdup'ing
 * if there is a possibility of intervening getpwxxx() calls.
 */
/* Structures inside "struct caps" are Linux-specific and libcap-specific: */
/* setup_environment:
 * if chdir pw->pw_dir: ok: else if to_tmp == 1: goto /tmp else: goto / or die
 * if clear_env = 1: cd(pw->pw_dir), clear environment, then set
 *   TERM=(old value)
 *   USER=pw->pw_name, LOGNAME=pw->pw_name
 *   PATH=bb_default_[root_]path
 *   HOME=pw->pw_dir
 *   SHELL=shell
 * else if change_env = 1:
 *   if not root (if pw->pw_uid != 0):
 *     USER=pw->pw_name, LOGNAME=pw->pw_name
 *   HOME=pw->pw_dir
 *   SHELL=shell
 * else does nothing
 *
 * NB: CHANGEENV and CLEARENV use setenv() - this leaks memory!
 * If setup_environment() is used is vforked child, this leaks memory _in parent too_!
 */
/* Returns a malloced string */
/*
 * rnd is additional random input. New one is returned.
 * Useful if you call crypt_make_salt many times in a row:
 * rnd = crypt_make_salt(buf1, 4, 0);
 * rnd = crypt_make_salt(buf2, 4, rnd);
 * rnd = crypt_make_salt(buf3, 4, rnd);
 * (otherwise we risk having same salt generated)
 */
/*, int rnd*/
/* "$N$" + sha_salt_16_bytes + NUL */
/* Returns number of lines changed, or -1 on error */
/* NB: typically you want to pass fd 0, not 1. Think 'applet | grep something' */
/* NB: "unsigned request" is crucial! "int request" will break some arches! */
/* At least glibc has horrendously large inline for this, so wrap it */
/* "Keycodes" that report an escape sequence.
 * We use something which fits into signed char,
 * yet doesn't represent any valid Unicode character.
 * Also, -1 is reserved for error indication and we don't use it. */
/* Used only if Alt/Ctrl/Shifted */
/* Used only if Alted */
/* ^^^^^ Be sure that last defined value is small enough.
 * Current read_key() code allows going up to -32 (0xfff..fffe0).
 * This gives three upper bits in LSB to play with:
 * KEYCODE_foo values are 0xfff..fffXX, lowest XX bits are: scavvvvv,
 * s=0 if SHIFT, c=0 if CTRL, a=0 if ALT,
 * vvvvv bits are the same for same key regardless of "shift bits".
 */
//KEYCODE_SHIFT_...   = KEYCODE_...   & ~0x80,
/* 0xfff..fff00 */
/* How long is the longest ESC sequence we know?
 * We want it big enough to be able to contain
 * cursor position sequence "ESC [ 9999 ; 9999 R"
 */
/* Note: fd may be in blocking or non-blocking mode, both make sense.
 * For one, less uses non-blocking mode.
 * Only the first read syscall inside read_key may block indefinitely
 * (unless fd is in non-blocking mode),
 * subsequent reads will time out after a few milliseconds.
 * Return of -1 means EOF or error (errno == 0 on EOF).
 * buffer[0] is used as a counter of buffered chars and must be 0
 * on first call.
 * timeout:
 * -2: do not poll(-1) for input - read() it, return on EAGAIN at once
 * -1: poll(-1) (i.e. block even on NONBLOCKed fd)
 * >=0: poll() for TIMEOUT milliseconds, return -1/EAGAIN on timeout
 */
/* It's NOT just ENABLEd or disabled. It's a number: */
/* must never be <= 0 */
/* meaning of this field depends on FEATURE_EDITING_SAVE_ON_EXIT:
 * if !FEATURE_EDITING_SAVE_ON_EXIT: "how many lines are
 * in on-disk history"
 * if FEATURE_EDITING_SAVE_ON_EXIT: "how many in-memory lines are
 * also in on-disk history (and thus need to be skipped on save)"
 */
/*
 * maxsize must be >= 2.
 * Returns:
 * -1 on read errors or EOF, or on bare Ctrl-D,
 * 0  on ctrl-C (the line entered is still returned in 'command'),
 * >0 length of input string, including terminating '\n'
 */
/* synchronize with sizeof(task_struct.comm) in /usr/include/linux/sched.h */
// For mixed 32/64 userspace, 32-bit pmap still needs
// 64-bit field here to correctly show 64-bit processes:
// (strictly speaking, other fields need to be wider too,
// but they are in kbytes, not bytes, and they hold sizes,
// not start addresses, sizes tend to be less than 4 terabytes)
/* Fields are set to 0/NULL if failed to determine (or not requested) */
/* Everything below must contain no ptrs to malloc'ed data:
 * it is memset(0) for each process in procps_scan() */
/* we round it to kbytes */
/* basename of executable in exec(2), read from /proc/N/stat
 * (if executable is symlink or script, it is NOT replaced
 * by link target or interpreter name) */
/* user/group? - use passwd/group parsing functions */
/* flag bits for procps_scan(xx, flags) calls */
/* PSSCAN_CMD      = 1 << 6, - use read_cmdline instead */
/* NB: used by find_pid_by_name(). Any applet using it
 * needs to be mentioned here. */
//procps_status_t* alloc_procps_scan(void) FAST_FUNC;
/* Format cmdline (up to col chars) into char buf[size] */
/* Puts [comm] if cmdline is empty (-> process is a kernel thread) */
/* Use strict=1 if you process input from untrusted source:
 * it will return NULL on invalid %xx (bad hex chars)
 * and str + 1 if decoded char is / or NUL.
 * In non-strict mode, it always succeeds (returns str),
 * and also it additionally decoded '+' to space.
 */
/* Sign-extends to a value which never matches fgetc result: */
/* always correctly aligned for uint64_t */
/* must be directly before hash[] */
/* 4 elements for md5, 5 for sha1, 8 for sha256 */
/* must be directly before hash[] */
/* always correctly aligned for uint64_t */
/* TLS benefits from knowing that sha1 and sha256 share these. Give them "agnostic" names too */
/*unsigned last_eta;*/
/* Some older linkers don't perform string merging, we used to have common strings
 * as global arrays to do it by hand. But:
 * (1) newer linkers do it themselves,
 * (2) however, they DONT merge string constants with global arrays,
 * even if the value is the same (!). Thus global arrays actually
 * increased size a bit: for example, "/etc/passwd" string from libc
 * wasn't merged with bb_path_passwd_file[] array!
 * Therefore now we use #defines.
 */
/* "BusyBox vN.N.N (timestamp or extra_version)" */
/* NB: (bb_hexdigits_upcase[i] | 0x20) -> lowercase hex digit */
/* Busybox mount uses either /proc/mounts or /etc/mtab to
 * get the list of currently mounted filesystems */
/* allow default system PATH to be extended via CFLAGS */
/* BB_PATH_ROOT_PATH */
/* util-linux manpage says /sbin:/bin:/usr/sbin:/usr/bin,
 * but I want to save a few bytes here:
 */
//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */
/* '*const' ptr makes gcc optimize code much better.
 * Magic prevents ptr_to_globals from going into rodata.
 * If you want to assign a value, use SET_PTR_TO_GLOBALS(x) */
/* At least gcc 3.4.6 on mipsel system needs optimization barrier */
/* You can change LIBBB_DEFAULT_LOGIN_SHELL, but don't use it,
 * use bb_default_login_shell and following defines.
 * If you change LIBBB_DEFAULT_LOGIN_SHELL,
 * don't forget to change increment constant. */
/* "/bin/sh" */
/* "sh" */
/* The following devices are the same on all systems.  */
/*Linux, normal names */
// storage helpers for mk*fs utilities
/* We redefine ctype macros. Unicode-correct handling of char types
 * can't be done with such byte-oriented operations anyway,
 * we don't lose anything.
 */
/* We save ~500 bytes on isdigit alone.
 * BTW, x86 likes (unsigned char) cast more than (unsigned). */
/* These work the same for ASCII and Unicode,
 * assuming no one asks "is this a *Unicode* letter?" using isalpha(letter) */
/* In POSIX/C locale isspace is only these chars: "\t\n\v\f\r" and space.
 * "\t\n\v\f\r" happen to have ASCII codes 9,10,11,12,13.
 */
// Unsafe wrt NUL: #define ispunct(a) (strchr("!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~", (a)) != NULL)
// Bigger code: #define isalnum(a) ({ unsigned char bb__isalnum = (a) - '0'; bb__isalnum <= 9 || ((bb__isalnum - ('A' - '0')) & 0xdf) <= 25; })
#[inline(always)]
unsafe extern "C" fn bb_ascii_isalnum(mut a: libc::c_uchar) -> libc::c_int {
  let mut b: libc::c_uchar = (a as libc::c_int - '0' as i32) as libc::c_uchar;
  if b as libc::c_int <= 9i32 {
    return (b as libc::c_int <= 9i32) as libc::c_int;
  }
  b = ((a as libc::c_int | 0x20i32) - 'a' as i32) as libc::c_uchar;
  return (b as libc::c_int <= 'z' as i32 - 'a' as i32) as libc::c_int;
}
/*
 * Utility routines.
 *
 * Copyright (C) 2013 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//kbuild:lib-y += endofname.o
#[no_mangle]
pub unsafe extern "C" fn endofname(mut name: *const libc::c_char) -> *const libc::c_char {
  if !(*name as libc::c_int == '_' as i32
    || ((*name as libc::c_uchar as libc::c_int | 0x20i32) - 'a' as i32) as libc::c_uchar
      as libc::c_int
      <= 'z' as i32 - 'a' as i32)
  {
    return name;
  }
  loop {
    name = name.offset(1);
    if !(*name != 0) {
      break;
    }
    if !(*name as libc::c_int == '_' as i32 || bb_ascii_isalnum(*name as libc::c_uchar) != 0) {
      break;
    }
  }
  return name;
}
