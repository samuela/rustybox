use libc;
extern "C" {
  #[no_mangle]
  fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn getuid() -> __uid_t;
  #[no_mangle]
  static ptr_to_globals: *mut globals;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut stdin: *mut FILE;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn trim(s: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xdup2(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn create_and_connect_stream_or_die(peer: *const libc::c_char, port: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xgethostbyname(name: *const libc::c_char) -> *mut hostent;
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc_fgetline(file: *mut FILE) -> *mut libc::c_char;
  #[no_mangle]
  fn xfdopen_for_read(fd: libc::c_int) -> *mut FILE;
  #[no_mangle]
  fn safe_gethostname() -> *mut libc::c_char;
  #[no_mangle]
  fn xuid2uname(uid: uid_t) -> *mut libc::c_char;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn llist_add_to_end(list_head: *mut *mut llist_t, data: *mut libc::c_void);
  #[no_mangle]
  fn llist_pop(elm: *mut *mut llist_t) -> *mut libc::c_void;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  //char FAST_FUNC *parse_url(char *url, char **user, char **pass);
  #[no_mangle]
  fn launch_helper(argv: *mut *const libc::c_char);
  #[no_mangle]
  fn get_cred_or_die(fd: libc::c_int);
  #[no_mangle]
  fn send_mail_command(fmt: *const libc::c_char, param: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn printbuf_base64(buf: *const libc::c_char, len: libc::c_uint);
  #[no_mangle]
  fn printstr_base64(buf: *const libc::c_char);
}

use crate::librb::__uid_t;

use crate::librb::pid_t;
use crate::librb::size_t;
use crate::librb::uid_t;
use crate::librb::uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostent {
  pub h_name: *mut libc::c_char,
  pub h_aliases: *mut *mut libc::c_char,
  pub h_addrtype: libc::c_int,
  pub h_length: libc::c_int,
  pub h_addr_list: *mut *mut libc::c_char,
}

use crate::libbb::llist::llist_t;

use crate::librb::FILE;
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
pub type C2RustUnnamed = libc::c_uint;
pub const HDR_BCC: C2RustUnnamed = 2;
pub const HDR_TOCC: C2RustUnnamed = 1;
pub const HDR_OTHER: C2RustUnnamed = 0;
//--- standard options
pub const OPT_t: C2RustUnnamed_0 = 1;
// verbosity
//--- for -amMETHOD
pub const OPT_am_plain: C2RustUnnamed_0 = 512;
// specify connection string
pub const OPT_a: C2RustUnnamed_0 = 128;
// network timeout
pub const OPT_H: C2RustUnnamed_0 = 32;
pub type C2RustUnnamed_0 = libc::c_uint;
// AUTH PLAIN
// authentication tokens
pub const OPT_v: C2RustUnnamed_0 = 256;
// use external connection helper
pub const OPT_S: C2RustUnnamed_0 = 64;
// IMPLIED!
//--- BB specific options
pub const OPT_w: C2RustUnnamed_0 = 16;
// various options. -oi IMPLIED! others are IGNORED!
pub const OPT_i: C2RustUnnamed_0 = 8;
// sender address
pub const OPT_o: C2RustUnnamed_0 = 4;
// read message for recipients, append them to those on cmdline
pub const OPT_f: C2RustUnnamed_0 = 2;
#[inline(always)]
unsafe extern "C" fn bb_ascii_isalnum(mut a: libc::c_uchar) -> libc::c_int {
  let mut b: libc::c_uchar = (a as libc::c_int - '0' as i32) as libc::c_uchar;
  if b as libc::c_int <= 9i32 {
    return (b as libc::c_int <= 9i32) as libc::c_int;
  }
  b = ((a as libc::c_int | 0x20i32) - 'a' as i32) as libc::c_uchar;
  return (b as libc::c_int <= 'z' as i32 - 'a' as i32) as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
unsafe extern "C" fn send_r_n(mut s: *const libc::c_char) {
  if (*ptr_to_globals).verbose != 0 {
    bb_error_msg(b"send:\'%s\'\x00" as *const u8 as *const libc::c_char, s);
  }
  printf(b"%s\r\n\x00" as *const u8 as *const libc::c_char, s);
}
unsafe extern "C" fn smtp_checkp(
  mut fmt: *const libc::c_char,
  mut param: *const libc::c_char,
  mut code: libc::c_int,
) -> libc::c_int {
  let mut answer: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut msg: *mut libc::c_char = send_mail_command(fmt, param);
  loop
  // read stdin
  // if the string has a form NNN- -- read next string. E.g. EHLO response
  // parse first bytes to a number
  // if code = -1 then just return this number
  // if code != -1 then checks whether the number equals the code
  // if not equal -> die saying msg
  {
    answer = xmalloc_fgetline(stdin);
    if answer.is_null() {
      break;
    }
    if (*ptr_to_globals).verbose != 0 {
      bb_error_msg(
        b"recv:\'%.*s\'\x00" as *const u8 as *const libc::c_char,
        strchrnul(answer, '\r' as i32).wrapping_offset_from(answer) as libc::c_long as libc::c_int,
        answer,
      );
    }
    if strlen(answer) <= 3i32 as libc::c_ulong || '-' as i32 != *answer.offset(3) as libc::c_int {
      break;
    }
    free(answer as *mut libc::c_void);
  }
  if !answer.is_null() {
    let mut n: libc::c_int = atoi(answer);
    if (*ptr_to_globals).timeout != 0 {
      alarm(0i32 as libc::c_uint);
    }
    free(answer as *mut libc::c_void);
    if -1i32 == code || n == code {
      free(msg as *mut libc::c_void);
      return n;
    }
  }
  bb_error_msg_and_die(b"%s failed\x00" as *const u8 as *const libc::c_char, msg);
}
unsafe extern "C" fn smtp_check(
  mut fmt: *const libc::c_char,
  mut code: libc::c_int,
) -> libc::c_int {
  return smtp_checkp(fmt, 0 as *const libc::c_char, code);
}
// strip argument of bad chars
unsafe extern "C" fn sane_address(mut str: *mut libc::c_char) -> *mut libc::c_char {
  let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
  trim(str);
  s = str;
  while *s != 0 {
    /* Standard allows these chars in username without quoting:
     * /!#$%&'*+-=?^_`{|}~
     * and allows dot (.) with some restrictions.
     * I chose to only allow a saner subset.
     * I propose to expand it only on user's request.
     */
    if bb_ascii_isalnum(*s as libc::c_uchar) == 0
      && strchr(
        b"=+_-.@\x00" as *const u8 as *const libc::c_char,
        *s as libc::c_int,
      )
      .is_null()
    {
      bb_error_msg(
        b"bad address \'%s\'\x00" as *const u8 as *const libc::c_char,
        str,
      );
      /* returning "": */
      *str.offset(0) = '\u{0}' as i32 as libc::c_char;
      return str;
    }
    s = s.offset(1)
  }
  return str;
}
// check for an address inside angle brackets, if not found fall back to normal
unsafe extern "C" fn angle_address(mut str: *mut libc::c_char) -> *mut libc::c_char {
  let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
  e = trim(str);
  if e != str && {
    e = e.offset(-1);
    (*e as libc::c_int) == '>' as i32
  } {
    s = strrchr(str, '<' as i32);
    if !s.is_null() {
      *e = '\u{0}' as i32 as libc::c_char;
      str = s.offset(1)
    }
  }
  return sane_address(str);
}
unsafe extern "C" fn rcptto(mut s: *const libc::c_char) {
  if *s == 0 {
    return;
  }
  // N.B. we don't die if recipient is rejected, for the other recipients may be accepted
  if 250i32
    != smtp_checkp(
      b"RCPT TO:<%s>\x00" as *const u8 as *const libc::c_char,
      s,
      -1i32,
    )
  {
    bb_error_msg(
      b"Bad recipient: <%s>\x00" as *const u8 as *const libc::c_char,
      s,
    );
  };
}
// send to a list of comma separated addresses
unsafe extern "C" fn rcptto_list(mut list: *const libc::c_char) {
  let mut free_me: *mut libc::c_char = xstrdup(list);
  let mut str: *mut libc::c_char = free_me;
  let mut s: *mut libc::c_char = free_me;
  let mut prev: libc::c_char = 0i32 as libc::c_char;
  let mut in_quote: libc::c_int = 0i32;
  while *s != 0 {
    let fresh0 = s;
    s = s.offset(1);
    let mut ch: libc::c_char = *fresh0;
    if ch as libc::c_int == '\"' as i32 && prev as libc::c_int != '\\' as i32 {
      in_quote = (in_quote == 0) as libc::c_int
    } else if in_quote == 0 && ch as libc::c_int == ',' as i32 {
      *s.offset(-1i32 as isize) = '\u{0}' as i32 as libc::c_char;
      rcptto(angle_address(str));
      str = s
    }
    prev = ch
  }
  if prev as libc::c_int != ',' as i32 {
    rcptto(angle_address(str));
  }
  free(free_me as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn sendmail_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut opt_connect: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut opt_from: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut list: *mut llist_t = 0 as *mut llist_t;
  let mut host: *mut libc::c_char = sane_address(safe_gethostname());
  let mut nheaders: libc::c_uint = 0i32 as libc::c_uint;
  let mut code: libc::c_int = 0;
  let mut last_hdr: C2RustUnnamed = HDR_OTHER;
  let mut check_hdr: libc::c_int = 0;
  let mut has_to: libc::c_int = 0i32;
  // init global variables
  let ref mut fresh1 = *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
    as *mut *mut globals);
  *fresh1 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  (*ptr_to_globals).opt_charset =
    b"us-ascii\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
  // default HOST[:PORT] is $SMTPHOST, or localhost
  opt_connect = getenv(b"SMTPHOST\x00" as *const u8 as *const libc::c_char);
  if opt_connect.is_null() {
    opt_connect = b"127.0.0.1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
  }
  // save initial stdin since body is piped!
  xdup2(0i32, 3i32);
  (*ptr_to_globals).fp0 = xfdopen_for_read(3i32);
  // parse options
  // N.B. since -H and -S are mutually exclusive they do not interfere in opt_connect
  // -a is for ssmtp (http://downloads.openwrt.org/people/nico/man/man8/ssmtp.8.html) compatibility,
  // it is still under development.
  (*ptr_to_globals).opts = getopt32(
    argv,
    b"^tf:o:iw:+H:S:a:*:v\x00vv:H--S:S--H\x00" as *const u8 as *const libc::c_char,
    &mut opt_from as *mut *mut libc::c_char,
    0 as *mut libc::c_void,
    &mut (*ptr_to_globals).timeout as *mut libc::c_uint,
    &mut opt_connect as *mut *mut libc::c_char,
    &mut opt_connect as *mut *mut libc::c_char,
    &mut list as *mut *mut llist_t,
    &mut (*ptr_to_globals).verbose as *mut libc::c_uint,
  );
  //argc -= optind;
  argv = argv.offset(optind as isize);
  // process -a[upm]<token> options
  if (*ptr_to_globals).opts & OPT_a as libc::c_int as libc::c_uint != 0 && list.is_null() {
    bb_show_usage();
  }
  while !list.is_null() {
    let mut a: *mut libc::c_char = llist_pop(&mut list) as *mut libc::c_char;
    if 'u' as i32 == *a.offset(0) as libc::c_int {
      (*ptr_to_globals).user = xstrdup(a.offset(1))
    }
    if 'p' as i32 == *a.offset(0) as libc::c_int {
      (*ptr_to_globals).pass = xstrdup(a.offset(1))
    }
    if 'm' as i32 == *a.offset(0) as libc::c_int {
      if *a.offset(1) as libc::c_int | 0x20i32 == 'p' as i32 {
        // PLAIN
        (*ptr_to_globals).opts |= OPT_am_plain as libc::c_int as libc::c_uint
      } else if *a.offset(1) as libc::c_int | 0x20i32 == 'l' as i32 {
      } else {
        bb_error_msg_and_die(
          b"unsupported AUTH method %s\x00" as *const u8 as *const libc::c_char,
          a.offset(1),
        );
      }
    }
  }
  // N.B. list == NULL here
  //bb_error_msg("OPT[%x] AU[%s], AP[%s], AM[%s], ARGV[%s]", opts, au, ap, am, *argv);
  // connect to server
  // connection helper ordered? ->
  if (*ptr_to_globals).opts & OPT_H as libc::c_int as libc::c_uint != 0 {
    let mut delay: *const libc::c_char = 0 as *const libc::c_char;
    let mut args: [*const libc::c_char; 4] = [
      b"sh\x00" as *const u8 as *const libc::c_char,
      b"-c\x00" as *const u8 as *const libc::c_char,
      opt_connect as *const libc::c_char,
      0 as *const libc::c_char,
    ];
    // plug it in
    launch_helper(args.as_mut_ptr());
    // Now:
    // our stdout will go to helper's stdin,
    // helper's stdout will be available on our stdin.
    // Wait for initial server message.
    // If helper (such as openssl) invokes STARTTLS, the initial 220
    // is swallowed by helper (and not repeated after TLS is initiated).
    // We will send NOOP cmd to server and check the response.
    // We should get 220+250 on plain connection, 250 on STARTTLSed session.
    //
    // The problem here is some servers delay initial 220 message,
    // and consider client to be a spammer if it starts sending cmds
    // before 220 reached it. The code below is unsafe in this regard:
    // in non-STARTTLSed case, we potentially send NOOP before 220
    // is sent by server.
    //
    // If $SMTP_ANTISPAM_DELAY is set, we pause before sending NOOP.
    //
    delay = getenv(b"SMTP_ANTISPAM_DELAY\x00" as *const u8 as *const libc::c_char);
    if !delay.is_null() {
      sleep(atoi(delay) as libc::c_uint);
    }
    code = smtp_check(b"NOOP\x00" as *const u8 as *const libc::c_char, -1i32);
    if code == 220i32 {
      // we got 220 - this is not STARTTLSed connection,
      // eat 250 response to our NOOP
      smtp_check(0 as *const libc::c_char, 250i32);
    } else if code != 250i32 {
      bb_simple_error_msg_and_die(b"SMTP init failed\x00" as *const u8 as *const libc::c_char);
    }
  } else {
    // vanilla connection
    let mut fd: libc::c_int = 0;
    fd = create_and_connect_stream_or_die(opt_connect, 25i32);
    // and make ourselves a simple IO filter
    xmove_fd(fd, 0i32);
    xdup2(0i32, 1i32);
    // Wait for initial server 220 message
    smtp_check(0 as *const libc::c_char, 220i32);
  }
  // we should start with modern EHLO
  if 250i32
    != smtp_checkp(
      b"EHLO %s\x00" as *const u8 as *const libc::c_char,
      host,
      -1i32,
    )
  {
    smtp_checkp(
      b"HELO %s\x00" as *const u8 as *const libc::c_char,
      host,
      250i32,
    );
  }
  // perform authentication
  if (*ptr_to_globals).opts & OPT_a as libc::c_int as libc::c_uint != 0 {
    // read credentials unless they are given via -a[up] options
    if (*ptr_to_globals).user.is_null() || (*ptr_to_globals).pass.is_null() {
      get_cred_or_die(4i32);
    }
    if (*ptr_to_globals).opts & OPT_am_plain as libc::c_int as libc::c_uint != 0 {
      // C: AUTH PLAIN
      // S: 334
      // C: base64encoded(auth<NUL>user<NUL>pass)
      // S: 235 2.7.0 Authentication successful
      //Note: a shorter format is allowed:
      // C: AUTH PLAIN base64encoded(auth<NUL>user<NUL>pass)
      // S: 235 2.7.0 Authentication successful
      smtp_check(
        b"AUTH PLAIN\x00" as *const u8 as *const libc::c_char,
        334i32,
      );
      let mut user_len: libc::c_uint = strlen((*ptr_to_globals).user) as libc::c_uint;
      let mut pass_len: libc::c_uint = strlen((*ptr_to_globals).pass) as libc::c_uint;
      let mut sz: libc::c_uint = (1i32 as libc::c_uint)
        .wrapping_add(user_len)
        .wrapping_add(1i32 as libc::c_uint)
        .wrapping_add(pass_len);
      let vla = sz.wrapping_add(1i32 as libc::c_uint) as usize;
      let mut plain_auth: Vec<libc::c_char> = ::std::vec::from_elem(0, vla);
      // the format is:
      // "authorization identity<NUL>username<NUL>password"
      // authorization identity is empty.
      *plain_auth.as_mut_ptr().offset(0) = '\u{0}' as i32 as libc::c_char;
      strcpy(
        stpcpy(plain_auth.as_mut_ptr().offset(1), (*ptr_to_globals).user).offset(1),
        (*ptr_to_globals).pass,
      );
      printbuf_base64(plain_auth.as_mut_ptr(), sz);
    } else {
      // C: AUTH LOGIN
      // S: 334 VXNlcm5hbWU6
      //        ^^^^^^^^^^^^ server says "Username:"
      // C: base64encoded(user)
      // S: 334 UGFzc3dvcmQ6
      //        ^^^^^^^^^^^^ server says "Password:"
      // C: base64encoded(pass)
      // S: 235 2.7.0 Authentication successful
      smtp_check(
        b"AUTH LOGIN\x00" as *const u8 as *const libc::c_char,
        334i32,
      );
      printstr_base64((*ptr_to_globals).user);
      smtp_check(b"\x00" as *const u8 as *const libc::c_char, 334i32);
      printstr_base64((*ptr_to_globals).pass);
    }
    smtp_check(b"\x00" as *const u8 as *const libc::c_char, 235i32);
  }
  // set sender
  // N.B. we have here a very loosely defined algorythm
  // since sendmail historically offers no means to specify secrets on cmdline.
  // 1) server can require no authentication ->
  //	we must just provide a (possibly fake) reply address.
  // 2) server can require AUTH ->
  //	we must provide valid username and password along with a (possibly fake) reply address.
  //	For the sake of security username and password are to be read either from console or from a secured file.
  //	Since reading from console may defeat usability, the solution is either to read from a predefined
  //	file descriptor (e.g. 4), or again from a secured file.
  // got no sender address? use auth name, then UID username as a last resort
  if opt_from.is_null() {
    opt_from = xasprintf(
      b"%s@%s\x00" as *const u8 as *const libc::c_char,
      if !(*ptr_to_globals).user.is_null() {
        (*ptr_to_globals).user
      } else {
        xuid2uname(getuid())
      },
      (*xgethostbyname(host)).h_name,
    )
  }
  free(host as *mut libc::c_void);
  smtp_checkp(
    b"MAIL FROM:<%s>\x00" as *const u8 as *const libc::c_char,
    opt_from,
    250i32,
  );
  // process message
  // read recipients from message and add them to those given on cmdline.
  // this means we scan stdin for To:, Cc:, Bcc: lines until an empty line
  // and then use the rest of stdin as message body
  code = 0i32; // set "analyze headers" mode
  's_369: loop {
    s = xmalloc_fgetline((*ptr_to_globals).fp0);
    if !s.is_null() {
      current_block = 16252544171633782868;
    } else {
      current_block = 228501038991332163;
    }
    loop {
      match current_block {
        228501038991332163 =>
        // odd case: we didn't stop "analyze headers" mode -> message body is empty. Reenter the loop
        // N.B. after reenter code will be > 0
        {
          if !(code == 0) {
            // finalize the message
            smtp_check(b".\x00" as *const u8 as *const libc::c_char, 250i32);
            break 's_369;
          }
        }
        _ =>
        // put message lines doubling leading dots
        {
          if code != 0 {
            // escape leading dots
            // N.B. this feature is implied even if no -i (-oi) switch given
            // N.B. we need to escape the leading dot regardless of
            // whether it is single or not character on the line
            if '.' as i32 == *s.offset(0) as libc::c_int {
              /*&& '\0' == s[1] */
              bb_putchar('.' as i32);
            }
            // dump read line
            send_r_n(s);
            free(s as *mut libc::c_void);
            continue 's_369;
          } else {
            // analyze headers
            // To: or Cc: headers add recipients
            check_hdr = (0i32
              == strncasecmp(
                b"To:\x00" as *const u8 as *const libc::c_char,
                s,
                3i32 as libc::c_ulong,
              )) as libc::c_int;
            has_to |= check_hdr;
            if (*ptr_to_globals).opts & OPT_t as libc::c_int as libc::c_uint != 0 {
              if check_hdr != 0
                || 0i32
                  == strncasecmp(
                    (b"Bcc:\x00" as *const u8 as *const libc::c_char).offset(1),
                    s,
                    3i32 as libc::c_ulong,
                  )
              {
                rcptto_list(s.offset(3));
                last_hdr = HDR_TOCC;
                current_block = 2265380199544777579;
                break;
              } else if 0i32
                == strncasecmp(
                  b"Bcc:\x00" as *const u8 as *const libc::c_char,
                  s,
                  4i32 as libc::c_ulong,
                )
              {
                rcptto_list(s.offset(4));
                free(s as *mut libc::c_void);
                last_hdr = HDR_BCC;
                continue 's_369;
                // Bcc: header adds blind copy (hidden) recipient
                // N.B. Bcc: vanishes from headers!
              }
            }
            check_hdr = (!list.is_null()
              && ({
                let mut bb__isspace: libc::c_uchar =
                  (*s.offset(0) as libc::c_int - 9i32) as libc::c_uchar;
                (bb__isspace as libc::c_int == ' ' as i32 - 9i32
                  || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
              }) != 0) as libc::c_int;
            if !strchr(s, ':' as i32).is_null() || check_hdr != 0 {
              // other headers go verbatim
              // N.B. RFC2822 2.2.3 "Long Header Fields" allows for headers to occupy several lines.
              // Continuation is denoted by prefixing additional lines with whitespace(s).
              // Thanks (stefan.seyfried at googlemail.com) for pointing this out.
              if check_hdr != 0
                && last_hdr as libc::c_uint != HDR_OTHER as libc::c_int as libc::c_uint
              {
                current_block = 8723848109087415604;
                break;
              } else {
                current_block = 7301440000599063274;
                break;
              }
            }
          }
        }
      }
      // a line without ":" (an empty line too, by definition) doesn't look like a valid header
      // so stop "analyze headers" mode
      // put recipients specified on cmdline
      check_hdr = 1i32;
      while !(*argv).is_null() {
        let mut t: *mut libc::c_char = sane_address(*argv);
        rcptto(t);
        //if (MAX_HEADERS && ++nheaders >= MAX_HEADERS)
        //	goto bail;
        if has_to == 0 {
          let mut hdr: *const libc::c_char = 0 as *const libc::c_char;
          if check_hdr != 0 && !(*argv.offset(1)).is_null() {
            hdr = b"To: %s,\x00" as *const u8 as *const libc::c_char
          } else if check_hdr != 0 {
            hdr = b"To: %s\x00" as *const u8 as *const libc::c_char
          } else if !(*argv.offset(1)).is_null() {
            hdr = (b"To: %s,\x00" as *const u8 as *const libc::c_char).offset(3)
          } else {
            hdr = (b"To: %s\x00" as *const u8 as *const libc::c_char).offset(3)
          }
          llist_add_to_end(&mut list, xasprintf(hdr, t) as *mut libc::c_void);
          check_hdr = 0i32
        }
        argv = argv.offset(1)
      }
      // enter "put message" mode
      // N.B. DATA fails iff no recipients were accepted (or even provided)
      // in this case just bail out gracefully
      if 354i32 != smtp_check(b"DATA\x00" as *const u8 as *const libc::c_char, -1i32) {
        break 's_369;
      }
      // dump the headers
      while !list.is_null() {
        send_r_n(llist_pop(&mut list) as *mut libc::c_char);
      }
      // stop analyzing headers
      code += 1;
      // N.B. !s means: we read nothing, and nothing to be read in the future.
      // just dump empty line and break the loop
      if s.is_null() {
        send_r_n(b"\x00" as *const u8 as *const libc::c_char);
        current_block = 228501038991332163;
      } else {
        // go dump message body
        // N.B. "s" already contains the first non-header line, so pretend we read it from input
        current_block = 16252544171633782868;
      }
    }
    match current_block {
      8723848109087415604 => {
        rcptto_list(s.offset(1));
        if last_hdr as libc::c_uint == HDR_BCC as libc::c_int as libc::c_uint {
          continue;
        }
        // N.B. Bcc: vanishes from headers!
      }
      7301440000599063274 => last_hdr = HDR_OTHER,
      _ => {}
    }
    // N.B. we allow MAX_HEADERS generic headers at most to prevent attacks
    if 256i32 != 0 && {
      nheaders = nheaders.wrapping_add(1);
      (nheaders) >= 256i32 as libc::c_uint
    } {
      break;
    }
    llist_add_to_end(&mut list, s as *mut libc::c_void);
  }
  // ... and say goodbye
  smtp_check(b"QUIT\x00" as *const u8 as *const libc::c_char, 221i32);
  // cleanup
  return 0i32;
}
