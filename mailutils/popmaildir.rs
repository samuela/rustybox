use libc;
extern "C" {
  #[no_mangle]
  fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn getpid() -> __pid_t;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static ptr_to_globals: *mut globals;
  #[no_mangle]
  static mut stdin: *mut FILE;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn pclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn monotonic_us() -> libc::c_ulonglong;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xchdir(path: *const libc::c_char);
  #[no_mangle]
  fn xsetenv(key: *const libc::c_char, value: *const libc::c_char);
  #[no_mangle]
  fn rename_or_warn(oldpath: *const libc::c_char, newpath: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc_fgets_str(
    file: *mut FILE,
    terminating_string: *const libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc_fgetline(file: *mut FILE) -> *mut libc::c_char;
  #[no_mangle]
  fn xfopen_for_write(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn safe_gethostname() -> *mut libc::c_char;
  #[no_mangle]
  fn bin2hex(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    count: libc::c_int,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn md5_begin(ctx: *mut md5_ctx_t);
  #[no_mangle]
  fn md5_hash(ctx: *mut md5_ctx_t, buffer: *const libc::c_void, len: size_t);
  #[no_mangle]
  fn md5_end(ctx: *mut md5_ctx_t, resbuf: *mut libc::c_void) -> libc::c_uint;
  //char FAST_FUNC *parse_url(char *url, char **user, char **pass);
  #[no_mangle]
  fn launch_helper(argv: *mut *const libc::c_char);
  #[no_mangle]
  fn get_cred_or_die(fd: libc::c_int);
  #[no_mangle]
  fn send_mail_command(fmt: *const libc::c_char, param: *const libc::c_char) -> *mut libc::c_char;
}

use crate::librb::__pid_t;

use libc::uint32_t;

use crate::librb::pid_t;
use crate::librb::size_t;

use crate::librb::md5_ctx_t;
use crate::librb::ptrdiff_t;
use libc::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub helper_pid: pid_t,
  pub timeout: libc::c_uint,
  pub verbose: libc::c_uint,
  pub opts: libc::c_uint,
  pub user: *mut libc::c_char,
  pub pass: *mut libc::c_char,
  pub fp0: *mut FILE,
  pub opt_charset: *mut libc::c_char,
}
// -T get messages with TOP instead with RETR
pub const OPT_k: C2RustUnnamed_0 = 256;
// -H30 type first 30 lines of a message; (-L12000 -H30). Ignored
pub const OPT_M: C2RustUnnamed_0 = 16384;
// -M\"program arg1 arg2 ...\"; deliver by program. Treated like -F
pub const OPT_F: C2RustUnnamed_0 = 32768;
// -s skip authorization
pub const OPT_T: C2RustUnnamed_0 = 128;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
  pub ctx: md5_ctx_t,
  pub hex: [libc::c_char; 33],
}
// -c use tcpclient. Ignored
pub const OPT_a: C2RustUnnamed_0 = 32;
// -a use APOP protocol
pub const OPT_s: C2RustUnnamed_0 = 64;
pub type C2RustUnnamed_0 = libc::c_uint;
// -F\"program arg1 arg2 ...\"; filter by program. Treated like -M
// -L50000 not retrieve new messages >= 50000 bytes. Ignored
pub const OPT_H: C2RustUnnamed_0 = 8192;
// -Z11-23 remove messages from 11 to 23 (dangerous). Ignored
pub const OPT_L: C2RustUnnamed_0 = 4096;
// -R20000 remove old messages on the server >= 20000 bytes (requires -k). Ignored
pub const OPT_Z: C2RustUnnamed_0 = 2048;
// -t90 set timeout to 90 sec
pub const OPT_R: C2RustUnnamed_0 = 1024;
// -k keep retrieved messages on the server
pub const OPT_t: C2RustUnnamed_0 = 512;
// -V version. Ignored
pub const OPT_c: C2RustUnnamed_0 = 16;
// -m show used memory. Ignored
pub const OPT_V: C2RustUnnamed_0 = 8;
// -d,-dd,-ddd debug. Ignored
pub const OPT_m: C2RustUnnamed_0 = 4;
// -b binary mode. Ignored
pub const OPT_d: C2RustUnnamed_0 = 2;
pub const OPT_b: C2RustUnnamed_0 = 1;
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}

/*
 * popmaildir: a simple yet powerful POP3 client
 * Delivers contents of remote mailboxes to local Maildir
 *
 * Inspired by original utility by Nikola Vladov
 *
 * Copyright (C) 2008 by Vladimir Dronnikov <dronnikov@gmail.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config POPMAILDIR
//config:	bool "popmaildir (10 kb)"
//config:	default y
//config:	help
//config:	Simple yet powerful POP3 mail popper. Delivers content
//config:	of remote mailboxes to local Maildir.
//config:
//config:config FEATURE_POPMAILDIR_DELIVERY
//config:	bool "Allow message filters and custom delivery program"
//config:	default y
//config:	depends on POPMAILDIR
//config:	help
//config:	Allow to use a custom program to filter the content
//config:	of the message before actual delivery (-F "prog [args...]").
//config:	Allow to use a custom program for message actual delivery
//config:	(-M "prog [args...]").
//applet:IF_POPMAILDIR(APPLET(popmaildir, BB_DIR_USR_SBIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_POPMAILDIR) += popmaildir.o mail.o
//usage:#define popmaildir_trivial_usage
//usage:       "[OPTIONS] MAILDIR [CONN_HELPER ARGS]"
//usage:#define popmaildir_full_usage "\n\n"
//usage:       "Fetch content of remote mailbox to local maildir\n"
/* //usage:  "\n	-b		Binary mode. Ignored" */
/* //usage:  "\n	-d		Debug. Ignored" */
/* //usage:  "\n	-m		Show used memory. Ignored" */
/* //usage:  "\n	-V		Show version. Ignored" */
/* //usage:  "\n	-c		Use tcpclient. Ignored" */
/* //usage:  "\n	-a		Use APOP protocol. Implied. If server supports APOP -> use it" */
//usage:     "\n	-s		Skip authorization"
//usage:     "\n	-T		Get messages with TOP instead of RETR"
//usage:     "\n	-k		Keep retrieved messages on the server"
//usage:     "\n	-t SEC		Network timeout"
//usage:	IF_FEATURE_POPMAILDIR_DELIVERY(
//usage:     "\n	-F 'PROG ARGS'	Filter program (may be repeated)"
//usage:     "\n	-M 'PROG ARGS'	Delivery program"
//usage:	)
//usage:     "\n"
//usage:     "\nFetch from plain POP3 server:"
//usage:     "\npopmaildir -k DIR nc pop3.server.com 110 <user_and_pass.txt"
//usage:     "\nFetch from SSLed POP3 server and delete fetched emails:"
//usage:     "\npopmaildir DIR -- openssl s_client -quiet -connect pop3.server.com:995 <user_and_pass.txt"
/* //usage:  "\n	-R BYTES	Remove old messages on the server >= BYTES. Ignored" */
/* //usage:  "\n	-Z N1-N2	Remove messages from N1 to N2 (dangerous). Ignored" */
/* //usage:  "\n	-L BYTES	Don't retrieve new messages >= BYTES. Ignored" */
/* //usage:  "\n	-H LINES	Type first LINES of a message. Ignored" */
//usage:
//usage:#define popmaildir_example_usage
//usage:       "$ popmaildir -k ~/Maildir -- nc pop.drvv.ru 110 [<password_file]\n"
//usage:       "$ popmaildir ~/Maildir -- openssl s_client -quiet -connect pop.gmail.com:995 [<password_file]\n"
unsafe extern "C" fn pop3_checkr(
  mut fmt: *const libc::c_char,
  mut param: *const libc::c_char,
  mut ret: *mut *mut libc::c_char,
) {
  let mut msg: *mut libc::c_char = send_mail_command(fmt, param);
  let mut answer: *mut libc::c_char = xmalloc_fgetline(stdin);
  if !answer.is_null() && '+' as i32 == *answer.offset(0) as libc::c_int {
    free(msg as *mut libc::c_void);
    if (*ptr_to_globals).timeout != 0 {
      alarm(0i32 as libc::c_uint);
    }
    if !ret.is_null() {
      // skip "+OK "
      memmove(
        answer as *mut libc::c_void,
        answer.offset(4) as *const libc::c_void,
        strlen(answer).wrapping_sub(4i32 as libc::c_ulong),
      );
      *ret = answer
    } else {
      free(answer as *mut libc::c_void);
    }
    return;
  }
  bb_error_msg_and_die(
    b"%s failed, reply was: %s\x00" as *const u8 as *const libc::c_char,
    msg,
    answer,
  );
}
unsafe extern "C" fn pop3_check(mut fmt: *const libc::c_char, mut param: *const libc::c_char) {
  pop3_checkr(fmt, param, 0 as *mut *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn popmaildir_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut nmsg: libc::c_uint = 0;
  let mut hostname: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut pid: pid_t = 0;
  let mut retr: *const libc::c_char = 0 as *const libc::c_char;
  let mut delivery: *const libc::c_char = 0 as *const libc::c_char;
  let mut opt_nlines: libc::c_uint = 0i32 as libc::c_uint;
  // init global variables
  let ref mut fresh0 = *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
    as *mut *mut globals);
  *fresh0 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  (*ptr_to_globals).opt_charset =
    b"us-ascii\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
  // parse options
  (*ptr_to_globals).opts = getopt32(
    argv,
    b"^bdmVcasTkt:+R:+Z:L:+H:+M:F:\x00-1:dd\x00" as *const u8 as *const libc::c_char,
    &mut (*ptr_to_globals).timeout as *mut libc::c_uint,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
    &mut opt_nlines as *mut libc::c_uint,
    &mut delivery as *mut *const libc::c_char,
    &mut delivery as *mut *const libc::c_char,
  );
  //argc -= optind;
  argv = argv.offset(optind as isize);
  // get auth info
  if (*ptr_to_globals).opts & OPT_s as libc::c_int as libc::c_uint == 0 {
    get_cred_or_die(0i32);
  }
  // goto maildir
  let fresh1 = argv;
  argv = argv.offset(1);
  xchdir(*fresh1);
  // launch connect helper, if any
  if !(*argv).is_null() {
    launch_helper(argv as *mut *const libc::c_char);
  }
  // get server greeting
  pop3_checkr(0 as *const libc::c_char, 0 as *const libc::c_char, &mut buf);
  // authenticate (if no -s given)
  if (*ptr_to_globals).opts & OPT_s as libc::c_int as libc::c_uint == 0 {
    // server supports APOP and we want it?
    if '<' as i32 == *buf.offset(0) as libc::c_int
      && (*ptr_to_globals).opts & OPT_a as libc::c_int as libc::c_uint != 0
    {
      let mut md5: C2RustUnnamed = C2RustUnnamed {
        ctx: md5_ctx_t {
          wbuffer: [0; 64],
          process_block: None,
          total64: 0,
          hash: [0; 8],
        },
      };
      let mut res: [uint32_t; 4] = [0; 4];
      let mut s: *mut libc::c_char = strchr(buf, '>' as i32);
      if !s.is_null() {
        *s.offset(1) = '\u{0}' as i32 as libc::c_char
      }
      // server ignores APOP -> use simple text authentication
      md5_begin(&mut md5.ctx);
      md5_hash(&mut md5.ctx, buf as *const libc::c_void, strlen(buf));
      md5_hash(
        &mut md5.ctx,
        (*ptr_to_globals).pass as *const libc::c_void,
        strlen((*ptr_to_globals).pass),
      );
      md5_end(&mut md5.ctx, res.as_mut_ptr() as *mut libc::c_void);
      *bin2hex(
        md5.hex.as_mut_ptr(),
        res.as_mut_ptr() as *mut libc::c_char,
        16i32,
      ) = '\u{0}' as i32 as libc::c_char;
      s = xasprintf(
        b"%s %s\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).user,
        md5.hex.as_mut_ptr(),
      );
      pop3_check(b"APOP %s\x00" as *const u8 as *const libc::c_char, s);
      free(s as *mut libc::c_void);
      free(buf as *mut libc::c_void);
    } else {
      // get md5 sum of "<stamp>password" string
      // APOP
      // USER
      pop3_check(
        b"USER %s\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).user,
      );
      // PASS
      pop3_check(
        b"PASS %s\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).pass,
      );
    }
  }
  // get mailbox statistics
  pop3_checkr(
    b"STAT\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    &mut buf,
  );
  // prepare message filename suffix
  hostname = safe_gethostname();
  pid = getpid();
  // get messages counter
  // NOTE: we don't use xatou(buf) since buf is "nmsg nbytes"
  // we only need nmsg and atoi is just exactly what we need
  // if atoi fails to convert buf into number it returns 0
  // in this case the following loop simply will not be executed
  nmsg = atoi(buf) as libc::c_uint;
  free(buf as *mut libc::c_void);
  // loop through messages
  retr = if (*ptr_to_globals).opts & OPT_T as libc::c_int as libc::c_uint != 0 {
    xasprintf(
      b"TOP %%u %u\x00" as *const u8 as *const libc::c_char,
      opt_nlines,
    )
  } else {
    b"RETR %u\x00" as *const u8 as *const libc::c_char
  };
  while nmsg != 0 {
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut target: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut answer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut rc: libc::c_int = 0;
    // generate unique filename
    filename = xasprintf(
      b"tmp/%llu.%u.%s\x00" as *const u8 as *const libc::c_char,
      monotonic_us(),
      pid as libc::c_uint,
      hostname,
    );
    // retrieve message in ./tmp/ unless filter is specified
    pop3_check(retr, nmsg as ptrdiff_t as *const libc::c_char);
    // delivery helper ordered? -> setup pipe
    if (*ptr_to_globals).opts & (OPT_F as libc::c_int | OPT_M as libc::c_int) as libc::c_uint != 0 {
      // helper will have $FILENAME set to filename
      xsetenv(
        b"FILENAME\x00" as *const u8 as *const libc::c_char,
        filename,
      );
      fp = popen(delivery, b"w\x00" as *const u8 as *const libc::c_char);
      unsetenv(b"FILENAME\x00" as *const u8 as *const libc::c_char);
      if fp.is_null() {
        bb_simple_perror_msg(b"delivery helper\x00" as *const u8 as *const libc::c_char);
        break;
      }
    } else {
      // create and open file filename
      fp = xfopen_for_write(filename)
    }
    loop
    // copy stdin to fp (either filename or delivery helper)
    {
      answer = xmalloc_fgets_str(stdin, b"\r\n\x00" as *const u8 as *const libc::c_char);
      if answer.is_null() {
        break;
      }
      let mut s_0: *mut libc::c_char = answer;
      if '.' as i32 == *answer.offset(0) as libc::c_int {
        if '.' as i32 == *answer.offset(1) as libc::c_int {
          s_0 = s_0.offset(1)
        } else if '\r' as i32 == *answer.offset(1) as libc::c_int
          && '\n' as i32 == *answer.offset(2) as libc::c_int
          && '\u{0}' as i32 == *answer.offset(3) as libc::c_int
        {
          break;
        }
      }
      //*strchrnul(s, '\r') = '\n';
      fputs_unlocked(s_0, fp);
      free(answer as *mut libc::c_void);
    }
    // analyse delivery status
    if (*ptr_to_globals).opts & (OPT_F as libc::c_int | OPT_M as libc::c_int) as libc::c_uint != 0 {
      rc = pclose(fp);
      if 99i32 == rc {
        break;
      }
    //			// 0 means continue
    } else {
      // close filename
      fclose(fp);
      // delete message from server
      if (*ptr_to_globals).opts & OPT_k as libc::c_int as libc::c_uint == 0 {
        pop3_check(
          b"DELE %u\x00" as *const u8 as *const libc::c_char,
          nmsg as ptrdiff_t as *const libc::c_char,
        );
      }
      // atomically move message to ./new/
      target = xstrdup(filename);
      memcpy(
        target as *mut libc::c_void,
        b"new\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        3i32 as libc::c_ulong,
      );
      // ... or just stop receiving on failure
      if rename_or_warn(filename, target) != 0 {
        break;
      }
      free(target as *mut libc::c_void);
    }
    //			if (rc) // !0 means skip to the next message
    free(filename as *mut libc::c_void);
    nmsg = nmsg.wrapping_sub(1)
  }
  // Bye
  pop3_check(
    b"QUIT\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
  );
  return 0i32;
}
