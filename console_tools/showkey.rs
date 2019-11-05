use libc;
extern "C" {
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
  #[no_mangle]
  static ptr_to_globals: *mut globals;
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
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn bb_signals_recursive_norestart(
    sigs: libc::c_int,
    f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
  );
  #[no_mangle]
  static mut bb_got_signal: smallint;
  #[no_mangle]
  fn record_signo(signo: libc::c_int);
  #[no_mangle]
  static mut option_mask32: uint32_t;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_xioctl(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
use crate::librb::uint32_t;
use crate::librb::smallint;
use crate::librb::ssize_t;
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
  pub c_iflag: tcflag_t,
  pub c_oflag: tcflag_t,
  pub c_cflag: tcflag_t,
  pub c_lflag: tcflag_t,
  pub c_line: cc_t,
  pub c_cc: [cc_t; 32],
  pub c_ispeed: speed_t,
  pub c_ospeed: speed_t,
}
pub type C2RustUnnamed = libc::c_uint;
pub const BB_FATAL_SIGS: C2RustUnnamed = 117503054;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub kbmode: libc::c_int,
  pub tio: termios,
  pub tio0: termios,
}
// display only the interpreted keycodes (default)
pub const OPT_s: C2RustUnnamed_0 = 4;
// display the decimal/octal/hex values of the keys
pub const OPT_k: C2RustUnnamed_0 = 2;
pub const OPT_a: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
// display only the raw scan-codes
// set raw tty mode
// also used by microcom
// libbb candidates?
unsafe extern "C" fn xget1(mut t: *mut termios, mut oldt: *mut termios) {
  tcgetattr(0i32, oldt);
  *t = *oldt;
  cfmakeraw(t);
}
unsafe extern "C" fn xset1(mut t: *mut termios) {
  let mut ret: libc::c_int = tcsetattr(0i32, 2i32, t);
  if ret != 0 {
    bb_simple_perror_msg(b"can\'t tcsetattr for stdin\x00" as *const u8 as *const libc::c_char);
  };
}
#[no_mangle]
pub unsafe extern "C" fn showkey_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let ref mut fresh0 = *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
    as *mut *mut globals);
  *fresh0 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  // FIXME: aks are all mutually exclusive
  getopt32(argv, b"aks\x00" as *const u8 as *const libc::c_char);
  // prepare for raw mode
  xget1(&mut (*ptr_to_globals).tio, &mut (*ptr_to_globals).tio0);
  // put stdin in raw mode
  xset1(&mut (*ptr_to_globals).tio);
  if option_mask32 & OPT_a as libc::c_int as libc::c_uint != 0 {
    // just read stdin char by char
    let mut c: libc::c_uchar = 0;
    printf(
      b"Press any keys, program terminates %s:\r\n\n\x00" as *const u8 as *const libc::c_char,
      b"on EOF (ctrl-D)\x00" as *const u8 as *const libc::c_char,
    );
    // read and show byte values
    while 1i32 as libc::c_long
      == read(
        0i32,
        &mut c as *mut libc::c_uchar as *mut libc::c_void,
        1i32 as size_t,
      )
    {
      printf(
        b"%3u 0%03o 0x%02x\r\n\x00" as *const u8 as *const libc::c_char,
        c as libc::c_int,
        c as libc::c_int,
        c as libc::c_int,
      );
      if 0o4i32 == c as libc::c_int {
        break;
      }
    }
  } else {
    // we assume a PC keyboard
    bb_xioctl(
      0i32,
      0x4b44i32 as libc::c_uint,
      &mut (*ptr_to_globals).kbmode as *mut libc::c_int as *mut libc::c_void,
      b"KDGKBMODE\x00" as *const u8 as *const libc::c_char,
    );
    printf(
      b"Keyboard mode was %s.\r\n\n\x00" as *const u8 as *const libc::c_char,
      if (*ptr_to_globals).kbmode == 0i32 {
        b"RAW\x00" as *const u8 as *const libc::c_char
      } else if (*ptr_to_globals).kbmode == 0x1i32 {
        b"XLATE\x00" as *const u8 as *const libc::c_char
      } else if (*ptr_to_globals).kbmode == 0x2i32 {
        b"MEDIUMRAW\x00" as *const u8 as *const libc::c_char
      } else if (*ptr_to_globals).kbmode == 0x3i32 {
        b"UNICODE\x00" as *const u8 as *const libc::c_char
      } else {
        b"UNKNOWN\x00" as *const u8 as *const libc::c_char
      },
    );
    // set raw keyboard mode
    bb_xioctl(
      0i32,
      0x4b45i32 as libc::c_uint,
      if option_mask32 & OPT_k as libc::c_int as libc::c_uint != 0 {
        0x2i32
      } else {
        0i32
      } as ptrdiff_t as *mut libc::c_void,
      b"KDSKBMODE\x00" as *const u8 as *const libc::c_char,
    );
    // we should exit on any signal; signals should interrupt read
    bb_signals_recursive_norestart(
      BB_FATAL_SIGS as libc::c_int,
      Some(record_signo as unsafe extern "C" fn(_: libc::c_int) -> ()),
    );
    // inform user that program ends after time of inactivity
    printf(
      b"Press any keys, program terminates %s:\r\n\n\x00" as *const u8 as *const libc::c_char,
      b"10s after last keypress\x00" as *const u8 as *const libc::c_char,
    );
    // read and show scancodes
    while bb_got_signal == 0 {
      let mut buf: [libc::c_char; 18] = [0; 18];
      let mut i: libc::c_int = 0;
      let mut n: libc::c_int = 0;
      // setup 10s watchdog
      alarm(10i32 as libc::c_uint);
      // read scancodes
      n = read(
        0i32,
        buf.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong,
      ) as libc::c_int;
      i = 0i32;
      while i < n {
        if option_mask32 & OPT_s as libc::c_int as libc::c_uint != 0 {
          // show raw scancodes
          let fresh1 = i;
          i = i + 1;
          printf(
            b"0x%02x \x00" as *const u8 as *const libc::c_char,
            buf[fresh1 as usize] as libc::c_int,
          );
        } else {
          // show interpreted scancodes (default)
          let mut c_0: libc::c_char = buf[i as usize];
          let mut kc: libc::c_int = 0;
          if i + 2i32 < n
            && c_0 as libc::c_int & 0x7fi32 == 0i32
            && buf[(i + 1i32) as usize] as libc::c_int & 0x80i32 != 0i32
            && buf[(i + 2i32) as usize] as libc::c_int & 0x80i32 != 0i32
          {
            kc = (buf[(i + 1i32) as usize] as libc::c_int & 0x7fi32) << 7i32
              | buf[(i + 2i32) as usize] as libc::c_int & 0x7fi32;
            i += 3i32
          } else {
            kc = c_0 as libc::c_int & 0x7fi32;
            i += 1
          }
          printf(
            b"keycode %3u %s\x00" as *const u8 as *const libc::c_char,
            kc,
            if c_0 as libc::c_int & 0x80i32 != 0 {
              b"release\x00" as *const u8 as *const libc::c_char
            } else {
              b"press\x00" as *const u8 as *const libc::c_char
            },
          );
        }
      }
      puts(b"\r\x00" as *const u8 as *const libc::c_char);
    }
    // restore keyboard mode
    bb_xioctl(
      0i32,
      0x4b45i32 as libc::c_uint,
      (*ptr_to_globals).kbmode as ptrdiff_t as *mut libc::c_void,
      b"KDSKBMODE\x00" as *const u8 as *const libc::c_char,
    );
  }
  // restore console settings
  xset1(&mut (*ptr_to_globals).tio0);
  return 0i32;
}
