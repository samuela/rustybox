
use libc;
use libc::getenv;
use libc::kill;
use libc::printf;
use libc::sleep;
extern "C" {
  #[no_mangle]
  fn exit(_: libc::c_int) -> !;

  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
  #[no_mangle]
  fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
  #[no_mangle]
  fn tcsetattr(
    __fd: libc::c_int,
    __optional_actions: libc::c_int,
    __termios_p: *const termios,
  ) -> libc::c_int;
  #[no_mangle]
  fn cfmakeraw(__termios_p: *mut termios);
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  /* Standard handler which just records signo */
  #[no_mangle]
  static mut bb_got_signal: smallint;
  #[no_mangle]
  fn record_signo(signo: libc::c_int);
  /* On Linux this never fails. */
  #[no_mangle]
  fn setsockopt_keepalive(fd: libc::c_int) -> libc::c_int;
  /* NB: returns port in host byte order */
  #[no_mangle]
  fn bb_lookup_port(
    port: *const libc::c_char,
    protocol: *const libc::c_char,
    default_port: libc::c_uint,
  ) -> libc::c_uint;
  /* Create client TCP socket connected to peer:port. Peer cannot be NULL.
   * Peer can be numeric IP ("N.N.N.N"), numeric IPv6 address or hostname,
   * and can have ":PORT" suffix (for IPv6 use "[X:X:...:X]:PORT").
   * If there is no suffix, port argument is used */
  #[no_mangle]
  fn create_and_connect_stream_or_die(peer: *const libc::c_char, port: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  // NB: will return short write on error, not -1,
  // if some data was written before error occurred
  #[no_mangle]
  fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn full_write1_str(str: *const libc::c_char) -> ssize_t;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_show_usage() -> !;
  /* NB: typically you want to pass fd 0, not 1. Think 'applet | grep something' */
  #[no_mangle]
  fn get_terminal_width_height(
    fd: libc::c_int,
    width: *mut libc::c_uint,
    height: *mut libc::c_uint,
  ) -> libc::c_int;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

use crate::librb::signal::__sighandler_t;
use crate::librb::size_t;
use crate::librb::smallint;
use libc::ssize_t;
pub type nfds_t = libc::c_ulong;
use libc::pollfd;
use libc::termios;
//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub iaclen: libc::c_int,
  pub telstate: byte,
  pub telwish: byte,
  pub charmode: byte,
  pub telflags: byte,
  pub do_termios: byte,
  pub ttype: *mut libc::c_char,
  pub autologin: *const libc::c_char,
  pub win_width: libc::c_uint,
  pub win_height: libc::c_uint,
  pub buf: [libc::c_char; 128],
  pub iacbuf: [libc::c_char; 128],
  pub termios_def: termios,
  pub termios_raw: termios,
}
pub type byte = libc::c_uchar;
pub type C2RustUnnamed = libc::c_uint;
pub const TS_CR: C2RustUnnamed = 6;
pub const TS_SUB2: C2RustUnnamed = 5;
pub const TS_SUB1: C2RustUnnamed = 4;
pub const TS_OPT: C2RustUnnamed = 3;
pub const TS_IAC: C2RustUnnamed = 2;
pub const TS_COPY: C2RustUnnamed = 1;
pub const TS_NORMAL: C2RustUnnamed = 0;
pub const UF_SGA: C2RustUnnamed = 2;
pub const UF_ECHO: C2RustUnnamed = 1;
pub const CHM_OFF: C2RustUnnamed = 2;
pub const CHM_ON: C2RustUnnamed = 1;
pub const CHM_TRY: C2RustUnnamed = 0;
pub const IACBUFSIZE: C2RustUnnamed = 128;
pub const DATABUFSIZE: C2RustUnnamed = 128;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const netfd: C2RustUnnamed_0 = 3;
unsafe extern "C" fn iac_flush() {
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).iaclen != 0i32 {
    full_write(
      netfd as libc::c_int,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .iacbuf
        .as_mut_ptr() as *const libc::c_void,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).iaclen as size_t,
    );
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).iaclen = 0i32
  };
}
unsafe extern "C" fn doexit(mut ev: libc::c_int) -> ! {
  cookmode();
  exit(ev);
}
unsafe extern "C" fn con_escape() {
  let mut current_block: u64;
  let mut b: libc::c_char = 0;
  if bb_got_signal != 0 {
    /* came from line mode... go raw */
    rawmode(); /* IAC -> IAC IAC */
  }
  full_write1_str(b"\r\nConsole escape. Commands are:\r\n\n l\tgo to line mode\r\n c\tgo to character mode\r\n z\tsuspend telnet\r\n e\texit telnet\r\n\x00"
                        as *const u8 as *const libc::c_char);
  if read(
    0i32,
    &mut b as *mut libc::c_char as *mut libc::c_void,
    1i32 as size_t,
  ) <= 0
  {
    doexit(1i32);
  }
  match b as libc::c_int {
    108 => {
      if bb_got_signal == 0 {
        do_linemode();
        current_block = 12937026566716488366;
      } else {
        current_block = 5143058163439228106;
      }
    }
    99 => {
      if bb_got_signal != 0 {
        will_charmode();
        current_block = 12937026566716488366;
      } else {
        current_block = 5143058163439228106;
      }
    }
    122 => {
      cookmode();
      kill(0i32, 20i32);
      rawmode();
      current_block = 5143058163439228106;
    }
    101 => {
      doexit(0i32);
    }
    _ => {
      current_block = 5143058163439228106;
    }
  }
  match current_block {
    5143058163439228106 => {
      full_write1_str(b"continuing...\r\n\x00" as *const u8 as *const libc::c_char);
      if bb_got_signal != 0 {
        cookmode();
      }
    }
    _ => {}
  }
  bb_got_signal = 0i32 as smallint;
}
unsafe extern "C" fn handle_net_output(mut len: libc::c_int) {
  let mut outbuf: [byte; 256] = [0; 256];
  let mut dst: *mut byte = outbuf.as_mut_ptr();
  let mut src: *mut byte = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .buf
    .as_mut_ptr() as *mut byte;
  let mut end: *mut byte = src.offset(len as isize);
  while src < end {
    let fresh0 = src;
    src = src.offset(1);
    let mut c: byte = *fresh0;
    if c as libc::c_int == 0x1di32 {
      con_escape();
      return;
    }
    *dst = c;
    if c as libc::c_int == 255i32 {
      dst = dst.offset(1);
      *dst = c
    } else if c as libc::c_int == '\r' as i32 || c as libc::c_int == '\n' as i32 {
      /* Enter key sends '\r' in raw mode and '\n' in cooked one.
       *
       * See RFC 1123 3.3.1 Telnet End-of-Line Convention.
       * Using CR LF instead of other allowed possibilities
       * like CR NUL - easier to talk to HTTP/SMTP servers.
       */
      *dst = '\r' as i32 as byte; /* Enter -> CR LF */
      dst = dst.offset(1);
      *dst = '\n' as i32 as byte
    }
    dst = dst.offset(1)
  }
  if dst.wrapping_offset_from(outbuf.as_mut_ptr()) as libc::c_long != 0 {
    full_write(
      netfd as libc::c_int,
      outbuf.as_mut_ptr() as *const libc::c_void,
      dst.wrapping_offset_from(outbuf.as_mut_ptr()) as libc::c_long as size_t,
    );
  };
}
unsafe extern "C" fn handle_net_input(mut len: libc::c_int) {
  let mut c: byte = 0;
  let mut i: libc::c_int = 0;
  let mut cstart: libc::c_int = 0i32;
  i = 0i32;
  //bb_error_msg("[%u,'%.*s']", G.telstate, len, G.buf);
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telstate as libc::c_int
    == TS_NORMAL as libc::c_int
  {
    let mut current_block_9: u64;
    loop
    /* most typical state */
    {
      if !(i < len) {
        current_block_9 = 11650488183268122163;
        break;
      }
      c = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).buf[i as usize] as byte;
      i += 1;
      if c as libc::c_int == 255i32 {
        current_block_9 = 9959655061420637642;
        break;
      }
      if !(c as libc::c_int != '\r' as i32) {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telstate = TS_CR as libc::c_int as byte;
        cstart = i;
        current_block_9 = 17407779659766490442;
        break;
      }
    }
    match current_block_9 {
      9959655061420637642 =>
      /* unlikely */
      {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telstate =
          TS_IAC as libc::c_int as byte;
        cstart = i - 1i32
      }
      11650488183268122163 => {
        full_write(
          1i32,
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .buf
            .as_mut_ptr() as *const libc::c_void,
          len as size_t,
        );
        return;
      }
      _ => {}
    }
  }
  while i < len {
    c = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).buf[i as usize] as byte;
    let mut current_block_29: u64;
    match (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telstate as libc::c_int {
      6 => {
        /* Prev char was CR. If cur one is NUL, ignore it.
         * See RFC 1123 section 3.3.1 for discussion of telnet EOL handling.
         */
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telstate =
          TS_COPY as libc::c_int as byte;
        if c as libc::c_int == '\u{0}' as i32 {
          current_block_29 = 1356832168064818221;
        } else {
          current_block_29 = 17777247491025708018;
        }
      }
      1 => {
        current_block_29 = 17777247491025708018;
      }
      2 => {
        /* Prev char was IAC */
        match c as libc::c_int {
          255 => {
            /* IAC IAC -> one IAC */
            let fresh2 = cstart;
            cstart = cstart + 1;
            (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).buf[fresh2 as usize] =
              c as libc::c_char;
            (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telstate =
              TS_COPY as libc::c_int as byte
          }
          250 => {
            (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telstate =
              TS_SUB1 as libc::c_int as byte
          }
          253 | 254 | 251 | 252 => {
            (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telwish = c;
            (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telstate =
              TS_OPT as libc::c_int as byte
          }
          _ => {
            /* DATA MARK must be added later */
            (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telstate =
              TS_COPY as libc::c_int as byte
          }
        }
        current_block_29 = 1356832168064818221;
      }
      3 => {
        /* Prev chars were IAC WILL/WONT/DO/DONT */
        telopt(c);
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telstate =
          TS_COPY as libc::c_int as byte;
        current_block_29 = 1356832168064818221;
      }
      4 | 5 => {
        /* Subnegotiation */
        /* Subnegotiation */
        subneg(c);
        current_block_29 = 1356832168064818221;
      }
      _ => {
        current_block_29 = 1356832168064818221;
      }
    }
    match current_block_29 {
      17777247491025708018 =>
      /* else: fall through - need to handle CR IAC ... properly */
      /* Prev char was ordinary */
      /* Similar to NORMAL, but in TS_COPY we need to copy bytes */
      {
        if c as libc::c_int == 255i32 {
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telstate =
            TS_IAC as libc::c_int as byte
        } else {
          let fresh1 = cstart;
          cstart = cstart + 1;
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).buf[fresh1 as usize] =
            c as libc::c_char;
          if c as libc::c_int == '\r' as i32 {
            (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telstate =
              TS_CR as libc::c_int as byte
          }
        }
      }
      _ => {}
    }
    i += 1
  }
  /* We had some IACs, or CR */
  iac_flush();
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telstate as libc::c_int
    == TS_COPY as libc::c_int
  {
    /* we aren't in the middle of IAC */
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telstate = TS_NORMAL as libc::c_int as byte
  } /* "... & 0xff" is implicit */
  if cstart != 0i32 {
    full_write(
      1i32,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .buf
        .as_mut_ptr() as *const libc::c_void,
      cstart as size_t,
    ); /* "... & 0xff" is implicit */
  };
}
unsafe extern "C" fn put_iac(mut c: libc::c_int) {
  let mut iaclen: libc::c_int = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).iaclen;
  if iaclen >= IACBUFSIZE as libc::c_int {
    iac_flush();
    iaclen = 0i32
  }
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).iacbuf[iaclen as usize] = c as libc::c_char;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).iaclen = iaclen + 1i32;
}
unsafe extern "C" fn put_iac2_msb_lsb(mut x_y: libc::c_uint) {
  put_iac((x_y >> 8i32) as libc::c_int);
  put_iac(x_y as libc::c_int);
  /* "... & 0xff" is implicit */
}
unsafe extern "C" fn put_iac4_msb_lsb(mut x_y_z_t: libc::c_uint) {
  put_iac2_msb_lsb(x_y_z_t >> 16i32);
  put_iac2_msb_lsb(x_y_z_t);
  /* "... & 0xffff" is implicit */
}
unsafe extern "C" fn put_iac3_IAC_x_y_merged(mut wwdd_and_c: libc::c_uint) {
  put_iac(255i32); /* "USER" */
  put_iac2_msb_lsb(wwdd_and_c);
}
unsafe extern "C" fn put_iac_subopt(mut c: byte, mut str: *mut libc::c_char) {
  put_iac4_msb_lsb(
    ((255i32 << 24i32) + (250i32 << 16i32) + ((c as libc::c_int) << 8i32) + 0i32) as libc::c_uint,
  );
  while *str != 0 {
    let fresh3 = str;
    str = str.offset(1);
    put_iac(*fresh3 as libc::c_int);
  }
  put_iac2_msb_lsb(((255i32 << 8i32) + 240i32) as libc::c_uint);
}
unsafe extern "C" fn put_iac_subopt_autologin() {
  let mut p: *const libc::c_char = 0 as *const libc::c_char;
  put_iac4_msb_lsb(
    ((255i32 << 24i32) + (250i32 << 16i32) + (39i32 << 8i32) + 0i32) as libc::c_uint,
  );
  put_iac4_msb_lsb(
    ((0i32 << 24i32) + (('U' as i32) << 16i32) + (('S' as i32) << 8i32) + 'E' as i32)
      as libc::c_uint,
  );
  put_iac2_msb_lsb(((('R' as i32) << 8i32) + 1i32) as libc::c_uint);
  p = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).autologin;
  while *p != 0 {
    let fresh4 = p;
    p = p.offset(1);
    put_iac(*fresh4 as libc::c_int);
  }
  put_iac2_msb_lsb(((255i32 << 8i32) + 240i32) as libc::c_uint);
}
unsafe extern "C" fn put_iac_naws(mut c: byte, mut x: libc::c_int, mut y: libc::c_int) {
  put_iac3_IAC_x_y_merged(((250i32 << 8i32) + c as libc::c_int) as libc::c_uint);
  put_iac4_msb_lsb(((x << 16i32) + y) as libc::c_uint);
  put_iac2_msb_lsb(((255i32 << 8i32) + 240i32) as libc::c_uint);
}
unsafe extern "C" fn setConMode() {
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telflags as libc::c_int
    & UF_ECHO as libc::c_int
    != 0
  {
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).charmode as libc::c_int
      == CHM_TRY as libc::c_int
    {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).charmode = CHM_ON as libc::c_int as byte;
      printf(
        b"\r\nEntering %s mode\r\nEscape character is \'^%c\'.\r\n\x00" as *const u8
          as *const libc::c_char,
        b"character\x00" as *const u8 as *const libc::c_char,
        ']' as i32,
      );
      rawmode();
    }
  } else if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).charmode as libc::c_int
    != CHM_OFF as libc::c_int
  {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).charmode = CHM_OFF as libc::c_int as byte;
    printf(
      b"\r\nEntering %s mode\r\nEscape character is \'^%c\'.\r\n\x00" as *const u8
        as *const libc::c_char,
      b"line\x00" as *const u8 as *const libc::c_char,
      'C' as i32,
    );
    cookmode();
  };
}
unsafe extern "C" fn will_charmode() {
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).charmode = CHM_TRY as libc::c_int as byte;
  let ref mut fresh5 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telflags;
  *fresh5 = (*fresh5 as libc::c_int | (UF_ECHO as libc::c_int | UF_SGA as libc::c_int)) as byte;
  setConMode();
  put_iac3_IAC_x_y_merged(((253i32 << 8i32) + 1i32) as libc::c_uint);
  put_iac3_IAC_x_y_merged(((253i32 << 8i32) + 3i32) as libc::c_uint);
  iac_flush();
}
unsafe extern "C" fn do_linemode() {
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).charmode = CHM_TRY as libc::c_int as byte;
  let ref mut fresh6 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telflags;
  *fresh6 = (*fresh6 as libc::c_int & !(UF_ECHO as libc::c_int | UF_SGA as libc::c_int)) as byte;
  setConMode();
  put_iac3_IAC_x_y_merged(((254i32 << 8i32) + 1i32) as libc::c_uint);
  put_iac3_IAC_x_y_merged(((254i32 << 8i32) + 3i32) as libc::c_uint);
  iac_flush();
}
unsafe extern "C" fn to_notsup(mut c: libc::c_char) {
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telwish as libc::c_int == 251i32 {
    put_iac3_IAC_x_y_merged(((254i32 << 8i32) + c as libc::c_int) as libc::c_uint);
  } else if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telwish as libc::c_int == 253i32 {
    put_iac3_IAC_x_y_merged(((252i32 << 8i32) + c as libc::c_int) as libc::c_uint);
  };
}
unsafe extern "C" fn to_echo() {
  /* if server requests ECHO, don't agree */
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telwish as libc::c_int == 253i32 {
    put_iac3_IAC_x_y_merged(((252i32 << 8i32) + 1i32) as libc::c_uint);
    return;
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telwish as libc::c_int == 254i32 {
    return;
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telflags as libc::c_int
    & UF_ECHO as libc::c_int
    != 0
  {
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telwish as libc::c_int == 251i32 {
      return;
    }
  } else if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telwish as libc::c_int == 252i32 {
    return;
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).charmode as libc::c_int
    != CHM_OFF as libc::c_int
  {
    let ref mut fresh7 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telflags;
    *fresh7 = (*fresh7 as libc::c_int ^ UF_ECHO as libc::c_int) as byte
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telflags as libc::c_int
    & UF_ECHO as libc::c_int
    != 0
  {
    put_iac3_IAC_x_y_merged(((253i32 << 8i32) + 1i32) as libc::c_uint);
  } else {
    put_iac3_IAC_x_y_merged(((254i32 << 8i32) + 1i32) as libc::c_uint);
  }
  setConMode();
  full_write1_str(b"\r\n\x00" as *const u8 as *const libc::c_char);
  /* sudden modec */
}
unsafe extern "C" fn to_sga() {
  /* daemon always sends will/wont, client do/dont */
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telflags as libc::c_int
    & UF_SGA as libc::c_int
    != 0
  {
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telwish as libc::c_int == 251i32 {
      return;
    }
  } else if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telwish as libc::c_int == 252i32 {
    return;
  } /* toggle */
  let ref mut fresh8 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telflags;
  *fresh8 = (*fresh8 as libc::c_int ^ UF_SGA as libc::c_int) as byte;
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telflags as libc::c_int
    & UF_SGA as libc::c_int
    != 0
  {
    put_iac3_IAC_x_y_merged(((253i32 << 8i32) + 3i32) as libc::c_uint);
  } else {
    put_iac3_IAC_x_y_merged(((254i32 << 8i32) + 3i32) as libc::c_uint);
  };
}
unsafe extern "C" fn to_ttype() {
  /* Tell server we will (or won't) do TTYPE */
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .ttype
    .is_null()
  {
    put_iac3_IAC_x_y_merged(((251i32 << 8i32) + 24i32) as libc::c_uint);
  } else {
    put_iac3_IAC_x_y_merged(((252i32 << 8i32) + 24i32) as libc::c_uint);
  };
}
unsafe extern "C" fn to_new_environ() {
  /* Tell server we will (or will not) do AUTOLOGIN */
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .autologin
    .is_null()
  {
    put_iac3_IAC_x_y_merged(((251i32 << 8i32) + 39i32) as libc::c_uint);
  } else {
    put_iac3_IAC_x_y_merged(((252i32 << 8i32) + 39i32) as libc::c_uint);
  };
}
unsafe extern "C" fn to_naws() {
  /* Tell server we will do NAWS */
  put_iac3_IAC_x_y_merged(((251i32 << 8i32) + 31i32) as libc::c_uint);
}
unsafe extern "C" fn telopt(mut c: byte) {
  match c as libc::c_int {
    1 => {
      to_echo();
    }
    3 => {
      to_sga();
    }
    24 => {
      to_ttype();
    }
    39 => {
      to_new_environ();
    }
    31 => {
      to_naws();
      put_iac_naws(
        c,
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).win_width as libc::c_int,
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).win_height as libc::c_int,
      );
    }
    _ => {
      to_notsup(c as libc::c_char);
    }
  };
}
/* subnegotiation -- ignore all (except TTYPE,NAWS) */
unsafe extern "C" fn subneg(mut c: byte) {
  match (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telstate as libc::c_int {
    4 => {
      if c as libc::c_int == 255i32 {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telstate =
          TS_SUB2 as libc::c_int as byte
      } else if c as libc::c_int == 24i32
        && !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .ttype
          .is_null()
      {
        put_iac_subopt(
          24i32 as byte,
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ttype,
        );
      } else if c as libc::c_int == 39i32
        && !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .autologin
          .is_null()
      {
        put_iac_subopt_autologin();
      }
    }
    5 => {
      if c as libc::c_int == 240i32 {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telstate =
          TS_COPY as libc::c_int as byte;
        return;
      }
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).telstate = TS_SUB1 as libc::c_int as byte
    }
    _ => {}
  };
}
unsafe extern "C" fn rawmode() {
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).do_termios != 0 {
    tcsetattr(
      0i32,
      1i32,
      &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).termios_raw,
    );
  };
}
unsafe extern "C" fn cookmode() {
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).do_termios != 0 {
    tcsetattr(
      0i32,
      1i32,
      &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).termios_def,
    );
  };
}
#[no_mangle]
pub unsafe extern "C" fn telnet_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut host: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut port: libc::c_int = 0;
  let mut len: libc::c_int = 0;
  let mut ufds: [pollfd; 2] = [pollfd {
    fd: 0,
    events: 0,
    revents: 0,
  }; 2];
  let ref mut fresh9 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).ttype;
  *fresh9 = getenv(b"TERM\x00" as *const u8 as *const libc::c_char);
  if tcgetattr(
    0i32,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).termios_def,
  ) >= 0i32
  {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).do_termios = 1i32 as byte;
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).termios_raw =
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).termios_def;
    cfmakeraw(&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).termios_raw);
  }
  if 1i32 as libc::c_uint
    == getopt32(
      argv,
      b"al:\x00" as *const u8 as *const libc::c_char,
      &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).autologin
        as *mut *const libc::c_char,
    )
  {
    /* Only -a without -l USER picks $USER from envvar */
    let ref mut fresh10 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).autologin;
    *fresh10 = getenv(b"USER\x00" as *const u8 as *const libc::c_char)
  }
  argv = argv.offset(optind as isize);
  if (*argv).is_null() {
    bb_show_usage();
  }
  let fresh11 = argv;
  argv = argv.offset(1);
  host = *fresh11;
  port = if !(*argv).is_null() {
    let fresh12 = argv;
    argv = argv.offset(1);
    bb_lookup_port(
      *fresh12,
      b"tcp\x00" as *const u8 as *const libc::c_char,
      23i32 as libc::c_uint,
    )
  } else {
    23i32 as libc::c_uint
  } as libc::c_int;
  if !(*argv).is_null() {
    /* extra params?? */
    bb_show_usage();
  }
  xmove_fd(
    create_and_connect_stream_or_die(host, port),
    netfd as libc::c_int,
  );
  printf(
    b"Connected to %s\n\x00" as *const u8 as *const libc::c_char,
    host,
  );
  setsockopt_keepalive(netfd as libc::c_int);
  get_terminal_width_height(
    0i32,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).win_width,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).win_height,
  );
  //TODO: support dynamic resize?
  signal(
    2i32,
    Some(record_signo as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  ufds[0].fd = 0i32;
  ufds[0].events = 0x1i32 as libc::c_short;
  ufds[1].fd = netfd as libc::c_int;
  ufds[1].events = 0x1i32 as libc::c_short;
  loop {
    if poll(ufds.as_mut_ptr(), 2i32 as nfds_t, -1i32) < 0i32 {
      /* error, ignore and/or log something, bay go to loop */
      if bb_got_signal != 0 {
        con_escape();
      } else {
        sleep(1i32 as libc::c_uint);
      }
    } else {
      // FIXME: reads can block. Need full bidirectional buffering.
      if ufds[0].revents != 0 {
        len = safe_read(
          0i32,
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .buf
            .as_mut_ptr() as *mut libc::c_void,
          DATABUFSIZE as libc::c_int as size_t,
        ) as libc::c_int;
        if len <= 0i32 {
          doexit(0i32);
        }
        handle_net_output(len);
      }
      if ufds[1].revents != 0 {
        len = safe_read(
          netfd as libc::c_int,
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .buf
            .as_mut_ptr() as *mut libc::c_void,
          DATABUFSIZE as libc::c_int as size_t,
        ) as libc::c_int;
        if len <= 0i32 {
          full_write1_str(
            b"Connection closed by foreign host\r\n\x00" as *const u8 as *const libc::c_char,
          );
          doexit(1i32);
        }
        handle_net_input(len);
      }
    }
  }
  /* while (1) */
}
