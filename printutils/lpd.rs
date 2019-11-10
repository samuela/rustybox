use libc;
use libc::putenv;
use libc::umask;
use libc::mknod;
use libc::fchmod;
use libc::fscanf;
use libc::alarm;
use libc::sync;
use libc::setsid;
use libc::ioctl;
use libc::statfs;
use libc::mount;
use libc::prctl;
use libc::opendir;
use libc::closedir;
use libc::readdir;
use libc::strtok;
use libc::putchar_unlocked;
use libc::endmntent;
use libc::setmntent;
use libc::umount2;
use libc::getegid;
use libc::getuid;
use libc::getgid;
use libc::setutxent;
use libc::endutxent;
use libc::chdir;

use libc::printf;
use libc::puts;

use libc::strchr;

use libc::close;
use libc::free;
use libc::unlink;

extern "C" {

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  static bb_errno: *mut libc::c_int;

  #[no_mangle]
  fn chomp(s: *mut libc::c_char);

  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn bb_copyfd_size(fd1: libc::c_int, fd2: libc::c_int, size: off_t) -> off_t;

  #[no_mangle]
  fn xdup2(_: libc::c_int, _: libc::c_int);

  #[no_mangle]
  fn xchdir(path: *const libc::c_char);

  #[no_mangle]
  fn xsetenv(key: *const libc::c_char, value: *const libc::c_char);

  #[no_mangle]
  fn open3_or_warn(
    pathname: *const libc::c_char,
    flags: libc::c_int,
    mode: libc::c_int,
  ) -> libc::c_int;

  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;

  #[no_mangle]
  fn xmalloc_reads(fd: libc::c_int, maxsz_p: *mut size_t) -> *mut libc::c_char;

  #[no_mangle]
  fn xmalloc_xopen_read_close(
    filename: *const libc::c_char,
    maxsz_p: *mut size_t,
  ) -> *mut libc::c_void;

  #[no_mangle]
  fn safe_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;

  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;

  #[no_mangle]
  fn BB_EXECVP_or_die(argv: *mut *mut libc::c_char) -> !;

  #[no_mangle]
  fn bb_daemonize_or_rexec(flags: libc::c_int);
}

use libc::mode_t;

use crate::librb::size_t;
use libc::off_t;
use libc::ssize_t;
pub type C2RustUnnamed = libc::c_uint;
pub const DAEMON_ONLY_SANITIZE: C2RustUnnamed = 8;
pub const DAEMON_CLOSE_EXTRA_FDS: C2RustUnnamed = 4;
pub const DAEMON_DEVNULL_STDIO: C2RustUnnamed = 2;
pub const DAEMON_CHDIR_ROOT: C2RustUnnamed = 1;

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
 * micro lpd
 *
 * Copyright (C) 2008 by Vladimir Dronnikov <dronnikov@gmail.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/*
 * A typical usage of BB lpd looks as follows:
 * # tcpsvd -E 0 515 lpd [SPOOLDIR] [HELPER-PROG [ARGS...]]
 *
 * This starts TCP listener on port 515 (default for LP protocol).
 * When a client connection is made (via lpr) lpd first changes its
 * working directory to SPOOLDIR (current dir is the default).
 *
 * SPOOLDIR is the spool directory which contains printing queues
 * and should have the following structure:
 *
 * SPOOLDIR/
 *      <queue1>
 *      ...
 *      <queueN>
 *
 * <queueX> can be of two types:
 *      A. a printer character device, an ordinary file or a link to such;
 *      B. a directory.
 *
 * In case A lpd just dumps the data it receives from client (lpr) to the
 * end of queue file/device. This is non-spooling mode.
 *
 * In case B lpd enters spooling mode. It reliably saves client data along
 * with control info in two unique files under the queue directory. These
 * files are named dfAXXXHHHH and cfAXXXHHHH, where XXX is the job number
 * and HHHH is the client hostname. Unless a printing helper application
 * is specified lpd is done at this point.
 *
 * NB: file names are produced by peer! They actually may be anything at all.
 * lpd only sanitizes them (by removing most non-alphanumerics).
 *
 * If HELPER-PROG (with optional arguments) is specified then lpd continues
 * to process client data:
 *      1. it reads and parses control file (cfA...). The parse process
 *      results in setting environment variables whose values were passed
 *      in control file; when parsing is complete, lpd deletes control file.
 *      2. it spawns specified helper application. It is then
 *      the helper application who is responsible for both actual printing
 *      and deleting of processed data file.
 *
 * A good lpr passes control files which when parsed provides the following
 * variables:
 * $H = host which issues the job
 * $P = user who prints
 * $C = class of printing (what is printed on banner page)
 * $J = the name of the job
 * $L = print banner page
 * $M = the user to whom a mail should be sent if a problem occurs
 *
 * We specifically filter out and NOT provide:
 * $l = name of datafile ("dfAxxx") - file whose content are to be printed
 *
 * lpd provides $DATAFILE instead - the ACTUAL name
 * of the datafile under which it was saved.
 * $l would be not reliable (you would be at mercy of remote peer).
 *
 * Thus, a typical helper can be something like this:
 * #!/bin/sh
 * cat ./"$DATAFILE" >/dev/lp0
 * mv -f ./"$DATAFILE" save/
 */
