use crate::libbb::appletlib::applet_name;
use crate::libbb::ptr_to_globals::bb_errno;
use crate::libbb::xfuncs_printf::xmalloc;
use crate::librb::size_t;
use crate::librb::smallint;
use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
use libc::close;
use libc::free;
use libc::fstat;
use libc::off_t;
use libc::open;
use libc::openlog;
use libc::ptrdiff_t;
use libc::sprintf;
use libc::ssize_t;
use libc::stat;
use libc::strcasecmp;
use libc::strchr;
use libc::strcmp;
use libc::strcpy;
use libc::strrchr;
use libc::strstr;
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
  static mut optind: libc::c_int;

  #[no_mangle]
  fn recvfrom(
    __fd: libc::c_int,
    __buf: *mut libc::c_void,
    __n: size_t,
    __flags: libc::c_int,
    __addr: __SOCKADDR_ARG,
    __addr_len: *mut socklen_t,
  ) -> ssize_t;

  #[no_mangle]
  fn snprintf(
    _: *mut libc::c_char,
    _: libc::c_ulong,
    _: *const libc::c_char,
    _: ...
  ) -> libc::c_int;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xchroot(path: *const libc::c_char);
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xsocket(domain: libc::c_int, type_0: libc::c_int, protocol: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xbind(sockfd: libc::c_int, my_addr: *mut sockaddr, addrlen: socklen_t);
  #[no_mangle]
  fn xconnect(s: libc::c_int, s_addr: *const sockaddr, addrlen: socklen_t);
  #[no_mangle]
  fn xsendto(
    s: libc::c_int,
    buf: *const libc::c_void,
    len: size_t,
    to: *const sockaddr,
    tolen: socklen_t,
  ) -> ssize_t;
  #[no_mangle]
  fn setsockopt_reuseaddr(fd: libc::c_int);
  #[no_mangle]
  fn bb_lookup_port(
    port: *const libc::c_char,
    protocol: *const libc::c_char,
    default_port: libc::c_uint,
  ) -> libc::c_uint;
  #[no_mangle]
  fn get_sock_lsa(fd: libc::c_int) -> *mut len_and_sockaddr;
  #[no_mangle]
  fn xhost2sockaddr(host: *const libc::c_char, port: libc::c_int) -> *mut len_and_sockaddr;
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
  fn bb_putchar_stderr(ch: libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn full_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn safe_poll(ufds: *mut pollfd, nfds: nfds_t, timeout_ms: libc::c_int) -> libc::c_int;
  /* Non-aborting kind of convertors: bb_strto[u][l]l */
  /* On exit: errno = 0 only if there was non-empty, '\0' terminated value
   * errno = EINVAL if value was not '\0' terminated, but otherwise ok
   *    Return value is still valid, caller should just check whether end[0]
   *    is a valid terminating char for particular case. OTOH, if caller
   *    requires '\0' terminated input, [s]he can just check errno == 0.
   * errno = ERANGE if value had alphanumeric terminating char ("1234abcg").
   * errno = ERANGE if value is out of range, missing, etc.
   * errno = ERANGE if value had minus sign for strtouXX (even "-0" is not ok )
   *    return value is all-ones in this case.
   */
  #[no_mangle]
  fn bb_strtoull(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_ulonglong;
  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  #[no_mangle]
  fn xgetpwnam(name: *const libc::c_char) -> *mut passwd;
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  static mut logmode: smallint;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn change_identity(pw: *const passwd);
  #[no_mangle]
  fn nth_string(strings: *const libc::c_char, n: libc::c_int) -> *const libc::c_char;
  #[no_mangle]
  fn bb_progress_init(p: *mut bb_progress_t, curfile: *const libc::c_char);
  #[no_mangle]
  fn bb_progress_update(
    p: *mut bb_progress_t,
    beg_range: uoff_t,
    transferred: uoff_t,
    totalsize: uoff_t,
  ) -> libc::c_int;

  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];

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
pub type nfds_t = libc::c_ulong;
use crate::librb::uoff_t;
use libc::passwd;
use libc::pollfd;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct len_and_sockaddr {
  pub len: socklen_t,
  pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
  pub sa: sockaddr,
  pub sin: sockaddr_in,
  pub sin6: sockaddr_in6,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const LSA_SIZEOF_SA: C2RustUnnamed_1 = 28;
pub const LSA_LEN_SIZE: C2RustUnnamed_1 = 4;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const LOGMODE_BOTH: C2RustUnnamed_2 = 3;
pub const LOGMODE_SYSLOG: C2RustUnnamed_2 = 2;
pub const LOGMODE_STDIO: C2RustUnnamed_2 = 1;
pub const LOGMODE_NONE: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bb_progress_t {
  pub last_size: libc::c_uint,
  pub last_update_sec: libc::c_uint,
  pub last_change_sec: libc::c_uint,
  pub start_sec: libc::c_uint,
  pub curfile: *const libc::c_char,
}
//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub error_pkt: [u8; 36],
  pub pw: *mut passwd,
  pub block_buf: [libc::c_char; 516],
  pub block_buf_tail: [libc::c_char; 1],
  pub pos: off_t,
  pub size: off_t,
  pub file: *const libc::c_char,
  pub pmt: bb_progress_t,
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const TFTPD_OPT_l: C2RustUnnamed_3 = 2048;
pub const TFTPD_OPT_u: C2RustUnnamed_3 = 1024;
pub const TFTPD_OPT_c: C2RustUnnamed_3 = 512;
pub const TFTPD_OPT_r: C2RustUnnamed_3 = 256;
pub const TFTPD_OPT: C2RustUnnamed_3 = 128;
pub const TFTP_OPT_PUT: C2RustUnnamed_3 = 2;
pub const TFTP_OPT_GET: C2RustUnnamed_3 = 1;
unsafe extern "C" fn tftp_progress_update() {
  bb_progress_update(
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pmt,
    0 as uoff_t,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pos as uoff_t,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).size as uoff_t,
  );
}
unsafe extern "C" fn tftp_progress_init() {
  bb_progress_init(
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pmt,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).file,
  );
  tftp_progress_update();
}
unsafe extern "C" fn tftp_progress_done() {
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .pmt
    .curfile
    .is_null()
  {
    tftp_progress_update();
    bb_putchar_stderr('\n' as i32 as libc::c_char);
    free(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .pmt
        .curfile as *mut libc::c_char as *mut libc::c_void,
    );
    let ref mut fresh0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .pmt
      .curfile;
    *fresh0 = std::ptr::null()
  };
}
unsafe extern "C" fn tftp_blksize_check(
  mut blksize_str: *const libc::c_char,
  mut maxsize: libc::c_int,
) -> libc::c_int {
  /* Check if the blksize is valid:
   * RFC2348 says between 8 and 65464,
   * but our implementation makes it impossible
   * to use blksizes smaller than 22 octets. */
  let mut blksize: libc::c_uint = bb_strtou(blksize_str, 0 as *mut *mut libc::c_char, 10i32);
  if *bb_errno != 0 || blksize < 24i32 as libc::c_uint || blksize > maxsize as libc::c_uint {
    bb_error_msg(
      b"bad blocksize \'%s\'\x00" as *const u8 as *const libc::c_char,
      blksize_str,
    );
    return -1i32;
  }
  return blksize as libc::c_int;
}
unsafe extern "C" fn tftp_get_option(
  mut option: *const libc::c_char,
  mut buf: *mut libc::c_char,
  mut len: libc::c_int,
) -> *mut libc::c_char {
  let mut opt_val: libc::c_int = 0;
  let mut opt_found: libc::c_int = 0;
  let mut k: libc::c_int = 0;
  /* buf points to:
   * "opt_name<NUL>opt_val<NUL>opt_name2<NUL>opt_val2<NUL>..." */
  while len > 0 {
    let mut current_block_13: u64;
    /* Make sure options are terminated correctly */
    k = 0;
    loop {
      if !(k < len) {
        current_block_13 = 11006700562992250127;
        break;
      }
      if *buf.offset(k as isize) as libc::c_int == '\u{0}' as i32 {
        current_block_13 = 8114179180390253173;
        break;
      }
      k += 1
    }
    match current_block_13 {
      11006700562992250127 => return std::ptr::null_mut::<libc::c_char>(),
      _ => {
        if opt_val == 0 {
          /* it's "name" part */
          if strcasecmp(buf, option) == 0 {
            opt_found = 1i32
          }
        } else if opt_found != 0 {
          return buf;
        }
        k += 1;
        buf = buf.offset(k as isize);
        len -= k;
        opt_val ^= 1i32
      }
    }
  }
  return std::ptr::null_mut::<libc::c_char>();
}
unsafe extern "C" fn tftp_protocol(
  mut our_lsa: *mut len_and_sockaddr,
  mut peer_lsa: *mut len_and_sockaddr,
  mut local_file: *const libc::c_char,
  mut remote_file: *const libc::c_char,
  mut want_transfer_size: libc::c_int,
  mut blksize: libc::c_int,
) -> libc::c_int {
  let mut sz: libc::c_int = 0;
  let mut res: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut current_block: u64;
  let mut pfd: [pollfd; 1] = [pollfd {
    fd: 0,
    events: 0,
    revents: 0,
  }; 1];
  let mut len: libc::c_int = 0;
  let mut send_len: libc::c_int = 0;
  let mut expect_OACK: smallint = 0 as smallint;
  let mut finished: smallint = 0 as smallint;
  let mut opcode: u16 = 0;
  let mut block_nr: u16 = 0;
  let mut recv_blk: u16 = 0;
  let mut open_mode: libc::c_int = 0;
  let mut local_fd: libc::c_int = 0;
  let mut retries: libc::c_int = 0;
  let mut waittime_ms: libc::c_int = 0;
  let mut io_bufsize: libc::c_int = blksize + 4i32;
  let mut cp: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  /* Can't use RESERVE_CONFIG_BUFFER here since the allocation
   * size varies meaning BUFFERS_GO_ON_STACK would fail.
   *
   * We must keep the transmit and receive buffers separate
   * in case we rcv a garbage pkt - we need to rexmit the last pkt.
   */
  let mut xbuf: *mut libc::c_char = xmalloc(io_bufsize as size_t) as *mut libc::c_char;
  let mut rbuf: *mut libc::c_char = xmalloc(io_bufsize as size_t) as *mut libc::c_char;
  pfd[0].fd = xsocket(
    (*peer_lsa).u.sa.sa_family as libc::c_int,
    SOCK_DGRAM as libc::c_int,
    0,
  );
  setsockopt_reuseaddr(pfd[0].fd);
  if 1i32 == 0 || !our_lsa.is_null() {
    /* tftpd */
    /* Create a socket which is:
     * 1. bound to IP:port peer sent 1st datagram to,
     * 2. connected to peer's IP:port
     * This way we will answer from the IP:port peer
     * expects, will not get any other packets on
     * the socket, and also plain read/write will work. */
    xbind(pfd[0].fd, &mut (*our_lsa).u.sa, (*our_lsa).len);
    xconnect(pfd[0].fd, &mut (*peer_lsa).u.sa, (*peer_lsa).len);
    /* Is there an error already? Send pkt and bail out */
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).error_pkt[3] as libc::c_int != 0
      || *((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .error_pkt
        .as_mut_ptr()
        .offset(4) as *mut libc::c_char)
        .offset(0) as libc::c_int
        != 0
    {
      current_block = 15308042354214156955;
    } else {
      if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .pw
        .is_null()
      {
        change_identity((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pw);
        /* initgroups, setgid, setuid */
      }
      current_block = 15976848397966268834;
    }
  } else {
    current_block = 15976848397966268834;
  }
  match current_block {
    15976848397966268834 => {
      /* Prepare open mode */
      if option_mask32 & TFTP_OPT_PUT as libc::c_int as libc::c_uint != 0 {
        open_mode = 0
      } else {
        open_mode = 0o1i32 | 0o1000i32 | 0o100i32;
        if option_mask32 & (TFTPD_OPT as libc::c_int + TFTPD_OPT_c as libc::c_int) as libc::c_uint
          == TFTPD_OPT as libc::c_int as libc::c_uint
        {
          /* tftpd without -c */
          open_mode = 0o1i32 | 0o1000i32
        }
      }
      /* Examples of network traffic.
       * Note two cases when ACKs with block# of 0 are sent.
       *
       * Download without options:
       * tftp -> "\0\1FILENAME\0octet\0"
       *         "\0\3\0\1FILEDATA..." <- tftpd
       * tftp -> "\0\4\0\1"
       * ...
       * Download with option of blksize 16384:
       * tftp -> "\0\1FILENAME\0octet\0blksize\00016384\0"
       *         "\0\6blksize\00016384\0" <- tftpd
       * tftp -> "\0\4\0\0"
       *         "\0\3\0\1FILEDATA..." <- tftpd
       * tftp -> "\0\4\0\1"
       * ...
       * Upload without options:
       * tftp -> "\0\2FILENAME\0octet\0"
       *         "\0\4\0\0" <- tftpd
       * tftp -> "\0\3\0\1FILEDATA..."
       *         "\0\4\0\1" <- tftpd
       * ...
       * Upload with option of blksize 16384:
       * tftp -> "\0\2FILENAME\0octet\0blksize\00016384\0"
       *         "\0\6blksize\00016384\0" <- tftpd
       * tftp -> "\0\3\0\1FILEDATA..."
       *         "\0\4\0\1" <- tftpd
       * ...
       */
      block_nr = 1i32 as u16; /* tftp */
      cp = xbuf.offset(2);
      if 1i32 == 0 || !our_lsa.is_null() {
        /* tftpd */
        /* Open file (must be after changing user) */
        local_fd = open(local_file, open_mode, 0o666i32);
        if local_fd < 0 {
          (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).error_pkt[3] = 1i32 as u8;
          strcpy(
            (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
              .error_pkt
              .as_mut_ptr()
              .offset(4) as *mut libc::c_char,
            b"can\'t open file\x00" as *const u8 as *const libc::c_char,
          );
          current_block = 15308042354214156955;
        } else if blksize != 512i32 || want_transfer_size != 0 {
          /* gcc 4.3.1 would NOT optimize it out as it should! */
          /* Create and send OACK packet. */
          /* For the download case, block_nr is still 1 -
           * we expect 1st ACK from peer to be for (block_nr-1),
           * that is, for "block 0" which is our OACK pkt */
          opcode = 6i32 as u16;
          current_block = 1694039017486245274;
        } else {
          if option_mask32 & TFTP_OPT_GET as libc::c_int as libc::c_uint != 0 {
            /* It's upload and we don't send OACK.
             * We must ACK 1st packet (with filename)
             * as if it is "block 0" */
            block_nr = 0 as u16
          }
          /* Using mostly goto's - continue/break will be less clear
           * in where we actually jump to */
          current_block = 13484060386966298149; /* end of "while (1)" */
        }
      } else {
        /* Open file (must be after changing user) */
        local_fd = if option_mask32 & TFTP_OPT_GET as libc::c_int as libc::c_uint != 0 {
          1i32
        } else {
          0
        };
        if *local_file.offset(0) as libc::c_int != '-' as i32
          || *local_file.offset(1) as libc::c_int != 0
        {
          local_fd = xopen(local_file, open_mode)
        }
        /* Removing #if, or using if() statement instead of #if may lead to
         * "warning: null argument where non-null required": */
        /* tftp */
        /* We can't (and don't really need to) bind the socket:
         * we don't know from which local IP datagrams will be sent,
         * but kernel will pick the same IP every time (unless routing
         * table is changed), thus peer will see dgrams consistently
         * coming from the same IP.
         * We would like to connect the socket, but since peer's
         * UDP code can be less perfect than ours, _peer's_ IP:port
         * in replies may differ from IP:port we used to send
         * our first packet. We can connect() only when we get
         * first reply. */
        /* build opcode */
        opcode = 2i32 as u16;
        if option_mask32 & TFTP_OPT_GET as libc::c_int as libc::c_uint != 0 {
          opcode = 1i32 as u16
        }
        /* add filename and mode */
        /* fill in packet if the filename fits into xbuf */
        len = strlen(remote_file).wrapping_add(1i32 as libc::c_ulong) as libc::c_int;
        if ((2i32 + len) as libc::c_ulong)
          .wrapping_add(::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
          >= io_bufsize as libc::c_ulong
        {
          bb_simple_error_msg(
            b"remote filename is too long\x00" as *const u8 as *const libc::c_char,
          );
          current_block = 4894395567674443800;
        } else {
          strcpy(cp, remote_file);
          cp = cp.offset(len as isize);
          /* add "mode" part of the packet */
          strcpy(cp, b"octet\x00" as *const u8 as *const libc::c_char);
          cp = cp.offset(::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as isize);
          if blksize == 512i32 && want_transfer_size == 0 {
            current_block = 1653815207292225316;
          } else if ((&mut *xbuf.offset((io_bufsize - 1i32) as isize) as *mut libc::c_char)
            .wrapping_offset_from(cp) as libc::c_long as libc::c_ulong)
            < (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong).wrapping_add(
              (::std::mem::size_of::<off_t>() as libc::c_ulong).wrapping_mul(3i32 as libc::c_ulong),
            )
          {
            bb_simple_error_msg(
              b"remote filename is too long\x00" as *const u8 as *const libc::c_char,
            );
            current_block = 4894395567674443800;
          } else {
            expect_OACK = 1i32 as smallint;
            current_block = 1694039017486245274;
          }
        }
      }
      match current_block {
        15308042354214156955 => {}
        _ => {
          match current_block {
            1694039017486245274 =>
            /* Need to add option to pkt */
            /* ENABLE_TFTP */
            {
              if blksize != 512i32 {
                /* add "blksize", <nul>, blksize, <nul> */
                strcpy(cp, b"blksize\x00" as *const u8 as *const libc::c_char);
                cp =
                  cp.offset(::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as isize);
                cp = cp.offset(
                  (snprintf(
                    cp,
                    6i32 as libc::c_ulong,
                    b"%d\x00" as *const u8 as *const libc::c_char,
                    blksize,
                  ) + 1i32) as isize,
                )
              }
              if want_transfer_size != 0 {
                /* add "tsize", <nul>, size, <nul> (see RFC2349) */
                /* if tftp and downloading, we send "0" (since we opened local_fd with O_TRUNC)
                 * and this makes server to send "tsize" option with the size */
                /* if tftp and uploading, we send file size (maybe dont, to not confuse old servers???) */
                /* if tftpd and downloading, we are answering to client's request */
                /* if tftpd and uploading: !want_transfer_size, this code is not executed */
                let mut st: stat = std::mem::zeroed();
                strcpy(cp, b"tsize\x00" as *const u8 as *const libc::c_char);
                cp =
                  cp.offset(::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as isize);
                st.st_size = 0 as off_t;
                fstat(local_fd, &mut st);
                cp = cp.offset(
                  (sprintf(
                    cp,
                    b"%lu\x00" as *const u8 as *const libc::c_char,
                    st.st_size,
                  ) + 1i32) as isize,
                );
                /* Save for progress bar. If 0 (tftp downloading),
                 * we look at server's reply later */
                (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).size = st.st_size;
                if !remote_file.is_null() && st.st_size != 0 {
                  tftp_progress_init();
                }
              }
              /* First packet is built, so skip packet generation */
              current_block = 1653815207292225316;
            }
            _ => {}
          }
          's_871: loop {
            match current_block {
              4894395567674443800 =>
              /*bb_perror_msg("poll"); - done in safe_poll */
              {
                return (finished as libc::c_int == 0) as libc::c_int
              }
              13484060386966298149 => {
                /* returns 1 on failure */
                /* Build ACK or DATA */
                cp = xbuf.offset(2);
                *(cp as *mut u16) = {
                  let mut __v: libc::c_ushort = 0;
                  let mut __x: libc::c_ushort = block_nr;
                  if false {
                    __v = (__x as libc::c_int >> 8i32 & 0xffi32
                      | (__x as libc::c_int & 0xffi32) << 8i32)
                      as libc::c_ushort
                  } else {
                    let fresh1 = &mut __v;
                    let fresh2;
                    let fresh3 = __x;
                    asm!("rorw $$8, ${0:w}" :
                                                      "=r" (fresh2) : "0"
                                                      (c2rust_asm_casts::AsmCast::cast_in(fresh1, fresh3))
                                                      : "cc");
                    c2rust_asm_casts::AsmCast::cast_out(fresh1, fresh3, fresh2);
                  }
                  __v
                };
                cp = cp.offset(2);
                block_nr = block_nr.wrapping_add(1);
                opcode = 4i32 as u16;
                if !(option_mask32 & TFTP_OPT_PUT as libc::c_int as libc::c_uint != 0) {
                  current_block = 1653815207292225316;
                  continue;
                }
                opcode = 3i32 as u16;
                len =
                  full_read(local_fd, cp as *mut libc::c_void, blksize as size_t) as libc::c_int;
                if len < 0 {
                  current_block = 12367016239538502269;
                  break;
                }
                if len != blksize {
                  finished = 1i32 as smallint
                }
                cp = cp.offset(len as isize);
                let ref mut fresh4 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pos;
                *fresh4 += len as libc::c_long;
                current_block = 1653815207292225316;
                /* why recv_again? - rfc1123 says:
                 * "The sender (i.e., the side originating the DATA packets)
                 *  must never resend the current DATA packet on receipt
                 *  of a duplicate ACK".
                 * DATA pkts are resent ONLY on timeout.
                 * Thus "goto send_again" will ba a bad mistake above.
                 * See:
                 * http://en.wikipedia.org/wiki/Sorcerer's_Apprentice_Syndrome
                 */
              }
              _ => {
                *(xbuf as *mut u16) = {
                  let mut __v: libc::c_ushort = 0; /* fill in opcode part */
                  let mut __x: libc::c_ushort = opcode;
                  if false {
                    __v = (__x as libc::c_int >> 8i32 & 0xffi32
                      | (__x as libc::c_int & 0xffi32) << 8i32)
                      as libc::c_ushort
                  } else {
                    let fresh5 = &mut __v;
                    let fresh6;
                    let fresh7 = __x;
                    asm!("rorw $$8, ${0:w}" :
                                                      "=r" (fresh6) : "0"
                                                      (c2rust_asm_casts::AsmCast::cast_in(fresh5, fresh7))
                                                      : "cc");
                    c2rust_asm_casts::AsmCast::cast_out(fresh5, fresh7, fresh6);
                  }
                  __v
                };
                send_len = cp.wrapping_offset_from(xbuf) as libc::c_long as libc::c_int;
                /* Send packet */
                /* NB: send_len value is preserved in code below
                 * for potential resend */
                retries = 12i32; /* re-initialize */
                waittime_ms = 100i32; /* resend last sent pkt */
                'c_9340: loop {
                  xsendto(
                    pfd[0].fd,
                    xbuf as *const libc::c_void,
                    send_len as size_t,
                    &mut (*peer_lsa).u.sa,
                    (*peer_lsa).len,
                  );
                  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                    .pmt
                    .curfile
                    .is_null()
                  {
                    tftp_progress_update();
                  }
                  /* Was it final ACK? then exit */
                  if finished as libc::c_int != 0 && opcode as libc::c_int == 4i32 {
                    current_block = 4894395567674443800;
                    continue 's_871;
                  }
                  loop
                  /* Awww... recv'd packet is not recognized! */
                  /* Receive packet */
                  /*pfd[0].fd = socket_fd;*/
                  {
                    pfd[0].events = 0x1i32 as libc::c_short;
                    match safe_poll(pfd.as_mut_ptr(), 1i32 as nfds_t, waittime_ms) {
                      0 => {
                        retries -= 1;
                        if retries == 0 {
                          current_block = 6406431739208918833;
                          break;
                        } else {
                          current_block = 13824533195664196414;
                          break;
                        }
                      }
                      1 => {
                        if our_lsa.is_null() {
                          /* tftp (not tftpd!) receiving 1st packet */
                          our_lsa =
                            -1i32 as ptrdiff_t as *mut libc::c_void as *mut len_and_sockaddr; /* not NULL */
                          len = recvfrom(
                            pfd[0].fd,
                            rbuf as *mut libc::c_void,
                            io_bufsize as size_t,
                            0,
                            __SOCKADDR_ARG {
                              __sockaddr__: &mut (*peer_lsa).u.sa as *mut sockaddr,
                            },
                            &mut (*peer_lsa).len,
                          ) as libc::c_int;
                          /* Our first dgram went to port 69
                           * but reply may come from different one.
                           * Remember and use this new port (and IP) */
                          if len >= 0 {
                            xconnect(pfd[0].fd, &mut (*peer_lsa).u.sa, (*peer_lsa).len);
                          }
                        } else {
                          /* tftpd, or not the very first packet:
                           * socket is connect()ed, can just read from it. */
                          /* Don't full_read()!
                           * This is not TCP, one read == one pkt! */
                          len =
                            safe_read(pfd[0].fd, rbuf as *mut libc::c_void, io_bufsize as size_t)
                              as libc::c_int
                        }
                        if len < 0 {
                          current_block = 12367016239538502269;
                          break 's_871;
                        }
                        if !(len < 4i32) {
                          /* Process recv'ed packet */
                          opcode = {
                            let mut __v: libc::c_ushort = 0;
                            let mut __x: libc::c_ushort = *(rbuf as *mut u16).offset(0);
                            if false {
                              __v = (__x as libc::c_int >> 8i32 & 0xffi32
                                | (__x as libc::c_int & 0xffi32) << 8i32)
                                as libc::c_ushort
                            } else {
                              let fresh8 = &mut __v;
                              let fresh9;
                              let fresh10 = __x;
                              asm!("rorw $$8, ${0:w}"
                                                                              :
                                                                              "=r"
                                                                              (fresh9)
                                                                              :
                                                                              "0"
                                                                              (c2rust_asm_casts::AsmCast::cast_in(fresh8, fresh10))
                                                                              :
                                                                              "cc");
                              c2rust_asm_casts::AsmCast::cast_out(fresh8, fresh10, fresh9);
                            }
                            __v
                          };
                          recv_blk = {
                            let mut __v: libc::c_ushort = 0;
                            let mut __x: libc::c_ushort = *(rbuf as *mut u16).offset(1);
                            if false {
                              __v = (__x as libc::c_int >> 8i32 & 0xffi32
                                | (__x as libc::c_int & 0xffi32) << 8i32)
                                as libc::c_ushort
                            } else {
                              let fresh11 = &mut __v;
                              let fresh12;
                              let fresh13 = __x;
                              asm!("rorw $$8, ${0:w}"
                                                                              :
                                                                              "=r"
                                                                              (fresh12)
                                                                              :
                                                                              "0"
                                                                              (c2rust_asm_casts::AsmCast::cast_in(fresh11, fresh13))
                                                                              :
                                                                              "cc");
                              c2rust_asm_casts::AsmCast::cast_out(fresh11, fresh13, fresh12);
                            }
                            __v
                          };
                          if opcode as libc::c_int == 5i32 {
                            static mut errcode_str: [libc::c_char; 121] = [
                              0, 102, 105, 108, 101, 32, 110, 111, 116, 32, 102, 111, 117, 110,
                              100, 0, 97, 99, 99, 101, 115, 115, 32, 118, 105, 111, 108, 97, 116,
                              105, 111, 110, 0, 100, 105, 115, 107, 32, 102, 117, 108, 108, 0, 98,
                              97, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 0, 117, 110,
                              107, 110, 111, 119, 110, 32, 116, 114, 97, 110, 115, 102, 101, 114,
                              32, 105, 100, 0, 102, 105, 108, 101, 32, 97, 108, 114, 101, 97, 100,
                              121, 32, 101, 120, 105, 115, 116, 115, 0, 110, 111, 32, 115, 117, 99,
                              104, 32, 117, 115, 101, 114, 0, 98, 97, 100, 32, 111, 112, 116, 105,
                              111, 110, 0,
                            ];
                            let mut msg: *const libc::c_char =
                              b"\x00" as *const u8 as *const libc::c_char;
                            if len > 4i32 && *rbuf.offset(4) as libc::c_int != '\u{0}' as i32 {
                              msg = &mut *rbuf.offset(4) as *mut libc::c_char;
                              *rbuf.offset((io_bufsize - 1i32) as isize) =
                                '\u{0}' as i32 as libc::c_char
                            /* paranoia */
                            } else if recv_blk as libc::c_int <= 8i32 {
                              msg = nth_string(errcode_str.as_ptr(), recv_blk as libc::c_int)
                            }
                            bb_error_msg(
                              b"server error: (%u) %s\x00" as *const u8 as *const libc::c_char,
                              recv_blk as libc::c_int,
                              msg,
                            );
                            current_block = 4894395567674443800;
                            continue 's_871;
                          } else {
                            if expect_OACK != 0 {
                              expect_OACK = 0 as smallint;
                              if opcode as libc::c_int == 6i32 {
                                /* server seems to support options */
                                res = std::ptr::null_mut::<libc::c_char>();
                                res = tftp_get_option(
                                  b"blksize\x00" as *const u8 as *const libc::c_char,
                                  &mut *rbuf.offset(2),
                                  len - 2i32,
                                );
                                if !res.is_null() {
                                  current_block = 17392506108461345148;
                                  break 'c_9340;
                                } else {
                                  current_block = 10763371041174037105;
                                  break 'c_9340;
                                }
                              } else {
                                /* rfc2347:
                                 * "An option not acknowledged by the server
                                 * must be ignored by the client and server
                                 * as if it were never requested." */
                                if blksize != 512i32 {
                                  bb_simple_error_msg(
                                    b"falling back to blocksize 512\x00" as *const u8
                                      as *const libc::c_char,
                                  );
                                }
                                blksize = 512i32;
                                io_bufsize = 512i32 + 4i32
                              }
                            }
                            /* block_nr is already advanced to next block# we expect
                             * to get / block# we are about to send next time */
                            if option_mask32 & TFTP_OPT_GET as libc::c_int as libc::c_uint != 0
                              && opcode as libc::c_int == 3i32
                            {
                              if recv_blk as libc::c_int == block_nr as libc::c_int {
                                sz = full_write(
                                  local_fd,
                                  &mut *rbuf.offset(4) as *mut libc::c_char as *const libc::c_void,
                                  (len - 4i32) as size_t,
                                ) as libc::c_int;
                                if sz != len - 4i32 {
                                  current_block = 5393508166988163885;
                                  break 'c_9340;
                                } else {
                                  current_block = 9270770154621591809;
                                  break 'c_9340;
                                }
                                /* send ACK */
                              }
                              /* Disabled to cope with servers with Sorcerer's Apprentice Syndrome */
                            }
                            if !(option_mask32 & TFTP_OPT_PUT as libc::c_int as libc::c_uint != 0
                              && opcode as libc::c_int == 4i32)
                            {
                              continue;
                            }
                            /* did peer ACK our last DATA pkt? */
                            if !(recv_blk as libc::c_int
                              == (block_nr as libc::c_int - 1i32) as u16 as libc::c_int)
                            {
                              continue;
                            }
                            if finished != 0 {
                              current_block = 4894395567674443800;
                              continue 's_871;
                            /* send next block */
                            } else {
                              current_block = 13484060386966298149;
                              continue 's_871;
                            }
                          }
                        }
                      }
                      _ => {
                        current_block = 4894395567674443800;
                        continue 's_871;
                      }
                    }
                  }
                  match current_block {
                    13824533195664196414 => {
                      /* exponential backoff with limit */
                      waittime_ms += waittime_ms / 2i32;
                      if waittime_ms > 2000i32 {
                        waittime_ms = 2000i32
                      }
                    }
                    _ => {
                      tftp_progress_done();
                      bb_simple_error_msg(b"timeout\x00" as *const u8 as *const libc::c_char);
                      current_block = 4894395567674443800;
                      continue 's_871;
                      /* no err packet sent */
                    }
                  }
                }
                match current_block {
                  5393508166988163885 => {
                    strcpy(
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                        .error_pkt
                        .as_mut_ptr()
                        .offset(4) as *mut libc::c_char,
                      b"write error\x00" as *const u8 as *const libc::c_char,
                    );
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).error_pkt[3] = 3i32 as u8;
                    current_block = 15308042354214156955;
                    break;
                  }
                  17392506108461345148 => {
                    blksize = tftp_blksize_check(res, blksize);
                    if blksize < 0 {
                      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).error_pkt[3] = 8i32 as u8;
                      current_block = 15308042354214156955;
                      break;
                    } else {
                      io_bufsize = blksize + 4i32
                    }
                  }
                  9270770154621591809 => {
                    if sz != blksize {
                      finished = 1i32 as smallint
                    }
                    let ref mut fresh14 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pos;
                    *fresh14 += sz as libc::c_long;
                    current_block = 13484060386966298149;
                    continue;
                  }
                  _ => {}
                }
                if !remote_file.is_null()
                  && (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).size == 0
                {
                  /* if we don't know it yet */
                  res = tftp_get_option(
                    b"tsize\x00" as *const u8 as *const libc::c_char,
                    &mut *rbuf.offset(2),
                    len - 2i32,
                  );
                  if !res.is_null() {
                    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).size =
                      bb_strtoull(res, 0 as *mut *mut libc::c_char, 10i32) as off_t;
                    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).size != 0 {
                      tftp_progress_init();
                    }
                  }
                }
                if option_mask32 & TFTP_OPT_GET as libc::c_int as libc::c_uint != 0 {
                  /* We'll send ACK for OACK,
                   * such ACK has "block no" of 0 */
                  block_nr = 0 as u16
                }
                current_block = 13484060386966298149;
              }
            }
          }
          match current_block {
            15308042354214156955 => {}
            _ => {
              strcpy(
                (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
                  .error_pkt
                  .as_mut_ptr()
                  .offset(4) as *mut libc::c_char,
                b"read error\x00" as *const u8 as *const libc::c_char,
              );
            }
          }
        }
      }
    }
    _ => {}
  }
  if *((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .error_pkt
    .as_mut_ptr()
    .offset(4) as *mut libc::c_char)
    .offset(0)
    != 0
  {
    bb_simple_error_msg(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .error_pkt
        .as_mut_ptr()
        .offset(4) as *mut libc::c_char,
    );
  }
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).error_pkt[1] = 5i32 as u8;
  xsendto(
    pfd[0].fd,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .error_pkt
      .as_mut_ptr() as *const libc::c_void,
    ((4i32 + 1i32) as libc::c_ulong).wrapping_add(strlen(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .error_pkt
        .as_mut_ptr()
        .offset(4) as *mut libc::c_char,
    )),
    &mut (*peer_lsa).u.sa,
    (*peer_lsa).len,
  );
  return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn tftp_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut peer_lsa: *mut len_and_sockaddr = std::ptr::null_mut();
  let mut local_file: *const libc::c_char = std::ptr::null();
  let mut remote_file: *const libc::c_char = std::ptr::null();
  let mut blksize_str: *const libc::c_char = b"512\x00" as *const u8 as *const libc::c_char;
  let mut blksize: libc::c_int = 0;
  let mut result: libc::c_int = 0;
  let mut port: libc::c_int = 0;
  let mut opt: libc::c_int = 0;
  /* As of 2019, common tftp client in Linux distros
   * is one maintained by H. Peter Anvin:
   * I've seen "tftp-hpa 5.2" version.
   * Make the following command work:
   *  "tftp HOST [PORT] -m binary -c get/put FILE"
   * by mangling it into "....... -g/-p -r FILE"
   * and accepting and ignoring -m STR option.
   */
  let mut i: libc::c_uint = 1i32 as libc::c_uint;
  while !(*argv.offset(i as isize)).is_null() {
    /* Accept not only -c, but also
     * -lc, -cl, -llcclcllcc etc:
     * "-l Literal mode (do not recognize HOST:FILE)"
     * since we do not recognize that syntax anyway,
     * might as well allow the option.
     */
    if *(*argv.offset(i as isize)).offset(0) as libc::c_int == '-' as i32
      && !strchr(*argv.offset(i as isize), 'c' as i32).is_null()
    {
      /*&& argv[i][1+strspn(argv[i]+1, "lc")] == '\0'*/
      i = i.wrapping_add(1);
      if (*argv.offset(i as isize)).is_null() {
        break;
      }
      if strcmp(
        *argv.offset(i as isize),
        b"get\x00" as *const u8 as *const libc::c_char,
      ) == 0
      {
        let ref mut fresh15 = *argv.offset(i.wrapping_sub(1i32 as libc::c_uint) as isize);
        *fresh15 = b"-g\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        let ref mut fresh16 = *argv.offset(i as isize);
        *fresh16 = b"-r\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        break;
      } else if strcmp(
        *argv.offset(i as isize),
        b"put\x00" as *const u8 as *const libc::c_char,
      ) == 0
      {
        let ref mut fresh17 = *argv.offset(i.wrapping_sub(1i32 as libc::c_uint) as isize);
        *fresh17 = b"-p\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        let ref mut fresh18 = *argv.offset(i as isize);
        *fresh18 = b"-r\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        break;
      }
    }
    i = i.wrapping_add(1)
  }
  opt = getopt32(
    argv,
    b"^gpl:r:b:m:\x00g:p:g--p:p--g:\x00" as *const u8 as *const libc::c_char,
    &mut local_file as *mut *const libc::c_char,
    &mut remote_file as *mut *const libc::c_char,
    &mut blksize_str as *mut *const libc::c_char,
    0 as *mut libc::c_void,
  ) as libc::c_int;
  argv = argv.offset(optind as isize);
  /* Check if the blksize is valid:
   * RFC2348 says between 8 and 65464 */
  blksize = tftp_blksize_check(blksize_str, 65564i32);
  if blksize < 0 {
    //bb_error_msg("bad block size");
    return 1i32;
  }
  if !remote_file.is_null() {
    if local_file.is_null() {
      let mut slash: *const libc::c_char = strrchr(remote_file, '/' as i32);
      local_file = if !slash.is_null() {
        slash.offset(1)
      } else {
        remote_file
      }
    }
  } else {
    remote_file = local_file
  }
  /* Error if filename or host is not known */
  if remote_file.is_null() || (*argv.offset(0)).is_null() {
    bb_show_usage();
  }
  port = bb_lookup_port(
    *argv.offset(1),
    b"udp\x00" as *const u8 as *const libc::c_char,
    69i32 as libc::c_uint,
  ) as libc::c_int;
  peer_lsa = xhost2sockaddr(*argv.offset(0), port);
  let ref mut fresh19 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).file;
  *fresh19 = remote_file;
  result = tftp_protocol(
    0 as *mut len_and_sockaddr,
    peer_lsa,
    local_file,
    remote_file,
    1i32,
    blksize,
  );
  tftp_progress_done();
  if result != 0
    && (*local_file.offset(0) as libc::c_int != '-' as i32
      || *local_file.offset(1) as libc::c_int != 0)
    && opt & TFTP_OPT_GET as libc::c_int != 0
  {
    unlink(local_file);
  }
  return result;
}
/* ENABLE_TFTP */
#[no_mangle]
pub unsafe extern "C" fn tftpd_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut our_lsa: *mut len_and_sockaddr = std::ptr::null_mut();
  let mut peer_lsa: *mut len_and_sockaddr = std::ptr::null_mut();
  let mut mode: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut user_opt: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut local_file: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  local_file = local_file;
  let mut error_msg: *const libc::c_char = std::ptr::null();
  let mut opt: libc::c_int = 0;
  let mut result: libc::c_int = 0;
  let mut opcode: libc::c_int = 0;
  let mut blksize: libc::c_int = 512i32;
  let mut want_transfer_size: libc::c_int = 0;
  our_lsa = get_sock_lsa(0i32);
  if our_lsa.is_null() {
    /* This is confusing:
     *bb_error_msg_and_die("stdin is not a socket");
     * Better: */
    bb_show_usage();
    /* Help text says that tftpd must be used as inetd service,
     * which is by far the most usual cause of get_sock_lsa
     * failure */
  }
  peer_lsa =
    xzalloc((LSA_LEN_SIZE as libc::c_int as libc::c_uint).wrapping_add((*our_lsa).len) as size_t)
      as *mut len_and_sockaddr;
  (*peer_lsa).len = (*our_lsa).len;
  /* Shifting to not collide with TFTP_OPTs */
  option_mask32 = TFTPD_OPT as libc::c_int as libc::c_uint
    | getopt32(
      argv,
      b"rcu:l\x00" as *const u8 as *const libc::c_char,
      &mut user_opt as *mut *mut libc::c_char,
    ) << 8i32;
  opt = option_mask32 as libc::c_int;
  argv = argv.offset(optind as isize);
  if opt & TFTPD_OPT_l as libc::c_int != 0 {
    openlog(applet_name, 0x1i32, 3i32 << 3i32);
    logmode = LOGMODE_SYSLOG as libc::c_int as smallint
  }
  if opt & TFTPD_OPT_u as libc::c_int != 0 {
    /* Must be before xchroot */
    let ref mut fresh20 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).pw;
    *fresh20 = xgetpwnam(user_opt)
  }
  if !(*argv.offset(0)).is_null() {
    xchroot(*argv.offset(0));
  }
  result = recv_from_to(
    0,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .block_buf
      .as_mut_ptr() as *mut libc::c_void,
    (::std::mem::size_of::<[libc::c_char; 516]>() as libc::c_ulong)
      .wrapping_add(1i32 as libc::c_ulong),
    0,
    &mut (*peer_lsa).u.sa,
    &mut (*our_lsa).u.sa,
    (*our_lsa).len,
  ) as libc::c_int;
  error_msg = b"malformed packet\x00" as *const u8 as *const libc::c_char;
  opcode = ({
    let mut __v: libc::c_ushort = 0;
    let mut __x: libc::c_ushort = *((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .block_buf
      .as_mut_ptr() as *mut u16);
    if false {
      __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
        as libc::c_ushort
    } else {
      let fresh21 = &mut __v;
      let fresh22;
      let fresh23 = __x;
      asm!("rorw $$8, ${0:w}" : "=r" (fresh22) : "0"
                      (c2rust_asm_casts::AsmCast::cast_in(fresh21, fresh23)) :
                      "cc");
      c2rust_asm_casts::AsmCast::cast_out(fresh21, fresh23, fresh22);
    }
    __v
  }) as libc::c_int;
  if result < 4i32
    || result as libc::c_ulong > ::std::mem::size_of::<[libc::c_char; 516]>() as libc::c_ulong
    || opcode != 1i32 && opcode != 2i32
  {
    current_block = 16343923103195422028;
  } else {
    /* Some HP PA-RISC firmware always sends fixed 516-byte requests,
     * with trailing garbage.
     * Support that by not requiring NUL to be the last byte (see above).
     * To make strXYZ() ops safe, force NUL termination:
     */
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).block_buf_tail[0] =
      '\u{0}' as i32 as libc::c_char;
    local_file = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .block_buf
      .as_mut_ptr()
      .offset(2);
    if *local_file.offset(0) as libc::c_int == '.' as i32
      || !strstr(local_file, b"/.\x00" as *const u8 as *const libc::c_char).is_null()
    {
      error_msg = b"dot in file name\x00" as *const u8 as *const libc::c_char;
      current_block = 16343923103195422028;
    } else {
      mode = local_file.offset(strlen(local_file) as isize).offset(1);
      /* RFC 1350 says mode string is case independent */
      if mode
        >= (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .block_buf
          .as_mut_ptr()
          .offset(result as isize)
        || strcasecmp(mode, b"octet\x00" as *const u8 as *const libc::c_char) != 0
      {
        error_msg = b"mode is not \'octet\'\x00" as *const u8 as *const libc::c_char;
        current_block = 16343923103195422028;
      } else {
        let mut res: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
        let mut opt_str: *mut libc::c_char =
          mode.offset(::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as isize);
        let mut opt_len: libc::c_int = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .block_buf
          .as_mut_ptr()
          .offset(result as isize)
          .wrapping_offset_from(opt_str) as libc::c_long
          as libc::c_int;
        if opt_len > 0 {
          res = tftp_get_option(
            b"blksize\x00" as *const u8 as *const libc::c_char,
            opt_str,
            opt_len,
          );
          if !res.is_null() {
            blksize = tftp_blksize_check(res, 65564i32);
            if blksize < 0 {
              (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).error_pkt[3] = 8i32 as u8;
              /* will just send error pkt */
              current_block = 13887286082274607397;
            } else {
              current_block = 1622411330066726685;
            }
          } else {
            current_block = 1622411330066726685;
          }
          match current_block {
            13887286082274607397 => {}
            _ => {
              if opcode != 2i32
                && !tftp_get_option(
                  b"tsize\x00" as *const u8 as *const libc::c_char,
                  opt_str,
                  opt_len,
                )
                .is_null()
              {
                want_transfer_size = 1i32
              }
              current_block = 1924505913685386279;
            }
          }
        } else {
          current_block = 1924505913685386279;
        }
        match current_block {
          13887286082274607397 => {}
          _ => {
            if 1i32 == 0 || opcode == 2i32 {
              if opt & TFTPD_OPT_r as libc::c_int != 0 {
                /* This would mean "disk full" - not true */
                /*G_error_pkt_reason = ERR_WRITE;*/
                error_msg = b"write error\x00" as *const u8 as *const libc::c_char;
                current_block = 16343923103195422028;
              } else {
                option_mask32 |= TFTP_OPT_GET as libc::c_int as libc::c_uint;
                current_block = 13887286082274607397;
              }
            /* will receive file's data */
            } else {
              option_mask32 |= TFTP_OPT_PUT as libc::c_int as libc::c_uint;
              current_block = 13887286082274607397;
              /* will send file's data */
            }
          }
        }
      }
    }
  }
  match current_block {
    16343923103195422028 => {
      strcpy(
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .error_pkt
          .as_mut_ptr()
          .offset(4) as *mut libc::c_char,
        error_msg,
      );
    }
    _ => {}
  }
  /* NB: if G_error_pkt_str or G_error_pkt_reason is set up,
   * tftp_protocol() just sends one error pkt and returns */
  close(0i32); /* close old, possibly wildcard socket */
  /* tftp_protocol() will create new one, bound to particular local IP */
  result = tftp_protocol(
    our_lsa,
    peer_lsa,
    local_file,
    0 as *const libc::c_char,
    want_transfer_size,
    blksize,
  );
  return result;
}
/* ENABLE_FEATURE_TFTP_GET || ENABLE_FEATURE_TFTP_PUT */
/* ENABLE_TFTPD */
