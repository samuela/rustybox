use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn getline(__lineptr: *mut *mut libc::c_char, __n: *mut size_t, __stream: *mut FILE)
    -> __ssize_t;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
  #[no_mangle]
  fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
  #[no_mangle]
  fn skip_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn fopen_or_warn_stdin(filename: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn xfunc_die() -> !;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
}
pub type __uint8_t = libc::c_uchar;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;



use crate::librb::FILE;
pub type C2RustUnnamed = libc::c_uint;
pub const PARSE_NORMAL: C2RustUnnamed = 4653056;
pub const PARSE_WS_COMMENTS: C2RustUnnamed = 16777216;
pub const PARSE_ALT_COMMENTS: C2RustUnnamed = 8388608;
pub const PARSE_EOL_COMMENTS: C2RustUnnamed = 4194304;
pub const PARSE_KEEP_COPY: C2RustUnnamed = 2097152;
pub const PARSE_MIN_DIE: C2RustUnnamed = 1048576;
pub const PARSE_GREEDY: C2RustUnnamed = 262144;
pub const PARSE_TRIM: C2RustUnnamed = 131072;
pub const PARSE_COLLAPSE: C2RustUnnamed = 65536;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parser_t {
  pub fp: *mut FILE,
  pub data: *mut libc::c_char,
  pub line: *mut libc::c_char,
  pub nline: *mut libc::c_char,
  pub line_alloc: size_t,
  pub nline_alloc: size_t,
  pub lineno: libc::c_int,
}
/* vi: set sw=4 ts=4: */
/*
 * config file parser helper
 *
 * Copyright (C) 2008 by Vladimir Dronnikov <dronnikov@gmail.com>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 * Also for use in uClibc (http://uclibc.org/) licensed under LGPLv2.1 or later.
 */
/* Uncomment to enable test applet */
// //config:config PARSE
// //config:	bool "Uniform config file parser debugging applet: parse"
// //config:	default n
// //config:	help
// //config:	Typical usage of parse API:
// //config:		char *t[3];
// //config:		parser_t *p = config_open(filename);
// //config:		while (config_read(p, t, 3, 0, delimiters, flags)) { // 1..3 tokens
// //config:			bb_error_msg("TOKENS: '%s''%s''%s'", t[0], t[1], t[2]);
// //config:		}
// //config:		config_close(p);
// //applet:IF_PARSE(APPLET(parse, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-y += parse_config.o
//usage:#define parse_trivial_usage
//usage:       "[-x] [-n MAXTOKENS] [-m MINTOKENS] [-d DELIMS] [-f FLAGS] FILE..."
//usage:#define parse_full_usage "\n\n"
//usage:       "	-x	Suppress output (for benchmarking)"
#[no_mangle]
pub unsafe extern "C" fn config_open2(
  mut filename: *const libc::c_char,
  mut fopen_func: Option<unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE>,
) -> *mut parser_t {
  let mut fp: *mut FILE = 0 as *mut FILE;
  let mut parser: *mut parser_t = 0 as *mut parser_t;
  fp = fopen_func.expect("non-null function pointer")(filename);
  if fp.is_null() {
    return 0 as *mut parser_t;
  }
  parser = xzalloc(::std::mem::size_of::<parser_t>() as libc::c_ulong) as *mut parser_t;
  (*parser).fp = fp;
  return parser;
}
#[no_mangle]
pub unsafe extern "C" fn config_open(mut filename: *const libc::c_char) -> *mut parser_t {
  return config_open2(
    filename,
    Some(fopen_or_warn_stdin as unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE),
  );
}
#[no_mangle]
pub unsafe extern "C" fn config_close(mut parser: *mut parser_t) {
  if !parser.is_null() {
    if PARSE_KEEP_COPY as libc::c_int != 0 {
      /* compile-time constant */
      free((*parser).data as *mut libc::c_void);
    }
    fclose((*parser).fp);
    free((*parser).line as *mut libc::c_void);
    free((*parser).nline as *mut libc::c_void);
    free(parser as *mut libc::c_void);
  };
}
/* This function reads an entire line from a text file,
 * up to a newline, exclusive.
 * Trailing '\' is recognized as line continuation.
 * Returns -1 if EOF/error.
 */
