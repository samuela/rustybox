use libc;
extern "C" {
  #[no_mangle]
  fn exit(_: libc::c_int) -> !;
  #[no_mangle]
  fn getpid() -> __pid_t;
  #[no_mangle]
  fn setsid() -> __pid_t;
  #[no_mangle]
  fn vfork() -> libc::c_int;
  #[no_mangle]
  fn tcsetpgrp(__fd: libc::c_int, __pgrp_id: __pid_t) -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
  #[no_mangle]
  static mut stdout: *mut FILE;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn execl(__path: *const libc::c_char, __arg: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
  #[no_mangle]
  fn mknod(__path: *const libc::c_char, __mode: __mode_t, __dev: __dev_t) -> libc::c_int;
  #[no_mangle]
  fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
  #[no_mangle]
  fn tcsetattr(
    __fd: libc::c_int,
    __optional_actions: libc::c_int,
    __termios_p: *const termios,
  ) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xdup2(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn bb_signals(sigs: libc::c_int, f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>);
  #[no_mangle]
  fn kill_myself_with_sig(sig: libc::c_int) -> !;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xread(fd: libc::c_int, buf: *mut libc::c_void, count: size_t);
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn xatou_range(str: *const libc::c_char, l: libc::c_uint, u: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  static mut option_mask32: uint32_t;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> uint32_t;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn get_shell_name() -> *const libc::c_char;
  #[no_mangle]
  fn get_terminal_width_height(
    fd: libc::c_int,
    width: *mut libc::c_uint,
    height: *mut libc::c_uint,
  ) -> libc::c_int;
  #[no_mangle]
  fn tcsetattr_stdin_TCSANOW(tp: *const termios) -> libc::c_int;
  #[no_mangle]
  fn set_termios_to_raw(fd: libc::c_int, oldterm: *mut termios, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn bb_makedev(major: libc::c_uint, minor: libc::c_uint) -> libc::c_ulonglong;
  #[no_mangle]
  static ptr_to_globals: *mut globals;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type smallint = libc::c_schar;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type pid_t = __pid_t;
pub type dev_t = __dev_t;
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;

pub type _IO_lock_t = ();

use crate::librb::FILE;
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
  pub fd: libc::c_int,
  pub events: libc::c_short,
  pub revents: libc::c_short,
}
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
  pub data: *mut libc::c_char,
  pub size: libc::c_int,
  pub x: libc::c_int,
  pub y: libc::c_int,
  pub kbd_fd: libc::c_int,
  pub ioerror_count: libc::c_int,
  pub key_count: libc::c_int,
  pub escape_count: libc::c_int,
  pub nokeys: libc::c_int,
  pub current: libc::c_int,
  pub first_line_offset: libc::c_int,
  pub last_attr: libc::c_int,
  pub width: libc::c_uint,
  pub height: libc::c_uint,
  pub col: libc::c_uint,
  pub line: libc::c_uint,
  pub curoff: smallint,
  pub attrbuf: [libc::c_char; 13],
  pub remote: screen_info,
  pub term_orig: termios,
  pub vcsa_name: [libc::c_char; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct screen_info {
  pub lines: libc::c_uchar,
  pub cols: libc::c_uchar,
  pub cursor_x: libc::c_uchar,
  pub cursor_y: libc::c_uchar,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const COMMON_BUFSIZE: C2RustUnnamed_0 = 1024;
pub type C2RustUnnamed_1 = libc::c_uint;
// framebuffer
// follow cursor
pub const FLAG_F: C2RustUnnamed_1 = 7;
// dump screen
pub const FLAG_f: C2RustUnnamed_1 = 6;
// no colors
pub const FLAG_d: C2RustUnnamed_1 = 5;
// session
pub const FLAG_n: C2RustUnnamed_1 = 4;
// never exit
pub const FLAG_s: C2RustUnnamed_1 = 3;
// create device if need
pub const FLAG_Q: C2RustUnnamed_1 = 2;
// view only
pub const FLAG_c: C2RustUnnamed_1 = 1;
pub const FLAG_v: C2RustUnnamed_1 = 0;
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
unsafe extern "C" fn putcsi(mut s: *const libc::c_char) {
  fputs_unlocked(b"\x1b[\x00" as *const u8 as *const libc::c_char, stdout);
  fputs_unlocked(s, stdout);
}
unsafe extern "C" fn clrscr() {
  // Home, clear till end of screen
  putcsi(b"1;1H\x1b[J\x00" as *const u8 as *const libc::c_char);
  (*ptr_to_globals).line = 0i32 as libc::c_uint;
  (*ptr_to_globals).col = (*ptr_to_globals).line;
}
unsafe extern "C" fn set_cursor(mut state: libc::c_int) {
  if (*ptr_to_globals).curoff as libc::c_int != state {
    (*ptr_to_globals).curoff = state as smallint;
    putcsi(b"?25\x00" as *const u8 as *const libc::c_char);
    bb_putchar(
      (*::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"h?l\x00"))[(1i32 + state) as usize]
        as libc::c_int,
    );
  };
}
unsafe extern "C" fn gotoxy(mut col: libc::c_int, mut line: libc::c_int) {
  if (*ptr_to_globals).col != col as libc::c_uint || (*ptr_to_globals).line != line as libc::c_uint
  {
    (*ptr_to_globals).col = col as libc::c_uint;
    (*ptr_to_globals).line = line as libc::c_uint;
    printf(
      b"\x1b[%u;%uH\x00" as *const u8 as *const libc::c_char,
      line + 1i32,
      col + 1i32,
    );
  };
}
unsafe extern "C" fn cleanup(mut code: libc::c_int) -> ! {
  set_cursor(-1i32);
  tcsetattr(
    (*ptr_to_globals).kbd_fd,
    0i32,
    &mut (*ptr_to_globals).term_orig,
  );
  // Reset attributes
  if option_mask32 & (1i32 << FLAG_n as libc::c_int) as libc::c_uint == 0 {
    putcsi(b"0m\x00" as *const u8 as *const libc::c_char);
  }
  bb_putchar('\n' as i32);
  if code > 1i32 {
    kill_myself_with_sig(code);
  }
  exit(code);
}
unsafe extern "C" fn screen_read_close() {
  let mut i: libc::c_uint = 0;
  let mut j: libc::c_uint = 0;
  let mut vcsa_fd: libc::c_int = 0;
  let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
  // Close & re-open vcsa in case they have swapped virtual consoles
  vcsa_fd = xopen((*ptr_to_globals).vcsa_name.as_mut_ptr(), 0i32); // if will catch j < G.x too
  xread(
    vcsa_fd,
    &mut (*ptr_to_globals).remote as *mut screen_info as *mut libc::c_void,
    4i32 as size_t,
  ); // if will catch i < G.y too
  i = ((*ptr_to_globals).remote.cols as libc::c_int * 2i32) as libc::c_uint;
  (*ptr_to_globals).first_line_offset =
    ((*ptr_to_globals).y as libc::c_uint).wrapping_mul(i) as libc::c_int;
  i = i.wrapping_mul((*ptr_to_globals).remote.lines as libc::c_uint);
  if (*ptr_to_globals).data.is_null() {
    (*ptr_to_globals).size = i as libc::c_int;
    (*ptr_to_globals).data =
      xzalloc((2i32 as libc::c_uint).wrapping_mul(i) as size_t) as *mut libc::c_char
  }
  if (*ptr_to_globals).size as libc::c_uint != i {
    cleanup(1i32);
  }
  data = (*ptr_to_globals)
    .data
    .offset((*ptr_to_globals).current as isize);
  xread(
    vcsa_fd,
    data as *mut libc::c_void,
    (*ptr_to_globals).size as size_t,
  );
  close(vcsa_fd);
  i = 0i32 as libc::c_uint;
  while i < (*ptr_to_globals).remote.lines as libc::c_uint {
    j = 0i32 as libc::c_uint;
    while j < (*ptr_to_globals).remote.cols as libc::c_uint {
      let mut x: libc::c_uint = j.wrapping_sub((*ptr_to_globals).x as libc::c_uint);
      let mut y: libc::c_uint = i.wrapping_sub((*ptr_to_globals).y as libc::c_uint);
      if y >= (*ptr_to_globals).height || x >= (*ptr_to_globals).width {
        *(data as *mut uint16_t) = 0i32 as uint16_t
      } else {
        let mut ch: uint8_t = *(data as *mut uint8_t);
        if (ch as libc::c_int) < ' ' as i32 {
          *(data as *mut uint8_t) = (ch as libc::c_int | 0x40i32) as uint8_t
        } else if ch as libc::c_int > 0x7ei32 {
          *(data as *mut uint8_t) = '?' as i32 as uint8_t
        }
      }
      j = j.wrapping_add(1);
      data = data.offset(2)
    }
    i = i.wrapping_add(1)
  }
}
unsafe extern "C" fn screen_char(mut data: *mut libc::c_char) {
  if option_mask32 & (1i32 << FLAG_n as libc::c_int) as libc::c_uint == 0 {
    let mut attr_diff: uint8_t = 0;
    let mut attr: uint8_t = *(data as *mut uint8_t).offset(1);
    if option_mask32 & (1i32 << FLAG_F as libc::c_int) as libc::c_uint != 0 {
      attr = (attr as libc::c_int >> 1i32) as uint8_t
    }
    attr_diff = ((*ptr_to_globals).last_attr ^ attr as libc::c_int) as uint8_t;
    if attr_diff != 0 {
      // Attribute layout for VGA compatible text videobuffer:
      // blinking text
      // |red bkgd
      // ||green bkgd
      // |||blue bkgd
      // vvvv
      // 00000000 <- lsb bit on the right
      //     bold text / text 8th bit
      //      red text
      //       green text
      //        blue text
      // TODO: apparently framebuffer-based console uses different layout
      // (bug? attempt to get 8th text bit in better position?)
      // red bkgd
      // |green bkgd
      // ||blue bkgd
      // vvv
      // 00000000 <- lsb bit on the right
      //    bold text
      //     red text
      //      green text
      //       blue text
      //        text 8th bit
      // converting RGB color bit triad to BGR:
      static mut color: [libc::c_char; 8] = [48, 52, 50, 54, 49, 53, 51, 55];
      let fg_mask: uint8_t = 0x7i32 as uint8_t;
      let bold_mask: uint8_t = 0x8i32 as uint8_t;
      let bg_mask: uint8_t = 0x70i32 as uint8_t;
      let blink_mask: uint8_t = 0x80i32 as uint8_t;
      let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
      ptr = (*ptr_to_globals).attrbuf.as_mut_ptr();
      // (G.last_attr & ~attr) has 1 only where
      // G.last_attr has 1 but attr has 0.
      // Here we check whether we have transition
      // bold->non-bold or blink->non-blink:
      if (*ptr_to_globals).last_attr < 0i32
        || (*ptr_to_globals).last_attr
          & !(attr as libc::c_int)
          & (bold_mask as libc::c_int | blink_mask as libc::c_int)
          != 0i32
      {
        let fresh0 = ptr; // "reset all attrs"
        ptr = ptr.offset(1);
        *fresh0 = '0' as i32 as libc::c_char;
        let fresh1 = ptr;
        ptr = ptr.offset(1);
        *fresh1 = ';' as i32 as libc::c_char;
        // must set fg & bg, maybe need to set bold or blink:
        attr_diff =
          (attr as libc::c_int | !(bold_mask as libc::c_int | blink_mask as libc::c_int)) as uint8_t
      }
      (*ptr_to_globals).last_attr = attr as libc::c_int;
      if attr_diff as libc::c_int & bold_mask as libc::c_int != 0 {
        let fresh2 = ptr;
        ptr = ptr.offset(1);
        *fresh2 = '1' as i32 as libc::c_char;
        let fresh3 = ptr;
        ptr = ptr.offset(1);
        *fresh3 = ';' as i32 as libc::c_char
      }
      if attr_diff as libc::c_int & blink_mask as libc::c_int != 0 {
        let fresh4 = ptr;
        ptr = ptr.offset(1);
        *fresh4 = '5' as i32 as libc::c_char;
        let fresh5 = ptr;
        ptr = ptr.offset(1);
        *fresh5 = ';' as i32 as libc::c_char
      }
      if attr_diff as libc::c_int & fg_mask as libc::c_int != 0 {
        let fresh6 = ptr;
        ptr = ptr.offset(1);
        *fresh6 = '3' as i32 as libc::c_char;
        let fresh7 = ptr;
        ptr = ptr.offset(1);
        *fresh7 = color[(attr as libc::c_int & fg_mask as libc::c_int) as usize];
        let fresh8 = ptr;
        ptr = ptr.offset(1);
        *fresh8 = ';' as i32 as libc::c_char
      }
      if attr_diff as libc::c_int & bg_mask as libc::c_int != 0 {
        let fresh9 = ptr;
        ptr = ptr.offset(1);
        *fresh9 = '4' as i32 as libc::c_char;
        let fresh10 = ptr;
        ptr = ptr.offset(1);
        *fresh10 = color[((attr as libc::c_int & bg_mask as libc::c_int) >> 4i32) as usize];
        ptr = ptr.offset(1)
        // last attribute
      } // if will catch col < G.x too
      if ptr != (*ptr_to_globals).attrbuf.as_mut_ptr() {
        *ptr.offset(-1i32 as isize) = 'm' as i32 as libc::c_char;
        *ptr = '\u{0}' as i32 as libc::c_char;
        putcsi((*ptr_to_globals).attrbuf.as_mut_ptr());
      }
    }
  }
  putchar_unlocked(*(data as *mut uint8_t) as libc::c_int);
  (*ptr_to_globals).col = (*ptr_to_globals).col.wrapping_add(1);
}
unsafe extern "C" fn screen_dump() {
  let mut linefeed_cnt: libc::c_int = 0;
  let mut line: libc::c_int = 0;
  let mut col: libc::c_int = 0;
  let mut linecnt: libc::c_int =
    (*ptr_to_globals).remote.lines as libc::c_int - (*ptr_to_globals).y;
  let mut data: *mut libc::c_char = (*ptr_to_globals)
    .data
    .offset((*ptr_to_globals).current as isize)
    .offset((*ptr_to_globals).first_line_offset as isize);
  linefeed_cnt = 0i32;
  line = 0i32;
  while line < linecnt && (line as libc::c_uint) < (*ptr_to_globals).height {
    let mut space_cnt: libc::c_int = 0i32;
    col = 0i32;
    while col < (*ptr_to_globals).remote.cols as libc::c_int {
      let mut tty_col: libc::c_uint = (col - (*ptr_to_globals).x) as libc::c_uint;
      if !(tty_col >= (*ptr_to_globals).width) {
        space_cnt += 1;
        if !(option_mask32 & (1i32 << FLAG_n as libc::c_int) as libc::c_uint != 0
          && *(data as *mut uint8_t) as libc::c_int == ' ' as i32)
        {
          while linefeed_cnt != 0i32 {
            //bb_putchar('\r'); - tty driver does it for us
            bb_putchar('\n' as i32);
            linefeed_cnt -= 1
          }
          loop {
            space_cnt -= 1;
            if !(space_cnt != 0) {
              break;
            }
            bb_putchar(' ' as i32);
          }
          screen_char(data);
        }
      }
      col += 1;
      data = data.offset(2)
    }
    linefeed_cnt += 1;
    line += 1
  }
}
unsafe extern "C" fn curmove() {
  let mut cx: libc::c_uint =
    ((*ptr_to_globals).remote.cursor_x as libc::c_int - (*ptr_to_globals).x) as libc::c_uint;
  let mut cy: libc::c_uint =
    ((*ptr_to_globals).remote.cursor_y as libc::c_int - (*ptr_to_globals).y) as libc::c_uint;
  let mut cursor: libc::c_int = 1i32;
  if cx < (*ptr_to_globals).width && cy < (*ptr_to_globals).height {
    gotoxy(cx as libc::c_int, cy as libc::c_int);
    cursor = -1i32
  }
  set_cursor(cursor);
}
unsafe extern "C" fn create_cdev_if_doesnt_exist(mut name: *const libc::c_char, mut dev: dev_t) {
  let mut fd: libc::c_int = open(name, 0i32);
  if fd != -1i32 {
    close(fd);
  } else if *bb_errno == 2i32 {
    mknod(name, (0o20000i32 | 0o660i32) as __mode_t, dev);
  };
}
#[inline(never)]
unsafe extern "C" fn start_shell_in_child(mut tty_name: *const libc::c_char) {
  let mut pid: libc::c_int = {
    let mut bb__xvfork_pid: pid_t = vfork();
    if bb__xvfork_pid < 0i32 {
      bb_simple_perror_msg_and_die(b"vfork\x00" as *const u8 as *const libc::c_char);
    }
    bb__xvfork_pid
  };
  if pid == 0i32 {
    let mut termchild: termios = termios {
      c_iflag: 0,
      c_oflag: 0,
      c_cflag: 0,
      c_lflag: 0,
      c_line: 0,
      c_cc: [0; 32],
      c_ispeed: 0,
      c_ospeed: 0,
    };
    let mut shell: *const libc::c_char = get_shell_name();
    signal(
      1i32,
      ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
    );
    // set tty as a controlling tty
    setsid();
    // make tty to be input, output, error
    close(0i32); // uses fd 0
    xopen(tty_name, 0o2i32);
    xdup2(0i32, 1i32);
    xdup2(0i32, 2i32);
    ioctl(0i32, 0x540ei32 as libc::c_ulong, 1i32);
    tcsetpgrp(0i32, getpid());
    tcgetattr(0i32, &mut termchild);
    termchild.c_lflag |= 0o10i32 as libc::c_uint;
    termchild.c_oflag |= (0o4i32 | 0o14000i32) as libc::c_uint;
    termchild.c_iflag |= 0o400i32 as libc::c_uint;
    termchild.c_iflag &= !0o10000i32 as libc::c_uint;
    tcsetattr_stdin_TCSANOW(&mut termchild);
    execl(
      shell,
      shell,
      b"-i\x00" as *const u8 as *const libc::c_char,
      0 as *mut libc::c_void as *mut libc::c_char,
    );
    bb_simple_perror_msg_and_die(shell);
  };
}
#[no_mangle]
pub unsafe extern "C" fn conspy_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut k: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut tty_name: [libc::c_char; 11] = [0; 11];
  let mut opts: libc::c_uint = 0;
  let mut ttynum: libc::c_uint = 0;
  let mut poll_timeout_ms: libc::c_int = 0;
  static mut conspy_longopts: [libc::c_char; 90] = [
    118, 105, 101, 119, 111, 110, 108, 121, 0, 0, 118, 99, 114, 101, 97, 116, 101, 100, 101, 118,
    105, 99, 101, 0, 0, 99, 110, 101, 118, 101, 114, 113, 117, 105, 116, 0, 0, 81, 115, 101, 115,
    115, 105, 111, 110, 0, 0, 115, 110, 111, 99, 111, 108, 111, 114, 115, 0, 0, 110, 100, 117, 109,
    112, 0, 0, 100, 102, 111, 108, 108, 111, 119, 0, 0, 102, 102, 114, 97, 109, 101, 98, 117, 102,
    102, 101, 114, 0, 0, 70, 0,
  ];
  let ref mut fresh11 =
    *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
      as *mut *mut globals);
  *fresh11 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  (*ptr_to_globals).height = (2147483647i32 as libc::c_uint)
    .wrapping_mul(2u32)
    .wrapping_add(1u32);
  (*ptr_to_globals).width = (*ptr_to_globals).height;
  (*ptr_to_globals).last_attr -= 1;
  strcpy(
    (*ptr_to_globals).vcsa_name.as_mut_ptr(),
    b"/dev/vcsa\x00" as *const u8 as *const libc::c_char,
  );
  // numeric params
  opts = getopt32long(
    argv,
    b"vcQsndfFx:+y:+\x00" as *const u8 as *const libc::c_char,
    conspy_longopts.as_ptr(),
    &mut (*ptr_to_globals).x as *mut libc::c_int,
    &mut (*ptr_to_globals).y as *mut libc::c_int,
  );
  argv = argv.offset(optind as isize);
  ttynum = 0i32 as libc::c_uint;
  if !(*argv.offset(0)).is_null() {
    ttynum = xatou_range(*argv.offset(0), 0i32 as libc::c_uint, 63i32 as libc::c_uint);
    sprintf(
      (*ptr_to_globals)
        .vcsa_name
        .as_mut_ptr()
        .offset(::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as isize)
        .offset(-1),
      b"%u\x00" as *const u8 as *const libc::c_char,
      ttynum,
    );
  }
  sprintf(
    tty_name.as_mut_ptr(),
    b"%s%u\x00" as *const u8 as *const libc::c_char,
    b"/dev/tty\x00" as *const u8 as *const libc::c_char,
    ttynum,
  );
  if opts & (1i32 << FLAG_c as libc::c_int) as libc::c_uint != 0 {
    if opts & (1i32 << FLAG_s as libc::c_int | 1i32 << FLAG_v as libc::c_int) as libc::c_uint
      != (1i32 << FLAG_v as libc::c_int) as libc::c_uint
    {
      create_cdev_if_doesnt_exist(
        tty_name.as_mut_ptr(),
        bb_makedev(4i32 as libc::c_uint, ttynum) as dev_t,
      );
    }
    create_cdev_if_doesnt_exist(
      (*ptr_to_globals).vcsa_name.as_mut_ptr(),
      bb_makedev(
        7i32 as libc::c_uint,
        (128i32 as libc::c_uint).wrapping_add(ttynum),
      ) as dev_t,
    );
  }
  if opts & (1i32 << FLAG_s as libc::c_int) as libc::c_uint != 0 && ttynum != 0 {
    start_shell_in_child(tty_name.as_mut_ptr());
  }
  screen_read_close();
  if opts & (1i32 << FLAG_d as libc::c_int) as libc::c_uint != 0 {
    screen_dump();
    bb_putchar('\n' as i32);
    return 0i32;
  }
  bb_signals(
    BB_FATAL_SIGS as libc::c_int,
    ::std::mem::transmute::<
      Option<unsafe extern "C" fn(_: libc::c_int) -> !>,
      Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    >(Some(cleanup as unsafe extern "C" fn(_: libc::c_int) -> !)),
  );
  (*ptr_to_globals).kbd_fd = xopen(b"/dev/tty\x00" as *const u8 as *const libc::c_char, 0i32);
  // All characters must be passed through to us unaltered
  set_termios_to_raw(
    (*ptr_to_globals).kbd_fd,
    &mut (*ptr_to_globals).term_orig,
    0i32 | 1i32 << 0i32 | 1i32 << 3i32,
  );
  //Note: termios.c_oflag &= ~(OPOST); - no, we still want \n -> \r\n
  poll_timeout_ms = 250i32; /* while (1) */
  loop {
    let mut pfd: pollfd = pollfd {
      fd: 0,
      events: 0,
      revents: 0,
    };
    let mut bytes_read: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut old: *mut libc::c_char = 0 as *mut libc::c_char;
    // in the first loop G.width = G.height = 0: refresh
    i = (*ptr_to_globals).width as libc::c_int;
    j = (*ptr_to_globals).height as libc::c_int;
    get_terminal_width_height(
      (*ptr_to_globals).kbd_fd,
      &mut (*ptr_to_globals).width,
      &mut (*ptr_to_globals).height,
    );
    if option_mask32 & (1i32 << FLAG_f as libc::c_int) as libc::c_uint != 0 {
      let mut nx: libc::c_int = ((*ptr_to_globals).remote.cursor_x as libc::c_uint)
        .wrapping_sub((*ptr_to_globals).width)
        .wrapping_add(1i32 as libc::c_uint) as libc::c_int;
      let mut ny: libc::c_int = ((*ptr_to_globals).remote.cursor_y as libc::c_uint)
        .wrapping_sub((*ptr_to_globals).height)
        .wrapping_add(1i32 as libc::c_uint) as libc::c_int;
      if ((*ptr_to_globals).remote.cursor_x as libc::c_int) < (*ptr_to_globals).x {
        (*ptr_to_globals).x = (*ptr_to_globals).remote.cursor_x as libc::c_int;
        i = 0i32
        // force refresh
      }
      if nx > (*ptr_to_globals).x {
        (*ptr_to_globals).x = nx;
        i = 0i32
        // force refresh
      }
      if ((*ptr_to_globals).remote.cursor_y as libc::c_int) < (*ptr_to_globals).y {
        (*ptr_to_globals).y = (*ptr_to_globals).remote.cursor_y as libc::c_int;
        i = 0i32
        // force refresh
      }
      if ny > (*ptr_to_globals).y {
        (*ptr_to_globals).y = ny;
        i = 0i32
        // force refresh
      }
    }
    // Scan console data and redraw our tty where needed
    old = (*ptr_to_globals)
      .data
      .offset((*ptr_to_globals).current as isize);
    (*ptr_to_globals).current = (*ptr_to_globals).size - (*ptr_to_globals).current;
    data = (*ptr_to_globals)
      .data
      .offset((*ptr_to_globals).current as isize);
    screen_read_close();
    if i as libc::c_uint != (*ptr_to_globals).width || j as libc::c_uint != (*ptr_to_globals).height
    {
      clrscr();
      screen_dump();
    } else {
      // For each remote line
      old = old.offset((*ptr_to_globals).first_line_offset as isize); // first char which needs updating
      data = data.offset((*ptr_to_globals).first_line_offset as isize); // last char which needs updating
      i = (*ptr_to_globals).y; // if will catch j >= G.x too
      while i < (*ptr_to_globals).remote.lines as libc::c_int {
        let mut first: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut last: *mut libc::c_char = 0 as *mut libc::c_char;
        last = last;
        let mut iy: libc::c_uint = (i - (*ptr_to_globals).y) as libc::c_uint;
        if iy >= (*ptr_to_globals).height {
          break;
        }
        j = 0i32;
        while j < (*ptr_to_globals).remote.cols as libc::c_int {
          let mut jx: libc::c_uint = (j - (*ptr_to_globals).x) as libc::c_uint;
          if jx < (*ptr_to_globals).width
            && *(data as *mut uint16_t) as libc::c_int != *(old as *mut uint16_t) as libc::c_int
          {
            last = data;
            if first.is_null() {
              first = data;
              gotoxy(jx as libc::c_int, iy as libc::c_int);
            }
          }
          j += 1;
          old = old.offset(2);
          data = data.offset(2)
        }
        if !first.is_null() {
          // Rewrite updated data on the local screen
          while first <= last {
            screen_char(first);
            first = first.offset(2)
          }
        }
        i += 1
      }
    }
    curmove();
    // Wait for local user keypresses
    fflush_all();
    pfd.fd = (*ptr_to_globals).kbd_fd;
    pfd.events = 0x1i32 as libc::c_short;
    bytes_read = 0i32;
    match poll(&mut pfd, 1i32 as nfds_t, poll_timeout_ms) {
      -1 => {
        if *bb_errno != 4i32 {
          break;
        }
      }
      0 => {
        (*ptr_to_globals).nokeys += 1;
        if (*ptr_to_globals).nokeys >= 4i32 {
          (*ptr_to_globals).escape_count = 0i32;
          (*ptr_to_globals).nokeys = (*ptr_to_globals).escape_count
        }
      }
      _ => {
        // Read the keys pressed
        k = bb_common_bufsiz1
          .as_mut_ptr()
          .offset((*ptr_to_globals).key_count as isize);
        bytes_read = read(
          (*ptr_to_globals).kbd_fd,
          k as *mut libc::c_void,
          (COMMON_BUFSIZE as libc::c_int - (*ptr_to_globals).key_count) as size_t,
        ) as libc::c_int;
        if bytes_read < 0i32 {
          break;
        }
        // Do exit processing
        if option_mask32 & (1i32 << FLAG_Q as libc::c_int) as libc::c_uint == 0 {
          i = 0i32;
          while i < bytes_read {
            if *k.offset(i as isize) as libc::c_int != '\u{1b}' as i32 {
              (*ptr_to_globals).escape_count = -1i32
            }
            (*ptr_to_globals).escape_count += 1;
            if (*ptr_to_globals).escape_count >= 3i32 {
              cleanup(0i32);
            }
            i += 1
          }
        }
      }
    }
    poll_timeout_ms = 250i32;
    if option_mask32 & (1i32 << FLAG_v as libc::c_int) as libc::c_uint != 0 {
      continue;
    }
    // Insert all keys pressed into the virtual console's input
    // buffer.  Don't do this if the virtual console is in scan
    // code mode - giving ASCII characters to a program expecting
    // scan codes will confuse it.
    (*ptr_to_globals).key_count += bytes_read;
    if !((*ptr_to_globals).escape_count == 0i32) {
      continue;
    }
    let mut handle: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    let mut kbd_mode: libc::c_long = 0;
    handle = xopen(tty_name.as_mut_ptr(), 0o1i32);
    result = ioctl(
      handle,
      0x4b44i32 as libc::c_ulong,
      &mut kbd_mode as *mut libc::c_long,
    );
    if result >= 0i32 {
      let mut p: *mut libc::c_char = bb_common_bufsiz1.as_mut_ptr();
      (*ptr_to_globals).ioerror_count = 0i32;
      if kbd_mode != 0x1i32 as libc::c_long && kbd_mode != 0x3i32 as libc::c_long {
        (*ptr_to_globals).key_count = 0i32
        // scan code mode
      }
      while (*ptr_to_globals).key_count != 0i32 {
        result = ioctl(handle, 0x5412i32 as libc::c_ulong, p);
        if result < 0i32 {
          memmove(
            bb_common_bufsiz1.as_mut_ptr() as *mut libc::c_void,
            p as *const libc::c_void,
            (*ptr_to_globals).key_count as libc::c_ulong,
          );
          break;
        } else {
          // If there is an application on console which reacts
          // to keypresses, we need to make our first sleep
          // shorter to quickly redraw whatever it printed there.
          poll_timeout_ms = 20i32;
          p = p.offset(1);
          (*ptr_to_globals).key_count -= 1
        }
      }
    } else if *bb_errno != 5i32 || {
      (*ptr_to_globals).ioerror_count += 1;
      ((*ptr_to_globals).ioerror_count) > 4i32
    }
    // We sometimes get spurious IO errors on the TTY
    // as programs close and re-open it
    {
      break;
    }
    // Close & re-open tty in case they have
    // swapped virtual consoles
    close(handle);
  }
  cleanup(1i32);
}
