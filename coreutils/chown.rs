use libc;



























use libc::chown;


























use libc::printf;










use libc::gid_t;
use libc::stat;
use libc::uid_t;

use crate::librb::bb_uidgid_t;

extern "C" {

  #[no_mangle]
  fn lchown(__file: *const libc::c_char, __owner: uid_t, __group: gid_t) -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn recursive_action(
    fileName: *const libc::c_char,
    flags: libc::c_uint,
    fileAction_0: Option<
      unsafe extern "C" fn(
        _: *const libc::c_char,
        _: *mut stat,
        _: *mut libc::c_void,
        _: libc::c_int,
      ) -> libc::c_int,
    >,
    dirAction: Option<
      unsafe extern "C" fn(
        _: *const libc::c_char,
        _: *mut stat,
        _: *mut libc::c_void,
        _: libc::c_int,
      ) -> libc::c_int,
    >,
    userData: *mut libc::c_void,
    depth: libc::c_uint,
  ) -> libc::c_int;
  #[no_mangle]
  fn parse_chown_usergroup_or_die(u: *mut bb_uidgid_t, user_group: *mut libc::c_char);
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> u32;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
}

pub type C2RustUnnamed = libc::c_uint;
pub const ACTION_DANGLING_OK: C2RustUnnamed = 64;
pub const ACTION_QUIET: C2RustUnnamed = 32;
pub const ACTION_DEPTHFIRST: C2RustUnnamed = 8;
pub const ACTION_FOLLOWLINKS_L0: C2RustUnnamed = 4;
pub const ACTION_FOLLOWLINKS: C2RustUnnamed = 2;
pub const ACTION_RECURSE: C2RustUnnamed = 1;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct param_t {
  pub ugid: bb_uidgid_t,
  pub chown_func: chown_fptr,
}
pub type chown_fptr =
  Option<unsafe extern "C" fn(_: *const libc::c_char, _: uid_t, _: gid_t) -> libc::c_int>;

/*
 * Mini chown implementation for busybox
 *
 * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config CHOWN
//config:	bool "chown (7.6 kb)"
//config:	default y
//config:	help
//config:	chown is used to change the user and/or group ownership
//config:	of files.
//config:
//config:config FEATURE_CHOWN_LONG_OPTIONS
//config:	bool "Enable long options"
//config:	default y
//config:	depends on CHOWN && LONG_OPTS
//applet:IF_CHOWN(APPLET_NOEXEC(chown, chown, BB_DIR_BIN, BB_SUID_DROP, chown))
//kbuild:lib-$(CONFIG_CHOWN) += chown.o
/* BB_AUDIT SUSv3 defects - none? */
/* http://www.opengroup.org/onlinepubs/007904975/utilities/chown.html */
//usage:#define chown_trivial_usage
//usage:       "[-Rh"IF_DESKTOP("LHPcvf")"]... USER[:[GRP]] FILE..."
//usage:#define chown_full_usage "\n\n"
//usage:       "Change the owner and/or group of each FILE to USER and/or GRP\n"
//usage:     "\n	-R	Recurse"
//usage:     "\n	-h	Affect symlinks instead of symlink targets"
//usage:	IF_DESKTOP(
//usage:     "\n	-L	Traverse all symlinks to directories"
//usage:     "\n	-H	Traverse symlinks on command line only"
//usage:     "\n	-P	Don't traverse symlinks (default)"
//usage:     "\n	-c	List changed files"
//usage:     "\n	-v	List all files"
//usage:     "\n	-f	Hide errors"
//usage:	)
//usage:
//usage:#define chown_example_usage
//usage:       "$ ls -l /tmp/foo\n"
//usage:       "-r--r--r--    1 andersen andersen        0 Apr 12 18:25 /tmp/foo\n"
//usage:       "$ chown root /tmp/foo\n"
//usage:       "$ ls -l /tmp/foo\n"
//usage:       "-r--r--r--    1 root     andersen        0 Apr 12 18:25 /tmp/foo\n"
//usage:       "$ chown root.root /tmp/foo\n"
//usage:       "ls -l /tmp/foo\n"
//usage:       "-r--r--r--    1 root     root            0 Apr 12 18:25 /tmp/foo\n"
/* This is a NOEXEC applet. Be very careful! */
/* POSIX options
 * -L traverse every symbolic link to a directory encountered
 * -H if a command line argument is a symbolic link to a directory, traverse it
 * -P do not traverse any symbolic links (default)
 * We do not conform to the following:
 * "Specifying more than one of -H, -L, and -P is not an error.
 * The last option specified shall determine the behavior of the utility." */