//config:config LPD
//config:	bool "lpd (5.5 kb)"
//config:	default y
//config:	help
//config:	lpd is a print spooling daemon.
//applet:IF_LPD(APPLET(lpd, BB_DIR_USR_SBIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_LPD) += lpd.o
//usage:#define lpd_trivial_usage
//usage:       "SPOOLDIR [HELPER [ARGS]]"
//usage:#define lpd_full_usage "\n\n"
//usage:       "SPOOLDIR must contain (symlinks to) device nodes or directories"
//usage:     "\nwith names matching print queue names. In the first case, jobs are"
//usage:     "\nsent directly to the device. Otherwise each job is stored in queue"
//usage:     "\ndirectory and HELPER program is called. Name of file to print"
//usage:     "\nis passed in $DATAFILE variable."
//usage:     "\nExample:"
//usage:     "\n	tcpsvd -E 0 515 softlimit -m 999999 lpd /var/spool ./print"
// strip argument of bad chars
unsafe extern "C" fn sane(mut string: *mut libc::c_char) -> *mut libc::c_char {
  let mut s: *mut libc::c_char = string;
  let mut p: *mut libc::c_char = s;
  while *s != 0 {
    if bb_ascii_isalnum(*s as libc::c_uchar) != 0
      || '-' as i32 == *s as libc::c_int
      || '_' as i32 == *s as libc::c_int
    {
      let fresh0 = p;
      p = p.offset(1);
      *fresh0 = *s
    }
    s = s.offset(1)
  }
  *p = '\u{0}' as i32 as libc::c_char;
  return string;
}

unsafe extern "C" fn xmalloc_read_stdin() -> *mut libc::c_char {
  // SECURITY:
  let mut max: size_t = (4i32 * 1024i32) as size_t; // more than enough for commands!
  return xmalloc_reads(0i32, &mut max); // for compiler
}

