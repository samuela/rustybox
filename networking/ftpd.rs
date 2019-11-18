use crate::libbb::appletlib::applet_name;
use crate::libbb::ptr_to_globals::bb_errno;
use crate::libbb::xfuncs_printf::xmalloc;
use crate::libpwdgrp::pwd_grp::bb_internal_getpwnam;
use crate::librb::size_t;
use crate::librb::smallint;
use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
use libc::alarm;
use libc::chdir;
use libc::close;
use libc::fclose;
use libc::free;
use libc::fstat;
use libc::getpid;
use libc::mode_t;
use libc::off_t;
use libc::open;
use libc::openlog;
use libc::pid_t;
use libc::rename;
use libc::rmdir;
use libc::sprintf;
use libc::stat;
use libc::strchr;
use libc::strcmp;
use libc::strcpy;
use libc::strrchr;
use libc::time_t;
use libc::unlink;
extern "C" {
  pub type sockaddr_x25;
  pub type sockaddr_un;
  pub type sockaddr_ns;
  pub type sockaddr_iso;
  pub type sockaddr_ipx;
  pub type sockaddr_inarp;
  pub type sockaddr_eon;
  pub type sockaddr_dl;
  pub type sockaddr_ax25;
  pub type sockaddr_at;

  #[no_mangle]
  fn exit(_: libc::c_int) -> !;
  #[no_mangle]
  fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn vfork() -> libc::c_int;

  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn chroot(__path: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn getsockname(__fd: libc::c_int, __addr: __SOCKADDR_ARG, __len: *mut socklen_t) -> libc::c_int;
  #[no_mangle]
  fn accept(__fd: libc::c_int, __addr: __SOCKADDR_ARG, __addr_len: *mut socklen_t) -> libc::c_int;
  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
  #[no_mangle]
  static mut stdin: *mut FILE;

  #[no_mangle]
  fn dup(__fd: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  fn mkdir(__path: *const libc::c_char, __mode: mode_t) -> libc::c_int;
  #[no_mangle]
  fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
  /* Search for an entry with a matching username.  */

  /* Some useful definitions */
  /* Macros for min/max.  */
  /* buffer allocation schemes */
  /* glibc uses __errno_location() to get a ptr to errno */
  /* We can just memorize it once - no multithreading in busybox :) */

  #[no_mangle]
  fn monotonic_sec() -> libc::c_uint;

  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  /* bb_copyfd_XX print read/write errors and return -1 if they occur */
  #[no_mangle]
  fn bb_copyfd_eof(fd1: libc::c_int, fd2: libc::c_int) -> off_t;
  #[no_mangle]
  fn ndelay_off(fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  /* !RETURNS_MALLOC: it's a realloc-like function */
  #[no_mangle]
  fn xrealloc_getcwd_or_warn(cwd: *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_signals(sigs: libc::c_int, f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>);
  #[no_mangle]
  fn xchdir(path: *const libc::c_char);
  #[no_mangle]
  fn xlseek(fd: libc::c_int, offset: off_t, whence: libc::c_int) -> off_t;
  #[no_mangle]
  fn xpipe(filedes: *mut libc::c_int);
  #[no_mangle]
  fn xsocket(domain: libc::c_int, type_0: libc::c_int, protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xbind(sockfd: libc::c_int, my_addr: *mut sockaddr, addrlen: socklen_t);
  #[no_mangle]
  fn xlisten(s: libc::c_int, backlog: libc::c_int);
  #[no_mangle]
  fn setsockopt_1(fd: libc::c_int, level: libc::c_int, optname: libc::c_int) -> libc::c_int;
  /* SO_REUSEADDR allows a server to rebind to an address that is already
   * "in use" by old connections to e.g. previous server instance which is
   * killed or crashed. Without it bind will fail until all such connections
   * time out. Linux does not allow multiple live binds on same ip:port
   * regardless of SO_REUSEADDR (unlike some other flavors of Unix).
   * Turn it on before you call bind(). */
  #[no_mangle]
  fn setsockopt_reuseaddr(fd: libc::c_int);
  /* On Linux this never fails. */
  #[no_mangle]
  fn setsockopt_keepalive(fd: libc::c_int) -> libc::c_int;
  /* Connect to peer identified by lsa */
  #[no_mangle]
  fn xconnect_stream(lsa: *const len_and_sockaddr) -> libc::c_int;
  /* Get local address of bound or accepted socket */
  #[no_mangle]
  fn get_sock_lsa(fd: libc::c_int) -> *mut len_and_sockaddr;
  /* Get remote address of connected or accepted socket */
  #[no_mangle]
  fn get_peer_lsa(fd: libc::c_int) -> *mut len_and_sockaddr;
  /* Assign sin[6]_port member if the socket is an AF_INET[6] one,
   * otherwise no-op. Useful for ftp.
   * NB: does NOT do htons() internally, just direct assignment. */
  #[no_mangle]
  fn set_nport(sa: *mut sockaddr, port: libc::c_uint);
  /* Retrieve sin[6]_port or return -1 for non-INET[6] lsa's */
  #[no_mangle]
  fn get_nport(sa: *const sockaddr) -> libc::c_int;
  #[no_mangle]
  fn xmalloc_sockaddr2dotted_noport(sa: *const sockaddr) -> *mut libc::c_char;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn xwrite(fd: libc::c_int, buf: *const libc::c_void, count: size_t);
  #[no_mangle]
  fn xwrite_str(fd: libc::c_int, str: *const libc::c_char);
  /* Same, with limited max size, and returns the length (excluding NUL): */
  #[no_mangle]
  fn xmalloc_fgets_str_len(
    file: *mut FILE,
    terminating_string: *const libc::c_char,
    maxsz_p: *mut size_t,
  ) -> *mut libc::c_char;
  /* Reads up to (and including) "\n" or NUL byte: */
  #[no_mangle]
  fn xmalloc_fgets(file: *mut FILE) -> *mut libc::c_char;
  /* Chops off '\n' from the end, unlike fgets: */
  #[no_mangle]
  fn xmalloc_fgetline(file: *mut FILE) -> *mut libc::c_char;
  #[no_mangle]
  fn xfdopen_for_read(fd: libc::c_int) -> *mut FILE;
  #[no_mangle]
  fn xatoull_range(
    str: *const libc::c_char,
    l: libc::c_ulonglong,
    u: libc::c_ulonglong,
  ) -> libc::c_ulonglong;
  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  /* xvfork() can't be a _function_, return after vfork in child mangles stack
   * in the parent. It must be a macro. */
  #[no_mangle]
  fn xfork() -> pid_t;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  static mut logmode: smallint;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  /* Used by ftpd */
  #[no_mangle]
  fn ls_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn change_identity(pw: *const passwd);
  #[no_mangle]
  fn check_password(pw: *const passwd, plaintext: *const libc::c_char) -> libc::c_int;

  /* '*const' ptr makes gcc optimize code much better.
   * Magic prevents ptr_to_globals from going into rodata.
   * If you want to assign a value, use SET_PTR_TO_GLOBALS(x) */
  #[no_mangle]
  static ptr_to_globals: *mut globals;

}

pub type __socklen_t = libc::c_uint;

pub type bb__aliased_u32 = u32;
pub type socklen_t = __socklen_t;

pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
use libc::sa_family_t;
use libc::sockaddr;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
  pub __sockaddr__: *mut sockaddr,
  pub __sockaddr_at__: *mut sockaddr_at,
  pub __sockaddr_ax25__: *mut sockaddr_ax25,
  pub __sockaddr_dl__: *mut sockaddr_dl,
  pub __sockaddr_eon__: *mut sockaddr_eon,
  pub __sockaddr_in__: *mut sockaddr_in,
  pub __sockaddr_in6__: *mut sockaddr_in6,
  pub __sockaddr_inarp__: *mut sockaddr_inarp,
  pub __sockaddr_ipx__: *mut sockaddr_ipx,
  pub __sockaddr_iso__: *mut sockaddr_iso,
  pub __sockaddr_ns__: *mut sockaddr_ns,
  pub __sockaddr_un__: *mut sockaddr_un,
  pub __sockaddr_x25__: *mut sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
  pub sin6_family: sa_family_t,
  pub sin6_port: in_port_t,
  pub sin6_flowinfo: u32,
  pub sin6_addr: in6_addr,
  pub sin6_scope_id: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
  pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
  pub __u6_addr8: [u8; 16],
  pub __u6_addr16: [u16; 8],
  pub __u6_addr32: [u32; 4],
}
pub type in_port_t = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
  pub sin_family: sa_family_t,
  pub sin_port: in_port_t,
  pub sin_addr: in_addr,
  pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
  pub s_addr: in_addr_t,
}
pub type in_addr_t = u32;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_0 = 256;
pub const IPPROTO_RAW: C2RustUnnamed_0 = 255;
pub const IPPROTO_MPLS: C2RustUnnamed_0 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_0 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_0 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_0 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_0 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_0 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_0 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_0 = 92;
pub const IPPROTO_AH: C2RustUnnamed_0 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_0 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_0 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_0 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_0 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_0 = 33;
pub const IPPROTO_TP: C2RustUnnamed_0 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_0 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_0 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_0 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_0 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_0 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_0 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_0 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_0 = 1;
pub const IPPROTO_IP: C2RustUnnamed_0 = 0;
use crate::librb::signal::__sighandler_t;
use libc::passwd;
use libc::tm;
use libc::FILE;
/* In this form code with pipes is much more readable */
use crate::librb::fd_pair;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct len_and_sockaddr {
  pub len: socklen_t,
  pub u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
  pub sa: sockaddr,
  pub sin: sockaddr_in,
  pub sin6: sockaddr_in6,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const LOGMODE_BOTH: C2RustUnnamed_2 = 3;
pub const LOGMODE_SYSLOG: C2RustUnnamed_2 = 2;
pub const LOGMODE_STDIO: C2RustUnnamed_2 = 1;
pub const LOGMODE_NONE: C2RustUnnamed_2 = 0;
//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub pasv_listen_fd: libc::c_int,
  pub local_file_fd: libc::c_int,
  pub end_time: libc::c_uint,
  pub timeout: libc::c_uint,
  pub verbose: libc::c_uint,
  pub local_file_pos: off_t,
  pub restart_pos: off_t,
  pub local_addr: *mut len_and_sockaddr,
  pub port_addr: *mut len_and_sockaddr,
  pub ftp_cmd: *mut libc::c_char,
  pub ftp_arg: *mut libc::c_char,
  pub rnfr_filename: *mut libc::c_char,
  pub msg_ok: [libc::c_char; 28],
  pub msg_err: [libc::c_char; 12],
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const SHIFTsp: C2RustUnnamed_3 = 24;
pub const SHIFT0: C2RustUnnamed_3 = 16;
pub const SHIFT1: C2RustUnnamed_3 = 8;
pub const SHIFT2: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const LONG_LISTING: C2RustUnnamed_4 = 2;
pub const USE_CTRL_CONN: C2RustUnnamed_4 = 1;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const OPT_w: C2RustUnnamed_5 = 8;
pub const OPT_S: C2RustUnnamed_5 = 4;
pub const OPT_v: C2RustUnnamed_5 = 2;
pub const OPT_A: C2RustUnnamed_5 = 1;
pub const BIT_A: C2RustUnnamed_5 = 0;
pub const const_USER: C2RustUnnamed_5 = 1431520594;
pub const const_TYPE: C2RustUnnamed_5 = 1415139397;
pub const const_SYST: C2RustUnnamed_5 = 1398362964;
pub const const_STRU: C2RustUnnamed_5 = 1398035029;
pub const const_STOU: C2RustUnnamed_5 = 1398034261;
pub const const_STOR: C2RustUnnamed_5 = 1398034258;
pub const const_STAT: C2RustUnnamed_5 = 1398030676;
pub const const_SIZE: C2RustUnnamed_5 = 1397316165;
pub const const_RNTO: C2RustUnnamed_5 = 1380865103;
pub const const_RNFR: C2RustUnnamed_5 = 1380861522;
pub const const_RMD: C2RustUnnamed_5 = 5393732;
pub const const_RETR: C2RustUnnamed_5 = 1380275282;
pub const const_REST: C2RustUnnamed_5 = 1380275028;
pub const const_QUIT: C2RustUnnamed_5 = 1364543828;
/* Same as PWD. Reportedly used by windows ftp client */
pub const const_XPWD: C2RustUnnamed_5 = 1481660228;
pub const const_PWD: C2RustUnnamed_5 = 5265220;
pub const const_PORT: C2RustUnnamed_5 = 1347375700;
pub const const_PASV: C2RustUnnamed_5 = 1346458454;
pub const const_PASS: C2RustUnnamed_5 = 1346458451;
pub const const_NOOP: C2RustUnnamed_5 = 1313820496;
pub const const_NLST: C2RustUnnamed_5 = 1313624916;
pub const const_MODE: C2RustUnnamed_5 = 1297040453;
pub const const_MKD: C2RustUnnamed_5 = 5065540;
pub const const_MDTM: C2RustUnnamed_5 = 1296323661;
pub const const_LIST: C2RustUnnamed_5 = 1279873876;
pub const const_HELP: C2RustUnnamed_5 = 1212501072;
pub const const_FEAT: C2RustUnnamed_5 = 1178943828;
pub const const_EPSV: C2RustUnnamed_5 = 1162892118;
pub const const_DELE: C2RustUnnamed_5 = 1145392197;
pub const const_CWD: C2RustUnnamed_5 = 4413252;
pub const const_CDUP: C2RustUnnamed_5 = 1128551760;
pub const const_APPE: C2RustUnnamed_5 = 1095782469;
pub const const_ALLO: C2RustUnnamed_5 = 1095519311;
#[inline(always)]
unsafe extern "C" fn xatoul_range(
  mut str: *const libc::c_char,
  mut l: libc::c_ulong,
  mut u: libc::c_ulong,
) -> libc::c_ulong {
  return xatoull_range(str, l as libc::c_ulonglong, u as libc::c_ulonglong) as libc::c_ulong;
}
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
/* ^^^ about 75 bytes smaller code than this: */
//#define G (*(struct globals*)bb_common_bufsiz1)
/*setup_common_bufsiz();*/
/* Moved to main */
/*strcpy(G.msg_ok  + 4, MSG_OK );*/
/*strcpy(G.msg_err + 4, MSG_ERR);*/
unsafe extern "C" fn escape_text(
  mut prepend: *const libc::c_char,
  mut str: *const libc::c_char,
  mut escapee: libc::c_uint,
) -> *mut libc::c_char {
  let mut retlen: libc::c_uint = 0;
  let mut remainlen: libc::c_uint = 0;
  let mut chunklen: libc::c_uint = 0;
  let mut ret: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut found: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut append: libc::c_char = 0;
  append = escapee as libc::c_char;
  escapee >>= 8i32;
  remainlen = strlen(str) as libc::c_uint;
  retlen = strlen(prepend) as libc::c_uint;
  ret = xmalloc(
    retlen
      .wrapping_add(remainlen.wrapping_mul(2i32 as libc::c_uint))
      .wrapping_add(1i32 as libc::c_uint)
      .wrapping_add(1i32 as libc::c_uint) as size_t,
  ) as *mut libc::c_char;
  strcpy(ret, prepend);
  loop {
    found = strchrnul(str, escapee as libc::c_int);
    chunklen = (found.wrapping_offset_from(str) as libc::c_long + 1) as libc::c_uint;
    /* Copy chunk up to and including escapee (or NUL) to ret */
    memcpy(
      ret.offset(retlen as isize) as *mut libc::c_void,
      str as *const libc::c_void,
      chunklen as libc::c_ulong,
    );
    retlen = retlen.wrapping_add(chunklen);
    if *found as libc::c_int == '\u{0}' as i32 {
      /* It wasn't escapee, it was NUL! */
      *ret.offset(retlen.wrapping_sub(1i32 as libc::c_uint) as isize) = append; /* replace NUL */
      *ret.offset(retlen as isize) = '\u{0}' as i32 as libc::c_char; /* add NUL */
      break; /* duplicate escapee */
    } else {
      let fresh0 = retlen;
      retlen = retlen.wrapping_add(1);
      *ret.offset(fresh0 as isize) = escapee as libc::c_char;
      str = found.offset(1)
    }
  }
  return ret;
}
/* Returns strlen as a bonus */
unsafe extern "C" fn replace_char(
  mut str: *mut libc::c_char,
  mut from: libc::c_char,
  mut to: libc::c_char,
) -> libc::c_uint {
  let mut p: *mut libc::c_char = str;
  while *p != 0 {
    if *p as libc::c_int == from as libc::c_int {
      *p = to
    }
    p = p.offset(1)
  }
  return p.wrapping_offset_from(str) as libc::c_long as libc::c_uint;
}
unsafe extern "C" fn verbose_log(mut str: *const libc::c_char) {
  bb_error_msg(
    b"%.*s\x00" as *const u8 as *const libc::c_char,
    strcspn(str, b"\r\n\x00" as *const u8 as *const libc::c_char) as libc::c_int,
    str,
  );
}
/* NB: status_str is char[4] packed into u32 */
unsafe extern "C" fn cmdio_write(mut status_str: u32, mut str: *const libc::c_char) {
  let mut response: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut len: libc::c_int = 0;
  /* FTP uses telnet protocol for command link.
   * In telnet, 0xff is an escape char, and needs to be escaped: */
  response = escape_text(
    &mut status_str as *mut u32 as *mut libc::c_char,
    str,
    ((0xffi32 << 8i32) + '\r' as i32) as libc::c_uint,
  );
  /* FTP sends embedded LFs as NULs */
  len = replace_char(
    response,
    '\n' as i32 as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
  ) as libc::c_int; /* tack on trailing '\n' */
  let fresh1 = len;
  len = len + 1;
  *response.offset(fresh1 as isize) = '\n' as i32 as libc::c_char;
  xwrite(1i32, response as *const libc::c_void, len as size_t);
  if (*ptr_to_globals).verbose > 1i32 as libc::c_uint {
    verbose_log(response);
  }
  free(response as *mut libc::c_void);
}
unsafe extern "C" fn cmdio_write_ok(mut status: libc::c_uint) {
  *((*ptr_to_globals).msg_ok.as_mut_ptr() as *mut bb__aliased_u32) = status;
  xwrite(
    1i32,
    (*ptr_to_globals).msg_ok.as_mut_ptr() as *const libc::c_void,
    (::std::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong),
  );
  if (*ptr_to_globals).verbose > 1i32 as libc::c_uint {
    verbose_log((*ptr_to_globals).msg_ok.as_mut_ptr());
  };
}
/* TODO: output strerr(errno) if errno != 0? */
unsafe extern "C" fn cmdio_write_error(mut status: libc::c_uint) {
  *((*ptr_to_globals).msg_err.as_mut_ptr() as *mut bb__aliased_u32) = status;
  xwrite(
    1i32,
    (*ptr_to_globals).msg_err.as_mut_ptr() as *const libc::c_void,
    (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong),
  );
  if (*ptr_to_globals).verbose > 0i32 as libc::c_uint {
    verbose_log((*ptr_to_globals).msg_err.as_mut_ptr());
  };
}
unsafe extern "C" fn cmdio_write_raw(mut p_text: *const libc::c_char) {
  xwrite_str(1i32, p_text);
  if (*ptr_to_globals).verbose > 1i32 as libc::c_uint {
    verbose_log(p_text);
  };
}
unsafe extern "C" fn timeout_handler(mut _sig: libc::c_int) {
  let mut pos: off_t = 0;
  let mut sv_errno: libc::c_int = *bb_errno;
  if !(monotonic_sec().wrapping_sub((*ptr_to_globals).end_time) as libc::c_int >= 0i32) {
    if !((*ptr_to_globals).local_file_fd == 0) {
      pos = xlseek((*ptr_to_globals).local_file_fd, 0i32 as off_t, 1i32);
      if !(pos == (*ptr_to_globals).local_file_pos) {
        (*ptr_to_globals).local_file_pos = pos;
        alarm((*ptr_to_globals).timeout);
        *bb_errno = sv_errno;
        return;
      }
    }
  }
  cmdio_write_raw(b"421 Timeout\r\n\x00" as *const u8 as *const libc::c_char);
  /* TODO: do we need to abort (as opposed to usual shutdown) data transfer? */
  exit(1i32);
}
/* Simple commands */
unsafe extern "C" fn handle_pwd() {
  let mut cwd: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut response: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  cwd = xrealloc_getcwd_or_warn(0 as *mut libc::c_char);
  if cwd.is_null() {
    cwd = xstrdup(b"\x00" as *const u8 as *const libc::c_char)
  }
  /* We have to promote each " to "" */
  
  response = escape_text(
    b" \"\x00" as *const u8 as *const libc::c_char,
    cwd,
    ((('\"' as i32) << 8i32) + '\"' as i32) as libc::c_uint,
  );
  free(cwd as *mut libc::c_void);
  cmdio_write(
    (0i32
      | '0' as i32 + 257i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
      | '0' as i32 + 257i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
      | '0' as i32 + 257i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
    response,
  );
  free(response as *mut libc::c_void);
}
unsafe extern "C" fn handle_cwd() {
  if (*ptr_to_globals).ftp_arg.is_null() || chdir((*ptr_to_globals).ftp_arg) != 0i32 {
    cmdio_write_error(
      (0i32
        | (' ' as i32) << SHIFTsp as libc::c_int
        | '0' as i32 + 550i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
        | '0' as i32 + 550i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
        | '0' as i32 + 550i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
    );
    return;
  }
  cmdio_write_ok(
    (0i32
      | (' ' as i32) << SHIFTsp as libc::c_int
      | '0' as i32 + 250i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
      | '0' as i32 + 250i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
      | '0' as i32 + 250i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
  );
}
unsafe extern "C" fn handle_cdup() {
  (*ptr_to_globals).ftp_arg = b"..\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
  handle_cwd();
}
unsafe extern "C" fn handle_stat() {
  cmdio_write_raw(
    b"211-Server status:\r\n TYPE: BINARY\r\n211 Ok\r\n\x00" as *const u8 as *const libc::c_char,
  );
}
/* Examples of HELP and FEAT:
# nc -vvv ftp.kernel.org 21
ftp.kernel.org (130.239.17.4:21) open
220 Welcome to ftp.kernel.org.
FEAT
211-Features:
 EPRT
 EPSV
 MDTM
 PASV
 REST STREAM
 SIZE
 TVFS
 UTF8
211 End
HELP
214-The following commands are recognized.
 ABOR ACCT ALLO APPE CDUP CWD  DELE EPRT EPSV FEAT HELP LIST MDTM MKD
 MODE NLST NOOP OPTS PASS PASV PORT PWD  QUIT REIN REST RETR RMD  RNFR
 RNTO SITE SIZE SMNT STAT STOR STOU STRU SYST TYPE USER XCUP XCWD XMKD
 XPWD XRMD
214 Help OK.
*/
unsafe extern "C" fn handle_feat(mut status: libc::c_uint) {
  cmdio_write(
    status,
    b"-Features:\x00" as *const u8 as *const libc::c_char,
  );
  cmdio_write_raw(
    b" EPSV\r\n PASV\r\n REST STREAM\r\n MDTM\r\n SIZE\r\n\x00" as *const u8 as *const libc::c_char,
  );
  cmdio_write(status, b" Ok\x00" as *const u8 as *const libc::c_char);
}
/* Download commands */
#[inline]
unsafe extern "C" fn port_active() -> libc::c_int {
  return ((*ptr_to_globals).port_addr != 0 as *mut libc::c_void as *mut len_and_sockaddr)
    as libc::c_int;
}
#[inline]
unsafe extern "C" fn pasv_active() -> libc::c_int {
  return ((*ptr_to_globals).pasv_listen_fd > 1i32) as libc::c_int;
}
unsafe extern "C" fn port_pasv_cleanup() {
  free((*ptr_to_globals).port_addr as *mut libc::c_void);
  (*ptr_to_globals).port_addr = 0 as *mut len_and_sockaddr;
  if (*ptr_to_globals).pasv_listen_fd > 1i32 {
    close((*ptr_to_globals).pasv_listen_fd);
  }
  (*ptr_to_globals).pasv_listen_fd = -1i32;
}
/* On error, emits error code to the peer */
unsafe extern "C" fn ftpdataio_get_pasv_fd() -> libc::c_int {
  let mut remote_fd: libc::c_int = 0;
  remote_fd = accept(
    (*ptr_to_globals).pasv_listen_fd,
    __SOCKADDR_ARG {
      __sockaddr__: 0 as *mut libc::c_void as *mut sockaddr,
    },
    0 as *mut socklen_t,
  );
  if remote_fd < 0i32 {
    cmdio_write_error(
      (0i32
        | (' ' as i32) << SHIFTsp as libc::c_int
        | '0' as i32 + 425i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
        | '0' as i32 + 425i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
        | '0' as i32 + 425i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
    );
    return remote_fd;
  }
  setsockopt_keepalive(remote_fd);
  return remote_fd;
}
/* Clears port/pasv data.
 * This means we dont waste resources, for example, keeping
 * PASV listening socket open when it is no longer needed.
 * On error, emits error code to the peer (or exits).
 * On success, emits p_status_msg to the peer.
 */
unsafe extern "C" fn get_remote_transfer_fd(mut p_status_msg: *const libc::c_char) -> libc::c_int {
  let mut remote_fd: libc::c_int = 0;
  if pasv_active() != 0 {
    /* On error, emits error code to the peer */
    remote_fd = ftpdataio_get_pasv_fd()
  } else {
    /* Exits on error */
    remote_fd = xconnect_stream((*ptr_to_globals).port_addr)
  }
  port_pasv_cleanup();
  if remote_fd < 0i32 {
    return remote_fd;
  }
  cmdio_write(
    (0i32
      | '0' as i32 + 150i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
      | '0' as i32 + 150i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
      | '0' as i32 + 150i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
    p_status_msg,
  );
  return remote_fd;
}
/* If there were neither PASV nor PORT, emits error code to the peer */
unsafe extern "C" fn port_or_pasv_was_seen() -> libc::c_int {
  if pasv_active() == 0 && port_active() == 0 {
    cmdio_write_raw(b"425 Use PORT/PASV first\r\n\x00" as *const u8 as *const libc::c_char);
    return 0i32;
  }
  return 1i32;
}
/* Exits on error */
unsafe extern "C" fn bind_for_passive_mode() -> libc::c_uint {
  let mut fd: libc::c_int = 0;
  let mut port: libc::c_uint = 0;
  port_pasv_cleanup();
  fd = xsocket(
    (*(*ptr_to_globals).local_addr).u.sa.sa_family as libc::c_int,
    SOCK_STREAM as libc::c_int,
    0i32,
  );
  (*ptr_to_globals).pasv_listen_fd = fd;
  setsockopt_reuseaddr(fd);
  set_nport(
    &mut (*(*ptr_to_globals).local_addr).u.sa,
    0i32 as libc::c_uint,
  );
  xbind(
    fd,
    &mut (*(*ptr_to_globals).local_addr).u.sa,
    (*(*ptr_to_globals).local_addr).len,
  );
  xlisten(fd, 1i32);
  getsockname(
    fd,
    __SOCKADDR_ARG {
      __sockaddr__: &mut (*(*ptr_to_globals).local_addr).u.sa as *mut sockaddr,
    },
    &mut (*(*ptr_to_globals).local_addr).len,
  );
  port = get_nport(&mut (*(*ptr_to_globals).local_addr).u.sa) as libc::c_uint;
  port = ({
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = port as libc::c_ushort;
    if false {
      __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
        as libc::c_ushort
    } else {
      let fresh2 = &mut __v;
      let fresh3;
      let fresh4 = __x;
      asm!("rorw $$8, ${0:w}" : "=r" (fresh3) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh2, fresh4)) :
                      "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh2, fresh4, fresh3);
    }
    __v
  }) as libc::c_uint;
  return port;
}
/* Exits on error */
unsafe extern "C" fn handle_pasv() {
  let mut port: libc::c_uint = 0;
  let mut addr: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut response: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  port = bind_for_passive_mode();
  if (*(*ptr_to_globals).local_addr).u.sa.sa_family as libc::c_int == 2i32 {
    addr = xmalloc_sockaddr2dotted_noport(&mut (*(*ptr_to_globals).local_addr).u.sa)
  } else {
    /* seen this in the wild done by other ftp servers: */
    addr = xstrdup(b"0.0.0.0\x00" as *const u8 as *const libc::c_char)
  }
  replace_char(addr, '.' as i32 as libc::c_char, ',' as i32 as libc::c_char);
  response = xasprintf(
    b"227 PASV ok (%s,%u,%u)\r\n\x00" as *const u8 as *const libc::c_char,
    addr,
    (port >> 8i32) as libc::c_int,
    (port & 255i32 as libc::c_uint) as libc::c_int,
  );
  free(addr as *mut libc::c_void);
  cmdio_write_raw(response);
  free(response as *mut libc::c_void);
}
/* Exits on error */
unsafe extern "C" fn handle_epsv() {
  let mut port: libc::c_uint = 0;
  let mut response: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  port = bind_for_passive_mode();
  response = xasprintf(
    b"229 EPSV ok (|||%u|)\r\n\x00" as *const u8 as *const libc::c_char,
    port,
  );
  cmdio_write_raw(response);
  free(response as *mut libc::c_void);
}
unsafe extern "C" fn handle_port() {
  let mut port: libc::c_uint = 0;
  let mut port_hi: libc::c_uint = 0;
  let mut raw: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut comma: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  port_pasv_cleanup();
  raw = (*ptr_to_globals).ftp_arg;
  /* PORT command format makes sense only over IPv4 */
  if !raw.is_null() {
    comma = strrchr(raw, ',' as i32);
    if !comma.is_null() {
      *comma = '\u{0}' as i32 as libc::c_char;
      port = bb_strtou(&mut *comma.offset(1), 0 as *mut *mut libc::c_char, 10i32);
      if !(*bb_errno != 0 || port > 0xffi32 as libc::c_uint) {
        comma = strrchr(raw, ',' as i32);
        if !comma.is_null() {
          *comma = '\u{0}' as i32 as libc::c_char;
          port_hi = bb_strtou(&mut *comma.offset(1), 0 as *mut *mut libc::c_char, 10i32);
          if !(*bb_errno != 0 || port_hi > 0xffi32 as libc::c_uint) {
            port |= port_hi << 8i32;
            (*ptr_to_globals).port_addr = get_peer_lsa(0i32);
            set_nport(
              &mut (*(*ptr_to_globals).port_addr).u.sa,
              ({
                let mut __v: libc::c_ushort = 0;
                let mut __x: libc::c_ushort = port as libc::c_ushort;
                if false {
                  __v = (__x as libc::c_int >> 8i32 & 0xffi32
                    | (__x as libc::c_int & 0xffi32) << 8i32)
                    as libc::c_ushort
                } else {
                  let fresh5 = &mut __v;
                  let fresh6;
                  let fresh7 = __x;
                  asm!("rorw $$8, ${0:w}" : "=r"
                                                (fresh6) : "0"
                                                (c2rust_asm_casts::AsmCast::cast_in(fresh5, fresh7))
                                                : "cc");
                  c2rust_asm_casts::AsmCast::cast_out(fresh5, fresh7, fresh6);
                }
                __v
              }) as libc::c_uint,
            );
            cmdio_write_ok(
              (0i32
                | (' ' as i32) << SHIFTsp as libc::c_int
                | '0' as i32 + 200i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
                | '0' as i32 + 200i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
                | '0' as i32 + 200i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int)
                as u32,
            );
            return;
          }
        }
      }
    }
  }
  cmdio_write_error(
    (0i32
      | (' ' as i32) << SHIFTsp as libc::c_int
      | '0' as i32 + 500i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
      | '0' as i32 + 500i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
      | '0' as i32 + 500i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
  );
}
unsafe extern "C" fn handle_rest() {
  /* When ftp_arg == NULL simply restart from beginning */
  (*ptr_to_globals).restart_pos = if !(*ptr_to_globals).ftp_arg.is_null() {
    xatoul_range(
      (*ptr_to_globals).ftp_arg,
      0i32 as libc::c_ulong,
      9223372036854775807i64 as libc::c_ulong,
    )
  } else {
    0i32 as libc::c_ulong
  } as off_t; /* port_or_pasv_was_seen emitted error response */
  cmdio_write_ok(
    (0i32
      | (' ' as i32) << SHIFTsp as libc::c_int
      | '0' as i32 + 350i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
      | '0' as i32 + 350i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
      | '0' as i32 + 350i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
  );
}
unsafe extern "C" fn handle_retr() {
  let mut statbuf: stat = std::mem::zeroed();
  let mut bytes_transferred: off_t = 0;
  let mut remote_fd: libc::c_int = 0;
  let mut local_file_fd: libc::c_int = 0;
  let mut offset: off_t = (*ptr_to_globals).restart_pos;
  let mut response: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  (*ptr_to_globals).restart_pos = 0i32 as off_t;
  if port_or_pasv_was_seen() == 0 {
    return;
  }
  /* O_NONBLOCK is useful if file happens to be a device node */
  local_file_fd = if !(*ptr_to_globals).ftp_arg.is_null() {
    open((*ptr_to_globals).ftp_arg, 0i32 | 0o4000i32)
  } else {
    -1i32
  };
  if local_file_fd < 0i32 {
    cmdio_write_error(
      (0i32
        | (' ' as i32) << SHIFTsp as libc::c_int
        | '0' as i32 + 550i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
        | '0' as i32 + 550i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
        | '0' as i32 + 550i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
    );
    return;
  }
  if fstat(local_file_fd, &mut statbuf) != 0i32
    || !(statbuf.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint)
  {
    /* Note - pretend open failed */
    cmdio_write_error(
      (0i32
        | (' ' as i32) << SHIFTsp as libc::c_int
        | '0' as i32 + 550i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
        | '0' as i32 + 550i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
        | '0' as i32 + 550i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
    );
  } else {
    (*ptr_to_globals).local_file_fd = local_file_fd;
    /* Now deactive O_NONBLOCK, otherwise we have a problem
     * on DMAPI filesystems such as XFS DMAPI.
     */
    ndelay_off(local_file_fd);
    /* Set the download offset (from REST) if any */
    if offset != 0 {
      xlseek(local_file_fd, offset, 0i32);
    }
    response = xasprintf(
      b" Opening BINARY connection for %s (%lu bytes)\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).ftp_arg,
      statbuf.st_size,
    );
    remote_fd = get_remote_transfer_fd(response);
    free(response as *mut libc::c_void);
    if !(remote_fd < 0i32) {
      bytes_transferred = bb_copyfd_eof(local_file_fd, remote_fd);
      close(remote_fd);
      if bytes_transferred < 0 {
        cmdio_write_error(
          (0i32
            | (' ' as i32) << SHIFTsp as libc::c_int
            | '0' as i32 + 451i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
            | '0' as i32 + 451i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
            | '0' as i32 + 451i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
        );
      } else {
        cmdio_write_ok(
          (0i32
            | (' ' as i32) << SHIFTsp as libc::c_int
            | '0' as i32 + 226i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
            | '0' as i32 + 226i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
            | '0' as i32 + 226i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
        );
      }
    }
  }
  close(local_file_fd);
  (*ptr_to_globals).local_file_fd = 0i32;
}
/* List commands */
unsafe extern "C" fn popen_ls(mut opt: *const libc::c_char) -> libc::c_int {
  let mut argv: [*const libc::c_char; 5] = [0 as *const libc::c_char; 5]; /* "-lA" or "-1A" */
  let mut outfd: fd_pair = fd_pair { rd: 0, wr: 0 };
  let mut pid: pid_t = 0;
  argv[0] = b"ftpd\x00" as *const u8 as *const libc::c_char;
  argv[1] = opt;
  argv[2] = b"--\x00" as *const u8 as *const libc::c_char;
  argv[3] = (*ptr_to_globals).ftp_arg;
  argv[4] = 0 as *const libc::c_char;
  /* Improve compatibility with non-RFC conforming FTP clients
   * which send e.g. "LIST -l", "LIST -la", "LIST -aL".
   * See https://bugs.kde.org/show_bug.cgi?id=195578 */
  if 1i32 != 0
    && !(*ptr_to_globals).ftp_arg.is_null()
    && *(*ptr_to_globals).ftp_arg.offset(0) as libc::c_int == '-' as i32
  {
    let mut tmp: *const libc::c_char = strchr((*ptr_to_globals).ftp_arg, ' ' as i32);
    if !tmp.is_null() {
      /* skip the space */
      tmp = tmp.offset(1)
    }
    argv[3] = tmp
  }
  xpipe(&mut outfd.rd);
  /*fflush_all(); - so far we dont use stdio on output */
  pid = if 1i32 != 0 {
    xfork()
  } else {
    ({
      let mut bb__xvfork_pid: pid_t = vfork();
      if bb__xvfork_pid < 0i32 {
        bb_simple_perror_msg_and_die(b"vfork\x00" as *const u8 as *const libc::c_char);
      }
      bb__xvfork_pid
    })
  };
  if pid == 0i32 {
    /* child */
    /* NB: close _first_, then move fd! */
    close(outfd.rd);
    xmove_fd(outfd.wr, 1i32);
    /* Opening /dev/null in chroot is hard.
     * Just making sure STDIN_FILENO is opened
     * to something harmless. Paranoia,
     * ls won't read it anyway */
    close(0i32); /* copy will become STDIN_FILENO */
    dup(1i32);
    /* memset(&G, 0, sizeof(G)); - ls_main does it */
    exit(ls_main(0i32, argv.as_mut_ptr() as *mut *mut libc::c_char));
  }
  /* parent */
  close(outfd.wr); /* port_or_pasv_was_seen emitted error response */
  return outfd.rd;
}
unsafe extern "C" fn handle_dir_common(mut opts: libc::c_int) {
  let mut ls_fp: *mut FILE = 0 as *mut FILE;
  let mut line: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut ls_fd: libc::c_int = 0;
  if opts & USE_CTRL_CONN as libc::c_int == 0 && port_or_pasv_was_seen() == 0 {
    return;
  }
  ls_fd = popen_ls(if opts & LONG_LISTING as libc::c_int != 0 {
    b"-lA\x00" as *const u8 as *const libc::c_char
  } else {
    b"-1A\x00" as *const u8 as *const libc::c_char
  });
  ls_fp = xfdopen_for_read(ls_fd);
  /* FIXME: filenames with embedded newlines are mishandled */
  if opts & USE_CTRL_CONN as libc::c_int != 0 {
    /* STAT <filename> */
    cmdio_write_raw(b"213-File status:\r\n\x00" as *const u8 as *const libc::c_char);
    loop {
      line = xmalloc_fgetline(ls_fp);
      if line.is_null() {
        break;
      }
      /* Hack: 0 results in no status at all */
      /* Note: it's ok that we don't prepend space,
       * ftp.kernel.org doesn't do that too */
      cmdio_write(0i32 as u32, line);
      free(line as *mut libc::c_void);
    }
    cmdio_write_ok(
      (0i32
        | (' ' as i32) << SHIFTsp as libc::c_int
        | '0' as i32 + 213i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
        | '0' as i32 + 213i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
        | '0' as i32 + 213i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
    );
  } else {
    /* LIST/NLST [<filename>] */
    let mut remote_fd: libc::c_int =
      get_remote_transfer_fd(b" Directory listing\x00" as *const u8 as *const libc::c_char);
    if remote_fd >= 0i32 {
      loop {
        let mut len: libc::c_uint = 0;
        line = xmalloc_fgets(ls_fp);
        if line.is_null() {
          break;
        }
        /* I've seen clients complaining when they
         * are fed with ls output with bare '\n'.
         * Replace trailing "\n\0" with "\r\n".
         */
        len = strlen(line) as libc::c_uint;
        if len != 0i32 as libc::c_uint {
          /* paranoia check */
          *line.offset(len.wrapping_sub(1i32 as libc::c_uint) as isize) =
            '\r' as i32 as libc::c_char
        }
        *line.offset(len as isize) = '\n' as i32 as libc::c_char;
        xwrite(
          remote_fd,
          line as *const libc::c_void,
          len.wrapping_add(1i32 as libc::c_uint) as size_t,
        );
        free(line as *mut libc::c_void);
      }
    }
    close(remote_fd);
    cmdio_write_ok(
      (0i32
        | (' ' as i32) << SHIFTsp as libc::c_int
        | '0' as i32 + 226i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
        | '0' as i32 + 226i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
        | '0' as i32 + 226i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
    );
  }
  fclose(ls_fp);
  /* closes ls_fd too */
}
unsafe extern "C" fn handle_list() {
  handle_dir_common(LONG_LISTING as libc::c_int);
}
unsafe extern "C" fn handle_nlst() {
  /* NLST returns list of names, "\r\n" terminated without regard
   * to the current binary flag. Names may start with "/",
   * then they represent full names (we don't produce such names),
   * otherwise names are relative to current directory.
   * Embedded "\n" are replaced by NULs. This is safe since names
   * can never contain NUL.
   */
  handle_dir_common(0i32);
}
unsafe extern "C" fn handle_stat_file() {
  handle_dir_common(LONG_LISTING as libc::c_int + USE_CTRL_CONN as libc::c_int);
}
/* This can be extended to handle MLST, as all info is available
 * in struct stat for that:
 * MLST file_name
 * 250-Listing file_name
 *  type=file;size=4161;modify=19970214165800; /dir/dir/file_name
 * 250 End
 * Nano-doc:
 * MLST [<file or dir name, "." assumed if not given>]
 * Returned name should be either the same as requested, or fully qualified.
 * If there was no parameter, return "" or (preferred) fully-qualified name.
 * Returned "facts" (case is not important):
 *  size    - size in octets
 *  modify  - last modification time
 *  type    - entry type (file,dir,OS.unix=block)
 *            (+ cdir and pdir types for MLSD)
 *  unique  - unique id of file/directory (inode#)
 *  perm    -
 *      a: can be appended to (APPE)
 *      d: can be deleted (RMD/DELE)
 *      f: can be renamed (RNFR)
 *      r: can be read (RETR)
 *      w: can be written (STOR)
 *      e: can CWD into this dir
 *      l: this dir can be listed (dir only!)
 *      c: can create files in this dir
 *      m: can create dirs in this dir (MKD)
 *      p: can delete files in this dir
 *  UNIX.mode - unix file mode
 */
unsafe extern "C" fn handle_size_or_mdtm(mut need_size: libc::c_int) {
  let mut statbuf: stat = std::mem::zeroed();
  let mut broken_out: tm = std::mem::zeroed();
  let mut buf: [libc::c_char; 55] = [0; 55];
  if (*ptr_to_globals).ftp_arg.is_null()
    || stat((*ptr_to_globals).ftp_arg, &mut statbuf) != 0i32
    || !(statbuf.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint)
  {
    cmdio_write_error(
      (0i32
        | (' ' as i32) << SHIFTsp as libc::c_int
        | '0' as i32 + 550i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
        | '0' as i32 + 550i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
        | '0' as i32 + 550i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
    );
    return;
  }
  if need_size != 0 {
    sprintf(
      buf.as_mut_ptr(),
      b"213 %lu\r\n\x00" as *const u8 as *const libc::c_char,
      statbuf.st_size,
    );
  } else {
    gmtime_r(&mut statbuf.st_mtime, &mut broken_out);
    sprintf(
      buf.as_mut_ptr(),
      b"213 %04u%02u%02u%02u%02u%02u\r\n\x00" as *const u8 as *const libc::c_char,
      broken_out.tm_year + 1900i32,
      broken_out.tm_mon + 1i32,
      broken_out.tm_mday,
      broken_out.tm_hour,
      broken_out.tm_min,
      broken_out.tm_sec,
    );
  }
  cmdio_write_raw(buf.as_mut_ptr());
}
/* Upload commands */
unsafe extern "C" fn handle_mkd() {
  if (*ptr_to_globals).ftp_arg.is_null()
    || mkdir((*ptr_to_globals).ftp_arg, 0o777i32 as mode_t) != 0i32
  {
    cmdio_write_error(
      (0i32
        | (' ' as i32) << SHIFTsp as libc::c_int
        | '0' as i32 + 550i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
        | '0' as i32 + 550i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
        | '0' as i32 + 550i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
    );
    return;
  }
  cmdio_write_ok(
    (0i32
      | (' ' as i32) << SHIFTsp as libc::c_int
      | '0' as i32 + 257i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
      | '0' as i32 + 257i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
      | '0' as i32 + 257i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
  );
}
unsafe extern "C" fn handle_rmd() {
  if (*ptr_to_globals).ftp_arg.is_null() || rmdir((*ptr_to_globals).ftp_arg) != 0i32 {
    cmdio_write_error(
      (0i32
        | (' ' as i32) << SHIFTsp as libc::c_int
        | '0' as i32 + 550i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
        | '0' as i32 + 550i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
        | '0' as i32 + 550i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
    );
    return;
  }
  cmdio_write_ok(
    (0i32
      | (' ' as i32) << SHIFTsp as libc::c_int
      | '0' as i32 + 250i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
      | '0' as i32 + 250i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
      | '0' as i32 + 250i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
  );
}
unsafe extern "C" fn handle_dele() {
  if (*ptr_to_globals).ftp_arg.is_null() || unlink((*ptr_to_globals).ftp_arg) != 0i32 {
    cmdio_write_error(
      (0i32
        | (' ' as i32) << SHIFTsp as libc::c_int
        | '0' as i32 + 550i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
        | '0' as i32 + 550i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
        | '0' as i32 + 550i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
    );
    return;
  }
  cmdio_write_ok(
    (0i32
      | (' ' as i32) << SHIFTsp as libc::c_int
      | '0' as i32 + 250i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
      | '0' as i32 + 250i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
      | '0' as i32 + 250i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
  );
}
unsafe extern "C" fn handle_rnfr() {
  free((*ptr_to_globals).rnfr_filename as *mut libc::c_void);
  (*ptr_to_globals).rnfr_filename = xstrdup((*ptr_to_globals).ftp_arg);
  cmdio_write_ok(
    (0i32
      | (' ' as i32) << SHIFTsp as libc::c_int
      | '0' as i32 + 350i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
      | '0' as i32 + 350i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
      | '0' as i32 + 350i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
  );
}
unsafe extern "C" fn handle_rnto() {
  let mut retval: libc::c_int = 0;
  /* If we didn't get a RNFR, throw a wobbly */
  if (*ptr_to_globals).rnfr_filename.is_null() || (*ptr_to_globals).ftp_arg.is_null() {
    cmdio_write_raw(b"503 Use RNFR first\r\n\x00" as *const u8 as *const libc::c_char); /* port_or_pasv_was_seen emitted error response */
    return;
  }
  retval = rename((*ptr_to_globals).rnfr_filename, (*ptr_to_globals).ftp_arg);
  free((*ptr_to_globals).rnfr_filename as *mut libc::c_void);
  (*ptr_to_globals).rnfr_filename = std::ptr::null_mut::<libc::c_char>();
  if retval != 0 {
    cmdio_write_error(
      (0i32
        | (' ' as i32) << SHIFTsp as libc::c_int
        | '0' as i32 + 550i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
        | '0' as i32 + 550i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
        | '0' as i32 + 550i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
    );
    return;
  }
  cmdio_write_ok(
    (0i32
      | (' ' as i32) << SHIFTsp as libc::c_int
      | '0' as i32 + 250i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
      | '0' as i32 + 250i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
      | '0' as i32 + 250i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
  );
}
unsafe extern "C" fn handle_upload_common(mut is_append: libc::c_int, mut is_unique: libc::c_int) {
  let mut statbuf: stat = std::mem::zeroed();
  let mut tempname: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut bytes_transferred: off_t = 0;
  let mut offset: off_t = 0;
  let mut local_file_fd: libc::c_int = 0;
  let mut remote_fd: libc::c_int = 0;
  offset = (*ptr_to_globals).restart_pos;
  (*ptr_to_globals).restart_pos = 0i32 as off_t;
  if port_or_pasv_was_seen() == 0 {
    return;
  }
  tempname = std::ptr::null_mut::<libc::c_char>();
  local_file_fd = -1i32;
  if is_unique != 0 {
    tempname = xstrdup(b" FILE: uniq.XXXXXX\x00" as *const u8 as *const libc::c_char);
    local_file_fd = mkstemp(tempname.offset(7))
  } else if !(*ptr_to_globals).ftp_arg.is_null() {
    let mut flags: libc::c_int = 0o1i32 | 0o100i32 | 0o1000i32;
    if is_append != 0 {
      flags = 0o1i32 | 0o100i32 | 0o2000i32
    }
    if offset != 0 {
      flags = 0o1i32 | 0o100i32
    }
    local_file_fd = open((*ptr_to_globals).ftp_arg, flags, 0o666i32)
  }
  if local_file_fd < 0i32
    || fstat(local_file_fd, &mut statbuf) != 0i32
    || !(statbuf.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint)
  {
    free(tempname as *mut libc::c_void);
    cmdio_write_error(
      (0i32
        | (' ' as i32) << SHIFTsp as libc::c_int
        | '0' as i32 + 553i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
        | '0' as i32 + 553i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
        | '0' as i32 + 553i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
    );
    if !(local_file_fd >= 0i32) {
      return;
    }
  } else {
    (*ptr_to_globals).local_file_fd = local_file_fd;
    if offset != 0 {
      xlseek(local_file_fd, offset, 0i32);
    }
    remote_fd = get_remote_transfer_fd(if !tempname.is_null() {
      tempname
    } else {
      b" Ok to send data\x00" as *const u8 as *const libc::c_char
    });
    free(tempname as *mut libc::c_void);
    if !(remote_fd < 0i32) {
      bytes_transferred = bb_copyfd_eof(remote_fd, local_file_fd);
      close(remote_fd);
      if bytes_transferred < 0 {
        cmdio_write_error(
          (0i32
            | (' ' as i32) << SHIFTsp as libc::c_int
            | '0' as i32 + 451i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
            | '0' as i32 + 451i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
            | '0' as i32 + 451i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
        );
      } else {
        cmdio_write_ok(
          (0i32
            | (' ' as i32) << SHIFTsp as libc::c_int
            | '0' as i32 + 226i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
            | '0' as i32 + 226i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
            | '0' as i32 + 226i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
        );
      }
    }
  }
  close(local_file_fd);
  (*ptr_to_globals).local_file_fd = 0i32;
}
unsafe extern "C" fn handle_stor() {
  handle_upload_common(0i32, 0i32);
}
unsafe extern "C" fn handle_appe() {
  (*ptr_to_globals).restart_pos = 0i32 as off_t;
  handle_upload_common(1i32, 0i32);
}
unsafe extern "C" fn handle_stou() {
  (*ptr_to_globals).restart_pos = 0i32 as off_t;
  handle_upload_common(0i32, 1i32);
}
/* ENABLE_FEATURE_FTPD_WRITE */
unsafe extern "C" fn cmdio_get_cmd_and_arg() -> u32 {
  let mut len: libc::c_int = 0;
  let mut cmdval: u32 = 0;
  let mut cmd: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  alarm((*ptr_to_globals).timeout);
  free((*ptr_to_globals).ftp_cmd as *mut libc::c_void);
  /* Paranoia. Peer may send 1 gigabyte long cmd... */
  /* Using separate len_on_stk instead of len optimizes
   * code size (allows len to be in CPU register) */
  let mut len_on_stk: size_t = (8i32 * 1024i32) as size_t;
  cmd = xmalloc_fgets_str_len(
    stdin,
    b"\r\n\x00" as *const u8 as *const libc::c_char,
    &mut len_on_stk,
  );
  (*ptr_to_globals).ftp_cmd = cmd;
  if cmd.is_null() {
    exit(0i32);
  }
  len = len_on_stk as libc::c_int;
  /* De-escape telnet: 0xff,0xff => 0xff */
  /* RFC959 says that ABOR, STAT, QUIT may be sent even during
   * data transfer, and may be preceded by telnet's "Interrupt Process"
   * code (two-byte sequence 255,244) and then by telnet "Synch" code
   * 255,242 (byte 242 is sent with TCP URG bit using send(MSG_OOB)
   * and may generate SIGURG on our side. See RFC854).
   * So far we don't support that (may install SIGURG handler if we'd want to),
   * but we need to at least remove 255,xxx pairs. lftp sends those. */
  /* Then de-escape FTP: NUL => '\n' */
  /* Testing for \xff:
   * Create file named '\xff': echo Hello >`echo -ne "\xff"`
   * Try to get it:            ftpget -v 127.0.0.1 Eff `echo -ne "\xff\xff"`
   * (need "\xff\xff" until ftpget applet is fixed to do escaping :)
   * Testing for embedded LF:
   * LF_HERE=`echo -ne "LF\nHERE"`
   * echo Hello >"$LF_HERE"
   * ftpget -v 127.0.0.1 LF_HERE "$LF_HERE"
   */
  let mut dst: libc::c_int = 0;
  let mut src: libc::c_int = 0;
  /* Strip "\r\n" if it is there */
  if len != 0i32 && *cmd.offset((len - 1i32) as isize) as libc::c_int == '\n' as i32 {
    len -= 1;
    if len != 0i32 && *cmd.offset((len - 1i32) as isize) as libc::c_int == '\r' as i32 {
      len -= 1
    }
    *cmd.offset(len as isize) = '\u{0}' as i32 as libc::c_char
  }
  src = strchrnul(cmd, 0xffi32).wrapping_offset_from(cmd) as libc::c_long as libc::c_int;
  /* 99,99% there are neither NULs nor 255s and src == len */
  if src < len {
    dst = src;
    let mut current_block_18: u64;
    loop {
      if *cmd.offset(src as isize) as libc::c_uchar as libc::c_int == 255i32 {
        src += 1;
        /* 255,255 - retain one 255 */
        if *cmd.offset(src as isize) as libc::c_uchar as libc::c_int != 255i32 {
          /* 255,xxx - skip 255 */
          /* 255,!255 - skip both */
          src += 1;
          current_block_18 = 15652330335145281839;
        } else {
          current_block_18 = 1109700713171191020;
        }
      } else {
        current_block_18 = 1109700713171191020;
      }
      match current_block_18 {
        1109700713171191020 => {
          /* NUL => '\n' */
          let fresh8 = dst;
          dst = dst + 1;
          *cmd.offset(fresh8 as isize) = if *cmd.offset(src as isize) as libc::c_int != 0 {
            *cmd.offset(src as isize) as libc::c_int
          } else {
            '\n' as i32
          } as libc::c_char;
          src += 1
        }
        _ => {}
      }
      if !(src < len) {
        break;
      }
    }
    *cmd.offset(dst as isize) = '\u{0}' as i32 as libc::c_char
  }
  if (*ptr_to_globals).verbose > 1i32 as libc::c_uint {
    verbose_log(cmd);
  }
  (*ptr_to_globals).ftp_arg = strchr(cmd, ' ' as i32);
  if !(*ptr_to_globals).ftp_arg.is_null() {
    let fresh9 = (*ptr_to_globals).ftp_arg;
    (*ptr_to_globals).ftp_arg = (*ptr_to_globals).ftp_arg.offset(1);
    *fresh9 = '\u{0}' as i32 as libc::c_char
  }
  /* Uppercase and pack into u32 first word of the command */
  cmdval = 0i32 as u32;
  while *cmd != 0 {
    let fresh10 = cmd;
    cmd = cmd.offset(1);
    cmdval = (cmdval << 8i32).wrapping_add(
      (*fresh10 as libc::c_uchar as libc::c_int & !0x20i32 as libc::c_uchar as libc::c_int)
        as libc::c_uint,
    )
  }
  return cmdval;
}
#[no_mangle]
pub unsafe extern "C" fn ftpd_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut pw: *mut passwd = 0 as *mut passwd;
  let mut anon_opt: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut abs_timeout: libc::c_uint = 0;
  let mut verbose_S: libc::c_uint = 0;
  let mut opts: smallint = 0;
  let ref mut fresh11 =
    *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
      as *mut *mut globals);
  *fresh11 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  abs_timeout = (1i32 * 60i32 * 60i32) as libc::c_uint;
  verbose_S = 0i32 as libc::c_uint;
  (*ptr_to_globals).timeout = (2i32 * 60i32) as libc::c_uint;
  opts = getopt32(
    argv,
    b"^AvSwt:+T:+a:\x00vv:SS\x00" as *const u8 as *const libc::c_char,
    &mut (*ptr_to_globals).timeout as *mut libc::c_uint,
    &mut abs_timeout as *mut libc::c_uint,
    &mut anon_opt as *mut *mut libc::c_char,
    &mut (*ptr_to_globals).verbose as *mut libc::c_uint,
    &mut verbose_S as *mut libc::c_uint,
  ) as smallint;
  if (*ptr_to_globals).verbose < verbose_S {
    (*ptr_to_globals).verbose = verbose_S
  }
  if abs_timeout | (*ptr_to_globals).timeout != 0 {
    if abs_timeout == 0i32 as libc::c_uint {
      abs_timeout = 2147483647i32 as libc::c_uint
    }
    (*ptr_to_globals).end_time = monotonic_sec().wrapping_add(abs_timeout);
    if (*ptr_to_globals).timeout > abs_timeout {
      (*ptr_to_globals).timeout = abs_timeout
    }
  }
  strcpy(
    (*ptr_to_globals).msg_ok.as_mut_ptr().offset(4),
    b"Operation successful\r\n\x00" as *const u8 as *const libc::c_char,
  );
  strcpy(
    (*ptr_to_globals).msg_err.as_mut_ptr().offset(4),
    b"Error\r\n\x00" as *const u8 as *const libc::c_char,
  );
  (*ptr_to_globals).local_addr = get_sock_lsa(0i32);
  if (*ptr_to_globals).local_addr.is_null() {
    /* This is confusing:
     * bb_error_msg_and_die("stdin is not a socket");
     * Better: */
    bb_show_usage();
    /* Help text says that ftpd must be used as inetd service,
     * which is by far the most usual cause of get_sock_lsa
     * failure */
  }
  if opts as libc::c_int & OPT_v as libc::c_int == 0 {
    logmode = LOGMODE_NONE as libc::c_int as smallint
  }
  if opts as libc::c_int & OPT_S as libc::c_int != 0 {
    /* LOG_NDELAY is needed since we may chroot later */
    openlog(applet_name, 0x1i32 | 0x8i32, 3i32 << 3i32);
    logmode = (logmode as libc::c_int | LOGMODE_SYSLOG as libc::c_int) as smallint
  }
  if logmode != 0 {
    applet_name = xasprintf(
      b"%s[%u]\x00" as *const u8 as *const libc::c_char,
      applet_name,
      getpid(),
    )
  }
  //umask(077); - admin can set umask before starting us
  /* Signals */
  bb_signals(
    0i32 + (1i32 << 13i32) + (1i32 << 17i32),
    ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
  );
  /* Set up options on the command socket (do we need these all? why?) */
  setsockopt_1(0i32, IPPROTO_TCP as libc::c_int, 1i32);
  setsockopt_keepalive(0i32);
  /* Telnet protocol over command link may send "urgent" data,
   * we prefer it to be received in the "normal" data stream: */
  setsockopt_1(0i32, 1i32, 10i32);
  cmdio_write_ok(
    (0i32
      | (' ' as i32) << SHIFTsp as libc::c_int
      | '0' as i32 + 220i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
      | '0' as i32 + 220i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
      | '0' as i32 + 220i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
  );
  signal(
    14i32,
    Some(timeout_handler as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  if opts as libc::c_int & OPT_A as libc::c_int == 0 {
    loop {
      let mut cmdval: u32 = cmdio_get_cmd_and_arg();
      if cmdval == const_USER as libc::c_int as libc::c_uint {
        if !anon_opt.is_null()
          && strcmp(
            (*ptr_to_globals).ftp_arg,
            b"anonymous\x00" as *const u8 as *const libc::c_char,
          ) == 0i32
        {
          pw = bb_internal_getpwnam(anon_opt);
          if !pw.is_null() {
            break;
          }
          /* does not even ask for password */
        }
        pw = bb_internal_getpwnam((*ptr_to_globals).ftp_arg);
        cmdio_write_raw(b"331 Specify password\r\n\x00" as *const u8 as *const libc::c_char);
      } else if cmdval == const_PASS as libc::c_int as libc::c_uint {
        if check_password(pw, (*ptr_to_globals).ftp_arg) > 0i32 {
          break;
        }
        cmdio_write_raw(b"530 Login failed\r\n\x00" as *const u8 as *const libc::c_char);
        pw = 0 as *mut passwd
      } else if cmdval == const_QUIT as libc::c_int as libc::c_uint {
        cmdio_write_ok(
          (0i32
            | (' ' as i32) << SHIFTsp as libc::c_int
            | '0' as i32 + 221i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
            | '0' as i32 + 221i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
            | '0' as i32 + 221i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
        );
        return 0i32;
      } else {
        cmdio_write_raw(b"530 Login with USER+PASS\r\n\x00" as *const u8 as *const libc::c_char);
      }
    }
    cmdio_write_ok(
      (0i32
        | (' ' as i32) << SHIFTsp as libc::c_int
        | '0' as i32 + 230i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
        | '0' as i32 + 230i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
        | '0' as i32 + 230i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
    );
  }
  /* Do this after auth, else /etc/passwd is not accessible */
  argv = argv.offset(optind as isize);
  if !(*argv.offset(0)).is_null() {
    let mut basedir: *const libc::c_char = *argv.offset(0);
    if chroot(basedir) == 0i32 {
      basedir = b"/\x00" as *const u8 as *const libc::c_char
    }
    /*
     * If chroot failed, assume that we aren't root,
     * and at least chdir to the specified DIR
     * (older versions were dying with error message).
     * If chroot worked, move current dir to new "/":
     */
    xchdir(basedir);
  }
  if !pw.is_null() {
    change_identity(pw);
  }
  loop
  /* else: -A is in effect */
  /* RFC-959 Section 5.1
   * The following commands and options MUST be supported by every
   * server-FTP and user-FTP, except in cases where the underlying
   * file system or operating system does not allow or support
   * a particular command.
   * Type: ASCII Non-print, IMAGE, LOCAL 8
   * Mode: Stream
   * Structure: File, Record*
   * (Record structure is REQUIRED only for hosts whose file
   *  systems support record structure).
   * Commands:
   * USER, PASS, ACCT, [bbox: ACCT not supported]
   * PORT, PASV,
   * TYPE, MODE, STRU,
   * RETR, STOR, APPE,
   * RNFR, RNTO, DELE,
   * CWD,  CDUP, RMD,  MKD,  PWD,
   * LIST, NLST,
   * SYST, STAT,
   * HELP, NOOP, QUIT.
   */
  /* ACCOUNT (ACCT)
   * "The argument field is a Telnet string identifying the user's account.
   * The command is not necessarily related to the USER command, as some
   * sites may require an account for login and others only for specific
   * access, such as storing files. In the latter case the command may
   * arrive at any time.
   * There are reply codes to differentiate these cases for the automation:
   * when account information is required for login, the response to
   * a successful PASSword command is reply code 332. On the other hand,
   * if account information is NOT required for login, the reply to
   * a successful PASSword command is 230; and if the account information
   * is needed for a command issued later in the dialogue, the server
   * should return a 332 or 532 reply depending on whether it stores
   * (pending receipt of the ACCounT command) or discards the command,
   * respectively."
   */
  {
    let mut cmdval_0: u32 = cmdio_get_cmd_and_arg();
    if cmdval_0 == const_QUIT as libc::c_int as libc::c_uint {
      cmdio_write_ok(
        (0i32
          | (' ' as i32) << SHIFTsp as libc::c_int
          | '0' as i32 + 221i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
          | '0' as i32 + 221i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
          | '0' as i32 + 221i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
      );
      return 0i32;
    } else {
      if cmdval_0 == const_USER as libc::c_int as libc::c_uint {
        /* This would mean "ok, now give me PASS". */
        /*WRITE_OK(FTP_GIVEPWORD);*/
        /* vsftpd can be configured to not require that,
         * and this also saves one roundtrip:
         */
        cmdio_write_ok(
          (0i32
            | (' ' as i32) << SHIFTsp as libc::c_int
            | '0' as i32 + 230i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
            | '0' as i32 + 230i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
            | '0' as i32 + 230i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
        );
      } else if cmdval_0 == const_PASS as libc::c_int as libc::c_uint {
        cmdio_write_ok(
          (0i32
            | (' ' as i32) << SHIFTsp as libc::c_int
            | '0' as i32 + 230i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
            | '0' as i32 + 230i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
            | '0' as i32 + 230i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
        );
      } else if cmdval_0 == const_NOOP as libc::c_int as libc::c_uint {
        cmdio_write_ok(
          (0i32
            | (' ' as i32) << SHIFTsp as libc::c_int
            | '0' as i32 + 200i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
            | '0' as i32 + 200i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
            | '0' as i32 + 200i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
        );
      } else if cmdval_0 == const_TYPE as libc::c_int as libc::c_uint {
        cmdio_write_ok(
          (0i32
            | (' ' as i32) << SHIFTsp as libc::c_int
            | '0' as i32 + 200i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
            | '0' as i32 + 200i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
            | '0' as i32 + 200i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
        );
      } else if cmdval_0 == const_STRU as libc::c_int as libc::c_uint {
        cmdio_write_ok(
          (0i32
            | (' ' as i32) << SHIFTsp as libc::c_int
            | '0' as i32 + 200i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
            | '0' as i32 + 200i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
            | '0' as i32 + 200i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
        );
      } else if cmdval_0 == const_MODE as libc::c_int as libc::c_uint {
        cmdio_write_ok(
          (0i32
            | (' ' as i32) << SHIFTsp as libc::c_int
            | '0' as i32 + 200i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
            | '0' as i32 + 200i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
            | '0' as i32 + 200i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
        );
      } else if cmdval_0 == const_ALLO as libc::c_int as libc::c_uint {
        cmdio_write_ok(
          (0i32
            | (' ' as i32) << SHIFTsp as libc::c_int
            | '0' as i32 + 202i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
            | '0' as i32 + 202i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
            | '0' as i32 + 202i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32,
        );
      } else if cmdval_0 == const_SYST as libc::c_int as libc::c_uint {
        cmdio_write_raw(b"215 UNIX Type: L8\r\n\x00" as *const u8 as *const libc::c_char);
      } else if cmdval_0 == const_PWD as libc::c_int as libc::c_uint
        || cmdval_0 == const_XPWD as libc::c_int as libc::c_uint
      {
        handle_pwd();
      } else if cmdval_0 == const_CWD as libc::c_int as libc::c_uint {
        handle_cwd();
      } else if cmdval_0 == const_CDUP as libc::c_int as libc::c_uint {
        /* cd .. */
        handle_cdup();
      } else if cmdval_0 == const_HELP as libc::c_int as libc::c_uint
        || cmdval_0 == const_FEAT as libc::c_int as libc::c_uint
      {
        handle_feat(if cmdval_0 == const_HELP as libc::c_int as libc::c_uint {
          (0i32
            | '0' as i32 + 214i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
            | '0' as i32 + 214i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
            | '0' as i32 + 214i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32
        } else {
          (0i32
            | '0' as i32 + 211i32 / 1i32 % 10i32 << SHIFT0 as libc::c_int
            | '0' as i32 + 211i32 / 10i32 % 10i32 << SHIFT1 as libc::c_int
            | '0' as i32 + 211i32 / 100i32 % 10i32 << SHIFT2 as libc::c_int) as u32
        });
      } else if cmdval_0 == const_LIST as libc::c_int as libc::c_uint {
        /* HELP is nearly useless, but we can reuse FEAT for it */
        /* lftp uses FEAT */
        /* ls -l */
        handle_list();
      } else if cmdval_0 == const_NLST as libc::c_int as libc::c_uint {
        /* "name list", bare ls */
        handle_nlst();
      } else if cmdval_0 == const_SIZE as libc::c_int as libc::c_uint
        || cmdval_0 == const_MDTM as libc::c_int as libc::c_uint
      {
        handle_size_or_mdtm((cmdval_0 == const_SIZE as libc::c_int as libc::c_uint) as libc::c_int);
      } else if cmdval_0 == const_STAT as libc::c_int as libc::c_uint {
        if (*ptr_to_globals).ftp_arg.is_null() {
          handle_stat();
        } else {
          handle_stat_file();
        }
      } else if cmdval_0 == const_PASV as libc::c_int as libc::c_uint {
        handle_pasv();
      } else if cmdval_0 == const_EPSV as libc::c_int as libc::c_uint {
        handle_epsv();
      } else if cmdval_0 == const_RETR as libc::c_int as libc::c_uint {
        handle_retr();
      } else if cmdval_0 == const_PORT as libc::c_int as libc::c_uint {
        handle_port();
      } else if cmdval_0 == const_REST as libc::c_int as libc::c_uint {
        handle_rest();
      } else {
        let mut current_block_97: u64;
        if opts as libc::c_int & OPT_w as libc::c_int != 0 {
          if cmdval_0 == const_STOR as libc::c_int as libc::c_uint {
            handle_stor();
            current_block_97 = 6665878751423064961;
          } else if cmdval_0 == const_MKD as libc::c_int as libc::c_uint {
            handle_mkd();
            current_block_97 = 6665878751423064961;
          } else if cmdval_0 == const_RMD as libc::c_int as libc::c_uint {
            handle_rmd();
            current_block_97 = 6665878751423064961;
          } else if cmdval_0 == const_DELE as libc::c_int as libc::c_uint {
            handle_dele();
            current_block_97 = 6665878751423064961;
          } else if cmdval_0 == const_RNFR as libc::c_int as libc::c_uint {
            /* SIZE is crucial for wget's download indicator etc */
            /* Mozilla, lftp use MDTM (presumably for caching) */
            /* "rename from" */
            handle_rnfr();
            current_block_97 = 6665878751423064961;
          } else if cmdval_0 == const_RNTO as libc::c_int as libc::c_uint {
            /* "rename to" */
            handle_rnto();
            current_block_97 = 6665878751423064961;
          } else if cmdval_0 == const_APPE as libc::c_int as libc::c_uint {
            handle_appe();
            current_block_97 = 6665878751423064961;
          } else if cmdval_0 == const_STOU as libc::c_int as libc::c_uint {
            /* "store unique" */
            handle_stou();
            current_block_97 = 6665878751423064961;
          } else {
            current_block_97 = 5398443459044258236;
          }
        } else {
          current_block_97 = 5398443459044258236;
        }
        match current_block_97 {
          5398443459044258236 =>
          /* Which unsupported commands were seen in the wild?
           * (doesn't necessarily mean "we must support them")
           * foo 1.2.3: XXXX - comment
           */
          {
            cmdio_write_raw(b"500 Unknown command\r\n\x00" as *const u8 as *const libc::c_char);
          }
          _ => {}
        }
      }
    }
  }
}
