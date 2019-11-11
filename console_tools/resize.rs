

use libc;
use libc::alarm;
use libc::fprintf;
use libc::ioctl;
use libc::printf;
use libc::termios;
use libc::winsize;
use libc::FILE;
extern "C" {
  #[no_mangle]
  fn _exit(_: libc::c_int) -> !;

  #[no_mangle]
  static mut stderr: *mut FILE;

  #[no_mangle]
  fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

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
