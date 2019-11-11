use crate::librb::size_t;
use crate::libbb::ptr_to_globals::bb_errno;

use libc;
use libc::free;
use libc::getpid;
use libc::kill;
use libc::pid_t;
use libc::printf;
use libc::puts;
extern "C" {

  #[no_mangle]
  fn getsid(__pid: pid_t) -> pid_t;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  #[no_mangle]
  fn bb_strtoi(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn find_pid_by_name(procName: *const libc::c_char) -> *mut pid_t;
  #[no_mangle]
  fn procps_scan(sp: *mut procps_status_t, flags: libc::c_int) -> *mut procps_status_t;
  #[no_mangle]
  fn get_signum(name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn get_signame(number: libc::c_int) -> *const libc::c_char;
  #[no_mangle]
  fn print_signames();
}

use libc::DIR;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct procps_status_t {
  pub dir: *mut DIR,
  pub task_dir: *mut DIR,
  pub shift_pages_to_bytes: u8,
  pub shift_pages_to_kb: u8,
  pub argv_len: u16,
  pub argv0: *mut libc::c_char,
  pub exe: *mut libc::c_char,
  pub main_thread_pid: libc::c_uint,
  pub vsz: libc::c_ulong,
  pub rss: libc::c_ulong,
  pub stime: libc::c_ulong,
  pub utime: libc::c_ulong,
  pub start_time: libc::c_ulong,
  pub pid: libc::c_uint,
  pub ppid: libc::c_uint,
  pub pgid: libc::c_uint,
  pub sid: libc::c_uint,
  pub uid: libc::c_uint,
  pub gid: libc::c_uint,
  pub ruid: libc::c_uint,
  pub rgid: libc::c_uint,
  pub niceness: libc::c_int,
  pub tty_major: libc::c_uint,
  pub tty_minor: libc::c_uint,
  pub smaps: smaprec,
  pub state: [libc::c_char; 4],
  pub comm: [libc::c_char; 16],
  pub last_seen_on_cpu: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct smaprec {
  pub mapped_rw: libc::c_ulong,
  pub mapped_ro: libc::c_ulong,
  pub shared_clean: libc::c_ulong,
  pub shared_dirty: libc::c_ulong,
  pub private_clean: libc::c_ulong,
  pub private_dirty: libc::c_ulong,
  pub stack: libc::c_ulong,
  pub smap_pss: libc::c_ulong,
  pub smap_swap: libc::c_ulong,
  pub smap_size: libc::c_ulong,
  pub smap_start: libc::c_ulonglong,
  pub smap_mode: [libc::c_char; 5],
  pub smap_name: *mut libc::c_char,
}
pub const PSSCAN_SID: C2RustUnnamed = 8;
pub const PSSCAN_PID: C2RustUnnamed = 1;
pub type C2RustUnnamed = libc::c_uint;
pub const PSSCAN_TASKS: C2RustUnnamed = 4194304;
pub const PSSCAN_RUIDGID: C2RustUnnamed = 2097152;
pub const PSSCAN_NICE: C2RustUnnamed = 1048576;
pub const PSSCAN_CPU: C2RustUnnamed = 524288;
pub const PSSCAN_START_TIME: C2RustUnnamed = 262144;
pub const PSSCAN_CONTEXT: C2RustUnnamed = 0;
pub const PSSCAN_ARGVN: C2RustUnnamed = 65536;
pub const PSSCAN_SMAPS: C2RustUnnamed = 32768;
pub const PSSCAN_TTY: C2RustUnnamed = 16384;
pub const PSSCAN_UTIME: C2RustUnnamed = 8192;
pub const PSSCAN_STIME: C2RustUnnamed = 4096;
pub const PSSCAN_RSS: C2RustUnnamed = 2048;
pub const PSSCAN_VSZ: C2RustUnnamed = 1024;
pub const PSSCAN_STATE: C2RustUnnamed = 512;
pub const PSSCAN_EXE: C2RustUnnamed = 256;
pub const PSSCAN_ARGV0: C2RustUnnamed = 128;
pub const PSSCAN_COMM: C2RustUnnamed = 32;
pub const PSSCAN_UIDGID: C2RustUnnamed = 16;
pub const PSSCAN_PGID: C2RustUnnamed = 4;
pub const PSSCAN_PPID: C2RustUnnamed = 2;

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
#[no_mangle]
pub unsafe extern "C" fn kill_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut pid: pid_t = 0;
  let mut signo: libc::c_int = 15i32;
  let mut errors: libc::c_int = 0i32;
  let mut quiet: libc::c_int = 0i32;
  /* How to determine who we are? find 3rd char from the end:
   * kill, killall, killall5
   *  ^i       ^a        ^l  - it's unique
   * (checking from the start is complicated by /bin/kill... case) */
  let char3: libc::c_char =
    *(*argv.offset(0)).offset(strlen(*argv.offset(0)).wrapping_sub(3i32 as libc::c_ulong) as isize);
  /* Parse any options */
  argv = argv.offset(1);
  arg = *argv;
  if !(arg.is_null() || *arg.offset(0) as libc::c_int != '-' as i32) {
    /* The -l option, which prints out signal names.
     * Intended usage in shell:
     * echo "Died of SIG`kill -l $?`"
     * We try to mimic what kill from coreutils-6.8 does */
    if *arg.offset(1) as libc::c_int == 'l' as i32
      && *arg.offset(2) as libc::c_int == '\u{0}' as i32
    {
      argv = argv.offset(1);
      arg = *argv;
      if arg.is_null() {
        /* Print the whole signal list */
        print_signames();
        return 0i32;
      }
      loop
      /* -l <sig list> */
      {
        if (*arg.offset(0) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
          signo = bb_strtou(arg, 0 as *mut *mut libc::c_char, 10i32) as libc::c_int;
          if *bb_errno != 0 {
            bb_error_msg(
              b"unknown signal \'%s\'\x00" as *const u8 as *const libc::c_char,
              arg,
            );
            return 1i32;
          }
          /* TODO: 'bad' signal# - coreutils says:
           * kill: 127: invalid signal
           * we just print "127" instead */
          puts(get_signame(signo & 0x7fi32));
        } else {
          signo = get_signum(arg);
          if signo < 0i32 {
            bb_error_msg(
              b"unknown signal \'%s\'\x00" as *const u8 as *const libc::c_char,
              arg,
            );
            return 1i32;
          }
          printf(b"%d\n\x00" as *const u8 as *const libc::c_char, signo);
        }
        argv = argv.offset(1);
        arg = *argv;
        if arg.is_null() {
          break;
        }
      }
      return 0i32;
    }
    /* Exitcodes >= 0x80 are to be treated
     * as "killed by signal (exitcode & 0x7f)" */
    /* The -q quiet option */
    if 1i32 != 0
      && char3 as libc::c_int == 'a' as i32
      && *arg.offset(1) as libc::c_int == 'q' as i32
      && *arg.offset(2) as libc::c_int == '\u{0}' as i32
    {
      quiet = 1i32; /* skip '-' */
      argv = argv.offset(1);
      arg = *argv;
      if arg.is_null() {
        bb_show_usage();
      }
      if *arg.offset(0) as libc::c_int != '-' as i32 {
        current_block = 2454055584343734536;
      } else {
        current_block = 8704759739624374314;
      }
    } else {
      current_block = 8704759739624374314;
    }
    match current_block {
      2454055584343734536 => {}
      _ => {
        arg = arg.offset(1);
        /* -o PID? (if present, it always is at the end of command line) */
        if !(1i32 != 0
          && char3 as libc::c_int == 'l' as i32
          && *arg.offset(0) as libc::c_int == 'o' as i32)
        {
          /* "--" separates options from args. Testcase: "kill -- -123" */
          if !(!(1i32 != 0 && char3 as libc::c_int == 'l' as i32)
            && *arg.offset(0) as libc::c_int == '-' as i32
            && *arg.offset(1) as libc::c_int == '\u{0}' as i32)
          {
            if !(*argv.offset(1)).is_null()
              && *arg.offset(0) as libc::c_int == 's' as i32
              && *arg.offset(1) as libc::c_int == '\u{0}' as i32
            {
              /* else it must be -SIG */
              /* -s SIG? */
              argv = argv.offset(1);
              arg = *argv
            }
            signo = get_signum(arg);
            if signo < 0i32 {
              bb_error_msg(
                b"bad signal name \'%s\'\x00" as *const u8 as *const libc::c_char,
                arg,
              );
              return 1i32;
            }
          }
          argv = argv.offset(1);
          arg = *argv
        }
      }
    }
  }
  pid = getpid();
  if 1i32 != 0 && char3 as libc::c_int == 'l' as i32 {
    let mut sid: pid_t = 0;
    let mut p: *mut procps_status_t = 0 as *mut procps_status_t;
    /* compat: exitcode 2 is "no one was signaled" */
    errors = 2i32;
    /* Find out our session id */
    sid = getsid(pid);
    /* Stop all processes */
    if signo != 19i32 && signo != 18i32 {
      kill(-1i32, 19i32);
    }
    's_259: loop
    /* Signal all processes except those in our session */
    {
      p = procps_scan(p, PSSCAN_PID as libc::c_int | PSSCAN_SID as libc::c_int);
      if p.is_null() {
        break;
      }
      let mut args: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
      if (*p).sid == sid as libc::c_uint
        || (*p).sid == 0i32 as libc::c_uint
        || (*p).pid == pid as libc::c_uint
        || (*p).pid == 1i32 as libc::c_uint
      {
        continue;
      }
      /* All remaining args must be -o PID options.
       * Check p->pid against them. */
      args = argv;
      while !(*args).is_null() {
        let mut omit: pid_t = 0;
        let fresh0 = args;
        args = args.offset(1);
        arg = *fresh0;
        if *arg.offset(0) as libc::c_int != '-' as i32
          || *arg.offset(1) as libc::c_int != 'o' as i32
        {
          bb_error_msg(
            b"bad option \'%s\'\x00" as *const u8 as *const libc::c_char,
            arg,
          );
          errors = 1i32;
          break 's_259;
        } else {
          arg = arg.offset(2);
          if *arg.offset(0) == 0 && !(*args).is_null() {
            let fresh1 = args;
            args = args.offset(1);
            arg = *fresh1
          }
          omit = bb_strtoi(arg, 0 as *mut *mut libc::c_char, 10i32);
          if *bb_errno != 0 {
            bb_error_msg(
              b"invalid number \'%s\'\x00" as *const u8 as *const libc::c_char,
              arg,
            );
            errors = 1i32;
            break 's_259;
          } else if (*p).pid == omit as libc::c_uint {
            continue 's_259;
          }
        }
      }
      kill((*p).pid as pid_t, signo);
      errors = 0i32
    }
    /* And let them continue */
    if signo != 19i32 && signo != 18i32 {
      kill(-1i32, 18i32);
    }
    return errors;
  } else {
    /* Pid or name is required for kill/killall */
    if arg.is_null() {
      bb_simple_error_msg(
        b"you need to specify whom to kill\x00" as *const u8 as *const libc::c_char,
      );
      return 1i32;
    }
    if 1i32 == 0 || 1i32 != 0 && char3 as libc::c_int == 'a' as i32 {
      loop
      /* Looks like they want to do a killall.  Do that */
      {
        let mut pidList: *mut pid_t = 0 as *mut pid_t;
        pidList = find_pid_by_name(arg);
        if *pidList == 0i32 {
          errors += 1;
          if quiet == 0 {
            bb_error_msg(
              b"%s: no process killed\x00" as *const u8 as *const libc::c_char,
              arg,
            );
          }
        } else {
          let mut pl: *mut pid_t = 0 as *mut pid_t;
          pl = pidList;
          while *pl != 0 {
            if !(*pl == pid) {
              if !(kill(*pl, signo) == 0i32) {
                errors += 1;
                if quiet == 0 {
                  bb_perror_msg(
                    b"can\'t kill pid %d\x00" as *const u8 as *const libc::c_char,
                    *pl,
                  );
                }
              }
            }
            pl = pl.offset(1)
          }
        }
        free(pidList as *mut libc::c_void);
        argv = argv.offset(1);
        arg = *argv;
        if arg.is_null() {
          break;
        }
      }
      return errors;
    }
    /* Looks like they want to do a kill. Do that */
    while !arg.is_null() {
      /*
       * We need to support shell's "hack formats" of
       * " -PRGP_ID" (yes, with a leading space)
       * and " PID1 PID2 PID3" (with degenerate case "")
       */
      while *arg as libc::c_int != '\u{0}' as i32 {
        let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
        if *arg as libc::c_int == ' ' as i32 {
          arg = arg.offset(1)
        }
        pid = bb_strtoi(arg, &mut end, 10i32);
        if *bb_errno != 0 && (*bb_errno != 22i32 || *end as libc::c_int != ' ' as i32) {
          bb_error_msg(
            b"invalid number \'%s\'\x00" as *const u8 as *const libc::c_char,
            arg,
          );
          errors += 1;
          break;
        } else {
          if kill(pid, signo) != 0i32 {
            bb_perror_msg(
              b"can\'t kill pid %d\x00" as *const u8 as *const libc::c_char,
              pid,
            );
            errors += 1
          }
          arg = end
        }
        /* can only point to ' ' or '\0' now */
      }
      /* ENABLE_KILL but !SH_KILL */
      argv = argv.offset(1);
      arg = *argv
    }
    return errors;
  };
}
