use libc;
use libc::chdir;
use libc::chmod;
use libc::chown;
use libc::closelog;
use libc::dup2;
use libc::fstat;
use libc::getenv;
use libc::geteuid;
use libc::getopt;
use libc::getpid;
use libc::isatty;
use libc::kill;
use libc::openlog;
use libc::sigaddset;
use libc::sigemptyset;
use libc::sigprocmask;
use libc::sleep;
use libc::sscanf;
use libc::strcasecmp;
use libc::strcpy;
use libc::symlink;
use libc::syscall;
use libc::syslog;
use libc::time;
use libc::access;
use libc::atoi;
use libc::fclose;
use libc::fprintf;
use libc::lstat;
use libc::printf;
use libc::puts;
use libc::rename;
use libc::rmdir;
use libc::sprintf;
use libc::strchr;
use libc::strcmp;
use libc::strrchr;
use libc::strstr;
use libc::system;

use libc::gid_t;
use libc::group;
use libc::uid_t;

extern "C" {

  #[no_mangle]
  fn exit(_: libc::c_int) -> !;

  #[no_mangle]
  static mut optind: libc::c_int;

  /* Search for an entry with a matching group ID.  */
  #[no_mangle]
  fn bb_internal_getgrgid(__gid: gid_t) -> *mut group;
  /* Search for an entry with a matching group name.  */
  #[no_mangle]
  fn bb_internal_getgrnam(__name: *const libc::c_char) -> *mut group;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn itoa(n: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn xatou_range(str: *const libc::c_char, l: libc::c_uint, u: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn xuname2uid(name: *const libc::c_char) -> libc::c_long;
  #[no_mangle]
  fn xgetgrnam(name: *const libc::c_char) -> *mut group;
  #[no_mangle]
  static mut option_mask32: u32;
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
  static bb_msg_perm_denied_are_you_root: [libc::c_char; 0];
}

unsafe extern "C" fn xgroup_study(mut g: *mut group) {
  let mut max: libc::c_uint = 60000i32 as libc::c_uint;
  /* Make sure gr_name is unused */
  if !bb_internal_getgrnam((*g).gr_name).is_null() {
    bb_error_msg_and_die(
      b"%s \'%s\' in use\x00" as *const u8 as *const libc::c_char,
      b"group\x00" as *const u8 as *const libc::c_char,
      (*g).gr_name,
    );
    /* these format strings are reused in adduser and addgroup */
  }
  /* if a specific gid is requested, the --system switch and */
  /* min and max values are overridden, and the range of valid */
  /* gid values is set to [0, INT_MAX] */
  if option_mask32 & (1i32 << 0i32) as libc::c_uint == 0 {
    if option_mask32 & (1i32 << 1i32) as libc::c_uint != 0 {
      (*g).gr_gid = 100i32 as gid_t;
      max = 999i32 as libc::c_uint
    } else {
      (*g).gr_gid = (999i32 + 1i32) as gid_t
    }
  }
  loop
  /* Check if the desired gid is free
   * or find the first free one */
  {
    if bb_internal_getgrgid((*g).gr_gid).is_null() {
      return;
      /* found free group: return */
    }
    if option_mask32 & (1i32 << 0i32) as libc::c_uint != 0 {
      /* -g N, cannot pick gid other than N: error */
      bb_error_msg_and_die(
        b"%s \'%s\' in use\x00" as *const u8 as *const libc::c_char,
        b"gid\x00" as *const u8 as *const libc::c_char,
        itoa((*g).gr_gid as libc::c_int),
      );
      /* this format strings is reused in adduser and addgroup */
    }
    if (*g).gr_gid == max {
      /* overflowed: error */
      bb_error_msg_and_die(
        b"no %cids left\x00" as *const u8 as *const libc::c_char,
        'g' as i32,
      );
      /* this format string is reused in adduser and addgroup */
    }
    (*g).gr_gid = (*g).gr_gid.wrapping_add(1)
  }
}
/* append a new user to the passwd file */
unsafe extern "C" fn new_group(mut group: *mut libc::c_char, mut gid: gid_t) {
  let mut gr: group = group {
    gr_name: 0 as *mut libc::c_char,
    gr_passwd: 0 as *mut libc::c_char,
    gr_gid: 0,
    gr_mem: 0 as *mut *mut libc::c_char,
  };
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  /* make sure gid and group haven't already been allocated */
  gr.gr_gid = gid;
  gr.gr_name = group;
  xgroup_study(&mut gr);
  /* add entry to group */
  p = xasprintf(b"x:%u:\x00" as *const u8 as *const libc::c_char, gr.gr_gid);
  if update_passwd(
    b"/etc/group\x00" as *const u8 as *const libc::c_char,
    group,
    p,
    0 as *const libc::c_char,
  ) < 0i32
  {
    exit(1i32);
  }
  /* /etc/gshadow fields:
   * 1. Group name.
   * 2. Encrypted password.
   *    If set, non-members of the group can join the group
   *    by typing the password for that group using the newgrp command.
   *    If the value is of this field ! then no user is allowed
   *    to access the group using the newgrp command. A value of !!
   *    is treated the same as a value of ! only it indicates
   *    that a password has never been set before. If the value is null,
   *    only group members can log into the group.
   * 3. Group administrators (comma delimited list).
   *    Group members listed here can add or remove group members
   *    using the gpasswd command.
   * 4. Group members (comma delimited list).
   */
  /* Ignore errors: if file is missing we assume admin doesn't want it */
  update_passwd(
    b"/etc/gshadow\x00" as *const u8 as *const libc::c_char,
    group,
    b"!::\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
  );
}
//FIXME: upstream addgroup has no short options! NOT COMPATIBLE!
static mut addgroup_longopts: [libc::c_char; 16] = [
  103, 105, 100, 0, 1, 103, 115, 121, 115, 116, 101, 109, 0, 0, 83, 0,
];
/*
 * addgroup will take a login_name as its first parameter.
 *
 * gid can be customized via command-line parameters.
 * If called with two non-option arguments, addgroup
 * will add an existing user to an existing group.
 */
#[no_mangle]
pub unsafe extern "C" fn addgroup_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opts: libc::c_uint = 0;
  let mut gid: *const libc::c_char = b"0\x00" as *const u8 as *const libc::c_char;
  /* need to be root */
  if geteuid() != 0 {
    bb_simple_error_msg_and_die(bb_msg_perm_denied_are_you_root.as_ptr());
  }
  /* Syntax:
   *  addgroup group
   *  addgroup --gid num group
   *  addgroup user group
   * Check for min, max and missing args */
  opts = getopt32long(
    argv,
    b"^g:S\x00-1:?2\x00" as *const u8 as *const libc::c_char,
    addgroup_longopts.as_ptr(),
    &mut gid as *mut *const libc::c_char,
  );
  /* move past the commandline options */
  argv = argv.offset(optind as isize);
  //argc -= optind;
  if !(*argv.offset(1)).is_null() {
    let mut gr: *mut group = 0 as *mut group;
    if opts & (1i32 << 0i32) as libc::c_uint != 0 {
      /* -g was there, but "addgroup -g num user group"
       * is a no-no */
      bb_show_usage();
    }
    /* check if group and user exist */
    xuname2uid(*argv.offset(0)); /* unknown user: exit */
    gr = xgetgrnam(*argv.offset(1)); /* unknown group: exit */
    /* check if user is already in this group */
    while !(*(*gr).gr_mem).is_null() {
      if strcmp(*argv.offset(0), *(*gr).gr_mem) == 0i32 {
        /* user is already in group: do nothing */
        return 0i32;
      }
      (*gr).gr_mem = (*gr).gr_mem.offset(1)
    }
    if update_passwd(
      b"/etc/group\x00" as *const u8 as *const libc::c_char,
      *argv.offset(1),
      0 as *const libc::c_char,
      *argv.offset(0),
    ) < 0i32
    {
      return 1i32;
    }
    update_passwd(
      b"/etc/gshadow\x00" as *const u8 as *const libc::c_char,
      *argv.offset(1),
      0 as *const libc::c_char,
      *argv.offset(0),
    );
  } else {
    /* ENABLE_FEATURE_ADDUSER_TO_GROUP */
    new_group(
      *argv.offset(0),
      xatou_range(gid, 0i32 as libc::c_uint, 60000i32 as libc::c_uint),
    );
  }
  /* Reached only on success */
  return 0i32;
}
