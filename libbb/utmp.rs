use crate::librb::size_t;
use libc;

use libc::pid_t;
use libc::time_t;

extern "C" {
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn time(__timer: *mut time_t) -> time_t;
  #[no_mangle]
  fn setutxent();
  #[no_mangle]
  fn endutxent();
  #[no_mangle]
  fn getutxent() -> *mut utmpx;
  #[no_mangle]
  fn pututxline(__utmpx: *const utmpx) -> *mut utmpx;
  #[no_mangle]
  fn updwtmpx(__wtmpx_file: *const libc::c_char, __utmpx: *const utmpx);
  #[no_mangle]
  fn skip_dev_pfx(tty_name: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn safe_strncpy(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    size: size_t,
  ) -> *mut libc::c_char;
  #[no_mangle]
  static bb_path_wtmp_file: [libc::c_char; 0];
}

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
  pub ut_pid: pid_t,
  pub ut_line: [libc::c_char; 32],
  pub ut_id: [libc::c_char; 4],
  pub ut_user: [libc::c_char; 32],
  pub ut_host: [libc::c_char; 256],
  pub ut_exit: __exit_status,
  pub ut_session: i32,
  pub ut_tv: C2RustUnnamed,
  pub ut_addr_v6: [i32; 4],
  pub __glibc_reserved: [libc::c_char; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
  pub tv_sec: i32,
  pub tv_usec: i32,
}

/*
 * utmp/wtmp support routines.
 *
 * Copyright (C) 2010 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
unsafe extern "C" fn touch(mut filename: *const libc::c_char) {
  if access(filename, 4i32 | 2i32) == -1i32 {
    close(open(filename, 0o1i32 | 0o100i32, 0o664i32));
  };
}
#[no_mangle]
pub unsafe extern "C" fn write_new_utmp(
  mut pid: pid_t,
  mut new_type: libc::c_int,
  mut tty_name: *const libc::c_char,
  mut username: *const libc::c_char,
  mut hostname: *const libc::c_char,
) {
  let mut utent: utmpx = utmpx {
    ut_type: 0,
    ut_pid: 0,
    ut_line: [0; 32],
    ut_id: [0; 4],
    ut_user: [0; 32],
    ut_host: [0; 256],
    ut_exit: __exit_status {
      e_termination: 0,
      e_exit: 0,
    },
    ut_session: 0,
    ut_tv: C2RustUnnamed {
      tv_sec: 0,
      tv_usec: 0,
    },
    ut_addr_v6: [0; 4],
    __glibc_reserved: [0; 20],
  };
  let mut id: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut width: libc::c_uint = 0;
  memset(
    &mut utent as *mut utmpx as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<utmpx>() as libc::c_ulong,
  );
  utent.ut_pid = pid;
  utent.ut_type = new_type as libc::c_short;
  tty_name = skip_dev_pfx(tty_name);
  safe_strncpy(
    utent.ut_line.as_mut_ptr(),
    tty_name,
    ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
  );
  if !username.is_null() {
    safe_strncpy(
      utent.ut_user.as_mut_ptr(),
      username,
      ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
    );
  }
  if !hostname.is_null() {
    safe_strncpy(
      utent.ut_host.as_mut_ptr(),
      hostname,
      ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
    );
  }
  utent.ut_tv.tv_sec = time(0 as *mut time_t) as i32;
  /* Invent our own ut_id. ut_id is only 4 chars wide.
   * Try to fit something remotely meaningful... */
  id = utent.ut_id.as_mut_ptr(); /* else: usually it's "ttyXXXX", map to "XXXX" */
  width = ::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as libc::c_uint;
  if *tty_name.offset(0) as libc::c_int == 'p' as i32 {
    /* if "ptyXXX", map to "pXXX" */
    /* if "pts/XX", map to "p/XX" */
    let fresh0 = id;
    id = id.offset(1);
    *fresh0 = 'p' as i32 as libc::c_char;
    width = width.wrapping_sub(1)
  }
  if strlen(tty_name) > 3i32 as libc::c_ulong {
    tty_name = tty_name.offset(3)
  }
  strncpy(id, tty_name, width as libc::c_ulong);
  touch(b"/var/run/utmp\x00" as *const u8 as *const libc::c_char);
  //utmpxname(_PATH_UTMPX);
  setutxent();
  /* Append new one (hopefully, unless we collide on ut_id) */
  pututxline(&mut utent);
  endutxent();
  /* "man utmp" says wtmp file should *not* be created automagically */
  /*touch(bb_path_wtmp_file);*/
  updwtmpx(bb_path_wtmp_file.as_ptr(), &mut utent);
}
/*
 * Read "man utmp" to make sense out of it.
 */
