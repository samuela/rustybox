use crate::libbb::print_numbered_lines::number_state;
use crate::librb::size_t;
use crate::librb::smallint;
use libc;
use libc::close;
use libc::printf;
use libc::ssize_t;
use libc::FILE;
extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut stdout: *mut FILE;

  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;

  /* Applets which are useful from another applets */

  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}

pub type C2RustUnnamed = libc::c_uint;
pub const COMMON_BUFSIZE: C2RustUnnamed = 1024;

unsafe extern "C" fn catv(mut opts: libc::c_uint, mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut retval: libc::c_int = 0;
  let mut fd: libc::c_int = 0;
  let mut eol_seen: bool = opts & ((1 << 4) * 1 | (1 << 5) * 1) as libc::c_uint != 0;
  let mut eol_char: libc::c_uint = if eol_seen as libc::c_int != 0 {
    '\n' as i32
  } else {
    0x100
  } as libc::c_uint;
  let mut skip_num_on: libc::c_uint = if opts & ((1 << 5) * 1) as libc::c_uint != 0 {
    '\n' as i32
  } else {
    0x100
  } as libc::c_uint;
  let mut lineno: libc::c_uint = 0 as libc::c_uint;
  /* These consts match, we can just pass "opts" to visible() */
  loop {
    fd = crate::libbb::wfopen_input::open_or_warn_stdin(*argv);
    if fd < 0 {
      retval = 1
    } else {
      loop {
        let mut i: libc::c_int = 0;
        let mut res: libc::c_int = 0;
        res = read(
          fd,
          bb_common_bufsiz1.as_mut_ptr() as *mut libc::c_void,
          COMMON_BUFSIZE as libc::c_int as size_t,
        ) as libc::c_int;
        if res < 0 {
          retval = 1
        }
        if res <= 0 {
          break;
        }
        i = 0;
        while i < res {
          let mut c: libc::c_uchar =
            *bb_common_bufsiz1.as_mut_ptr().offset(i as isize) as libc::c_uchar;
          let mut buf: [libc::c_char; 5] = [0; 5];
          if eol_seen as libc::c_int != 0 && c as libc::c_uint != skip_num_on {
            lineno = lineno.wrapping_add(1);
            printf(b"%6u  \x00" as *const u8 as *const libc::c_char, lineno);
          }
          eol_seen = c as libc::c_uint == eol_char;
          crate::libbb::printable::visible(
            c as libc::c_uint,
            buf.as_mut_ptr(),
            opts as libc::c_int,
          );
          fputs_unlocked(buf.as_mut_ptr(), stdout);
          i += 1
        }
      }
      if false && fd != 0 {
        close(fd);
      }
    }
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  crate::libbb::fflush_stdout_and_exit::fflush_stdout_and_exit(retval);
}
#[no_mangle]
pub unsafe extern "C" fn cat_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opts: libc::c_uint = 0;
  opts = crate::libbb::getopt32::getopt32(
    argv,
    b"^etvAnbu\x00Aetv\x00" as *const u8 as *const libc::c_char,
  );
  argv = argv.offset(optind as isize);
  /* Read from stdin if there's nothing else to do. */
  if (*argv.offset(0)).is_null() {
    argv = argv.offset(-1);
    *argv = b"-\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
  }
  if opts & 7 as libc::c_uint != 0 {
    return catv(opts, argv);
  }
  opts >>= 4;
  if opts & (1 << 0 | 1 << 1) as libc::c_uint != 0 {
    /* -n or -b */
    let mut ns: number_state = number_state {
      width: 0,
      start: 0,
      inc: 0,
      sep: 0 as *const libc::c_char,
      empty_str: 0 as *const libc::c_char,
      all: 0,
      nonempty: 0,
    }; /* -n without -b */
    let mut exitcode: libc::c_int = 0; /* -b (with or without -n) */
    ns.width = 6 as libc::c_uint;
    ns.start = 1 as libc::c_uint;
    ns.inc = 1 as libc::c_uint;
    ns.sep = b"\t\x00" as *const u8 as *const libc::c_char;
    ns.empty_str = b"\n\x00" as *const u8 as *const libc::c_char;
    ns.all = (opts & (1 << 1) as libc::c_uint == 0) as libc::c_int as smallint;
    ns.nonempty = (opts & (1 << 1) as libc::c_uint) as smallint;
    exitcode = 0;
    loop {
      exitcode |= crate::libbb::print_numbered_lines::print_numbered_lines(&mut ns, *argv);
      argv = argv.offset(1);
      if (*argv).is_null() {
        break;
      }
    }
    crate::libbb::fflush_stdout_and_exit::fflush_stdout_and_exit(exitcode);
  }
  /*opts >>= 2;*/
  return crate::libbb::bb_cat::bb_cat(argv);
}
