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





























use libc::printf;



use libc::sprintf;









extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn ftruncate(__fd: libc::c_int, __length: off64_t) -> libc::c_int;
  #[no_mangle]
  static ptr_to_globals: *mut globals;
  #[no_mangle]
  static mut stdout: *mut FILE;
  #[no_mangle]
  fn setvbuf(
    __stream: *mut FILE,
    __buf: *mut libc::c_char,
    __modes: libc::c_int,
    __n: size_t,
  ) -> libc::c_int;



  #[no_mangle]
  fn mmap(
    __addr: *mut libc::c_void,
    __len: size_t,
    __prot: libc::c_int,
    __flags: libc::c_int,
    __fd: libc::c_int,
    __offset: off64_t,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn bb_signals(sigs: libc::c_int, f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>);
  #[no_mangle]
  fn kill_myself_with_sig(sig: libc::c_int) -> !;
  #[no_mangle]
  static mut bb_got_signal: smallint;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xlseek(fd: libc::c_int, offset: off_t, whence: libc::c_int) -> off_t;
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  static bb_hexdigits_upcase: [libc::c_char; 0];
  /* Non-aborting kind of convertors: bb_strto[u][l]l */
  /* On exit: errno = 0 only if there was non-empty, '\0' terminated value
   * errno = EINVAL if value was not '\0' terminated, but otherwise ok
   *    Return value is still valid, caller should just check whether end[0]
   *    is a valid terminating char for particular case. OTOH, if caller
   *    requires '\0' terminated input, [s]he can just check errno == 0.
   * errno = ERANGE if value had alphanumeric terminating char ("1234abcg").
   * errno = ERANGE if value is out of range, missing, etc.
   * errno = ERANGE if value had minus sign for strtouXX (even "-0" is not ok )
   *    return value is all-ones in this case.
   */
  #[no_mangle]
  fn bb_strtoull(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_ulonglong;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
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
  fn read_key(fd: libc::c_int, buffer: *mut libc::c_char, timeout: libc::c_int) -> int64_t;
  #[no_mangle]
  fn read_line_input(
    st: *mut line_input_t,
    prompt: *const libc::c_char,
    command: *mut libc::c_char,
    maxsize: libc::c_int,
  ) -> libc::c_int;
}

pub type __int64_t = libc::c_long;

use libc::off64_t;

pub type int64_t = __int64_t;


pub type uintptr_t = libc::c_ulong;
use libc::off_t;
use crate::librb::size_t;
use crate::librb::smallint;

use libc::FILE;

use libc::termios;
pub type C2RustUnnamed = libc::c_uint;
pub const BB_FATAL_SIGS: C2RustUnnamed = 117503054;
pub type C2RustUnnamed_0 = libc::c_int;
pub const KEYCODE_BUFFER_SIZE: C2RustUnnamed_0 = 16;
pub const KEYCODE_CURSOR_POS: C2RustUnnamed_0 = -256;
pub const KEYCODE_ALT_D: C2RustUnnamed_0 = -45;
pub const KEYCODE_ALT_BACKSPACE: C2RustUnnamed_0 = -44;
pub const KEYCODE_ALT_LEFT: C2RustUnnamed_0 = -37;
pub const KEYCODE_ALT_RIGHT: C2RustUnnamed_0 = -36;
pub const KEYCODE_CTRL_LEFT: C2RustUnnamed_0 = -69;
pub const KEYCODE_CTRL_RIGHT: C2RustUnnamed_0 = -68;
pub const KEYCODE_D: C2RustUnnamed_0 = -13;
pub const KEYCODE_BACKSPACE: C2RustUnnamed_0 = -12;
pub const KEYCODE_PAGEDOWN: C2RustUnnamed_0 = -11;
pub const KEYCODE_PAGEUP: C2RustUnnamed_0 = -10;
pub const KEYCODE_DELETE: C2RustUnnamed_0 = -9;
pub const KEYCODE_INSERT: C2RustUnnamed_0 = -8;
pub const KEYCODE_END: C2RustUnnamed_0 = -7;
pub const KEYCODE_HOME: C2RustUnnamed_0 = -6;
pub const KEYCODE_LEFT: C2RustUnnamed_0 = -5;
pub const KEYCODE_RIGHT: C2RustUnnamed_0 = -4;
pub const KEYCODE_DOWN: C2RustUnnamed_0 = -3;
pub const KEYCODE_UP: C2RustUnnamed_0 = -2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line_input_t {
  pub flags: libc::c_int,
  pub timeout: libc::c_int,
  pub path_lookup: *const libc::c_char,
  pub cnt_history: libc::c_int,
  pub cur_history: libc::c_int,
  pub max_history: libc::c_int,
  pub cnt_history_in_file: libc::c_uint,
  pub hist_file: *const libc::c_char,
  pub history: [*mut libc::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub half: smallint,
  pub in_read_key: smallint,
  pub fd: libc::c_int,
  pub height: libc::c_uint,
  pub row: libc::c_uint,
  pub pagesize: libc::c_uint,
  pub baseaddr: *mut u8,
  pub current_byte: *mut u8,
  pub eof_byte: *mut u8,
  pub size: off_t,
  pub offset: off_t,
  pub read_key_buffer: [libc::c_char; 16],
  pub orig_termios: termios,
}
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
unsafe extern "C" fn restore_term() {
  tcsetattr_stdin_TCSANOW(&mut (*ptr_to_globals).orig_termios);
  printf(b"\x1b[?1049l\x00" as *const u8 as *const libc::c_char);
  fflush_all();
}
unsafe extern "C" fn sig_catcher(mut sig: libc::c_int) {
  if (*ptr_to_globals).in_read_key == 0 {
    /* now it's not safe to do I/O, just inform the main loop */
    bb_got_signal = sig as smallint;
    return;
  }
  restore_term();
  kill_myself_with_sig(sig);
}
unsafe extern "C" fn format_line(
  mut hex: *mut libc::c_char,
  mut data: *mut u8,
  mut offset: off_t,
) -> libc::c_int {
  let mut ofs_pos: libc::c_int = 0;
  let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut end: *mut u8 = 0 as *mut u8;
  let mut end1: *mut u8 = 0 as *mut u8;
  /* Can be more than 4Gb, thus >8 chars, thus use a variable - don't assume 8! */
  ofs_pos = sprintf(
    hex,
    b"%08lx \x00" as *const u8 as *const libc::c_char,
    offset,
  );
  hex = hex.offset(ofs_pos as isize);
  text = hex.offset((16i32 * 3i32) as isize);
  end1 = data.offset(15);
  if (*ptr_to_globals).size - offset >0{
    end = end1;
    if (*ptr_to_globals).size - offset <= 15i32 as libc::c_long {
      end = data
        .offset(((*ptr_to_globals).size - offset) as isize)
        .offset(-1)
    }
    while data <= end {
      let fresh0 = data;
      data = data.offset(1);
      let mut c: u8 = *fresh0;
      let fresh1 = hex;
      hex = hex.offset(1);
      *fresh1 = *bb_hexdigits_upcase
        .as_ptr()
        .offset((c as libc::c_int >> 4i32) as isize);
      let fresh2 = hex;
      hex = hex.offset(1);
      *fresh2 = *bb_hexdigits_upcase
        .as_ptr()
        .offset((c as libc::c_int & 0xfi32) as isize);
      let fresh3 = hex;
      hex = hex.offset(1);
      *fresh3 = ' ' as i32 as libc::c_char;
      if (c as libc::c_int) < ' ' as i32 || c as libc::c_int > 0x7ei32 {
        c = '.' as i32 as u8
      }
      let fresh4 = text;
      text = text.offset(1);
      *fresh4 = c as libc::c_char
    }
  }
  while data <= end1 {
    let fresh5 = hex;
    hex = hex.offset(1);
    *fresh5 = ' ' as i32 as libc::c_char;
    let fresh6 = hex;
    hex = hex.offset(1);
    *fresh6 = ' ' as i32 as libc::c_char;
    let fresh7 = hex;
    hex = hex.offset(1);
    *fresh7 = ' ' as i32 as libc::c_char;
    let fresh8 = text;
    text = text.offset(1);
    *fresh8 = ' ' as i32 as libc::c_char;
    data = data.offset(1)
  }
  *text = '\u{0}' as i32 as libc::c_char;
  return ofs_pos;
}
unsafe extern "C" fn redraw(mut cursor: libc::c_uint) {
  let mut data: *mut u8 = 0 as *mut u8;
  let mut offset: off_t = 0;
  let mut i: libc::c_uint = 0;
  let mut pos: libc::c_uint = 0;
  printf(b"\x1b[H\x1b[J\x00" as *const u8 as *const libc::c_char);
  /* if cursor is past end of screen, how many lines to move down? */
  i = cursor
    .wrapping_div(16i32 as libc::c_uint)
    .wrapping_sub((*ptr_to_globals).height)
    .wrapping_add(1i32 as libc::c_uint);
  if (i as libc::c_int) < 0i32 {
    i = 0i32 as libc::c_uint
  }
  data = (*ptr_to_globals)
    .baseaddr
    .offset(i.wrapping_mul(16i32 as libc::c_uint) as isize);
  offset = (*ptr_to_globals).offset + i.wrapping_mul(16i32 as libc::c_uint) as libc::c_long;
  cursor = cursor.wrapping_sub(i.wrapping_mul(16i32 as libc::c_uint));
  i = 0i32 as libc::c_uint;
  pos = i;
  while i < (*ptr_to_globals).height {
    let mut buf: [libc::c_char; 88] = [0; 88];
    pos = format_line(buf.as_mut_ptr(), data, offset) as libc::c_uint;
    printf(
      (b"\r\n%s\x00" as *const u8 as *const libc::c_char)
        .offset(((i == 0) as libc::c_int * 2i32) as isize),
      buf.as_mut_ptr(),
    );
    data = data.offset(16);
    offset += 16i32 as libc::c_long;
    i = i.wrapping_add(1)
  }
  (*ptr_to_globals).row = cursor.wrapping_div(16i32 as libc::c_uint);
  printf(
    b"\x1b[%u;%uH\x00" as *const u8 as *const libc::c_char,
    (1i32 as libc::c_uint).wrapping_add((*ptr_to_globals).row),
    (1i32 as libc::c_uint)
      .wrapping_add(pos)
      .wrapping_add((cursor & 0xfi32 as libc::c_uint).wrapping_mul(3i32 as libc::c_uint)),
  );
}
unsafe extern "C" fn redraw_cur_line() {
  let mut buf: [libc::c_char; 88] = [0; 88];
  let mut data: *mut u8 = 0 as *mut u8;
  let mut offset: off_t = 0;
  let mut column: libc::c_int = 0;
  column = (0xfi32 as libc::c_ulong & (*ptr_to_globals).current_byte as uintptr_t) as libc::c_int;
  data = (*ptr_to_globals).current_byte.offset(-(column as isize));
  offset = (*ptr_to_globals).offset
    + data.wrapping_offset_from((*ptr_to_globals).baseaddr) as libc::c_long;
  column = column * 3i32 + (*ptr_to_globals).half as libc::c_int;
  column += format_line(buf.as_mut_ptr(), data, offset);
  printf(
    b"%s\r%.*s\x00" as *const u8 as *const libc::c_char,
    buf.as_mut_ptr().offset(column as isize),
    column,
    buf.as_mut_ptr(),
  );
}
/* if remappers return 0, no change was done */
unsafe extern "C" fn remap(mut cur_pos: libc::c_uint) -> libc::c_int {
  if !(*ptr_to_globals).baseaddr.is_null() {
    munmap(
      (*ptr_to_globals).baseaddr as *mut libc::c_void,
      (64i32 * 1024i32) as size_t,
    );
  }
  (*ptr_to_globals).baseaddr = mmap(
    0 as *mut libc::c_void,
    (64i32 * 1024i32) as size_t,
    0x1i32 | 0x2i32,
    0x1i32,
    (*ptr_to_globals).fd,
    (*ptr_to_globals).offset,
  ) as *mut u8;
  if (*ptr_to_globals).baseaddr == -1i32 as *mut libc::c_void as *mut u8 {
    restore_term();
    bb_simple_perror_msg_and_die(b"mmap\x00" as *const u8 as *const libc::c_char);
  }
  (*ptr_to_globals).current_byte = (*ptr_to_globals).baseaddr.offset(cur_pos as isize);
  (*ptr_to_globals).eof_byte = (*ptr_to_globals)
    .baseaddr
    .offset((64i32 * 1024i32) as isize);
  if (*ptr_to_globals).size - (*ptr_to_globals).offset < (64i32 * 1024i32) as libc::c_long {
    /* mapping covers tail of the file */
    /* we do have a mapped byte which is past eof */
    (*ptr_to_globals).eof_byte = (*ptr_to_globals)
      .baseaddr
      .offset(((*ptr_to_globals).size - (*ptr_to_globals).offset) as isize)
  } /* can't move mapping even further, it's at the end already */
  return 1i32; /* constant on most arches */
}
unsafe extern "C" fn move_mapping_further() -> libc::c_int {
  let mut pos: libc::c_uint = 0;
  let mut pagesize: libc::c_uint = 0;
  if (*ptr_to_globals).size - (*ptr_to_globals).offset < (64i32 * 1024i32) as libc::c_long {
    return 0i32;
  }
  pagesize = 4096i32 as libc::c_uint;
  pos = (*ptr_to_globals)
    .current_byte
    .wrapping_offset_from((*ptr_to_globals).baseaddr) as libc::c_long as libc::c_uint;
  if pos >= pagesize {
    loop
    /* move offset up until current position is in 1st page */
    {
      (*ptr_to_globals).offset += pagesize as libc::c_long;
      if (*ptr_to_globals).offset ==0{
        /* whoops */
        (*ptr_to_globals).offset -= pagesize as libc::c_long; /* we are at 0 already */
        break; /* constant on most arches */
      } else {
        pos = pos.wrapping_sub(pagesize);
        if !(pos >= pagesize) {
          break;
        }
      }
    }
    return remap(pos);
  }
  return 0i32;
}
unsafe extern "C" fn move_mapping_lower() -> libc::c_int {
  let mut pos: libc::c_uint = 0;
  let mut pagesize: libc::c_uint = 0;
  if (*ptr_to_globals).offset ==0{
    return 0i32;
  }
  pagesize = 4096i32 as libc::c_uint;
  pos = (*ptr_to_globals)
    .current_byte
    .wrapping_offset_from((*ptr_to_globals).baseaddr) as libc::c_long as libc::c_uint;
  /* move offset down until current position is in last page */
  pos = pos.wrapping_add(pagesize);
  while pos < (64i32 * 1024i32) as libc::c_uint {
    pos = pos.wrapping_add(pagesize);
    (*ptr_to_globals).offset -= pagesize as libc::c_long;
    if (*ptr_to_globals).offset ==0{
      break;
    }
  }
  pos = pos.wrapping_sub(pagesize);
  return remap(pos);
}
//usage:#define hexedit_trivial_usage
//usage:	"FILE"
//usage:#define hexedit_full_usage "\n\n"
//usage:	"Edit FILE in hexadecimal"
#[no_mangle]
pub unsafe extern "C" fn hexedit_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let ref mut fresh9 = *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
    as *mut *mut globals);
  *fresh9 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  get_terminal_width_height(-1i32, 0 as *mut libc::c_uint, &mut (*ptr_to_globals).height);
  /* reduce number of write() syscalls while PgUp/Down: fully buffered output */
  let mut sz: libc::c_uint = ((*ptr_to_globals).height | 0xfi32 as libc::c_uint)
    .wrapping_mul((8i32 + 1i32 + 3i32 * 16i32 + 16i32 + 1i32 + 1i32 + 13i32) as libc::c_uint);
  setvbuf(
    stdout,
    xmalloc(sz as size_t) as *mut libc::c_char,
    0i32,
    sz as size_t,
  );
  getopt32(argv, b"^\x00=1\x00" as *const u8 as *const libc::c_char);
  argv = argv.offset(optind as isize);
  (*ptr_to_globals).fd = xopen(*argv, 0o2i32);
  (*ptr_to_globals).size = xlseek((*ptr_to_globals).fd, 0i32 as off_t, 2i32);
  /* TERMIOS_RAW_CRNL suppresses \n -> \r\n translation, helps with down-arrow */
  printf(b"\x1b[?1049h\x00" as *const u8 as *const libc::c_char);
  set_termios_to_raw(
    0i32,
    &mut (*ptr_to_globals).orig_termios,
    1i32 << 1i32 | 1i32 << 2i32,
  );
  bb_signals(
    BB_FATAL_SIGS as libc::c_int,
    Some(sig_catcher as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  remap(0i32 as libc::c_uint);
  redraw(0i32 as libc::c_uint);
  loop
  //TODO: //Home/End: start/end of line; '<'/'>': start/end of file
  //Backspace: undo
  //Ctrl-L: redraw
  //Ctrl-Z: suspend
  //'/', Ctrl-S: search
  //TODO: detect window resize
  {
    let mut cnt: libc::c_uint = 0; /* for (;;) */
    /* switch */
    let mut key: i32 = 0; /* for compiler */
    key = key; /* convert A-Z to a-z */
    let mut byte: u8 = 0;
    fflush_all();
    (*ptr_to_globals).in_read_key = 1i32 as smallint;
    if bb_got_signal == 0 {
      key = read_key(0i32, (*ptr_to_globals).read_key_buffer.as_mut_ptr(), -1i32) as i32
    }
    (*ptr_to_globals).in_read_key = 0i32 as smallint;
    if bb_got_signal != 0 {
      key = 'X' as i32 & !0x60i32 as u8 as libc::c_int
    }
    cnt = 1i32 as libc::c_uint;
    if (key - 'A' as i32) as libc::c_uint <= ('Z' as i32 - 'A' as i32) as libc::c_uint {
      key |= 0x20i32
    }
    's_653: {
      let mut current_block_97: u64;
      match key {
        97 | 98 | 99 | 100 | 101 | 102 => {
          /* convert to '0'+10...15 */
          key = key - ('a' as i32 - '0' as i32 - 10i32);
          current_block_97 = 4691753191039046465;
        }
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
          current_block_97 = 4691753191039046465;
        }
        -4 => {
          current_block_97 = 8845539310339867835;
        }
        -11 => {
          cnt = (*ptr_to_globals).height;
          current_block_97 = 10365604997048951659;
        }
        -3 => {
          current_block_97 = 10365604997048951659;
        }
        -5 => {
          if (*ptr_to_globals).half != 0 {
            (*ptr_to_globals).half = 0i32 as smallint;
            printf(b"\x1b[D\x00" as *const u8 as *const libc::c_char);
            current_block_97 = 1966075811433896587;
          } else if 0xfi32 as libc::c_ulong & (*ptr_to_globals).current_byte as uintptr_t
            == 0i32 as libc::c_ulong
          {
            /* leftmost pos, wrap to prev line */
            if (*ptr_to_globals).current_byte == (*ptr_to_globals).baseaddr {
              if move_mapping_lower() == 0 {
                current_block_97 = 1966075811433896587;
              } else {
                current_block_97 = 12070711452894729854;
              }
            /* first line, don't do anything */
            } else {
              current_block_97 = 12070711452894729854; /* cursor right 3*15 + 1 chars */
            }
            match current_block_97 {
              1966075811433896587 => {}
              _ => {
                (*ptr_to_globals).half = 1i32 as smallint;
                (*ptr_to_globals).current_byte = (*ptr_to_globals).current_byte.offset(-1);
                printf(b"\x1b[46C\x00" as *const u8 as *const libc::c_char);
                current_block_97 = 17726534489295324900;
              }
            }
          } else {
            (*ptr_to_globals).half = 1i32 as smallint;
            (*ptr_to_globals).current_byte = (*ptr_to_globals).current_byte.offset(-1);
            printf(b"\x1b[2D\x00" as *const u8 as *const libc::c_char);
            current_block_97 = 1966075811433896587;
          }
        }
        -10 => {
          cnt = (*ptr_to_globals).height;
          current_block_97 = 1407665506148312647;
        }
        -2 => {
          current_block_97 = 1407665506148312647;
        }
        10 | 13 => {
          /* [Enter]: goto specified position */
          let mut buf: [libc::c_char; 28] = [0; 28];
          /* ^C/EOF/error: fall through to exiting */
          printf(b"\x1b[999;1H\x1b[K\x00" as *const u8 as *const libc::c_char); /* go to last line */
          if read_line_input(
            0 as *mut line_input_t,
            b"Go to (dec,0Xhex,0oct): \x00" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong as libc::c_int,
          ) > 0i32
          {
            let mut t: off_t = 0;
            let mut cursor: libc::c_uint = 0;
            t = bb_strtoull(buf.as_mut_ptr(), 0 as *mut *mut libc::c_char, 0i32) as off_t;
            if t >= (*ptr_to_globals).size {
              t = (*ptr_to_globals).size -1            }
            cursor = (t & (4096i32 - 1i32) as libc::c_long) as libc::c_uint;
            t -= cursor as libc::c_long;
            if t < 0 {
              t = 0i32 as off_t;
              cursor = t as libc::c_uint
            }
            if t !=0&& cursor < 0x1ffi32 as libc::c_uint {
              /* very close to end of page, possibly to EOF */
              /* move one page lower */
              t -= 4096i32 as libc::c_long;
              cursor = cursor.wrapping_add(4096i32 as libc::c_uint)
            }
            (*ptr_to_globals).offset = t;
            remap(cursor);
            redraw(cursor);
            current_block_97 = 1966075811433896587;
          } else {
            current_block_97 = 557539052108834644;
          }
        }
        24 => {
          current_block_97 = 557539052108834644;
        }
        _ => {
          current_block_97 = 1966075811433896587;
        }
      }
      match current_block_97 {
        557539052108834644 => {
          restore_term();
          return 0i32;
        }
        4691753191039046465 =>
        /* fall through */
        {
          if (*ptr_to_globals).current_byte == (*ptr_to_globals).eof_byte {
            if move_mapping_further() == 0 {
              /* already at EOF; extend the file */
              (*ptr_to_globals).size += 1;
              if (*ptr_to_globals).size <= 0                 || ftruncate((*ptr_to_globals).fd, (*ptr_to_globals).size) != 0i32
              {
                /* error extending? (e.g. block dev) */
                (*ptr_to_globals).size -= 1;
                current_block_97 = 1966075811433896587;
              } else {
                (*ptr_to_globals).eof_byte = (*ptr_to_globals).eof_byte.offset(1);
                current_block_97 = 1356832168064818221;
              }
            } else {
              current_block_97 = 1356832168064818221;
            }
          } else {
            current_block_97 = 1356832168064818221;
          }
          match current_block_97 {
            1966075811433896587 => {}
            _ => {
              key -= '0' as i32;
              byte = (*(*ptr_to_globals).current_byte as libc::c_int & 0xf0i32) as u8;
              if (*ptr_to_globals).half == 0 {
                byte = (*(*ptr_to_globals).current_byte as libc::c_int & 0xfi32) as u8;
                key <<= 4i32
              }
              *(*ptr_to_globals).current_byte = (byte as libc::c_int + key) as u8;
              /* can't just print one updated hex char: need to update right-hand ASCII too */
              redraw_cur_line();
              current_block_97 = 8845539310339867835;
            }
          }
        }
        _ => {}
      }
      match current_block_97 {
        8845539310339867835 =>
        /* fall through */
        {
          if (*ptr_to_globals).current_byte == (*ptr_to_globals).eof_byte {
            current_block_97 = 1966075811433896587; /* eof - don't allow going past it */
          } else {
            byte = *(*ptr_to_globals).current_byte;
            if (*ptr_to_globals).half == 0 {
              (*ptr_to_globals).half = 1i32 as smallint;
              putchar_unlocked(
                *bb_hexdigits_upcase
                  .as_ptr()
                  .offset((byte as libc::c_int >> 4i32) as isize) as libc::c_int,
              );
              current_block_97 = 1966075811433896587;
            } else {
              (*ptr_to_globals).half = 0i32 as smallint;
              (*ptr_to_globals).current_byte = (*ptr_to_globals).current_byte.offset(1);
              if 0xfi32 as libc::c_ulong & (*ptr_to_globals).current_byte as uintptr_t
                == 0i32 as libc::c_ulong
              {
                /* rightmost pos, wrap to next line */
                if (*ptr_to_globals).current_byte == (*ptr_to_globals).eof_byte {
                  move_mapping_further(); /* cursor left 3*15 + 1 chars */
                }
                printf(b"\x1b[46D\x00" as *const u8 as *const libc::c_char);
                current_block_97 = 3989310813288763114;
              } else {
                putchar_unlocked(
                  *bb_hexdigits_upcase
                    .as_ptr()
                    .offset((byte as libc::c_int & 0xfi32) as isize)
                    as libc::c_int,
                );
                putchar_unlocked(' ' as i32);
                current_block_97 = 1966075811433896587;
              }
            }
          }
        }
        _ => {}
      }
      loop {
        match current_block_97 {
          1966075811433896587 => {
            break 's_653;
          }
          1407665506148312647 => {
            if ((*ptr_to_globals)
              .current_byte
              .wrapping_offset_from((*ptr_to_globals).baseaddr) as libc::c_long)
              < 16i32 as libc::c_long
            {
              if move_mapping_lower() == 0 {
                current_block_97 = 1966075811433896587;
                continue;
                /* already at 0, stop */
              }
            } /* down one line, possibly scroll screen */
            (*ptr_to_globals).current_byte = (*ptr_to_globals).current_byte.offset(-16);
            current_block_97 = 17726534489295324900;
          }
          3989310813288763114 => {
            putchar_unlocked('\n' as i32);
            (*ptr_to_globals).row = (*ptr_to_globals).row.wrapping_add(1);
            if (*ptr_to_globals).row >= (*ptr_to_globals).height {
              (*ptr_to_globals).row = (*ptr_to_globals).row.wrapping_sub(1);
              redraw_cur_line();
            }
            cnt = cnt.wrapping_sub(1);
            if cnt != 0 {
              current_block_97 = 10365604997048951659;
            } else {
              current_block_97 = 1966075811433896587;
            }
          }
          10365604997048951659 => {
            (*ptr_to_globals).current_byte = (*ptr_to_globals).current_byte.offset(16);
            if !((*ptr_to_globals).current_byte >= (*ptr_to_globals).eof_byte) {
              current_block_97 = 3989310813288763114;
              continue;
            }
            move_mapping_further();
            if !((*ptr_to_globals).current_byte > (*ptr_to_globals).eof_byte) {
              current_block_97 = 3989310813288763114;
              continue;
            }
            /* _after_ eof - don't allow this */
            (*ptr_to_globals).current_byte = (*ptr_to_globals).current_byte.offset(-16);
            if (*ptr_to_globals).current_byte < (*ptr_to_globals).baseaddr {
              move_mapping_lower();
            }
            current_block_97 = 1966075811433896587;
          }
          _ => {
            if (*ptr_to_globals).row != 0i32 as libc::c_uint {
              (*ptr_to_globals).row = (*ptr_to_globals).row.wrapping_sub(1);
              printf(b"\x1b[A\x00" as *const u8 as *const libc::c_char);
            /* up (won't scroll) */
            } else {
              //printf(ESC"[T"); /* scroll up */ - not implemented on Linux VT!
              printf(b"\x1bM\x00" as *const u8 as *const libc::c_char); /* scroll up */
              redraw_cur_line();
            }
            cnt = cnt.wrapping_sub(1);
            if cnt != 0 {
              current_block_97 = 1407665506148312647;
            } else {
              current_block_97 = 1966075811433896587;
            }
          }
        }
      }
    }
  }
}
