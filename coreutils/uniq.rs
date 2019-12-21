use crate::libbb::skip_whitespace::skip_whitespace;
use libc;
use libc::close;
use libc::free;
use libc::printf;
use libc::puts;
use libc::FILE;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut stdin: *mut FILE;

  #[no_mangle]
  fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;

}

pub const OPT_c: C2RustUnnamed = 1;
pub const OPT_d: C2RustUnnamed = 2;
pub const OPT_i: C2RustUnnamed = 64;
pub type C2RustUnnamed = libc::c_uint;
pub const OPT_w: C2RustUnnamed = 32;
pub const OPT_s: C2RustUnnamed = 16;
/* print only uniq */
pub const OPT_f: C2RustUnnamed = 8;
/* print only dups */
pub const OPT_u: C2RustUnnamed = 4;

/*
 * uniq implementation for busybox
 *
 * Copyright (C) 2005  Manuel Novoa III  <mjn3@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config UNIQ
//config:	bool "uniq (4.9 kb)"
//config:	default y
//config:	help
//config:	uniq is used to remove duplicate lines from a sorted file.
//applet:IF_UNIQ(APPLET(uniq, BB_DIR_USR_BIN, SUID_DROP))
//kbuild:lib-$(CONFIG_UNIQ) += uniq.o
/* BB_AUDIT SUSv3 compliant */
/* http://www.opengroup.org/onlinepubs/007904975/utilities/uniq.html */
//usage:#define uniq_trivial_usage
//usage:       "[-cdu][-f,s,w N] [INPUT [OUTPUT]]"
//usage:#define uniq_full_usage "\n\n"
//usage:       "Discard duplicate lines\n"
//usage:     "\n	-c	Prefix lines by the number of occurrences"
//usage:     "\n	-d	Only print duplicate lines"
//usage:     "\n	-u	Only print unique lines"
//usage:     "\n	-i	Ignore case"
//usage:     "\n	-f N	Skip first N fields"
//usage:     "\n	-s N	Skip first N chars (after any skipped fields)"
//usage:     "\n	-w N	Compare N characters in line"
//usage:
//usage:#define uniq_example_usage
//usage:       "$ echo -e \"a\\na\\nb\\nc\\nc\\na\" | sort | uniq\n"
//usage:       "a\n"
//usage:       "b\n"
//usage:       "c\n"
#[no_mangle]
pub unsafe extern "C" fn uniq_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut input_filename: *const libc::c_char = 0 as *const libc::c_char; /* == 0 */
  let mut skip_fields: libc::c_uint = 0;
  let mut skip_chars: libc::c_uint = 0;
  let mut max_chars: libc::c_uint = 0;
  let mut opt: libc::c_uint = 0;
  let mut cur_line: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut cur_compare: *const libc::c_char = 0 as *const libc::c_char;
  skip_chars = 0i32 as libc::c_uint;
  skip_fields = skip_chars;
  max_chars = 2147483647i32 as libc::c_uint;
  opt = crate::libbb::getopt32::getopt32(
    argv,
    b"cduf:+s:+w:+i\x00" as *const u8 as *const libc::c_char,
    &mut skip_fields as *mut libc::c_uint,
    &mut skip_chars as *mut libc::c_uint,
    &mut max_chars as *mut libc::c_uint,
  );
  argv = argv.offset(optind as isize);
  input_filename = *argv.offset(0);
  if !input_filename.is_null() {
    let mut output: *const libc::c_char = 0 as *const libc::c_char;
    if *input_filename.offset(0) as libc::c_int != '-' as i32
      || *input_filename.offset(1) as libc::c_int != 0
    {
      close(0i32);
      crate::libbb::xfuncs_printf::xopen(input_filename, 0i32);
      /* fd will be 0 */
    }
    output = *argv.offset(1);
    if !output.is_null() {
      if !(*argv.offset(2)).is_null() {
        crate::libbb::appletlib::bb_show_usage();
      }
      if *output.offset(0) as libc::c_int != '-' as i32 || *output.offset(1) as libc::c_int != 0 {
        // Won't work with "uniq - FILE" and closed stdin:
        //close(STDOUT_FILENO);
        //xopen(output, O_WRONLY | O_CREAT | O_TRUNC);
        crate::libbb::xfuncs_printf::xmove_fd(
          crate::libbb::xfuncs_printf::xopen(output, 0o1i32 | 0o100i32 | 0o1000i32),
          1i32,
        ); /* prime the pump */
      }
    }
  }
  cur_line = std::ptr::null_mut::<libc::c_char>();
  cur_compare = cur_line;
  loop {
    let mut i: libc::c_uint = 0;
    let mut dups: libc::c_ulong = 0;
    let mut old_line: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut old_compare: *const libc::c_char = 0 as *const libc::c_char;
    old_line = cur_line;
    old_compare = cur_compare;
    dups = 0i32 as libc::c_ulong;
    loop
    /* gnu uniq ignores newlines */
    {
      cur_line = crate::libbb::get_line_from_file::xmalloc_fgetline(stdin);
      if cur_line.is_null() {
        break;
      }
      cur_compare = cur_line;
      i = skip_fields;
      while i != 0 {
        cur_compare = skip_whitespace(cur_compare);
        cur_compare = crate::libbb::skip_whitespace::skip_non_whitespace(cur_compare);
        i = i.wrapping_sub(1)
      }
      i = skip_chars;
      while *cur_compare as libc::c_int != 0 && i != 0 {
        cur_compare = cur_compare.offset(1);
        i = i.wrapping_sub(1)
      }
      if old_line.is_null() {
        break;
      }
      if if opt & OPT_i as libc::c_int as libc::c_uint != 0 {
        strncasecmp(old_compare, cur_compare, max_chars as libc::c_ulong)
      } else {
        strncmp(old_compare, cur_compare, max_chars as libc::c_ulong)
      } != 0
      {
        break;
      }
      free(cur_line as *mut libc::c_void);
      dups = dups.wrapping_add(1)
      /* testing for overflow seems excessive */
    }
    if !old_line.is_null() {
      if opt & ((OPT_d as libc::c_int) << (dups != 0) as libc::c_int) as libc::c_uint == 0 {
        /* (if dups, opt & OPT_u) */
        if opt & OPT_c as libc::c_int as libc::c_uint != 0 {
          /* %7lu matches GNU coreutils 6.9 */
          printf(
            b"%7lu \x00" as *const u8 as *const libc::c_char,
            dups.wrapping_add(1i32 as libc::c_ulong),
          );
        }
        puts(old_line);
      }
      free(old_line as *mut libc::c_void);
    }
    if cur_line.is_null() {
      break;
    }
  }
  crate::libbb::xfuncs_printf::die_if_ferror(stdin, input_filename);
  crate::libbb::fflush_stdout_and_exit::fflush_stdout_and_exit(0i32);
}
