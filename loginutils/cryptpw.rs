use libc;
use libc::open;



extern "C" {
  #[no_mangle]
  fn isatty(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  static mut stdin: *mut FILE;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn safe_strncpy(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    size: size_t,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc_fgetline(file: *mut FILE) -> *mut libc::c_char;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> u32;
  #[no_mangle]
  fn bb_ask_noecho_stdin(prompt: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn pw_encrypt(
    clear: *const libc::c_char,
    salt: *const libc::c_char,
    cleanup: libc::c_int,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn crypt_make_pw_salt(p: *mut libc::c_char, algo: *const libc::c_char) -> *mut libc::c_char;
}

use crate::librb::size_t;


use libc::FILE;

/*
 * cryptpw.c - output a crypt(3)ed password to stdout.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 *
 * Cooked from passwd.c by Thomas Lundquist <thomasez@zelow.no>
 * mkpasswd compatible options added by Bernhard Reutner-Fischer
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config CRYPTPW
//config:	bool "cryptpw (14 kb)"
//config:	default y
//config:	help
//config:	Encrypts the given password with the crypt(3) libc function
//config:	using the given salt.
//config:
//config:config MKPASSWD
//config:	bool "mkpasswd (15 kb)"
//config:	default y
//config:	help
//config:	Encrypts the given password with the crypt(3) libc function
//config:	using the given salt. Debian has this utility under mkpasswd
//config:	name. Busybox provides mkpasswd as an alias for cryptpw.
//applet:IF_CRYPTPW( APPLET_NOEXEC(cryptpw,  cryptpw, BB_DIR_USR_BIN, BB_SUID_DROP, cryptpw))
//                   APPLET_NOEXEC:name      main     location        suid_type     help
//applet:IF_MKPASSWD(APPLET_NOEXEC(mkpasswd, cryptpw, BB_DIR_USR_BIN, BB_SUID_DROP, cryptpw))
//kbuild:lib-$(CONFIG_CRYPTPW) += cryptpw.o
//kbuild:lib-$(CONFIG_MKPASSWD) += cryptpw.o
//usage:#define cryptpw_trivial_usage
//usage:       "[OPTIONS] [PASSWORD] [SALT]"
/* We do support -s, we just don't mention it */
//usage:#define cryptpw_full_usage "\n\n"
//usage:       "Print crypt(3) hashed PASSWORD\n"
//usage:	IF_LONG_OPTS(
//usage:     "\n	-P,--password-fd N	Read password from fd N"
/* //usage:  "\n	-s,--stdin		Use stdin; like -P0" */
//usage:     "\n	-m,--method TYPE	"CRYPT_METHODS_HELP_STR
//usage:     "\n	-S,--salt SALT"
//usage:	)
//usage:	IF_NOT_LONG_OPTS(
//usage:     "\n	-P N	Read password from fd N"
/* //usage:  "\n	-s	Use stdin; like -P0" */
//usage:     "\n	-m TYPE	"CRYPT_METHODS_HELP_STR
//usage:     "\n	-S SALT"
//usage:	)
/* Debian has 'mkpasswd' utility, manpage says:

NAME
    mkpasswd - Overfeatured front end to crypt(3)
SYNOPSIS
    mkpasswd PASSWORD SALT
...
OPTIONS
-S, --salt=STRING
    Use the STRING as salt. It must not  contain  prefixes  such  as
    $1$.
-R, --rounds=NUMBER
    Use NUMBER rounds. This argument is ignored if the method
    chosen does not support variable rounds. For the OpenBSD Blowfish
    method this is the logarithm of the number of rounds.
-m, --method=TYPE
    Compute the password using the TYPE method. If TYPE is 'help'
    then the available methods are printed.
-P, --password-fd=NUM
    Read the password from file descriptor NUM instead of using getpass(3).
    If the file descriptor is not connected to a tty then
    no other message than the hashed password is printed on stdout.
-s, --stdin
    Like --password-fd=0.
ENVIRONMENT
    $MKPASSWD_OPTIONS
    A list of options which will be evaluated before the ones
    specified on the command line.
BUGS
    This programs suffers of a bad case of featuritis.
    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Very true...

cryptpw was in bbox before this gem, so we retain it, and alias mkpasswd
to cryptpw. -a option (alias for -m) came from cryptpw.
*/
#[no_mangle]
pub unsafe extern "C" fn cryptpw_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  /* Supports: cryptpw -m sha256 PASS 'rounds=999999999$SALT' */
  let mut salt: [libc::c_char; 38] = [0; 38];
  let mut salt_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut password: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut opt_m: *const libc::c_char = 0 as *const libc::c_char;
  let mut opt_S: *const libc::c_char = 0 as *const libc::c_char;
  let mut fd: libc::c_int = 0;
  static mut mkpasswd_longopts: [libc::c_char; 39] = [
    115, 116, 100, 105, 110, 0, 0, 115, 112, 97, 115, 115, 119, 111, 114, 100, 45, 102, 100, 0, 1,
    80, 115, 97, 108, 116, 0, 1, 83, 109, 101, 116, 104, 111, 100, 0, 1, 109, 0,
  ];
  fd = 0i32;
  opt_m = b"des\x00" as *const u8 as *const libc::c_char;
  opt_S = 0 as *const libc::c_char;
  /* at most two non-option arguments; -P NUM */
  getopt32long(
    argv,
    b"^sP:+S:m:a:\x00?2\x00" as *const u8 as *const libc::c_char,
    mkpasswd_longopts.as_ptr(),
    &mut fd as *mut libc::c_int,
    &mut opt_S as *mut *const libc::c_char,
    &mut opt_m as *mut *const libc::c_char,
    &mut opt_m as *mut *const libc::c_char,
  );
  argv = argv.offset(optind as isize);
  /* have no idea how to handle -s... */
  if !(*argv.offset(0)).is_null() && opt_S.is_null() {
    opt_S = *argv.offset(1)
  }
  salt_ptr = crypt_make_pw_salt(salt.as_mut_ptr(), opt_m);
  if !opt_S.is_null() {
    /* put user's data after the "$N$" prefix */
    safe_strncpy(
      salt_ptr,
      opt_S,
      (::std::mem::size_of::<[libc::c_char; 38]>() as libc::c_ulong).wrapping_sub(
        (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
          .wrapping_sub(1i32 as libc::c_ulong),
      ),
    );
  }
  xmove_fd(fd, 0i32);
  password = *argv.offset(0);
  if password.is_null() {
    /* Only mkpasswd, and only from tty, prompts.
     * Otherwise it is a plain read. */
    password =
      if 1i32 != 0 && *applet_name.offset(0) as libc::c_int == 'm' as i32 && isatty(0i32) != 0 {
        bb_ask_noecho_stdin(b"Password: \x00" as *const u8 as *const libc::c_char)
      } else {
        xmalloc_fgetline(stdin)
      }
    /* may still be NULL on EOF/error */
  }
  if !password.is_null() {
    puts(pw_encrypt(password, salt.as_mut_ptr(), 1i32));
  }
  return 0i32;
}
