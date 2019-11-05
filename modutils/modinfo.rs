use libc;
extern "C" {
  #[no_mangle]
  fn fnmatch(
    __pattern: *const libc::c_char,
    __name: *const libc::c_char,
    __flags: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn uname(__name: *mut utsname) -> libc::c_int;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn bb_basename(name: *const libc::c_char) -> *const libc::c_char;
  #[no_mangle]
  fn last_char_is(s: *const libc::c_char, c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc_open_zipped_read_close(
    fname: *const libc::c_char,
    maxsz_p: *mut size_t,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn xfopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  static mut option_mask32: uint32_t;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn config_open2(
    filename: *const libc::c_char,
    fopen_func: Option<unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE>,
  ) -> *mut parser_t;
  #[no_mangle]
  fn config_read(
    parser: *mut parser_t,
    tokens: *mut *mut libc::c_char,
    flags: libc::c_uint,
    delims: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  fn config_close(parser: *mut parser_t);
  #[no_mangle]
  fn filename2modname(
    filename: *const libc::c_char,
    modname: *mut libc::c_char,
  ) -> *mut libc::c_char;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
  pub sysname: [libc::c_char; 65],
  pub nodename: [libc::c_char; 65],
  pub release: [libc::c_char; 65],
  pub version: [libc::c_char; 65],
  pub machine: [libc::c_char; 65],
  pub domainname: [libc::c_char; 65],
}
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;



use crate::librb::FILE;
pub type C2RustUnnamed = libc::c_uint;
pub const PARSE_NORMAL: C2RustUnnamed = 4653056;
pub const PARSE_WS_COMMENTS: C2RustUnnamed = 16777216;
pub const PARSE_ALT_COMMENTS: C2RustUnnamed = 8388608;
pub const PARSE_EOL_COMMENTS: C2RustUnnamed = 4194304;
pub const PARSE_KEEP_COPY: C2RustUnnamed = 2097152;
pub const PARSE_MIN_DIE: C2RustUnnamed = 1048576;
pub const PARSE_GREEDY: C2RustUnnamed = 262144;
pub const PARSE_TRIM: C2RustUnnamed = 131072;
pub const PARSE_COLLAPSE: C2RustUnnamed = 65536;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parser_t {
  pub fp: *mut FILE,
  pub data: *mut libc::c_char,
  pub line: *mut libc::c_char,
  pub nline: *mut libc::c_char,
  pub line_alloc: size_t,
  pub nline_alloc: size_t,
  pub lineno: libc::c_int,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const OPT_TAGS: C2RustUnnamed_0 = 32764;
/* field name */
/* first bits are for -nadlp options, the rest are for
 * fields not selectable with "shortcut" options
 */
pub const OPT_n: C2RustUnnamed_0 = 4;
/* \0 as separator */
pub const OPT_F: C2RustUnnamed_0 = 2;
pub const OPT_0: C2RustUnnamed_0 = 1;
/* vi: set sw=4 ts=4: */
/*
 * modinfo - retrieve module info
 * Copyright (c) 2008 Pascal Bellard
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config MODINFO
//config:	bool "modinfo (24 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	Show information about a Linux Kernel module
//applet:IF_MODINFO(APPLET_NOEXEC(modinfo, modinfo, BB_DIR_SBIN, BB_SUID_DROP, modinfo))
//kbuild:lib-$(CONFIG_MODINFO) += modinfo.o modutils.o
/* uname() */
static mut shortcuts: [*const libc::c_char; 13] = [
  b"filename\x00" as *const u8 as *const libc::c_char,
  b"author\x00" as *const u8 as *const libc::c_char,
  b"description\x00" as *const u8 as *const libc::c_char,
  b"license\x00" as *const u8 as *const libc::c_char,
  b"parm\x00" as *const u8 as *const libc::c_char,
  b"version\x00" as *const u8 as *const libc::c_char,
  b"alias\x00" as *const u8 as *const libc::c_char,
  b"srcversion\x00" as *const u8 as *const libc::c_char,
  b"depends\x00" as *const u8 as *const libc::c_char,
  b"uts_release\x00" as *const u8 as *const libc::c_char,
  b"intree\x00" as *const u8 as *const libc::c_char,
  b"vermagic\x00" as *const u8 as *const libc::c_char,
  b"firmware\x00" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn display(mut data: *const libc::c_char, mut pattern: *const libc::c_char) {
  let mut flag: libc::c_int = (option_mask32 >> 1i32) as libc::c_int; /* shift out -0 bit */
  if flag & flag - 1i32 != 0 {
    /* more than one field to show: print "FIELD:" pfx */
    let mut n: libc::c_int = printf(b"%s:\x00" as *const u8 as *const libc::c_char, pattern);
    loop {
      let fresh0 = n;
      n = n + 1;
      if !(fresh0 < 16i32) {
        break;
      }
      bb_putchar(' ' as i32);
    }
  }
  printf(
    b"%s%c\x00" as *const u8 as *const libc::c_char,
    data,
    if option_mask32 & OPT_0 as libc::c_int as libc::c_uint != 0 {
      '\u{0}' as i32
    } else {
      '\n' as i32
    },
  );
}
unsafe extern "C" fn modinfo(
  mut path: *const libc::c_char,
  mut version: *const libc::c_char,
  mut field: *const libc::c_char,
) {
  let mut current_block: u64;
  let mut len: size_t = 0;
  let mut j: libc::c_int = 0;
  let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut the_module: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut allocated: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut tags: libc::c_int = option_mask32 as libc::c_int;
  allocated = 0 as *mut libc::c_char;
  len = if -1i32 as ssize_t > 0i32 as libc::c_long {
    -1i32 as ssize_t
  } else {
    !((1i32 as ssize_t)
      << (::std::mem::size_of::<ssize_t>() as libc::c_ulong)
        .wrapping_mul(8i32 as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong))
  } as size_t;
  the_module = xmalloc_open_zipped_read_close(path, &mut len) as *mut libc::c_char;
  if the_module.is_null() {
    if *path.offset(0) as libc::c_int == '/' as i32 {
      return;
    }
    /* Newer depmod puts relative paths in modules.dep */
    allocated = xasprintf(
      b"%s/%s/%s\x00" as *const u8 as *const libc::c_char,
      b"/lib/modules\x00" as *const u8 as *const libc::c_char,
      version,
      path,
    );
    path = allocated;
    the_module = xmalloc_open_zipped_read_close(path, &mut len) as *mut libc::c_char;
    if the_module.is_null() {
      bb_error_msg(
        b"module \'%s\' not found\x00" as *const u8 as *const libc::c_char,
        path,
      );
      current_block = 3153887004257281803;
    } else {
      current_block = 1856101646708284338;
    }
  } else {
    current_block = 1856101646708284338;
  }
  match current_block {
    1856101646708284338 => {
      j = 1i32;
      while 1i32 << j & (OPT_TAGS as libc::c_int | OPT_F as libc::c_int) != 0 {
        let mut pattern: *const libc::c_char = 0 as *const libc::c_char;
        if !(1i32 << j & tags == 0) {
          pattern = field;
          if 1i32 << j & OPT_TAGS as libc::c_int != 0 {
            pattern = shortcuts[(j - 2i32) as usize]
          }
          if strcmp(pattern, shortcuts[0]) == 0i32 {
            /* "-n" or "-F filename" */
            display(path, shortcuts[0]);
          } else {
            ptr = the_module;
            loop {
              let mut after_pattern: *mut libc::c_char = 0 as *mut libc::c_char;
              ptr = memchr(
                ptr as *const libc::c_void,
                *pattern as libc::c_int,
                len.wrapping_sub(
                  ptr.wrapping_offset_from(the_module) as libc::c_long as libc::c_ulong
                ),
              ) as *mut libc::c_char;
              if ptr.is_null() {
                break;
              }
              after_pattern = is_prefixed_with(ptr, pattern);
              if !after_pattern.is_null() && *after_pattern as libc::c_int == '=' as i32 {
                /* field prefixes are 0x80 or 0x00 */
                if *ptr.offset(-1i32 as isize) as libc::c_int & 0x7fi32 == 0i32 {
                  ptr = after_pattern.offset(1);
                  display(ptr, pattern);
                  ptr = ptr.offset(strlen(ptr) as isize)
                }
              }
              ptr = ptr.offset(1)
            }
          }
        }
        j += 1
      }
      free(the_module as *mut libc::c_void);
    }
    _ => {}
  }
  free(allocated as *mut libc::c_void);
}
//usage:#define modinfo_trivial_usage
//usage:       "[-adlpn0] [-F keyword] MODULE"
//usage:#define modinfo_full_usage "\n\n"
//usage:       "	-a		Shortcut for '-F author'"
//usage:     "\n	-d		Shortcut for '-F description'"
//usage:     "\n	-l		Shortcut for '-F license'"
//usage:     "\n	-p		Shortcut for '-F parm'"
// //usage:     "\n	-n		Shortcut for '-F filename'"
//usage:     "\n	-F keyword	Keyword to look for"
//usage:     "\n	-0		Separate output with NULs"
//usage:#define modinfo_example_usage
//usage:       "$ modinfo -F vermagic loop\n"
#[no_mangle]
pub unsafe extern "C" fn modinfo_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut field: *const libc::c_char = 0 as *const libc::c_char;
  let mut name: [libc::c_char; 256] = [0; 256];
  let mut uts: utsname = utsname {
    sysname: [0; 65],
    nodename: [0; 65],
    release: [0; 65],
    version: [0; 65],
    machine: [0; 65],
    domainname: [0; 65],
  };
  let mut parser: *mut parser_t = 0 as *mut parser_t;
  let mut colon: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut tokens: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
  let mut opts: libc::c_uint = 0;
  let mut i: libc::c_uint = 0;
  field = 0 as *const libc::c_char;
  opts = getopt32(
    argv,
    b"^0F:nadlp\x00-1\x00" as *const u8 as *const libc::c_char,
    &mut field as *mut *const libc::c_char,
  );
  /* If no field selected, show all */
  if opts & (OPT_TAGS as libc::c_int | OPT_F as libc::c_int) as libc::c_uint == 0 {
    option_mask32 |= OPT_TAGS as libc::c_int as libc::c_uint
  }
  argv = argv.offset(optind as isize);
  uname(&mut uts);
  parser = config_open2(
    xasprintf(
      b"%s/%s/%s\x00" as *const u8 as *const libc::c_char,
      b"/lib/modules\x00" as *const u8 as *const libc::c_char,
      uts.release.as_mut_ptr(),
      b"modules.dep\x00" as *const u8 as *const libc::c_char,
    ),
    Some(xfopen_for_read as unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE),
  );
  while config_read(
    parser,
    tokens.as_mut_ptr(),
    (PARSE_NORMAL as libc::c_int | (1i32 & 0xffi32) << 8i32 | 2i32 & 0xffi32) as libc::c_uint,
    b"# \t\x00" as *const u8 as *const libc::c_char,
  ) != 0
  {
    colon = last_char_is(tokens[0], ':' as i32);
    if colon.is_null() {
      continue;
    }
    *colon = '\u{0}' as i32 as libc::c_char;
    filename2modname(bb_basename(tokens[0]), name.as_mut_ptr());
    i = 0i32 as libc::c_uint;
    while !(*argv.offset(i as isize)).is_null() {
      if fnmatch(*argv.offset(i as isize), name.as_mut_ptr(), 0i32) == 0i32 {
        modinfo(tokens[0], uts.release.as_mut_ptr(), field);
        let ref mut fresh1 = *argv.offset(i as isize);
        *fresh1 = b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      }
      i = i.wrapping_add(1)
    }
  }
  i = 0i32 as libc::c_uint;
  while !(*argv.offset(i as isize)).is_null() {
    if *(*argv.offset(i as isize)).offset(0) != 0 {
      modinfo(*argv.offset(i as isize), uts.release.as_mut_ptr(), field);
    }
    i = i.wrapping_add(1)
  }
  return 0i32;
}
