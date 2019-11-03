use libc;
extern "C" {
  #[no_mangle]
  fn _exit(_: libc::c_int) -> !;
  #[no_mangle]
  static mut stderr: *mut _IO_FILE;
  #[no_mangle]
  fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
  #[no_mangle]
  fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
  #[no_mangle]
  fn tcsetattr(
    __fd: libc::c_int,
    __optional_actions: libc::c_int,
    __termios_p: *const termios,
  ) -> libc::c_int;
  #[no_mangle]
  fn bb_signals(sigs: libc::c_int, f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>);
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
  pub _flags: libc::c_int,
  pub _IO_read_ptr: *mut libc::c_char,
  pub _IO_read_end: *mut libc::c_char,
  pub _IO_read_base: *mut libc::c_char,
  pub _IO_write_base: *mut libc::c_char,
  pub _IO_write_ptr: *mut libc::c_char,
  pub _IO_write_end: *mut libc::c_char,
  pub _IO_buf_base: *mut libc::c_char,
  pub _IO_buf_end: *mut libc::c_char,
  pub _IO_save_base: *mut libc::c_char,
  pub _IO_backup_base: *mut libc::c_char,
  pub _IO_save_end: *mut libc::c_char,
  pub _markers: *mut _IO_marker,
  pub _chain: *mut _IO_FILE,
  pub _fileno: libc::c_int,
  pub _flags2: libc::c_int,
  pub _old_offset: __off_t,
  pub _cur_column: libc::c_ushort,
  pub _vtable_offset: libc::c_schar,
  pub _shortbuf: [libc::c_char; 1],
  pub _lock: *mut libc::c_void,
  pub _offset: __off64_t,
  pub __pad1: *mut libc::c_void,
  pub __pad2: *mut libc::c_void,
  pub __pad3: *mut libc::c_void,
  pub __pad4: *mut libc::c_void,
  pub __pad5: size_t,
  pub _mode: libc::c_int,
  pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
  pub _next: *mut _IO_marker,
  pub _sbuf: *mut _IO_FILE,
  pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
  pub ws_row: libc::c_ushort,
  pub ws_col: libc::c_ushort,
  pub ws_xpixel: libc::c_ushort,
  pub ws_ypixel: libc::c_ushort,
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
unsafe extern "C" fn onintr(mut _sig: libc::c_int) {
  tcsetattr(2i32, 0i32, bb_common_bufsiz1.as_mut_ptr() as *mut termios);
  _exit(1i32);
}
#[no_mangle]
pub unsafe extern "C" fn resize_main(
  mut _argc: libc::c_int,
  mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut new: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
  };
  let mut w: winsize = {
    let mut init = winsize {
      ws_row: 0i32 as libc::c_ushort,
      ws_col: 0i32 as libc::c_ushort,
      ws_xpixel: 0i32 as libc::c_ushort,
      ws_ypixel: 0i32 as libc::c_ushort,
    };
    init
  };
  let mut ret: libc::c_int = 0;
  /* We use _stderr_ in order to make resize usable
   * in shell backticks (those redirect stdout away from tty).
   * NB: other versions of resize open "/dev/tty"
   * and operate on it - should we do the same?
   */
  tcgetattr(2i32, bb_common_bufsiz1.as_mut_ptr() as *mut termios); /* fiddle echo */
  //TODO: die if the above fails?
  memcpy(
    &mut new as *mut termios as *mut libc::c_void,
    bb_common_bufsiz1.as_mut_ptr() as *mut termios as *const libc::c_void,
    ::std::mem::size_of::<termios>() as libc::c_ulong,
  );
  new.c_cflag |= (0o4000i32 | 0o200i32) as libc::c_uint;
  new.c_lflag &= !(0o2i32 | 0o10i32 | 0o20i32 | 0o1i32) as libc::c_uint;
  bb_signals(
    0i32 + (1i32 << 2i32) + (1i32 << 3i32) + (1i32 << 15i32) + (1i32 << 14i32),
    Some(onintr as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  tcsetattr(2i32, 0i32, &mut new);
  /* save_cursor_pos 7
   * scroll_whole_screen [r
   * put_cursor_waaaay_off [$x;$yH
   * get_cursor_pos [6n
   * restore_cursor_pos 8
   */
  fprintf(
    stderr,
    b"\x1b7\x1b[r\x1b[999;999H\x1b[6n\x00" as *const u8 as *const libc::c_char,
  ); /* Just in case terminal won't answer */
  alarm(3i32 as libc::c_uint);
  //BUG: death by signal won't restore termios
  scanf(
    b"\x1b[%hu;%huR\x00" as *const u8 as *const libc::c_char,
    &mut w.ws_row as *mut libc::c_ushort,
    &mut w.ws_col as *mut libc::c_ushort,
  );
  fprintf(stderr, b"\x1b8\x00" as *const u8 as *const libc::c_char);
  /* BTW, other versions of resize recalculate w.ws_xpixel, ws.ws_ypixel
   * by calculating character cell HxW from old values
   * (gotten via TIOCGWINSZ) and recomputing *pixel values */
  ret = ioctl(2i32, 0x5414i32 as libc::c_ulong, &mut w as *mut winsize);
  tcsetattr(2i32, 0i32, bb_common_bufsiz1.as_mut_ptr() as *mut termios);
  printf(
    b"COLUMNS=%d;LINES=%d;export COLUMNS LINES;\n\x00" as *const u8 as *const libc::c_char,
    w.ws_col as libc::c_int,
    w.ws_row as libc::c_int,
  );
  return ret;
}
