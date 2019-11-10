use crate::librb::size_t;
use libc;
use libc::close;
use libc::free;
use libc::getpid;
use libc::gid_t;
use libc::pid_t;
use libc::putenv;
use libc::ssize_t;
use libc::uid_t;
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
  fn vfork() -> libc::c_int;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn getsockname(__fd: libc::c_int, __addr: __SOCKADDR_ARG, __len: *mut socklen_t) -> libc::c_int;
  #[no_mangle]
  fn send(
    __fd: libc::c_int,
    __buf: *const libc::c_void,
    __n: size_t,
    __flags: libc::c_int,
  ) -> ssize_t;
  #[no_mangle]
  fn getsockopt(
    __fd: libc::c_int,
    __level: libc::c_int,
    __optname: libc::c_int,
    __optval: *mut libc::c_void,
    __optlen: *mut socklen_t,
  ) -> libc::c_int;
  #[no_mangle]
  fn accept(__fd: libc::c_int, __addr: __SOCKADDR_ARG, __addr_len: *mut socklen_t) -> libc::c_int;
  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn close_on_exec_on(fd: libc::c_int);
  #[no_mangle]
  fn xdup2(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn bb_signals(sigs: libc::c_int, f: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>);
  #[no_mangle]
  fn wait_for_any_sig();
  #[no_mangle]
  fn kill_myself_with_sig(sig: libc::c_int) -> !;
  #[no_mangle]
  fn sig_block(sig: libc::c_int);
  #[no_mangle]
  fn sig_unblock(sig: libc::c_int);
  #[no_mangle]
  fn xsetgid(gid: gid_t);
  #[no_mangle]
  fn xsetuid(uid: uid_t);
  #[no_mangle]
  fn bb_unsetenv_and_free(key: *mut libc::c_char);
  #[no_mangle]
  fn xsocket(domain: libc::c_int, type_0: libc::c_int, protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xbind(sockfd: libc::c_int, my_addr: *mut sockaddr, addrlen: socklen_t);
  #[no_mangle]
  fn xlisten(s: libc::c_int, backlog: libc::c_int);
  #[no_mangle]
  fn xconnect(s: libc::c_int, s_addr: *const sockaddr, addrlen: socklen_t);
  #[no_mangle]
  fn setsockopt_reuseaddr(fd: libc::c_int);
  #[no_mangle]
  fn bb_lookup_port(
    port: *const libc::c_char,
    protocol: *const libc::c_char,
    default_port: libc::c_uint,
  ) -> libc::c_uint;
  #[no_mangle]
  fn xhost2sockaddr(host: *const libc::c_char, port: libc::c_int) -> *mut len_and_sockaddr;
  #[no_mangle]
  fn xmalloc_sockaddr2host_noport(sa: *const sockaddr) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc_sockaddr2dotted(sa: *const sockaddr) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc_sockaddr2dotted_noport(sa: *const sockaddr) -> *mut libc::c_char;
  #[no_mangle]
  fn socket_want_pktinfo(fd: libc::c_int);
  #[no_mangle]
  fn recv_from_to(
    fd: libc::c_int,
    buf: *mut libc::c_void,
    len: size_t,
    flags: libc::c_int,
    from: *mut sockaddr,
    to: *mut sockaddr,
    sa_size: socklen_t,
  ) -> ssize_t;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn utoa(n: libc::c_uint) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  #[no_mangle]
  fn xget_uidgid(_: *mut bb_uidgid_t, _: *const libc::c_char);
  #[no_mangle]
  fn BB_EXECVP_or_die(argv: *mut *mut libc::c_char) -> !;
  #[no_mangle]
  fn wait_any_nohang(wstat: *mut libc::c_int) -> pid_t;
  #[no_mangle]
  fn bb_sanitize_stdio();
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
  #[no_mangle]
  fn ipsvd_perhost_init(_: libc::c_uint) -> *mut hcc;
  /* Returns number of already opened connects to this ips, including this one.
   * ip should be a malloc'ed ptr.
   * If return value is <= maxconn, ip is inserted into the table
   * and pointer to table entry if stored in *hccpp
   * (useful for storing pid later).
   * Else ip is NOT inserted (you must take care of it - free() etc) */
  #[no_mangle]
  fn ipsvd_perhost_add(
    cc: *mut hcc,
    ip: *mut libc::c_char,
    maxconn: libc::c_uint,
    hccpp: *mut *mut hcc,
  ) -> libc::c_uint;
  /* Finds and frees element with pid */
  #[no_mangle]
  fn ipsvd_perhost_remove(cc: *mut hcc, pid: libc::c_int);
}

pub type __socklen_t = libc::c_uint;
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
pub type C2RustUnnamed = libc::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed = 67108864;
pub const MSG_BATCH: C2RustUnnamed = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed = 65536;
pub const MSG_MORE: C2RustUnnamed = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed = 8192;
pub const MSG_RST: C2RustUnnamed = 4096;
pub const MSG_CONFIRM: C2RustUnnamed = 2048;
pub const MSG_SYN: C2RustUnnamed = 1024;
pub const MSG_FIN: C2RustUnnamed = 512;
pub const MSG_WAITALL: C2RustUnnamed = 256;
pub const MSG_EOR: C2RustUnnamed = 128;
pub const MSG_DONTWAIT: C2RustUnnamed = 64;
pub const MSG_TRUNC: C2RustUnnamed = 32;
pub const MSG_PROXY: C2RustUnnamed = 16;
pub const MSG_CTRUNC: C2RustUnnamed = 8;
pub const MSG_TRYHARD: C2RustUnnamed = 4;
pub const MSG_DONTROUTE: C2RustUnnamed = 4;
pub const MSG_PEEK: C2RustUnnamed = 2;
pub const MSG_OOB: C2RustUnnamed = 1;
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
  pub __in6_u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
use crate::librb::signal::__sighandler_t;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const BB_FATAL_SIGS: C2RustUnnamed_1 = 117503054;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct len_and_sockaddr {
  pub len: socklen_t,
  pub u: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
  pub sa: sockaddr,
  pub sin: sockaddr_in,
  pub sin6: sockaddr_in6,
}
use crate::librb::bb_uidgid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub verbose: libc::c_uint,
  pub max_per_host: libc::c_uint,
  pub cur_per_host: libc::c_uint,
  pub cnum: libc::c_uint,
  pub cmax: libc::c_uint,
  pub cc: *mut hcc,
  pub env_cur: *mut *mut libc::c_char,
  pub env_var: [*mut libc::c_char; 1],
}
/* Based on ipsvd utilities written by Gerrit Pape <pape@smarden.org>
 * which are released into public domain by the author.
 * Homepage: http://smarden.sunsite.dk/ipsvd/
 *
 * Copyright (C) 2007 Denys Vlasenko.
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hcc {
  pub ip: *mut libc::c_char,
  pub pid: libc::c_int,
}
/* Must match getopt32 in main! */
pub type C2RustUnnamed_3 = libc::c_uint;
pub const OPT_K: C2RustUnnamed_3 = 65536;
pub const OPT_Z: C2RustUnnamed_3 = 32768;
/* from here: sslsvd only */
pub const OPT_slash: C2RustUnnamed_3 = 16384;
pub const OPT_U: C2RustUnnamed_3 = 8192;
pub const OPT_V: C2RustUnnamed_3 = 4096;
pub const OPT_v: C2RustUnnamed_3 = 2048;
pub const OPT_t: C2RustUnnamed_3 = 1024;
pub const OPT_p: C2RustUnnamed_3 = 512;
pub const OPT_h: C2RustUnnamed_3 = 256;
pub const OPT_b: C2RustUnnamed_3 = 128;
pub const OPT_E: C2RustUnnamed_3 = 64;
pub const OPT_l: C2RustUnnamed_3 = 32;
pub const OPT_u: C2RustUnnamed_3 = 16;
pub const OPT_x: C2RustUnnamed_3 = 8;
pub const OPT_i: C2RustUnnamed_3 = 4;
pub const OPT_C: C2RustUnnamed_3 = 2;
pub const OPT_c: C2RustUnnamed_3 = 1;
/* We have to be careful about leaking memory in repeated setenv's */
unsafe extern "C" fn xsetenv_plain(mut n: *const libc::c_char, mut v: *const libc::c_char) {
  let mut var: *mut libc::c_char =
    xasprintf(b"%s=%s\x00" as *const u8 as *const libc::c_char, n, v);
  let ref mut fresh0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).env_cur;
  let fresh1 = *fresh0;
  *fresh0 = (*fresh0).offset(1);
  *fresh1 = var;
  putenv(var);
}
unsafe extern "C" fn xsetenv_proto(
  mut proto: *const libc::c_char,
  mut n: *const libc::c_char,
  mut v: *const libc::c_char,
) {
  let mut var: *mut libc::c_char = xasprintf(
    b"%s%s=%s\x00" as *const u8 as *const libc::c_char,
    proto,
    n,
    v,
  );
  let ref mut fresh2 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).env_cur;
  let fresh3 = *fresh2;
  *fresh2 = (*fresh2).offset(1);
  *fresh3 = var;
  putenv(var);
}
unsafe extern "C" fn undo_xsetenv() {
  let ref mut fresh4 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).env_cur;
  *fresh4 = &mut *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .env_var
    .as_mut_ptr()
    .offset(0) as *mut *mut libc::c_char;
  let mut pp: *mut *mut libc::c_char = *fresh4;
  while !(*pp).is_null() {
    let mut var: *mut libc::c_char = *pp;
    bb_unsetenv_and_free(var);
    let fresh5 = pp;
    pp = pp.offset(1);
    *fresh5 = 0 as *mut libc::c_char
  }
}
unsafe extern "C" fn sig_term_handler(mut sig: libc::c_int) {
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose != 0 {
    bb_error_msg(
      b"got signal %u, exit\x00" as *const u8 as *const libc::c_char,
      sig,
    );
  }
  kill_myself_with_sig(sig);
}
/* Little bloated, but tries to give accurate info how child exited.
 * Makes easier to spot segfaulting children etc... */
