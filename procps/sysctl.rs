use libc;
extern "C" {
  pub type __dirstream;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn opendir(__name: *const libc::c_char) -> *mut DIR;
  #[no_mangle]
  fn closedir(__dirp: *mut DIR) -> libc::c_int;
  #[no_mangle]
  fn readdir(__dirp: *mut DIR) -> *mut dirent;
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  static mut stdout: *mut FILE;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn putc_unlocked(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;

  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn trim(s: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xchdir(path: *const libc::c_char);
  #[no_mangle]
  fn xfchdir(fd: libc::c_int);
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn overlapping_strcpy(dst: *mut libc::c_char, src: *const libc::c_char);
  #[no_mangle]
  fn xmalloc_read(fd: libc::c_int, maxsz_p: *mut size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xwrite_str(fd: libc::c_int, str: *const libc::c_char);
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn config_open(filename: *const libc::c_char) -> *mut parser_t;
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
  fn concat_subpath_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;
}

use libc::ino64_t;

use crate::librb::__off64_t;

use crate::librb::size_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
  pub d_ino: ino64_t,
  pub d_off: __off64_t,
  pub d_reclen: libc::c_ushort,
  pub d_type: libc::c_uchar,
  pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
use libc::stat;


use libc::FILE;
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

/*
 * Sysctl 1.01 - A utility to read and manipulate the sysctl parameters
 *
 * Copyright 1999 George Staikos
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 *
 * Changelog:
 * v1.01   - added -p <preload> to preload values from a file
 * v1.01.1 - busybox applet aware by <solar@gentoo.org>
 */
//config:config BB_SYSCTL
//config:	bool "sysctl (7.4 kb)"
//config:	default y
//config:	help
//config:	Configure kernel parameters at runtime.
//applet:IF_BB_SYSCTL(APPLET_NOEXEC(sysctl, sysctl, BB_DIR_SBIN, BB_SUID_DROP, sysctl))
//kbuild:lib-$(CONFIG_BB_SYSCTL) += sysctl.o
//usage:#define sysctl_trivial_usage
//usage:       "-p [-enq] [FILE...] / [-enqaw] [KEY[=VALUE]]..."
//usage:#define sysctl_full_usage "\n\n"
//usage:       "Show/set kernel parameters\n"
//usage:     "\n	-p	Set values from FILEs (default /etc/sysctl.conf)"
//usage:     "\n	-e	Don't warn about unknown keys"
//usage:     "\n	-n	Don't show key names"
//usage:     "\n	-q      Quiet"
//usage:     "\n	-a	Show all values"
/* Same as -a, no need to show it */
/* //usage:     "\n	-A	Show all values in table form" */
//usage:     "\n	-w	Set values"
//usage:
//usage:#define sysctl_example_usage
//usage:       "sysctl [-n] [-e] variable...\n"
//usage:       "sysctl [-n] [-e] [-q] -w variable=value...\n"
//usage:       "sysctl [-n] [-e] -a\n"
//usage:       "sysctl [-n] [-e] [-q] -p file	(default /etc/sysctl.conf)\n"
//usage:       "sysctl [-n] [-e] -A\n"
pub type C2RustUnnamed_0 = libc::c_uint;
pub const FLAG_QUIET: C2RustUnnamed_0 = 64;
/* NB: procps 3.2.8 does not require -w for KEY=VAL to work, it only rejects non-KEY=VAL form */
pub const FLAG_WRITE: C2RustUnnamed_0 = 32;
pub const FLAG_PRELOAD_FILE: C2RustUnnamed_0 = 16;
/* not implemented */
pub const FLAG_SHOW_ALL: C2RustUnnamed_0 = 8;
pub const FLAG_TABLE_FORMAT: C2RustUnnamed_0 = 4;
pub const FLAG_SHOW_KEY_ERRORS: C2RustUnnamed_0 = 2;
pub const FLAG_SHOW_KEYS: C2RustUnnamed_0 = 1;
unsafe extern "C" fn sysctl_dots_to_slashes(mut name: *mut libc::c_char) {
  let mut cptr: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut last_good: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut end_ch: libc::c_char = 0;
  end = strchrnul(name, '=' as i32);
  slash = strchrnul(name, '/' as i32);
  if slash < end && strchrnul(name, '.' as i32) < slash {
    /* There are both dots and slashes, and 1st dot is
     * before 1st slash.
     * (IOW: not raw, unmangled a/b/c.d format)
     *
     * procps supports this syntax for names with dots:
     *  net.ipv4.conf.eth0/100.mc_forwarding
     * (dots and slashes are simply swapped)
     */
    while end != name {
      end = end.offset(-1);
      if *end as libc::c_int == '.' as i32 {
        *end = '/' as i32 as libc::c_char
      } else if *end as libc::c_int == '/' as i32 {
        *end = '.' as i32 as libc::c_char
      }
    }
    return;
  }
  /* else: use our old behavior: */
  /* Convert minimum number of '.' to '/' so that
   * we end up with existing file's name.
   *
   * Example from bug 3894:
   * net.ipv4.conf.eth0.100.mc_forwarding ->
   * net/ipv4/conf/eth0.100/mc_forwarding
   * NB: net/ipv4/conf/eth0/mc_forwarding *also exists*,
   * therefore we must start from the end, and if
   * we replaced even one . -> /, start over again,
   * but never replace dots before the position
   * where last replacement occurred.
   *
   * Another bug we later had is that
   * net.ipv4.conf.eth0.100
   * (without .mc_forwarding) was mishandled.
   *
   * To set up testing: modprobe 8021q; vconfig add eth0 100
   */
  end_ch = *end; /* trick the loop into trying full name too */
  *end = '.' as i32 as libc::c_char;
  last_good = name.offset(-1);
  'c_8722: loop {
    cptr = end;
    loop {
      if !(cptr > last_good) {
        break 'c_8722;
      }
      if *cptr as libc::c_int == '.' as i32 {
        *cptr = '\u{0}' as i32 as libc::c_char;
        //bb_error_msg("trying:'%s'", name);
        if access(name, 0i32) == 0i32 {
          *cptr = '/' as i32 as libc::c_char;
          //bb_error_msg("replaced:'%s'", name);
          last_good = cptr; /* for compiler */
          break; /* point to the value in name=value */
        } else {
          *cptr = '.' as i32 as libc::c_char
        }
      }
      cptr = cptr.offset(-1)
    }
  }
  *end = end_ch;
}
unsafe extern "C" fn sysctl_act_on_setting(mut setting: *mut libc::c_char) -> libc::c_int {
  let mut current_block: u64;
  let mut fd: libc::c_int = 0;
  let mut retval: libc::c_int = 0i32;
  let mut cptr: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut outname: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
  value = value;
  let mut writing: bool = option_mask32 & FLAG_WRITE as libc::c_int as libc::c_uint != 0;
  outname = xstrdup(setting);
  cptr = outname;
  while *cptr != 0 {
    if *cptr as libc::c_int == '/' as i32 {
      *cptr = '.' as i32 as libc::c_char
    } else if *cptr as libc::c_int == '.' as i32 {
      *cptr = '/' as i32 as libc::c_char
    }
    cptr = cptr.offset(1)
  }
  cptr = strchr(setting, '=' as i32);
  if !cptr.is_null() {
    writing = 1i32 != 0
  }
  if writing {
    if cptr.is_null() {
      bb_error_msg(
        b"error: \'%s\' must be of the form name=value\x00" as *const u8 as *const libc::c_char,
        outname,
      );
      retval = 1i32;
      current_block = 14594744208347999533;
    } else {
      value = cptr.offset(1);
      if setting == cptr {
        /* "name" can't be empty */
        /* || !*value - WRONG: "sysctl net.ipv4.ip_local_reserved_ports=" is a valid syntax (clears the value) */
        bb_error_msg(
          b"error: malformed setting \'%s\'\x00" as *const u8 as *const libc::c_char,
          outname,
        );
        retval = 1i32;
        current_block = 14594744208347999533;
      } else {
        *cptr = '\u{0}' as i32 as libc::c_char;
        *outname.offset(cptr.wrapping_offset_from(setting) as libc::c_long as isize) =
          '\u{0}' as i32 as libc::c_char;
        /* procps 3.2.7 actually uses these flags */
        fd = open(setting, 0o1i32 | 0o100i32 | 0o1000i32, 0o666i32);
        current_block = 4068382217303356765;
      }
    }
  } else {
    fd = open(setting, 0i32);
    current_block = 4068382217303356765;
  }
  match current_block {
    4068382217303356765 => {
      if fd < 0i32 {
        match *bb_errno {
          13 => {}
          2 => {
            current_block = 4227907091559855388;
            match current_block {
              8559883396898198220 => {
                bb_perror_msg(
                  b"error %sing key \'%s\'\x00" as *const u8 as *const libc::c_char,
                  if writing as libc::c_int != 0 {
                    b"sett\x00" as *const u8 as *const libc::c_char
                  } else {
                    b"read\x00" as *const u8 as *const libc::c_char
                  },
                  outname,
                );
              }
              _ => {
                if option_mask32 & FLAG_SHOW_KEY_ERRORS as libc::c_int as libc::c_uint != 0 {
                  bb_error_msg(
                    b"error: \'%s\' is an unknown key\x00" as *const u8 as *const libc::c_char,
                    outname,
                  );
                }
              }
            }
            retval = 1i32
          }
          _ => {
            current_block = 8559883396898198220;
            match current_block {
              8559883396898198220 => {
                bb_perror_msg(
                  b"error %sing key \'%s\'\x00" as *const u8 as *const libc::c_char,
                  if writing as libc::c_int != 0 {
                    b"sett\x00" as *const u8 as *const libc::c_char
                  } else {
                    b"read\x00" as *const u8 as *const libc::c_char
                  },
                  outname,
                );
              }
              _ => {
                if option_mask32 & FLAG_SHOW_KEY_ERRORS as libc::c_int as libc::c_uint != 0 {
                  bb_error_msg(
                    b"error: \'%s\' is an unknown key\x00" as *const u8 as *const libc::c_char,
                    outname,
                  );
                }
              }
            }
            retval = 1i32
          }
        }
      } else if writing {
        //TODO: procps 3.2.7 writes "value\n", note trailing "\n"
        xwrite_str(fd, value);
        close(fd);
        if option_mask32 & FLAG_QUIET as libc::c_int as libc::c_uint == 0 {
          if option_mask32 & FLAG_SHOW_KEYS as libc::c_int as libc::c_uint != 0 {
            printf(b"%s = \x00" as *const u8 as *const libc::c_char, outname);
          }
          puts(value);
        }
      } else {
        let mut c: libc::c_char = 0;
        cptr = xmalloc_read(fd, 0 as *mut size_t) as *mut libc::c_char;
        value = cptr;
        close(fd);
        if value.is_null() {
          bb_perror_msg(
            b"error reading key \'%s\'\x00" as *const u8 as *const libc::c_char,
            outname,
          );
          retval = 1i32
        } else {
          loop
          /* dev.cdrom.info and sunrpc.transports, for example,
           * are multi-line. Try "sysctl sunrpc.transports"
           */
          {
            c = *cptr;
            if !(c as libc::c_int != '\u{0}' as i32) {
              break;
            }
            if option_mask32 & FLAG_SHOW_KEYS as libc::c_int as libc::c_uint != 0 {
              printf(b"%s = \x00" as *const u8 as *const libc::c_char, outname);
            }
            loop {
              putc_unlocked(c as libc::c_int, stdout);
              cptr = cptr.offset(1);
              if c as libc::c_int == '\n' as i32 {
                break;
              }
              c = *cptr;
              if c as libc::c_int == '\u{0}' as i32 {
                break;
              }
            }
          }
          free(value as *mut libc::c_void);
        }
      }
    }
    _ => {}
  }
  /* Happens for write-only settings, e.g. net.ipv6.route.flush */
  free(outname as *mut libc::c_void); /* d_name is "." or ".." */
  return retval;
}
unsafe extern "C" fn sysctl_act_recursive(mut path: *const libc::c_char) -> libc::c_int {
  let mut buf: stat = std::mem::zeroed();
  let mut retval: libc::c_int = 0i32;
  if option_mask32 & FLAG_WRITE as libc::c_int as libc::c_uint == 0
    && stat(path, &mut buf) == 0i32
    && buf.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint
  {
    let mut entry: *mut dirent = 0 as *mut dirent;
    let mut dirp: *mut DIR = 0 as *mut DIR;
    dirp = opendir(path);
    if dirp.is_null() {
      return -1i32;
    }
    loop {
      entry = readdir(dirp);
      if entry.is_null() {
        break;
      }
      let mut next: *mut libc::c_char = concat_subpath_file(path, (*entry).d_name.as_mut_ptr());
      if next.is_null() {
        continue;
      }
      /* if path was ".", drop "./" prefix: */
      retval |= sysctl_act_recursive(
        if *next.offset(0) as libc::c_int == '.' as i32
          && *next.offset(1) as libc::c_int == '/' as i32
        {
          next.offset(2)
        } else {
          next
        },
      );
      free(next as *mut libc::c_void);
    }
    closedir(dirp);
  } else {
    let mut name: *mut libc::c_char = xstrdup(path);
    retval |= sysctl_act_on_setting(name);
    free(name as *mut libc::c_void);
  }
  return retval;
}
/* Set sysctl's from a conf file. Format example:
 * # Controls IP packet forwarding
 * net.ipv4.ip_forward = 0
 */
unsafe extern "C" fn sysctl_handle_preload_file(mut filename: *const libc::c_char) -> libc::c_int {
  let mut token: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
  let mut parser: *mut parser_t = 0 as *mut parser_t;
  let mut parse_flags: libc::c_int = 0;
  parser = config_open(filename);
  /* Must do it _after_ config_open(): */
  xchdir(b"/proc/sys\x00" as *const u8 as *const libc::c_char); // NO (var==val is not var=val) - treat consecutive delimiters as one
  parse_flags = 0i32; // NO - trim leading and trailing delimiters
  parse_flags &= !(PARSE_COLLAPSE as libc::c_int); // YES - last token takes entire remainder of the line
  parse_flags &= !(PARSE_TRIM as libc::c_int); // NO - die if < min tokens found
  parse_flags |= PARSE_GREEDY as libc::c_int; // NO (only first char) - comments are recognized even if not first char
  parse_flags &= !(PARSE_MIN_DIE as libc::c_int); // YES - two comment chars: ';' and '#'
  parse_flags &= !(PARSE_EOL_COMMENTS as libc::c_int);
  parse_flags |= PARSE_ALT_COMMENTS as libc::c_int;
  /* <space><tab><space>#comment is also comment, not strictly 1st char only */
  parse_flags |= PARSE_WS_COMMENTS as libc::c_int; // YES - comments are recognized even if there is whitespace before
  while config_read(
    parser,
    token.as_mut_ptr(),
    (parse_flags | (2i32 & 0xffi32) << 8i32 | 2i32 & 0xffi32) as libc::c_uint,
    b";#=\x00" as *const u8 as *const libc::c_char,
  ) != 0
  {
    let mut tp: *mut libc::c_char = 0 as *mut libc::c_char;
    trim(token[1]);
    tp = trim(token[0]);
    sysctl_dots_to_slashes(token[0]);
    /* ^^^converted in-place. tp still points to NUL */
    /* now, add "=TOKEN1" */
    let fresh0 = tp; /* '+' - stop on first non-option */
    tp = tp.offset(1);
    *fresh0 = '=' as i32 as libc::c_char;
    overlapping_strcpy(tp, token[1]);
    sysctl_act_on_setting(token[0]);
  }
  return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn sysctl_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut retval: libc::c_int = 0;
  let mut opt: libc::c_int = 0;
  opt = getopt32(argv, b"+neAapwq\x00" as *const u8 as *const libc::c_char) as libc::c_int;
  argv = argv.offset(optind as isize);
  opt ^= FLAG_SHOW_KEYS as libc::c_int | FLAG_SHOW_KEY_ERRORS as libc::c_int;
  option_mask32 = opt as u32;
  if opt & FLAG_PRELOAD_FILE as libc::c_int != 0 {
    let mut cur_dir_fd: libc::c_int = 0;
    option_mask32 |= FLAG_WRITE as libc::c_int as libc::c_uint;
    if (*argv).is_null() {
      argv = argv.offset(-1);
      *argv = b"/etc/sysctl.conf\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    cur_dir_fd = xopen(
      b".\x00" as *const u8 as *const libc::c_char,
      0i32 | 0o200000i32,
    );
    loop {
      /* procps-ng 3.3.10 does not flag parse errors */
      /* xchdir("/proc/sys") is inside */
      sysctl_handle_preload_file(*argv);
      xfchdir(cur_dir_fd);
      argv = argv.offset(1);
      if (*argv).is_null() {
        break;
      }
      /* files can be relative, must restore cwd */
    }
    return 0i32;
  }
  xchdir(b"/proc/sys\x00" as *const u8 as *const libc::c_char);
  if opt & (FLAG_TABLE_FORMAT as libc::c_int | FLAG_SHOW_ALL as libc::c_int) != 0 {
    return sysctl_act_recursive(b".\x00" as *const u8 as *const libc::c_char);
  }
  //TODO: if(!argv[0]) bb_show_usage() ?
  retval = 0i32;
  while !(*argv).is_null() {
    sysctl_dots_to_slashes(*argv);
    retval |= sysctl_act_recursive(*argv);
    argv = argv.offset(1)
  }
  return retval;
}
