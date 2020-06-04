use crate::libbb::appletlib::applet_name;
use crate::libbb::ptr_to_globals::bb_errno;
use crate::librb::signal::__sighandler_t;
use crate::librb::smallint;
use crate::librb::spwd;
use libc;
use libc::getuid;
use libc::mode_t;
use libc::openlog;
use libc::passwd;
use libc::printf;
use libc::puts;
use libc::strcmp;
use libc::syslog;
use libc::uid_t;
use libc::umask;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn setrlimit(__resource: __rlimit_resource_t, __rlimits: *const rlimit) -> libc::c_int;

  /* All function names below should be remapped by #defines above
   * in order to not collide with libc names. */

  #[no_mangle]
  static mut logmode: smallint;

}

pub type __rlim64_t = libc::c_ulong;
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rlimit {
  pub rlim_cur: rlim_t,
  pub rlim_max: rlim_t,
}
pub type __rlimit_resource_t = __rlimit_resource;
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
//applet:IF_PASSWD(APPLET(passwd, BB_DIR_USR_BIN, SUID_REQUIRE))
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
unsafe fn new_password(
  mut pw: *const passwd,
  mut myuid: uid_t,
  mut algo: *const libc::c_char,
) -> *mut libc::c_char {
  let mut current_block: u64; /* failure so far */
  let mut salt: [libc::c_char; 20] = [0; 20]; /* returns malloced str */
  let mut orig: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>(); /* returns malloced str */
  let mut newp: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>(); /* returns malloced str */
  let mut cp: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut ret: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  if myuid != 0 as libc::c_uint && *(*pw).pw_passwd.offset(0) as libc::c_int != 0 {
    let mut encrypted: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    orig = crate::libbb::bb_askpass::bb_ask_noecho_stdin(
      b"Old password: \x00" as *const u8 as *const libc::c_char,
    );
    if orig.is_null() {
      current_block = 8393339141576953219;
    } else {
      encrypted = crate::libbb::pw_encrypt::pw_encrypt(orig, (*pw).pw_passwd, 1i32);
      if strcmp(encrypted, (*pw).pw_passwd) != 0 {
        syslog(
          4i32,
          b"incorrect password for %s\x00" as *const u8 as *const libc::c_char,
          (*pw).pw_name,
        );
        crate::libbb::bb_do_delay::bb_do_delay(3i32);
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
      newp = crate::libbb::bb_askpass::bb_ask_noecho_stdin(
        b"New password: \x00" as *const u8 as *const libc::c_char,
      );
      if !newp.is_null() {
        if !(1i32 != 0
          && crate::libbb::obscure::obscure(orig, newp, pw) != 0
          && myuid != 0 as libc::c_uint)
        {
          cp = crate::libbb::bb_askpass::bb_ask_noecho_stdin(
            b"Retype password: \x00" as *const u8 as *const libc::c_char,
          );
          if !cp.is_null() {
            if strcmp(cp, newp) != 0 {
              puts(b"Passwords don\'t match\x00" as *const u8 as *const libc::c_char);
            } else {
              crate::libbb::pw_encrypt::crypt_make_pw_salt(salt.as_mut_ptr(), algo);
              /* pw_encrypt returns malloced str */
              ret = crate::libbb::pw_encrypt::pw_encrypt(newp, salt.as_mut_ptr(), 1i32)
            }
          }
        }
      }
    }
    _ => {}
  }
  /* whee, success! */
  crate::libbb::nuke_str::nuke_str(orig);
  crate::libbb::nuke_str::nuke_str(newp);
  crate::libbb::nuke_str::nuke_str(cp);
  return ret;
}
pub unsafe fn passwd_main(mut _argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut current_block: u64;
  let mut opt: libc::c_uint = 0;
  let mut rc: libc::c_int = 0;
  let mut opt_a: *const libc::c_char = b"des\x00" as *const u8 as *const libc::c_char;
  let mut filename: *const libc::c_char = std::ptr::null();
  let mut myname: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut name: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut newp: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut pw: *mut passwd = std::ptr::null_mut();
  let mut myuid: uid_t = 0;
  let mut rlimit_fsize: rlimit = rlimit {
    rlim_cur: 0,
    rlim_max: 0,
  };
  let mut c: libc::c_char = 0;
  /* Using _r function to avoid pulling in static buffers */
  let mut spw: spwd = spwd {
    sp_namp: std::ptr::null_mut::<libc::c_char>(),
    sp_pwdp: std::ptr::null_mut::<libc::c_char>(),
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
  openlog(applet_name, 0, 4i32 << 3i32);
  opt = crate::libbb::getopt32::getopt32(
    argv,
    b"a:lud\x00" as *const u8 as *const libc::c_char,
    &mut opt_a as *mut *const libc::c_char,
  );
  //argc -= optind;
  argv = argv.offset(optind as isize);
  myuid = getuid();
  /* -l, -u, -d require root priv and username argument */
  if opt & OPT_lud as libc::c_int as libc::c_uint != 0
    && (myuid != 0 as libc::c_uint || (*argv.offset(0)).is_null())
  {
    crate::libbb::appletlib::bb_show_usage();
  }
  /* Will complain and die if username not found */
  myname = crate::libbb::xfuncs_printf::xstrdup(crate::libbb::bb_pwd::xuid2uname(myuid));
  name = if !(*argv.offset(0)).is_null() {
    *argv.offset(0)
  } else {
    myname
  };
  pw = crate::libbb::bb_pwd::xgetpwnam(name);
  if myuid != 0 as libc::c_uint && (*pw).pw_uid != myuid {
    /* LOGMODE_BOTH */
    crate::libbb::verror_msg::bb_error_msg_and_die(
      b"%s can\'t change password for %s\x00" as *const u8 as *const libc::c_char,
      myname,
      name,
    );
  }
  /* getspnam_r may return 0 yet set result to NULL.
   * At least glibc 2.4 does this. Be extra paranoid here. */
  let mut result: *mut spwd = std::ptr::null_mut();
  *bb_errno = 0;
  if crate::libpwdgrp::pwd_grp::bb_internal_getspnam_r(
    (*pw).pw_name,
    &mut spw,
    buffer.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
    &mut result,
  ) != 0
    || result.is_null()
    || strcmp((*result).sp_namp, (*pw).pw_name) != 0
  {
    /* paranoia */
    if *bb_errno != 2i32 {
      /* LOGMODE_BOTH */
      crate::libbb::perror_msg::bb_perror_msg(
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
  newp = std::ptr::null_mut::<libc::c_char>();
  c = (*(*pw).pw_passwd.offset(0) as libc::c_int - '!' as i32) as libc::c_char;
  if opt & OPT_lud as libc::c_int as libc::c_uint == 0 {
    if myuid != 0 as libc::c_uint && c == 0 {
      /* passwd starts with '!' */
      /* LOGMODE_BOTH */
      crate::libbb::verror_msg::bb_error_msg_and_die(
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
      crate::libbb::verror_msg::bb_error_msg_and_die(
        b"password for %s is unchanged\x00" as *const u8 as *const libc::c_char,
        name,
      );
    }
    current_block = 11763295167351361500;
  } else if opt & OPT_lock as libc::c_int as libc::c_uint != 0 {
    if c == 0 {
      current_block = 9419412629514855253;
    } else {
      newp = crate::libbb::xfuncs_printf::xasprintf(
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
      newp = crate::libbb::xfuncs_printf::xstrdup(&mut *(*pw).pw_passwd.offset(1));
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
      rlimit_fsize.rlim_max = (512i64 * 30000i32 as i64) as rlim_t;
      rlimit_fsize.rlim_cur = rlimit_fsize.rlim_max;
      setrlimit(RLIMIT_FSIZE, &mut rlimit_fsize);
      crate::libbb::signals::bb_signals(
        0 + (1i32 << 1i32) + (1i32 << 2i32) + (1i32 << 3i32),
        ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
      );
      umask(0o77i32 as mode_t);
      crate::libbb::xfuncs_printf::xsetuid(0i32 as uid_t);
      filename = b"/etc/shadow\x00" as *const u8 as *const libc::c_char;
      rc = crate::libbb::update_passwd::update_passwd(
        b"/etc/shadow\x00" as *const u8 as *const libc::c_char,
        name,
        newp,
        0 as *const libc::c_char,
      );
      if rc > 0 {
        /* password in /etc/shadow was updated */
        newp = b"x\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      }
      if rc >= 0 {
        /* 0 = /etc/shadow missing (not an error), >0 = passwd changed in /etc/shadow */
        filename = b"/etc/passwd\x00" as *const u8 as *const libc::c_char;
        rc = crate::libbb::update_passwd::update_passwd(
          b"/etc/passwd\x00" as *const u8 as *const libc::c_char,
          name,
          newp,
          0 as *const libc::c_char,
        )
      }
      /* LOGMODE_BOTH */
      if rc < 0 {
        crate::libbb::verror_msg::bb_error_msg_and_die(
          b"can\'t update password file %s\x00" as *const u8 as *const libc::c_char,
          filename,
        );
      }
      crate::libbb::verror_msg::bb_info_msg(
        b"password for %s changed by %s\x00" as *const u8 as *const libc::c_char,
        name,
        myname,
      );
    }
    _ => {}
  }
  /*if (ENABLE_FEATURE_CLEAN_UP) free(newp); - can't, it may be non-malloced */
  if newp.is_null() {
    crate::libbb::verror_msg::bb_error_msg_and_die(
      b"password for %s is already %slocked\x00" as *const u8 as *const libc::c_char,
      name,
      if opt & OPT_unlock as libc::c_int as libc::c_uint != 0 {
        b"un\x00" as *const u8 as *const libc::c_char
      } else {
        b"\x00" as *const u8 as *const libc::c_char
      },
    );
  }
  return 0;
}
