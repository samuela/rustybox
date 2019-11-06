use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn sethostname(__name: *const libc::c_char, __len: size_t) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn puts(__s: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
  // "old" (ipv4 only) API
  // users: traceroute.c hostname.c - use _list_ of all IPs
  #[no_mangle]
  fn xgethostbyname(name: *const libc::c_char) -> *mut hostent;
  /* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
   * But potentially slow, don't use in one-billion-times loops */
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xfopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn safe_gethostname() -> *mut libc::c_char;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn config_open2(
    filename: *const libc::c_char,
    fopen_func: Option<unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE>,
  ) -> *mut parser_t;
  /* delims[0] is a comment char (use '\0' to disable), the rest are token delimiters */
  #[no_mangle]
  fn config_read(
    parser: *mut parser_t,
    tokens: *mut *mut libc::c_char,
    flags: libc::c_uint,
    delims: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  fn config_close(parser: *mut parser_t);
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
}

use crate::librb::size_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
  pub s_addr: in_addr_t,
}
pub type in_addr_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostent {
  pub h_name: *mut libc::c_char,
  pub h_aliases: *mut *mut libc::c_char,
  pub h_addrtype: libc::c_int,
  pub h_length: libc::c_int,
  pub h_addr_list: *mut *mut libc::c_char,
}

use libc::FILE;
/*
 * Config file parser
 */
pub type C2RustUnnamed = libc::c_uint;
// comments are recognized even if there is whitespace before
// ("line start><space><tab><space>#comment" is also comment, not only "line start>#comment")
// NORMAL is:
// * remove leading and trailing delimiters and collapse
//   multiple delimiters into one
// * warn and continue if less than mintokens delimiters found
// * grab everything into last token
// * comments are recognized even if they aren't the first char
pub const PARSE_NORMAL: C2RustUnnamed = 4653056;
// delim[0] and delim[1] are two different allowed comment chars
// (so far, delim[0] will only work as comment char for full-line comment)
// (IOW: it works as if PARSE_EOL_COMMENTS is not set. sysctl applet is okay with this)
pub const PARSE_WS_COMMENTS: C2RustUnnamed = 16777216;
// comments are recognized even if they aren't the first char
pub const PARSE_ALT_COMMENTS: C2RustUnnamed = 8388608;
pub const PARSE_EOL_COMMENTS: C2RustUnnamed = 4194304;
// die if < min tokens found
// keep a copy of current line
pub const PARSE_KEEP_COPY: C2RustUnnamed = 2097152;
// last token takes entire remainder of the line
pub const PARSE_MIN_DIE: C2RustUnnamed = 1048576;
// trim leading and trailing delimiters
// TODO: COLLAPSE and TRIM seem to always go in pair
pub const PARSE_GREEDY: C2RustUnnamed = 262144;
// treat consecutive delimiters as one
pub const PARSE_TRIM: C2RustUnnamed = 131072;
pub const PARSE_COLLAPSE: C2RustUnnamed = 65536;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parser_t {
  pub fp: *mut FILE,
  pub data: *mut libc::c_char,
  pub line: *mut libc::c_char,
  pub nline: *mut libc::c_char,
  pub line_alloc: size_t,
  pub nline_alloc: size_t,
  pub lineno: libc::c_int,
}
pub const OPT_F: C2RustUnnamed_0 = 16;
pub const OPT_s: C2RustUnnamed_0 = 8;
pub const OPT_d: C2RustUnnamed_0 = 1;
pub const OPT_f: C2RustUnnamed_0 = 2;
pub const OPT_dfi: C2RustUnnamed_0 = 7;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const OPT_i: C2RustUnnamed_0 = 4;

