use crate::librb::smallint;

use crate::libbb::appletlib::applet_name;
use libc;
use libc::pid_t;
use libc::sigaddset;
use libc::sigemptyset;
use libc::sigprocmask;
use libc::sigset_t;
use libc::sigval;
use libc::uid_t;
extern "C" {
  #[no_mangle]
  fn _exit(_: libc::c_int) -> !;
  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
  #[no_mangle]
  fn raise(__sig: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn sigfillset(__set: *mut sigset_t) -> libc::c_int;

  #[no_mangle]
  fn sigsuspend(__set: *const sigset_t) -> libc::c_int;
  #[no_mangle]
  fn sigaction(__sig: libc::c_int, __act: *const sigaction, __oact: *mut sigaction) -> libc::c_int;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
  pub _pad: [libc::c_int; 28],
  pub _kill: C2RustUnnamed_8,
  pub _timer: C2RustUnnamed_7,
  pub _rt: C2RustUnnamed_6,
  pub _sigchld: C2RustUnnamed_5,
  pub _sigfault: C2RustUnnamed_2,
  pub _sigpoll: C2RustUnnamed_1,
  pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
  pub _call_addr: *mut libc::c_void,
  pub _syscall: libc::c_int,
  pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
  pub si_band: libc::c_long,
  pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
  pub si_addr: *mut libc::c_void,
  pub si_addr_lsb: libc::c_short,
  pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
  pub _addr_bnd: C2RustUnnamed_4,
  pub _pkey: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
  pub _lower: *mut libc::c_void,
  pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
  pub si_pid: pid_t,
  pub si_uid: uid_t,
  pub si_status: libc::c_int,
  pub si_utime: libc::clock_t,
  pub si_stime: libc::clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
  pub si_pid: pid_t,
  pub si_uid: uid_t,
  pub si_sigval: sigval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
  pub si_tid: libc::c_int,
  pub si_overrun: libc::c_int,
  pub si_sigval: sigval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
  pub si_pid: pid_t,
  pub si_uid: uid_t,
}
use crate::librb::signal::__sighandler_t;
use crate::librb::signal::sigaction;
/*
 * Utility routines.
 *
 * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
 * Copyright (C) 2006 Rob Landley
 * Copyright (C) 2006 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* All known arches use small ints for signals */
#[no_mangle]
pub static mut bb_got_signal: smallint = 0;
/* Standard handler which just records signo */
#[no_mangle]
pub unsafe extern "C" fn record_signo(mut signo: libc::c_int) {
  bb_got_signal = signo as smallint;
}
/* Saves 2 bytes on x86! Oh my... */
#[no_mangle]
pub unsafe extern "C" fn sigaction_set(
  mut signum: libc::c_int,
  mut act: *const sigaction,
) -> libc::c_int {
  return sigaction(signum, act, 0 as *mut sigaction);
}
#[no_mangle]
pub unsafe extern "C" fn sigprocmask_allsigs(mut how: libc::c_int) -> libc::c_int {
  let mut set: sigset_t = std::mem::zeroed();
  sigfillset(&mut set);
  return sigprocmask(how, &mut set, 0 as *mut sigset_t);
}
/* Will do sigaction(signum, act, NULL): */
/* SIG_BLOCK/SIG_UNBLOCK all signals: */
/* Return old set in the same set: */
#[no_mangle]
pub unsafe extern "C" fn sigprocmask2(mut how: libc::c_int, mut set: *mut sigset_t) -> libc::c_int {
  // Grr... gcc 8.1.1:
  // "passing argument 3 to restrict-qualified parameter aliases with argument 2"
  // dance around that...
  let mut oset: *mut sigset_t = 0 as *mut sigset_t;
  oset = set;
  return sigprocmask(how, set, oset);
}
#[no_mangle]
pub unsafe extern "C" fn bb_signals(
  mut sigs: libc::c_int,
  mut f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
) {
  let mut sig_no: libc::c_int = 0i32;
  let mut bit: libc::c_int = 1i32;
  while sigs != 0 {
    if sigs & bit != 0 {
      sigs -= bit;
      signal(sig_no, f);
    }
    sig_no += 1;
    bit <<= 1i32
  }
}
#[no_mangle]
pub unsafe extern "C" fn bb_signals_recursive_norestart(
  mut sigs: libc::c_int,
  mut f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
) {
  let mut sig_no: libc::c_int = 0i32;
  let mut bit: libc::c_int = 1i32;
  let mut sa: sigaction = std::mem::zeroed();
  memset(
    &mut sa as *mut sigaction as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<sigaction>() as libc::c_ulong,
  );
  sa.__sigaction_handler.sa_handler = f;
  /*sa.sa_flags = 0;*/
  /*sigemptyset(&sa.sa_mask); - hope memset did it*/
  while sigs != 0 {
    if sigs & bit != 0 {
      sigs -= bit;
      sigaction_set(sig_no, &mut sa);
    }
    sig_no += 1;
    bit <<= 1i32
  }
}
#[no_mangle]
pub unsafe extern "C" fn sig_block(mut sig: libc::c_int) {
  let mut ss: sigset_t = std::mem::zeroed();
  sigemptyset(&mut ss);
  sigaddset(&mut ss, sig);
  sigprocmask(0i32, &mut ss, 0 as *mut sigset_t);
}
#[no_mangle]
pub unsafe extern "C" fn sig_unblock(mut sig: libc::c_int) {
  let mut ss: sigset_t = std::mem::zeroed();
  sigemptyset(&mut ss);
  sigaddset(&mut ss, sig);
  sigprocmask(1i32, &mut ss, 0 as *mut sigset_t);
}
#[no_mangle]
pub unsafe extern "C" fn wait_for_any_sig() {
  let mut ss: sigset_t = std::mem::zeroed();
  sigemptyset(&mut ss);
  sigsuspend(&mut ss);
}
/* Assuming the sig is fatal */
#[no_mangle]
pub unsafe extern "C" fn kill_myself_with_sig(mut sig: libc::c_int) -> ! {
  signal(sig, None);
  sig_unblock(sig);
  raise(sig);
  _exit(sig | 128i32);
  /* Should not reach it */
}
/* syscalls like read() won't be interrupted (though select/poll will be): */
#[no_mangle]
pub unsafe extern "C" fn signal_SA_RESTART_empty_mask(
  mut sig: libc::c_int,
  mut handler: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
) {
  let mut sa: sigaction = std::mem::zeroed();
  memset(
    &mut sa as *mut sigaction as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<sigaction>() as libc::c_ulong,
  );
  /*sigemptyset(&sa.sa_mask);*/
  sa.sa_flags = 0x10000000i32;
  sa.__sigaction_handler.sa_handler = handler;
  sigaction_set(sig, &mut sa);
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
#[no_mangle]
pub unsafe extern "C" fn signal_no_SA_RESTART_empty_mask(
  mut sig: libc::c_int,
  mut handler: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
) {
  let mut sa: sigaction = std::mem::zeroed();
  memset(
    &mut sa as *mut sigaction as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<sigaction>() as libc::c_ulong,
  );
  /*sigemptyset(&sa.sa_mask);*/
  /*sa.sa_flags = 0;*/
  sa.__sigaction_handler.sa_handler = handler;
  sigaction_set(sig, &mut sa);
}
