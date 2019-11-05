use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn getuid() -> __uid_t;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn setrlimit(__resource: __rlimit_resource_t, __rlimits: *const rlimit) -> libc::c_int;
  #[no_mangle]
  fn umask(__mask: __mode_t) -> __mode_t;
  /* All function names below should be remapped by #defines above
   * in order to not collide with libc names. */
  #[no_mangle]
  fn bb_internal_getspnam_r(
    __name: *const libc::c_char,
    __result_buf: *mut spwd,
    __buffer: *mut libc::c_char,
    __buflen: size_t,
    __result: *mut *mut spwd,
  ) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_signals(sigs: libc::c_int, f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>);
  #[no_mangle]
  fn xsetuid(uid: uid_t);
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn xgetpwnam(name: *const libc::c_char) -> *mut passwd;
  #[no_mangle]
  fn xuid2uname(uid: uid_t) -> *mut libc::c_char;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  static mut logmode: smallint;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_info_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_ask_noecho_stdin(prompt: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_do_delay(seconds: libc::c_int);
  #[no_mangle]
  fn nuke_str(str: *mut libc::c_char);
  #[no_mangle]
  fn pw_encrypt(
    clear: *const libc::c_char,
    salt: *const libc::c_char,
    cleanup: libc::c_int,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn obscure(
    old: *const libc::c_char,
    newval: *const libc::c_char,
    pwdp: *const passwd,
  ) -> libc::c_int;
  #[no_mangle]
  fn crypt_make_pw_salt(p: *mut libc::c_char, algo: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn update_passwd(
    filename: *const libc::c_char,
    username: *const libc::c_char,
    data: *const libc::c_char,
    member: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  fn openlog(__ident: *const libc::c_char, __option: libc::c_int, __facility: libc::c_int);
  #[no_mangle]
  fn syslog(__pri: libc::c_int, __fmt: *const libc::c_char, _: ...);
}
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __rlim64_t = libc::c_ulong;
use crate::librb::uint32_t;
use crate::librb::smallint;
pub type size_t = libc::c_ulong;
use crate::librb::uid_t;
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
pub type __rlimit_resource = libc::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;
pub type rlim_t = __rlim64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
  pub rlim_cur: rlim_t,
  pub rlim_max: rlim_t,
}
pub type __rlimit_resource_t = __rlimit_resource;
use crate::librb::passwd;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spwd {
  pub sp_namp: *mut libc::c_char,
  pub sp_pwdp: *mut libc::c_char,
  pub sp_lstchg: libc::c_long,
  pub sp_min: libc::c_long,
  pub sp_max: libc::c_long,
  pub sp_warn: libc::c_long,
  pub sp_inact: libc::c_long,
  pub sp_expire: libc::c_long,
  pub sp_flag: libc::c_ulong,
}
pub type C2RustUnnamed = libc::c_uint;
pub const LOGMODE_BOTH: C2RustUnnamed = 3;
pub const LOGMODE_SYSLOG: C2RustUnnamed = 2;
pub const LOGMODE_STDIO: C2RustUnnamed = 1;
pub const LOGMODE_NONE: C2RustUnnamed = 0;
/* -l - lock account */
pub const OPT_unlock: C2RustUnnamed_0 = 4;
/* -u - unlock account */
pub const OPT_delete: C2RustUnnamed_0 = 8;
/* -a - password algorithm */
pub const OPT_lock: C2RustUnnamed_0 = 2;
/* -d - delete password */
pub const OPT_lud: C2RustUnnamed_0 = 14;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const OPT_algo: C2RustUnnamed_0 = 1;
/* vi: set sw=4 ts=4: */
/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config PASSWD
//config:	bool "passwd (21 kb)"
//config:	default y
//config:	select FEATURE_SYSLOG
//config:	help
//config:	passwd changes passwords for user and group accounts. A normal user
//config:	may only change the password for his/her own account, the super user
//config:	may change the password for any account. The administrator of a group
//config:	may change the password for the group.
//config:
//config:	Note that busybox binary must be setuid root for this applet to
//config:	work properly.
//config:
//config:config FEATURE_PASSWD_WEAK_CHECK
//config:	bool "Check new passwords for weakness"
//config:	default y
//config:	depends on PASSWD
//config:	help
//config:	With this option passwd will refuse new passwords which are "weak".
//applet:/* Needs to be run by root or be suid root - needs to change /etc/{passwd,shadow}: */
//applet:IF_PASSWD(APPLET(passwd, BB_DIR_USR_BIN, BB_SUID_REQUIRE))
//kbuild:lib-$(CONFIG_PASSWD) += passwd.o
//usage:#define passwd_trivial_usage
//usage:       "[OPTIONS] [USER]"
//usage:#define passwd_full_usage "\n\n"
//usage:       "Change USER's password (default: current user)"
//usage:     "\n"
//usage:     "\n	-a ALG	"CRYPT_METHODS_HELP_STR
//usage:     "\n	-d	Set password to ''"
//usage:     "\n	-l	Lock (disable) account"
//usage:     "\n	-u	Unlock (enable) account"
unsafe extern "C" fn new_password(
  mut pw: *const passwd,
  mut myuid: uid_t,
  mut algo: *const libc::c_char,
) -> *mut libc::c_char {
  let mut current_block: u64; /* failure so far */
  let mut salt: [libc::c_char; 20] = [0; 20]; /* returns malloced str */
  let mut orig: *mut libc::c_char = 0 as *mut libc::c_char; /* returns malloced str */
  let mut newp: *mut libc::c_char = 0 as *mut libc::c_char; /* returns malloced str */
  let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
  if myuid != 0i32 as libc::c_uint && *(*pw).pw_passwd.offset(0) as libc::c_int != 0 {
    let mut encrypted: *mut libc::c_char = 0 as *mut libc::c_char;
    orig = bb_ask_noecho_stdin(b"Old password: \x00" as *const u8 as *const libc::c_char);
    if orig.is_null() {
      current_block = 8393339141576953219;
    } else {
      encrypted = pw_encrypt(orig, (*pw).pw_passwd, 1i32);
      if strcmp(encrypted, (*pw).pw_passwd) != 0i32 {
        syslog(
          4i32,
          b"incorrect password for %s\x00" as *const u8 as *const libc::c_char,
          (*pw).pw_name,
        );
        bb_do_delay(3i32);
        puts(b"Incorrect password\x00" as *const u8 as *const libc::c_char);
        current_block = 8393339141576953219;
      } else {
        current_block = 4166486009154926805;
      }
    }
  } else {
    current_block = 4166486009154926805;
  }
  match current_block {
    4166486009154926805 => {
      newp = bb_ask_noecho_stdin(b"New password: \x00" as *const u8 as *const libc::c_char);
      if !newp.is_null() {
        if !(1i32 != 0 && obscure(orig, newp, pw) != 0 && myuid != 0i32 as libc::c_uint) {
          cp = bb_ask_noecho_stdin(b"Retype password: \x00" as *const u8 as *const libc::c_char);
          if !cp.is_null() {
            if strcmp(cp, newp) != 0i32 {
              puts(b"Passwords don\'t match\x00" as *const u8 as *const libc::c_char);
            } else {
              crypt_make_pw_salt(salt.as_mut_ptr(), algo);
              /* pw_encrypt returns malloced str */
              ret = pw_encrypt(newp, salt.as_mut_ptr(), 1i32)
            }
          }
        }
      }
    }
    _ => {}
  }
  /* whee, success! */
  nuke_str(orig);
  nuke_str(newp);
  nuke_str(cp);
  return ret;
}
#[no_mangle]
pub unsafe extern "C" fn passwd_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut opt: libc::c_uint = 0;
  let mut rc: libc::c_int = 0;
  let mut opt_a: *const libc::c_char = b"des\x00" as *const u8 as *const libc::c_char;
  let mut filename: *const libc::c_char = 0 as *const libc::c_char;
  let mut myname: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut newp: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut pw: *mut passwd = 0 as *mut passwd;
  let mut myuid: uid_t = 0;
  let mut rlimit_fsize: rlimit = rlimit {
    rlim_cur: 0,
    rlim_max: 0,
  };
  let mut c: libc::c_char = 0;
  /* Using _r function to avoid pulling in static buffers */
  let mut spw: spwd = spwd {
    sp_namp: 0 as *mut libc::c_char,
    sp_pwdp: 0 as *mut libc::c_char,
    sp_lstchg: 0,
    sp_min: 0,
    sp_max: 0,
    sp_warn: 0,
    sp_inact: 0,
    sp_expire: 0,
    sp_flag: 0,
  };
  let mut buffer: [libc::c_char; 256] = [0; 256];
  logmode = LOGMODE_BOTH as libc::c_int as smallint;
  openlog(applet_name, 0i32, 4i32 << 3i32);
  opt = getopt32(
    argv,
    b"a:lud\x00" as *const u8 as *const libc::c_char,
    &mut opt_a as *mut *const libc::c_char,
  );
  //argc -= optind;
  argv = argv.offset(optind as isize);
  myuid = getuid();
  /* -l, -u, -d require root priv and username argument */
  if opt & OPT_lud as libc::c_int as libc::c_uint != 0
    && (myuid != 0i32 as libc::c_uint || (*argv.offset(0)).is_null())
  {
    bb_show_usage();
  }
  /* Will complain and die if username not found */
  myname = xstrdup(xuid2uname(myuid));
  name = if !(*argv.offset(0)).is_null() {
    *argv.offset(0)
  } else {
    myname
  };
  pw = xgetpwnam(name);
  if myuid != 0i32 as libc::c_uint && (*pw).pw_uid != myuid {
    /* LOGMODE_BOTH */
    bb_error_msg_and_die(
      b"%s can\'t change password for %s\x00" as *const u8 as *const libc::c_char,
      myname,
      name,
    );
  }
  /* getspnam_r may return 0 yet set result to NULL.
   * At least glibc 2.4 does this. Be extra paranoid here. */
  let mut result: *mut spwd = 0 as *mut spwd;
  *bb_errno = 0i32;
  if bb_internal_getspnam_r(
    (*pw).pw_name,
    &mut spw,
    buffer.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
    &mut result,
  ) != 0i32
    || result.is_null()
    || strcmp((*result).sp_namp, (*pw).pw_name) != 0i32
  {
    /* paranoia */
    if *bb_errno != 2i32 {
      /* LOGMODE_BOTH */
      bb_perror_msg(
        b"no record of %s in %s, using %s\x00" as *const u8 as *const libc::c_char,
        name,
        b"/etc/shadow\x00" as *const u8 as *const libc::c_char,
        b"/etc/passwd\x00" as *const u8 as *const libc::c_char,
      );
    }
  /* else: /etc/shadow does not exist,
   * apparently we are on a shadow-less system,
   * no surprise there */
  } else {
    (*pw).pw_passwd = (*result).sp_pwdp
  }
  /* Decide what the new password will be */
  newp = 0 as *mut libc::c_char;
  c = (*(*pw).pw_passwd.offset(0) as libc::c_int - '!' as i32) as libc::c_char;
  if opt & OPT_lud as libc::c_int as libc::c_uint == 0 {
    if myuid != 0i32 as libc::c_uint && c == 0 {
      /* passwd starts with '!' */
      /* LOGMODE_BOTH */
      bb_error_msg_and_die(
        b"can\'t change locked password for %s\x00" as *const u8 as *const libc::c_char,
        name,
      ); /* passwd starts with '!' */
    } /* not '!' */
    printf(
      b"Changing password for %s\n\x00" as *const u8 as *const libc::c_char,
      name,
    );
    newp = new_password(pw, myuid, opt_a);
    if newp.is_null() {
      logmode = LOGMODE_STDIO as libc::c_int as smallint;
      bb_error_msg_and_die(
        b"password for %s is unchanged\x00" as *const u8 as *const libc::c_char,
        name,
      );
    }
    current_block = 11763295167351361500;
  } else if opt & OPT_lock as libc::c_int as libc::c_uint != 0 {
    if c == 0 {
      current_block = 9419412629514855253;
    } else {
      newp = xasprintf(
        b"!%s\x00" as *const u8 as *const libc::c_char,
        (*pw).pw_passwd,
      );
      current_block = 11763295167351361500;
    }
  } else if opt & OPT_unlock as libc::c_int as libc::c_uint != 0 {
    if c != 0 {
      current_block = 9419412629514855253;
    } else {
      /* pw->pw_passwd points to static storage,
       * strdup'ing to avoid nasty surprizes */
      newp = xstrdup(&mut *(*pw).pw_passwd.offset(1));
      current_block = 11763295167351361500;
    }
  } else {
    if opt & OPT_delete as libc::c_int as libc::c_uint != 0 {
      newp = b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    current_block = 11763295167351361500;
  }
  match current_block {
    11763295167351361500 => {
      rlimit_fsize.rlim_max = (512i64 * 30000i32 as libc::c_long) as rlim_t;
      rlimit_fsize.rlim_cur = rlimit_fsize.rlim_max;
      setrlimit(RLIMIT_FSIZE, &mut rlimit_fsize);
      bb_signals(
        0i32 + (1i32 << 1i32) + (1i32 << 2i32) + (1i32 << 3i32),
        ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
      );
      umask(0o77i32 as __mode_t);
      xsetuid(0i32 as uid_t);
      filename = b"/etc/shadow\x00" as *const u8 as *const libc::c_char;
      rc = update_passwd(
        b"/etc/shadow\x00" as *const u8 as *const libc::c_char,
        name,
        newp,
        0 as *const libc::c_char,
      );
      if rc > 0i32 {
        /* password in /etc/shadow was updated */
        newp = b"x\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      }
      if rc >= 0i32 {
        /* 0 = /etc/shadow missing (not an error), >0 = passwd changed in /etc/shadow */
        filename = b"/etc/passwd\x00" as *const u8 as *const libc::c_char;
        rc = update_passwd(
          b"/etc/passwd\x00" as *const u8 as *const libc::c_char,
          name,
          newp,
          0 as *const libc::c_char,
        )
      }
      /* LOGMODE_BOTH */
      if rc < 0i32 {
        bb_error_msg_and_die(
          b"can\'t update password file %s\x00" as *const u8 as *const libc::c_char,
          filename,
        );
      }
      bb_info_msg(
        b"password for %s changed by %s\x00" as *const u8 as *const libc::c_char,
        name,
        myname,
      );
    }
    _ => {}
  }
  /*if (ENABLE_FEATURE_CLEAN_UP) free(newp); - can't, it may be non-malloced */
  if newp.is_null() {
    bb_error_msg_and_die(
      b"password for %s is already %slocked\x00" as *const u8 as *const libc::c_char,
      name,
      if opt & OPT_unlock as libc::c_int as libc::c_uint != 0 {
        b"un\x00" as *const u8 as *const libc::c_char
      } else {
        b"\x00" as *const u8 as *const libc::c_char
      },
    );
  }
  return 0i32;
}