/*
 * Mini hostname implementation for busybox
 *
 * Copyright (C) 1999 by Randolph Chung <tausq@debian.org>
 *
 * Adjusted by Erik Andersen <andersen@codepoet.org> to remove
 * use of long options and GNU getopt.  Improved the usage info.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config HOSTNAME
//config:	bool "hostname (5.5 kb)"
//config:	default y
//config:	help
//config:	Show or set the system's host name.
//config:
//config:config DNSDOMAINNAME
//config:	bool "dnsdomainname (3.6 kb)"
//config:	default y
//config:	help
//config:	Alias to "hostname -d".
//                        APPLET_NOEXEC:name           main      location    suid_type     help
//applet:IF_DNSDOMAINNAME(APPLET_NOEXEC(dnsdomainname, hostname, BB_DIR_BIN, BB_SUID_DROP, dnsdomainname))
//applet:IF_HOSTNAME(     APPLET_NOEXEC(hostname,      hostname, BB_DIR_BIN, BB_SUID_DROP, hostname     ))
//kbuild: lib-$(CONFIG_HOSTNAME) += hostname.o
//kbuild: lib-$(CONFIG_DNSDOMAINNAME) += hostname.o
//usage:#define hostname_trivial_usage
//usage:       "[OPTIONS] [HOSTNAME | -F FILE]"
//usage:#define hostname_full_usage "\n\n"
//usage:       "Get or set hostname or DNS domain name\n"
//usage:     "\n	-s	Short"
//usage:     "\n	-i	Addresses for the hostname"
//usage:     "\n	-d	DNS domain name"
//usage:     "\n	-f	Fully qualified domain name"
//usage:     "\n	-F FILE	Use FILE's content as hostname"
//usage:
//usage:#define hostname_example_usage
//usage:       "$ hostname\n"
//usage:       "sage\n"
//usage:
//usage:#define dnsdomainname_trivial_usage NOUSAGE_STR
//usage:#define dnsdomainname_full_usage ""
unsafe extern "C" fn do_sethostname(mut s: *mut libc::c_char, mut isfile: libc::c_int) {
  //	if (!s)
  //		return;
  if isfile != 0 {
    let mut parser: *mut parser_t = config_open2(
      s,
      Some(xfopen_for_read as unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE),
    );
    while config_read(
      parser,
      &mut s,
      (PARSE_NORMAL as libc::c_int & !(PARSE_GREEDY as libc::c_int)
        | (1i32 & 0xffi32) << 8i32
        | 1i32 & 0xffi32) as libc::c_uint,
      b"# \t\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
      do_sethostname(s, 0i32);
    }
  } else if sethostname(s, strlen(s)) != 0 {
    //		if (errno == EPERM)
    //			bb_error_msg_and_die(bb_msg_perm_denied_are_you_root);
    bb_simple_perror_msg_and_die(b"sethostname\x00" as *const u8 as *const libc::c_char);
  };
}
/* Manpage circa 2009:
 *
 * hostname [-v] [-a] [--alias] [-d] [--domain] [-f] [--fqdn] [--long]
 *      [-i] [--ip-address] [-s] [--short] [-y] [--yp] [--nis]
 *
 * hostname [-v] [-F filename] [--file filename] / [hostname]
 *
 * domainname [-v] [-F filename] [--file filename]  / [name]
 *  { bbox: not supported }
 *
 * nodename [-v] [-F filename] [--file filename] / [name]
 *  { bbox: not supported }
 *
 * dnsdomainname [-v]
 *  { bbox: supported: Linux kernel build needs this }
 * nisdomainname [-v]
 *  { bbox: not supported }
 * ypdomainname [-v]
 *  { bbox: not supported }
 *
 * -a, --alias
 *  Display the alias name of the host (if used).
 *  { bbox: not supported }
 * -d, --domain
 *  Display the name of the DNS domain. Don't use the command
 *  domainname to get the DNS domain name because it will show the
 *  NIS domain name and not the DNS domain name. Use dnsdomainname
 *  instead.
 * -f, --fqdn, --long
 *  Display the FQDN (Fully Qualified Domain Name). A FQDN consists
 *  of a short host name and the DNS domain name. Unless you are
 *  using bind or NIS for host lookups you can change the FQDN and
 *  the DNS domain name (which is part of the FQDN) in the
 *  /etc/hosts file.
 * -i, --ip-address
 *  Display the IP address(es) of the host.
 * -s, --short
 *  Display the short host name. This is the host name cut at the
 *  first dot.
 * -v, --verbose
 *  Be verbose and tell what's going on.
 *  { bbox: supported but ignored }
 * -y, --yp, --nis
 *  Display the NIS domain name. If a parameter is given (or --file
 *  name ) then root can also set a new NIS domain.
 *  { bbox: not supported }
 * -F, --file filename
 *  Read the host name from the specified file. Comments (lines
 *  starting with a '#') are ignored.
 */
#[no_mangle]
pub unsafe extern "C" fn hostname_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opts: libc::c_uint = 0;
  let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut hostname_str: *mut libc::c_char = 0 as *mut libc::c_char;
  /* dnsdomainname from net-tools 1.60, hostname 1.100 (2001-04-14),
   * supports hostname's options too (not just -v as manpage says) */
  opts = getopt32(
    argv,
    b"dfisF:v\x00" as *const u8 as *const libc::c_char,
    &mut hostname_str as *mut *mut libc::c_char,
    b"domain\x00\x00dfqdn\x00\x00ffile\x00\x00F\x00" as *const u8 as *const libc::c_char,
  );
  argv = argv.offset(optind as isize);
  buf = safe_gethostname();
  if 1i32 == 0 || *applet_name.offset(0) as libc::c_int == 'd' as i32 {
    /* dnsdomainname */
    opts = OPT_d as libc::c_int as libc::c_uint
  }
  if opts & OPT_dfi as libc::c_int as libc::c_uint != 0 {
    /* Cases when we need full hostname (or its part) */
    let mut hp: *mut hostent = 0 as *mut hostent;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    hp = xgethostbyname(buf);
    p = strchrnul((*hp).h_name, '.' as i32);
    if opts & OPT_f as libc::c_int as libc::c_uint != 0 {
      puts((*hp).h_name);
    } else if opts & OPT_s as libc::c_int as libc::c_uint != 0 {
      *p = '\u{0}' as i32 as libc::c_char;
      puts((*hp).h_name);
    } else if opts & OPT_d as libc::c_int as libc::c_uint != 0 {
      if *p != 0 {
        puts(p.offset(1));
      }
    } else if (*hp).h_length as libc::c_ulong == ::std::mem::size_of::<in_addr>() as libc::c_ulong {
      let mut h_addr_list: *mut *mut in_addr = (*hp).h_addr_list as *mut *mut in_addr;
      while !(*h_addr_list).is_null() {
        printf(
          if !(*h_addr_list.offset(1)).is_null() {
            b"%s \x00" as *const u8 as *const libc::c_char
          } else {
            b"%s\x00" as *const u8 as *const libc::c_char
          },
          inet_ntoa(**h_addr_list),
        );
        h_addr_list = h_addr_list.offset(1)
      }
      bb_putchar('\n' as i32);
    }
  } else if opts & OPT_s as libc::c_int as libc::c_uint != 0 {
    *strchrnul(buf, '.' as i32).offset(0) = '\u{0}' as i32 as libc::c_char;
    puts(buf);
  } else if opts & OPT_F as libc::c_int as libc::c_uint != 0 {
    /*if (opts & OPT_i)*/
    /* Set the hostname */
    do_sethostname(hostname_str, 1i32);
  } else if !(*argv.offset(0)).is_null() {
    /* Set the hostname */
    do_sethostname(*argv.offset(0), 0i32);
  } else {
    /* Just print the current hostname */
    puts(buf);
  }
  return 0i32;
}
