use libc;
use libc::chdir;
use libc::chmod;
use libc::chown;
use libc::closelog;
use libc::dup2;
use libc::fstat;
use libc::getenv;
use libc::geteuid;
use libc::getopt;
use libc::getpid;
use libc::isatty;
use libc::kill;
use libc::openlog;
use libc::sigaddset;
use libc::sigemptyset;
use libc::sigprocmask;
use libc::sleep;
use libc::sscanf;
use libc::strcasecmp;
use libc::strcpy;
use libc::symlink;
use libc::syscall;
use libc::syslog;
use libc::time;
use libc::access;
use libc::atoi;
use libc::fclose;
use libc::fprintf;
use libc::lstat;
use libc::printf;
use libc::puts;
use libc::rename;
use libc::rmdir;
use libc::sprintf;
use libc::strchr;
use libc::strcmp;
use libc::strrchr;
use libc::strstr;
use libc::system;




extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
  #[no_mangle]
  static mut stdin: *mut FILE;
  #[no_mangle]
  static mut stdout: *mut FILE;
  #[no_mangle]
  fn setbuf(__stream: *mut FILE, __buf: *mut libc::c_char);
  #[no_mangle]
  fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t, __s: *mut FILE) -> size_t;
  #[no_mangle]
  static bb_msg_standard_input: [libc::c_char; 0];
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf_0: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn die_if_ferror(file: *mut FILE, msg: *const libc::c_char);
  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;
  #[no_mangle]
  fn fopen_or_warn(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

use crate::librb::signal::__sighandler_t;
use crate::librb::size_t;
use libc::ssize_t;

use libc::FILE;
pub type C2RustUnnamed = libc::c_uint;
pub const COMMON_BUFSIZE: C2RustUnnamed = 1024;

/*
 * tee implementation for busybox
 *
 * Copyright (C) 2003  Manuel Novoa III  <mjn3@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config TEE
//config:	bool "tee (4.2 kb)"
//config:	default y
//config:	help
//config:	tee is used to read from standard input and write
//config:	to standard output and files.
//config:
//config:config FEATURE_TEE_USE_BLOCK_IO
//config:	bool "Enable block I/O (larger/faster) instead of byte I/O"
//config:	default y
//config:	depends on TEE
//config:	help
//config:	Enable this option for a faster tee, at expense of size.
//applet:IF_TEE(APPLET(tee, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_TEE) += tee.o
/* BB_AUDIT SUSv3 compliant */
/* http://www.opengroup.org/onlinepubs/007904975/utilities/tee.html */
//usage:#define tee_trivial_usage
//usage:       "[-ai] [FILE]..."
//usage:#define tee_full_usage "\n\n"
//usage:       "Copy stdin to each FILE, and also to stdout\n"
//usage:     "\n	-a	Append to the given FILEs, don't overwrite"
//usage:     "\n	-i	Ignore interrupt signals (SIGINT)"
//usage:
//usage:#define tee_example_usage
//usage:       "$ echo \"Hello\" | tee /tmp/foo\n"
//usage:       "$ cat /tmp/foo\n"
//usage:       "Hello\n"
// Bare "tee" with no below options does not install SIGPIPE handler - just dies on it.
// TODO:
//	--output-error[=MODE]
//		'warn'		diagnose errors writing to any output
//		'warn-nopipe'	diagnose errors writing to any output not a pipe
//		'exit'		exit on error writing to any output
//		'exit-nopipe'	exit on error writing to any output not a pipe
// ^^^ all of these should set SIGPIPE to SIG_IGN.
// Because "exit" mode should print error message and exit1(1) - not die on SIGPIPE.
// "exit-nopipe" does not exit on EPIPE and does not set exitcode to 1 too.
//	-p	diagnose errors writing to non pipes
// ^^^^ this should set SIGPIPE to SIG_IGN. EPIPE is ignored (same as "warn-nopipe")
#[no_mangle]
pub unsafe extern "C" fn tee_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut mode: *const libc::c_char = b"w\x00a\x00" as *const u8 as *const libc::c_char;
  let mut files: *mut *mut FILE = 0 as *mut *mut FILE;
  let mut fp: *mut *mut FILE = 0 as *mut *mut FILE;
  let mut names: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut np: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut retval: libc::c_char = 0;
  //TODO: make unconditional
  let mut c: ssize_t = 0; /* 'a' must be 2nd */
  retval = getopt32(argv, b"ia\x00" as *const u8 as *const libc::c_char) as libc::c_char; /* Since 'a' is the 2nd option... */
  argc -= optind;
  argv = argv.offset(optind as isize);
  mode = mode.offset((retval as libc::c_int & 2i32) as isize);
  if retval as libc::c_int & 1i32 != 0 {
    signal(
      2i32,
      ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
    );
  }
  retval = 0i32 as libc::c_char;
  /* if (opt_p || opt_output_error)
   signal(SIGPIPE, SIG_IGN);
  */
  /* Allocate an array of FILE *'s, with one extra for a sentinel. */
  files = xzalloc(
    (::std::mem::size_of::<*mut FILE>() as libc::c_ulong)
      .wrapping_mul((argc + 2i32) as libc::c_ulong),
  ) as *mut *mut FILE; /* tee must not buffer output. */
  fp = files;
  names = argv.offset(-1);
  np = names;
  let ref mut fresh0 = *files.offset(0);
  *fresh0 = stdout;
  'c_8862: loop {
    setbuf(*fp, 0 as *mut libc::c_char);
    fp = fp.offset(1);
    np = np.offset(1);
    loop {
      if (*argv).is_null() {
        break 'c_8862;
      }
      *fp = stdout;
      if !(*(*argv).offset(0) as libc::c_int != '-' as i32
        || *(*argv).offset(1) as libc::c_int != 0)
      {
        break;
      }
      *fp = fopen_or_warn(*argv, mode);
      if !(*fp).is_null() {
        break;
      }
      retval = 1i32 as libc::c_char;
      argv = argv.offset(1)
    }
    let fresh1 = argv;
    argv = argv.offset(1);
    *np = *fresh1
  }
  loop
  /* names[0] will be filled later */
  {
    c = safe_read(
      0i32,
      bb_common_bufsiz1.as_mut_ptr() as *mut libc::c_void,
      COMMON_BUFSIZE as libc::c_int as size_t,
    );
    if !(c > 0) {
      break;
    }
    fp = files;
    loop {
      fwrite(
        bb_common_bufsiz1.as_mut_ptr() as *const libc::c_void,
        1i32 as size_t,
        c as size_t,
        *fp,
      );
      fp = fp.offset(1);
      if (*fp).is_null() {
        break;
      }
    }
  }
  if c < 0 {
    /* Make sure read errors are signaled. */
    retval = 1i32 as libc::c_char
  }
  /* Now we need to check for i/o errors on stdin and the various
   * output files.  Since we know that the first entry in the output
   * file table is stdout, we can save one "if ferror" test by
   * setting the first entry to stdin and checking stdout error
   * status with fflush_stdout_and_exit()... although fflush()ing
   * is unnecessary here. */
  np = names;
  fp = files;
  let ref mut fresh2 = *names.offset(0);
  *fresh2 = bb_msg_standard_input.as_ptr() as *mut libc::c_char;
  let ref mut fresh3 = *files.offset(0);
  *fresh3 = stdin;
  loop {
    /* Now check for input and output errors. */
    /* Checking ferror should be sufficient, but we may want to fclose.
     * If we do, remember not to close stdin! */
    let fresh4 = fp;
    fp = fp.offset(1);
    let fresh5 = np;
    np = np.offset(1);
    die_if_ferror(*fresh4, *fresh5);
    if (*fp).is_null() {
      break;
    }
  }
  fflush_stdout_and_exit(retval as libc::c_int);
}
