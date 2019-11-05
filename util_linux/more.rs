use libc;
extern "C" {
  #[no_mangle]
  fn _exit(_: libc::c_int) -> !;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut stdin: *mut FILE;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn ftello(__stream: *mut FILE) -> __off64_t;
  #[no_mangle]
  fn fileno_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn tcsetattr(
    __fd: libc::c_int,
    __optional_actions: libc::c_int,
    __termios_p: *const termios,
  ) -> libc::c_int;
  #[no_mangle]
  fn bb_signals(sigs: libc::c_int, f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>);
  #[no_mangle]
  fn bb_putchar_stderr(ch: libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn die_if_ferror_stdout();
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn fopen_or_warn(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_cat(argv: *mut *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn get_terminal_width_height(
    fd: libc::c_int,
    width: *mut libc::c_uint,
    height: *mut libc::c_uint,
  ) -> libc::c_int;
  #[no_mangle]
  fn set_termios_to_raw(fd: libc::c_int, oldterm: *mut termios, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn isatty(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

use crate::librb::__off64_t;
use crate::librb::__off_t;

use crate::librb::stat;
use crate::librb::timespec;
use crate::librb::uint32_t;

use crate::librb::termios;
use crate::librb::uoff_t;
use crate::librb::FILE;
pub type C2RustUnnamed = libc::c_uint;
pub const BB_FATAL_SIGS: C2RustUnnamed = 117503054;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub tty_fileno: libc::c_int,
  pub terminal_width: libc::c_uint,
  pub terminal_height: libc::c_uint,
  pub initial_settings: termios,
}
#[inline(always)]
unsafe extern "C" fn bb_ascii_tolower(mut a: libc::c_uchar) -> libc::c_uchar {
  let mut b: libc::c_uchar = (a as libc::c_int - 'A' as i32) as libc::c_uchar;
  if b as libc::c_int <= 'Z' as i32 - 'A' as i32 {
    a = (a as libc::c_int + ('a' as i32 - 'A' as i32)) as libc::c_uchar
  }
  return a;
}
unsafe extern "C" fn get_wh() {
  /* never returns w, h <= 1 */
  get_terminal_width_height(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tty_fileno,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).terminal_width,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).terminal_height,
  );
  let ref mut fresh0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).terminal_height;
  *fresh0 = (*fresh0).wrapping_sub(1i32 as libc::c_uint);
}
unsafe extern "C" fn tcsetattr_tty_TCSANOW(mut settings: *mut termios) {
  tcsetattr(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tty_fileno,
    0i32,
    settings,
  );
}
unsafe extern "C" fn gotsig(mut _sig: libc::c_int) {
  /* bb_putchar_stderr doesn't use stdio buffering,
   * therefore it is safe in signal handler */
  bb_putchar_stderr('\n' as i32 as libc::c_char); /* for compiler */
  tcsetattr_tty_TCSANOW(&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).initial_settings);
  _exit(1i32);
}
#[no_mangle]
pub unsafe extern "C" fn more_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut c: libc::c_int = 0;
  c = c;
  let mut input: libc::c_int = 0i32;
  let mut spaces: libc::c_int = 0i32;
  let mut please_display_more_prompt: libc::c_int = 0;
  let mut tty: *mut FILE = 0 as *mut FILE;
  /* Parse options */
  /* Accepted but ignored: */
  /* -d	Display help instead of ringing bell */
  /* -f	Count logical lines (IOW: long lines are not folded) */
  /* -l	Do not pause after any line containing a ^L (form feed) */
  /* -s	Squeeze blank lines into one */
  /* -u	Suppress underlining */
  getopt32(argv, b"dflsu\x00" as *const u8 as *const libc::c_char);
  argv = argv.offset(optind as isize);
  /* Another popular pager, most, detects when stdout
   * is not a tty and turns into cat. This makes sense. */
  if isatty(1i32) == 0 {
    return bb_cat(argv);
  }
  tty = fopen_for_read(b"/dev/tty\x00" as *const u8 as *const libc::c_char);
  if tty.is_null() {
    return bb_cat(argv);
  }
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tty_fileno = fileno_unlocked(tty);
  /* Turn on unbuffered input; turn off echoing */
  set_termios_to_raw(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).tty_fileno,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).initial_settings,
    0i32,
  );
  bb_signals(
    BB_FATAL_SIGS as libc::c_int,
    Some(gotsig as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  's_75: loop {
    let mut st: stat = stat {
      st_dev: 0,
      st_ino: 0,
      st_nlink: 0,
      st_mode: 0,
      st_uid: 0,
      st_gid: 0,
      __pad0: 0,
      st_rdev: 0,
      st_size: 0,
      st_blksize: 0,
      st_blocks: 0,
      st_atim: timespec {
        tv_sec: 0,
        tv_nsec: 0,
      },
      st_mtim: timespec {
        tv_sec: 0,
        tv_nsec: 0,
      },
      st_ctim: timespec {
        tv_sec: 0,
        tv_nsec: 0,
      },
      __glibc_reserved: [0; 3],
    };
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut len: libc::c_int = 0;
    let mut lines: libc::c_int = 0;
    file = stdin;
    if !(*argv).is_null() {
      file = fopen_or_warn(*argv, b"r\x00" as *const u8 as *const libc::c_char);
      if file.is_null() {
        current_block = 12349973810996921269;
      } else {
        current_block = 15089075282327824602;
      }
    } else {
      current_block = 15089075282327824602;
    }
    match current_block {
      15089075282327824602 => {
        st.st_size = 0i32 as __off_t;
        fstat(fileno_unlocked(file), &mut st);
        get_wh();
        please_display_more_prompt = 0i32;
        len = 0i32;
        lines = 0i32;
        loop {
          let mut wrap: libc::c_int = 0;
          if spaces != 0 {
            spaces -= 1
          } else {
            c = getc_unlocked(file);
            if c == -1i32 {
              break;
            }
          }
          loop
          /* if tty was destroyed (closed xterm, etc) */
          /* Then outputting this will also put a character on
           * the beginning of that new line. Thus we first want to
           * display the prompt (if any), so we skip the putchar()
           * and go back to the top of the loop, without reading
           * a new character. */
          {
            if input != 'r' as i32 && please_display_more_prompt != 0 {
              len = printf(b"--More-- \x00" as *const u8 as *const libc::c_char);
              if st.st_size != 0i32 as libc::c_long {
                let mut d: uoff_t = (st.st_size as uoff_t).wrapping_div(100i32 as libc::c_ulong);
                if d == 0i32 as libc::c_ulong {
                  d = 1i32 as uoff_t
                }
                len += printf(
                  b"(%u%% of %lu bytes)\x00" as *const u8 as *const libc::c_char,
                  (ftello(file) as uoff_t).wrapping_div(d) as libc::c_int,
                  st.st_size,
                )
              }
              loop
              /*
               * We've just displayed the "--More--" prompt, so now we need
               * to get input from the user.
               */
              {
                fflush_all();
                input = getc_unlocked(tty);
                input = bb_ascii_tolower(input as libc::c_uchar) as libc::c_int;
                /* Erase the last message */
                printf(
                  b"\r%*s\r\x00" as *const u8 as *const libc::c_char,
                  len,
                  b"\x00" as *const u8 as *const libc::c_char,
                );
                if input == 'q' as i32 {
                  break 's_75;
                }
                /* Due to various multibyte escape
                 * sequences, it's not ok to accept
                 * any input as a command to scroll
                 * the screen. We only allow known
                 * commands, else we show help msg. */
                if input == ' ' as i32 || input == '\n' as i32 || input == 'r' as i32 {
                  break;
                }
                len = printf(
                  b"(Enter:next line Space:next page Q:quit R:show the rest)\x00" as *const u8
                    as *const libc::c_char,
                )
              }
              len = 0i32;
              lines = 0i32;
              please_display_more_prompt = 0i32;
              /* The user may have resized the terminal.
               * Re-read the dimensions. */
              get_wh();
            }
            /* Crudely convert tabs into spaces, which are
             * a bajillion times easier to deal with. */
            if c == '\t' as i32 {
              spaces = (!len as libc::c_uint).wrapping_rem(8i32 as libc::c_uint) as libc::c_int;
              c = ' ' as i32
            }
            /*
             * There are two input streams to worry about here:
             *
             * c    : the character we are reading from the file being "mored"
             * input: a character received from the keyboard
             *
             * If we hit a newline in the _file_ stream, we want to test and
             * see if any characters have been hit in the _input_ stream. This
             * allows the user to quit while in the middle of a file.
             */
            len += 1;
            wrap = (len as libc::c_uint
              > (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).terminal_width)
              as libc::c_int;
            if c == '\n' as i32 || wrap != 0 {
              /* Then outputting this character
               * will move us to a new line. */
              lines += 1;
              if lines as libc::c_uint
                >= (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).terminal_height
                || input == '\n' as i32
              {
                please_display_more_prompt = 1i32
              }
              len = 0i32
            }
            if !(c != '\n' as i32 && wrap != 0) {
              break;
            }
          }
          /* My small mind cannot fathom backspaces and UTF-8 */
          putchar_unlocked(c);
          die_if_ferror_stdout();
        }
        fclose(file);
        fflush_all();
      }
      _ => {}
    }
    if !(!(*argv).is_null() && {
      argv = argv.offset(1);
      !(*argv).is_null()
    }) {
      break;
    }
  }
  tcsetattr_tty_TCSANOW(&mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).initial_settings);
  return 0i32;
}