unsafe extern "C" fn get_line_with_continuation(mut parser: *mut parser_t) -> libc::c_int {
  let mut len: ssize_t = 0;
  let mut nlen: ssize_t = 0;
  let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
  len = getline(&mut (*parser).line, &mut (*parser).line_alloc, (*parser).fp);
  if len <= 0i32 as libc::c_long {
    return len as libc::c_int;
  }
  line = (*parser).line;
  loop {
    (*parser).lineno += 1;
    if *line.offset((len - 1i32 as libc::c_long) as isize) as libc::c_int == '\n' as i32 {
      len -= 1
    }
    if len == 0i32 as libc::c_long
      || *line.offset((len - 1i32 as libc::c_long) as isize) as libc::c_int != '\\' as i32
    {
      break;
    }
    len -= 1;
    nlen = getline(
      &mut (*parser).nline,
      &mut (*parser).nline_alloc,
      (*parser).fp,
    );
    if nlen <= 0i32 as libc::c_long {
      break;
    }
    if (*parser).line_alloc < (len + nlen + 1i32 as libc::c_long) as libc::c_ulong {
      (*parser).line_alloc = (len + nlen + 1i32 as libc::c_long) as size_t;
      (*parser).line =
        xrealloc(line as *mut libc::c_void, (*parser).line_alloc) as *mut libc::c_char;
      line = (*parser).line
    }
    memcpy(
      &mut *line.offset(len as isize) as *mut libc::c_char as *mut libc::c_void,
      (*parser).nline as *const libc::c_void,
      nlen as libc::c_ulong,
    );
    len += nlen
  }
  *line.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
  return len as libc::c_int;
}
/* vi: set sw=4 ts=4: */
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
/*
0. If parser is NULL return 0.
1. Read a line from config file. If nothing to read then return 0.
   Handle continuation character. Advance lineno for each physical line.
   Discard everything past comment character.
2. if PARSE_TRIM is set (default), remove leading and trailing delimiters.
3. If resulting line is empty goto 1.
4. Look for first delimiter. If !PARSE_COLLAPSE or !PARSE_TRIM is set then
   remember the token as empty.
5. Else (default) if number of seen tokens is equal to max number of tokens
   (token is the last one) and PARSE_GREEDY is set then the remainder
   of the line is the last token.
   Else (token is not last or PARSE_GREEDY is not set) just replace
   first delimiter with '\0' thus delimiting the token.
6. Advance line pointer past the end of token. If number of seen tokens
   is less than required number of tokens then goto 4.
7. Check the number of seen tokens is not less the min number of tokens.
   Complain or die otherwise depending on PARSE_MIN_DIE.
8. Return the number of seen tokens.

mintokens > 0 make config_read() print error message if less than mintokens
(but more than 0) are found. Empty lines are always skipped (not warned about).
*/
#[no_mangle]
pub unsafe extern "C" fn config_read(
  mut parser: *mut parser_t,
  mut tokens: *mut *mut libc::c_char,
  mut flags: libc::c_uint,
  mut delims: *const libc::c_char,
) -> libc::c_int {
  let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut ntokens: libc::c_int = 0;
  let mut mintokens: libc::c_int = 0;
  let mut t: libc::c_int = 0;
  let mut alt_comment_ch: libc::c_char = 0;
  if parser.is_null() {
    return 0i32;
  }
  alt_comment_ch = '\u{0}' as i32 as libc::c_char;
  if flags & PARSE_ALT_COMMENTS as libc::c_int as libc::c_uint != 0 {
    let fresh0 = delims;
    delims = delims.offset(1);
    alt_comment_ch = *fresh0
  }
  ntokens = flags as uint8_t as libc::c_int;
  mintokens = (flags >> 8i32) as uint8_t as libc::c_int;
  loop {
    memset(
      tokens as *mut libc::c_void,
      0i32,
      (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
        .wrapping_mul(ntokens as libc::c_ulong),
    );
    /* Read one line (handling continuations with backslash) */
    if get_line_with_continuation(parser) < 0i32 {
      return 0i32;
    }
    line = (*parser).line;
    /* Skip token in the start of line? */
    if flags & PARSE_TRIM as libc::c_int as libc::c_uint != 0 {
      line = line.offset(strspn(line, delims.offset(1)) as isize)
    }
    p = line;
    if flags & PARSE_WS_COMMENTS as libc::c_int as libc::c_uint != 0 {
      p = skip_whitespace(p)
    }
    if *p.offset(0) as libc::c_int == '\u{0}' as i32
      || *p.offset(0) as libc::c_int == *delims.offset(0) as libc::c_int
      || *p.offset(0) as libc::c_int == alt_comment_ch as libc::c_int
    {
      continue;
    }
    if flags & PARSE_KEEP_COPY as libc::c_int as libc::c_uint != 0 {
      free((*parser).data as *mut libc::c_void);
      (*parser).data = xstrdup(line)
    }
    /* Tokenize the line */
    t = 0i32;
    loop {
      /* Pin token */
      let ref mut fresh1 = *tokens.offset(t as isize);
      *fresh1 = line;
      /* Combine remaining arguments? */
      if t != ntokens - 1i32 || flags & PARSE_GREEDY as libc::c_int as libc::c_uint == 0 {
        /* Vanilla token, find next delimiter */
        line = line.offset(strcspn(
          line,
          if *delims.offset(0) as libc::c_int != 0
            && flags & PARSE_EOL_COMMENTS as libc::c_int as libc::c_uint != 0
          {
            delims
          } else {
            delims.offset(1)
          },
        ) as isize)
      } else {
        /* Combining, find comment char if any */
        line = strchrnul(
          line,
          if flags & PARSE_EOL_COMMENTS as libc::c_int as libc::c_uint != 0 {
            *delims.offset(0) as libc::c_int
          } else {
            '\u{0}' as i32
          },
        );
        /* Trim any extra delimiters from the end */
        if flags & PARSE_TRIM as libc::c_int as libc::c_uint != 0 {
          while !strchr(
            delims.offset(1),
            *line.offset(-1i32 as isize) as libc::c_int,
          )
          .is_null()
          {
            line = line.offset(-1)
          }
        }
      }
      /* Token not terminated? */
      if flags & PARSE_EOL_COMMENTS as libc::c_int as libc::c_uint != 0
        && *line as libc::c_int == *delims.offset(0) as libc::c_int
      {
        /* token is done, continue parsing line */
        *line = '\u{0}' as i32 as libc::c_char
      } else if *line as libc::c_int != '\u{0}' as i32 {
        let fresh2 = line; /* ends with comment char: this line is done */
        line = line.offset(1);
        *fresh2 = '\u{0}' as i32 as libc::c_char
      }
      /* unused so far */
      /* Skip possible delimiters */
      if flags & PARSE_COLLAPSE as libc::c_int as libc::c_uint != 0 {
        line = line.offset(strspn(line, delims.offset(1)) as isize)
      }
      t += 1;
      if !(*line as libc::c_int != 0
        && *line as libc::c_int != *delims.offset(0) as libc::c_int
        && t < ntokens)
      {
        break;
      }
    }
    if !(t < mintokens) {
      break;
    }
    bb_error_msg(
      b"bad line %u: %d tokens found, %d needed\x00" as *const u8 as *const libc::c_char,
      (*parser).lineno,
      t,
      mintokens,
    );
    if flags & PARSE_MIN_DIE as libc::c_int as libc::c_uint != 0 {
      xfunc_die();
    }
  }
  return t;
}
