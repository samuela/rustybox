use crate::libbb::appletlib::applet_name;
use crate::libbb::ptr_to_globals::bb_errno;
use crate::libbb::xfuncs_printf::xmalloc;
use libc;
use libc::getegid;
use libc::geteuid;
use libc::getgid;
use libc::getuid;
use libc::gid_t;
use libc::passwd;
use libc::printf;
use libc::uid_t;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;

  /* Reentrant versions of some of the functions above. */
  /* Store at most *NGROUPS members of the group set for USER into
  *GROUPS.  Also include GROUP.  The actual number of groups found is
  returned in *NGROUPS.  Return -1 if the if *NGROUPS is too small.  */

  #[no_mangle]
  static mut option_mask32: u32;

  #[no_mangle]
  fn getgroups(__size: libc::c_int, __list: *mut gid_t) -> libc::c_int;
}

/*
 * Mini id implementation for busybox
 *
 * Copyright (C) 2000 by Randolph Chung <tausq@debian.org>
 * Copyright (C) 2008 by Tito Ragusa <farmatito@tiscali.it>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Hacked by Tito Ragusa (C) 2004 to handle usernames of whatever
 * length and to be more similar to GNU id.
 * -Z option support: by Yuichi Nakamura <ynakam@hitachisoft.jp>
 * Added -G option Tito Ragusa (C) 2008 for SUSv3.
 */
//config:config ID
//config:	bool "id (7 kb)"
//config:	default y
//config:	help
//config:	id displays the current user and group ID names.
//config:
//config:config GROUPS
//config:	bool "groups (6.7 kb)"
//config:	default y
//config:	help
//config:	Print the group names associated with current user id.
//applet:IF_GROUPS(APPLET_NOEXEC(groups, id, BB_DIR_USR_BIN, SUID_DROP, groups))
//applet:IF_ID(    APPLET_NOEXEC(id,     id, BB_DIR_USR_BIN, SUID_DROP, id    ))
//kbuild:lib-$(CONFIG_GROUPS) += id.o
//kbuild:lib-$(CONFIG_ID)     += id.o
/* BB_AUDIT SUSv3 compliant. */
//usage:#define id_trivial_usage
//usage:       "[OPTIONS] [USER]"
//usage:#define id_full_usage "\n\n"
//usage:       "Print information about USER or the current user\n"
//usage:	IF_SELINUX(
//usage:     "\n	-Z	Security context"
//usage:	)
//usage:     "\n	-u	User ID"
//usage:     "\n	-g	Group ID"
//usage:     "\n	-G	Supplementary group IDs"
//usage:     "\n	-n	Print names instead of numbers"
//usage:     "\n	-r	Print real ID instead of effective ID"
//usage:
//usage:#define id_example_usage
//usage:       "$ id\n"
//usage:       "uid=1000(andersen) gid=1000(andersen)\n"
//usage:#define groups_trivial_usage
//usage:       "[USER]"
//usage:#define groups_full_usage "\n\n"
//usage:       "Print the group memberships of USER or for the current process"
//usage:
//usage:#define groups_example_usage
//usage:       "$ groups\n"
//usage:       "andersen lp dialout cdrom floppy\n"
/* This is a NOEXEC applet. Be very careful! */
pub type C2RustUnnamed = libc::c_uint;
pub const JUST_ALL_GROUPS: C2RustUnnamed = 16;
pub const JUST_GROUP: C2RustUnnamed = 8;
pub const JUST_USER: C2RustUnnamed = 4;
pub const NAME_NOT_NUMBER: C2RustUnnamed = 2;
pub const PRINT_REAL: C2RustUnnamed = 1;
unsafe extern "C" fn print_common(
  mut id: libc::c_uint,
  mut name: *const libc::c_char,
  mut prefix: *const libc::c_char,
) -> libc::c_int {
  if !prefix.is_null() {
    printf(b"%s\x00" as *const u8 as *const libc::c_char, prefix);
  }
  if option_mask32 & NAME_NOT_NUMBER as libc::c_int as libc::c_uint == 0 || name.is_null() {
    printf(b"%u\x00" as *const u8 as *const libc::c_char, id);
  }
  if option_mask32 == 0 || option_mask32 & NAME_NOT_NUMBER as libc::c_int as libc::c_uint != 0 {
    if !name.is_null() {
      printf(
        if option_mask32 != 0 {
          b"%s\x00" as *const u8 as *const libc::c_char
        } else {
          b"(%s)\x00" as *const u8 as *const libc::c_char
        },
        name,
      );
    } else if option_mask32 != 0 {
      crate::libbb::verror_msg::bb_error_msg(
        b"unknown ID %u\x00" as *const u8 as *const libc::c_char,
        id,
      );
      return 1i32;
    }
  }
  return 0i32;
}
unsafe extern "C" fn print_group(mut id: gid_t, mut prefix: *const libc::c_char) -> libc::c_int {
  return print_common(id, crate::libbb::bb_pwd::gid2group(id), prefix);
}
unsafe extern "C" fn print_user(mut id: uid_t, mut prefix: *const libc::c_char) -> libc::c_int {
  return print_common(id, crate::libbb::bb_pwd::uid2uname(id), prefix);
}
/* Don't set error status flag in default mode */
/* On error set *n < 0 and return >= 0
 * If *n is too small, update it and return < 0
 * (ok to trash groups[] in both cases)
 * Otherwise fill in groups[] and return >= 0
 */
