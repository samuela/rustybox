use libc;





























use libc::printf;












use libc::free;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut stdout: *mut FILE;

  #[no_mangle]
  fn putc_unlocked(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn strcpy_and_process_escape_sequences(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc_fgetline(file: *mut FILE) -> *mut libc::c_char;
  #[no_mangle]
  fn fflush_stdout_and_exit(retval: libc::c_int) -> !;
  #[no_mangle]
  fn fclose_if_not_stdin(file: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fopen_or_warn_stdin(filename: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn xfunc_die() -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
}



use libc::FILE;

/*
 * paste.c - implementation of the posix paste command
 *
 * Written by Maxime Coste <mawww@kakoune.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config PASTE
//config:	bool "paste (4.9 kb)"
//config:	default y
//config:	help
//config:	paste is used to paste lines of different files together
//config:	and write the result to stdout
//applet:IF_PASTE(APPLET_NOEXEC(paste, paste, BB_DIR_USR_BIN, BB_SUID_DROP, paste))
//kbuild:lib-$(CONFIG_PASTE) += paste.o
//usage:#define paste_trivial_usage
//usage:       "[OPTIONS] [FILE]..."
//usage:#define paste_full_usage "\n\n"
//usage:       "Paste lines from each input file, separated with tab\n"
//usage:     "\n	-d LIST	Use delimiters from LIST, not tab"
//usage:     "\n	-s      Serial: one file at a time"
//usage:
//usage:#define paste_example_usage
//usage:       "# write out directory in four columns\n"
//usage:       "$ ls | paste - - - -\n"
//usage:       "# combine pairs of lines from a file into single lines\n"
//usage:       "$ paste -s -d '\\t\\n' file\n"
unsafe extern "C" fn paste_files(
  mut files: *mut *mut FILE,
  mut file_cnt: libc::c_int,
  mut delims: *mut libc::c_char,
  mut del_cnt: libc::c_int,
) {
  let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut delim: libc::c_char = 0;
  let mut active_files: libc::c_int = file_cnt;
  let mut i: libc::c_int = 0;
  while active_files > 0i32 {
    let mut del_idx: libc::c_int = 0i32;
    i = 0i32;
    while i < file_cnt {
      if !(*files.offset(i as isize)).is_null() {
        line = xmalloc_fgetline(*files.offset(i as isize));
        if line.is_null() {
          fclose_if_not_stdin(*files.offset(i as isize));
          let ref mut fresh0 = *files.offset(i as isize);
          *fresh0 = 0 as *mut FILE;
          active_files -= 1
        } else {
          fputs_unlocked(line, stdout);
          free(line as *mut libc::c_void);
          delim = '\n' as i32 as libc::c_char;
          if i != file_cnt - 1i32 {
            let fresh1 = del_idx;
            del_idx = del_idx + 1;
            delim = *delims.offset(fresh1 as isize);
            if del_idx == del_cnt {
              del_idx = 0i32
            }
          }
          if delim as libc::c_int != '\u{0}' as i32 {
            putc_unlocked(delim as libc::c_int, stdout);
          }
        }
      }
      i += 1
    }
  }
}
unsafe extern "C" fn paste_files_separate(
  mut files: *mut *mut FILE,
  mut delims: *mut libc::c_char,
  mut del_cnt: libc::c_int,
) {
  let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut next_line: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut delim: libc::c_char = 0;
  let mut i: libc::c_int = 0;
  i = 0i32;
  while !(*files.offset(i as isize)).is_null() {
    let mut del_idx: libc::c_int = 0i32;
    line = 0 as *mut libc::c_char;
    loop {
      next_line = xmalloc_fgetline(*files.offset(i as isize));
      if next_line.is_null() {
        break;
      }
      if !line.is_null() {
        fputs_unlocked(line, stdout);
        free(line as *mut libc::c_void);
        let fresh2 = del_idx;
        del_idx = del_idx + 1;
        delim = *delims.offset(fresh2 as isize);
        if del_idx == del_cnt {
          del_idx = 0i32
        }
        if delim as libc::c_int != '\u{0}' as i32 {
          putc_unlocked(delim as libc::c_int, stdout);
        }
      }
      line = next_line
    }
    if !line.is_null() {
      /* coreutils adds \n even if this is a final line
       * of the last file and it was not \n-terminated.
       */
      printf(b"%s\n\x00" as *const u8 as *const libc::c_char, line);
      free(line as *mut libc::c_void);
    }
    fclose_if_not_stdin(*files.offset(i as isize));
    i += 1
  }
}
#[no_mangle]
pub unsafe extern "C" fn paste_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut delims: *mut libc::c_char =
    b"\t\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
  let mut del_cnt: libc::c_int = 1i32;
  let mut opt: libc::c_uint = 0;
  let mut i: libc::c_int = 0;
  opt = getopt32(
    argv,
    b"d:s\x00" as *const u8 as *const libc::c_char,
    &mut delims as *mut *mut libc::c_char,
  );
  argv = argv.offset(optind as isize);
  if opt & (1i32 << 0i32) as libc::c_uint != 0 {
    if *delims.offset(0) == 0 {
      bb_simple_error_msg_and_die(
        b"-d \'\' is not supported\x00" as *const u8 as *const libc::c_char,
      );
    }
    /* note: handle NUL properly (do not stop at it!): try -d'\t\0\t' */
    del_cnt = strcpy_and_process_escape_sequences(delims, delims).wrapping_offset_from(delims)
      as libc::c_long as libc::c_int
  }
  if (*argv.offset(0)).is_null() {
    argv = argv.offset(-1);
    let ref mut fresh3 = *argv.offset(0);
    *fresh3 = b"-\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
  }
  i = 0i32;
  while !(*argv.offset(i as isize)).is_null() {
    let ref mut fresh4 = *argv.offset(i as isize);
    *fresh4 =
      fopen_or_warn_stdin(*argv.offset(i as isize)) as *mut libc::c_void as *mut libc::c_char;
    if (*argv.offset(i as isize)).is_null() {
      xfunc_die();
    }
    i += 1
  }
  if opt & (1i32 << 1i32) as libc::c_uint != 0 {
    paste_files_separate(argv as *mut *mut FILE, delims, del_cnt);
  } else {
    paste_files(argv as *mut *mut FILE, i, delims, del_cnt);
  }
  fflush_stdout_and_exit(0i32);
}
/* unknown mappings are not changed: "\z" -> '\\' 'z' */
/* trailing backslash, if any, is preserved */