#[no_mangle]
pub unsafe extern "C" fn update_utmp(
  mut pid: pid_t,
  mut new_type: libc::c_int,
  mut tty_name: *const libc::c_char,
  mut username: *const libc::c_char,
  mut hostname: *const libc::c_char,
) {
  let mut utent: utmpx = utmpx {
    ut_type: 0,
    ut_pid: 0,
    ut_line: [0; 32],
    ut_id: [0; 4],
    ut_user: [0; 32],
    ut_host: [0; 256],
    ut_exit: __exit_status {
      e_termination: 0,
      e_exit: 0,
    },
    ut_session: 0,
    ut_tv: C2RustUnnamed {
      tv_sec: 0,
      tv_usec: 0,
    },
    ut_addr_v6: [0; 4],
    __glibc_reserved: [0; 20],
  };
  let mut utp: *mut utmpx = 0 as *mut utmpx;
  touch(b"/var/run/utmp\x00" as *const u8 as *const libc::c_char);
  //utmpxname(_PATH_UTMPX);
  setutxent();
  loop
  /* Did init/getty/telnetd/sshd/... create an entry for us?
   * It should be (new_type-1), but we'd also reuse
   * any other potentially stale xxx_PROCESS entry */
  {
    utp = getutxent();
    if utp.is_null() {
      break;
    }
    if !((*utp).ut_pid == pid
      && (*utp).ut_id[0] as libc::c_int != 0
      && ((*utp).ut_type as libc::c_int == 5i32
        || (*utp).ut_type as libc::c_int == 6i32
        || (*utp).ut_type as libc::c_int == 7i32
        || (*utp).ut_type as libc::c_int == 8i32))
    {
      continue;
    }
    if (*utp).ut_type as libc::c_int >= new_type {
      /* Stale record. Nuke hostname */
      memset(
        (*utp).ut_host.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
      );
    }
    break;
  }
  //endutxent(); - no need, pututxline can deal with (and actually likes)
  //the situation when utmp file is positioned on found record
  if utp.is_null() {
    if new_type != 8i32 {
      write_new_utmp(pid, new_type, tty_name, username, hostname);
    } else {
      endutxent();
    }
    return;
  }
  /* Make a copy. We can't use *utp, pututxline's internal getutxid
   * will overwrite it before it is used! */
  utent = *utp;
  utent.ut_type = new_type as libc::c_short;
  if !tty_name.is_null() {
    safe_strncpy(
      utent.ut_line.as_mut_ptr(),
      skip_dev_pfx(tty_name),
      ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
    );
  }
  if !username.is_null() {
    safe_strncpy(
      utent.ut_user.as_mut_ptr(),
      username,
      ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
    );
  }
  if !hostname.is_null() {
    safe_strncpy(
      utent.ut_host.as_mut_ptr(),
      hostname,
      ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
    );
  }
  utent.ut_tv.tv_sec = time(0 as *mut time_t) as i32;
  /* Update, or append new one */
  //setutxent();
  pututxline(&mut utent);
  endutxent();
  /* "man utmp" says wtmp file should *not* be created automagically */
  /*touch(bb_path_wtmp_file);*/
  updwtmpx(bb_path_wtmp_file.as_ptr(), &mut utent);
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
/* man utmp:
 * When init(8) finds that a process has exited, it locates its utmp entry
 * by ut_pid, sets ut_type to DEAD_PROCESS, and clears ut_user, ut_host
 * and ut_time with null bytes.
 * [same applies to other processes which maintain utmp entries, like telnetd]
 *
 * We do not bother actually clearing fields:
 * it might be interesting to know who was logged in and from where
 */
#[no_mangle]
pub unsafe extern "C" fn update_utmp_DEAD_PROCESS(mut pid: pid_t) {
  update_utmp(
    pid,
    8i32,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
  );
}
