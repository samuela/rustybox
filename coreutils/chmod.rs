use libc;
extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
  #[no_mangle]
  fn bb_mode_string(mode: mode_t) -> *const libc::c_char;
  #[no_mangle]
  fn recursive_action(
    fileName: *const libc::c_char,
    flags: libc::c_uint,
    fileAction_0: Option<
      unsafe extern "C" fn(
        _: *const libc::c_char,
        _: *mut stat,
        _: *mut libc::c_void,
        _: libc::c_int,
      ) -> libc::c_int,
    >,
    dirAction: Option<
      unsafe extern "C" fn(
        _: *const libc::c_char,
        _: *mut stat,
        _: *mut libc::c_void,
        _: libc::c_int,
      ) -> libc::c_int,
    >,
    userData: *mut libc::c_void,
    depth: libc::c_uint,
  ) -> libc::c_int;
  #[no_mangle]
  static mut option_mask32: uint32_t;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_parse_mode(s: *const libc::c_char, cur_mode: libc::c_uint) -> libc::c_int;
}

use crate::librb::__mode_t;

use crate::librb::mode_t;
use crate::librb::stat;
use crate::librb::uint32_t;

/*
 * Mini chmod implementation for busybox
 *
 * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
 *
 * Reworked by (C) 2002 Vladimir Oleynik <dzo@simtreas.ru>
 *  to correctly parse '-rwxgoa'
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config CHMOD
//config:	bool "chmod (5.5 kb)"
//config:	default y
//config:	help
//config:	chmod is used to change the access permission of files.
//applet:IF_CHMOD(APPLET_NOEXEC(chmod, chmod, BB_DIR_BIN, BB_SUID_DROP, chmod))
//kbuild:lib-$(CONFIG_CHMOD) += chmod.o
/* BB_AUDIT SUSv3 compliant */
/* BB_AUDIT GNU defects - unsupported long options. */
/* http://www.opengroup.org/onlinepubs/007904975/utilities/chmod.html */
//usage:#define chmod_trivial_usage
//usage:       "[-R"IF_DESKTOP("cvf")"] MODE[,MODE]... FILE..."
//usage:#define chmod_full_usage "\n\n"
//usage:       "Each MODE is one or more of the letters ugoa, one of the\n"
//usage:       "symbols +-= and one or more of the letters rwxst\n"
//usage:     "\n	-R	Recurse"
//usage:	IF_DESKTOP(
//usage:     "\n	-c	List changed files"
//usage:     "\n	-v	List all files"
//usage:     "\n	-f	Hide errors"
//usage:	)
//usage:
//usage:#define chmod_example_usage
//usage:       "$ ls -l /tmp/foo\n"
//usage:       "-rw-rw-r--    1 root     root            0 Apr 12 18:25 /tmp/foo\n"
//usage:       "$ chmod u+x /tmp/foo\n"
//usage:       "$ ls -l /tmp/foo\n"
//usage:       "-rwxrw-r--    1 root     root            0 Apr 12 18:25 /tmp/foo*\n"
//usage:       "$ chmod 444 /tmp/foo\n"
//usage:       "$ ls -l /tmp/foo\n"
//usage:       "-r--r--r--    1 root     root            0 Apr 12 18:25 /tmp/foo\n"
/* This is a NOEXEC applet. Be very careful! */
/* coreutils:
 * chmod never changes the permissions of symbolic links; the chmod
 * system call cannot change their permissions. This is not a problem
 * since the permissions of symbolic links are never used.
 * However, for each symbolic link listed on the command line, chmod changes
 * the permissions of the pointed-to file. In contrast, chmod ignores
 * symbolic links encountered during recursive directory traversals.
 */
