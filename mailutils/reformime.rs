use crate::librb::signal::__sighandler_t;
use crate::librb::size_t;
use libc;
use libc::pid_t;
use libc::FILE;

extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn getpid() -> pid_t;
  #[no_mangle]
  static ptr_to_globals: *mut globals;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
  #[no_mangle]
  static mut stdin: *mut FILE;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn monotonic_us() -> libc::c_ulonglong;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_get_last_path_component_strip(path: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn xsetenv(key: *const libc::c_char, value: *const libc::c_char);
  #[no_mangle]
  fn xpipe(filedes: *mut libc::c_int);
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc_fgets(file: *mut FILE) -> *mut libc::c_char;
  #[no_mangle]
  fn xfopen_for_write(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn xfdopen_for_write(fd: libc::c_int) -> *mut FILE;
  #[no_mangle]
  fn BB_EXECVP_or_die(argv: *mut *mut libc::c_char) -> !;
  #[no_mangle]
  fn wait4pid(pid: pid_t) -> libc::c_int;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn read_base64(src_stream: *mut FILE, dst_stream: *mut FILE, flags: libc::c_int);
  #[no_mangle]
  fn vfork() -> libc::c_int;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub helper_pid: pid_t,
  pub timeout: libc::c_uint,
  pub verbose: libc::c_uint,
  pub opts: libc::c_uint,
  pub user: *mut libc::c_char,
  pub pass: *mut libc::c_char,
  pub fp0: *mut FILE,
  pub opt_charset: *mut libc::c_char,
}
pub type C2RustUnnamed = libc::c_uint;
pub const OPT_O: C2RustUnnamed = 2048;
pub const OPT_o: C2RustUnnamed = 1024;
pub const OPT_h: C2RustUnnamed = 512;
pub const OPT_m: C2RustUnnamed = 256;
pub const OPT_c: C2RustUnnamed = 128;
pub const OPT_r: C2RustUnnamed = 64;
pub const OPT_s: C2RustUnnamed = 32;
pub const OPT_i: C2RustUnnamed = 16;
pub const OPT_e: C2RustUnnamed = 8;
pub const OPT_d: C2RustUnnamed = 4;
pub const OPT_X: C2RustUnnamed = 2;
pub const OPT_x: C2RustUnnamed = 1;
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}

