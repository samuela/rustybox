use crate::networking::tls::tls_handshake_data;
use c2rust_bitfields::BitfieldStruct;
pub mod signal;

// These bits all seem to be from c2rust...
// See https://pubs.opengroup.org/onlinepubs/7908799/xsh/systypes.h.html.

// Ok, so libc thinks that size_t is usize which totally makes sense but does
// not jive at all with the output from c2rust. Changing this would not be a
// refactor for the faint of heart.
pub type size_t = libc::c_ulong;

// These don't seem to exist in libc.
pub type smallint = libc::c_schar;
pub type uoff_t = libc::c_ulong;
pub type __syscall_slong_t = libc::c_long;
pub type __compar_fn_t =
  Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int>;

// ... end c2rust stuff

// Defined in libbb.h

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bb_uidgid_t {
  pub uid: libc::uid_t,
  pub gid: libc::gid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bb_progress_t {
  pub last_size: libc::c_uint,
  pub last_update_sec: libc::c_uint,
  pub last_change_sec: libc::c_uint,
  pub start_sec: libc::c_uint,
  pub curfile: *const libc::c_char,
}

// TODO: probably not as readable as a rust tuple.
/* In this form code with pipes is much more readable */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fd_pair {
  pub rd: libc::c_int,
  pub wr: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct md5_ctx_t {
  pub wbuffer: [u8; 64], /* always correctly aligned for uint64_t */
  pub process_block: Option<unsafe extern "C" fn(_: *mut md5_ctx_t) -> ()>,
  pub total64: u64,   /* must be directly before hash[] */
  pub hash: [u32; 8], /* 4 elements for md5, 5 for sha1, 8 for sha256 */
}
pub type sha1_ctx_t = md5_ctx_t;
pub type sha256_ctx_t = md5_ctx_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sha512_ctx_t {
  pub total64: [u64; 2], /* must be directly before hash[] */
  pub hash: [u64; 8],
  pub wbuffer: [u8; 128], /* always correctly aligned for uint64_t */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sha3_ctx_t {
  pub state: [u64; 25],
  pub bytes_queued: libc::c_uint,
  pub input_block_bytes: libc::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct suffix_mult {
  pub suffix: [libc::c_char; 4],
  pub mult: libc::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uni_stat_t {
  pub byte_count: libc::c_uint,
  pub unicode_count: libc::c_uint,
  pub unicode_width: libc::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct line_input_t {
  pub flags: libc::c_int,
  pub timeout: libc::c_int,
  pub path_lookup: *const libc::c_char,
  pub cnt_history: libc::c_int,
  pub cur_history: libc::c_int,
  pub max_history: libc::c_int,
  pub cnt_history_in_file: libc::c_uint,
  pub hist_file: *const libc::c_char,
  pub history: [*mut libc::c_char; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smaprec {
  pub mapped_rw: libc::c_ulong,
  pub mapped_ro: libc::c_ulong,
  pub shared_clean: libc::c_ulong,
  pub shared_dirty: libc::c_ulong,
  pub private_clean: libc::c_ulong,
  pub private_dirty: libc::c_ulong,
  pub stack: libc::c_ulong,
  pub smap_pss: libc::c_ulong,
  pub smap_swap: libc::c_ulong,
  pub smap_size: libc::c_ulong,
  pub smap_start: libc::c_ulonglong,
  pub smap_mode: [libc::c_char; 5],
  pub smap_name: *mut libc::c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct procps_status_t {
  pub dir: *mut libc::DIR,
  pub task_dir: *mut libc::DIR,
  pub shift_pages_to_bytes: u8,
  pub shift_pages_to_kb: u8,
  pub argv_len: u16,
  pub argv0: *mut libc::c_char,
  pub exe: *mut libc::c_char,
  pub main_thread_pid: libc::c_uint,
  pub vsz: libc::c_ulong,
  pub rss: libc::c_ulong,
  pub stime: libc::c_ulong,
  pub utime: libc::c_ulong,
  pub start_time: libc::c_ulong,
  pub pid: libc::c_uint,
  pub ppid: libc::c_uint,
  pub pgid: libc::c_uint,
  pub sid: libc::c_uint,
  pub uid: libc::c_uint,
  pub gid: libc::c_uint,
  pub ruid: libc::c_uint,
  pub rgid: libc::c_uint,
  pub niceness: libc::c_int,
  pub tty_major: libc::c_uint,
  pub tty_minor: libc::c_uint,
  pub smaps: smaprec,
  pub state: [libc::c_char; 4],
  pub comm: [libc::c_char; 16],
  pub last_seen_on_cpu: libc::c_int,
}

pub type socklen_t = libc::c_uint;

// Couldn't use the libc version because it produces this error:
// https://github.com/rust-lang/rust/issues/67383.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
  pub __in6_u: in6_addr_payload,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union in6_addr_payload {
  pub __u6_addr8: [u8; 16],
  pub __u6_addr16: [u16; 8],
  pub __u6_addr32: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union len_and_sockaddr_u {
  pub sa: libc::sockaddr,
  pub sin: libc::sockaddr_in,
  pub sin6: libc::sockaddr_in6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct len_and_sockaddr {
  pub len: socklen_t,
  pub u: len_and_sockaddr_u,
}

/* This structure defines protocol families and their handlers. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aftype {
  pub name: *const libc::c_char,
  pub title: *const libc::c_char,
  pub af: libc::c_int,
  pub alen: libc::c_int,
  pub print: Option<unsafe extern "C" fn(_: *mut libc::c_uchar) -> *mut libc::c_char>,
  pub sprint:
    Option<unsafe extern "C" fn(_: *mut libc::sockaddr, _: libc::c_int) -> *const libc::c_char>,
  pub input:
    Option<unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::sockaddr) -> libc::c_int>,
  pub herror: Option<unsafe extern "C" fn(_: *mut libc::c_char) -> ()>,
  pub rprint: Option<unsafe extern "C" fn(_: libc::c_int) -> libc::c_int>,
  pub rinput: Option<
    unsafe extern "C" fn(_: libc::c_int, _: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
  >,
  pub getmask: Option<
    unsafe extern "C" fn(
      _: *mut libc::c_char,
      _: *mut libc::sockaddr,
      _: *mut libc::c_char,
    ) -> libc::c_int,
  >,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwtype {
  pub name: *const libc::c_char,
  pub title: *const libc::c_char,
  pub type_0: libc::c_int,
  pub alen: libc::c_int,
  pub print: Option<unsafe extern "C" fn(_: *mut libc::c_uchar) -> *mut libc::c_char>,
  pub input:
    Option<unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::sockaddr) -> libc::c_int>,
  pub activate: Option<unsafe extern "C" fn(_: libc::c_int) -> libc::c_int>,
  pub suppress_null_addr: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tls_aes {
  pub key: [u32; 60],
  pub rounds: libc::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
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
// ... end libbb.h stuff

// Originally in shadow_.h
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spwd {
  pub sp_namp: *mut libc::c_char, /* Login name */
  pub sp_pwdp: *mut libc::c_char, /* Encrypted password */
  pub sp_lstchg: libc::c_long,    /* Date of last change */
  pub sp_min: libc::c_long,       /* Minimum number of days between changes */
  pub sp_max: libc::c_long,       /* Maximum number of days between changes */
  pub sp_warn: libc::c_long,      /* Number of days to warn user to change the password */
  pub sp_inact: libc::c_long,     /* Number of days the account may be inactive */
  pub sp_expire: libc::c_long,    /* Number of days since 1970-01-01 until account expires */
  pub sp_flag: libc::c_ulong,     /* Reserved */
}

// This is a GNU/POSIX thing: http://web.mit.edu/gnu/doc/html/regex_7.html.
pub type reg_syntax_t = libc::c_ulong;
#[repr(C)]
#[derive(Copy, Clone, BitfieldStruct)]
pub struct re_pattern_buffer {
  pub buffer: *mut libc::c_uchar,
  pub allocated: libc::c_ulong,
  pub used: libc::c_ulong,
  pub syntax: reg_syntax_t,
  pub fastmap: *mut libc::c_char,
  pub translate: *mut libc::c_uchar,
  pub re_nsub: size_t,
  #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
  #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
  #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
  #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
  #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
  #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
  #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
  pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
  #[bitfield(padding)]
  pub c2rust_padding: [u8; 7],
}

// This seems to be a linux specific thing: http://man7.org/linux/man-pages/man7/rtnetlink.7.html.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtattr {
  pub rta_len: libc::c_ushort,
  pub rta_type: libc::c_ushort,
}