unsafe extern "C" fn fileAction(
  mut fileName: *const libc::c_char,
  mut statbuf: *mut stat,
  mut param: *mut libc::c_void,
  mut depth: libc::c_int,
) -> libc::c_int {
  let mut current_block: u64;
  let mut newmode: mode_t = 0;
  /* match coreutils behavior */
  if depth == 0i32 {
    if stat(fileName, statbuf) != 0 {
      current_block = 18130555599396617792; /* depth > 0: skip links */
    } else {
      current_block = 6873731126896040597;
    }
  } else {
    if (*statbuf).st_mode & 0o170000i32 as libc::c_uint == 0o120000i32 as libc::c_uint {
      return 1i32;
    }
    current_block = 6873731126896040597;
  }
  match current_block {
    6873731126896040597 => {
      newmode = bb_parse_mode(param as *mut libc::c_char, (*statbuf).st_mode) as mode_t;
      if newmode == -1i32 as mode_t {
        bb_error_msg_and_die(
          b"invalid mode \'%s\'\x00" as *const u8 as *const libc::c_char,
          param as *mut libc::c_char,
        );
      }
      if chmod(fileName, newmode) == 0i32 {
        if option_mask32 & 2i32 as libc::c_uint != 0
          || option_mask32 & 4i32 as libc::c_uint != 0 && (*statbuf).st_mode != newmode
        {
          printf(
            b"mode of \'%s\' changed to %04o (%s)\n\x00" as *const u8 as *const libc::c_char,
            fileName,
            newmode & 0o7777i32 as libc::c_uint,
            bb_mode_string(newmode).offset(1),
          );
        }
        return 1i32;
      }
    }
    _ => {}
  }
  if option_mask32 & 8i32 as libc::c_uint == 0 {
    bb_simple_perror_msg(fileName);
  }
  return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn chmod_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut retval: libc::c_int = 0i32;
  let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut argp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut smode: *mut libc::c_char = 0 as *mut libc::c_char;
  /* statbuf holds lstat result, but we need stat (follow link) */
  /* Convert first encountered -r into ar, -w into aw etc
   * so that getopt would not eat it */
  argp = argv;
  loop {
    argp = argp.offset(1);
    arg = *argp;
    if arg.is_null() {
      break;
    }
    /* Mode spec must be the first arg (sans -R etc) */
    /* (protect against mishandling e.g. "chmod 644 -r") */
    if *arg.offset(0) as libc::c_int != '-' as i32 {
      arg = 0 as *mut libc::c_char;
      break;
    } else {
      /* An option. Not a -- or valid option? */
      if !(*arg.offset(1) as libc::c_int != 0
        && strchr(
          b"-Rvcf\x00" as *const u8 as *const libc::c_char,
          *arg.offset(1) as libc::c_int,
        )
        .is_null())
      {
        continue;
      }
      *arg.offset(0) = 'a' as i32 as libc::c_char;
      break;
    }
  }
  /* Parse options */
  getopt32(argv, b"^Rvcf\x00-2\x00" as *const u8 as *const libc::c_char);
  argv = argv.offset(optind as isize);
  /* Restore option-like mode if needed */
  if !arg.is_null() {
    *arg.offset(0) = '-' as i32 as libc::c_char
  }
  /* Ok, ready to do the deed now */
  let fresh0 = argv;
  argv = argv.offset(1);
  smode = *fresh0;
  loop {
    if recursive_action(
      *argv,
      option_mask32 & 1i32 as libc::c_uint,
      Some(
        fileAction
          as unsafe extern "C" fn(
            _: *const libc::c_char,
            _: *mut stat,
            _: *mut libc::c_void,
            _: libc::c_int,
          ) -> libc::c_int,
      ),
      Some(
        fileAction
          as unsafe extern "C" fn(
            _: *const libc::c_char,
            _: *mut stat,
            _: *mut libc::c_void,
            _: libc::c_int,
          ) -> libc::c_int,
      ),
      smode as *mut libc::c_void,
      0i32 as libc::c_uint,
    ) == 0
    {
      // depth
      retval = 1i32
    }
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  return retval;
}
/*
Security: chmod is too important and too subtle.
This is a test script (busybox chmod versus coreutils).
Run it in empty directory.

#!/bin/sh
t1="/tmp/busybox chmod"
t2="/usr/bin/chmod"
create() {
    rm -rf $1; mkdir $1
    (
    cd $1 || exit 1
    mkdir dir
    >up
    >file
    >dir/file
    ln -s dir linkdir
    ln -s file linkfile
    ln -s ../up dir/up
    )
}
tst() {
    (cd test1; $t1 $1)
    (cd test2; $t2 $1)
    (cd test1; ls -lR) >out1
    (cd test2; ls -lR) >out2
    echo "chmod $1" >out.diff
    if ! diff -u out1 out2 >>out.diff; then exit 1; fi
    rm out.diff
}
echo "If script produced 'out.diff' file, then at least one testcase failed"
create test1; create test2
tst "a+w file"
tst "a-w dir"
tst "a+w linkfile"
tst "a-w linkdir"
tst "-R a+w file"
tst "-R a-w dir"
tst "-R a+w linkfile"
tst "-R a-w linkdir"
tst "a-r,a+x linkfile"
*/
