use crate::librb::size_t;
use libc;





use libc::fstat;
use libc::getenv;


use libc::getpid;









use libc::strcpy;














use libc::strchr;





use libc::close;
use libc::free;
use libc::off_t;

use libc::ssize_t;
use libc::stat;
use libc::uid_t;
use libc::unlink;

extern "C" {

  #[no_mangle]
  fn getuid() -> uid_t;

  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn dprintf(__fd: libc::c_int, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  fn bb_copyfd_eof(fd1: libc::c_int, fd2: libc::c_int) -> off_t;
  #[no_mangle]
  fn bb_copyfd_size(fd1: libc::c_int, fd2: libc::c_int, size: off_t) -> off_t;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xlseek(fd: libc::c_int, offset: off_t, whence: libc::c_int) -> off_t;
  #[no_mangle]
  fn xmkstemp(template: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn create_and_connect_stream_or_die(peer: *const libc::c_char, port: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn overlapping_strcpy(dst: *mut libc::c_char, src: *const libc::c_char);
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn full_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn safe_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn safe_gethostname() -> *mut libc::c_char;
  #[no_mangle]
  fn xuid2uname(uid: uid_t) -> *mut libc::c_char;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn xfunc_die() -> !;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  static bb_msg_standard_input: [libc::c_char; 0];
}

// -U username
pub const LPR_V: C2RustUnnamed = 4;
// -J title: the job title for the banner page
pub const LPR_m: C2RustUnnamed = 64;
// -V: be verbose
pub const LPR_h: C2RustUnnamed = 8;
// -C class: job "class" (? supposedly printed on banner)
pub const LPR_J: C2RustUnnamed = 32;
// -m: send mail back to user
pub const LPQ_SHORT_FMT: C2RustUnnamed = 4;
// -s: short listing format
pub const LPQ_DELETE: C2RustUnnamed = 8;
// -d: delete job(s)
pub const LPQ_FORCE: C2RustUnnamed = 16;
pub type C2RustUnnamed = libc::c_uint;
// -f: force waiting job(s) to be printed
// -h: want banner printed
pub const LPR_C: C2RustUnnamed = 16;
// -P queue[@host[:port]]. If no -P is given use $PRINTER, then "lp@localhost:515"
pub const OPT_U: C2RustUnnamed = 2;
pub const OPT_P: C2RustUnnamed = 1;

/*
 * bare bones version of lpr & lpq: BSD printing utilities
 *
 * Copyright (C) 2008 by Vladimir Dronnikov <dronnikov@gmail.com>
 *
 * Original idea and code:
 *      Walter Harms <WHarms@bfs.de>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 *
 * See RFC 1179 for protocol description.
 */
//config:config LPR
//config:	bool "lpr (9.9 kb)"
//config:	default y
//config:	help
//config:	lpr sends files (or standard input) to a print spooling daemon.
//config:
//config:config LPQ
//config:	bool "lpq (9.9 kb)"
//config:	default y
//config:	help
//config:	lpq is a print spool queue examination and manipulation program.
//              APPLET_ODDNAME:name main  location        suid_type     help
//applet:IF_LPQ(APPLET_ODDNAME(lpq, lpqr, BB_DIR_USR_BIN, BB_SUID_DROP, lpq))
//applet:IF_LPR(APPLET_ODDNAME(lpr, lpqr, BB_DIR_USR_BIN, BB_SUID_DROP, lpr))
//kbuild:lib-$(CONFIG_LPR) += lpr.o
//kbuild:lib-$(CONFIG_LPQ) += lpr.o
//usage:#define lpr_trivial_usage
//usage:       "-P queue[@host[:port]] -U USERNAME -J TITLE -Vmh [FILE]..."
/* -C CLASS exists too, not shown.
 * CLASS is supposed to be printed on banner page, if one is requested */
//usage:#define lpr_full_usage "\n\n"
//usage:       "	-P	lp service to connect to (else uses $PRINTER)"
//usage:     "\n	-m	Send mail on completion"
//usage:     "\n	-h	Print banner page too"
//usage:     "\n	-V	Verbose"
//usage:
//usage:#define lpq_trivial_usage
//usage:       "[-P queue[@host[:port]]] [-U USERNAME] [-d JOBID]... [-fs]"
//usage:#define lpq_full_usage "\n\n"
//usage:       "	-P	lp service to connect to (else uses $PRINTER)"
//usage:     "\n	-d	Delete jobs"
//usage:     "\n	-f	Force any waiting job to be printed"
//usage:     "\n	-s	Short display"
/*
 * LPD returns binary 0 on success.
 * Otherwise it returns error message.
 */
unsafe extern "C" fn get_response_or_say_and_die(
  mut fd: libc::c_int,
  mut errmsg: *const libc::c_char,
) {
  let mut sz: ssize_t = 0;
  let mut buf: [libc::c_char; 128] = [0; 128];
  buf[0] = ' ' as i32 as libc::c_char;
  sz = safe_read(fd, buf.as_mut_ptr() as *mut libc::c_void, 1i32 as size_t);
  if '\u{0}' as i32 != buf[0] as libc::c_int {
    // request has failed
    // try to make sure last char is '\n', but do not add
    // superfluous one
    sz = full_read(
      fd,
      buf.as_mut_ptr().offset(1) as *mut libc::c_void,
      126i32 as size_t,
    );
    bb_error_msg(
      b"error while %s%s\x00" as *const u8 as *const libc::c_char,
      errmsg,
      if sz > 0 {
        b". Server said:\x00" as *const u8 as *const libc::c_char
      } else {
        b"\x00" as *const u8 as *const libc::c_char
      },
    );
    if sz > 0 {
      // sz = (bytes in buf) - 1
      if buf[sz as usize] as libc::c_int != '\n' as i32 {
        sz += 1; // printer class, max 32 char
        buf[sz as usize] = '\n' as i32 as libc::c_char
      } // name of printer queue
      safe_write(
        2i32,
        buf.as_mut_ptr() as *const libc::c_void,
        (sz + 1) as size_t,
      ); // server[:port] of printer queue
    }
    xfunc_die();
  };
}
#[no_mangle]
pub unsafe extern "C" fn lpqr_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut tempfile: [libc::c_char; 15] = [0; 15];
  let mut job_title: *const libc::c_char = 0 as *const libc::c_char;
  let mut printer_class: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
  let mut queue: *const libc::c_char = 0 as *const libc::c_char;
  let mut server: *const libc::c_char = b"localhost\x00" as *const u8 as *const libc::c_char;
  let mut hostname: *mut libc::c_char = 0 as *mut libc::c_char;
  // N.B. IMHO getenv("USER") can be way easily spoofed!
  let mut user: *const libc::c_char = xuid2uname(getuid());
  let mut job: libc::c_uint = 0;
  let mut opts: libc::c_uint = 0;
  let mut fd: libc::c_int = 0;
  queue = getenv(b"PRINTER\x00" as *const u8 as *const libc::c_char);
  if queue.is_null() {
    queue = b"lp\x00" as *const u8 as *const libc::c_char
  }
  // parse options
  // TODO: set opt_complementary: s,d,f are mutually exclusive
  opts = getopt32(
    argv,
    if 'r' as i32 == *applet_name.offset(2) as libc::c_int {
      b"P:U:VhC:J:m\x00" as *const u8 as *const libc::c_char
    } else {
      b"P:U:sdf\x00" as *const u8 as *const libc::c_char
    },
    &mut queue as *mut *const libc::c_char,
    &mut user as *mut *const libc::c_char,
    &mut printer_class as *mut *const libc::c_char,
    &mut job_title as *mut *const libc::c_char,
  );
  argv = argv.offset(optind as isize);
  // queue name is to the left of '@'
  let mut s: *mut libc::c_char = strchr(queue, '@' as i32);
  if !s.is_null() {
    // server name is to the right of '@'
    *s = '\u{0}' as i32 as libc::c_char;
    server = s.offset(1)
  }
  // do connect
  fd = create_and_connect_stream_or_die(server, 515i32);
  //
  // LPQ ------------------------
  //
  if 'q' as i32 == *applet_name.offset(2) as libc::c_int {
    let mut cmd: libc::c_char = 0;
    let mut current_block_20: u64;
    // force printing of every job still in queue
    if opts & LPQ_FORCE as libc::c_int as libc::c_uint != 0 {
      cmd = 1i32 as libc::c_char;
      current_block_20 = 4839426858821150325;
    // delete job(s)
    } else if opts & LPQ_DELETE as libc::c_int as libc::c_uint != 0 {
      dprintf(
        fd,
        b"\x05%s %s\x00" as *const u8 as *const libc::c_char,
        queue,
        user,
      );
      while !(*argv).is_null() {
        let fresh0 = argv;
        argv = argv.offset(1);
        dprintf(fd, b" %s\x00" as *const u8 as *const libc::c_char, *fresh0);
      }
      bb_putchar('\n' as i32);
      current_block_20 = 17788412896529399552;
    // dump current jobs status
    // N.B. periodical polling should be achieved
    // via "watch -n delay lpq"
    // They say it's the UNIX-way :)
    } else {
      cmd = if opts & LPQ_SHORT_FMT as libc::c_int as libc::c_uint != 0 {
        3i32
      } else {
        4i32
      } as libc::c_char;
      current_block_20 = 4839426858821150325;
    }
    match current_block_20 {
      4839426858821150325 => {
        dprintf(
          fd,
          b"%c%s\n\x00" as *const u8 as *const libc::c_char,
          cmd as libc::c_int,
          queue,
        );
        bb_copyfd_eof(fd, 1i32);
      }
      _ => {}
    }
    return 0i32;
  }
  //
  // LPR ------------------------
  //
  if opts & LPR_V as libc::c_int as libc::c_uint != 0 {
    bb_simple_error_msg(b"connected to server\x00" as *const u8 as *const libc::c_char);
  }
  job = (getpid() % 1000i32) as libc::c_uint;
  hostname = safe_gethostname();
  // no files given on command line? -> use stdin
  if (*argv).is_null() {
    argv = argv.offset(-1);
    *argv = b"-\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
  }
  dprintf(
    fd,
    b"\x02%s\n\x00" as *const u8 as *const libc::c_char,
    queue,
  );
  get_response_or_say_and_die(fd, b"setting queue\x00" as *const u8 as *const libc::c_char);
  loop
  // process files
  {
    let mut cflen: libc::c_uint = 0;
    let mut dfd: libc::c_int = 0;
    let mut st: stat = std::mem::zeroed();
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut remote_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut controlfile: *mut libc::c_char = 0 as *mut libc::c_char;
    // if data file is stdin, we need to dump it first
    if *(*argv).offset(0) as libc::c_int == '-' as i32 && *(*argv).offset(1) == 0 {
      strcpy(
        tempfile.as_mut_ptr(),
        b"/tmp/lprXXXXXX\x00" as *const u8 as *const libc::c_char,
      ); /* paranoia: fstat may theoretically fail */
      dfd = xmkstemp(tempfile.as_mut_ptr());
      bb_copyfd_eof(0i32, dfd);
      xlseek(dfd, 0i32 as off_t, 0i32);
      *argv = bb_msg_standard_input.as_ptr() as *mut libc::c_char
    } else {
      dfd = xopen(*argv, 0i32)
    }
    st.st_size = 0i32 as off_t;
    fstat(dfd, &mut st);
    /* Apparently, some servers are buggy and won't accept 0-sized jobs.
     * Standard lpr works around it by refusing to send such jobs:
     */
    if st.st_size == 0 {
      bb_simple_error_msg(b"nothing to print\x00" as *const u8 as *const libc::c_char);
    } else {
      /* "The name ... should start with ASCII "cfA",
       * followed by a three digit job number, followed
       * by the host name which has constructed the file."
       * We supply 'c' or 'd' as needed for control/data file. */
      remote_filename = xasprintf(
        b"fA%03u%s\x00" as *const u8 as *const libc::c_char,
        job,
        hostname,
      );
      // create control file
      // TODO: all lines but 2 last are constants! How we can use this fact?
      controlfile = xasprintf(
        b"H%.32s\nP%.32s\nC%.32s\nJ%.99s\nL%.32s\nM%.32s\nld%.31s\n\x00" as *const u8
          as *const libc::c_char,
        hostname,
        user,
        printer_class,
        if opts & LPR_J as libc::c_int as libc::c_uint != 0 {
          job_title
        } else {
          *argv
        },
        if opts & LPR_h as libc::c_int as libc::c_uint != 0 {
          user
        } else {
          b"\x00" as *const u8 as *const libc::c_char
        },
        if opts & LPR_m as libc::c_int as libc::c_uint != 0 {
          user
        } else {
          b"\x00" as *const u8 as *const libc::c_char
        },
        remote_filename,
      );
      // delete possible "\nX\n" (that is, one-char) patterns
      c = controlfile;
      loop {
        c = strchr(c, '\n' as i32);
        if c.is_null() {
          break;
        }
        if *c.offset(1) as libc::c_int != 0 && *c.offset(2) as libc::c_int == '\n' as i32 {
          overlapping_strcpy(c, c.offset(2));
        } else {
          c = c.offset(1)
        }
      }
      // send control file
      if opts & LPR_V as libc::c_int as libc::c_uint != 0 {
        bb_simple_error_msg(b"sending control file\x00" as *const u8 as *const libc::c_char);
      }
      /* "Acknowledgement processing must occur as usual
       * after the command is sent." */
      cflen = strlen(controlfile) as libc::c_uint;
      dprintf(
        fd,
        b"\x02%u c%s\n\x00" as *const u8 as *const libc::c_char,
        cflen,
        remote_filename,
      );
      get_response_or_say_and_die(
        fd,
        b"sending control file\x00" as *const u8 as *const libc::c_char,
      );
      /* "Once all of the contents have
       * been delivered, an octet of zero bits is sent as
       * an indication that the file being sent is complete.
       * A second level of acknowledgement processing
       * must occur at this point." */
      full_write(
        fd,
        controlfile as *const libc::c_void,
        cflen.wrapping_add(1i32 as libc::c_uint) as size_t,
      ); /* writes NUL byte too */
      get_response_or_say_and_die(
        fd,
        b"sending control file\x00" as *const u8 as *const libc::c_char,
      );
      // send data file, with name "dfaXXX"
      if opts & LPR_V as libc::c_int as libc::c_uint != 0 {
        bb_simple_error_msg(b"sending data file\x00" as *const u8 as *const libc::c_char);
      }
      dprintf(
        fd,
        b"\x03%lu d%s\n\x00" as *const u8 as *const libc::c_char,
        st.st_size,
        remote_filename,
      );
      get_response_or_say_and_die(
        fd,
        b"sending data file\x00" as *const u8 as *const libc::c_char,
      );
      if bb_copyfd_size(dfd, fd, st.st_size) != st.st_size {
        // We're screwed. We sent less bytes than we advertised.
        bb_simple_error_msg_and_die(
          b"local file changed size?!\x00" as *const u8 as *const libc::c_char,
        ); // send ACK
      }
      write(
        fd,
        b"\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        1i32 as size_t,
      );
      get_response_or_say_and_die(
        fd,
        b"sending data file\x00" as *const u8 as *const libc::c_char,
      );
      // delete temporary file if we dumped stdin
      if *argv == bb_msg_standard_input.as_ptr() as *mut libc::c_char {
        unlink(tempfile.as_mut_ptr());
      }
      // cleanup
      close(fd);
      free(remote_filename as *mut libc::c_void);
      free(controlfile as *mut libc::c_void);
      // say job accepted
      if opts & LPR_V as libc::c_int as libc::c_uint != 0 {
        bb_simple_error_msg(b"job accepted\x00" as *const u8 as *const libc::c_char);
      }
      // next, please!
      job = job
        .wrapping_add(1i32 as libc::c_uint)
        .wrapping_rem(1000i32 as libc::c_uint)
    }
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  return 0i32;
}
