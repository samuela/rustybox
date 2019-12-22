use crate::libbb::xfuncs_printf::xmalloc;
use libc;
use libc::printf;
use libc::strchr;
use libc::FILE;
extern "C" {
  #[no_mangle]
  fn rand() -> libc::c_int;
  #[no_mangle]
  fn srand(__seed: libc::c_uint);
  #[no_mangle]
  static mut optind: libc::c_int;

}

pub type uintptr_t = libc::c_ulong;
/*
 * Use the Fisher-Yates shuffle algorithm on an array of lines.
 */
unsafe extern "C" fn shuffle_lines(mut lines: *mut *mut libc::c_char, mut numlines: libc::c_uint) {
  let mut i: libc::c_uint = 0;
  let mut r: libc::c_uint = 0;
  let mut tmp: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  srand(crate::libbb::time::monotonic_us() as libc::c_uint);
  i = numlines.wrapping_sub(1i32 as libc::c_uint);
  while i > 0i32 as libc::c_uint {
    r = rand() as libc::c_uint;
    /* RAND_MAX can be as small as 32767 */
    if i > 2147483647i32 as libc::c_uint {
      r ^= (rand() << 15i32) as libc::c_uint
    }
    r = r.wrapping_rem(i.wrapping_add(1i32 as libc::c_uint));
    tmp = *lines.offset(i as isize);
    let ref mut fresh0 = *lines.offset(i as isize);
    *fresh0 = *lines.offset(r as isize);
    let ref mut fresh1 = *lines.offset(r as isize);
    *fresh1 = tmp;
    i = i.wrapping_sub(1)
  }
}
#[no_mangle]
pub unsafe extern "C" fn shuf_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opts: libc::c_uint = 0;
  let mut opt_i_str: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut opt_n_str: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut opt_o_str: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut i: libc::c_uint = 0;
  let mut lines: *mut *mut libc::c_char = std::ptr::null_mut();
  let mut numlines: libc::c_uint = 0;
  let mut eol: libc::c_char = 0;
  opts = crate::libbb::getopt32::getopt32(
    argv,
    b"^ei:n:o:z\x00e--i:i--e\x00" as *const u8 as *const libc::c_char,
    &mut opt_i_str as *mut *mut libc::c_char,
    &mut opt_n_str as *mut *mut libc::c_char,
    &mut opt_o_str as *mut *mut libc::c_char,
  );
  argc -= optind;
  argv = argv.offset(optind as isize);
  /* Prepare lines for shuffling - either: */
  if opts & (1i32 << 0i32) as libc::c_uint != 0 {
    /* make lines from command-line arguments */
    numlines = argc as libc::c_uint;
    lines = argv
  } else if opts & (1i32 << 1i32) as libc::c_uint != 0 {
    /* create a range of numbers */
    let mut dash: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut lo: libc::c_uint = 0;
    let mut hi: libc::c_uint = 0;
    dash = strchr(opt_i_str, '-' as i32);
    if dash.is_null() {
      crate::libbb::verror_msg::bb_error_msg_and_die(
        b"bad range \'%s\'\x00" as *const u8 as *const libc::c_char,
        opt_i_str,
      );
    }
    *dash = '\u{0}' as i32 as libc::c_char;
    lo = crate::libbb::xatonum::xatou(opt_i_str);
    hi = crate::libbb::xatonum::xatou(dash.offset(1));
    *dash = '-' as i32 as libc::c_char;
    if hi < lo {
      crate::libbb::verror_msg::bb_error_msg_and_die(
        b"bad range \'%s\'\x00" as *const u8 as *const libc::c_char,
        opt_i_str,
      );
    }
    numlines = hi.wrapping_add(1i32 as libc::c_uint).wrapping_sub(lo);
    lines = xmalloc(
      (numlines as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    i = 0i32 as libc::c_uint;
    while i < numlines {
      let ref mut fresh2 = *lines.offset(i as isize);
      *fresh2 = lo as uintptr_t as *mut libc::c_char;
      lo = lo.wrapping_add(1);
      i = i.wrapping_add(1)
    }
  } else {
    /* default - read lines from stdin or the input file */
    let mut fp: *mut FILE = std::ptr::null_mut();
    if argc > 1i32 {
      crate::libbb::appletlib::bb_show_usage();
    }
    fp = crate::libbb::wfopen_input::xfopen_stdin(if !(*argv.offset(0)).is_null() {
      *argv.offset(0)
    } else {
      b"-\x00" as *const u8 as *const libc::c_char
    });
    lines = std::ptr::null_mut();
    numlines = 0i32 as libc::c_uint;
    loop {
      let mut line: *mut libc::c_char = crate::libbb::get_line_from_file::xmalloc_fgetline(fp);
      if line.is_null() {
        break;
      }
      lines = crate::libbb::xrealloc_vector::xrealloc_vector_helper(
        lines as *mut libc::c_void,
        ((::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong) << 8i32)
          .wrapping_add(6i32 as libc::c_ulong) as libc::c_uint,
        numlines as libc::c_int,
      ) as *mut *mut libc::c_char;
      let fresh3 = numlines;
      numlines = numlines.wrapping_add(1);
      let ref mut fresh4 = *lines.offset(fresh3 as isize);
      *fresh4 = line
    }
    crate::libbb::fclose_nonstdin::fclose_if_not_stdin(fp);
  }
  if numlines != 0i32 as libc::c_uint {
    shuffle_lines(lines, numlines);
  }
  if opts & (1i32 << 3i32) as libc::c_uint != 0 {
    crate::libbb::xfuncs_printf::xmove_fd(
      crate::libbb::xfuncs_printf::xopen(opt_o_str, 0o1i32 | 0o100i32 | 0o1000i32),
      1i32,
    );
  }
  if opts & (1i32 << 2i32) as libc::c_uint != 0 {
    let mut maxlines: libc::c_uint = 0;
    maxlines = crate::libbb::xatonum::xatou(opt_n_str);
    if numlines > maxlines {
      numlines = maxlines
    }
  }
  eol = '\n' as i32 as libc::c_char;
  if opts & (1i32 << 4i32) as libc::c_uint != 0 {
    eol = '\u{0}' as i32 as libc::c_char
  }
  i = 0i32 as libc::c_uint;
  while i < numlines {
    if opts & (1i32 << 1i32) as libc::c_uint != 0 {
      printf(
        b"%u%c\x00" as *const u8 as *const libc::c_char,
        *lines.offset(i as isize) as uintptr_t as libc::c_uint,
        eol as libc::c_int,
      );
    } else {
      printf(
        b"%s%c\x00" as *const u8 as *const libc::c_char,
        *lines.offset(i as isize),
        eol as libc::c_int,
      );
    }
    i = i.wrapping_add(1)
  }
  crate::libbb::fflush_stdout_and_exit::fflush_stdout_and_exit(0i32);
}
