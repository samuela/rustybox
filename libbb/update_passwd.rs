use crate::librb::__mode_t;
use crate::librb::__off64_t;
use crate::librb::__pid_t;
use crate::librb::__useconds_t;
use crate::librb::size_t;
use libc;
use libc::gid_t;
use libc::stat;
use libc::time_t;
use libc::uid_t;
use libc::FILE;

extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn link(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn unlink(__name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn fsync(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fflush(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn ferror_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fileno_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fchown(__fd: libc::c_int, __owner: uid_t, __group: gid_t) -> libc::c_int;
  #[no_mangle]
  fn usleep(__useconds: __useconds_t) -> libc::c_int;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn strsep(__stringp: *mut *mut libc::c_char, __delim: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn fchmod(__fd: libc::c_int, __mode: __mode_t) -> libc::c_int;
  #[no_mangle]
  fn time(__timer: *mut time_t) -> time_t;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn last_char_is(s: *const libc::c_char, c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc_follow_symlinks(path: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn overlapping_strcpy(dst: *mut libc::c_char, src: *const libc::c_char);
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc_fgetline(file: *mut FILE) -> *mut libc::c_char;
  #[no_mangle]
  fn fopen_or_warn(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn xfdopen_for_write(fd: libc::c_int) -> *mut FILE;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_perror_nomsg();
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct flock {
  pub l_type: libc::c_short,
  pub l_whence: libc::c_short,
  pub l_start: __off64_t,
  pub l_len: __off64_t,
  pub l_pid: __pid_t,
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

/*
 * update_passwd
 *
 * update_passwd is a common function for passwd and chpasswd applets;
 * it is responsible for updating password file (i.e. /etc/passwd or
 * /etc/shadow) for a given user and password.
 *
 * Moved from loginutils/passwd.c by Alexander Shishkin <virtuoso@slind.org>
 *
 * Modified to be able to add or delete users, groups and users to/from groups
 * by Tito Ragusa <farmatito@tiscali.it>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/*
 1) add a user: update_passwd(FILE, USER, REMAINING_PWLINE, NULL)
    only if CONFIG_ADDUSER=y and applet_name[0] == 'a' like in adduser

 2) add a group: update_passwd(FILE, GROUP, REMAINING_GRLINE, NULL)
    only if CONFIG_ADDGROUP=y and applet_name[0] == 'a' like in addgroup

 3) add a user to a group: update_passwd(FILE, GROUP, NULL, MEMBER)
    only if CONFIG_FEATURE_ADDUSER_TO_GROUP=y, applet_name[0] == 'a'
    like in addgroup and member != NULL

 4) delete a user: update_passwd(FILE, USER, NULL, NULL)

 5) delete a group: update_passwd(FILE, GROUP, NULL, NULL)

 6) delete a user from a group: update_passwd(FILE, GROUP, NULL, MEMBER)
    only if CONFIG_FEATURE_DEL_USER_FROM_GROUP=y and member != NULL

 7) change user's password: update_passwd(FILE, USER, NEW_PASSWD, NULL)
    only if CONFIG_PASSWD=y and applet_name[0] == 'p' like in passwd
    or if CONFIG_CHPASSWD=y and applet_name[0] == 'c' like in chpasswd

 8) delete a user from all groups: update_passwd(FILE, NULL, NULL, MEMBER)

 This function does not validate the arguments fed to it
 so the calling program should take care of that.

 Returns number of lines changed, or -1 on error.
*/
#[no_mangle]
pub unsafe extern "C" fn update_passwd(
  mut filename: *const libc::c_char,
  mut name: *const libc::c_char,
  mut new_passwd: *const libc::c_char,
  mut member: *const libc::c_char,
) -> libc::c_int {
  let mut current_block: u64; /* failure */
  let mut sb: stat = std::mem::zeroed();
  let mut lock: flock = flock {
    l_type: 0,
    l_whence: 0,
    l_start: 0,
    l_len: 0,
    l_pid: 0,
  };
  let mut old_fp: *mut FILE = 0 as *mut FILE;
  let mut new_fp: *mut FILE = 0 as *mut FILE;
  let mut fnamesfx: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut sfx_char: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut name_colon: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut old_fd: libc::c_int = 0;
  let mut new_fd: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  let mut changed_lines: libc::c_int = 0;
  let mut ret: libc::c_int = -1i32;
  /* used as a bool: "are we modifying /etc/shadow?" */
  let mut shadow: *const libc::c_char =
    strstr(filename, b"shadow\x00" as *const u8 as *const libc::c_char);
  filename = xmalloc_follow_symlinks(filename);
  if filename.is_null() {
    return ret;
  }

  // TODO: why was this translated this way?
  // !name.is_null();

  /* New passwd file, "/etc/passwd+" for now */
  fnamesfx = xasprintf(b"%s+\x00" as *const u8 as *const libc::c_char, filename); /* missing shadow is not an error */
  sfx_char = &mut *fnamesfx.offset(
    (strlen as unsafe extern "C" fn(_: *const libc::c_char) -> size_t)(fnamesfx)
      .wrapping_sub(1i32 as libc::c_ulong) as isize,
  ) as *mut libc::c_char;
  name_colon = xasprintf(
    b"%s:\x00" as *const u8 as *const libc::c_char,
    if !name.is_null() {
      name
    } else {
      b"\x00" as *const u8 as *const libc::c_char
    },
  );
  if !shadow.is_null() {
    old_fp = fopen(filename, b"r+\x00" as *const u8 as *const libc::c_char)
  } else {
    old_fp = fopen_or_warn(filename, b"r+\x00" as *const u8 as *const libc::c_char)
  }
  if old_fp.is_null() {
    if !shadow.is_null() {
      ret = 0i32
    }
  } else {
    old_fd = fileno_unlocked(old_fp);
    /* Try to create "/etc/passwd+". Wait if it exists. */
    i = 30i32;
    loop {
      // FIXME: on last iteration try w/o O_EXCL but with O_TRUNC?
      new_fd = open(fnamesfx, 0o1i32 | 0o100i32 | 0o200i32, 0o600i32);
      if new_fd >= 0i32 {
        current_block = 16804866792809015806;
        break;
      }
      if *bb_errno != 17i32 {
        current_block = 1608152415753874203;
        break;
      }
      usleep(100000i32 as __useconds_t);
      i -= 1;
      if !(i != 0) {
        current_block = 1608152415753874203;
        break;
      }
      /* 0.1 sec */
    } /* ignore errors */
    match current_block {
      1608152415753874203 => {
        bb_perror_msg(
          b"can\'t create \'%s\'\x00" as *const u8 as *const libc::c_char,
          fnamesfx,
        );
      }
      _ => {
        if fstat(old_fd, &mut sb) == 0i32 {
          fchmod(new_fd, sb.st_mode & 0o777i32 as libc::c_uint);
          fchown(new_fd, sb.st_uid, sb.st_gid);
        }
        *bb_errno = 0i32;
        new_fp = xfdopen_for_write(new_fd);
        /* Backup file is "/etc/passwd-" */
        *sfx_char = '-' as i32 as libc::c_char;
        /* Delete old backup */
        i = (unlink(fnamesfx) != 0 && *bb_errno != 2i32) as libc::c_int;
        /* Create backup as a hardlink to current */
        if i != 0 || link(filename, fnamesfx) != 0 {
          bb_perror_msg(
            b"warning: can\'t create backup copy \'%s\'\x00" as *const u8 as *const libc::c_char,
            fnamesfx,
          );
        }
        *sfx_char = '+' as i32 as libc::c_char;
        /* Lock the password file before updating */
        lock.l_type = 1i32 as libc::c_short;
        lock.l_whence = 0i32 as libc::c_short;
        lock.l_start = 0i32 as __off64_t;
        lock.l_len = 0i32 as __off64_t;
        if fcntl(old_fd, 6i32, &mut lock as *mut flock) < 0i32 {
          bb_perror_msg(
            b"warning: can\'t lock \'%s\'\x00" as *const u8 as *const libc::c_char,
            filename,
          );
        }
        lock.l_type = 2i32 as libc::c_short;
        /* Read current password file, write updated /etc/passwd+ */
        changed_lines = 0i32;
        loop {
          let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
          let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
          line = xmalloc_fgetline(old_fp);
          if line.is_null() {
            break;
          }
          if name.is_null() && !member.is_null() {
            /* Delete member from all groups */
            /* line is "GROUP:PASSWD:[member1[,member2]...]" */
            let mut member_len: libc::c_uint = strlen(member) as libc::c_uint;
            let mut list: *mut libc::c_char = strrchr(line, ':' as i32);
            's_262: while !list.is_null() {
              list = list.offset(1);
              while !is_prefixed_with(list, member).is_null() {
                let mut c: libc::c_char = 0;
                changed_lines += 1;
                c = *list.offset(member_len as isize);
                if c as libc::c_int == '\u{0}' as i32 {
                  if *list.offset(-1i32 as isize) as libc::c_int == ',' as i32 {
                    list = list.offset(-1)
                  }
                  *list = '\u{0}' as i32 as libc::c_char;
                  break 's_262;
                } else if c as libc::c_int == ',' as i32 {
                  overlapping_strcpy(list, list.offset(member_len as isize).offset(1));
                } else {
                  changed_lines -= 1;
                  break;
                }
              }
              list = strchr(list, ',' as i32)
            }
            fprintf(
              new_fp,
              b"%s\n\x00" as *const u8 as *const libc::c_char,
              line,
            );
          } else {
            cp = is_prefixed_with(line, name_colon);
            if cp.is_null() {
              fprintf(
                new_fp,
                b"%s\n\x00" as *const u8 as *const libc::c_char,
                line,
              );
            } else if !member.is_null() {
              /* We have a match with "name:"... */
              /* cp points past "name:" */
              /* else delete user or group: skip the line */
              /* It's actually /etc/group+, not /etc/passwd+ */
              if 1i32 != 0 && *applet_name.offset(0) as libc::c_int == 'a' as i32 {
                /* Add user to group */
                fprintf(
                  new_fp,
                  b"%s%s%s\n\x00" as *const u8 as *const libc::c_char,
                  line,
                  if !last_char_is(line, ':' as i32).is_null() {
                    b"\x00" as *const u8 as *const libc::c_char
                  } else {
                    b",\x00" as *const u8 as *const libc::c_char
                  },
                  member,
                );
                changed_lines += 1
              } else {
                /* && applet_name[0] == 'd' */
                /* Delete user from group */
                let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut fmt: *const libc::c_char = b"%s\x00" as *const u8 as *const libc::c_char;
                cp = strrchr(line, ':' as i32);
                let fresh0 = cp;
                cp = cp.offset(1);
                *fresh0 = '\u{0}' as i32 as libc::c_char;
                fprintf(new_fp, b"%s:\x00" as *const u8 as *const libc::c_char, line);
                tmp = cp;
                loop {
                  cp = strsep(&mut tmp, b",\x00" as *const u8 as *const libc::c_char);
                  if cp.is_null() {
                    break;
                  }
                  if strcmp(member, cp) != 0i32 {
                    fprintf(new_fp, fmt, cp);
                    fmt = b",%s\x00" as *const u8 as *const libc::c_char
                  } else {
                    /* find the start of the member list: last ':' */
                    /* cut it */
                    /* write the cut line name:passwd:gid:
                     * or name:!:: */
                    /* parse the tokens of the member list */
                    /* found member, skip it */
                    changed_lines += 1
                  }
                }
                fprintf(new_fp, b"\n\x00" as *const u8 as *const libc::c_char);
              }
            } else if 1i32 != 0 && *applet_name.offset(0) as libc::c_int == 'p' as i32
              || 1i32 != 0 && *applet_name.offset(0) as libc::c_int == 'c' as i32
            {
              /* Change passwd */
              cp = strchrnul(cp, ':' as i32); /* move past old passwd */
              if !shadow.is_null() && *cp as libc::c_int == ':' as i32 {
                /* /etc/shadow's field 3 (passwd change date) needs updating */
                /* move past old change date */
                let mut time_days: libc::c_uint = (time(0 as *mut time_t) as libc::c_ulong)
                  .wrapping_div((24i32 * 60i32 * 60i32) as libc::c_ulong)
                  as libc::c_uint;
                if time_days == 0i32 as libc::c_uint {
                  /* 0 as change date has special meaning, avoid it */
                  time_days = 1i32 as libc::c_uint
                }
                cp = strchrnul(cp.offset(1), ':' as i32);
                /* "name:" + "new_passwd" + ":" + "change date" + ":rest of line" */
                fprintf(
                  new_fp,
                  b"%s%s:%u%s\n\x00" as *const u8 as *const libc::c_char,
                  name_colon,
                  new_passwd,
                  time_days,
                  cp,
                );
              } else {
                /* "name:" + "new_passwd" + ":rest of line" */
                fprintf(
                  new_fp,
                  b"%s%s%s\n\x00" as *const u8 as *const libc::c_char,
                  name_colon,
                  new_passwd,
                  cp,
                );
              }
              changed_lines += 1
            }
          }
          free(line as *mut libc::c_void);
        }
        /* EOF/error */
        if changed_lines == 0i32 {
          if !member.is_null() {
            if 1i32 != 0 && *applet_name.offset(0) as libc::c_int == 'a' as i32 {
              bb_error_msg(
                b"can\'t find %s in %s\x00" as *const u8 as *const libc::c_char,
                name,
                filename,
              );
            }
            if 1i32 != 0 && *applet_name.offset(0) as libc::c_int == 'd' as i32 {
              bb_error_msg(
                b"can\'t find %s in %s\x00" as *const u8 as *const libc::c_char,
                member,
                filename,
              );
            }
          }
          if (1i32 != 0 || 1i32 != 0)
            && *applet_name.offset(0) as libc::c_int == 'a' as i32
            && member.is_null()
          {
            /* add user or group */
            fprintf(
              new_fp,
              b"%s%s\n\x00" as *const u8 as *const libc::c_char,
              name_colon,
              new_passwd,
            );
            changed_lines += 1
          }
        }
        fcntl(old_fd, 6i32, &mut lock as *mut flock);
        /* We do want all of them to execute, thus | instead of || */
        *bb_errno = 0i32;
        if ferror_unlocked(old_fp) | fflush(new_fp) | fsync(new_fd) | fclose(new_fp) != 0
          || rename(fnamesfx, filename) != 0
        {
          /* At least one of those failed */
          bb_perror_nomsg();
        } else {
          /* Success: ret >= 0 */
          ret = changed_lines
        }
        if ret < 0i32 {
          unlink(fnamesfx);
        }
      }
    }
    fclose(old_fp);
  }
  free(fnamesfx as *mut libc::c_void);
  free(filename as *mut libc::c_char as *mut libc::c_void);
  free(name_colon as *mut libc::c_void);
  return ret;
}
