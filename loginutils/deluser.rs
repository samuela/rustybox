use libc;
































use libc::geteuid;



































use libc::group;
use libc::passwd;


extern "C" {

  /* Read an entry from the password-file stream, opening it if necessary.  */
  #[no_mangle]
  fn bb_internal_getpwent() -> *mut passwd;
  /* Search for an entry with a matching group name.  */
  #[no_mangle]
  fn bb_internal_getgrnam(__name: *const libc::c_char) -> *mut group;
  #[no_mangle]
  fn remove_file(path: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xgetpwnam(name: *const libc::c_char) -> *mut passwd;
  #[no_mangle]
  fn xgetgrnam(name: *const libc::c_char) -> *mut group;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> u32;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn update_passwd(
    filename: *const libc::c_char,
    username: *const libc::c_char,
    data: *const libc::c_char,
    member: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  static bb_msg_perm_denied_are_you_root: [libc::c_char; 0];
}

pub type C2RustUnnamed = libc::c_int;
pub const FILEUTILS_IGNORE_CHMOD_ERR: C2RustUnnamed = -2147483648;
pub const FILEUTILS_REFLINK_ALWAYS: C2RustUnnamed = 262144;
pub const FILEUTILS_REFLINK: C2RustUnnamed = 131072;
pub const FILEUTILS_RMDEST: C2RustUnnamed = 32768;
pub const FILEUTILS_NO_TARGET_DIR: C2RustUnnamed = 16384;
pub const FILEUTILS_UPDATE: C2RustUnnamed = 8192;
pub const FILEUTILS_VERBOSE: C2RustUnnamed = 4096;
pub const FILEUTILS_DEREFERENCE_L0: C2RustUnnamed = 256;
pub const FILEUTILS_DEREF_SOFTLINK: C2RustUnnamed = 128;
pub const FILEUTILS_MAKE_SOFTLINK: C2RustUnnamed = 64;
pub const FILEUTILS_MAKE_HARDLINK: C2RustUnnamed = 32;
pub const FILEUTILS_INTERACTIVE: C2RustUnnamed = 16;
pub const FILEUTILS_FORCE: C2RustUnnamed = 8;
pub const FILEUTILS_RECUR: C2RustUnnamed = 4;
pub const FILEUTILS_DEREFERENCE: C2RustUnnamed = 2;
pub const FILEUTILS_PRESERVE_STATUS: C2RustUnnamed = 1;

/*
 * deluser/delgroup implementation for busybox
 *
 * Copyright (C) 1999 by Lineo, inc. and John Beppu
 * Copyright (C) 1999,2000,2001 by John Beppu <beppu@codepoet.org>
 * Copyright (C) 2007 by Tito Ragusa <farmatito@tiscali.it>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config DELUSER
//config:	bool "deluser (9.1 kb)"
//config:	default y
//config:	help
//config:	Utility for deleting a user account.
//config:
//config:config DELGROUP
//config:	bool "delgroup (6.4 kb)"
//config:	default y
//config:	help
//config:	Utility for deleting a group account.
//config:
//config:config FEATURE_DEL_USER_FROM_GROUP
//config:	bool "Support removing users from groups"
//config:	default y
//config:	depends on DELGROUP
//config:	help
//config:	If called with two non-option arguments, deluser
//config:	or delgroup will remove an user from a specified group.
//                   APPLET_NOEXEC:name      main     location         suid_type     help
//applet:IF_DELUSER( APPLET_NOEXEC(deluser,  deluser, BB_DIR_USR_SBIN, BB_SUID_DROP, deluser))
//applet:IF_DELGROUP(APPLET_NOEXEC(delgroup, deluser, BB_DIR_USR_SBIN, BB_SUID_DROP, delgroup))
//kbuild:lib-$(CONFIG_DELUSER) += deluser.o
//kbuild:lib-$(CONFIG_DELGROUP) += deluser.o
//usage:#define deluser_trivial_usage
//usage:       IF_LONG_OPTS("[--remove-home] ") "USER"
//usage:#define deluser_full_usage "\n\n"
//usage:       "Delete USER from the system"
//	--remove-home is self-explanatory enough to put it in --help
//usage:#define delgroup_trivial_usage
//usage:	IF_FEATURE_DEL_USER_FROM_GROUP("[USER] ")"GROUP"
//usage:#define delgroup_full_usage "\n\n"
//usage:       "Delete group GROUP from the system"
//usage:	IF_FEATURE_DEL_USER_FROM_GROUP(" or user USER from group GROUP")
#[no_mangle]
pub unsafe extern "C" fn deluser_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  /* User or group name */
  let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
  /* Username (non-NULL only in "delgroup USER GROUP" case) */
  let mut member: *mut libc::c_char = 0 as *mut libc::c_char;
  /* Name of passwd or group file */
  let mut pfile: *const libc::c_char = 0 as *const libc::c_char;
  /* Name of shadow or gshadow file */
  let mut sfile: *const libc::c_char = 0 as *const libc::c_char;
  /* Are we deluser or delgroup? */
  let mut do_deluser: libc::c_int = (1i32 != 0
    && (1i32 == 0 || *applet_name.offset(3) as libc::c_int == 'u' as i32))
    as libc::c_int;
  let mut opt_delhome: libc::c_int = 0i32;
  if do_deluser != 0 {
    opt_delhome = getopt32long(
      argv,
      b"\x00" as *const u8 as *const libc::c_char,
      b"remove-home\x00\x00\xff\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    argv = argv.offset(opt_delhome as isize);
    argc -= opt_delhome
  }
  if geteuid() != 0i32 as libc::c_uint {
    bb_simple_error_msg_and_die(bb_msg_perm_denied_are_you_root.as_ptr());
  }
  name = *argv.offset(1);
  member = 0 as *mut libc::c_char;
  let mut gr: *mut group = 0 as *mut group;
  let mut current_block_45: u64;
  match argc {
    3 => {
      if 1i32 == 0 || do_deluser != 0 {
        current_block_45 = 7018308795614528254;
      } else {
        /* It's "delgroup USER GROUP" */
        member = name;
        name = *argv.offset(2);
        current_block_45 = 12972848734546617528;
      }
    }
    2 => {
      current_block_45 = 12972848734546617528;
    }
    _ => {
      current_block_45 = 7018308795614528254;
    }
  }
  match current_block_45 {
    7018308795614528254 => {}
    _ =>
    /* Fallthrough */
    {
      if do_deluser != 0 {
        /* "deluser USER" */
        let mut pw: *mut passwd = 0 as *mut passwd; /* bail out if USER is wrong */
        pw = xgetpwnam(name);
        pfile = b"/etc/passwd\x00" as *const u8 as *const libc::c_char;
        sfile = b"/etc/shadow\x00" as *const u8 as *const libc::c_char;
        if opt_delhome != 0 {
          remove_file((*pw).pw_dir, FILEUTILS_RECUR as libc::c_int);
        }
        current_block_45 = 15090052786889560393;
      } else {
        gr = 0 as *mut group;
        current_block_45 = 16500901810917105941;
      }
      loop {
        match current_block_45 {
          16500901810917105941 => {
            /* "delgroup GROUP" or "delgroup USER GROUP" */
            if do_deluser < 0i32 {
              /* delgroup after deluser? */
              gr = bb_internal_getgrnam(name);
              if gr.is_null() {
                return 0i32;
              }
            } else {
              gr = xgetgrnam(name)
              /* bail out if GROUP is wrong */
            }
            if member.is_null() {
              /* "delgroup GROUP" */
              let mut pw_0: *mut passwd = 0 as *mut passwd;
              loop
              /* Check if the group is in use */
              {
                pw_0 = bb_internal_getpwent();
                if pw_0.is_null() {
                  break;
                }
                if (*pw_0).pw_gid == (*gr).gr_gid {
                  bb_error_msg_and_die(
                    b"\'%s\' still has \'%s\' as their primary group!\x00" as *const u8
                      as *const libc::c_char,
                    (*pw_0).pw_name,
                    name,
                  );
                }
              }
            }
            pfile = b"/etc/group\x00" as *const u8 as *const libc::c_char;
            sfile = b"/etc/gshadow\x00" as *const u8 as *const libc::c_char;
            current_block_45 = 15090052786889560393;
          }
          _ => {
            loop
            /* Modify pfile, then sfile */
            {
              if update_passwd(pfile, name, 0 as *const libc::c_char, member) == -1i32 {
                return 1i32;
              }
              pfile = sfile;
              sfile = 0 as *const libc::c_char;
              if !(1i32 != 0 && !pfile.is_null()) {
                break;
              }
            }
            if !(do_deluser > 0i32) {
              break;
            }
            /* Delete user from all groups */
            if update_passwd(
              b"/etc/group\x00" as *const u8 as *const libc::c_char,
              0 as *const libc::c_char,
              0 as *const libc::c_char,
              name,
            ) == -1i32
            {
              return 1i32;
            }
            /* "deluser USER" also should try to delete
             * same-named group. IOW: do "delgroup USER"
             */
            // On debian deluser is a perl script that calls userdel.
            // From man userdel:
            //  If USERGROUPS_ENAB is defined to yes in /etc/login.defs, userdel will
            //  delete the group with the same name as the user.
            do_deluser = -1i32;
            current_block_45 = 16500901810917105941;
          }
        }
      }
      return 0i32;
    }
  }
  /* Reached only if number of command line args is wrong */
  bb_show_usage();
}