/*
 * reformime: parse MIME-encoded message
 *
 * Copyright (C) 2008 by Vladimir Dronnikov <dronnikov@gmail.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config REFORMIME
//config:	bool "reformime (7.5 kb)"
//config:	default y
//config:	help
//config:	Parse MIME-formatted messages.
//config:
//config:config FEATURE_REFORMIME_COMPAT
//config:	bool "Accept and ignore options other than -x and -X"
//config:	default y
//config:	depends on REFORMIME
//config:	help
//config:	Accept (for compatibility only) and ignore options
//config:	other than -x and -X.
//applet:IF_REFORMIME(APPLET(reformime, BB_DIR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_REFORMIME) += reformime.o mail.o
unsafe extern "C" fn find_token(
  mut string_array: *const *const libc::c_char,
  mut key: *const libc::c_char,
  mut defvalue: *const libc::c_char,
) -> *const libc::c_char {
  let mut r: *const libc::c_char = 0 as *const libc::c_char;
  let mut i: libc::c_int = 0;
  i = 0i32;
  while !(*string_array.offset(i as isize)).is_null() {
    if strcasecmp(*string_array.offset(i as isize), key) == 0i32 {
      r = *string_array.offset((i + 1i32) as isize) as *mut libc::c_char;
      break;
    } else {
      i += 1
    }
  }
  return if !r.is_null() { r } else { defvalue };
}
unsafe extern "C" fn xfind_token(
  mut string_array: *const *const libc::c_char,
  mut key: *const libc::c_char,
) -> *const libc::c_char {
  let mut r: *const libc::c_char = find_token(string_array, key, 0 as *const libc::c_char);
  if !r.is_null() {
    return r;
  }
  bb_error_msg_and_die(
    b"not found: \'%s\'\x00" as *const u8 as *const libc::c_char,
    key,
  );
}
unsafe extern "C" fn parse(
  mut boundary: *const libc::c_char,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut boundary_len: libc::c_int = strlen(boundary) as libc::c_int;
  let mut uniq: [libc::c_char; 21] = [0; 21];
  // prepare unique string pattern
  sprintf(
    uniq.as_mut_ptr(),
    b"%%llu.%u\x00" as *const u8 as *const libc::c_char,
    getpid() as libc::c_uint,
  ); /* while (1) */
  loop {
    let mut header: *mut libc::c_char = 0 as *mut libc::c_char; /* 32 is enough */
    let mut tokens: [*const libc::c_char; 32] = [0 as *const libc::c_char; 32];
    let mut type_0: *const libc::c_char = 0 as *const libc::c_char;
    /* Read the header (everything up to two \n) */
    let mut header_idx: libc::c_uint = 0i32 as libc::c_uint;
    let mut last_ch: libc::c_int = 0i32;
    header = 0 as *mut libc::c_char;
    loop
    /* Support both line endings */
    {
      let mut ch: libc::c_int = getc_unlocked(stdin);
      if ch == '\r' as i32 {
        continue;
      }
      if ch == -1i32 {
        break;
      }
      if ch == '\n' as i32 && last_ch == ch {
        break;
      }
      if header_idx & 0xffi32 as libc::c_uint == 0 {
        header = xrealloc(
          header as *mut libc::c_void,
          header_idx.wrapping_add(0x101i32 as libc::c_uint) as size_t,
        ) as *mut libc::c_char
      }
      last_ch = ch;
      let fresh0 = header_idx;
      header_idx = header_idx.wrapping_add(1);
      *header.offset(fresh0 as isize) = last_ch as libc::c_char
    }
    if header.is_null() {
      break;
    }
    *header.offset(header_idx as isize) = '\u{0}' as i32 as libc::c_char;
    /* Split to tokens */
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ntokens: libc::c_uint = 0;
    let mut delims: *const libc::c_char = b";=\" \t\n\x00" as *const u8 as *const libc::c_char;
    /* Skip to last Content-Type: */
    p = header;
    s = p;
    loop {
      p = strchr(p, '\n' as i32);
      if p.is_null() {
        break;
      }
      p = p.offset(1);
      if strncasecmp(
        p,
        b"Content-Type:\x00" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong),
      ) == 0i32
      {
        s = p
      }
    }
    ntokens = 0i32 as libc::c_uint;
    s = strtok(s, delims);
    while !s.is_null() {
      tokens[ntokens as usize] = s;
      if ntokens
        < ((::std::mem::size_of::<[*const libc::c_char; 32]>() as libc::c_ulong)
          .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
          as libc::c_uint)
          .wrapping_sub(1i32 as libc::c_uint)
      {
        ntokens = ntokens.wrapping_add(1)
      }
      s = strtok(0 as *mut libc::c_char, delims)
    }
    tokens[ntokens as usize] = 0 as *const libc::c_char;
    if ntokens == 0i32 as libc::c_uint {
      break;
    }
    /* Is it multipart? */
    type_0 = find_token(
      tokens.as_mut_ptr() as *const *const libc::c_char,
      b"Content-Type:\x00" as *const u8 as *const libc::c_char,
      b"text/plain\x00" as *const u8 as *const libc::c_char,
    ); /* end of "handle one non-multipart block" */
    if 0i32
      == strncasecmp(
        type_0,
        b"multipart/\x00" as *const u8 as *const libc::c_char,
        10i32 as libc::c_ulong,
      )
    {
      /* Yes, recurse */
      if strcasecmp(
        type_0.offset(10),
        b"mixed\x00" as *const u8 as *const libc::c_char,
      ) != 0i32
      {
        bb_error_msg_and_die(
          b"no support of content type \'%s\'\x00" as *const u8 as *const libc::c_char,
          type_0,
        );
      }
      parse(
        xfind_token(
          tokens.as_mut_ptr() as *const *const libc::c_char,
          b"boundary\x00" as *const u8 as *const libc::c_char,
        ),
        argv,
      );
    } else {
      /* No, process one non-multipart section */
      let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
      let mut pid: pid_t = 0;
      pid = pid;
      let mut fp: *mut FILE = 0 as *mut FILE;
      let mut charset: *const libc::c_char = find_token(
        tokens.as_mut_ptr() as *const *const libc::c_char,
        b"charset\x00" as *const u8 as *const libc::c_char,
        b"us-ascii\x00" as *const u8 as *const libc::c_char,
      );
      let mut encoding: *const libc::c_char = find_token(
        tokens.as_mut_ptr() as *const *const libc::c_char,
        b"Content-Transfer-Encoding:\x00" as *const u8 as *const libc::c_char,
        b"7bit\x00" as *const u8 as *const libc::c_char,
      );
      /* Compose target filename */
      let mut filename: *mut libc::c_char = find_token(
        tokens.as_mut_ptr() as *const *const libc::c_char,
        b"filename\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
      ) as *mut libc::c_char;
      if filename.is_null() {
        filename = xasprintf(uniq.as_mut_ptr(), monotonic_us())
      } else {
        filename = bb_get_last_path_component_strip(xstrdup(filename))
      }
      if (*ptr_to_globals).opts & OPT_X as libc::c_int as libc::c_uint != 0 {
        let mut fd: [libc::c_int; 2] = [0; 2];
        /* start external helper */
        xpipe(fd.as_mut_ptr());
        pid = vfork();
        if 0i32 == pid {
          /* child reads from fd[0] */
          close(fd[1]);
          xmove_fd(fd[0], 0i32);
          xsetenv(
            b"CONTENT_TYPE\x00" as *const u8 as *const libc::c_char,
            type_0,
          );
          xsetenv(b"CHARSET\x00" as *const u8 as *const libc::c_char, charset);
          xsetenv(
            b"ENCODING\x00" as *const u8 as *const libc::c_char,
            encoding,
          );
          xsetenv(
            b"FILENAME\x00" as *const u8 as *const libc::c_char,
            filename,
          );
          BB_EXECVP_or_die(argv);
        }
        /* parent will write to fd[1] */
        close(fd[0]);
        fp = xfdopen_for_write(fd[1]);
        signal(
          13i32,
          ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
        );
      } else {
        /* write to file */
        let mut fname: *mut libc::c_char = xasprintf(
          b"%s%s\x00" as *const u8 as *const libc::c_char,
          *argv,
          filename,
        );
        fp = xfopen_for_write(fname);
        free(fname as *mut libc::c_void);
      }
      free(filename as *mut libc::c_void);
      /* write to fp */
      end = 0 as *mut libc::c_char;
      if 0i32 == strcasecmp(encoding, b"base64\x00" as *const u8 as *const libc::c_char) {
        read_base64(stdin, fp, '-' as i32);
      } else if 0i32 != strcasecmp(encoding, b"7bit\x00" as *const u8 as *const libc::c_char)
        && 0i32 != strcasecmp(encoding, b"8bit\x00" as *const u8 as *const libc::c_char)
      {
        /* quoted-printable, binary, user-defined are unsupported so far */
        bb_error_msg_and_die(
          b"encoding \'%s\' not supported\x00" as *const u8 as *const libc::c_char,
          encoding,
        );
      } else {
        loop
        /* plain 7bit or 8bit */
        {
          end = xmalloc_fgets(stdin);
          if end.is_null() {
            break;
          }
          if '-' as i32 == *end.offset(0) as libc::c_int
            && '-' as i32 == *end.offset(1) as libc::c_int
            && strncmp(end.offset(2), boundary, boundary_len as libc::c_ulong) == 0i32
          {
            break;
          }
          fputs_unlocked(end, fp);
        }
      }
      fclose(fp);
      /* Wait for child */
      if (*ptr_to_globals).opts & OPT_X as libc::c_int as libc::c_uint != 0 {
        let mut rc: libc::c_int = 0;
        signal(13i32, None);
        rc = wait4pid(pid) & 0xffi32;
        if rc != 0i32 {
          return rc + 20i32;
        }
      }
      /* Multipart ended? */
      if !end.is_null()
        && '-' as i32 == *end.offset((2i32 + boundary_len) as isize) as libc::c_int
        && '-' as i32 == *end.offset((2i32 + boundary_len + 1i32) as isize) as libc::c_int
      {
        break;
      }
      free(end as *mut libc::c_void);
    }
    free(header as *mut libc::c_void);
  }
  return 0i32;
}
//usage:#define reformime_trivial_usage
//usage:       "[OPTIONS]"
//usage:#define reformime_full_usage "\n\n"
//usage:       "Parse MIME-encoded message on stdin\n"
//usage:     "\n	-x PREFIX	Extract content of MIME sections to files"
//usage:     "\n	-X PROG ARGS	Filter content of MIME sections through PROG"
//usage:     "\n			Must be the last option"
//usage:     "\n"
//usage:     "\nOther options are silently ignored"
/*
Usage: reformime [options]
    -d - parse a delivery status notification.
    -e - extract contents of MIME section.
    -x - extract MIME section to a file.
    -X - pipe MIME section to a program.
    -i - show MIME info.
    -s n.n.n.n - specify MIME section.
    -r - rewrite message, filling in missing MIME headers.
    -r7 - also convert 8bit/raw encoding to quoted-printable, if possible.
    -r8 - also convert quoted-printable encoding to 8bit, if possible.
    -c charset - default charset for rewriting, -o, and -O.
    -m [file] [file]... - create a MIME message digest.
    -h "header" - decode RFC 2047-encoded header.
    -o "header" - encode unstructured header using RFC 2047.
    -O "header" - encode address list header using RFC 2047.
*/
#[no_mangle]
pub unsafe extern "C" fn reformime_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opt_prefix: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
  let ref mut fresh1 = *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
    as *mut *mut globals);
  *fresh1 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  (*ptr_to_globals).opt_charset =
    b"us-ascii\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
  // parse options
  // N.B. only -x and -X are supported so far
  (*ptr_to_globals).opts = getopt32(
    argv,
    b"^x:Xdeis:r:c:m:*h:o:O:\x00x--X:X--x\x00" as *const u8 as *const libc::c_char,
    &mut opt_prefix as *mut *const libc::c_char,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
    &mut (*ptr_to_globals).opt_charset as *mut *mut libc::c_char,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
  );
  argv = argv.offset(optind as isize);
  return parse(
    b"\x00" as *const u8 as *const libc::c_char,
    if (*ptr_to_globals).opts & OPT_X as libc::c_int as libc::c_uint != 0 {
      argv
    } else {
      &mut opt_prefix as *mut *const libc::c_char as *mut *mut libc::c_char
    },
  );
}
