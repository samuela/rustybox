use libc;
use libc::fchmod;
use libc::free;
use libc::strchr;
use libc::strcmp;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut stdout: *mut FILE;
  #[no_mangle]
  fn fflush(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t, __s: *mut FILE) -> size_t;
  #[no_mangle]
  fn fileno_unlocked(__stream: *mut FILE) -> libc::c_int;

  /* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
   * But potentially slow, don't use in one-billion-times loops */

  // NB: will return short read on error, not -1,
  // if some data was read before error occurred

  /* Same, with limited max size, and returns the length (excluding NUL): */

  /* Chops off '\n' from the end, unlike fgets: */

  /* "Opens" stdin if filename is special, else just opens file: */

  #[no_mangle]
  static bb_uuenc_tbl_base64: [libc::c_char; 0];

}

use crate::librb::size_t;
use libc::mode_t;
use libc::ssize_t;
use libc::FILE;
pub type C2RustUnnamed = libc::c_uint;
pub const BASE64_FLAG_NO_STOP_CHAR: C2RustUnnamed = 128;
pub const BASE64_FLAG_UU_STOP: C2RustUnnamed = 256;
pub const SRC_BUF_SIZE: C2RustUnnamed_0 = 57;
pub type C2RustUnnamed_0 = libc::c_uint;
/* This *MUST* be a multiple of 3 */
pub const DST_BUF_SIZE: C2RustUnnamed_0 = 76;

/*
 * Copyright 2003, Glenn McGrath
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 *
 * Based on specification from
 * http://www.opengroup.org/onlinepubs/007904975/utilities/uuencode.html
 *
 * Bugs: the spec doesn't mention anything about "`\n`\n" prior to the
 * "end" line
 */