unsafe extern "C" fn print_waitstat(mut pid: libc::c_uint, mut wstat: libc::c_int) {
  let mut e: libc::c_uint = 0i32 as libc::c_uint;
  let mut cause: *const libc::c_char = b"?exit\x00" as *const u8 as *const libc::c_char;
  if wstat & 0x7fi32 == 0i32 {
    cause = cause.offset(1);
    e = ((wstat & 0xff00i32) >> 8i32) as libc::c_uint
  } else if ((wstat & 0x7fi32) + 1i32) as libc::c_schar as libc::c_int >> 1i32 > 0i32 {
    cause = b"signal\x00" as *const u8 as *const libc::c_char;
    e = (wstat & 0x7fi32) as libc::c_uint
  }
  bb_error_msg(
    b"end %d %s %d\x00" as *const u8 as *const libc::c_char,
    pid,
    cause,
    e,
  );
}
unsafe extern "C" fn connection_status() {
  /* "only 1 client max" desn't need this */
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cmax > 1i32 as libc::c_uint {
    bb_error_msg(
      b"status %u/%u\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cnum,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cmax,
    ); /* gcc */
  }; /* for compiler */
}
unsafe extern "C" fn sig_child_handler(mut _sig: libc::c_int) {
  let mut wstat: libc::c_int = 0; /* for compiler */
  let mut pid: pid_t = 0;
  loop {
    pid = wait_any_nohang(&mut wstat);
    if !(pid > 0i32) {
      break;
    }
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_per_host != 0 {
      ipsvd_perhost_remove((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cc, pid);
    }
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cnum != 0 {
      let ref mut fresh6 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cnum;
      *fresh6 = (*fresh6).wrapping_sub(1)
    }
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose != 0 {
      print_waitstat(pid as libc::c_uint, wstat);
    }
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose != 0 {
    connection_status();
  };
}
#[no_mangle]
pub unsafe extern "C" fn tcpudpsvd_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut str_C: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut str_t: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut user: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut hccp: *mut hcc = 0 as *mut hcc;
  let mut instructs: *const libc::c_char = 0 as *const libc::c_char;
  let mut msg_per_host: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut len_per_host: libc::c_uint = 0;
  len_per_host = len_per_host;
  let mut ugid: bb_uidgid_t = bb_uidgid_t { uid: 0, gid: 0 };
  let mut tcp: bool = false;
  let mut local_port: u16 = 0;
  let mut preset_local_hostname: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut remote_hostname: *mut libc::c_char = 0 as *mut libc::c_char;
  remote_hostname = remote_hostname;
  let mut remote_addr: *mut libc::c_char = 0 as *mut libc::c_char;
  remote_addr = remote_addr;
  let mut lsa: *mut len_and_sockaddr = 0 as *mut len_and_sockaddr;
  let mut local: len_and_sockaddr = len_and_sockaddr {
    len: 0,
    u: C2RustUnnamed_2 {
      sa: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  let mut remote: len_and_sockaddr = len_and_sockaddr {
    len: 0,
    u: C2RustUnnamed_2 {
      sa: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
      },
    },
  };
  let mut sa_len: socklen_t = 0;
  let mut pid: libc::c_int = 0;
  let mut sock: libc::c_int = 0;
  let mut conn: libc::c_int = 0;
  let mut backlog: libc::c_uint = 20i32 as libc::c_uint;
  let mut opts: libc::c_uint = 0;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cmax = 30i32 as libc::c_uint;
  let ref mut fresh7 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).env_cur;
  *fresh7 = &mut *(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .env_var
    .as_mut_ptr()
    .offset(0) as *mut *mut libc::c_char;
  tcp = *applet_name.offset(0) as libc::c_int == 't' as i32;
  /* "+": stop on first non-option */
  opts = getopt32(
    argv,
    b"^+c:+C:i:x:u:l:Eb:+hpt:v\x00-3:i--i:ph:vv\x00" as *const u8 as *const libc::c_char,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cmax as *mut libc::c_uint,
    &mut str_C as *mut *mut libc::c_char,
    &mut instructs as *mut *const libc::c_char,
    &mut instructs as *mut *const libc::c_char,
    &mut user as *mut *mut libc::c_char,
    &mut preset_local_hostname as *mut *mut libc::c_char,
    &mut backlog as *mut libc::c_uint,
    &mut str_t as *mut *mut libc::c_char,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose as *mut libc::c_uint,
  );
  if opts & OPT_C as libc::c_int as libc::c_uint != 0 {
    /* -C n[:message] */
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_per_host =
      bb_strtou(str_C, &mut str_C, 10i32);
    if *str_C.offset(0) != 0 {
      if *str_C.offset(0) as libc::c_int != ':' as i32 {
        bb_show_usage();
      }
      msg_per_host = str_C.offset(1);
      len_per_host = strlen(msg_per_host) as libc::c_uint
    }
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_per_host
    > (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cmax
  {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_per_host =
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cmax
  }
  if opts & OPT_u as libc::c_int as libc::c_uint != 0 {
    xget_uidgid(&mut ugid, user);
  }
  argv = argv.offset(optind as isize);
  if *(*argv.offset(0)).offset(0) == 0
    || *(*argv.offset(0)).offset(0) as libc::c_int == '0' as i32
      && *(*argv.offset(0)).offset(1) == 0
  {
    let ref mut fresh8 = *argv.offset(0);
    *fresh8 = b"0.0.0.0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
  }
  /* Per-IP flood protection is not thought-out for UDP */
  if !tcp {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_per_host = 0i32 as libc::c_uint
  } /* fd# 0,1,2 must be opened */
  bb_sanitize_stdio(); /* I presume sockaddr len stays the same */
  sig_block(17i32); /* udp: needed for recv_from_to to work: */
  signal(
    17i32,
    Some(sig_child_handler as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  bb_signals(
    BB_FATAL_SIGS as libc::c_int,
    Some(sig_term_handler as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  signal(
    13i32,
    ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1i32 as libc::intptr_t),
  );
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_per_host != 0 {
    let ref mut fresh9 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cc;
    *fresh9 = ipsvd_perhost_init((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cmax)
  }
  local_port = bb_lookup_port(
    *argv.offset(1),
    if tcp as libc::c_int != 0 {
      b"tcp\x00" as *const u8 as *const libc::c_char
    } else {
      b"udp\x00" as *const u8 as *const libc::c_char
    },
    0i32 as libc::c_uint,
  ) as u16;
  lsa = xhost2sockaddr(*argv.offset(0), local_port as libc::c_int);
  argv = argv.offset(2);
  sock = xsocket(
    (*lsa).u.sa.sa_family as libc::c_int,
    if tcp as libc::c_int != 0 {
      SOCK_STREAM as libc::c_int
    } else {
      SOCK_DGRAM as libc::c_int
    },
    0i32,
  );
  setsockopt_reuseaddr(sock);
  sa_len = (*lsa).len;
  xbind(sock, &mut (*lsa).u.sa, sa_len);
  if tcp {
    xlisten(sock, backlog as libc::c_int);
    close_on_exec_on(sock);
  } else {
    socket_want_pktinfo(sock);
  }
  /* ndelay_off(sock); - it is the default I think? */
  if opts & OPT_u as libc::c_int as libc::c_uint != 0 {
    /* drop permissions */
    xsetgid(ugid.gid);
    xsetuid(ugid.uid);
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose != 0 {
    let mut addr: *mut libc::c_char = xmalloc_sockaddr2dotted(&mut (*lsa).u.sa);
    if opts & OPT_u as libc::c_int as libc::c_uint != 0 {
      bb_error_msg(
        b"listening on %s, starting, uid %u, gid %u\x00" as *const u8 as *const libc::c_char,
        addr,
        ugid.uid,
        ugid.gid,
      );
    } else {
      bb_error_msg(
        b"listening on %s, starting\x00" as *const u8 as *const libc::c_char,
        addr,
      );
    }
    free(addr as *mut libc::c_void);
  }
  loop
  /* Main accept() loop */
  {
    hccp = 0 as *mut hcc;
    loop {
      close(0i32);
      /* It's important to close(0) _before_ wait loop:
       * fd#0 can be a shared connection fd.
       * If kept open by us, peer can't detect PROG closing it.
       */
      while (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cnum
        >= (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cmax
      {
        wait_for_any_sig(); /* expecting SIGCHLD */
      }
      loop {
        sig_unblock(17i32);
        remote.len = sa_len;
        local.len = remote.len;
        if tcp {
          /* Accept a connection to fd #0 */
          conn = accept(
            sock,
            __SOCKADDR_ARG {
              __sockaddr__: &mut remote.u.sa as *mut sockaddr,
            },
            &mut remote.len,
          )
        } else {
          /* In case recv_from_to won't be able to recover local addr.
           * Also sets port - recv_from_to is unable to do it. */
          local = *lsa;
          conn = recv_from_to(
            sock,
            0 as *mut libc::c_void,
            0i32 as size_t,
            MSG_PEEK as libc::c_int,
            &mut remote.u.sa,
            &mut local.u.sa,
            sa_len,
          ) as libc::c_int
        }
        sig_block(17i32);
        if !(conn < 0i32) {
          break;
        }
        if *bb_errno != 4i32 {
          bb_simple_perror_msg(if tcp as libc::c_int != 0 {
            b"accept\x00" as *const u8 as *const libc::c_char
          } else {
            b"recv\x00" as *const u8 as *const libc::c_char
          });
        }
      }
      xmove_fd(if tcp as libc::c_int != 0 { conn } else { sock }, 0i32);
      if !((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_per_host != 0) {
        break;
      }
      /* Drop connection immediately if cur_per_host > max_per_host
       * (minimizing load under SYN flood) */
      remote_addr = xmalloc_sockaddr2dotted_noport(&mut remote.u.sa);
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cur_per_host = ipsvd_perhost_add(
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cc,
        remote_addr,
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_per_host,
        &mut hccp,
      );
      if !((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cur_per_host
        > (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_per_host)
      {
        break;
      }
      /* NB: remote_addr is not leaked, it is stored in conn table */
      /* ipsvd_perhost_add detected that max is exceeded
       * (and did not store ip in connection table) */
      free(remote_addr as *mut libc::c_void);
      if !msg_per_host.is_null() {
        /* don't block or test for errors */
        send(
          0i32,
          msg_per_host as *const libc::c_void,
          len_per_host as size_t,
          MSG_DONTWAIT as libc::c_int,
        );
      }
    }
    if !tcp {
      /* Voodoo magic: making udp sockets each receive its own
       * packets is not trivial, and I still not sure
       * I do it 100% right.
       * 1) we have to do it before fork()
       * 2) order is important - is it right now? */
      /* Open new non-connected UDP socket for further clients... */
      sock = xsocket(
        (*lsa).u.sa.sa_family as libc::c_int,
        SOCK_DGRAM as libc::c_int,
        0i32,
      );
      setsockopt_reuseaddr(sock);
      /* Doesn't work:
       * we cannot replace fd #0 - we will lose pending packet
       * which is already buffered for us! And we cannot use fd #1
       * instead - it will "intercept" all following packets, but child
       * does not expect data coming *from fd #1*! */
      xconnect(0i32, &mut remote.u.sa, sa_len);
      xbind(sock, &mut (*lsa).u.sa, sa_len);
      socket_want_pktinfo(sock);
    }
    pid = vfork();
    if pid == -1i32 {
      bb_simple_perror_msg(b"vfork\x00" as *const u8 as *const libc::c_char);
    } else {
      if !(pid != 0i32) {
        break;
      }
      /* Make plain write/send work for old socket by supplying default
       * destination address. This also restricts incoming packets
       * to ones coming from this remote IP. */
      /* hole? at this point we have no wildcard udp socket...
       * can this cause clients to get "port unreachable" icmp?
       * Yup, time window is very small, but it exists (is it?) */
      /* ..."open new socket", continued */
      /* Parent */
      let ref mut fresh10 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cnum;
      *fresh10 = (*fresh10).wrapping_add(1);
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose != 0 {
        connection_status();
      }
      if !hccp.is_null() {
        (*hccp).pid = pid
      }
      /* clean up changes done by vforked child */
      undo_xsetenv();
    }
  }
  /* Child: prepare env, log, and exec prog */
  /* vfork alert! every xmalloc in this block should be freed! */
  let mut local_hostname: *mut libc::c_char = 0 as *mut libc::c_char; /* for compiler */
  local_hostname = local_hostname;
  let mut local_addr: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut free_me0: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut free_me1: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut free_me2: *mut libc::c_char = 0 as *mut libc::c_char;
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose != 0
    || opts & OPT_E as libc::c_int as libc::c_uint == 0
  {
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_per_host == 0 {
      /* remote_addr is not yet known */
      remote_addr = xmalloc_sockaddr2dotted(&mut remote.u.sa);
      free_me0 = remote_addr
    }
    if opts & OPT_h as libc::c_int as libc::c_uint != 0 {
      remote_hostname = xmalloc_sockaddr2host_noport(&mut remote.u.sa);
      free_me1 = remote_hostname;
      if remote_hostname.is_null() {
        bb_error_msg(
          b"can\'t look up hostname for %s\x00" as *const u8 as *const libc::c_char,
          remote_addr,
        );
        remote_hostname = remote_addr
      }
    }
    /* Find out local IP peer connected to.
     * Errors ignored (I'm not paranoid enough to imagine kernel
     * which doesn't know local IP). */
    if tcp {
      getsockname(
        0i32,
        __SOCKADDR_ARG {
          __sockaddr__: &mut local.u.sa as *mut sockaddr,
        },
        &mut local.len,
      );
    }
    /* else: for UDP it is done earlier by parent */
    local_addr = xmalloc_sockaddr2dotted(&mut local.u.sa);
    if opts & OPT_h as libc::c_int as libc::c_uint != 0 {
      local_hostname = preset_local_hostname;
      if local_hostname.is_null() {
        local_hostname = xmalloc_sockaddr2host_noport(&mut local.u.sa);
        free_me2 = local_hostname;
        if local_hostname.is_null() {
          bb_error_msg_and_die(
            b"can\'t look up hostname for %s\x00" as *const u8 as *const libc::c_char,
            local_addr,
          );
        }
      }
      /* else: local_hostname is not NULL, but is NOT malloced! */
    }
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose != 0 {
    pid = getpid();
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_per_host != 0 {
      bb_error_msg(
        b"concurrency %s %u/%u\x00" as *const u8 as *const libc::c_char,
        remote_addr,
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cur_per_host,
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).max_per_host,
      );
    }
    bb_error_msg(
      if opts & OPT_h as libc::c_int as libc::c_uint != 0 {
        b"start %u %s-%s (%s-%s)\x00" as *const u8 as *const libc::c_char
      } else {
        b"start %u %s-%s\x00" as *const u8 as *const libc::c_char
      },
      pid,
      local_addr,
      remote_addr,
      local_hostname,
      remote_hostname,
    );
  }
  if opts & OPT_E as libc::c_int as libc::c_uint == 0 {
    /* setup ucspi env */
    let mut proto: *const libc::c_char = if tcp as libc::c_int != 0 {
      b"TCP\x00" as *const u8 as *const libc::c_char
    } else {
      b"UDP\x00" as *const u8 as *const libc::c_char
    };
    /* Extract "original" destination addr:port
     * from Linux firewall. Useful when you redirect
     * an outbond connection to local handler, and it needs
     * to know where it originally tried to connect */
    if tcp as libc::c_int != 0
      && getsockopt(
        0i32,
        0i32,
        80i32,
        &mut local.u.sa as *mut sockaddr as *mut libc::c_void,
        &mut local.len,
      ) == 0i32
    {
      let mut addr_0: *mut libc::c_char = xmalloc_sockaddr2dotted(&mut local.u.sa);
      xsetenv_plain(
        b"TCPORIGDSTADDR\x00" as *const u8 as *const libc::c_char,
        addr_0,
      );
      free(addr_0 as *mut libc::c_void);
    }
    xsetenv_plain(b"PROTO\x00" as *const u8 as *const libc::c_char, proto);
    xsetenv_proto(
      proto,
      b"LOCALADDR\x00" as *const u8 as *const libc::c_char,
      local_addr,
    );
    xsetenv_proto(
      proto,
      b"REMOTEADDR\x00" as *const u8 as *const libc::c_char,
      remote_addr,
    );
    if opts & OPT_h as libc::c_int as libc::c_uint != 0 {
      xsetenv_proto(
        proto,
        b"LOCALHOST\x00" as *const u8 as *const libc::c_char,
        local_hostname,
      );
      xsetenv_proto(
        proto,
        b"REMOTEHOST\x00" as *const u8 as *const libc::c_char,
        remote_hostname,
      );
    }
    //compat? xsetenv_proto(proto, "REMOTEINFO", "");
    /* additional */
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cur_per_host > 0i32 as libc::c_uint {
      /* can not be true for udp */
      xsetenv_plain(
        b"TCPCONCURRENCY\x00" as *const u8 as *const libc::c_char,
        utoa((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).cur_per_host),
      ); /* this one was SIG_IGNed */
    }
  }
  free(local_addr as *mut libc::c_void);
  free(free_me0 as *mut libc::c_void);
  free(free_me1 as *mut libc::c_void);
  free(free_me2 as *mut libc::c_void);
  xdup2(0i32, 1i32);
  signal(13i32, None);
  /* Non-ignored signals revert to SIG_DFL on exec anyway */
  /*signal(SIGCHLD, SIG_DFL);*/
  sig_unblock(17i32);
  BB_EXECVP_or_die(argv);
}
/*
tcpsvd [-hpEvv] [-c n] [-C n:msg] [-b n] [-u user] [-l name]
  [-i dir|-x cdb] [ -t sec] host port prog

tcpsvd creates a TCP/IP socket, binds it to the address host:port,
and listens on the socket for incoming connections.

On each incoming connection, tcpsvd conditionally runs a program,
with standard input reading from the socket, and standard output
writing to the socket, to handle this connection. tcpsvd keeps
listening on the socket for new connections, and can handle
multiple connections simultaneously.

tcpsvd optionally checks for special instructions depending
on the IP address or hostname of the client that initiated
the connection, see ipsvd-instruct(5).

host
    host either is a hostname, or a dotted-decimal IP address,
    or 0. If host is 0, tcpsvd accepts connections to any local
    IP address.
    * busybox accepts IPv6 addresses and host:port pairs too
      In this case second parameter is ignored
port
    tcpsvd accepts connections to host:port. port may be a name
    from /etc/services or a number.
prog
    prog consists of one or more arguments. For each connection,
    tcpsvd normally runs prog, with file descriptor 0 reading from
    the network, and file descriptor 1 writing to the network.
    By default it also sets up TCP-related environment variables,
    see tcp-environ(5)
-i dir
    read instructions for handling new connections from the instructions
    directory dir. See ipsvd-instruct(5) for details.
    * ignored by busyboxed version
-x cdb
    read instructions for handling new connections from the constant database
    cdb. The constant database normally is created from an instructions
    directory by running ipsvd-cdb(8).
    * ignored by busyboxed version
-t sec
    timeout. This option only takes effect if the -i option is given.
    While checking the instructions directory, check the time of last access
    of the file that matches the clients address or hostname if any, discard
    and remove the file if it wasn't accessed within the last sec seconds;
    tcpsvd does not discard or remove a file if the user's write permission
    is not set, for those files the timeout is disabled. Default is 0,
    which means that the timeout is disabled.
    * ignored by busyboxed version
-l name
    local hostname. Do not look up the local hostname in DNS, but use name
    as hostname. This option must be set if tcpsvd listens on port 53
    to avoid loops.
-u user[:group]
    drop permissions. Switch user ID to user's UID, and group ID to user's
    primary GID after creating and binding to the socket. If user is followed
    by a colon and a group name, the group ID is switched to the GID of group
    instead. All supplementary groups are removed.
-c n
    concurrency. Handle up to n connections simultaneously. Default is 30.
    If there are n connections active, tcpsvd defers acceptance of a new
    connection until an active connection is closed.
-C n[:msg]
    per host concurrency. Allow only up to n connections from the same IP
    address simultaneously. If there are n active connections from one IP
    address, new incoming connections from this IP address are closed
    immediately. If n is followed by :msg, the message msg is written
    to the client if possible, before closing the connection. By default
    msg is empty. See ipsvd-instruct(5) for supported escape sequences in msg.

    For each accepted connection, the current per host concurrency is
    available through the environment variable TCPCONCURRENCY. n and msg
    can be overwritten by ipsvd(7) instructions, see ipsvd-instruct(5).
    By default tcpsvd doesn't keep track of connections.
-h
    Look up the client's hostname in DNS.
-p
    paranoid. After looking up the client's hostname in DNS, look up the IP
    addresses in DNS for that hostname, and forget about the hostname
    if none of the addresses match the client's IP address. You should
    set this option if you use hostname based instructions. The -p option
    implies the -h option.
    * ignored by busyboxed version
-b n
    backlog. Allow a backlog of approximately n TCP SYNs. On some systems n
    is silently limited. Default is 20.
-E
    no special environment. Do not set up TCP-related environment variables.
-v
    verbose. Print verbose messages to standard output.
-vv
    more verbose. Print more verbose messages to standard output.
    * no difference between -v and -vv in busyboxed version
*/
