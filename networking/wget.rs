use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
extern "C" {
  pub type tls_handshake_data;
  #[no_mangle]
  fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strtoul(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_ulong;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn exit(_: libc::c_int) -> !;
  #[no_mangle]
  fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  static ptr_to_globals: *mut globals;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn bb_progress_update(
    p: *mut bb_progress_t,
    beg_range: uoff_t,
    transferred: uoff_t,
    totalsize: uoff_t,
  ) -> libc::c_int;
  #[no_mangle]
  fn bb_progress_init(p: *mut bb_progress_t, curfile: *const libc::c_char);
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn socketpair(
    __domain: libc::c_int,
    __type: libc::c_int,
    __protocol: libc::c_int,
    __fds: *mut libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
  #[no_mangle]
  static mut stderr: *mut FILE;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fflush(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fgets_unlocked(
    __s: *mut libc::c_char,
    __n: libc::c_int,
    __stream: *mut FILE,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t, __stream: *mut FILE) -> size_t;
  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn clearerr(__stream: *mut FILE);
  #[no_mangle]
  fn ferror_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fileno_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn lseek(__fd: libc::c_int, __offset: __off64_t, __whence: libc::c_int) -> __off64_t;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn strnlen(__string: *const libc::c_char, __maxlen: size_t) -> size_t;
  #[no_mangle]
  fn inet_pton(
    __af: libc::c_int,
    __cp: *const libc::c_char,
    __buf: *mut libc::c_void,
  ) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn skip_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn skip_non_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_get_last_path_component_nostrip(path: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn ndelay_on(fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn ndelay_off(fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xdup2(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xlseek(fd: libc::c_int, offset: off_t, whence: libc::c_int) -> off_t;
  #[no_mangle]
  fn xconnect_stream(lsa: *const len_and_sockaddr) -> libc::c_int;
  #[no_mangle]
  fn xhost2sockaddr(host: *const libc::c_char, port: libc::c_int) -> *mut len_and_sockaddr;
  #[no_mangle]
  fn set_nport(sa: *mut sockaddr, port: libc::c_uint);
  #[no_mangle]
  fn xmalloc_sockaddr2dotted(sa: *const sockaddr) -> *mut libc::c_char;
  #[no_mangle]
  fn tls_handshake(tls: *mut tls_state_t, sni: *const libc::c_char);
  #[no_mangle]
  fn tls_run_copy_loop(tls: *mut tls_state_t, flags: libc::c_uint);
  #[no_mangle]
  fn parse_pasv_epsv(buf: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn overlapping_strcpy(dst: *mut libc::c_char, src: *const libc::c_char);
  #[no_mangle]
  fn bb_putchar_stderr(ch: libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn xwrite(fd: libc::c_int, buf: *const libc::c_void, count: size_t);
  #[no_mangle]
  fn xclose(fd: libc::c_int);
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn safe_poll(ufds: *mut pollfd, nfds: nfds_t, timeout_ms: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn str_tolower(str: *mut libc::c_char) -> *mut libc::c_char;
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
  fn xatoi_positive(numstr: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xfork() -> pid_t;
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> u32;
  #[no_mangle]
  fn llist_pop(elm: *mut *mut llist_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xfunc_die() -> !;
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_die_memory_exhausted() -> !;
  #[no_mangle]
  fn concat_path_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn index_in_strings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn percent_decode_in_place(str: *mut libc::c_char, strict: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  static bb_uuenc_tbl_base64: [libc::c_char; 0];
  #[no_mangle]
  fn bb_uuencode(
    store: *mut libc::c_char,
    s: *const libc::c_void,
    length: libc::c_int,
    tbl: *const libc::c_char,
  );
  #[no_mangle]
  fn execvp(__file: *const libc::c_char, __argv: *const *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn ftruncate(__fd: libc::c_int, __length: __off64_t) -> libc::c_int;
  #[no_mangle]
  fn vfork() -> libc::c_int;
}

use crate::librb::__off64_t;

pub type __socklen_t = libc::c_uint;
use crate::librb::smallint;




pub type smalluint = libc::c_uchar;
use crate::librb::off_t;
use crate::librb::pid_t;
use crate::librb::size_t;
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
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
  pub sa_family: sa_family_t,
  pub sa_data: [libc::c_char; 14],
}
pub type C2RustUnnamed = libc::c_uint;
pub const SHUT_RDWR: C2RustUnnamed = 2;
pub const SHUT_WR: C2RustUnnamed = 1;
pub const SHUT_RD: C2RustUnnamed = 0;
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

use libc::FILE;
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
  pub fd: libc::c_int,
  pub events: libc::c_short,
  pub revents: libc::c_short,
}
use crate::librb::uoff_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tls_aes {
  pub key: [u32; 60],
  pub rounds: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tls_state {
  pub flags: libc::c_uint,
  pub ofd: libc::c_int,
  pub ifd: libc::c_int,
  pub min_encrypted_len_on_read: libc::c_uint,
  pub cipher_id: u16,
  pub MAC_size: libc::c_uint,
  pub key_size: libc::c_uint,
  pub IV_size: libc::c_uint,
  pub outbuf: *mut u8,
  pub outbuf_size: libc::c_int,
  pub inbuf_size: libc::c_int,
  pub ofs_to_buffered: libc::c_int,
  pub buffered_size: libc::c_int,
  pub inbuf: *mut u8,
  pub hsd: *mut tls_handshake_data,
  pub write_seq64_be: u64,
  pub client_write_key: *mut u8,
  pub server_write_key: *mut u8,
  pub client_write_IV: *mut u8,
  pub server_write_IV: *mut u8,
  pub client_write_MAC_key: [u8; 32],
  pub server_write_MAC_k__: [u8; 32],
  pub client_write_k__: [u8; 32],
  pub server_write_k__: [u8; 32],
  pub client_write_I_: [u8; 4],
  pub server_write_I_: [u8; 4],
  pub aes_encrypt: tls_aes,
  pub aes_decrypt: tls_aes,
  pub H: [u8; 16],
}
pub type tls_state_t = tls_state;
use crate::libbb::llist::llist_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bb_progress_t {
  pub last_size: libc::c_uint,
  pub last_update_sec: libc::c_uint,
  pub last_change_sec: libc::c_uint,
  pub start_sec: libc::c_uint,
  pub curfile: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub content_len: off_t,
  pub beg_range: off_t,
  pub transferred: off_t,
  pub curfile: *const libc::c_char,
  pub pmt: bb_progress_t,
  pub dir_prefix: *mut libc::c_char,
  pub post_data: *mut libc::c_char,
  pub extra_headers: *mut libc::c_char,
  pub user_headers: libc::c_uchar,
  pub fname_out: *mut libc::c_char,
  pub fname_log: *mut libc::c_char,
  pub proxy_flag: *const libc::c_char,
  pub user_agent: *const libc::c_char,
  pub output_fd: libc::c_int,
  pub log_fd: libc::c_int,
  pub o_flags: libc::c_int,
  pub timeout_seconds: libc::c_uint,
  pub die_if_timed_out: smallint,
  pub chunked: smallint,
  pub got_clen: smallint,
  pub wget_buf: [libc::c_char; 4096],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct host_info {
  pub allocated: *mut libc::c_char,
  pub path: *const libc::c_char,
  pub user: *mut libc::c_char,
  pub protocol: *const libc::c_char,
  pub host: *mut libc::c_char,
  pub port: libc::c_int,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const HDR_PROXY_AUTH: C2RustUnnamed_2 = 16;
pub const HDR_AUTH: C2RustUnnamed_2 = 8;
pub const HDR_RANGE: C2RustUnnamed_2 = 4;
pub const HDR_USER_AGENT: C2RustUnnamed_2 = 2;
pub const HDR_HOST: C2RustUnnamed_2 = 1;
/* Must match option string! */
pub type C2RustUnnamed_3 = libc::c_uint;
pub const WGET_OPT_NO_CHECK_CERT: C2RustUnnamed_3 = 16384;
pub const WGET_OPT_SPIDER: C2RustUnnamed_3 = 8192;
pub const WGET_OPT_POST_DATA: C2RustUnnamed_3 = 4096;
pub const WGET_OPT_HEADER: C2RustUnnamed_3 = 2048;
pub const WGET_OPT_nsomething: C2RustUnnamed_3 = 1024;
pub const WGET_OPT_RETRIES: C2RustUnnamed_3 = 512;
pub const WGET_OPT_NETWORK_READ_TIMEOUT: C2RustUnnamed_3 = 256;
pub const WGET_OPT_USER_AGENT: C2RustUnnamed_3 = 128;
pub const WGET_OPT_PROXY: C2RustUnnamed_3 = 64;
pub const WGET_OPT_PREFIX: C2RustUnnamed_3 = 32;
pub const WGET_OPT_LOGNAME: C2RustUnnamed_3 = 16;
pub const WGET_OPT_OUTNAME: C2RustUnnamed_3 = 8;
pub const WGET_OPT_SERVER_RESPONSE: C2RustUnnamed_3 = 4;
pub const WGET_OPT_QUIET: C2RustUnnamed_3 = 2;
pub const WGET_OPT_CONTINUE: C2RustUnnamed_3 = 1;
pub type C2RustUnnamed_4 = libc::c_int;
pub const PROGRESS_BUMP: C2RustUnnamed_4 = 1;
pub const PROGRESS_END: C2RustUnnamed_4 = 0;
pub const PROGRESS_START: C2RustUnnamed_4 = -1;
pub const KEY_location: C2RustUnnamed_5 = 3;
pub const KEY_transfer_encoding: C2RustUnnamed_5 = 2;
pub const KEY_content_length: C2RustUnnamed_5 = 1;
pub type C2RustUnnamed_5 = libc::c_uint;
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
#[inline]
unsafe extern "C" fn new_tls_state() -> *mut tls_state_t {
  let mut tls: *mut tls_state_t =
    xzalloc(::std::mem::size_of::<tls_state_t>() as libc::c_ulong) as *mut tls_state_t;
  return tls;
}
#[inline(always)]
unsafe extern "C" fn bb_strtoul(
  mut arg: *const libc::c_char,
  mut endp: *mut *mut libc::c_char,
  mut base: libc::c_int,
) -> libc::c_ulong {
  return bb_strtoull(arg, endp, base) as libc::c_ulong;
}
static mut P_FTP: [libc::c_char; 4] = [102, 116, 112, 0];
static mut P_HTTP: [libc::c_char; 5] = [104, 116, 116, 112, 0];
static mut P_FTPS: [libc::c_char; 5] = [102, 116, 112, 115, 0];
static mut P_HTTPS: [libc::c_char; 6] = [104, 116, 116, 112, 115, 0];
static mut wget_user_headers: [libc::c_char; 62] = [
  72, 111, 115, 116, 58, 0, 85, 115, 101, 114, 45, 65, 103, 101, 110, 116, 58, 0, 82, 97, 110, 103,
  101, 58, 0, 65, 117, 116, 104, 111, 114, 105, 122, 97, 116, 105, 111, 110, 58, 0, 80, 114, 111,
  120, 121, 45, 65, 117, 116, 104, 111, 114, 105, 122, 97, 116, 105, 111, 110, 58, 0, 0,
];
unsafe extern "C" fn progress_meter(mut flag: libc::c_int) {
  let mut notty: libc::c_int = 0;
  if option_mask32 & WGET_OPT_QUIET as libc::c_int as libc::c_uint != 0 {
    return;
  }
  /* Don't save progress to log file */
  if (*ptr_to_globals).log_fd >= 0i32 {
    return;
  } /* it's tty */
  if flag == PROGRESS_START as libc::c_int {
    bb_progress_init(&mut (*ptr_to_globals).pmt, (*ptr_to_globals).curfile);
  }
  notty = bb_progress_update(
    &mut (*ptr_to_globals).pmt,
    (*ptr_to_globals).beg_range as uoff_t,
    (*ptr_to_globals).transferred as uoff_t,
    if (*ptr_to_globals).chunked as libc::c_int != 0 || (*ptr_to_globals).got_clen == 0 {
      0i32 as libc::c_long
    } else {
      ((*ptr_to_globals).beg_range + (*ptr_to_globals).transferred) + (*ptr_to_globals).content_len
    } as uoff_t,
  );
  if flag == PROGRESS_END as libc::c_int {
    free((*ptr_to_globals).pmt.curfile as *mut libc::c_char as *mut libc::c_void);
    (*ptr_to_globals).pmt.curfile = 0 as *const libc::c_char;
    if notty == 0i32 {
      bb_putchar_stderr('\n' as i32 as libc::c_char);
    }
    (*ptr_to_globals).transferred = 0i32 as off_t
  };
}
/* IPv6 knows scoped address types i.e. link and site local addresses. Link
 * local addresses can have a scope identifier to specify the
 * interface/link an address is valid on (e.g. fe80::1%eth0). This scope
 * identifier is only valid on a single node.
 *
 * RFC 4007 says that the scope identifier MUST NOT be sent across the wire,
 * unless all nodes agree on the semantic. Apache e.g. regards zone identifiers
 * in the Host header as invalid requests, see
 * https://issues.apache.org/bugzilla/show_bug.cgi?id=35122
 */
unsafe extern "C" fn strip_ipv6_scope_id(mut host: *mut libc::c_char) {
  let mut scope: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
  /* bbox wget actually handles IPv6 addresses without [], like
   * wget "http://::1/xxx", but this is not standard.
   * To save code, _here_ we do not support it. */
  if *host.offset(0) as libc::c_int != '[' as i32 {
    return;
  } /* not IPv6 */
  scope = strchr(host, '%' as i32);
  if scope.is_null() {
    return;
  }
  /* Remove the IPv6 zone identifier from the host address */
  cp = strchr(host, ']' as i32);
  if cp.is_null()
    || *cp.offset(1) as libc::c_int != ':' as i32 && *cp.offset(1) as libc::c_int != '\u{0}' as i32
  {
    /* malformed address (not "[xx]:nn" or "[xx]") */
    return;
  }
  /* cp points to "]...", scope points to "%eth0]..." */
  overlapping_strcpy(scope, cp);
}
/* Base64-encode character string. */
unsafe extern "C" fn base64enc(mut str: *const libc::c_char) -> *mut libc::c_char {
  /* paranoia */
  let mut len: libc::c_uint = strnlen(
    str,
    (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
      .wrapping_div(4i32 as libc::c_ulong)
      .wrapping_mul(3i32 as libc::c_ulong)
      .wrapping_sub(10i32 as libc::c_ulong),
  ) as libc::c_uint;
  bb_uuencode(
    (*ptr_to_globals).wget_buf.as_mut_ptr(),
    str as *const libc::c_void,
    len as libc::c_int,
    bb_uuenc_tbl_base64.as_ptr(),
  );
  return (*ptr_to_globals).wget_buf.as_mut_ptr();
}
unsafe extern "C" fn alarm_handler(mut _sig: libc::c_int) {
  /* This is theoretically unsafe (uses stdio and malloc in signal handler) */
  if (*ptr_to_globals).die_if_timed_out != 0 {
    bb_simple_error_msg_and_die(b"download timed out\x00" as *const u8 as *const libc::c_char);
  };
}
unsafe extern "C" fn set_alarm() {
  if (*ptr_to_globals).timeout_seconds != 0 {
    alarm((*ptr_to_globals).timeout_seconds);
    (*ptr_to_globals).die_if_timed_out = 1i32 as smallint
  };
}
/*
 * is_ip_address() attempts to verify whether or not a string
 * contains an IPv4 or IPv6 address (vs. an FQDN).  The result
 * of inet_pton() can be used to determine this.
 */
unsafe extern "C" fn is_ip_address(mut string: *const libc::c_char) -> libc::c_int {
  let mut sa: sockaddr_in = sockaddr_in {
    sin_family: 0,
    sin_port: 0,
    sin_addr: in_addr { s_addr: 0 },
    sin_zero: [0; 8],
  };
  let mut result: libc::c_int = inet_pton(
    2i32,
    string,
    &mut sa.sin_addr as *mut in_addr as *mut libc::c_void,
  );
  if result == 0i32 {
    let mut sa6: sockaddr_in6 = sockaddr_in6 {
      sin6_family: 0,
      sin6_port: 0,
      sin6_flowinfo: 0,
      sin6_addr: in6_addr {
        __in6_u: C2RustUnnamed_0 {
          __u6_addr8: [0; 16],
        },
      },
      sin6_scope_id: 0,
    };
    result = inet_pton(
      10i32,
      string,
      &mut sa6.sin6_addr as *mut in6_addr as *mut libc::c_void,
    )
  }
  return (result == 1i32) as libc::c_int;
}
unsafe extern "C" fn open_socket(mut lsa: *mut len_and_sockaddr) -> *mut FILE {
  let mut fd: libc::c_int = 0;
  let mut fp: *mut FILE = 0 as *mut FILE;
  set_alarm();
  fd = xconnect_stream(lsa);
  (*ptr_to_globals).die_if_timed_out = 0i32 as smallint;
  /* glibc 2.4 seems to try seeking on it - ??! */
  /* hopefully it understands what ESPIPE means... */
  fp = fdopen(fd, b"r+\x00" as *const u8 as *const libc::c_char);
  if fp.is_null() {
    bb_die_memory_exhausted();
  }
  return fp;
}
/* We balk at any control chars in other side's messages.
 * This prevents nasty surprises (e.g. ESC sequences) in "Location:" URLs
 * and error messages.
 *
 * The only exception is tabs, which are converted to (one) space:
 * HTTP's "headers: <whitespace> values" may have those.
 */
unsafe extern "C" fn sanitize_string(mut s: *mut libc::c_char) -> *mut libc::c_char {
  let mut p: *mut libc::c_uchar = s as *mut libc::c_void as *mut libc::c_uchar;
  while *p != 0 {
    if (*p as libc::c_int) < ' ' as i32 {
      if *p as libc::c_int != '\t' as i32 {
        break;
      }
      *p = ' ' as i32 as libc::c_uchar
    }
    p = p.offset(1)
  }
  *p = '\u{0}' as i32 as libc::c_uchar;
  return s;
}
/* Returns '\n' if it was seen, else '\0'. Trims at first '\r' or '\n' */
unsafe extern "C" fn fgets_trim_sanitize(
  mut fp: *mut FILE,
  mut fmt: *const libc::c_char,
) -> libc::c_char {
  let mut c: libc::c_char = 0;
  let mut buf_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
  set_alarm();
  if fgets_unlocked(
    (*ptr_to_globals).wget_buf.as_mut_ptr(),
    ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
    fp,
  )
  .is_null()
  {
    bb_simple_perror_msg_and_die(b"error getting response\x00" as *const u8 as *const libc::c_char);
  }
  (*ptr_to_globals).die_if_timed_out = 0i32 as smallint;
  buf_ptr = strchrnul((*ptr_to_globals).wget_buf.as_mut_ptr(), '\n' as i32);
  c = *buf_ptr;
  /* Disallow any control chars: trim at first char < 0x20 */
  sanitize_string((*ptr_to_globals).wget_buf.as_mut_ptr());
  if !fmt.is_null() && option_mask32 & WGET_OPT_SERVER_RESPONSE as libc::c_int as libc::c_uint != 0
  {
    fprintf(stderr, fmt, (*ptr_to_globals).wget_buf.as_mut_ptr());
  }
  return c;
}
unsafe extern "C" fn ftpcmd(
  mut s1: *const libc::c_char,
  mut s2: *const libc::c_char,
  mut fp: *mut FILE,
) -> libc::c_int {
  let mut result: libc::c_int = 0;
  if !s1.is_null() {
    if s2.is_null() {
      s2 = b"\x00" as *const u8 as *const libc::c_char
    }
    fprintf(
      fp,
      b"%s%s\r\n\x00" as *const u8 as *const libc::c_char,
      s1,
      s2,
    );
    /* With --server-response, wget also shows its ftp commands */
    if option_mask32 & WGET_OPT_SERVER_RESPONSE as libc::c_int as libc::c_uint != 0 {
      fprintf(
        stderr,
        b"--> %s%s\n\n\x00" as *const u8 as *const libc::c_char,
        s1,
        s2,
      );
    }
    fflush(fp);
  }
  /* Read until "Nxx something" is received */
  (*ptr_to_globals).wget_buf[3] = 0i32 as libc::c_char;
  loop {
    fgets_trim_sanitize(fp, b"%s\n\x00" as *const u8 as *const libc::c_char);
    if !(!(((*ptr_to_globals).wget_buf[0] as libc::c_int - '0' as i32) as libc::c_uchar
      as libc::c_int
      <= 9i32)
      || (*ptr_to_globals).wget_buf[3] as libc::c_int != ' ' as i32)
    {
      break;
    }
  }
  (*ptr_to_globals).wget_buf[3] = '\u{0}' as i32 as libc::c_char;
  result = xatoi_positive((*ptr_to_globals).wget_buf.as_mut_ptr());
  (*ptr_to_globals).wget_buf[3] = ' ' as i32 as libc::c_char;
  return result;
}
unsafe extern "C" fn parse_url(mut src_url: *const libc::c_char, mut h: *mut host_info) {
  let mut url: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
  free((*h).allocated as *mut libc::c_void);
  url = xstrdup(src_url);
  (*h).allocated = url;
  (*h).protocol = P_FTP.as_ptr();
  p = strstr(url, b"://\x00" as *const u8 as *const libc::c_char);
  let mut current_block_19: u64;
  if !p.is_null() {
    *p = '\u{0}' as i32 as libc::c_char;
    (*h).host = p.offset(3);
    if strcmp(url, P_FTP.as_ptr()) == 0i32 {
      (*h).port = 21i32;
      current_block_19 = 11298138898191919651;
    } else if strcmp(url, P_FTPS.as_ptr()) == 0i32 {
      (*h).port = 990i32;
      (*h).protocol = P_FTPS.as_ptr();
      current_block_19 = 11298138898191919651;
    } else if strcmp(url, P_HTTPS.as_ptr()) == 0i32 {
      (*h).port = 443i32;
      (*h).protocol = P_HTTPS.as_ptr();
      current_block_19 = 11298138898191919651;
    } else if strcmp(url, P_HTTP.as_ptr()) == 0i32 {
      current_block_19 = 15973521690641649086;
    } else {
      *p = ':' as i32 as libc::c_char;
      bb_error_msg_and_die(
        b"not an http or ftp url: %s\x00" as *const u8 as *const libc::c_char,
        url,
      );
    }
  } else {
    // GNU wget is user-friendly and falls back to http://
    (*h).host = url;
    current_block_19 = 15973521690641649086;
  }
  match current_block_19 {
    15973521690641649086 => {
      (*h).port = 80i32;
      (*h).protocol = P_HTTP.as_ptr()
    }
    _ => {}
  }
  // FYI:
  // "Real" wget 'http://busybox.net?var=a/b' sends this request:
  //   'GET /?var=a/b HTTP/1.0'
  //   and saves 'index.html?var=a%2Fb' (we save 'b')
  // wget 'http://busybox.net?login=john@doe':
  //   request: 'GET /?login=john@doe HTTP/1.0'
  //   saves: 'index.html?login=john@doe' (we save 'login=john@doe')
  // wget 'http://busybox.net#test/test':
  //   request: 'GET / HTTP/1.0'
  //   saves: 'index.html' (we save 'test')
  //
  // We also don't add unique .N suffix if file exists...
  sp = strchr((*h).host, '/' as i32);
  p = strchr((*h).host, '?' as i32);
  if sp.is_null() || !p.is_null() && sp > p {
    sp = p
  }
  p = strchr((*h).host, '#' as i32);
  if sp.is_null() || !p.is_null() && sp > p {
    sp = p
  }
  if sp.is_null() {
    (*h).path = b"\x00" as *const u8 as *const libc::c_char
  } else if *sp as libc::c_int == '/' as i32 {
    *sp = '\u{0}' as i32 as libc::c_char;
    (*h).path = sp.offset(1)
  } else {
    // sp points to '#' or '?'
    // Note:
    // http://busybox.net?login=john@doe is a valid URL
    // (without '/' between ".net" and "?"),
    // can't store NUL at sp[-1] - this destroys hostname.
    let fresh0 = sp;
    sp = sp.offset(1);
    *fresh0 = '\u{0}' as i32 as libc::c_char;
    (*h).path = sp
  }
  sp = strrchr((*h).host, '@' as i32);
  if !sp.is_null() {
    // URL-decode "user:password" string before base64-encoding:
    // wget http://test:my%20pass@example.com should send
    // Authorization: Basic dGVzdDpteSBwYXNz
    // which decodes to "test:my pass".
    // Standard wget and curl do this too.
    *sp = '\u{0}' as i32 as libc::c_char;
    free((*h).user as *mut libc::c_void);
    (*h).user = xstrdup(percent_decode_in_place((*h).host, 0i32));
    (*h).host = sp.offset(1)
  };
  /* else: h->user remains NULL, or as set by original request
   * before redirect (if we are here after a redirect).
   */
}
unsafe extern "C" fn get_sanitized_hdr(mut fp: *mut FILE) -> *mut libc::c_char {
  let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut hdrval: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut c: libc::c_int = 0;
  /* retrieve header line */
  c = fgets_trim_sanitize(fp, b"  %s\n\x00" as *const u8 as *const libc::c_char) as libc::c_int;
  /* end of the headers? */
  if (*ptr_to_globals).wget_buf[0] as libc::c_int == '\u{0}' as i32 {
    return 0 as *mut libc::c_char;
  }
  /* convert the header name to lower case */
  s = (*ptr_to_globals).wget_buf.as_mut_ptr();
  while bb_ascii_isalnum(*s as libc::c_uchar) != 0
    || *s as libc::c_int == '-' as i32
    || *s as libc::c_int == '.' as i32
    || *s as libc::c_int == '_' as i32
  {
    /*
     * No-op for 20-3f and 60-7f. "0-9a-z-." are in these ranges.
     * 40-5f range ("@A-Z[\]^_") maps to 60-7f.
     * "A-Z" maps to "a-z".
     * "@[\]" can't occur in header names.
     * "^_" maps to "~,DEL" (which is wrong).
     * "^" was never seen yet, "_" was seen from web.archive.org
     * (x-archive-orig-x_commoncrawl_Signature: HEXSTRING).
     */
    *s = (*s as libc::c_int | 0x20i32) as libc::c_char;
    s = s.offset(1)
  }
  /* verify we are at the end of the header name */
  if *s as libc::c_int != ':' as i32 {
    bb_error_msg_and_die(
      b"bad header line: %s\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).wget_buf.as_mut_ptr(),
    );
  }
  /* locate the start of the header value */
  let fresh1 = s;
  s = s.offset(1);
  *fresh1 = '\u{0}' as i32 as libc::c_char;
  hdrval = skip_whitespace(s);
  if c != '\n' as i32 {
    loop
    /* Rats! The buffer isn't big enough to hold the entire header value */
    {
      c = getc_unlocked(fp);
      if !(c != -1i32 && c != '\n' as i32) {
        break;
      }
    }
  }
  return hdrval;
}
unsafe extern "C" fn reset_beg_range_to_zero() {
  bb_simple_error_msg(b"restart failed\x00" as *const u8 as *const libc::c_char);
  (*ptr_to_globals).beg_range = 0i32 as off_t;
  xlseek((*ptr_to_globals).output_fd, 0i32 as off_t, 0i32);
  /* Done at the end instead: */
  /* ftruncate(G.output_fd, 0); */
}
unsafe extern "C" fn spawn_https_helper_openssl(
  mut host: *const libc::c_char,
  mut port: libc::c_uint,
) -> libc::c_int {
  let mut allocated: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut servername: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut sp: [libc::c_int; 2] = [0; 2];
  let mut pid: libc::c_int = 0;
  let mut child_failed: libc::c_int = 0i32;
  if socketpair(1i32, SOCK_STREAM as libc::c_int, 0i32, sp.as_mut_ptr()) != 0i32 {
    /* Kernel can have AF_UNIX support disabled */
    bb_simple_perror_msg_and_die(b"socketpair\x00" as *const u8 as *const libc::c_char);
  }
  if strchr(host, ':' as i32).is_null() {
    allocated = xasprintf(b"%s:%u\x00" as *const u8 as *const libc::c_char, host, port);
    host = allocated
  }
  servername = xstrdup(host);
  *strrchr(servername, ':' as i32).offset(0) = '\u{0}' as i32 as libc::c_char;
  fflush_all();
  pid = {
    let mut bb__xvfork_pid: pid_t = vfork();
    if bb__xvfork_pid < 0i32 {
      bb_simple_perror_msg_and_die(b"vfork\x00" as *const u8 as *const libc::c_char);
    }
    bb__xvfork_pid
  };
  if pid == 0i32 {
    /* Child */
    let mut argv: [*mut libc::c_char; 8] = [0 as *mut libc::c_char; 8];
    close(sp[0]);
    xmove_fd(sp[1], 0i32);
    xdup2(0i32, 1i32);
    /*
     * openssl s_client -quiet -connect www.kernel.org:443 2>/dev/null
     * It prints some debug stuff on stderr, don't know how to suppress it.
     * Work around by dev-nulling stderr. We lose all error messages :(
     */
    xmove_fd(2i32, 3i32);
    xopen(b"/dev/null\x00" as *const u8 as *const libc::c_char, 0o2i32);
    memset(
      &mut argv as *mut [*mut libc::c_char; 8] as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<[*mut libc::c_char; 8]>() as libc::c_ulong,
    );
    argv[0] = b"openssl\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[1] = b"s_client\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[2] = b"-quiet\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[3] = b"-connect\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[4] = host as *mut libc::c_char;
    /*
     * Per RFC 6066 Section 3, the only permitted values in the
     * TLS server_name (SNI) field are FQDNs (DNS hostnames).
     * IPv4 and IPv6 addresses, port numbers are not allowed.
     */
    if is_ip_address(servername) == 0 {
      argv[5] = b"-servername\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
      argv[6] = servername
    }
    execvp(argv[0], argv.as_mut_ptr() as *const *mut libc::c_char);
    xmove_fd(3i32, 2i32);
    ::std::ptr::write_volatile(&mut child_failed as *mut libc::c_int, 1i32);
    xfunc_die();
    /* notreached */
  }
  /* Parent */
  free(servername as *mut libc::c_void);
  free(allocated as *mut libc::c_void);
  close(sp[1]);
  if child_failed != 0 {
    close(sp[0]);
    return -1i32;
  }
  return sp[0];
}
unsafe extern "C" fn spawn_ssl_client(
  mut host: *const libc::c_char,
  mut network_fd: libc::c_int,
  mut flags: libc::c_int,
) {
  let mut sp: [libc::c_int; 2] = [0; 2];
  let mut pid: libc::c_int = 0;
  let mut servername: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  if option_mask32 & WGET_OPT_NO_CHECK_CERT as libc::c_int as libc::c_uint == 0 {
    option_mask32 |= WGET_OPT_NO_CHECK_CERT as libc::c_int as libc::c_uint;
    bb_simple_error_msg(
      b"note: TLS certificate validation not implemented\x00" as *const u8 as *const libc::c_char,
    );
  }
  servername = xstrdup(host);
  p = strrchr(servername, ':' as i32);
  if !p.is_null() {
    *p = '\u{0}' as i32 as libc::c_char
  }
  if socketpair(1i32, SOCK_STREAM as libc::c_int, 0i32, sp.as_mut_ptr()) != 0i32 {
    /* Kernel can have AF_UNIX support disabled */
    bb_simple_perror_msg_and_die(b"socketpair\x00" as *const u8 as *const libc::c_char);
  }
  fflush_all();
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
    /* Child */
    close(sp[0]);
    xmove_fd(sp[1], 0i32);
    xdup2(0i32, 1i32);
    let mut tls: *mut tls_state_t = new_tls_state();
    (*tls).ofd = network_fd;
    (*tls).ifd = (*tls).ofd;
    tls_handshake(tls, servername);
    tls_run_copy_loop(tls, flags as libc::c_uint);
    exit(0i32);
    /* notreached */
  }
  /* Parent */
  free(servername as *mut libc::c_void);
  close(sp[1]);
  xmove_fd(sp[0], network_fd);
}
unsafe extern "C" fn prepare_ftp_session(
  mut dfpp: *mut *mut FILE,
  mut target: *mut host_info,
  mut lsa: *mut len_and_sockaddr,
) -> *mut FILE {
  let mut current_block: u64;
  let mut sfp: *mut FILE = 0 as *mut FILE;
  let mut pass: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut port: libc::c_int = 0;
  sfp = open_socket(lsa);
  if (*target).protocol == P_FTPS.as_ptr() {
    spawn_ssl_client((*target).host, fileno_unlocked(sfp), 1i32 << 0i32);
  }
  if ftpcmd(0 as *const libc::c_char, 0 as *const libc::c_char, sfp) != 220i32 {
    bb_simple_error_msg_and_die((*ptr_to_globals).wget_buf.as_mut_ptr());
  }
  /* note: ftpcmd() sanitizes G.wget_buf, ok to print */
  /* Split username:password pair */
  pass = b"busybox\x00" as *const u8 as *const libc::c_char as *mut libc::c_char; /* password for "anonymous" */
  if !(*target).user.is_null() {
    pass = strchr((*target).user, ':' as i32);
    if !pass.is_null() {
      let fresh2 = pass;
      pass = pass.offset(1);
      *fresh2 = '\u{0}' as i32 as libc::c_char
    }
  }
  let mut current_block_11: u64;
  /* Log in */
  match ftpcmd(
    b"USER \x00" as *const u8 as *const libc::c_char,
    if !(*target).user.is_null() {
      (*target).user
    } else {
      b"anonymous\x00" as *const u8 as *const libc::c_char
    },
    sfp,
  ) {
    230 => {
      current_block_11 = 9606288038608642794;
    }
    331 => {
      if ftpcmd(b"PASS \x00" as *const u8 as *const libc::c_char, pass, sfp) == 230i32 {
        current_block_11 = 9606288038608642794;
      } else {
        current_block_11 = 17720461952361946060;
      }
    }
    _ => {
      current_block_11 = 17720461952361946060;
    }
  }
  match current_block_11 {
    17720461952361946060 =>
    /* fall through (failed login) */
    {
      bb_error_msg_and_die(
        b"ftp login: %s\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).wget_buf.as_mut_ptr(),
      );
    }
    _ => {}
  }
  ftpcmd(
    b"TYPE I\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    sfp,
  );
  /* Query file size */
  if ftpcmd(
    b"SIZE \x00" as *const u8 as *const libc::c_char,
    (*target).path,
    sfp,
  ) == 213i32
  {
    (*ptr_to_globals).content_len = bb_strtoul(
      (*ptr_to_globals).wget_buf.as_mut_ptr().offset(4),
      0 as *mut *mut libc::c_char,
      10i32,
    ) as off_t;
    if (*ptr_to_globals).content_len < 0i32 as libc::c_long || *bb_errno != 0 {
      bb_error_msg_and_die(
        b"bad SIZE value \'%s\'\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).wget_buf.as_mut_ptr().offset(4),
      );
    }
    (*ptr_to_globals).got_clen = 1i32 as smallint
  }
  /* Enter passive mode */
  if 1i32 != 0
    && ftpcmd(
      b"EPSV\x00" as *const u8 as *const libc::c_char,
      0 as *const libc::c_char,
      sfp,
    ) == 229i32
  {
    current_block = 7056779235015430508;
  } else if ftpcmd(
    b"PASV\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    sfp,
  ) != 227i32
  {
    current_block = 16626807138230145733;
  } else {
    current_block = 7056779235015430508;
  }
  match current_block {
    7056779235015430508 =>
    /* good */
    {
      port = parse_pasv_epsv((*ptr_to_globals).wget_buf.as_mut_ptr());
      if !(port < 0i32) {
        set_nport(
          &mut (*lsa).u.sa,
          ({
            let mut __v: libc::c_ushort = 0;
            let mut __x: libc::c_ushort = port as libc::c_ushort;
            if 0 != 0 {
              __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
                as libc::c_ushort
            } else {
              let fresh3 = &mut __v;
              let fresh4;
              let fresh5 = __x;
              asm!("rorw $$8, ${0:w}" : "=r" (fresh4) :
                                        "0"
                                        (c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5))
                                        : "cc");
              c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
            }
            __v
          }) as libc::c_uint,
        );
        *dfpp = open_socket(lsa);
        if (*target).protocol == P_FTPS.as_ptr() {
          /* "PROT P" enables encryption of data stream.
           * Without it (or with "PROT C"), data is sent unencrypted.
           */
          if ftpcmd(
            b"PROT P\x00" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            sfp,
          ) == 200i32
          {
            spawn_ssl_client((*target).host, fileno_unlocked(*dfpp), 0i32);
          }
        }
        if (*ptr_to_globals).beg_range != 0i32 as libc::c_long {
          sprintf(
            (*ptr_to_globals).wget_buf.as_mut_ptr(),
            b"REST %lu\x00" as *const u8 as *const libc::c_char,
            (*ptr_to_globals).beg_range,
          );
          if ftpcmd(
            (*ptr_to_globals).wget_buf.as_mut_ptr(),
            0 as *const libc::c_char,
            sfp,
          ) == 350i32
          {
            (*ptr_to_globals).content_len -= (*ptr_to_globals).beg_range
          } else {
            reset_beg_range_to_zero();
          }
        }
        //TODO: needs ftp-escaping 0xff and '\n' bytes here.
        //Or disallow '\n' altogether via sanitize_string() in parse_url().
        //But 0xff's are possible in valid utf8 filenames.
        if ftpcmd(
          b"RETR \x00" as *const u8 as *const libc::c_char,
          (*target).path,
          sfp,
        ) > 150i32
        {
          bb_error_msg_and_die(
            b"bad response to %s: %s\x00" as *const u8 as *const libc::c_char,
            b"RETR\x00" as *const u8 as *const libc::c_char,
            (*ptr_to_globals).wget_buf.as_mut_ptr(),
          );
        }
        return sfp;
      }
    }
    _ => {}
  }
  bb_error_msg_and_die(
    b"bad response to %s: %s\x00" as *const u8 as *const libc::c_char,
    b"PASV\x00" as *const u8 as *const libc::c_char,
    (*ptr_to_globals).wget_buf.as_mut_ptr(),
  );
}
#[inline(never)]
unsafe extern "C" fn retrieve_file_data(mut dfp: *mut FILE) {
  let mut current_block: u64;
  let mut second_cnt: libc::c_uint = (*ptr_to_globals).timeout_seconds;
  let mut polldata: pollfd = pollfd {
    fd: 0,
    events: 0,
    revents: 0,
  };
  polldata.fd = fileno_unlocked(dfp);
  polldata.events = (0x1i32 | 0x2i32) as libc::c_short;
  if option_mask32 & WGET_OPT_QUIET as libc::c_int as libc::c_uint == 0 {
    if (*ptr_to_globals).output_fd == 1i32 {
      fprintf(
        stderr,
        b"writing to stdout\n\x00" as *const u8 as *const libc::c_char,
      );
    } else {
      fprintf(
        stderr,
        b"saving to \'%s\'\n\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).fname_out,
      );
    }
  }
  progress_meter(PROGRESS_START as libc::c_int);
  if (*ptr_to_globals).chunked != 0 {
    current_block = 13298589467384833406;
  } else {
    /* Loops only if chunked */
    current_block = 1917311967535052937;
  }
  loop {
    match current_block {
      1917311967535052937 => {
        /* Must use nonblocking I/O, otherwise fread will loop
         * and *block* until it reads full buffer,
         * which messes up progress bar and/or timeout logic.
         * Because of nonblocking I/O, we need to dance
         * very carefully around EAGAIN. See explanation at
         * clearerr() calls.
         */
        ndelay_on(polldata.fd);
        loop
        /*
         * Note that fgets may result in some data being buffered in dfp.
         * We loop back to fread, which will retrieve this data.
         * Also note that code has to be arranged so that fread
         * is done _before_ one-second poll wait - poll doesn't know
         * about stdio buffering and can result in spurious one second waits!
         */
        {
          let mut n: libc::c_int = 0; /* while (reading data) */
          let mut rdsz: libc::c_uint = 0;
          /* fread internally uses read loop, which in our case
           * is usually exited when we get EAGAIN.
           * In this case, libc sets error marker on the stream.
           * Need to clear it before next fread to avoid possible
           * rare false positive ferror below. Rare because usually
           * fread gets more than zero bytes, and we don't fall
           * into if (n <= 0) ...
           */
          clearerr(dfp);
          *bb_errno = 0i32;
          rdsz = ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_uint;
          if (*ptr_to_globals).got_clen != 0 {
            if (*ptr_to_globals).content_len
              < ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as off_t
            {
              if (*ptr_to_globals).content_len as libc::c_int <= 0i32 {
                break;
              }
              rdsz = (*ptr_to_globals).content_len as libc::c_uint
            }
          }
          n = fread(
            (*ptr_to_globals).wget_buf.as_mut_ptr() as *mut libc::c_void,
            1i32 as size_t,
            rdsz as size_t,
            dfp,
          ) as libc::c_int;
          if n > 0i32 {
            xwrite(
              (*ptr_to_globals).output_fd,
              (*ptr_to_globals).wget_buf.as_mut_ptr() as *const libc::c_void,
              n as size_t,
            );
            (*ptr_to_globals).transferred += n as libc::c_long;
            if (*ptr_to_globals).got_clen != 0 {
              (*ptr_to_globals).content_len -= n as libc::c_long;
              if (*ptr_to_globals).content_len == 0i32 as libc::c_long {
                break;
              }
            }
            second_cnt = (*ptr_to_globals).timeout_seconds
          } else if *bb_errno != 11i32 {
            if ferror_unlocked(dfp) != 0 {
              progress_meter(PROGRESS_END as libc::c_int);
              bb_simple_perror_msg_and_die(b"read error\x00" as *const u8 as *const libc::c_char);
            }
            break;
          /* n <= 0.
           * man fread:
           * If error occurs, or EOF is reached, the return value
           * is a short item count (or zero).
           * fread does not distinguish between EOF and error.
           */
          /* EOF, not error */
          } else if safe_poll(&mut polldata, 1i32 as nfds_t, 1000i32) == 0i32 {
            if second_cnt != 0i32 as libc::c_uint && {
              second_cnt = second_cnt.wrapping_sub(1);
              (second_cnt) == 0i32 as libc::c_uint
            } {
              progress_meter(PROGRESS_END as libc::c_int);
              bb_simple_error_msg_and_die(
                b"download timed out\x00" as *const u8 as *const libc::c_char,
              );
            }
            /* It was EAGAIN. There is no data. Wait up to one second
             * then abort if timed out, or update the bar and try reading again.
             */
            /* We used to loop back to poll here,
             * but there is no great harm in letting fread
             * to try reading anyway.
             */
          }
          /* Need to do it _every_ second for "stalled" indicator
           * to be shown properly.
           */
          progress_meter(PROGRESS_BUMP as libc::c_int); /* else fgets can get very unhappy */
        }
        clearerr(dfp);
        ndelay_off(polldata.fd);
        if (*ptr_to_globals).chunked == 0 {
          break;
        }
        /* Each chunk ends with "\r\n" - eat it */
        fgets_trim_sanitize(dfp, 0 as *const libc::c_char);
        current_block = 13298589467384833406;
      }
      _ => {
        /* chunk size format is "HEXNUM[;name[=val]]\r\n" */
        fgets_trim_sanitize(dfp, 0 as *const libc::c_char);
        *bb_errno = 0i32;
        (*ptr_to_globals).content_len = strtoul(
          (*ptr_to_globals).wget_buf.as_mut_ptr(),
          0 as *mut *mut libc::c_char,
          16i32,
        ) as off_t;
        /*
         * Had a bug with inputs like "ffffffff0001f400"
         * smashing the heap later. Ensure >= 0.
         */
        if (*ptr_to_globals).content_len < 0i32 as libc::c_long || *bb_errno != 0 {
          bb_error_msg_and_die(
            b"bad chunk length \'%s\'\x00" as *const u8 as *const libc::c_char,
            (*ptr_to_globals).wget_buf.as_mut_ptr(),
          ); /* all done! */
        }
        if (*ptr_to_globals).content_len == 0i32 as libc::c_long {
          break;
        }
        (*ptr_to_globals).got_clen = 1i32 as smallint;
        current_block = 1917311967535052937;
      }
    }
  }
  /* Draw full bar and free its resources */
  (*ptr_to_globals).chunked = 0i32 as smallint; /* makes it show 100% even for chunked download */
  (*ptr_to_globals).got_clen = 1i32 as smallint; /* makes it show 100% even for download of (formerly) unknown size */
  progress_meter(PROGRESS_END as libc::c_int);
  if (*ptr_to_globals).content_len != 0i32 as libc::c_long {
    bb_simple_perror_msg_and_die(
      b"connection closed prematurely\x00" as *const u8 as *const libc::c_char,
    );
    /* GNU wget says "DATE TIME (NN MB/s) - Connection closed at byte NNN. Retrying." */
  }
  /* If -c failed, we restart from the beginning,
   * but we do not truncate file then, we do it only now, at the end.
   * This lets user to ^C if his 99% complete 10 GB file download
   * failed to restart *without* losing the almost complete file.
   */
  let mut pos: off_t = lseek((*ptr_to_globals).output_fd, 0i32 as __off64_t, 1i32); /* Use proxies if env vars are set  */
  if pos != -1i32 as off_t {
    ftruncate((*ptr_to_globals).output_fd, pos); /* socket to web/ftp server         */
  } /* socket to ftp server (data)      */
  if option_mask32 & WGET_OPT_QUIET as libc::c_int as libc::c_uint == 0 {
    if (*ptr_to_globals).output_fd == 1i32 {
      fprintf(
        stderr,
        b"written to stdout\n\x00" as *const u8 as *const libc::c_char,
      );
    } else {
      fprintf(
        stderr,
        b"\'%s\' saved\n\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).fname_out,
      );
    }
  };
}
unsafe extern "C" fn download_one_url(mut url: *const libc::c_char) {
  let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut status: libc::c_int = 0;
  let mut current_block: u64;
  let mut use_proxy: bool = false;
  let mut redir_limit: libc::c_int = 0;
  let mut lsa: *mut len_and_sockaddr = 0 as *mut len_and_sockaddr;
  let mut sfp: *mut FILE = 0 as *mut FILE;
  let mut dfp: *mut FILE = 0 as *mut FILE;
  let mut fname_out_alloc: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut redirected_path: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut server: host_info = host_info {
    allocated: 0 as *mut libc::c_char,
    path: 0 as *const libc::c_char,
    user: 0 as *mut libc::c_char,
    protocol: 0 as *const libc::c_char,
    host: 0 as *mut libc::c_char,
    port: 0,
  };
  let mut target: host_info = host_info {
    allocated: 0 as *mut libc::c_char,
    path: 0 as *const libc::c_char,
    user: 0 as *mut libc::c_char,
    protocol: 0 as *const libc::c_char,
    host: 0 as *mut libc::c_char,
    port: 0,
  };
  server.allocated = 0 as *mut libc::c_char;
  target.allocated = 0 as *mut libc::c_char;
  server.user = 0 as *mut libc::c_char;
  target.user = 0 as *mut libc::c_char;
  parse_url(url, &mut target);
  /* Use the proxy if necessary */
  use_proxy = strcmp(
    (*ptr_to_globals).proxy_flag,
    b"off\x00" as *const u8 as *const libc::c_char,
  ) != 0i32;
  if use_proxy {
    let mut proxy: *mut libc::c_char =
      getenv(if *target.protocol.offset(0) as libc::c_int == 'f' as i32 {
        b"ftp_proxy\x00" as *const u8 as *const libc::c_char
      } else {
        b"http_proxy\x00" as *const u8 as *const libc::c_char
      });
    //FIXME: what if protocol is https? Ok to use http_proxy?
    use_proxy = !proxy.is_null() && *proxy.offset(0) as libc::c_int != 0;
    if use_proxy {
      parse_url(proxy, &mut server);
    }
  }
  if !use_proxy {
    server.protocol = target.protocol;
    server.port = target.port;
    //free(server.allocated); - can't be non-NULL
    server.allocated = xstrdup(target.host);
    server.host = server.allocated
  }
  strip_ipv6_scope_id(target.host);
  /* If there was no -O FILE, guess output filename */
  fname_out_alloc = 0 as *mut libc::c_char;
  if option_mask32 & WGET_OPT_OUTNAME as libc::c_int as libc::c_uint == 0 {
    (*ptr_to_globals).fname_out = bb_get_last_path_component_nostrip(target.path);
    /* handle "wget http://kernel.org//" */
    if *(*ptr_to_globals).fname_out.offset(0) as libc::c_int == '/' as i32
      || *(*ptr_to_globals).fname_out.offset(0) == 0
    {
      (*ptr_to_globals).fname_out =
        b"index.html\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    /* -P DIR is considered only if there was no -O FILE */
    if !(*ptr_to_globals).dir_prefix.is_null() {
      fname_out_alloc = concat_path_file((*ptr_to_globals).dir_prefix, (*ptr_to_globals).fname_out);
      (*ptr_to_globals).fname_out = fname_out_alloc
    } else {
      /* redirects may free target.path later, need to make a copy */
      fname_out_alloc = xstrdup((*ptr_to_globals).fname_out);
      (*ptr_to_globals).fname_out = fname_out_alloc
    }
  }
  (*ptr_to_globals).curfile = bb_get_last_path_component_nostrip((*ptr_to_globals).fname_out);
  /* Determine where to start transfer */
  (*ptr_to_globals).beg_range = 0i32 as off_t;
  if option_mask32 & WGET_OPT_CONTINUE as libc::c_int as libc::c_uint != 0 {
    (*ptr_to_globals).output_fd = open((*ptr_to_globals).fname_out, 0o1i32);
    if (*ptr_to_globals).output_fd >= 0i32 {
      (*ptr_to_globals).beg_range = xlseek((*ptr_to_globals).output_fd, 0i32 as off_t, 2i32)
    }
    /* File doesn't exist. We do not create file here yet.
     * We are not sure it exists on remote side */
  }
  redir_limit = 16i32;
  'c_12019: loop {
    lsa = xhost2sockaddr(server.host, server.port);
    if option_mask32 & WGET_OPT_QUIET as libc::c_int as libc::c_uint == 0 {
      let mut s: *mut libc::c_char = xmalloc_sockaddr2dotted(&mut (*lsa).u.sa);
      fprintf(
        stderr,
        b"Connecting to %s (%s)\n\x00" as *const u8 as *const libc::c_char,
        server.host,
        s,
      );
      free(s as *mut libc::c_void);
    }
    'c_12021: loop {
      /*G.content_len = 0; - redundant, got_clen = 0 is enough */
      (*ptr_to_globals).got_clen = 0i32 as smallint;
      (*ptr_to_globals).chunked = 0i32 as smallint;
      if use_proxy as libc::c_int != 0 || *target.protocol.offset(0) as libc::c_int != 'f' as i32 {
        /*not ftp[s]*/
        /*
         *  HTTP session
         */
        str = 0 as *mut libc::c_char;
        status = 0;
        /* Open socket to http(s) server */
        /* openssl (and maybe internal TLS) support is configured */
        if server.protocol == P_HTTPS.as_ptr() {
          /* openssl-based helper
           * Inconvenient API since we can't give it an open fd
           */
          let mut fd: libc::c_int =
            spawn_https_helper_openssl(server.host, server.port as libc::c_uint);
          if fd < 0i32 {
            /* no openssl? try internal */
            sfp = open_socket(lsa);
            spawn_ssl_client(server.host, fileno_unlocked(sfp), 0i32);
          } else {
            sfp = fdopen(fd, b"r+\x00" as *const u8 as *const libc::c_char);
            if sfp.is_null() {
              bb_die_memory_exhausted();
            }
          }
        } else {
          sfp = open_socket(lsa)
        }
        /* Send HTTP request */
        if use_proxy {
          fprintf(
            sfp,
            b"GET %s://%s/%s HTTP/1.1\r\n\x00" as *const u8 as *const libc::c_char,
            target.protocol,
            target.host,
            target.path,
          );
        } else {
          fprintf(
            sfp,
            b"%s /%s HTTP/1.1\r\n\x00" as *const u8 as *const libc::c_char,
            if option_mask32 & WGET_OPT_POST_DATA as libc::c_int as libc::c_uint != 0 {
              b"POST\x00" as *const u8 as *const libc::c_char
            } else {
              b"GET\x00" as *const u8 as *const libc::c_char
            },
            target.path,
          );
        }
        if (*ptr_to_globals).user_headers as libc::c_int & HDR_HOST as libc::c_int == 0 {
          fprintf(
            sfp,
            b"Host: %s\r\n\x00" as *const u8 as *const libc::c_char,
            target.host,
          );
        }
        if (*ptr_to_globals).user_headers as libc::c_int & HDR_USER_AGENT as libc::c_int == 0 {
          fprintf(
            sfp,
            b"User-Agent: %s\r\n\x00" as *const u8 as *const libc::c_char,
            (*ptr_to_globals).user_agent,
          );
        }
        /* Ask server to close the connection as soon as we are done
         * (IOW: we do not intend to send more requests)
         */
        fprintf(
          sfp,
          b"Connection: close\r\n\x00" as *const u8 as *const libc::c_char,
        );
        if !target.user.is_null()
          && (*ptr_to_globals).user_headers as libc::c_int & HDR_AUTH as libc::c_int == 0
        {
          fprintf(
            sfp,
            (b"Proxy-Authorization: Basic %s\r\n\x00" as *const u8 as *const libc::c_char)
              .offset(6),
            base64enc(target.user),
          );
        }
        if use_proxy as libc::c_int != 0
          && !server.user.is_null()
          && (*ptr_to_globals).user_headers as libc::c_int & HDR_PROXY_AUTH as libc::c_int == 0
        {
          fprintf(
            sfp,
            b"Proxy-Authorization: Basic %s\r\n\x00" as *const u8 as *const libc::c_char,
            base64enc(server.user),
          );
        }
        if (*ptr_to_globals).beg_range != 0i32 as libc::c_long
          && (*ptr_to_globals).user_headers as libc::c_int & HDR_RANGE as libc::c_int == 0
        {
          fprintf(
            sfp,
            b"Range: bytes=%lu-\r\n\x00" as *const u8 as *const libc::c_char,
            (*ptr_to_globals).beg_range,
          );
        }
        if !(*ptr_to_globals).extra_headers.is_null() {
          fputs_unlocked((*ptr_to_globals).extra_headers, sfp);
        }
        if option_mask32 & WGET_OPT_POST_DATA as libc::c_int as libc::c_uint != 0 {
          fprintf(
            sfp,
            b"Content-Type: application/x-www-form-urlencoded\r\nContent-Length: %u\r\n\r\n%s\x00"
              as *const u8 as *const libc::c_char,
            strlen((*ptr_to_globals).post_data) as libc::c_int,
            (*ptr_to_globals).post_data,
          );
        } else {
          fprintf(sfp, b"\r\n\x00" as *const u8 as *const libc::c_char);
        }
        fflush(sfp);
        /* Tried doing this unconditionally.
         * Cloudflare and nginx/1.11.5 are shocked to see SHUT_WR on non-HTTPS.
         */
        if target.protocol == P_HTTPS.as_ptr() {
          /* If we use SSL helper, keeping our end of the socket open for writing
           * makes our end (i.e. the same fd!) readable (EAGAIN instead of EOF)
           * even after child closes its copy of the fd.
           * This helps:
           */
          shutdown(fileno_unlocked(sfp), SHUT_WR as libc::c_int);
        }
        loop
        /*
         * Retrieve HTTP response line and check for "200" status code.
         */
        {
          fgets_trim_sanitize(sfp, b"  %s\n\x00" as *const u8 as *const libc::c_char);
          str = (*ptr_to_globals).wget_buf.as_mut_ptr();
          str = skip_non_whitespace(str);
          str = skip_whitespace(str);
          // FIXME: no error check
          // xatou wouldn't work: "200 OK"
          status = atoi(str);
          match status {
            0 | 100 => {
              /* eat all remaining headers */
              while !get_sanitized_hdr(sfp).is_null() {}
            }
            200 => {
              /* fall through */
              current_block = 7160558261828954880;
              break;
            }
            201 => {
              current_block = 7160558261828954880;
              break;
            }
            202 => {
              current_block = 13607574562452046888;
              break;
            }
            203 | 204 => {
              current_block = 7293184616306149889;
              break;
            }
            300 => {
              /* redirection */
              current_block = 3812947724376655173;
              break;
            }
            301 | 302 | 303 => {
              current_block = 3812947724376655173;
              break;
            }
            206 => {
              /* Partial Content */
              if !((*ptr_to_globals).beg_range != 0i32 as libc::c_long) {
                current_block = 7174816550491926890;
                break 'c_12019;
              }
              /* "Range:..." worked. Good. */
              current_block = 3812947724376655173;
              break;
            }
            _ => {
              current_block = 7174816550491926890;
              break 'c_12019;
            }
          }
        }
        match current_block {
          7160558261828954880 =>
          /* 201 Created */
          /* "The request has been fulfilled and resulted in a new resource being created" */
          /* Standard wget is reported to treat this as success */
          /* fall through */
          {
            current_block = 13607574562452046888;
          }
          _ => {}
        }
        match current_block {
          13607574562452046888 =>
          /* 202 Accepted */
          /* "The request has been accepted for processing, but the processing has not been completed" */
          /* Treat as success: fall through */
          {
            current_block = 7293184616306149889;
          }
          _ => {}
        }
        match current_block {
          7293184616306149889 =>
          /* 203 Non-Authoritative Information */
                                      /* "Use of this response code is not required and is only appropriate when the response would otherwise be 200 (OK)" */
                /* fall through */
                                      /* 204 No Content */
                                      /*
          Response 204 doesn't say "null file", it says "metadata
          has changed but data didn't":

          "10.2.5 204 No Content
          The server has fulfilled the request but does not need to return
          an entity-body, and might want to return updated metainformation.
          The response MAY include new or updated metainformation in the form
          of entity-headers, which if present SHOULD be associated with
          the requested variant.

          If the client is a user agent, it SHOULD NOT change its document
          view from that which caused the request to be sent. This response
          is primarily intended to allow input for actions to take place
          without causing a change to the user agent's active document view,
          although any new or updated metainformation SHOULD be applied
          to the document currently in the user agent's active view.

          The 204 response MUST NOT include a message-body, and thus
          is always terminated by the first empty line after the header fields."

          However, in real world it was observed that some web servers
          (e.g. Boa/0.94.14rc21) simply use code 204 when file size is zero.
          */
          {
            if (*ptr_to_globals).beg_range != 0i32 as libc::c_long {
              /* "Range:..." was not honored by the server.
               * Restart download from the beginning.
               */
              reset_beg_range_to_zero();
            }
          }
          _ => {}
        }
        loop
        /*
         * Retrieve HTTP headers.
         */
        {
          str = get_sanitized_hdr(sfp);
          if str.is_null() {
            break;
          }
          static mut keywords: [libc::c_char; 43] = [
            99, 111, 110, 116, 101, 110, 116, 45, 108, 101, 110, 103, 116, 104, 0, 116, 114, 97,
            110, 115, 102, 101, 114, 45, 101, 110, 99, 111, 100, 105, 110, 103, 0, 108, 111, 99,
            97, 116, 105, 111, 110, 0, 0,
          ];
          let mut key: smalluint = 0;
          /* get_sanitized_hdr converted "FOO:" string to lowercase */
          /* strip trailing whitespace */
          let mut s_0: *mut libc::c_char = strchrnul(str, '\u{0}' as i32).offset(-1);
          while s_0 >= str
            && (*s_0 as libc::c_int == ' ' as i32 || *s_0 as libc::c_int == '\t' as i32)
          {
            *s_0 = '\u{0}' as i32 as libc::c_char;
            s_0 = s_0.offset(-1)
          }
          key = (index_in_strings(keywords.as_ptr(), (*ptr_to_globals).wget_buf.as_mut_ptr())
            + 1i32) as smalluint;
          if key as libc::c_int == KEY_content_length as libc::c_int {
            (*ptr_to_globals).content_len =
              bb_strtoul(str, 0 as *mut *mut libc::c_char, 10i32) as off_t;
            if (*ptr_to_globals).content_len < 0i32 as libc::c_long || *bb_errno != 0 {
              bb_error_msg_and_die(
                b"content-length %s is garbage\x00" as *const u8 as *const libc::c_char,
                str,
              );
            }
            (*ptr_to_globals).got_clen = 1i32 as smallint
          } else {
            if key as libc::c_int == KEY_transfer_encoding as libc::c_int {
              if strcmp(
                str_tolower(str),
                b"chunked\x00" as *const u8 as *const libc::c_char,
              ) != 0i32
              {
                bb_error_msg_and_die(
                  b"transfer encoding \'%s\' is not supported\x00" as *const u8
                    as *const libc::c_char,
                  str,
                );
              }
              (*ptr_to_globals).chunked = 1i32 as smallint
            }
            if !(key as libc::c_int == KEY_location as libc::c_int && status >= 300i32) {
              continue;
            }
            redir_limit -= 1;
            if redir_limit == 0i32 {
              bb_simple_error_msg_and_die(
                b"too many redirections\x00" as *const u8 as *const libc::c_char,
              );
            }
            fclose(sfp);
            if *str.offset(0) as libc::c_int == '/' as i32 {
              free(redirected_path as *mut libc::c_void);
              redirected_path = xstrdup(str.offset(1));
              target.path = redirected_path;
              continue 'c_12021;
            /* lsa stays the same: it's on the same server */
            } else {
              parse_url(str, &mut target);
              if use_proxy {
                continue 'c_12021;
              }
              /* else: lsa stays the same: we use proxy */
              /* server.user remains untouched */
              free(server.allocated as *mut libc::c_void);
              server.allocated = 0 as *mut libc::c_char;
              server.protocol = target.protocol;
              server.host = target.host;
              /* strip_ipv6_scope_id(target.host); - no! */
              /* we assume remote never gives us IPv6 addr with scope id */
              server.port = target.port;
              free(lsa as *mut libc::c_void);
              break 'c_12021;
            }
          }
        }
        //		if (status >= 300)
        //			bb_error_msg_and_die("bad redirection (no Location: header from server)");
        /* For HTTP, data is pumped over the same connection */
        dfp = sfp;
        current_block = 5710330377809666066;
        break 'c_12019;
      } else {
        /*
         *  FTP session
         */
        sfp = prepare_ftp_session(&mut dfp, &mut target, lsa);
        current_block = 5710330377809666066;
        break 'c_12019;
      }
    }
  }
  match current_block {
    7174816550491926890 =>
    /* Partial Content even though we did not ask for it??? */
    /* fall through */
    {
      bb_error_msg_and_die(
        b"server returned error: %s\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).wget_buf.as_mut_ptr(),
      );
    }
    _ => {
      free(lsa as *mut libc::c_void);
      if option_mask32 & WGET_OPT_SPIDER as libc::c_int as libc::c_uint == 0 {
        if (*ptr_to_globals).output_fd < 0i32 {
          (*ptr_to_globals).output_fd =
            xopen((*ptr_to_globals).fname_out, (*ptr_to_globals).o_flags)
        }
        retrieve_file_data(dfp);
        if option_mask32 & WGET_OPT_OUTNAME as libc::c_int as libc::c_uint == 0 {
          xclose((*ptr_to_globals).output_fd);
          (*ptr_to_globals).output_fd = -1i32
        }
      } else if option_mask32 & WGET_OPT_QUIET as libc::c_int as libc::c_uint == 0 {
        fprintf(
          stderr,
          b"remote file exists\n\x00" as *const u8 as *const libc::c_char,
        );
      }
      if dfp != sfp {
        /* It's ftp. Close data connection properly */
        fclose(dfp);
        if ftpcmd(0 as *const libc::c_char, 0 as *const libc::c_char, sfp) != 226i32 {
          bb_error_msg_and_die(
            b"ftp error: %s\x00" as *const u8 as *const libc::c_char,
            (*ptr_to_globals).wget_buf.as_mut_ptr(),
          );
        }
        /* ftpcmd("QUIT", NULL, sfp); - why bother? */
      } /* use proxies if env vars are set */
      fclose(sfp); /* "User-Agent" header field */
      free(server.allocated as *mut libc::c_void);
      free(target.allocated as *mut libc::c_void);
      free(server.user as *mut libc::c_void);
      free(target.user as *mut libc::c_void);
      free(fname_out_alloc as *mut libc::c_void);
      free(redirected_path as *mut libc::c_void);
      return;
    }
  };
}
#[no_mangle]
pub unsafe extern "C" fn wget_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  static mut wget_longopts: [libc::c_char; 266] = [
    99, 111, 110, 116, 105, 110, 117, 101, 0, 0, 99, 113, 117, 105, 101, 116, 0, 0, 113, 115, 101,
    114, 118, 101, 114, 45, 114, 101, 115, 112, 111, 110, 115, 101, 0, 0, 83, 111, 117, 116, 112,
    117, 116, 45, 100, 111, 99, 117, 109, 101, 110, 116, 0, 1, 79, 111, 117, 116, 112, 117, 116,
    45, 102, 105, 108, 101, 0, 1, 111, 100, 105, 114, 101, 99, 116, 111, 114, 121, 45, 112, 114,
    101, 102, 105, 120, 0, 1, 80, 112, 114, 111, 120, 121, 0, 1, 89, 117, 115, 101, 114, 45, 97,
    103, 101, 110, 116, 0, 1, 85, 116, 105, 109, 101, 111, 117, 116, 0, 1, 84, 116, 114, 105, 101,
    115, 0, 1, 116, 104, 101, 97, 100, 101, 114, 0, 1, -1, 112, 111, 115, 116, 45, 100, 97, 116,
    97, 0, 1, -2, 115, 112, 105, 100, 101, 114, 0, 0, -3, 110, 111, 45, 99, 104, 101, 99, 107, 45,
    99, 101, 114, 116, 105, 102, 105, 99, 97, 116, 101, 0, 0, -4, 112, 97, 115, 115, 105, 118, 101,
    45, 102, 116, 112, 0, 0, -16, 110, 111, 45, 99, 97, 99, 104, 101, 0, 0, -16, 110, 111, 45, 118,
    101, 114, 98, 111, 115, 101, 0, 0, -16, 110, 111, 45, 99, 108, 111, 98, 98, 101, 114, 0, 0,
    -16, 110, 111, 45, 104, 111, 115, 116, 45, 100, 105, 114, 101, 99, 116, 111, 114, 105, 101,
    115, 0, 0, -16, 110, 111, 45, 112, 97, 114, 101, 110, 116, 0, 0, -16, 0,
  ];
  let mut headers_llist: *mut llist_t = 0 as *mut llist_t;
  let ref mut fresh6 = *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
    as *mut *mut globals);
  *fresh6 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  (*ptr_to_globals).timeout_seconds = 900i32 as libc::c_uint;
  signal(
    14i32,
    Some(alarm_handler as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  (*ptr_to_globals).proxy_flag = b"on\x00" as *const u8 as *const libc::c_char;
  (*ptr_to_globals).user_agent = b"Wget\x00" as *const u8 as *const libc::c_char;
  getopt32long(
    argv,
    b"^cqSO:o:P:Y:U:T:+t:n::\x00-1:\xff::\x00" as *const u8 as *const libc::c_char,
    wget_longopts.as_ptr(),
    &mut (*ptr_to_globals).fname_out as *mut *mut libc::c_char,
    &mut (*ptr_to_globals).fname_log as *mut *mut libc::c_char,
    &mut (*ptr_to_globals).dir_prefix as *mut *mut libc::c_char,
    &mut (*ptr_to_globals).proxy_flag as *mut *const libc::c_char,
    &mut (*ptr_to_globals).user_agent as *mut *const libc::c_char,
    &mut (*ptr_to_globals).timeout_seconds as *mut libc::c_uint,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
    &mut headers_llist as *mut *mut llist_t,
    &mut (*ptr_to_globals).post_data as *mut *mut libc::c_char,
  );
  /* option bits debug */
  argv = argv.offset(optind as isize);
  if !headers_llist.is_null() {
    let mut size: libc::c_int = 0i32;
    let mut hdr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ll: *mut llist_t = headers_llist;
    while !ll.is_null() {
      size = (size as libc::c_ulong)
        .wrapping_add(strlen((*ll).data).wrapping_add(2i32 as libc::c_ulong))
        as libc::c_int as libc::c_int;
      ll = (*ll).link
    }
    hdr = xmalloc((size + 1i32) as size_t) as *mut libc::c_char;
    (*ptr_to_globals).extra_headers = hdr;
    while !headers_llist.is_null() {
      let mut bit: libc::c_int = 0;
      let mut words: *const libc::c_char = 0 as *const libc::c_char;
      size = sprintf(
        hdr,
        b"%s\r\n\x00" as *const u8 as *const libc::c_char,
        llist_pop(&mut headers_llist) as *mut libc::c_char,
      );
      /* a bit like index_in_substrings but don't match full key */
      bit = 1i32;
      words = wget_user_headers.as_ptr();
      while *words != 0 {
        if strstr(hdr, words) == hdr {
          (*ptr_to_globals).user_headers =
            ((*ptr_to_globals).user_headers as libc::c_int | bit) as libc::c_uchar;
          break;
        } else {
          bit <<= 1i32;
          words = words.offset(strlen(words).wrapping_add(1i32 as libc::c_ulong) as isize)
        }
      }
      hdr = hdr.offset(size as isize)
    }
  }
  (*ptr_to_globals).output_fd = -1i32;
  (*ptr_to_globals).o_flags = 0o1i32 | 0o100i32 | 0o1000i32 | 0o200i32;
  if !(*ptr_to_globals).fname_out.is_null() {
    /* -O FILE ? */
    if *(*ptr_to_globals).fname_out.offset(0) as libc::c_int == '-' as i32
      && *(*ptr_to_globals).fname_out.offset(1) == 0
    {
      /* -O - ? */
      (*ptr_to_globals).output_fd = 1i32;
      option_mask32 &= !(WGET_OPT_CONTINUE as libc::c_int) as libc::c_uint
    }
    /* compat with wget: -O FILE can overwrite */
    (*ptr_to_globals).o_flags = 0o1i32 | 0o100i32 | 0o1000i32
  }
  (*ptr_to_globals).log_fd = -1i32;
  if !(*ptr_to_globals).fname_log.is_null() {
    /* -o FILE ? */
    if !(*(*ptr_to_globals).fname_log.offset(0) as libc::c_int == '-' as i32
      && *(*ptr_to_globals).fname_log.offset(1) == 0)
    {
      /* not -o - ? */
      /* compat with wget: -o FILE can overwrite */
      (*ptr_to_globals).log_fd = xopen((*ptr_to_globals).fname_log, 0o1i32 | 0o100i32 | 0o1000i32);
      /* Redirect only stderr to log file, so -O - will work */
      xdup2((*ptr_to_globals).log_fd, 2i32);
    }
  }
  while !(*argv).is_null() {
    let fresh7 = argv;
    argv = argv.offset(1);
    download_one_url(*fresh7);
  }
  if (*ptr_to_globals).output_fd >= 0i32 {
    xclose((*ptr_to_globals).output_fd);
  }
  if (*ptr_to_globals).log_fd >= 0i32 {
    xclose((*ptr_to_globals).log_fd);
  }
  return 0i32;
}
