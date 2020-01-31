use crate::librb::bb_uidgid_t;
use libc;
use libc::chown;
use libc::gid_t;
use libc::lchown;
use libc::printf;
use libc::stat;
use libc::uid_t;
extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut option_mask32: u32;
}

pub type C2RustUnnamed = libc::c_uint;
pub const ACTION_DANGLING_OK: C2RustUnnamed = 64;
pub const ACTION_QUIET: C2RustUnnamed = 32;
pub const ACTION_DEPTHFIRST: C2RustUnnamed = 8;
pub const ACTION_FOLLOWLINKS_L0: C2RustUnnamed = 4;
pub const ACTION_FOLLOWLINKS: C2RustUnnamed = 2;
pub const ACTION_RECURSE: C2RustUnnamed = 1;

#[repr(C)]
#[derive(Copy, Clone)]
struct param_t {
  ugid: bb_uidgid_t,
  chown_func: unsafe extern "C" fn(_: *const libc::c_char, _: uid_t, _: gid_t) -> libc::c_int,
}

/*
 * Mini chown implementation for busybox
 *
 * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config CHOWN
//config:	bool "chown (7.6 kb)"
//config:	default y
//config:	help
//config:	chown is used to change the user and/or group ownership
//config:	of files.
//config:
//config:config FEATURE_CHOWN_LONG_OPTIONS
//config:	bool "Enable long options"
//config:	default y
//config:	depends on CHOWN && LONG_OPTS
//applet:IF_CHOWN(APPLET_NOEXEC(chown, chown, BB_DIR_BIN, SUID_DROP, chown))
//kbuild:lib-$(CONFIG_CHOWN) += chown.o
/* BB_AUDIT SUSv3 defects - none? */
/* http://www.opengroup.org/onlinepubs/007904975/utilities/chown.html */
//usage:#define chown_trivial_usage
//usage:       "[-Rh"IF_DESKTOP("LHPcvf")"]... USER[:[GRP]] FILE..."
//usage:#define chown_full_usage "\n\n"
//usage:       "Change the owner and/or group of each FILE to USER and/or GRP\n"
//usage:     "\n	-R	Recurse"
//usage:     "\n	-h	Affect symlinks instead of symlink targets"
//usage:	IF_DESKTOP(
//usage:     "\n	-L	Traverse all symlinks to directories"
//usage:     "\n	-H	Traverse symlinks on command line only"
//usage:     "\n	-P	Don't traverse symlinks (default)"
//usage:     "\n	-c	List changed files"
//usage:     "\n	-v	List all files"
//usage:     "\n	-f	Hide errors"
//usage:	)
//usage:
//usage:#define chown_example_usage
//usage:       "$ ls -l /tmp/foo\n"
//usage:       "-r--r--r--    1 andersen andersen        0 Apr 12 18:25 /tmp/foo\n"
//usage:       "$ chown root /tmp/foo\n"
//usage:       "$ ls -l /tmp/foo\n"
//usage:       "-r--r--r--    1 root     andersen        0 Apr 12 18:25 /tmp/foo\n"
//usage:       "$ chown root.root /tmp/foo\n"
//usage:       "ls -l /tmp/foo\n"
//usage:       "-r--r--r--    1 root     root            0 Apr 12 18:25 /tmp/foo\n"
/* This is a NOEXEC applet. Be very careful! */
/* POSIX options
 * -L traverse every symbolic link to a directory encountered
 * -H if a command line argument is a symbolic link to a directory, traverse it
 * -P do not traverse any symbolic links (default)
 * We do not conform to the following:
 * "Specifying more than one of -H, -L, and -P is not an error.
 * The last option specified shall determine the behavior of the utility." */
/* -L */
/* -H or -L */
static mut chown_longopts: [libc::c_char; 81] = [
  114, 101, 99, 117, 114, 115, 105, 118, 101, 0, 0, 82, 100, 101, 114, 101, 102, 101, 114, 101,
  110, 99, 101, 0, 0, -1, 110, 111, 45, 100, 101, 114, 101, 102, 101, 114, 101, 110, 99, 101, 0, 0,
  104, 99, 104, 97, 110, 103, 101, 115, 0, 0, 99, 115, 105, 108, 101, 110, 116, 0, 0, 102, 113,
  117, 105, 101, 116, 0, 0, 102, 118, 101, 114, 98, 111, 115, 101, 0, 0, 118, 0,
];
unsafe fn fileAction(
  mut fileName: *const libc::c_char,
  mut statbuf: *mut stat,
  mut vparam: *mut libc::c_void,
  mut _depth: libc::c_int,
) -> libc::c_int {
  let mut u: uid_t = if (*(vparam as *mut param_t)).ugid.uid == std::u32::MAX {
    (*statbuf).st_uid
  } else {
    (*(vparam as *mut param_t)).ugid.uid
  };
  let mut g: gid_t = if (*(vparam as *mut param_t)).ugid.gid == std::u32::MAX {
    (*statbuf).st_gid
  } else {
    (*(vparam as *mut param_t)).ugid.gid
  };
  if ((*(vparam as *mut param_t)).chown_func)(fileName, u, g) == 0 {
    if option_mask32 & 0x4i32 as libc::c_uint != 0
      || option_mask32 & 0x8i32 as libc::c_uint != 0
        && ((*statbuf).st_uid != u || (*statbuf).st_gid != g)
    {
      printf(
        b"changed ownership of \'%s\' to %u:%u\n\x00" as *const u8 as *const libc::c_char,
        fileName,
        u,
        g,
      );
    }
    return 1i32;
  }
  if option_mask32 & 0x10i32 as libc::c_uint == 0 {
    crate::libbb::perror_msg::bb_simple_perror_msg(fileName);
  }
  return 0;
}