/* -L */
/* -H or -L */
static mut chown_longopts: [libc::c_char; 81] = [
  114, 101, 99, 117, 114, 115, 105, 118, 101, 0, 0, 82, 100, 101, 114, 101, 102, 101, 114, 101,
  110, 99, 101, 0, 0, -1, 110, 111, 45, 100, 101, 114, 101, 102, 101, 114, 101, 110, 99, 101, 0, 0,
  104, 99, 104, 97, 110, 103, 101, 115, 0, 0, 99, 115, 105, 108, 101, 110, 116, 0, 0, 102, 113,
  117, 105, 101, 116, 0, 0, 102, 118, 101, 114, 98, 111, 115, 101, 0, 0, 118, 0,
];
unsafe extern "C" fn fileAction(
  mut fileName: *const libc::c_char,
  mut statbuf: *mut stat,
  mut vparam: *mut libc::c_void,
  mut _depth: libc::c_int,
) -> libc::c_int {
  let mut u: uid_t = if (*(vparam as *mut param_t)).ugid.uid == -1i64 as uid_t {
    (*statbuf).st_uid
  } else {
    (*(vparam as *mut param_t)).ugid.uid
  };
  let mut g: gid_t = if (*(vparam as *mut param_t)).ugid.gid == -1i64 as gid_t {
    (*statbuf).st_gid
  } else {
    (*(vparam as *mut param_t)).ugid.gid
  };
  if (*(vparam as *mut param_t))
    .chown_func
    .expect("non-null function pointer")(fileName, u, g)
    == 0i32
  {
    if option_mask32 & 0x4i32 as libc::c_uint != 0
      || option_mask32 & 0x8i32 as libc::c_uint != 0
        && ((*statbuf).st_uid != u || (*statbuf).st_gid != g)
    {
      printf(
        b"changed ownership of \'%s\' to %u:%u\n\x00" as *const u8 as *const libc::c_char,
        fileName,
        u,
        g,
      );
    }
    return 1i32;
  }
  if option_mask32 & 0x10i32 as libc::c_uint == 0 {
    bb_simple_perror_msg(fileName);
  }
  return 0i32;
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
#[no_mangle]
pub unsafe extern "C" fn chown_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut retval: libc::c_int = 0i32;
  let mut opt: libc::c_int = 0;
  let mut flags: libc::c_int = 0;
  let mut param: param_t = param_t {
    ugid: bb_uidgid_t { uid: 0, gid: 0 },
    chown_func: None,
  };
  opt = getopt32long(
    argv,
    b"^RhvcfLHP\x00-2\x00" as *const u8 as *const libc::c_char,
    chown_longopts.as_ptr(),
  ) as libc::c_int;
  argv = argv.offset(optind as isize);
  /* This matches coreutils behavior (almost - see below) */
  param.chown_func =
    Some(chown as unsafe extern "C" fn(_: *const libc::c_char, _: uid_t, _: gid_t) -> libc::c_int); /* match coreutils order */
  if opt & 2i32 != 0 || opt & (1i32 | (0x20i32 | 0x40i32)) == 1i32 {
    param.chown_func = Some(
      lchown as unsafe extern "C" fn(_: *const libc::c_char, _: uid_t, _: gid_t) -> libc::c_int,
    )
  } /* -H/-L: follow links on depth 0 */
  flags = ACTION_DEPTHFIRST as libc::c_int; /* follow links if -L */
  if opt & 1i32 != 0 {
    flags |= ACTION_RECURSE as libc::c_int
  }
  if opt & (0x20i32 | 0x40i32) != 0 {
    flags |= ACTION_FOLLOWLINKS_L0 as libc::c_int
  }
  if opt & 0x20i32 != 0 {
    flags |= ACTION_FOLLOWLINKS as libc::c_int
  }
  parse_chown_usergroup_or_die(&mut param.ugid, *argv.offset(0));
  loop
  /* Ok, ready to do the deed now */
  {
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
    if recursive_action(
      *argv,
      flags as libc::c_uint,
      Some(
        fileAction
          as unsafe extern "C" fn(
            _: *const libc::c_char,
            _: *mut stat,
            _: *mut libc::c_void,
            _: libc::c_int,
          ) -> libc::c_int,
      ),
      Some(
        fileAction
          as unsafe extern "C" fn(
            _: *const libc::c_char,
            _: *mut stat,
            _: *mut libc::c_void,
            _: libc::c_int,
          ) -> libc::c_int,
      ),
      &mut param as *mut param_t as *mut libc::c_void,
      0i32 as libc::c_uint,
    ) == 0
    {
      /* depth */
      retval = 1i32
    }
  }
  return retval;
}
/*
Testcase. Run in empty directory.

#!/bin/sh
t1="/tmp/busybox chown"
t2="/usr/bin/chown"
create() {
    rm -rf $1; mkdir $1
    (
    cd $1 || exit 1
    mkdir dir dir2
    >up
    >file
    >dir/file
    >dir2/file
    ln -s dir linkdir
    ln -s file linkfile
    ln -s ../up dir/linkup
    ln -s ../dir2 dir/linkupdir2
    )
    chown -R 0:0 $1
}
tst() {
    create test1
    create test2
    echo "[$1]" >>test1.out
    echo "[$1]" >>test2.out
    (cd test1; $t1 $1) >>test1.out 2>&1
    (cd test2; $t2 $1) >>test2.out 2>&1
    (cd test1; ls -lnR) >out1
    (cd test2; ls -lnR) >out2
    echo "chown $1" >out.diff
    if ! diff -u out1 out2 >>out.diff; then exit 1; fi
    rm out.diff
}
tst_for_each() {
    tst "$1 1:1 file"
    tst "$1 1:1 dir"
    tst "$1 1:1 linkdir"
    tst "$1 1:1 linkfile"
}
echo "If script produced 'out.diff' file, then at least one testcase failed"
>test1.out
>test2.out
# These match coreutils 6.8:
tst_for_each "-v"
tst_for_each "-vR"
tst_for_each "-vRP"
tst_for_each "-vRL"
tst_for_each "-vRH"
tst_for_each "-vh"
tst_for_each "-vhR"
tst_for_each "-vhRP"
tst_for_each "-vhRL"
tst_for_each "-vhRH"
# Fix `name' in coreutils output
sed 's/`/'"'"'/g' -i test2.out
# Compare us with coreutils output
diff -u test1.out test2.out

*/
