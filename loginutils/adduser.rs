use libc;
use libc::unlink;


use libc::gid_t;
use libc::time_t;
use libc::uid_t;

use crate::librb::smallint;
use libc::group;
use libc::mode_t;
use libc::passwd;

extern "C" {

  #[no_mangle]
  fn execlp(__file: *const libc::c_char, __arg: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn geteuid() -> uid_t;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn chown(__file: *const libc::c_char, __owner: uid_t, __group: gid_t) -> libc::c_int;
  #[no_mangle]
  fn chmod(__file: *const libc::c_char, __mode: mode_t) -> libc::c_int;
  #[no_mangle]
  fn umask(__mask: mode_t) -> mode_t;
  #[no_mangle]
  fn mkdir(__path: *const libc::c_char, __mode: mode_t) -> libc::c_int;
  #[no_mangle]
  fn time(__timer: *mut time_t) -> time_t;
  /* Search for an entry with a matching user ID.  */
  #[no_mangle]
  fn bb_internal_getpwuid(__uid: uid_t) -> *mut passwd;
  /* Search for an entry with a matching username.  */
  #[no_mangle]
  fn bb_internal_getpwnam(__name: *const libc::c_char) -> *mut passwd;
  /* Search for an entry with a matching group ID.  */
  #[no_mangle]
  fn bb_internal_getgrgid(__gid: gid_t) -> *mut group;
  /* Search for an entry with a matching group name.  */
  #[no_mangle]
  fn bb_internal_getgrnam(__name: *const libc::c_char) -> *mut group;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn copy_file(
    source: *const libc::c_char,
    dest: *const libc::c_char,
    flags: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn utoa(n: libc::c_uint) -> *mut libc::c_char;
  #[no_mangle]
  fn itoa(n: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn xatou_range(str: *const libc::c_char, l: libc::c_uint, u: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn xgroup2gid(name: *const libc::c_char) -> libc::c_long;
  #[no_mangle]
  fn spawn_and_wait(argv: *mut *mut libc::c_char) -> libc::c_int;
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
  static mut logmode: smallint;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn chown_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn get_shell_name() -> *const libc::c_char;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const LOGMODE_BOTH: C2RustUnnamed_0 = 3;
pub const LOGMODE_SYSLOG: C2RustUnnamed_0 = 2;
pub const LOGMODE_STDIO: C2RustUnnamed_0 = 1;
pub const LOGMODE_NONE: C2RustUnnamed_0 = 0;
/* remix */
/* recoded such that the uid may be passed in *p */
unsafe extern "C" fn passwd_study(mut p: *mut passwd) {
  let mut max: libc::c_int = 60000i32;
  if !bb_internal_getpwnam((*p).pw_name).is_null() {
    bb_error_msg_and_die(
      b"%s \'%s\' in use\x00" as *const u8 as *const libc::c_char,
      b"user\x00" as *const u8 as *const libc::c_char,
      (*p).pw_name,
    );
    /* this format string is reused in adduser and addgroup */
  }
  if option_mask32 & (1i32 << 7i32) as libc::c_uint == 0 {
    if option_mask32 & (1i32 << 5i32) as libc::c_uint != 0 {
      (*p).pw_uid = 100i32 as uid_t;
      max = 999i32
    } else {
      (*p).pw_uid = (999i32 + 1i32) as uid_t
    }
  }
  /* check for a free uid (and maybe gid) */
  while !bb_internal_getpwuid((*p).pw_uid).is_null()
    || (*p).pw_gid == -1i32 as gid_t && !bb_internal_getgrgid((*p).pw_uid).is_null()
  {
    if option_mask32 & (1i32 << 7i32) as libc::c_uint != 0 {
      /* -u N, cannot pick uid other than N: error */
      bb_error_msg_and_die(
        b"%s \'%s\' in use\x00" as *const u8 as *const libc::c_char,
        b"uid\x00" as *const u8 as *const libc::c_char,
        itoa((*p).pw_uid as libc::c_int),
      );
      /* this format string is reused in adduser and addgroup */
    }
    if (*p).pw_uid == max as libc::c_uint {
      bb_error_msg_and_die(
        b"no %cids left\x00" as *const u8 as *const libc::c_char,
        'u' as i32,
      );
      /* this format string is reused in adduser and addgroup */
    } /* new gid = uid */
    (*p).pw_uid = (*p).pw_uid.wrapping_add(1)
  }
  if (*p).pw_gid == -1i32 as gid_t {
    (*p).pw_gid = (*p).pw_uid;
    if !bb_internal_getgrnam((*p).pw_name).is_null() {
      bb_error_msg_and_die(
        b"%s \'%s\' in use\x00" as *const u8 as *const libc::c_char,
        b"group\x00" as *const u8 as *const libc::c_char,
        (*p).pw_name,
      );
      /* this format string is reused in adduser and addgroup */
    }
  };
}
unsafe extern "C" fn addgroup_wrapper(
  mut p: *mut passwd,
  mut group_name: *const libc::c_char,
) -> libc::c_int {
  let mut argv: [*mut libc::c_char; 6] = [0 as *mut libc::c_char; 6];
  argv[0] = b"addgroup\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
  if !group_name.is_null() {
    /* Add user to existing group */
    argv[1] = b"--\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[2] = (*p).pw_name;
    argv[3] = group_name as *mut libc::c_char;
    argv[4] = 0 as *mut libc::c_char
  } else {
    /* Add user to his own group with the first free gid
     * found in passwd_study.
     */
    argv[1] = b"--gid\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[2] = utoa((*p).pw_gid);
    argv[3] = b"--\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[4] = (*p).pw_name;
    argv[5] = 0 as *mut libc::c_char
  }
  return spawn_and_wait(argv.as_mut_ptr());
}
unsafe extern "C" fn passwd_wrapper(mut login_name: *const libc::c_char) -> ! {
  execlp(
    b"passwd\x00" as *const u8 as *const libc::c_char,
    b"passwd\x00" as *const u8 as *const libc::c_char,
    b"--\x00" as *const u8 as *const libc::c_char,
    login_name,
    0 as *mut libc::c_void,
  );
  bb_simple_error_msg_and_die(
    b"can\'t execute passwd, you must set password manually\x00" as *const u8
      as *const libc::c_char,
  );
}
//FIXME: upstream adduser has no short options! NOT COMPATIBLE!
static mut adduser_longopts: [libc::c_char; 110] = [
  104, 111, 109, 101, 0, 1, 104, 103, 101, 99, 111, 115, 0, 1, 103, 115, 104, 101, 108, 108, 0, 1,
  115, 105, 110, 103, 114, 111, 117, 112, 0, 1, 71, 100, 105, 115, 97, 98, 108, 101, 100, 45, 112,
  97, 115, 115, 119, 111, 114, 100, 0, 0, 68, 101, 109, 112, 116, 121, 45, 112, 97, 115, 115, 119,
  111, 114, 100, 0, 0, 68, 115, 121, 115, 116, 101, 109, 0, 0, 83, 110, 111, 45, 99, 114, 101, 97,
  116, 101, 45, 104, 111, 109, 101, 0, 0, 72, 117, 105, 100, 0, 1, 117, 115, 107, 101, 108, 0, 1,
  107, 0,
];
/*
 * adduser will take a login_name as its first parameter.
 * home, shell, gecos:
 * can be customized via command-line parameters.
 */
#[no_mangle]
pub unsafe extern "C" fn adduser_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut pw: passwd = passwd {
    pw_name: 0 as *mut libc::c_char,
    pw_passwd: 0 as *mut libc::c_char,
    pw_uid: 0,
    pw_gid: 0,
    pw_gecos: 0 as *mut libc::c_char,
    pw_dir: 0 as *mut libc::c_char,
    pw_shell: 0 as *mut libc::c_char,
  };
  let mut usegroup: *const libc::c_char = 0 as *const libc::c_char;
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut opts: libc::c_uint = 0;
  let mut uid: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut skel: *const libc::c_char = b"/etc/skel\x00" as *const u8 as *const libc::c_char;
  /* got root? */
  if geteuid() != 0 {
    bb_simple_error_msg_and_die(bb_msg_perm_denied_are_you_root.as_ptr());
  }
  pw.pw_gecos = b"Linux User,,,\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
  /* We assume that newly created users "inherit" root's shell setting */
  pw.pw_shell = xstrdup(get_shell_name()); /* might come from getpwnam(), need to make a copy */
  pw.pw_dir = 0 as *mut libc::c_char;
  opts = getopt32long(
    argv,
    b"^h:g:s:G:DSHu:k:\x00-1:?2:SD\x00" as *const u8 as *const libc::c_char,
    adduser_longopts.as_ptr(),
    &mut pw.pw_dir as *mut *mut libc::c_char,
    &mut pw.pw_gecos as *mut *mut libc::c_char,
    &mut pw.pw_shell as *mut *mut libc::c_char,
    &mut usegroup as *mut *const libc::c_char,
    &mut uid as *mut *mut libc::c_char,
    &mut skel as *mut *const libc::c_char,
  );
  if opts & (1i32 << 7i32) as libc::c_uint != 0 {
    pw.pw_uid = xatou_range(uid, 0i32 as libc::c_uint, 60000i32 as libc::c_uint)
  }
  argv = argv.offset(optind as isize);
  pw.pw_name = *argv.offset(0);
  if opts == 0 && !(*argv.offset(1)).is_null() {
    /* if called with two non-option arguments, adduser
     * will add an existing user to an existing group.
     */
    return addgroup_wrapper(&mut pw, *argv.offset(1));
  }
  /* fill in the passwd struct */
  if pw.pw_dir.is_null() {
    /* create string for $HOME if not specified already */
    pw.pw_dir = xasprintf(
      b"/home/%s\x00" as *const u8 as *const libc::c_char,
      *argv.offset(0),
    )
  } /* exits on failure */
  pw.pw_passwd = b"x\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
  if opts & (1i32 << 5i32) as libc::c_uint != 0 {
    if usegroup.is_null() {
      usegroup = b"nogroup\x00" as *const u8 as *const libc::c_char
    }
    if opts & (1i32 << 2i32) as libc::c_uint == 0 {
      pw.pw_shell = b"/bin/false\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
  }
  pw.pw_gid = if !usegroup.is_null() {
    xgroup2gid(usegroup)
  } else {
    -1i32 as libc::c_long
  } as gid_t;
  /* make sure everything is kosher and setup uid && maybe gid */
  passwd_study(&mut pw);
  p = xasprintf(
    b"x:%u:%u:%s:%s:%s\x00" as *const u8 as *const libc::c_char,
    pw.pw_uid,
    pw.pw_gid,
    pw.pw_gecos,
    pw.pw_dir,
    pw.pw_shell,
  );
  if update_passwd(
    b"/etc/passwd\x00" as *const u8 as *const libc::c_char,
    pw.pw_name,
    p,
    0 as *const libc::c_char,
  ) < 0i32
  {
    return 1i32;
  }
  /* /etc/shadow fields:
   * 1. username
   * 2. encrypted password
   * 3. last password change (unix date (unix time/24*60*60))
   * 4. minimum days required between password changes
   * 5. maximum days password is valid
   * 6. days before password is to expire that user is warned
   * 7. days after password expires that account is disabled
   * 8. unix date when login expires (i.e. when it may no longer be used)
   */
  /* fields:     2 3  4 5     6 78 */
  p = xasprintf(
    b"!:%u:0:99999:7:::\x00" as *const u8 as *const libc::c_char,
    (time(0 as *mut time_t) as libc::c_uint).wrapping_div((24i32 * 60i32 * 60i32) as libc::c_uint),
  );
  /* ignore errors: if file is missing we suppose admin doesn't want it */
  update_passwd(
    b"/etc/shadow\x00" as *const u8 as *const libc::c_char,
    pw.pw_name,
    p,
    0 as *const libc::c_char,
  );
  /* add to group */
  addgroup_wrapper(&mut pw, usegroup);
  /* clear the umask for this process so it doesn't
   * screw up the permissions on the mkdir and chown. */
  umask(0i32 as mode_t);
  if opts & (1i32 << 6i32) as libc::c_uint == 0 {
    /* set the owner and group so it is owned by the new user,
     * then fix up the permissions to 2755. Can't do it before
     * since chown will clear the setgid bit */
    let mut mkdir_err: libc::c_int = mkdir(pw.pw_dir, 0o755i32 as mode_t);
    if mkdir_err == 0i32 {
      /* New home. Copy /etc/skel to it */
      let mut args: [*const libc::c_char; 5] = [
        b"chown\x00" as *const u8 as *const libc::c_char,
        b"-R\x00" as *const u8 as *const libc::c_char,
        xasprintf(
          b"%u:%u\x00" as *const u8 as *const libc::c_char,
          pw.pw_uid as libc::c_int,
          pw.pw_gid as libc::c_int,
        ) as *const libc::c_char,
        pw.pw_dir as *const libc::c_char,
        0 as *const libc::c_char,
      ];
      /* Be silent on any errors (like: no /etc/skel) */
      if opts & (1i32 << 8i32) as libc::c_uint == 0 {
        logmode = LOGMODE_NONE as libc::c_int as smallint
      }
      copy_file(skel, pw.pw_dir, FILEUTILS_RECUR as libc::c_int);
      logmode = LOGMODE_STDIO as libc::c_int as smallint;
      chown_main(4i32, args.as_mut_ptr() as *mut *mut libc::c_char);
    }
    if mkdir_err != 0i32 && *bb_errno != 17i32
      || chown(pw.pw_dir, pw.pw_uid, pw.pw_gid) != 0i32
      || chmod(pw.pw_dir, 0o2755i32 as mode_t) != 0i32
    {
      /* set setgid bit on homedir */
      bb_simple_perror_msg(pw.pw_dir);
    }
  }
  if opts & (1i32 << 4i32) as libc::c_uint == 0 {
    /* interactively set passwd */
    passwd_wrapper(pw.pw_name);
  }
  return 0i32;
}
