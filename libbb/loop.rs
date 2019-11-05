use libc;
extern "C" {
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
  #[no_mangle]
  fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn mknod(__path: *const libc::c_char, __mode: __mode_t, __dev: __dev_t) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn safe_strncpy(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    size: size_t,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_makedev(major: libc::c_uint, minor: libc::c_uint) -> libc::c_ulonglong;
}
use crate::librb::__dev_t;

use crate::librb::__mode_t;

use crate::librb::off_t;
use crate::librb::size_t;
use crate::librb::stat;
use crate::librb::timespec;
pub type __u8 = libc::c_uchar;

/*
 * Utility routines.
 *
 * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
 * Copyright (C) 2005 by Rob Landley <rob@landley.net>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* For 2.6, use the cleaned up header to get the 64 bit API. */
// Commented out per Rob's request
//# include "fix_u32.h" /* some old toolchains need __u64 for linux/loop.h */
pub type bb_loop_info = loop_info64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_info64 {
  pub lo_device: __u64,
  pub lo_inode: __u64,
  pub lo_rdevice: __u64,
  pub lo_offset: __u64,
  pub lo_sizelimit: __u64,
  pub lo_number: __u32,
  pub lo_encrypt_type: __u32,
  pub lo_encrypt_key_size: __u32,
  pub lo_flags: __u32,
  pub lo_file_name: [__u8; 64],
  pub lo_crypt_name: [__u8; 64],
  pub lo_encrypt_key: [__u8; 32],
  pub lo_init: [__u64; 2],
}
pub type __u64 = libc::c_ulonglong;
pub type __u32 = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn query_loop(mut device: *const libc::c_char) -> *mut libc::c_char {
  let mut fd: libc::c_int = 0;
  let mut loopinfo: bb_loop_info = bb_loop_info {
    lo_device: 0,
    lo_inode: 0,
    lo_rdevice: 0,
    lo_offset: 0,
    lo_sizelimit: 0,
    lo_number: 0,
    lo_encrypt_type: 0,
    lo_encrypt_key_size: 0,
    lo_flags: 0,
    lo_file_name: [0; 64],
    lo_crypt_name: [0; 64],
    lo_encrypt_key: [0; 32],
    lo_init: [0; 2],
  };
  let mut dev: *mut libc::c_char = 0 as *mut libc::c_char;
  fd = open(device, 0i32);
  if fd >= 0i32 {
    if ioctl(
      fd,
      0x4c05i32 as libc::c_ulong,
      &mut loopinfo as *mut bb_loop_info,
    ) == 0i32
    {
      dev = xasprintf(
        b"%lu %s\x00" as *const u8 as *const libc::c_char,
        loopinfo.lo_offset as off_t,
        loopinfo.lo_file_name.as_mut_ptr() as *mut libc::c_char,
      )
    }
    close(fd);
  }
  return dev;
}
#[no_mangle]
pub unsafe extern "C" fn del_loop(mut device: *const libc::c_char) -> libc::c_int {
  let mut fd: libc::c_int = 0;
  let mut rc: libc::c_int = 0;
  fd = open(device, 0i32);
  if fd < 0i32 {
    return 1i32;
  }
  rc = ioctl(fd, 0x4c01i32 as libc::c_ulong, 0i32);
  close(fd);
  return rc;
}
/* Obtain an unused loop device number */
#[no_mangle]
pub unsafe extern "C" fn get_free_loop() -> libc::c_int {
  let mut fd: libc::c_int = 0; /* -2: "no /dev/loop-control" */
  let mut loopdevno: libc::c_int = 0;
  fd = open(
    b"/dev/loop-control\x00" as *const u8 as *const libc::c_char,
    0o2i32 | 0o2000000i32,
  );
  if fd == -1i32 {
    return fd - 1i32;
  }
  loopdevno = ioctl(fd, 0x4c82i32 as libc::c_ulong);
  close(fd);
  return loopdevno;
  /* can be -1 if error */
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
/* Returns opened fd to the loop device, <0 on error.
 * *device is loop device to use, or if *device==NULL finds a loop device to
 * mount it on and sets *device to a strdup of that loop device name.  This
 * search will re-use an existing loop device already bound to that
 * file/offset if it finds one.
 */
#[no_mangle]
pub unsafe extern "C" fn set_loop(
  mut device: *mut *mut libc::c_char,
  mut file: *const libc::c_char,
  mut offset: libc::c_ulonglong,
  mut flags: libc::c_uint,
) -> libc::c_int {
  let mut current_block: u64;
  let mut dev: [libc::c_char; 23] = [0; 23];
  let mut try_0: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut loopinfo: bb_loop_info = bb_loop_info {
    lo_device: 0,
    lo_inode: 0,
    lo_rdevice: 0,
    lo_offset: 0,
    lo_sizelimit: 0,
    lo_number: 0,
    lo_encrypt_type: 0,
    lo_encrypt_key_size: 0,
    lo_flags: 0,
    lo_file_name: [0; 64],
    lo_crypt_name: [0; 64],
    lo_encrypt_key: [0; 32],
    lo_init: [0; 2],
  };
  let mut statbuf: stat = stat {
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
  let mut i: libc::c_int = 0;
  let mut dfd: libc::c_int = 0;
  let mut ffd: libc::c_int = 0;
  let mut mode: libc::c_int = 0;
  let mut rc: libc::c_int = 0;
  dfd = -1i32;
  rc = dfd;
  /* Open the file.  Barf if this doesn't work.  */
  mode = if flags & 1i32 as libc::c_uint != 0 {
    0i32
  } else {
    0o2i32
  };
  loop {
    ffd = open(file, mode);
    if ffd < 0i32 {
      if mode != 0i32 {
        mode = 0i32
      } else {
        return -*bb_errno;
      }
    } else {
      try_0 = *device;
      if try_0.is_null() {
        current_block = 12209867499936983673;
        break;
      } else {
        current_block = 4495394744059808450;
        break;
      }
    }
  }
  match current_block {
    12209867499936983673 => {
      i = get_free_loop();
      if i == -2i32 {
        /* no /dev/loop-control */
        i = 0i32;
        try_0 = dev.as_mut_ptr();
        current_block = 4495394744059808450;
      } else {
        if i == -1i32 {
          close(ffd);
          return -1i32;
          /* no free loop devices */
        }
        *device = xasprintf(b"/dev/loop%u\x00" as *const u8 as *const libc::c_char, i);
        try_0 = *device;
        current_block = 5504172152099367553;
      }
    }
    _ => {}
  }
  match current_block {
    4495394744059808450 =>
    /* Find a loop device.  */
    /* 1048575 (0xfffff) is a max possible minor number in Linux circa 2010 */
    {
      i = 0i32;
      current_block = 15089075282327824602;
    }
    _ => {}
  }
  loop {
    match current_block {
      15089075282327824602 => {
        if !(rc != 0 && i < 1048576i32) {
          break;
        }
        sprintf(
          dev.as_mut_ptr(),
          b"/dev/loop%u\x00" as *const u8 as *const libc::c_char,
          i,
        );
        *bb_errno = 0i32;
        if !(stat(try_0, &mut statbuf) != 0i32
          || !(statbuf.st_mode & 0o170000i32 as libc::c_uint == 0o60000i32 as libc::c_uint))
        {
          current_block = 5504172152099367553;
          continue;
        }
        if 1i32 != 0 && *bb_errno == 2i32 && try_0 == dev.as_mut_ptr() {
          /* Node doesn't exist, try to create it.  */
          if mknod(
            dev.as_mut_ptr(),
            (0o60000i32 | 0o644i32) as __mode_t,
            bb_makedev(7i32 as libc::c_uint, i as libc::c_uint) as __dev_t,
          ) == 0i32
          {
            current_block = 5504172152099367553;
            continue;
          }
        }
        /* Ran out of block devices, return failure.  */
        rc = -1i32;
        break;
      }
      _ => {
        /* Open the sucker and check its loopiness.  */
        dfd = open(try_0, mode);
        if dfd < 0i32 && *bb_errno == 30i32 {
          mode = 0i32;
          dfd = open(try_0, mode)
        }
        if dfd < 0i32 {
          if *bb_errno == 6i32 {
            /* Happens if loop module is not loaded */
            rc = -1i32;
            break;
          }
        } else {
          rc = ioctl(
            dfd,
            0x4c05i32 as libc::c_ulong,
            &mut loopinfo as *mut bb_loop_info,
          );
          /* If device is free, claim it.  */
          if rc != 0 && *bb_errno == 6i32 {
            /* Associate free loop device with file.  */
            if ioctl(dfd, 0x4c00i32 as libc::c_ulong, ffd) == 0i32 {
              memset(
                &mut loopinfo as *mut bb_loop_info as *mut libc::c_void,
                0i32,
                ::std::mem::size_of::<bb_loop_info>() as libc::c_ulong,
              );
              safe_strncpy(
                loopinfo.lo_file_name.as_mut_ptr() as *mut libc::c_char,
                file,
                64i32 as size_t,
              );
              loopinfo.lo_offset = offset;
              /*
               * Used by mount to set LO_FLAGS_AUTOCLEAR.
               * LO_FLAGS_READ_ONLY is not set because RO is controlled by open type of the file.
               * Note that closing LO_FLAGS_AUTOCLEARed dfd before mount
               * is wrong (would free the loop device!)
               */
              loopinfo.lo_flags = flags & !1i32 as libc::c_uint;
              rc = ioctl(
                dfd,
                0x4c04i32 as libc::c_ulong,
                &mut loopinfo as *mut bb_loop_info,
              );
              if rc != 0i32 && loopinfo.lo_flags & 4i32 as libc::c_uint != 0 {
                /* Old kernel, does not support LO_FLAGS_AUTOCLEAR? */
                /* (this code path is not tested) */
                loopinfo.lo_flags = (loopinfo.lo_flags as libc::c_uint)
                  .wrapping_sub(4i32 as libc::c_uint) as __u32
                  as __u32;
                rc = ioctl(
                  dfd,
                  0x4c04i32 as libc::c_ulong,
                  &mut loopinfo as *mut bb_loop_info,
                )
              }
              if rc != 0i32 {
                ioctl(dfd, 0x4c01i32 as libc::c_ulong, 0i32);
                // actually, 0 param is unnecessary
              }
            }
          } else {
            rc = -1i32
          }
          if rc != 0i32 {
            close(dfd);
          }
        }
        if !(*device).is_null() {
          break;
        }
        i += 1;
        current_block = 15089075282327824602;
      }
    }
  }
  close(ffd);
  if rc == 0i32 {
    if (*device).is_null() {
      *device = xstrdup(dev.as_mut_ptr())
    }
    return dfd;
  }
  return rc;
}
