use crate::librb::smallint;
use libc;
use libc::putenv;
use libc::umask;
use libc::mknod;
use libc::fchmod;
use libc::fscanf;
use libc::alarm;
use libc::sync;
use libc::setsid;
use libc::ioctl;
use libc::statfs;
use libc::mount;
use libc::prctl;
use libc::opendir;
use libc::closedir;
use libc::readdir;
use libc::strtok;
use libc::putchar_unlocked;
use libc::endmntent;
use libc::setmntent;
use libc::umount2;
use libc::getegid;
use libc::getuid;
use libc::getgid;
use libc::setutxent;
use libc::endutxent;

use libc::strchr;

use libc::free;
use libc::uid_t;
use libc::FILE;

extern "C" {

  #[no_mangle]
  static mut stdin: *mut FILE;

  #[no_mangle]
  fn xchroot(path: *const libc::c_char);
  #[no_mangle]
  fn xmalloc_fgetline(file: *mut FILE) -> *mut libc::c_char;
  #[no_mangle]
  fn xuname2uid(name: *const libc::c_char) -> libc::c_long;
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
  fn bb_info_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn pw_encrypt(
    clear: *const libc::c_char,
    salt: *const libc::c_char,
    cleanup: libc::c_int,
  ) -> *mut libc::c_char;
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
  static bb_msg_perm_denied_are_you_root: [libc::c_char; 0];
}

pub type C2RustUnnamed = libc::c_uint;
pub const LOGMODE_BOTH: C2RustUnnamed = 3;
pub const LOGMODE_SYSLOG: C2RustUnnamed = 2;
pub const LOGMODE_STDIO: C2RustUnnamed = 1;
pub const LOGMODE_NONE: C2RustUnnamed = 0;

/*
 * chpasswd.c
 *
 * Written for SLIND (from passwd.c) by Alexander Shishkin <virtuoso@slind.org>
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config CHPASSWD
//config:	bool "chpasswd (18 kb)"
//config:	default y
//config:	help
//config:	Reads a file of user name and password pairs from standard input
//config:	and uses this information to update a group of existing users.
//config:
//config:config FEATURE_DEFAULT_PASSWD_ALGO
//config:	string "Default encryption method (passwd -a, cryptpw -m, chpasswd -c ALG)"
//config:	default "des"
//config:	depends on PASSWD || CRYPTPW || CHPASSWD
//config:	help
//config:	Possible choices are "d[es]", "m[d5]", "s[ha256]" or "sha512".
//applet:IF_CHPASSWD(APPLET(chpasswd, BB_DIR_USR_SBIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_CHPASSWD) += chpasswd.o
//usage:#define chpasswd_trivial_usage
//usage:	IF_LONG_OPTS("[--md5|--encrypted|--crypt-method|--root]") IF_NOT_LONG_OPTS("[-m|-e|-c|-R]")
//usage:#define chpasswd_full_usage "\n\n"
//usage:       "Read user:password from stdin and update /etc/passwd\n"
//usage:	IF_LONG_OPTS(
//usage:     "\n	-e,--encrypted		Supplied passwords are in encrypted form"
//usage:     "\n	-m,--md5		Encrypt using md5, not des"
//usage:     "\n	-c,--crypt-method ALG	"CRYPT_METHODS_HELP_STR
//usage:     "\n	-R,--root DIR		Directory to chroot into"
//usage:	)
//usage:	IF_NOT_LONG_OPTS(
//usage:     "\n	-e	Supplied passwords are in encrypted form"
//usage:     "\n	-m	Encrypt using md5, not des"
//usage:     "\n	-c ALG	"CRYPT_METHODS_HELP_STR
//usage:     "\n	-R DIR	Directory to chroot into"
//usage:	)
static mut chpasswd_longopts: [libc::c_char; 41] = [
  101, 110, 99, 114, 121, 112, 116, 101, 100, 0, 0, 101, 109, 100, 53, 0, 0, 109, 99, 114, 121,
  112, 116, 45, 109, 101, 116, 104, 111, 100, 0, 1, 99, 114, 111, 111, 116, 0, 1, 82, 0,
];
#[no_mangle]
pub unsafe extern "C" fn chpasswd_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut name: *mut libc::c_char = 0 as *mut libc::c_char; /* dies if there is no such user */
  let mut algo: *const libc::c_char = b"des\x00" as *const u8 as *const libc::c_char;
  let mut root: *const libc::c_char = 0 as *const libc::c_char;
  let mut opt: libc::c_int = 0;
  if getuid() != 0i32 as libc::c_uint {
    bb_simple_error_msg_and_die(bb_msg_perm_denied_are_you_root.as_ptr());
  }
  opt = getopt32long(
    argv,
    b"^emc:R:\x00m--ec:e--mc:c--em\x00" as *const u8 as *const libc::c_char,
    chpasswd_longopts.as_ptr(),
    &mut algo as *mut *const libc::c_char,
    &mut root as *mut *const libc::c_char,
  ) as libc::c_int;
  if !root.is_null() {
    xchroot(root);
  }
  loop {
    name = xmalloc_fgetline(stdin);
    if name.is_null() {
      break;
    }
    let mut free_me: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pass: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rc: libc::c_int = 0;
    pass = strchr(name, ':' as i32);
    if pass.is_null() {
      bb_simple_error_msg_and_die(b"missing new password\x00" as *const u8 as *const libc::c_char);
    }
    let fresh0 = pass;
    pass = pass.offset(1);
    *fresh0 = '\u{0}' as i32 as libc::c_char;
    xuname2uid(name);
    free_me = 0 as *mut libc::c_char;
    if opt & 1i32 == 0 {
      let mut salt: [libc::c_char; 20] = [0; 20];
      if opt & 2i32 != 0 {
        /* Force MD5 if the -m flag is set */
        algo = b"md5\x00" as *const u8 as *const libc::c_char
      }
      crypt_make_pw_salt(salt.as_mut_ptr(), algo);
      pass = pw_encrypt(pass, salt.as_mut_ptr(), 0i32);
      free_me = pass
    }
    /* This is rather complex: if user is not found in /etc/shadow,
     * we try to find & change his passwd in /etc/passwd */
    rc = update_passwd(
      b"/etc/shadow\x00" as *const u8 as *const libc::c_char,
      name,
      pass,
      0 as *const libc::c_char,
    );
    if rc > 0i32 {
      /* password in /etc/shadow was updated */
      pass = b"x\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    if rc >= 0i32 {
      /* 0 = /etc/shadow missing (not an error), >0 = passwd changed in /etc/shadow */
      rc = update_passwd(
        b"/etc/passwd\x00" as *const u8 as *const libc::c_char,
        name,
        pass,
        0 as *const libc::c_char,
      )
    }
    /* LOGMODE_BOTH logs to syslog also */
    logmode = LOGMODE_BOTH as libc::c_int as smallint;
    if rc < 0i32 {
      bb_error_msg_and_die(
        b"an error occurred updating password for %s\x00" as *const u8 as *const libc::c_char,
        name,
      );
    }
    if rc != 0 {
      bb_info_msg(
        b"password for \'%s\' changed\x00" as *const u8 as *const libc::c_char,
        name,
      );
    }
    logmode = LOGMODE_STDIO as libc::c_int as smallint;
    free(name as *mut libc::c_void);
    free(free_me as *mut libc::c_void);
  }
  return 0i32;
}