unsafe extern "C" fn get_groups(
  mut username: *const libc::c_char,
  mut rgid: gid_t,
  mut groups: *mut gid_t,
  mut n: *mut libc::c_int,
) -> libc::c_int {
  let mut m: libc::c_int = 0;
  if !username.is_null() {
    /* If the user is a member of more than
     * *n groups, then -1 is returned. Otherwise >= 0.
     * (and no defined way of detecting errors?!) */
    m = crate::libpwdgrp::pwd_grp::bb_internal_getgrouplist(username, rgid, groups, n);
    /* I guess *n < 0 might indicate error. Anyway,
     * malloc'ing -1 bytes won't be good, so: */
    if *n < 0i32 {
      return 0i32;
    }
    return m;
  }
  *n = getgroups(*n, groups);
  if *n >= 0i32 {
    return *n;
  }
  /* Error */
  if *bb_errno == 22i32 {
    /* get needed *n */
    /* *n is too small? */
    *n = getgroups(0i32, groups)
  }
  /* if *n >= 0, return -1 (got new *n), else return 0 (error): */
  return -((*n >= 0i32) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn id_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut ruid: uid_t = 0;
  let mut rgid: gid_t = 0;
  let mut euid: uid_t = 0;
  let mut egid: gid_t = 0;
  let mut opt: libc::c_uint = 0;
  let mut i: libc::c_int = 0;
  let mut status: libc::c_int = 0i32;
  let mut prefix: *const libc::c_char = 0 as *const libc::c_char;
  let mut username: *const libc::c_char = 0 as *const libc::c_char;
  if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(0) as libc::c_int == 'g' as i32) {
    /* TODO: coreutils groups prepend "USER : " prefix,
     * and accept many usernames. Example:
     * # groups root root
     * root : root
     * root : root
     */
    option_mask32 =
      crate::libbb::getopt32::getopt32(argv, b"\x00" as *const u8 as *const libc::c_char)
        | JUST_ALL_GROUPS as libc::c_int as libc::c_uint
        | NAME_NOT_NUMBER as libc::c_int as libc::c_uint;
    opt = option_mask32
  } else {
    /* Don't allow -n -r -nr -ug -rug -nug -rnug -uZ -gZ -GZ*/
    /* Don't allow more than one username */
    opt = crate::libbb::getopt32::getopt32(
      argv,
      b"^rnugG\x00?1:u--g:g--u:G--u:u--G:g--G:G--g:r?ugG:n?ugG\x00" as *const u8
        as *const libc::c_char,
    )
  }
  username = *argv.offset(optind as isize);
  if !username.is_null() {
    let mut p: *mut passwd = crate::libbb::bb_pwd::xgetpwnam(username);
    ruid = (*p).pw_uid;
    euid = ruid;
    rgid = (*p).pw_gid;
    egid = rgid
  } else {
    egid = getegid();
    rgid = getgid();
    euid = geteuid();
    ruid = getuid()
  }
  /* JUST_ALL_GROUPS ignores -r PRINT_REAL flag even if man page for */
  /* id says: print the real ID instead of the effective ID, with -ugG */
  /* in fact in this case egid is always printed if egid != rgid */
  if opt == 0 || opt & JUST_ALL_GROUPS as libc::c_int as libc::c_uint != 0 {
    let mut groups: *mut gid_t = std::ptr::null_mut();
    let mut n: libc::c_int = 0;
    if opt == 0 {
      /* Default Mode */
      status |= print_user(ruid, b"uid=\x00" as *const u8 as *const libc::c_char);
      status |= print_group(rgid, b" gid=\x00" as *const u8 as *const libc::c_char);
      if euid != ruid {
        status |= print_user(euid, b" euid=\x00" as *const u8 as *const libc::c_char)
      }
      if egid != rgid {
        status |= print_group(egid, b" egid=\x00" as *const u8 as *const libc::c_char)
      }
    } else {
      /* JUST_ALL_GROUPS */
      status |= print_group(rgid, 0 as *const libc::c_char);
      if egid != rgid {
        status |= print_group(egid, b" \x00" as *const u8 as *const libc::c_char)
      }
    }
    /* We are supplying largish buffer, trying
     * to not run get_groups() twice. That might be slow
     * ("user database in remote SQL server" case) */
    groups = xmalloc(
      (64i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<gid_t>() as libc::c_ulong),
    ) as *mut gid_t;
    n = 64i32;
    if get_groups(username, rgid, groups, &mut n) < 0i32 {
      /* Need bigger buffer after all */
      groups = crate::libbb::xfuncs_printf::xrealloc(
        groups as *mut libc::c_void,
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<gid_t>() as libc::c_ulong),
      ) as *mut gid_t;
      get_groups(username, rgid, groups, &mut n);
    }
    if n > 0i32 {
      /* Print the list */
      prefix = b" groups=\x00" as *const u8 as *const libc::c_char;
      i = 0i32;
      while i < n {
        if !(opt != 0 && (*groups.offset(i as isize) == rgid || *groups.offset(i as isize) == egid))
        {
          status |= print_group(
            *groups.offset(i as isize),
            if opt != 0 {
              b" \x00" as *const u8 as *const libc::c_char
            } else {
              prefix
            },
          );
          prefix = b",\x00" as *const u8 as *const libc::c_char
        }
        i += 1
      }
    } else if n < 0i32 {
      /* error in get_groups() */
      crate::libbb::verror_msg::bb_simple_error_msg_and_die(
        b"can\'t get groups\x00" as *const u8 as *const libc::c_char,
      );
    }
  } else if opt & PRINT_REAL as libc::c_int as libc::c_uint != 0 {
    euid = ruid;
    egid = rgid
  }
  if opt & JUST_USER as libc::c_int as libc::c_uint != 0 {
    status |= print_user(euid, 0 as *const libc::c_char)
  } else if opt & JUST_GROUP as libc::c_int as libc::c_uint != 0 {
    status |= print_group(egid, 0 as *const libc::c_char)
  }
  crate::libbb::xfuncs_printf::bb_putchar('\n' as i32);
  crate::libbb::fflush_stdout_and_exit::fflush_stdout_and_exit(status);
}
