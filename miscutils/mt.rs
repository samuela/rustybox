use libc;

extern "C" {
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  /* Specialized: */
  /* Using xatoi() instead of naive atoi() is not always convenient -
   * in many places people want *non-negative* values, but store them
   * in signed int. Therefore we need this one:
   * dies if input is not in [0, INT_MAX] range. Also will reject '-0' etc.
   * It should really be named xatoi_nonnegative (since it allows 0),
   * but that would be too long.
   */
  #[no_mangle]
  fn xatoi_positive(numstr: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn index_in_strings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn ioctl_or_perror_and_die(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    fmt: *const libc::c_char,
    _: ...
  ) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtop {
  pub mt_op: libc::c_short,
  pub mt_count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtpos {
  pub mt_blkno: libc::c_long,
}

/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config MT
//config:	bool "mt (2.5 kb)"
//config:	default y
//config:	help
//config:	mt is used to control tape devices. You can use the mt utility
//config:	to advance or rewind a tape past a specified number of archive
//config:	files on the tape.
//applet:IF_MT(APPLET(mt, BB_DIR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_MT) += mt.o
//usage:#define mt_trivial_usage
//usage:       "[-f device] opcode value"
//usage:#define mt_full_usage "\n\n"
//usage:       "Control magnetic tape drive operation\n"
//usage:       "\n"
//usage:       "Available Opcodes:\n"
//usage:       "\n"
//usage:       "bsf bsfm bsr bss datacompression drvbuffer eof eom erase\n"
//usage:       "fsf fsfm fsr fss load lock mkpart nop offline ras1 ras2\n"
//usage:       "ras3 reset retension rewind rewoffline seek setblk setdensity\n"
//usage:       "setpart tell unload unlock weof wset"
/* missing: eod/seod, stoptions, stwrthreshold, densities */
static mut opcode_value: [libc::c_short; 34] = [
  2i32 as libc::c_short,
  10i32 as libc::c_short,
  4i32 as libc::c_short,
  26i32 as libc::c_short,
  32i32 as libc::c_short,
  12i32 as libc::c_short,
  13i32 as libc::c_short,
  1i32 as libc::c_short,
  11i32 as libc::c_short,
  3i32 as libc::c_short,
  25i32 as libc::c_short,
  30i32 as libc::c_short,
  28i32 as libc::c_short,
  34i32 as libc::c_short,
  8i32 as libc::c_short,
  7i32 as libc::c_short,
  7i32 as libc::c_short,
  14i32 as libc::c_short,
  15i32 as libc::c_short,
  16i32 as libc::c_short,
  0i32 as libc::c_short,
  9i32 as libc::c_short,
  6i32 as libc::c_short,
  22i32 as libc::c_short,
  20i32 as libc::c_short,
  21i32 as libc::c_short,
  24i32 as libc::c_short,
  33i32 as libc::c_short,
  23i32 as libc::c_short,
  27i32 as libc::c_short,
  31i32 as libc::c_short,
  29i32 as libc::c_short,
  5i32 as libc::c_short,
  5i32 as libc::c_short,
];
static mut opcode_name: [libc::c_char; 213] = [
  98, 115, 102, 0, 98, 115, 102, 109, 0, 98, 115, 114, 0, 98, 115, 115, 0, 100, 97, 116, 97, 99,
  111, 109, 112, 114, 101, 115, 115, 105, 111, 110, 0, 101, 111, 109, 0, 101, 114, 97, 115, 101, 0,
  102, 115, 102, 0, 102, 115, 102, 109, 0, 102, 115, 114, 0, 102, 115, 115, 0, 108, 111, 97, 100,
  0, 108, 111, 99, 107, 0, 109, 107, 112, 97, 114, 116, 0, 110, 111, 112, 0, 111, 102, 102, 108,
  105, 110, 101, 0, 114, 101, 119, 111, 102, 102, 108, 105, 110, 101, 0, 114, 97, 115, 49, 0, 114,
  97, 115, 50, 0, 114, 97, 115, 51, 0, 114, 101, 115, 101, 116, 0, 114, 101, 116, 101, 110, 115,
  105, 111, 110, 0, 114, 101, 119, 105, 110, 100, 0, 115, 101, 101, 107, 0, 115, 101, 116, 98, 108,
  107, 0, 115, 101, 116, 100, 101, 110, 115, 105, 116, 121, 0, 100, 114, 118, 98, 117, 102, 102,
  101, 114, 0, 115, 101, 116, 112, 97, 114, 116, 0, 116, 101, 108, 108, 0, 119, 115, 101, 116, 0,
  117, 110, 108, 111, 97, 100, 0, 117, 110, 108, 111, 99, 107, 0, 101, 111, 102, 0, 119, 101, 111,
  102, 0, 0,
];
#[no_mangle]
pub unsafe extern "C" fn mt_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut file: *const libc::c_char = b"/dev/tape\x00" as *const u8 as *const libc::c_char; /* One, not zero, right? */
  let mut op: mtop = mtop {
    mt_op: 0,
    mt_count: 0,
  };
  let mut position: mtpos = mtpos { mt_blkno: 0 };
  let mut fd: libc::c_int = 0;
  let mut mode: libc::c_int = 0;
  let mut idx: libc::c_int = 0;
  if (*argv.offset(1)).is_null() {
    bb_show_usage();
  }
  if strcmp(
    *argv.offset(1),
    b"-f\x00" as *const u8 as *const libc::c_char,
  ) == 0i32
  {
    if (*argv.offset(2)).is_null() || (*argv.offset(3)).is_null() {
      bb_show_usage();
    }
    file = *argv.offset(2);
    argv = argv.offset(2)
  }
  idx = index_in_strings(opcode_name.as_ptr(), *argv.offset(1));
  if idx < 0i32 {
    bb_error_msg_and_die(
      b"unrecognized opcode %s\x00" as *const u8 as *const libc::c_char,
      *argv.offset(1),
    );
  }
  op.mt_op = opcode_value[idx as usize];
  if !(*argv.offset(2)).is_null() {
    op.mt_count = xatoi_positive(*argv.offset(2))
  } else {
    op.mt_count = 1i32
  }
  match opcode_value[idx as usize] as libc::c_int {
    5 | 13 | 27 | 24 => mode = 0o1i32,
    _ => mode = 0i32,
  }
  fd = xopen(file, mode);
  match opcode_value[idx as usize] as libc::c_int {
    23 => {
      ioctl_or_perror_and_die(
        fd,
        ((2u32 << 0i32 + 8i32 + 8i32 + 14i32
          | (('m' as i32) << 0i32 + 8i32) as libc::c_uint
          | (3i32 << 0i32) as libc::c_uint) as libc::c_ulong
          | (::std::mem::size_of::<mtpos>() as libc::c_ulong) << 0i32 + 8i32 + 8i32)
          as libc::c_uint,
        &mut position as *mut mtpos as *mut libc::c_void,
        b"%s\x00" as *const u8 as *const libc::c_char,
        file,
      );
      printf(
        b"At block %d\n\x00" as *const u8 as *const libc::c_char,
        position.mt_blkno as libc::c_int,
      );
    }
    _ => {
      ioctl_or_perror_and_die(
        fd,
        ((1u32 << 0i32 + 8i32 + 8i32 + 14i32
          | (('m' as i32) << 0i32 + 8i32) as libc::c_uint
          | (1i32 << 0i32) as libc::c_uint) as libc::c_ulong
          | (::std::mem::size_of::<mtop>() as libc::c_ulong) << 0i32 + 8i32 + 8i32)
          as libc::c_uint,
        &mut op as *mut mtop as *mut libc::c_void,
        b"%s\x00" as *const u8 as *const libc::c_char,
        file,
      );
    }
  }
  return 0i32;
}