#[no_mangle]
pub unsafe extern "C" fn lpd_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut spooling: libc::c_int = 0;
  spooling = spooling;
  let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut queue: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut filenames: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];

  // goto spool directory
  argv = argv.offset(1);
  if !(*argv).is_null() {
    let fresh1 = argv;
    argv = argv.offset(1);
    xchdir(*fresh1);
  }

  // error messages of xfuncs will be sent over network
  xdup2(1i32, 2i32);

  // nullify ctrl/data filenames
  memset(
    filenames.as_mut_ptr() as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<[*mut libc::c_char; 2]>() as libc::c_ulong,
  );

  // read command
  queue = xmalloc_read_stdin();
  s = queue;
  if s.is_null() {
    // eof?
    return 1i32;
  }

  // we understand only "receive job" command
  if 2i32 != *queue as libc::c_int {
    current_block = 5976951161683668841;
  } else {
    // parse command: "2 | QUEUE_NAME | '\n'"
    queue = queue.offset(1);
    // protect against "/../" attacks
    // *strchrnul(queue, '\n') = '\0'; - redundant, sane() will do
    if *sane(queue) == 0 {
      return 1i32;
    }

    // queue is a directory -> chdir to it and enter spooling mode
    spooling = chdir(queue) + 1i32; // 0: cannot chdir, 1: done
                                    // we don't free(s), we might need "queue" var later

    loop {
      let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
      let mut fd: libc::c_int = 0; // while (1)
                                   // NB: can do close(-1). Who cares?
                                   // NB: don't do "signal OK" write here, it will be done
                                   // at the top of the loop
                                   // int is easier than ssize_t: can use xatoi_positive,
                                   // and can correctly display error returns (-1)
      let mut expected_len: libc::c_int = 0;
      let mut real_len: libc::c_int = 0;
      // signal OK
      safe_write(
        1i32,
        b"\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        1i32 as size_t,
      );
      // get subcommand
      // valid s must be of form: "SUBCMD | LEN | space | FNAME"
      // N.B. we bail out on any error
      s = xmalloc_read_stdin();
      if s.is_null() {
        // (probably) EOF
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut var: [libc::c_char; 2] = [0; 2];
        // non-spooling mode or no spool helper specified
        if spooling == 0 || (*argv).is_null() {
          return 0i32;
        } // the only non-error exit
          // spooling mode but we didn't see both ctrlfile & datafile
        if spooling != 7i32 {
          current_block = 12481496603591474651; // reject job
          break;
        }
        // spooling mode and spool helper specified -> exec spool helper
        // (we exit 127 if helper cannot be executed)
        var[1] = '\u{0}' as i32 as libc::c_char;
        // read and delete ctrlfile
        q = xmalloc_xopen_read_close(filenames[0], 0 as *mut size_t) as *mut libc::c_char;
        unlink(filenames[0]);
        // provide datafile name
        // we can use leaky setenv since we are about to exec or exit
        xsetenv(
          b"DATAFILE\x00" as *const u8 as *const libc::c_char,
          filenames[1],
        );
        loop
        // parse control file by "\n"
        {
          p = strchr(q, '\n' as i32);
          if !(!p.is_null()
            && ((*q as libc::c_int | 0x20i32) - 'a' as i32) as libc::c_uchar as libc::c_int
              <= 'z' as i32 - 'a' as i32)
          {
            break;
          }
          let fresh2 = p;
          p = p.offset(1);
          *fresh2 = '\u{0}' as i32 as libc::c_char;
          // next line
          if *q as libc::c_int != 'l' as i32 {
            let fresh3 = q;
            q = q.offset(1);
            var[0] = *fresh3;
            xsetenv(var.as_mut_ptr(), q);
          }
          q = p
        }
        // q is a line of <SYM><VALUE>,
        // we are setting environment string <SYM>=<VALUE>.
        // Ignoring "l<datafile>", exporting others:
        // helper should not talk over network.
        // this call reopens stdio fds to "/dev/null".
        bb_daemonize_or_rexec(
          DAEMON_DEVNULL_STDIO as libc::c_int | DAEMON_ONLY_SANITIZE as libc::c_int,
        );
        BB_EXECVP_or_die(argv);
      } else {
        // validate input.
        // we understand only "control file" or "data file" subcmds
        if 2i32 != *s.offset(0) as libc::c_int && 3i32 != *s.offset(0) as libc::c_int {
          current_block = 5976951161683668841;
          break;
        }
        if spooling & 1i32 << *s.offset(0) as libc::c_int - 1i32 != 0 {
          puts(b"Duplicated subcommand\x00" as *const u8 as *const libc::c_char);
          current_block = 12481496603591474651;
          break;
        } else {
          // get filename
          chomp(s);
          fname = strchr(s, ' ' as i32);
          if fname.is_null() {
            // bad_fname:
            puts(b"No or bad filename\x00" as *const u8 as *const libc::c_char);
            current_block = 12481496603591474651;
            break;
          } else {
            let fresh4 = fname;
            fname = fname.offset(1);
            *fresh4 = '\u{0}' as i32 as libc::c_char;
            //		// s[0]==2: ctrlfile, must start with 'c'
            //		// s[0]==3: datafile, must start with 'd'
            //		if (fname[0] != s[0] + ('c'-2))
            //			goto bad_fname;
            // get length
            expected_len =
              bb_strtou(s.offset(1), 0 as *mut *mut libc::c_char, 10i32) as libc::c_int;
            if *bb_errno != 0 || expected_len < 0i32 {
              puts(b"Bad length\x00" as *const u8 as *const libc::c_char);
              current_block = 12481496603591474651;
              break;
            } else if 2i32 == *s.offset(0) as libc::c_int && expected_len > 16i32 * 1024i32 {
              // SECURITY:
              // ctrlfile can't be big (we want to read it back later!)
              puts(b"File is too big\x00" as *const u8 as *const libc::c_char);
              current_block = 12481496603591474651;
              break;
            } else {
              // open the file
              if spooling != 0 {
                // spooling mode: dump both files
                // job in flight has mode 0200 "only writable"
                sane(fname);
                fd = open3_or_warn(fname, 0o100i32 | 0o1i32 | 0o1000i32 | 0o200i32, 0o200i32);
                if fd < 0i32 {
                  current_block = 12481496603591474651;
                  break;
                }
                filenames[(*s.offset(0) as libc::c_int - 2i32) as usize] = xstrdup(fname)
              } else {
                // non-spooling mode:
                // 2: control file (ignoring), 3: data file
                fd = -1i32;
                if 3i32 == *s.offset(0) as libc::c_int {
                  fd = xopen(queue, 0o2i32 | 0o2000i32)
                }
              }
              // signal OK
              safe_write(
                1i32,
                b"\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                1i32 as size_t,
              );
              // copy the file
              real_len = bb_copyfd_size(0i32, fd, expected_len as off_t) as libc::c_int;
              if real_len != expected_len {
                printf(
                  b"Expected %d but got %d bytes\n\x00" as *const u8 as *const libc::c_char,
                  expected_len,
                  real_len,
                );
                current_block = 12481496603591474651;
                break;
              } else {
                // get EOF indicator, see whether it is NUL (ok)
                // (and don't trash s[0]!)
                if safe_read(
                  0i32,
                  &mut *s.offset(1) as *mut libc::c_char as *mut libc::c_void,
                  1i32 as size_t,
                ) != 1
                  || *s.offset(1) as libc::c_int != 0i32
                {
                  current_block = 12481496603591474651;
                  break;
                }
                if spooling != 0 {
                  // chmod completely downloaded file as "readable+writable"
                  fchmod(fd, 0o600i32 as mode_t);
                  // bit 1: ctrlfile; bit 2: datafile
                  spooling |= 1i32 << *s.offset(0) as libc::c_int - 1i32
                }
                free(s as *mut libc::c_void);
                close(fd);
              }
            }
          }
        }
      }
    }
  }
  match current_block {
    5976951161683668841 => {
      printf(
        b"Command %02x %s\n\x00" as *const u8 as *const libc::c_char,
        *s.offset(0) as libc::c_uchar as libc::c_int,
        b"is not supported\x00" as *const u8 as *const libc::c_char,
      );
    }
    _ => {}
  }
  // accumulate dump state
  // N.B. after all files are dumped spooling should be 1+2+4==7
  // don't send error msg to peer - it obviously
  // doesn't follow the protocol, so probably
  // it can't understand us either
  // don't keep corrupted files
  if spooling != 0 {
    let mut i: libc::c_int = 0;
    i = 2i32;
    loop {
      i -= 1;
      if !(i >= 0i32) {
        break;
      }
      if !filenames[i as usize].is_null() {
        unlink(filenames[i as usize]);
      }
    }
  }
  return 1i32;
}