pub unsafe fn chown_main(mut _argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut retval: libc::c_int = 0;
  let mut opt: libc::c_int = 0;
  let mut flags: libc::c_int = 0;

  opt = crate::libbb::getopt32::getopt32long(
    argv,
    b"^RhvcfLHP\x00-2\x00" as *const u8 as *const libc::c_char,
    chown_longopts.as_ptr(),
  ) as libc::c_int;
  argv = argv.offset(optind as isize);

  /* This matches coreutils behavior (almost - see below) */
  let mut param: param_t = param_t {
    ugid: bb_uidgid_t { uid: 0, gid: 0 },
    chown_func: if opt & 2i32 != 0 || opt & (1i32 | (0x20i32 | 0x40i32)) == 1i32 {
      lchown
    } else {
      /* match coreutils order */
      chown
    },
  };

  /* -H/-L: follow links on depth 0 */
  flags = ACTION_DEPTHFIRST as libc::c_int; /* follow links if -L */
  if opt & 1i32 != 0 {
    flags |= ACTION_RECURSE as libc::c_int
  }
  if opt & (0x20i32 | 0x40i32) != 0 {
    flags |= ACTION_FOLLOWLINKS_L0 as libc::c_int
  }
  if opt & 0x20i32 != 0 {
    flags |= ACTION_FOLLOWLINKS as libc::c_int
  }
  crate::libpwdgrp::uidgid_get::parse_chown_usergroup_or_die(&mut param.ugid, *argv.offset(0));
  loop
  /* Ok, ready to do the deed now */
  {
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
    if crate::libbb::recursive_action::recursive_action(
      *argv,
      flags as libc::c_uint,
      Some(
        fileAction
          as unsafe fn(
            _: *const libc::c_char,
            _: *mut stat,
            _: *mut libc::c_void,
            _: libc::c_int,
          ) -> libc::c_int,
      ),
      Some(
        fileAction
          as unsafe fn(
            _: *const libc::c_char,
            _: *mut stat,
            _: *mut libc::c_void,
            _: libc::c_int,
          ) -> libc::c_int,
      ),
      &mut param as *mut param_t as *mut libc::c_void,
      0 as libc::c_uint,
    ) == 0
    {
      /* depth */
      retval = 1i32
    }
  }
  return retval;
}
/*
Testcase. Run in empty directory.

#!/bin/sh
t1="/tmp/busybox chown"
t2="/usr/bin/chown"
create() {
    rm -rf $1; mkdir $1
    (
    cd $1 || exit 1
    mkdir dir dir2
    >up
    >file
    >dir/file
    >dir2/file
    ln -s dir linkdir
    ln -s file linkfile
    ln -s ../up dir/linkup
    ln -s ../dir2 dir/linkupdir2
    )
    chown -R 0:0 $1
}
tst() {
    create test1
    create test2
    echo "[$1]" >>test1.out
    echo "[$1]" >>test2.out
    (cd test1; $t1 $1) >>test1.out 2>&1
    (cd test2; $t2 $1) >>test2.out 2>&1
    (cd test1; ls -lnR) >out1
    (cd test2; ls -lnR) >out2
    echo "chown $1" >out.diff
    if ! diff -u out1 out2 >>out.diff; then exit 1; fi
    rm out.diff
}
tst_for_each() {
    tst "$1 1:1 file"
    tst "$1 1:1 dir"
    tst "$1 1:1 linkdir"
    tst "$1 1:1 linkfile"
}
echo "If script produced 'out.diff' file, then at least one testcase failed"
>test1.out
>test2.out
# These match coreutils 6.8:
tst_for_each "-v"
tst_for_each "-vR"
tst_for_each "-vRP"
tst_for_each "-vRL"
tst_for_each "-vRH"
tst_for_each "-vh"
tst_for_each "-vhR"
tst_for_each "-vhRP"
tst_for_each "-vhRL"
tst_for_each "-vhRH"
# Fix `name' in coreutils output
sed 's/`/'"'"'/g' -i test2.out
# Compare us with coreutils output
diff -u test1.out test2.out

*/