//config:config UUDECODE
//config:	bool "uudecode (5.8 kb)"
//config:	default y
//config:	help
//config:	uudecode is used to decode a uuencoded file.
//applet:IF_UUDECODE(APPLET(uudecode, BB_DIR_USR_BIN, SUID_DROP))
//kbuild:lib-$(CONFIG_UUDECODE) += uudecode.o
//usage:#define uudecode_trivial_usage
//usage:       "[-o OUTFILE] [INFILE]"
//usage:#define uudecode_full_usage "\n\n"
//usage:       "Uudecode a file\n"
//usage:       "Finds OUTFILE in uuencoded source unless -o is given"
//usage:
//usage:#define uudecode_example_usage
//usage:       "$ uudecode -o busybox busybox.uu\n"
//usage:       "$ ls -l busybox\n"
//usage:       "-rwxr-xr-x   1 ams      ams        245264 Jun  7 21:35 busybox\n"
unsafe extern "C" fn read_stduu(
  mut src_stream: *mut FILE,
  mut dst_stream: *mut FILE,
  mut _flags: libc::c_int,
) {
  let mut line: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  loop {
    let mut encoded_len: libc::c_int = 0;
    let mut str_len: libc::c_int = 0;
    let mut line_ptr: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut dst: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut line_len: size_t = 0;
    line_len = (64i32 * 1024i32) as size_t;
    line = crate::libbb::fgets_str::xmalloc_fgets_str_len(
      src_stream,
      b"\n\x00" as *const u8 as *const libc::c_char,
      &mut line_len,
    );
    if line.is_null() {
      break;
    }
    /* Handle both Unix and MSDOS text.
     * Note: space should not be trimmed, some encoders use it instead of "`"
     * for padding of last incomplete 4-char block.
     */
    str_len = line_len as libc::c_int;
    loop {
      str_len -= 1;
      if !(str_len >= 0i32
        && (*line.offset(str_len as isize) as libc::c_int == '\n' as i32
          || *line.offset(str_len as isize) as libc::c_int == '\r' as i32))
      {
        break;
      }
      *line.offset(str_len as isize) = '\u{0}' as i32 as libc::c_char
    }
    if strcmp(line, b"end\x00" as *const u8 as *const libc::c_char) == 0i32 {
      return;
      /* the only non-error exit */
    }
    line_ptr = line;
    while *line_ptr != 0 {
      *line_ptr = (*line_ptr as libc::c_int - 0x20i32 & 0x3fi32) as libc::c_char;
      line_ptr = line_ptr.offset(1)
    }
    str_len = line_ptr.wrapping_offset_from(line) as libc::c_long as libc::c_int;
    encoded_len = *line.offset(0) as libc::c_int * 4i32 / 3i32;
    /* Check that line is not too short. (we tolerate
     * overly _long_ line to accommodate possible extra "`").
     * Empty line case is also caught here. */
    if str_len <= encoded_len {
      break;
    }
    if encoded_len <= 0i32 {
      /* Ignore the "`\n" line, why is it even in the encode file ? */
      free(line as *mut libc::c_void);
    } else {
      if encoded_len > 60i32 {
        crate::libbb::verror_msg::bb_simple_error_msg_and_die(
          b"line too long\x00" as *const u8 as *const libc::c_char,
        );
      }
      dst = line;
      line_ptr = line.offset(1);
      loop {
        /* Merge four 6 bit chars to three 8 bit chars */
        let fresh0 = dst;
        dst = dst.offset(1);
        *fresh0 = ((*line_ptr.offset(0) as libc::c_int) << 2i32
          | *line_ptr.offset(1) as libc::c_int >> 4i32) as libc::c_char;
        encoded_len -= 1;
        if encoded_len == 0i32 {
          break;
        }
        let fresh1 = dst;
        dst = dst.offset(1);
        *fresh1 = ((*line_ptr.offset(1) as libc::c_int) << 4i32
          | *line_ptr.offset(2) as libc::c_int >> 2i32) as libc::c_char;
        encoded_len -= 1;
        if encoded_len == 0i32 {
          break;
        }
        let fresh2 = dst;
        dst = dst.offset(1);
        *fresh2 = ((*line_ptr.offset(2) as libc::c_int) << 6i32
          | *line_ptr.offset(3) as libc::c_int) as libc::c_char;
        line_ptr = line_ptr.offset(4);
        encoded_len -= 2i32;
        if !(encoded_len > 0i32) {
          break;
        }
      }
      fwrite(
        line as *const libc::c_void,
        1i32 as size_t,
        dst.wrapping_offset_from(line) as libc::c_long as size_t,
        dst_stream,
      );
      free(line as *mut libc::c_void);
    }
  }
  crate::libbb::verror_msg::bb_simple_error_msg_and_die(
    b"short file\x00" as *const u8 as *const libc::c_char,
  );
}
#[no_mangle]
pub unsafe extern "C" fn uudecode_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut src_stream: *mut FILE = std::ptr::null_mut();
  let mut outname: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut line: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  crate::libbb::getopt32::getopt32(
    argv,
    b"^o:\x00?1\x00" as *const u8 as *const libc::c_char,
    &mut outname as *mut *mut libc::c_char,
  );
  argv = argv.offset(optind as isize);
  if (*argv.offset(0)).is_null() {
    argv = argv.offset(-1);
    *argv = b"-\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
  }
  src_stream = crate::libbb::wfopen_input::xfopen_stdin(*argv.offset(0));
  loop
  /* Search for the start of the encoding */
  {
    line = crate::libbb::get_line_from_file::xmalloc_fgetline(src_stream);
    if line.is_null() {
      break;
    }
    let mut decode_fn_ptr: Option<
      unsafe extern "C" fn(_: *mut FILE, _: *mut FILE, _: libc::c_int) -> (),
    > = None;
    let mut line_ptr: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut dst_stream: *mut FILE = std::ptr::null_mut();
    let mut mode: libc::c_int = 0;
    if !crate::libbb::compare_string_array::is_prefixed_with(
      line,
      b"begin-base64 \x00" as *const u8 as *const libc::c_char,
    )
    .is_null()
    {
      line_ptr = line.offset(13);
      decode_fn_ptr = Some(
        crate::libbb::uuencode::read_base64
          as unsafe extern "C" fn(_: *mut FILE, _: *mut FILE, _: libc::c_int) -> (),
      )
    } else if !crate::libbb::compare_string_array::is_prefixed_with(
      line,
      b"begin \x00" as *const u8 as *const libc::c_char,
    )
    .is_null()
    {
      line_ptr = line.offset(6);
      decode_fn_ptr =
        Some(read_stduu as unsafe extern "C" fn(_: *mut FILE, _: *mut FILE, _: libc::c_int) -> ())
    } else {
      free(line as *mut libc::c_void);
      continue;
    }
    /* begin line found. decode and exit */
    mode = crate::libbb::bb_strtonum::bb_strtou(line_ptr, 0 as *mut *mut libc::c_char, 8i32)
      as libc::c_int; /* remove trailing space (and '\r' for DOS text) */
    if outname.is_null() {
      outname = strchr(line_ptr, ' ' as i32);
      if outname.is_null() {
        break;
      }
      outname = outname.offset(1);
      crate::libbb::trim::trim(outname);
      if *outname.offset(0) == 0 {
        break;
      }
    }
    dst_stream = stdout;
    if *outname.offset(0) as libc::c_int != '-' as i32 || *outname.offset(1) as libc::c_int != 0 {
      dst_stream = crate::libbb::wfopen::xfopen_for_write(outname);
      fchmod(
        fileno_unlocked(dst_stream),
        (mode
          & (0o400i32
            | 0o200i32
            | 0o100i32
            | (0o400i32 | 0o200i32 | 0o100i32) >> 3i32
            | (0o400i32 | 0o200i32 | 0o100i32) >> 3i32 >> 3i32)) as mode_t,
      );
    }
    free(line as *mut libc::c_void);
    decode_fn_ptr.expect("non-null function pointer")(
      src_stream,
      dst_stream,
      BASE64_FLAG_UU_STOP as libc::c_int + BASE64_FLAG_NO_STOP_CHAR as libc::c_int,
    );
    /* fclose_if_not_stdin(src_stream); - redundant */
    return 0i32;
  }
  crate::libbb::verror_msg::bb_simple_error_msg_and_die(
    b"no \'begin\' line\x00" as *const u8 as *const libc::c_char,
  );
}
//applet:IF_BASE64(APPLET(base64, BB_DIR_BIN, SUID_DROP))
//kbuild:lib-$(CONFIG_BASE64) += uudecode.o
//config:config BASE64
//config:	bool "base64 (4.9 kb)"
//config:	default y
//config:	help
//config:	Base64 encode and decode
//usage:#define base64_trivial_usage
//usage:	"[-d] [FILE]"
//usage:#define base64_full_usage "\n\n"
//usage:       "Base64 encode or decode FILE to standard output"
//usage:     "\n	-d	Decode data"
// //usage:     "\n	-w COL	Wrap lines at COL (default 76, 0 disables)"
// //usage:     "\n	-i	When decoding, ignore non-alphabet characters"
#[no_mangle]
pub unsafe extern "C" fn base64_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut src_stream: *mut FILE = std::ptr::null_mut();
  let mut opts: libc::c_uint = 0;
  opts =
    crate::libbb::getopt32::getopt32(argv, b"^d\x00?1\x00" as *const u8 as *const libc::c_char);
  argv = argv.offset(optind as isize);
  if (*argv.offset(0)).is_null() {
    argv = argv.offset(-1);
    *argv = b"-\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
  }
  src_stream = crate::libbb::wfopen_input::xfopen_stdin(*argv.offset(0));
  if opts != 0 {
    crate::libbb::uuencode::read_base64(src_stream, stdout, -1i32 as libc::c_char as libc::c_int);
  } else {
    let mut src_buf: [libc::c_char; 57] = [0; 57];
    let mut dst_buf: [libc::c_char; 77] = [0; 77];
    let mut src_fd: libc::c_int = fileno_unlocked(src_stream);
    loop {
      let mut size: size_t = crate::libbb::read::full_read(
        src_fd,
        src_buf.as_mut_ptr() as *mut libc::c_void,
        SRC_BUF_SIZE as libc::c_int as size_t,
      ) as size_t;
      if size == 0 {
        break;
      }
      if (size as ssize_t) < 0 {
        crate::libbb::perror_msg::bb_simple_perror_msg_and_die(
          b"read error\x00" as *const u8 as *const libc::c_char,
        );
      }
      /* Encode the buffer we just read in */
      crate::libbb::uuencode::bb_uuencode(
        dst_buf.as_mut_ptr(),
        src_buf.as_mut_ptr() as *const libc::c_void,
        size as libc::c_int,
        bb_uuenc_tbl_base64.as_ptr(),
      );
      crate::libbb::xfuncs_printf::xwrite(
        1i32,
        dst_buf.as_mut_ptr() as *const libc::c_void,
        (4i32 as libc::c_ulong).wrapping_mul(
          size
            .wrapping_add(2i32 as libc::c_ulong)
            .wrapping_div(3i32 as libc::c_ulong),
        ),
      );
      crate::libbb::xfuncs_printf::bb_putchar('\n' as i32);
      fflush(stdout);
    }
  }
  crate::libbb::fflush_stdout_and_exit::fflush_stdout_and_exit(0i32);
}
/* Test script.
Put this into an empty dir with busybox binary, an run.

#!/bin/sh
test -x busybox || { echo "No ./busybox?"; exit; }
ln -sf busybox uudecode
ln -sf busybox uuencode
>A_null
echo -n A >A
echo -n AB >AB
echo -n ABC >ABC
echo -n ABCD >ABCD
echo -n ABCDE >ABCDE
echo -n ABCDEF >ABCDEF
cat busybox >A_bbox
for f in A*; do
    echo uuencode $f
    ./uuencode    $f <$f >u_$f
    ./uuencode -m $f <$f >m_$f
done
mkdir unpk_u unpk_m 2>/dev/null
for f in u_*; do
    ./uudecode <$f -o unpk_u/${f:2}
    diff -a ${f:2} unpk_u/${f:2} >/dev/null 2>&1
    echo uudecode $f: $?
done
for f in m_*; do
    ./uudecode <$f -o unpk_m/${f:2}
    diff -a ${f:2} unpk_m/${f:2} >/dev/null 2>&1
    echo uudecode $f: $?
done
*/
