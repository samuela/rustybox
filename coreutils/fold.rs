use crate::librb::size_t;
use crate::librb::smallint;
use libc;
use libc::putchar_unlocked;
use libc::FILE;
extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut stdout: *mut FILE;
  #[no_mangle]
  fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t, __s: *mut FILE) -> size_t;
  #[no_mangle]
  fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  static mut option_mask32: u32;

}

/*
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
pub type C2RustUnnamed = libc::c_uint;
pub const UNICODE_ON: C2RustUnnamed = 2;
pub const UNICODE_OFF: C2RustUnnamed = 1;
pub const UNICODE_UNKNOWN: C2RustUnnamed = 0;
/* Assuming the current column is COLUMN, return the column that
printing C will move the cursor to.
The first column is 0. */
unsafe extern "C" fn adjust_column(mut column: libc::c_uint, mut c: libc::c_char) -> libc::c_int {
  if option_mask32 & 1i32 as libc::c_uint != 0 {
    column = column.wrapping_add(1);
    return column as libc::c_int;
  }
  if c as libc::c_int == '\t' as i32 {
    return column
      .wrapping_add(8i32 as libc::c_uint)
      .wrapping_sub(column.wrapping_rem(8i32 as libc::c_uint)) as libc::c_int;
  }
  if c as libc::c_int == '\u{8}' as i32 {
    column = column.wrapping_sub(1);
    if (column as libc::c_int) < 0 {
      column = 0 as libc::c_uint
    }
  } else if c as libc::c_int == '\r' as i32 {
    column = 0 as libc::c_uint
  } else if UNICODE_ON as libc::c_int != UNICODE_ON as libc::c_int
    || c as libc::c_int & 0xc0i32 != 0x80i32
  {
    /* just a printable char */
    /* it isn't a 2nd+ byte of a Unicode char */
    column = column.wrapping_add(1)
  }
  return column as libc::c_int;
}
/* Note that this function can write NULs, unlike fputs etc. */
unsafe extern "C" fn write2stdout(mut buf: *const libc::c_void, mut size: libc::c_uint) {
  fwrite(buf, 1i32 as size_t, size as size_t, stdout);
}
#[no_mangle]
pub unsafe extern "C" fn fold_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut line_out: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut w_opt: *const libc::c_char = b"80\x00" as *const u8 as *const libc::c_char;
  let mut width: libc::c_uint = 0;
  let mut exitcode: smallint = 0 as smallint;
  /* Turn any numeric options into -w options.  */
  let mut i: libc::c_int = 0; /* Screen column where next char will go */
  i = 1i32; /* Index in 'line_out' for next char */
  while !(*argv.offset(i as isize)).is_null() {
    let mut a: *const libc::c_char = *argv.offset(i as isize); /* while (not EOF) */
    if *a as libc::c_int == '-' as i32 {
      a = a.offset(1);
      if *a as libc::c_int == '-' as i32 && *a.offset(1) == 0 {
        break;
      }
      if (*a as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
        let ref mut fresh0 = *argv.offset(i as isize);
        *fresh0 =
          crate::libbb::xfuncs_printf::xasprintf(b"-w%s\x00" as *const u8 as *const libc::c_char, a)
      }
    }
    i += 1
  }
  crate::libbb::getopt32::getopt32(
    argv,
    b"bsw:\x00" as *const u8 as *const libc::c_char,
    &mut w_opt as *mut *const libc::c_char,
  );
  width = crate::libbb::xatonum::xatou_range(w_opt, 1i32 as libc::c_uint, 10000i32 as libc::c_uint);
  argv = argv.offset(optind as isize);
  if (*argv).is_null() {
    argv = argv.offset(-1);
    *argv = b"-\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
  }
  loop {
    let mut istream: *mut FILE = crate::libbb::wfopen_input::fopen_or_warn_stdin(*argv);
    let mut c: libc::c_int = 0;
    let mut column: libc::c_uint = 0 as libc::c_uint;
    let mut offset_out: libc::c_uint = 0 as libc::c_uint;
    if istream.is_null() {
      exitcode = 1i32 as smallint
    } else {
      loop {
        c = getc_unlocked(istream);
        if !(c != -1i32) {
          break;
        }
        /* We grow line_out in chunks of 0x1000 bytes */
        if offset_out & 0xfffi32 as libc::c_uint == 0 as libc::c_uint {
          line_out = crate::libbb::xfuncs_printf::xrealloc(
            line_out as *mut libc::c_void,
            offset_out.wrapping_add(0x1000i32 as libc::c_uint) as size_t,
          ) as *mut libc::c_char
        }
        'c_8965: loop {
          *line_out.offset(offset_out as isize) = c as libc::c_char;
          if c == '\n' as i32 {
            write2stdout(
              line_out as *const libc::c_void,
              offset_out.wrapping_add(1i32 as libc::c_uint),
            );
            offset_out = 0 as libc::c_uint;
            column = offset_out;
            break;
          } else {
            column = adjust_column(column, c as libc::c_char) as libc::c_uint;
            if column <= width || offset_out == 0 as libc::c_uint {
              /* offset_out == 0 case happens
               * with small width (say, 1) and tabs.
               * The very first tab already goes to column 8,
               * but we must not wrap it */
              offset_out = offset_out.wrapping_add(1);
              break;
            } else {
              /* This character would make the line too long.
               * Print the line plus a newline, and make this character
               * start the next line */
              if option_mask32 & 2i32 as libc::c_uint != 0 {
                let mut i_0: libc::c_uint = 0;
                let mut logical_end: libc::c_uint = 0;
                /* No blank found, wrap will split the overlong word */
                /* Look for the last blank. */
                logical_end = offset_out.wrapping_sub(1i32 as libc::c_uint);
                while logical_end as libc::c_int >= 0 {
                  if ({
                    let mut bb__isblank: libc::c_uchar =
                      *line_out.offset(logical_end as isize) as libc::c_uchar;
                    (bb__isblank as libc::c_int == ' ' as i32
                      || bb__isblank as libc::c_int == '\t' as i32)
                      as libc::c_int
                  }) == 0
                  {
                    logical_end = logical_end.wrapping_sub(1)
                  } else {
                    /* Found a space or tab.
                     * Output up to and including it, and start a new line */
                    logical_end = logical_end.wrapping_add(1);
                    /*line_out[logical_end] = '\n'; - NO! this nukes one buffered character */
                    write2stdout(line_out as *const libc::c_void, logical_end);
                    putchar_unlocked('\n' as i32);
                    /* Move the remainder to the beginning of the next line.
                     * The areas being copied here might overlap. */
                    memmove(
                      line_out as *mut libc::c_void,
                      line_out.offset(logical_end as isize) as *const libc::c_void,
                      offset_out.wrapping_sub(logical_end) as libc::c_ulong,
                    );
                    offset_out = offset_out.wrapping_sub(logical_end);
                    i_0 = 0 as libc::c_uint;
                    column = i_0;
                    while i_0 < offset_out {
                      column =
                        adjust_column(column, *line_out.offset(i_0 as isize)) as libc::c_uint;
                      i_0 = i_0.wrapping_add(1)
                    }
                    continue 'c_8965;
                  }
                }
              }
              /* Output what we accumulated up to now, and start a new line */
              *line_out.offset(offset_out as isize) = '\n' as i32 as libc::c_char;
              write2stdout(
                line_out as *const libc::c_void,
                offset_out.wrapping_add(1i32 as libc::c_uint),
              );
              offset_out = 0 as libc::c_uint;
              column = offset_out
            }
          }
        }
      }
      if offset_out != 0 {
        write2stdout(line_out as *const libc::c_void, offset_out);
      }
      if crate::libbb::fclose_nonstdin::fclose_if_not_stdin(istream) != 0 {
        crate::libbb::perror_msg::bb_simple_perror_msg(*argv);
        exitcode = 1i32 as smallint
      }
    }
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  crate::libbb::fflush_stdout_and_exit::fflush_stdout_and_exit(exitcode as libc::c_int);
}
