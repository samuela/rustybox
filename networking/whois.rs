use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn dprintf(__fd: libc::c_int, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn fgets_unlocked(
    __s: *mut libc::c_char,
    __n: libc::c_int,
    __stream: *mut FILE,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  static mut option_mask32: uint32_t;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
  #[no_mangle]
  fn trim(s: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn skip_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn create_and_connect_stream_or_die(peer: *const libc::c_char, port: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xfdopen_for_read(fd: libc::c_int) -> *mut FILE;
  #[no_mangle]
  fn str_tolower(str: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
}



use crate::librb::uint32_t;
use crate::librb::size_t;



use crate::librb::FILE;

/*
 * whois - tiny client for the whois directory service
 *
 * Copyright (c) 2011 Pere Orga <gotrunks@gmail.com>
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* TODO
 * Add ipv6 support
 * Add proxy support
 */
//config:config WHOIS
//config:	bool "whois (6.3 kb)"
//config:	default y
//config:	help
//config:	whois is a client for the whois directory service
//applet:IF_WHOIS(APPLET(whois, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_WHOIS) += whois.o
//usage:#define whois_trivial_usage
//usage:       "[-i] [-h SERVER] [-p PORT] NAME..."
//usage:#define whois_full_usage "\n\n"
//usage:       "Query WHOIS info about NAME\n"
//usage:     "\n	-i	Show redirect results too"
//usage:     "\n	-h,-p	Server to query"
pub type C2RustUnnamed = libc::c_uint;
pub const OPT_i: C2RustUnnamed = 1;
unsafe extern "C" fn query(
  mut host: *const libc::c_char,
  mut port: libc::c_int,
  mut domain: *const libc::c_char,
) -> *mut libc::c_char {
  let mut fd: libc::c_int = 0;
  let mut fp: *mut FILE = 0 as *mut FILE;
  let mut success: bool = false;
  let mut redir: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut pfx: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
  /* some .io domains reported to have very long strings in whois
   * responses, 1k was not enough:
   */
  let mut linebuf: [libc::c_char; 2048] = [0; 2048]; /* closes fd too */
  let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut bufpos: libc::c_uint = 0i32 as libc::c_uint;
  loop {
    printf(
      b"[Querying %s:%d \'%s%s\']\n\x00" as *const u8 as *const libc::c_char,
      host,
      port,
      pfx,
      domain,
    );
    fd = create_and_connect_stream_or_die(host, port);
    dprintf(
      fd,
      b"%s%s\r\n\x00" as *const u8 as *const libc::c_char,
      pfx,
      domain,
    );
    fp = xfdopen_for_read(fd);
    success = 0i32 != 0;
    while !fgets_unlocked(
      linebuf.as_mut_ptr(),
      (::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int,
      fp,
    )
    .is_null()
    {
      let mut len: libc::c_uint = 0;
      len = strcspn(
        linebuf.as_mut_ptr(),
        b"\r\n\x00" as *const u8 as *const libc::c_char,
      ) as libc::c_uint;
      let fresh0 = len;
      len = len.wrapping_add(1);
      linebuf[fresh0 as usize] = '\n' as i32 as libc::c_char;
      linebuf[len as usize] = '\u{0}' as i32 as libc::c_char;
      buf = xrealloc(
        buf as *mut libc::c_void,
        bufpos.wrapping_add(len).wrapping_add(1i32 as libc::c_uint) as size_t,
      ) as *mut libc::c_char;
      memcpy(
        buf.offset(bufpos as isize) as *mut libc::c_void,
        linebuf.as_mut_ptr() as *const libc::c_void,
        len as libc::c_ulong,
      );
      bufpos = bufpos.wrapping_add(len);
      *buf.offset(bufpos as isize) = '\u{0}' as i32 as libc::c_char;
      if redir.is_null() || !success {
        trim(linebuf.as_mut_ptr());
        str_tolower(linebuf.as_mut_ptr());
        if !success {
          success = !is_prefixed_with(
            linebuf.as_mut_ptr(),
            b"domain:\x00" as *const u8 as *const libc::c_char,
          )
          .is_null()
            || !is_prefixed_with(
              linebuf.as_mut_ptr(),
              b"domain name:\x00" as *const u8 as *const libc::c_char,
            )
            .is_null()
        } else if redir.is_null() {
          let mut p: *mut libc::c_char = is_prefixed_with(
            linebuf.as_mut_ptr(),
            b"whois server:\x00" as *const u8 as *const libc::c_char,
          );
          if p.is_null() {
            p = is_prefixed_with(
              linebuf.as_mut_ptr(),
              b"whois:\x00" as *const u8 as *const libc::c_char,
            )
          }
          if !p.is_null() {
            redir = xstrdup(skip_whitespace(p))
          }
        }
      }
    }
    fclose(fp);
    if !(!success && *pfx.offset(0) == 0) {
      break;
    }
    /*
     * Looking at /etc/jwhois.conf, some whois servers use
     * "domain = DOMAIN", "DOMAIN ID <DOMAIN>"
     * and "domain=DOMAIN_WITHOUT_LAST_COMPONENT"
     * formats, but those are rare.
     * (There are a few even more contrived ones.)
     * We are trying only "domain DOMAIN", the typical one.
     */
    pfx = b"domain \x00" as *const u8 as *const libc::c_char;
    bufpos = 0i32 as libc::c_uint
  }
  /* Success */
  if !redir.is_null() && strcmp(redir, host) == 0i32 {
    /* Redirect to self does not count */
    free(redir as *mut libc::c_void);
    redir = 0 as *mut libc::c_char
  }
  if redir.is_null() || option_mask32 & OPT_i as libc::c_int as libc::c_uint != 0 {
    /* Output saved text */
    printf(
      b"[%s]\n%s\x00" as *const u8 as *const libc::c_char,
      host,
      if !buf.is_null() {
        buf
      } else {
        b"\x00" as *const u8 as *const libc::c_char
      },
    );
  }
  free(buf as *mut libc::c_void);
  return redir;
}
unsafe extern "C" fn recursive_query(
  mut host: *const libc::c_char,
  mut port: libc::c_int,
  mut domain: *const libc::c_char,
) {
  let mut free_me: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut redir: *mut libc::c_char = 0 as *mut libc::c_char;
  loop {
    redir = query(host, port, domain);
    free(free_me as *mut libc::c_void);
    if redir.is_null() {
      break;
    }
    printf(
      b"[Redirected to %s]\n\x00" as *const u8 as *const libc::c_char,
      redir,
    );
    free_me = redir;
    host = free_me;
    port = 43i32
  }
}
/* One of "big" whois implementations has these options:
*
* $ whois --help
* jwhois version 4.0, Copyright (C) 1999-2007  Free Software Foundation, Inc.
* -v, --verbose              verbose debug output
* -c FILE, --config=FILE     use FILE as configuration file
* -h HOST, --host=HOST       explicitly query HOST
* -n, --no-redirect          disable content redirection
* -s, --no-whoisservers      disable whois-servers.net service support
* -a, --raw                  disable reformatting of the query
* -i, --display-redirections display all redirects instead of hiding them
* -p PORT, --port=PORT       use port number PORT (in conjunction with HOST)
* -r, --rwhois               force an rwhois query to be made
* --rwhois-display=DISPLAY   sets the display option in rwhois queries
* --rwhois-limit=LIMIT       sets the maximum number of matches to return
*
* Example of its output:
* $ whois cnn.com
* [Querying whois.verisign-grs.com]
* [Redirected to whois.corporatedomains.com]
* [Querying whois.corporatedomains.com]
* [whois.corporatedomains.com]
* ...text of the reply...
*
* With -i, reply from each server is printed, after all redirects are done:
* [Querying whois.verisign-grs.com]
* [Redirected to whois.corporatedomains.com]
* [Querying whois.corporatedomains.com]
* [whois.verisign-grs.com]
* ...text of the reply...
* [whois.corporatedomains.com]
* ...text of the reply...
*
* With -a, no "DOMAIN" -> "domain DOMAIN" transformation is attempted.

* With -n, the first reply is shown, redirects are not followed:
* [Querying whois.verisign-grs.com]
* [whois.verisign-grs.com]
* ...text of the reply...
*/
#[no_mangle]
pub unsafe extern "C" fn whois_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut port: libc::c_int = 43i32;
  let mut host: *const libc::c_char = b"whois.iana.org\x00" as *const u8 as *const libc::c_char;
  getopt32(
    argv,
    b"^ih:p:+\x00-1\x00" as *const u8 as *const libc::c_char,
    &mut host as *mut *const libc::c_char,
    &mut port as *mut libc::c_int,
  );
  argv = argv.offset(optind as isize);
  loop {
    recursive_query(host, port, *argv);
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  return 0i32;
}
