use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
extern "C" {
  pub type sockaddr_x25;
  pub type sockaddr_un;
  pub type sockaddr_ns;
  pub type sockaddr_iso;
  pub type sockaddr_ipx;
  pub type sockaddr_inarp;
  pub type sockaddr_eon;
  pub type sockaddr_dl;
  pub type sockaddr_ax25;
  pub type sockaddr_at;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn exit(_: libc::c_int) -> !;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  fn select(
    __nfds: libc::c_int,
    __readfds: *mut fd_set,
    __writefds: *mut fd_set,
    __exceptfds: *mut fd_set,
    __timeout: *mut timeval,
  ) -> libc::c_int;
  #[no_mangle]
  fn accept(__fd: libc::c_int, __addr: __SOCKADDR_ARG, __addr_len: *mut socklen_t) -> libc::c_int;
  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
  #[no_mangle]
  fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xgetpty(line: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn ndelay_on(fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn close_on_exec_on(fd: libc::c_int);
  #[no_mangle]
  fn xdup2(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn bb_signals(sigs: libc::c_int, f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>);
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xlisten(s: libc::c_int, backlog: libc::c_int);
  #[no_mangle]
  fn setsockopt_keepalive(fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn create_and_bind_stream_or_die(bindaddr: *const libc::c_char, port: libc::c_int)
    -> libc::c_int;
  #[no_mangle]
  fn get_peer_lsa(fd: libc::c_int) -> *mut len_and_sockaddr;
  #[no_mangle]
  fn xmalloc_sockaddr2dotted(sa: *const sockaddr) -> *mut libc::c_char;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn safe_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn xatou16(numstr: *const libc::c_char) -> u16;
  #[no_mangle]
  fn write_new_utmp(
    pid: pid_t,
    new_type: libc::c_int,
    tty_name: *const libc::c_char,
    username: *const libc::c_char,
    hostname: *const libc::c_char,
  );
  #[no_mangle]
  fn update_utmp_DEAD_PROCESS(pid: pid_t);
  #[no_mangle]
  fn wait_any_nohang(wstat: *mut libc::c_int) -> pid_t;
  #[no_mangle]
  fn bb_daemonize_or_rexec(flags: libc::c_int);
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  static mut logmode: smallint;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn print_login_issue(issue_file: *const libc::c_char, tty: *const libc::c_char);
  #[no_mangle]
  fn tcsetattr_stdin_TCSANOW(tp: *const termios) -> libc::c_int;
  #[no_mangle]
  fn usleep(__useconds: __useconds_t) -> libc::c_int;
  #[no_mangle]
  fn execvp(__file: *const libc::c_char, __argv: *const *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn _exit(_: libc::c_int) -> !;
  #[no_mangle]
  fn tcsetpgrp(__fd: libc::c_int, __pgrp_id: __pid_t) -> libc::c_int;
  #[no_mangle]
  fn vfork() -> libc::c_int;
  #[no_mangle]
  fn setsid() -> __pid_t;
  #[no_mangle]
  fn getpid() -> __pid_t;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
  #[no_mangle]
  fn openlog(__ident: *const libc::c_char, __option: libc::c_int, __facility: libc::c_int);
}

use crate::librb::__pid_t;
use crate::librb::__suseconds_t;
use crate::librb::__time_t;
use crate::librb::__useconds_t;

pub type __socklen_t = libc::c_uint;
use crate::librb::pid_t;
use crate::librb::size_t;
use crate::librb::smallint;
use crate::librb::ssize_t;



pub type socklen_t = __socklen_t;
 use libc::timeval;
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
  pub fds_bits: [__fd_mask; 16],
}
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
  pub sa_family: sa_family_t,
  pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
  pub __sockaddr__: *mut sockaddr,
  pub __sockaddr_at__: *mut sockaddr_at,
  pub __sockaddr_ax25__: *mut sockaddr_ax25,
  pub __sockaddr_dl__: *mut sockaddr_dl,
  pub __sockaddr_eon__: *mut sockaddr_eon,
  pub __sockaddr_in__: *mut sockaddr_in,
  pub __sockaddr_in6__: *mut sockaddr_in6,
  pub __sockaddr_inarp__: *mut sockaddr_inarp,
  pub __sockaddr_ipx__: *mut sockaddr_ipx,
  pub __sockaddr_iso__: *mut sockaddr_iso,
  pub __sockaddr_ns__: *mut sockaddr_ns,
  pub __sockaddr_un__: *mut sockaddr_un,
  pub __sockaddr_x25__: *mut sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
  pub sin6_family: sa_family_t,
  pub sin6_port: in_port_t,
  pub sin6_flowinfo: u32,
  pub sin6_addr: in6_addr,
  pub sin6_scope_id: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
  pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
  pub __u6_addr8: [u8; 16],
  pub __u6_addr16: [u16; 8],
  pub __u6_addr32: [u32; 4],
}
pub type in_port_t = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
  pub sin_family: sa_family_t,
  pub sin_port: in_port_t,
  pub sin_addr: in_addr,
  pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
  pub s_addr: in_addr_t,
}
pub type in_addr_t = u32;
use crate::librb::signal::__sighandler_t;

use crate::librb::termios;
use crate::librb::winsize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct len_and_sockaddr {
  pub len: socklen_t,
  pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
  pub sa: sockaddr,
  pub sin: sockaddr_in,
  pub sin6: sockaddr_in6,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const LOGMODE_BOTH: C2RustUnnamed_1 = 3;
pub const LOGMODE_SYSLOG: C2RustUnnamed_1 = 2;
pub const LOGMODE_STDIO: C2RustUnnamed_1 = 1;
pub const LOGMODE_NONE: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub sessions: *mut tsession,
  pub loginpath: *const libc::c_char,
  pub issuefile: *const libc::c_char,
  pub maxfd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tsession {
  pub next: *mut tsession,
  pub shell_pid: pid_t,
  pub sockfd_read: libc::c_int,
  pub sockfd_write: libc::c_int,
  pub ptyfd: libc::c_int,
  pub buffered_IAC_for_pty: smallint,
  pub rdidx1: libc::c_int,
  pub wridx1: libc::c_int,
  pub size1: libc::c_int,
  pub rdidx2: libc::c_int,
  pub wridx2: libc::c_int,
  pub size2: libc::c_int,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const BUFSIZE: C2RustUnnamed_2 = 2020;
/* Must match getopt32 string */
pub type C2RustUnnamed_3 = libc::c_uint;
/* -w SEC */
/* -S */
pub const OPT_WAIT: C2RustUnnamed_3 = 256;
/* -F */
pub const OPT_SYSLOG: C2RustUnnamed_3 = 128;
/* -p PORT */
pub const OPT_FOREGROUND: C2RustUnnamed_3 = 64;
/* -i */
pub const OPT_PORT: C2RustUnnamed_3 = 16;
/* -K */
pub const OPT_INETD: C2RustUnnamed_3 = 8;
pub const OPT_WATCHCHILD: C2RustUnnamed_3 = 4;
/* Write some buf1 data to pty, processing IACs.
 * Update wridx1 and size1. Return < 0 on error.
 * Buggy if IAC is present but incomplete: skips them.
 */
unsafe extern "C" fn safe_write_to_pty_decode_iac(mut ts: *mut tsession) -> ssize_t {
  let mut current_block: u64;
  let mut wr: libc::c_uint = 0;
  let mut rc: ssize_t = 0;
  let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
  let mut found: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
  buf = (ts.offset(1) as *mut libc::c_uchar).offset((*ts).wridx1 as isize);
  wr = if BUFSIZE as libc::c_int - (*ts).wridx1 < (*ts).size1 {
    (BUFSIZE as libc::c_int) - (*ts).wridx1
  } else {
    (*ts).size1
  } as libc::c_uint;
  /* wr is at least 1 here */
  if (*ts).buffered_IAC_for_pty != 0 {
    /* Last time we stopped on a "dangling" IAC byte.
     * We removed it from the buffer back then.
     * Now pretend it's still there, and jump to IAC processing.
     */
    (*ts).buffered_IAC_for_pty = 0i32 as smallint; /* Yes, this can point before the buffer. It's ok */
    wr = wr.wrapping_add(1);
    (*ts).size1 += 1;
    buf = buf.offset(-1);
    (*ts).wridx1 -= 1;
    current_block = 9839504995240508490;
  } else {
    found = memchr(buf as *const libc::c_void, 255i32, wr as libc::c_ulong) as *mut libc::c_uchar;
    if found != buf {
      /* There is a "prefix" of non-IAC chars.
       * Write only them, and return.
       */
      if !found.is_null() {
        wr = found.wrapping_offset_from(buf) as libc::c_long as libc::c_uint
      }
      /* We map \r\n ==> \r for pragmatic reasons:
       * many client implementations send \r\n when
       * the user hits the CarriageReturn key.
       * See RFC 1123 3.3.1 Telnet End-of-Line Convention.
       */
      rc = wr as ssize_t;
      found =
        memchr(buf as *const libc::c_void, '\r' as i32, wr as libc::c_ulong) as *mut libc::c_uchar;
      if !found.is_null() {
        rc = found.wrapping_offset_from(buf) as libc::c_long + 1i32 as libc::c_long
      }
      rc = safe_write((*ts).ptyfd, buf as *const libc::c_void, rc as size_t);
      if rc <= 0i32 as libc::c_long {
        return rc;
      }
      if rc < wr as libc::c_long
        && *buf.offset((rc - 1i32 as libc::c_long) as isize) as libc::c_int == '\r' as i32
        && (*buf.offset(rc as isize) as libc::c_int == '\n' as i32
          || *buf.offset(rc as isize) as libc::c_int == '\u{0}' as i32)
      {
        rc += 1
      }
      current_block = 13835600803501426168;
    } else if wr <= 1i32 as libc::c_uint {
      /* buf starts with IAC char. Process that sequence.
       * Example: we get this from our own (bbox) telnet client:
       * read(5, "\377\374\1""\377\373\37""\377\372\37\0\262\0@\377\360""\377\375\1""\377\375\3"):
       * IAC WONT ECHO, IAC WILL NAWS, IAC SB NAWS <cols> <rows> IAC SE, IAC DO SGA
       * Another example (telnet-0.17 from old-netkit):
       * read(4, "\377\375\3""\377\373\30""\377\373\37""\377\373 ""\377\373!""\377\373\"""\377\373'"
       * "\377\375\5""\377\373#""\377\374\1""\377\372\37\0\257\0I\377\360""\377\375\1"):
       * IAC DO SGA, IAC WILL TTYPE, IAC WILL NAWS, IAC WILL TSPEED, IAC WILL LFLOW, IAC WILL LINEMODE, IAC WILL NEW_ENVIRON,
       * IAC DO STATUS, IAC WILL XDISPLOC, IAC WONT ECHO, IAC SB NAWS <cols> <rows> IAC SE, IAC DO ECHO
       */
      /* Only the single IAC byte is in the buffer, eat it
       * and set a flag "process the rest of the sequence
       * next time we are here".
       */
      //bb_error_msg("dangling IAC!");


      (*ts).buffered_IAC_for_pty = 1i32 as smallint;
      rc = 1i32 as ssize_t;
      current_block = 13835600803501426168;
    } else {
      current_block = 9839504995240508490;
    }
  }
  match current_block {
    9839504995240508490 =>
    /* 2-byte commands (240..250 and 255):
     * IAC IAC (255) Literal 255. Supported.
     * IAC SE  (240) End of subnegotiation. Treated as NOP.
     * IAC NOP (241) NOP. Supported.
     * IAC BRK (243) Break. Like serial line break. TODO via tcsendbreak()?
     * IAC AYT (246) Are you there.
     *  These don't look useful:
     * IAC DM  (242) Data mark. What is this?
     * IAC IP  (244) Suspend, interrupt or abort the process. (Ancient cousin of ^C).
     * IAC AO  (245) Abort output. "You can continue running, but do not send me the output".
     * IAC EC  (247) Erase character. The receiver should delete the last received char.
     * IAC EL  (248) Erase line. The receiver should delete everything up tp last newline.
     * IAC GA  (249) Go ahead. For half-duplex lines: "now you talk".
     *  Implemented only as part of NAWS:
     * IAC SB  (250) Subnegotiation of an option follows.
     */
    {
      if *buf.offset(1) as libc::c_int == 255i32 {
        /* Literal 255 (emacs M-DEL) */
        //bb_error_msg("255!");
        rc = safe_write(
          (*ts).ptyfd,
          &mut *buf.offset(1) as *mut libc::c_uchar as *const libc::c_void,
          1i32 as size_t,
        );
        /*
         * If we went through buffered_IAC_for_pty==1 path,
         * bailing out on error like below messes up the buffer.
         * EAGAIN is highly unlikely here, other errors will be
         * repeated on next write, let's just skip error check.
         */
        rc = 2i32 as ssize_t
      } else if *buf.offset(1) as libc::c_int == 246i32 {
        if (*ts).size2 == 0i32 {
          /* if nothing buffered yet... */
          /* Send back evidence that AYT was seen */
          let mut buf2: *mut libc::c_uchar =
            (ts.offset(1) as *mut libc::c_uchar).offset(BUFSIZE as libc::c_int as isize);
          *buf2.offset(0) = 255i32 as libc::c_uchar;
          *buf2.offset(1) = 241i32 as libc::c_uchar;
          (*ts).wridx2 = 0i32;
          (*ts).size2 = 2i32;
          (*ts).rdidx2 = (*ts).size2
        }
        rc = 2i32 as ssize_t
      } else if *buf.offset(1) as libc::c_int >= 240i32 && *buf.offset(1) as libc::c_int <= 249i32 {
        /* NOP (241). Ignore (putty keepalive, etc) */
        /* All other 2-byte commands also treated as NOPs here */
        rc = 2i32 as ssize_t
      } else if wr <= 2i32 as libc::c_uint {
        /* BUG: only 2 bytes of the IAC is in the buffer, we just eat them.
         * This is not a practical problem since >2 byte IACs are seen only
         * in initial negotiation, when buffer is empty
         */
        rc = 2i32 as ssize_t
      } else {
        if *buf.offset(1) as libc::c_int == 250i32 {
          if *buf.offset(2) as libc::c_int == 31i32 {
            /* IAC SB, TELOPT_NAWS, 4-byte, IAC SE */
            let mut ws: winsize = winsize {
              ws_row: 0,
              ws_col: 0,
              ws_xpixel: 0,
              ws_ypixel: 0,
            };
            if wr <= 6i32 as libc::c_uint {
              /* BUG: incomplete, can't process */
              rc = wr as ssize_t
            } else {
              memset(
                &mut ws as *mut winsize as *mut libc::c_void,
                0i32,
                ::std::mem::size_of::<winsize>() as libc::c_ulong,
              ); /* pixel sizes are set to 0 */
              ws.ws_col = ((*buf.offset(3) as libc::c_int) << 8i32 | *buf.offset(4) as libc::c_int)
                as libc::c_ushort;
              ws.ws_row = ((*buf.offset(5) as libc::c_int) << 8i32 | *buf.offset(6) as libc::c_int)
                as libc::c_ushort;
              ioctl(
                (*ts).ptyfd,
                0x5414i32 as libc::c_ulong,
                &mut ws as *mut winsize as *mut libc::c_char,
              );
              rc = 7i32 as ssize_t
            }
            current_block = 13835600803501426168;
          } else {
            current_block = 13281731871476506071;
          }
        /* else: other subnegs not supported yet */
        } else {
          current_block = 13281731871476506071;
        }
        match current_block {
          13835600803501426168 => {}
          _ => {
            /* Assume it is a 3-byte WILL/WONT/DO/DONT 251..254 command and skip it */
            rc = 3i32 as ssize_t
          }
        }
      }
    }
    _ => {}
  }
  /* trailing IAC SE will be eaten separately, as 2-byte NOP */
  (*ts).wridx1 = ((*ts).wridx1 as libc::c_long + rc) as libc::c_int;
  if (*ts).wridx1 >= BUFSIZE as libc::c_int {
    /* actually == BUFSIZE */
    (*ts).wridx1 = 0i32
  }
  (*ts).size1 = ((*ts).size1 as libc::c_long - rc) as libc::c_int;
  /*
   * Hack. We cannot process IACs which wrap around buffer's end.
   * Since properly fixing it requires writing bigger code,
   * we rely instead on this code making it virtually impossible
   * to have wrapped IAC (people don't type at 2k/second).
   * It also allows for bigger reads in common case.
   */
  if (*ts).size1 == 0i32 {
    /* very typical */
    //bb_error_msg("zero size1");
    (*ts).rdidx1 = 0i32;
    (*ts).wridx1 = 0i32;
    return rc;
  }
  wr = (*ts).wridx1 as libc::c_uint;
  if wr != 0i32 as libc::c_uint && wr < (*ts).rdidx1 as libc::c_uint {
    /* Buffer is not wrapped yet.
     * We can easily move it to the beginning.
     */
    //bb_error_msg("moved %d", wr);
    memmove(
      ts.offset(1) as *mut libc::c_uchar as *mut libc::c_void,
      (ts.offset(1) as *mut libc::c_uchar).offset(wr as isize) as *const libc::c_void,
      (*ts).size1 as libc::c_ulong,
    );
    (*ts).rdidx1 = ((*ts).rdidx1 as libc::c_uint).wrapping_sub(wr) as libc::c_int as libc::c_int;
    (*ts).wridx1 = 0i32
  }
  return rc;
}
/*
 * Converting single IAC into double on output
 */
unsafe extern "C" fn safe_write_double_iac(
  mut fd: libc::c_int,
  mut buf: *const libc::c_char,
  mut count: size_t,
) -> size_t {
  let mut IACptr: *const libc::c_char = 0 as *const libc::c_char;
  let mut wr: size_t = 0;
  let mut rc: size_t = 0;
  let mut total: size_t = 0;
  total = 0i32 as size_t;
  loop {
    if count == 0i32 as libc::c_ulong {
      return total;
    }
    if *buf as libc::c_int == 255i32 as libc::c_char as libc::c_int {
      static mut IACIAC: [libc::c_char; 2] = [255i32 as libc::c_char, 255i32 as libc::c_char];
      rc = safe_write(fd, IACIAC.as_ptr() as *const libc::c_void, 2i32 as size_t) as size_t;
      /* BUG: if partial write was only 1 byte long, we end up emitting just one IAC */
      if rc != 2i32 as libc::c_ulong {
        break;
      }
      buf = buf.offset(1);
      total = total.wrapping_add(1);
      count = count.wrapping_sub(1)
    } else {
      /* count != 0, *buf != IAC */
      IACptr = memchr(buf as *const libc::c_void, 255i32, count) as *const libc::c_char;
      wr = count;
      if !IACptr.is_null() {
        wr = IACptr.wrapping_offset_from(buf) as libc::c_long as size_t
      }
      rc = safe_write(fd, buf as *const libc::c_void, wr) as size_t;
      if rc != wr {
        break;
      }
      buf = buf.offset(rc as isize);
      total = (total as libc::c_ulong).wrapping_add(rc) as size_t as size_t;
      count = (count as libc::c_ulong).wrapping_sub(rc) as size_t as size_t
    }
  }
  /* here: rc - result of last short write */
  if (rc as ssize_t) < 0i32 as libc::c_long {
    /* error? */
    if total == 0i32 as libc::c_ulong {
      return rc;
    }
    rc = 0i32 as size_t
  }
  return total.wrapping_add(rc);
}
unsafe extern "C" fn make_new_session(mut sock: libc::c_int) -> *mut tsession {
  let mut login_argv: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
  let mut termbuf: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
  };
  let mut fd: libc::c_int = 0;
  let mut pid: libc::c_int = 0;
  let mut tty_name: [libc::c_char; 16] = [0; 16];
  let mut ts: *mut tsession = xzalloc(
    (::std::mem::size_of::<tsession>() as libc::c_ulong)
      .wrapping_add((BUFSIZE as libc::c_int * 2i32) as libc::c_ulong),
  ) as *mut tsession;
  /*ts->buf1 = (char *)(ts + 1);*/
  /*ts->buf2 = ts->buf1 + BUFSIZE;*/
  /* Got a new connection, set up a tty */
  fd = xgetpty(tty_name.as_mut_ptr());
  if fd > (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).maxfd {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).maxfd = fd
  }
  (*ts).ptyfd = fd;
  ndelay_on(fd);
  close_on_exec_on(fd);
  /* SO_KEEPALIVE by popular demand */
  setsockopt_keepalive(sock);
  (*ts).sockfd_read = sock;
  ndelay_on(sock);
  if sock == 0i32 {
    /* We are called with fd 0 - we are in inetd mode */
    sock += 1; /* so use fd 1 for output */
    ndelay_on(sock);
  }
  (*ts).sockfd_write = sock;
  if sock > (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).maxfd {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).maxfd = sock
  }
  /* Make the telnet client understand we will echo characters so it
   * should not do it locally. We don't tell the client to run linemode,
   * because we want to handle line editing and tab completion and other
   * stuff that requires char-by-char support. */
  static mut iacs_to_send: [libc::c_char; 12] = [
    255i32 as libc::c_char,
    253i32 as libc::c_char,
    1i32 as libc::c_char,
    255i32 as libc::c_char,
    253i32 as libc::c_char,
    31i32 as libc::c_char,
    255i32 as libc::c_char,
    251i32 as libc::c_char,
    1i32 as libc::c_char,
    255i32 as libc::c_char,
    251i32 as libc::c_char,
    3i32 as libc::c_char,
  ];
  /*ts->rdidx2 = 0; - xzalloc did it */
  /*ts->size2 = 0;*/
  safe_write(
    sock,
    iacs_to_send.as_ptr() as *const libc::c_void,
    ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
  );
  fflush_all();
  /* This confuses safe_write_double_iac(), it will try to duplicate
   * each IAC... */
  //memcpy(TS_BUF2(ts), iacs_to_send, sizeof(iacs_to_send));
  //ts->rdidx2 = sizeof(iacs_to_send);
  //ts->size2 = sizeof(iacs_to_send);
  /* So just stuff it into TCP stream! (no error check...) */
  pid = vfork(); /* NOMMU-friendly */
  if pid < 0i32 {
    free(ts as *mut libc::c_void);
    close(fd);
    /* sock will be closed by caller */
    bb_simple_perror_msg(b"vfork\x00" as *const u8 as *const libc::c_char);
    return 0 as *mut tsession;
  }
  if pid > 0i32 {
    /* Parent */
    (*ts).shell_pid = pid;
    return ts;
  }
  /* Child */
  /* Careful - we are after vfork! */
  /* Restore default signal handling ASAP */
  bb_signals((1i32 << 17i32) + (1i32 << 13i32), None);
  pid = getpid();
  let mut lsa: *mut len_and_sockaddr = get_peer_lsa(sock);
  let mut hostname: *mut libc::c_char = 0 as *mut libc::c_char;
  if !lsa.is_null() {
    hostname = xmalloc_sockaddr2dotted(&mut (*lsa).u.sa);
    free(lsa as *mut libc::c_void);
  }
  write_new_utmp(
    pid,
    6i32,
    tty_name.as_mut_ptr(),
    b"LOGIN\x00" as *const u8 as *const libc::c_char,
    hostname,
  );
  free(hostname as *mut libc::c_void);
  /* Make new session and process group */
  setsid();
  /* Open the child's side of the tty */
  /* NB: setsid() disconnects from any previous ctty's. Therefore
   * we must open child's side of the tty AFTER setsid! */
  close(0i32); /* becomes our ctty */
  xopen(tty_name.as_mut_ptr(), 0o2i32); /* switch this tty's process group to us */
  xdup2(0i32, 1i32);
  xdup2(0i32, 2i32);
  tcsetpgrp(0i32, pid);
  /* The pseudo-terminal allocated to the client is configured to operate
   * in cooked mode, and with XTABS CRMOD enabled (see tty(4)) */
  tcgetattr(0i32, &mut termbuf); /* if we use readline we dont want this */
  termbuf.c_lflag |= 0o10i32 as libc::c_uint;
  termbuf.c_oflag |= (0o4i32 | 0o14000i32) as libc::c_uint;
  termbuf.c_iflag |= 0o400i32 as libc::c_uint;
  termbuf.c_iflag &= !0o10000i32 as libc::c_uint;
  /*termbuf.c_lflag &= ~ICANON;*/
  tcsetattr_stdin_TCSANOW(&mut termbuf);
  /* Uses FILE-based I/O to stdout, but does fflush_all(),
   * so should be safe with vfork.
   * I fear, though, that some users will have ridiculously big
   * issue files, and they may block writing to fd 1,
   * (parent is supposed to read it, but parent waits
   * for vforked child to exec!) */
  print_login_issue(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).issuefile,
    tty_name.as_mut_ptr(),
  );
  /* Exec shell / login / whatever */
  login_argv[0] = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).loginpath;
  login_argv[1] = 0 as *const libc::c_char;
  /* exec busybox applet (if PREFER_APPLETS=y), if that fails,
   * exec external program.
   * NB: sock is either 0 or has CLOEXEC set on it.
   * fd has CLOEXEC set on it too. These two fds will be closed here.
   */
  execvp(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).loginpath,
    login_argv.as_mut_ptr() as *mut *mut libc::c_char as *const *mut libc::c_char,
  );
  _exit(1i32);
  /* _exit is safer with vfork, and we shouldn't send message
   * to remote clients anyway */
  /*bb_perror_msg_and_die("execv %s", G.loginpath);*/
}
unsafe extern "C" fn free_session(mut ts: *mut tsession) {
  let mut t: *mut tsession = 0 as *mut tsession;
  if option_mask32 & OPT_INETD as libc::c_int as libc::c_uint != 0 {
    exit(0i32);
  }
  /* Unlink this telnet session from the session list */
  t = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).sessions;
  if t == ts {
    let ref mut fresh0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).sessions;
    *fresh0 = (*ts).next
  } else {
    while (*t).next != ts {
      t = (*t).next
    }
    (*t).next = (*ts).next
  }
  close((*ts).ptyfd);
  close((*ts).sockfd_read);
  /* We do not need to close(ts->sockfd_write), it's the same
   * as sockfd_read unless we are in inetd mode. But in inetd mode
   * we do not reach this */
  free(ts as *mut libc::c_void);
  /* Scan all sessions and find new maxfd */
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).maxfd = 0i32;
  ts = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).sessions;
  while !ts.is_null() {
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).maxfd < (*ts).ptyfd {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).maxfd = (*ts).ptyfd
    }
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).maxfd < (*ts).sockfd_read {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).maxfd = (*ts).sockfd_read
    }
    ts = (*ts).next
  }
}
/* !FEATURE_TELNETD_STANDALONE */
unsafe extern "C" fn handle_sigchld(mut _sig: libc::c_int) {
  let mut pid: pid_t = 0;
  let mut ts: *mut tsession = 0 as *mut tsession;
  let mut save_errno: libc::c_int = *bb_errno;
  loop
  /* Looping: more than one child may have exited */
  {
    pid = wait_any_nohang(0 as *mut libc::c_int); /* for compiler */
    if pid <= 0i32 {
      break;
    }
    ts = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).sessions;
    while !ts.is_null() {
      if (*ts).shell_pid == pid {
        (*ts).shell_pid = -1i32;
        update_utmp_DEAD_PROCESS(pid);
        break;
      } else {
        ts = (*ts).next
      }
    }
  }
  *bb_errno = save_errno;
}
#[no_mangle]
pub unsafe extern "C" fn telnetd_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut rdfdset: fd_set = fd_set { fds_bits: [0; 16] };
  let mut wrfdset: fd_set = fd_set { fds_bits: [0; 16] };
  let mut opt: libc::c_uint = 0;
  let mut count: libc::c_int = 0;
  let mut ts: *mut tsession = 0 as *mut tsession;
  let mut master_fd: libc::c_int = 0;
  master_fd = master_fd;
  let mut sec_linger: libc::c_int = 0;
  sec_linger = sec_linger;
  let mut opt_bindaddr: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut opt_portnbr: *mut libc::c_char = 0 as *mut libc::c_char;
  let ref mut fresh1 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).loginpath;
  *fresh1 = b"/bin/login\x00" as *const u8 as *const libc::c_char;
  let ref mut fresh2 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).issuefile;
  *fresh2 = b"/etc/issue.net\x00" as *const u8 as *const libc::c_char;
  /* Even if !STANDALONE, we accept (and ignore) -i, thus people
   * don't need to guess whether it's ok to pass -i to us */
  opt = getopt32(
    argv,
    b"^f:l:Kip:b:FSw:+\x00wF:i--w:w--i\x00" as *const u8 as *const libc::c_char,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).issuefile as *mut *const libc::c_char,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).loginpath as *mut *const libc::c_char,
    &mut opt_portnbr as *mut *mut libc::c_char,
    &mut opt_bindaddr as *mut *mut libc::c_char,
    &mut sec_linger as *mut libc::c_int,
  );
  if opt & OPT_INETD as libc::c_int as libc::c_uint == 0 {
    /*&& !re_execed*/
    /* inform that we start in standalone mode?
     * May be useful when people forget to give -i */
    /*bb_error_msg("listening for connections");*/
    if opt & OPT_FOREGROUND as libc::c_int as libc::c_uint == 0 {
      /* DAEMON_CHDIR_ROOT was giving inconsistent
       * behavior with/without -F, -i */
      bb_daemonize_or_rexec(0i32);
      /*was DAEMON_CHDIR_ROOT*/
    }
  }
  /* Redirect log to syslog early, if needed */
  if opt & OPT_INETD as libc::c_int as libc::c_uint != 0
    || opt & OPT_SYSLOG as libc::c_int as libc::c_uint != 0
    || opt & OPT_FOREGROUND as libc::c_int as libc::c_uint == 0
  {
    openlog(applet_name, 0x1i32, 3i32 << 3i32);
    logmode = LOGMODE_SYSLOG as libc::c_int as smallint
  }
  if opt & OPT_INETD as libc::c_int as libc::c_uint != 0 {
    let ref mut fresh3 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).sessions;
    *fresh3 = make_new_session(0i32);
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .sessions
      .is_null()
    {
      /* make_new_session printed error message */
      /* pty opening or vfork problem, exit */
      return 1i32;
    }
  } else {
    master_fd = 0i32;
    if opt & OPT_WAIT as libc::c_int as libc::c_uint == 0 {
      let mut portnbr: libc::c_uint = 23i32 as libc::c_uint;
      if opt & OPT_PORT as libc::c_int as libc::c_uint != 0 {
        portnbr = xatou16(opt_portnbr) as libc::c_uint
      }
      master_fd = create_and_bind_stream_or_die(opt_bindaddr, portnbr as libc::c_int);
      xlisten(master_fd, 1i32);
    }
    close_on_exec_on(master_fd);
  }
  /* We don't want to die if just one session is broken */
  signal(
    13i32,
    ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
  );
  if opt & OPT_WATCHCHILD as libc::c_int as libc::c_uint != 0 {
    signal(
      17i32,
      Some(handle_sigchld as unsafe extern "C" fn(_: libc::c_int) -> ()),
    );
  } else {
    /* prevent dead children from becoming zombies */
    signal(
      17i32,
      ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
    );
  }
  loop
  /*
     This is how the buffers are used. The arrows indicate data flow.

     +-------+     wridx1++     +------+     rdidx1++     +----------+
     |       | <--------------  | buf1 | <--------------  |          |
     |       |     size1--      +------+     size1++      |          |
     |  pty  |                                            |  socket  |
     |       |     rdidx2++     +------+     wridx2++     |          |
     |       |  --------------> | buf2 |  --------------> |          |
     +-------+     size2++      +------+     size2--      +----------+

     size1: "how many bytes are buffered for pty between rdidx1 and wridx1?"
     size2: "how many bytes are buffered for socket between rdidx2 and wridx2?"

     Each session has got two buffers. Buffers are circular. If sizeN == 0,
     buffer is empty. If sizeN == BUFSIZE, buffer is full. In both these cases
     rdidxN == wridxN.
  */
  {
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh4 = &mut __d0;
    let fresh5;
    let fresh6 = &mut __d1;
    let fresh7;
    let fresh8 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh9 = &mut *rdfdset.fds_bits.as_mut_ptr().offset(0) as *mut __fd_mask;
    asm!("cld; rep; stosq" : "={cx}" (fresh5), "={di}" (fresh7) : "{ax}"
             (0i32), "0" (c2rust_asm_casts::AsmCast::cast_in(fresh4, fresh8)),
             "1" (c2rust_asm_casts::AsmCast::cast_in(fresh6, fresh9)) :
             "memory" : "volatile");
    c2rust_asm_casts::AsmCast::cast_out(fresh4, fresh8, fresh5);
    c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh9, fresh7);
    let mut __d0_0: libc::c_int = 0;
    let mut __d1_0: libc::c_int = 0;
    let fresh10 = &mut __d0_0;
    let fresh11;
    let fresh12 = &mut __d1_0;
    let fresh13;
    let fresh14 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh15 = &mut *wrfdset.fds_bits.as_mut_ptr().offset(0) as *mut __fd_mask;
    asm!("cld; rep; stosq" : "={cx}" (fresh11), "={di}" (fresh13) : "{ax}"
             (0i32), "0"
             (c2rust_asm_casts::AsmCast::cast_in(fresh10, fresh14)), "1"
             (c2rust_asm_casts::AsmCast::cast_in(fresh12, fresh15)) : "memory"
             : "volatile");
    c2rust_asm_casts::AsmCast::cast_out(fresh10, fresh14, fresh11);
    c2rust_asm_casts::AsmCast::cast_out(fresh12, fresh15, fresh13);
    /* Select on the master socket, all telnet sockets and their
     * ptys if there is room in their session buffers.
     * NB: scalability problem: we recalculate entire bitmap
     * before each select. Can be a problem with 500+ connections. */
    ts = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).sessions; /* in case we free ts */
    while !ts.is_null() {
      let mut next: *mut tsession = (*ts).next;
      if (*ts).shell_pid == -1i32 {
        /* Child died and we detected that */
        free_session(ts);
      } else {
        if (*ts).size1 > 0i32 {
          /* can write to pty */
          wrfdset.fds_bits[((*ts).ptyfd
            / (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize] |= (1u64
            << (*ts).ptyfd
              % (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as __fd_mask
        }
        if (*ts).size1 < BUFSIZE as libc::c_int {
          /* can read from socket */
          rdfdset.fds_bits[((*ts).sockfd_read
            / (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize] |= (1u64
            << (*ts).sockfd_read
              % (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as __fd_mask
        }
        if (*ts).size2 > 0i32 {
          /* can write to socket */
          wrfdset.fds_bits[((*ts).sockfd_write
            / (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize] |= (1u64
            << (*ts).sockfd_write
              % (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as __fd_mask
        }
        if (*ts).size2 < BUFSIZE as libc::c_int {
          /* can read from pty */
          rdfdset.fds_bits[((*ts).ptyfd
            / (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize] |= (1u64
            << (*ts).ptyfd
              % (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as __fd_mask
        }
      }
      ts = next
    }
    if opt & OPT_INETD as libc::c_int as libc::c_uint == 0 {
      rdfdset.fds_bits[(master_fd
        / (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize] |= (1u64
        << master_fd % (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as __fd_mask;
      /* This is needed because free_session() does not
       * take master_fd into account when it finds new
       * maxfd among remaining fd's */
      if master_fd > (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).maxfd {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).maxfd = master_fd
      }
    }
    let mut tv_ptr: *mut timeval = 0 as *mut timeval;
    let mut tv: timeval = timeval {
      tv_sec: 0,
      tv_usec: 0,
    };
    if opt & OPT_WAIT as libc::c_int as libc::c_uint != 0
      && (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .sessions
        .is_null()
    {
      tv.tv_sec = sec_linger as __time_t;
      tv.tv_usec = 0i32 as __suseconds_t;
      tv_ptr = &mut tv
    }
    count = select(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).maxfd + 1i32,
      &mut rdfdset,
      &mut wrfdset,
      0 as *mut fd_set,
      tv_ptr,
    );
    if count == 0i32 {
      /* "telnetd -w SEC" timed out */
      return 0i32;
    } /* EINTR or ENOMEM */
    if count < 0i32 {
      continue;
    }
    /* Check for and accept new sessions */
    if opt & OPT_INETD as libc::c_int as libc::c_uint == 0
      && rdfdset.fds_bits[(master_fd
        / (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        & (1u64
          << master_fd
            % (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
          as __fd_mask
        != 0i32 as libc::c_long
    {
      let mut fd: libc::c_int = 0;
      let mut new_ts: *mut tsession = 0 as *mut tsession;
      fd = accept(
        master_fd,
        __SOCKADDR_ARG {
          __sockaddr__: 0 as *mut libc::c_void as *mut sockaddr,
        },
        0 as *mut socklen_t,
      );
      if fd < 0i32 {
        continue;
      }
      close_on_exec_on(fd);
      /* Create a new session and link it into active list */
      new_ts = make_new_session(fd);
      if !new_ts.is_null() {
        (*new_ts).next = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).sessions;
        let ref mut fresh16 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).sessions;
        *fresh16 = new_ts
      } else {
        close(fd);
      }
    }
    /* Then check for data tunneling */
    ts = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).sessions;
    while !ts.is_null() {
      /* For all sessions... */
      let mut next_0: *mut tsession = (*ts).next; /* in case we free ts */
      if wrfdset.fds_bits[((*ts).ptyfd
        / (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        & (1u64
          << (*ts).ptyfd
            % (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
          as __fd_mask
        != 0i32 as libc::c_long
      {
        /* Write to pty from buffer 1 */
        count = safe_write_to_pty_decode_iac(ts) as libc::c_int;
        if count < 0i32 {
          if *bb_errno == 11i32 {
            current_block = 454644968774100691;
          } else {
            current_block = 13644292938706749517;
          }
        } else {
          current_block = 454644968774100691;
        }
      } else {
        current_block = 454644968774100691;
      }
      match current_block {
        454644968774100691 => {
          if wrfdset.fds_bits[((*ts).sockfd_write
            / (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            & (1u64
              << (*ts).sockfd_write
                % (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
              as __fd_mask
            != 0i32 as libc::c_long
          {
            /* Write to socket from buffer 2 */
            count = if BUFSIZE as libc::c_int - (*ts).wridx2 < (*ts).size2 {
              (BUFSIZE as libc::c_int) - (*ts).wridx2
            } else {
              (*ts).size2
            };
            count = safe_write_double_iac(
              (*ts).sockfd_write,
              (ts.offset(1) as *mut libc::c_uchar)
                .offset(BUFSIZE as libc::c_int as isize)
                .offset((*ts).wridx2 as isize) as *mut libc::c_void
                as *const libc::c_char,
              count as size_t,
            ) as libc::c_int;
            if count < 0i32 {
              if *bb_errno == 11i32 {
                current_block = 14977138807965774745;
              } else {
                current_block = 13644292938706749517;
              }
            } else {
              (*ts).wridx2 += count;
              if (*ts).wridx2 >= BUFSIZE as libc::c_int {
                /* actually == BUFSIZE */
                (*ts).wridx2 = 0i32
              }
              (*ts).size2 -= count;
              if (*ts).size2 == 0i32 {
                (*ts).rdidx2 = 0i32;
                (*ts).wridx2 = 0i32
              }
              current_block = 14977138807965774745;
            }
          } else {
            current_block = 14977138807965774745;
          }
          match current_block {
            13644292938706749517 => {}
            _ => {
              if rdfdset.fds_bits[((*ts).sockfd_read
                / (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                as usize]
                & (1u64
                  << (*ts).sockfd_read
                    % (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                  as __fd_mask
                != 0i32 as libc::c_long
              {
                /* Read from socket to buffer 1 */
                count =
                  if BUFSIZE as libc::c_int - (*ts).rdidx1 < BUFSIZE as libc::c_int - (*ts).size1 {
                    (BUFSIZE as libc::c_int) - (*ts).rdidx1
                  } else {
                    (BUFSIZE as libc::c_int) - (*ts).size1
                  };
                count = safe_read(
                  (*ts).sockfd_read,
                  (ts.offset(1) as *mut libc::c_uchar).offset((*ts).rdidx1 as isize)
                    as *mut libc::c_void,
                  count as size_t,
                ) as libc::c_int;
                if count <= 0i32 {
                  if count < 0i32 && *bb_errno == 11i32 {
                    current_block = 1387731475740922237;
                  } else {
                    current_block = 13644292938706749517;
                  }
                } else {
                  /* Ignore trailing NUL if it is there */
                  if *(ts.offset(1) as *mut libc::c_uchar)
                    .offset(((*ts).rdidx1 + count - 1i32) as isize)
                    == 0
                  {
                    count -= 1
                  }
                  (*ts).size1 += count;
                  (*ts).rdidx1 += count;
                  if (*ts).rdidx1 >= BUFSIZE as libc::c_int {
                    /* actually == BUFSIZE */
                    (*ts).rdidx1 = 0i32
                  }
                  current_block = 1387731475740922237;
                }
              } else {
                current_block = 1387731475740922237;
              }
              match current_block {
                13644292938706749517 => {}
                _ => {
                  if rdfdset.fds_bits[((*ts).ptyfd
                    / (8i32 * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                    as usize]
                    & (1u64
                      << (*ts).ptyfd
                        % (8i32
                          * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                      as __fd_mask
                    != 0i32 as libc::c_long
                  {
                    /* Read from pty to buffer 2 */
                    let mut eio: libc::c_int = 0i32;
                    loop {
                      count = if BUFSIZE as libc::c_int - (*ts).rdidx2
                        < BUFSIZE as libc::c_int - (*ts).size2
                      {
                        (BUFSIZE as libc::c_int) - (*ts).rdidx2
                      } else {
                        (BUFSIZE as libc::c_int) - (*ts).size2
                      };
                      count = safe_read(
                        (*ts).ptyfd,
                        (ts.offset(1) as *mut libc::c_uchar)
                          .offset(BUFSIZE as libc::c_int as isize)
                          .offset((*ts).rdidx2 as isize)
                          as *mut libc::c_void,
                        count as size_t,
                      ) as libc::c_int;
                      if count <= 0i32 {
                        if !(count < 0i32) {
                          current_block = 13644292938706749517;
                          break;
                        }
                        if *bb_errno == 11i32 {
                          current_block = 16043883375386484726;
                          break;
                        }
                        /* login process might call vhangup(),
                         * which causes intermittent EIOs on read above
                         * (observed on kernel 4.12.0). Try up to 10 ms.
                         */
                        if !(*bb_errno == 5i32 && eio < 10i32) {
                          current_block = 13644292938706749517;
                          break;
                        }
                        eio += 1;
                        //bb_error_msg("EIO pty %u", eio);
                        usleep(1000i32 as __useconds_t);
                      } else {
                        (*ts).size2 += count;
                        (*ts).rdidx2 += count;
                        if (*ts).rdidx2 >= BUFSIZE as libc::c_int {
                          /* actually == BUFSIZE */
                          (*ts).rdidx2 = 0i32
                        }
                        current_block = 16043883375386484726;
                        break;
                      }
                    }
                  } else {
                    current_block = 16043883375386484726;
                  }
                  match current_block {
                    13644292938706749517 => {}
                    _ => {
                      ts = next_0;
                      continue;
                    }
                  }
                }
              }
            }
          }
        }
        _ => {}
      }
      if (*ts).shell_pid > 0i32 {
        update_utmp_DEAD_PROCESS((*ts).shell_pid);
      }
      free_session(ts);
      ts = next_0
    }
  }
}
