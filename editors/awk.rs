use c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;
use libc;

extern "C" {
  /* Macros for min/max.  */
  /* buffer allocation schemes */
  /* glibc uses __errno_location() to get a ptr to errno */
  /* We can just memorize it once - no multithreading in busybox :) */
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn skip_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xstrndup(s: *const libc::c_char, n: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_process_escape_sequence(ptr: *mut *const libc::c_char) -> libc::c_char;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn xfopen(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
  /* "Opens" stdin if filename is special, else just opens file: */
  #[no_mangle]
  fn xfopen_stdin(filename: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn itoa(n: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn llist_pop(elm: *mut *mut llist_t) -> *mut libc::c_void;
  #[no_mangle]
  static mut stdin: *mut FILE;
  #[no_mangle]
  static mut stdout: *mut FILE;
  #[no_mangle]
  static mut stderr: *mut FILE;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fflush(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn snprintf(
    _: *mut libc::c_char,
    _: libc::c_ulong,
    _: *const libc::c_char,
    _: ...
  ) -> libc::c_int;
  #[no_mangle]
  fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  static ptr_to_globals: *mut globals;
  #[no_mangle]
  fn strtod(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char) -> libc::c_double;
  #[no_mangle]
  fn strtoull(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_ulonglong;
  #[no_mangle]
  fn rand() -> libc::c_int;
  #[no_mangle]
  fn srand(__seed: libc::c_uint);
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn exit(_: libc::c_int) -> !;
  #[no_mangle]
  fn system(__command: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
  #[no_mangle]
  fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn strerror(_: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
  #[no_mangle]
  fn time(__timer: *mut time_t) -> time_t;
  #[no_mangle]
  fn mktime(__tp: *mut tm) -> time_t;
  #[no_mangle]
  fn strftime(
    __s: *mut libc::c_char,
    __maxsize: size_t,
    __format: *const libc::c_char,
    __tp: *const tm,
  ) -> size_t;
  #[no_mangle]
  fn localtime(__timer: *const time_t) -> *mut tm;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut environ: *mut *mut libc::c_char;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_simple_error_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t, __stream: *mut FILE) -> size_t;
  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn fileno_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn pclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn regfree(__preg: *mut regex_t);
  #[no_mangle]
  fn xregcomp(preg: *mut regex_t, regex: *const libc::c_char, cflags: libc::c_int);
  #[no_mangle]
  fn regcomp(
    __preg: *mut regex_t,
    __pattern: *const libc::c_char,
    __cflags: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn regexec(
    __preg: *const regex_t,
    __string: *const libc::c_char,
    __nmatch: size_t,
    __pmatch: *mut regmatch_t,
    __eflags: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
  #[no_mangle]
  fn cos(_: libc::c_double) -> libc::c_double;
  #[no_mangle]
  fn sin(_: libc::c_double) -> libc::c_double;
  #[no_mangle]
  fn exp(_: libc::c_double) -> libc::c_double;
  #[no_mangle]
  fn log(_: libc::c_double) -> libc::c_double;
  #[no_mangle]
  fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
  #[no_mangle]
  fn sqrt(_: libc::c_double) -> libc::c_double;
}
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type smallint = libc::c_schar;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;



use crate::librb::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
  pub tm_sec: libc::c_int,
  pub tm_min: libc::c_int,
  pub tm_hour: libc::c_int,
  pub tm_mday: libc::c_int,
  pub tm_mon: libc::c_int,
  pub tm_year: libc::c_int,
  pub tm_wday: libc::c_int,
  pub tm_yday: libc::c_int,
  pub tm_isdst: libc::c_int,
  pub tm_gmtoff: libc::c_long,
  pub tm_zone: *const libc::c_char,
}
use crate::libbb::llist::llist_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub t_double: libc::c_double,
  pub beginseq: chain,
  pub mainseq: chain,
  pub endseq: chain,
  pub seq: *mut chain,
  pub break_ptr: *mut node,
  pub continue_ptr: *mut node,
  pub iF: *mut rstream,
  pub vhash: *mut xhash,
  pub ahash: *mut xhash,
  pub fdhash: *mut xhash,
  pub fnhash: *mut xhash,
  pub g_progname: *const libc::c_char,
  pub g_lineno: libc::c_int,
  pub nfields: libc::c_int,
  pub maxfields: libc::c_int,
  pub Fields: *mut var,
  pub g_cb: *mut nvblock,
  pub g_pos: *mut libc::c_char,
  pub g_buf: *mut libc::c_char,
  pub icase: smallint,
  pub exiting: smallint,
  pub nextrec: smallint,
  pub nextfile: smallint,
  pub is_f0_split: smallint,
  pub t_rollback: smallint,
}
pub type nvblock = nvblock_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nvblock_s {
  pub size: libc::c_int,
  pub pos: *mut var,
  pub prev: *mut nvblock_s,
  pub next: *mut nvblock_s,
  pub nv: [var; 0],
}
pub type var = var_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct var_s {
  pub type_0: libc::c_uint,
  pub number: libc::c_double,
  pub string: *mut libc::c_char,
  pub x: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
  pub aidx: libc::c_int,
  pub array: *mut xhash_s,
  pub parent: *mut var_s,
  pub walker: *mut walker_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct walker_list {
  pub end: *mut libc::c_char,
  pub cur: *mut libc::c_char,
  pub prev: *mut walker_list,
  pub wbuf: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xhash_s {
  pub nel: libc::c_uint,
  pub csize: libc::c_uint,
  pub nprime: libc::c_uint,
  pub glen: libc::c_uint,
  pub items: *mut *mut hash_item_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_item_s {
  pub data: C2RustUnnamed_0,
  pub next: *mut hash_item_s,
  pub name: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
  pub v: var_s,
  pub rs: rstream_s,
  pub f: func_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct func_s {
  pub nargs: libc::c_uint,
  pub body: chain_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chain_s {
  pub first: *mut node_s,
  pub last: *mut node_s,
  pub programname: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_s {
  pub info: uint32_t,
  pub lineno: libc::c_uint,
  pub l: C2RustUnnamed_3,
  pub r: C2RustUnnamed_2,
  pub a: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
  pub n: *mut node_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
  pub n: *mut node_s,
  pub ire: *mut regex_t,
  pub f: *mut func,
}
pub type func = func_s;
pub type regex_t = re_pattern_buffer;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
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
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
  pub n: *mut node_s,
  pub v: *mut var,
  pub aidx: libc::c_int,
  pub new_progname: *mut libc::c_char,
  pub re: *mut regex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rstream_s {
  pub F: *mut FILE,
  pub buffer: *mut libc::c_char,
  pub adv: libc::c_int,
  pub size: libc::c_int,
  pub pos: libc::c_int,
  pub is_pipe: smallint,
}
pub type xhash = xhash_s;
pub type rstream = rstream_s;
pub type node = node_s;
pub type chain = chain_s;
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
  pub rm_so: regoff_t,
  pub rm_eo: regoff_t,
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const OPT_W: C2RustUnnamed_4 = 16;
pub const OPT_e: C2RustUnnamed_4 = 8;
pub const OPT_f: C2RustUnnamed_4 = 4;
pub const OPT_v: C2RustUnnamed_4 = 2;
pub const OPT_F: C2RustUnnamed_4 = 1;
pub const OPTBIT_W: C2RustUnnamed_4 = 4;
pub const OPTBIT_e: C2RustUnnamed_4 = 3;
pub const OPTBIT_f: C2RustUnnamed_4 = 2;
pub const OPTBIT_v: C2RustUnnamed_4 = 1;
pub const OPTBIT_F: C2RustUnnamed_4 = 0;
pub type hash_item = hash_item_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tsplitter_s {
  pub n: node,
  pub re: [regex_t; 2],
}
pub type tsplitter = tsplitter_s;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const ST_WHILE: C2RustUnnamed_5 = 13056;
pub const ST_FOR: C2RustUnnamed_5 = 12800;
pub const ST_DO: C2RustUnnamed_5 = 12544;
pub const ST_IF: C2RustUnnamed_5 = 12288;
pub const OC_DONE: C2RustUnnamed_5 = 10240;
pub const OC_VAR: C2RustUnnamed_5 = 9984;
pub const OC_UNARY: C2RustUnnamed_5 = 9728;
pub const OC_TERNARY: C2RustUnnamed_5 = 9472;
pub const OC_SPRINTF: C2RustUnnamed_5 = 9216;
pub const OC_RETURN: C2RustUnnamed_5 = 8960;
pub const OC_REPLACE: C2RustUnnamed_5 = 8704;
pub const OC_REGEXP: C2RustUnnamed_5 = 8448;
pub const OC_PGETLINE: C2RustUnnamed_5 = 8192;
pub const OC_MOVE: C2RustUnnamed_5 = 7936;
pub const OC_MATCH: C2RustUnnamed_5 = 7680;
pub const OC_LOR: C2RustUnnamed_5 = 7424;
pub const OC_LAND: C2RustUnnamed_5 = 7168;
pub const OC_IN: C2RustUnnamed_5 = 6912;
pub const OC_GETLINE: C2RustUnnamed_5 = 6656;
pub const OC_FUNC: C2RustUnnamed_5 = 6400;
pub const OC_FNARG: C2RustUnnamed_5 = 6144;
pub const OC_FIELD: C2RustUnnamed_5 = 5888;
pub const OC_FBLTIN: C2RustUnnamed_5 = 5632;
pub const OC_CONCAT: C2RustUnnamed_5 = 5376;
pub const OC_COMPARE: C2RustUnnamed_5 = 5120;
pub const OC_COMMA: C2RustUnnamed_5 = 4864;
pub const OC_COLON: C2RustUnnamed_5 = 4608;
pub const OC_BUILTIN: C2RustUnnamed_5 = 4352;
pub const OC_BINARY: C2RustUnnamed_5 = 4096;
pub const OC_WALKNEXT: C2RustUnnamed_5 = 3584;
pub const OC_TEST: C2RustUnnamed_5 = 3328;
pub const OC_NEXTFILE: C2RustUnnamed_5 = 3072;
pub const OC_NEXT: C2RustUnnamed_5 = 2816;
pub const OC_EXIT: C2RustUnnamed_5 = 2560;
pub const OC_CONTINUE: C2RustUnnamed_5 = 2304;
pub const OC_BREAK: C2RustUnnamed_5 = 2048;
pub const OC_BR: C2RustUnnamed_5 = 1792;
pub const OC_WALKINIT: C2RustUnnamed_5 = 1536;
pub const OC_PRINTF: C2RustUnnamed_5 = 1280;
pub const OC_PRINT: C2RustUnnamed_5 = 1024;
pub const OC_NEWSOURCE: C2RustUnnamed_5 = 768;
pub const OC_EXEC: C2RustUnnamed_5 = 512;
pub const OC_DELETE: C2RustUnnamed_5 = 256;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const F_cl: C2RustUnnamed_6 = 12;
pub const F_ff: C2RustUnnamed_6 = 11;
pub const F_sy: C2RustUnnamed_6 = 10;
pub const F_le: C2RustUnnamed_6 = 9;
pub const F_ti: C2RustUnnamed_6 = 8;
pub const F_sr: C2RustUnnamed_6 = 7;
pub const F_sq: C2RustUnnamed_6 = 6;
pub const F_si: C2RustUnnamed_6 = 5;
pub const F_lg: C2RustUnnamed_6 = 4;
pub const F_ex: C2RustUnnamed_6 = 3;
pub const F_co: C2RustUnnamed_6 = 2;
pub const F_rn: C2RustUnnamed_6 = 1;
pub const F_in: C2RustUnnamed_6 = 0;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const B_xo: C2RustUnnamed_7 = 17;
pub const B_rs: C2RustUnnamed_7 = 16;
pub const B_or: C2RustUnnamed_7 = 15;
pub const B_ls: C2RustUnnamed_7 = 14;
pub const B_co: C2RustUnnamed_7 = 13;
pub const B_an: C2RustUnnamed_7 = 12;
pub const B_su: C2RustUnnamed_7 = 11;
pub const B_gs: C2RustUnnamed_7 = 10;
pub const B_ge: C2RustUnnamed_7 = 9;
pub const B_up: C2RustUnnamed_7 = 8;
pub const B_lo: C2RustUnnamed_7 = 7;
pub const B_mt: C2RustUnnamed_7 = 6;
pub const B_ti: C2RustUnnamed_7 = 5;
pub const B_ss: C2RustUnnamed_7 = 4;
pub const B_sp: C2RustUnnamed_7 = 3;
pub const B_ma: C2RustUnnamed_7 = 2;
pub const B_ix: C2RustUnnamed_7 = 1;
pub const B_a2: C2RustUnnamed_7 = 0;
pub type C2RustUnnamed_8 = libc::c_uint;
pub const NUM_INTERNAL_VARS: C2RustUnnamed_8 = 19;
pub const ENVIRON: C2RustUnnamed_8 = 18;
pub const IGNORECASE: C2RustUnnamed_8 = 17;
pub const NF: C2RustUnnamed_8 = 16;
pub const NR: C2RustUnnamed_8 = 15;
pub const FNR: C2RustUnnamed_8 = 14;
pub const ERRNO: C2RustUnnamed_8 = 13;
pub const ARGV: C2RustUnnamed_8 = 12;
pub const ARGC: C2RustUnnamed_8 = 11;
pub const ARGIND: C2RustUnnamed_8 = 10;
pub const F0: C2RustUnnamed_8 = 9;
pub const SUBSEP: C2RustUnnamed_8 = 8;
pub const FILENAME: C2RustUnnamed_8 = 7;
pub const RT: C2RustUnnamed_8 = 6;
pub const RS: C2RustUnnamed_8 = 5;
pub const ORS: C2RustUnnamed_8 = 4;
pub const OFS: C2RustUnnamed_8 = 3;
pub const FS: C2RustUnnamed_8 = 2;
pub const OFMT: C2RustUnnamed_8 = 1;
pub const CONVFMT: C2RustUnnamed_8 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals2 {
  pub t_info: uint32_t,
  pub t_tclass: uint32_t,
  pub t_string: *mut libc::c_char,
  pub t_lineno: libc::c_int,
  pub intvar: [*mut var; 19],
  pub split_f0__fstrings: *mut libc::c_char,
  pub next_token__save_tclass: uint32_t,
  pub next_token__save_info: uint32_t,
  pub next_token__ltclass: uint32_t,
  pub next_token__concat_inserted: smallint,
  pub next_input_file__files_happen: smallint,
  pub next_input_file__rsm: rstream,
  pub evaluate__fnargs: *mut var,
  pub evaluate__seed: libc::c_uint,
  pub evaluate__sreg: regex_t,
  pub ptest__v: var,
  pub exec_builtin__tspl: tsplitter,
  pub fsplitter: tsplitter,
  pub rsplitter: tsplitter,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
  pub v: *mut var,
  pub s: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
  pub v: *mut var,
  pub s: *const libc::c_char,
}
#[inline(always)]
unsafe extern "C" fn bb_ascii_tolower(mut a: libc::c_uchar) -> libc::c_uchar {
  let mut b: libc::c_uchar = (a as libc::c_int - 'A' as i32) as libc::c_uchar;
  if b as libc::c_int <= 'Z' as i32 - 'A' as i32 {
    a = (a as libc::c_int + ('a' as i32 - 'A' as i32)) as libc::c_uchar
  }
  return a;
}
#[inline(always)]
unsafe extern "C" fn bb_ascii_toupper(mut a: libc::c_uchar) -> libc::c_uchar {
  let mut b: libc::c_uchar = (a as libc::c_int - 'a' as i32) as libc::c_uchar;
  if b as libc::c_int <= 'z' as i32 - 'a' as i32 {
    a = (a as libc::c_int - ('a' as i32 - 'A' as i32)) as libc::c_uchar
  }
  return a;
}
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
static mut tokenlist: [libc::c_char; 448] = [
  1, 40, -1, 1, 41, -1, 1, 47, -1, 2, 62, 62, 1, 62, 1, 124, -1, 2, 43, 43, 2, 45, 45, -1, 2, 43,
  43, 2, 45, 45, 1, 36, -1, 2, 61, 61, 1, 61, 2, 43, 61, 2, 45, 61, 2, 42, 61, 2, 47, 61, 2, 37,
  61, 2, 94, 61, 1, 43, 1, 45, 3, 42, 42, 61, 2, 42, 42, 1, 47, 1, 37, 1, 94, 1, 42, 2, 33, 61, 2,
  62, 61, 2, 60, 61, 1, 62, 1, 60, 2, 33, 126, 1, 126, 2, 38, 38, 2, 124, 124, 1, 63, 1, 58, -1, 2,
  105, 110, -1, 1, 44, -1, 1, 124, -1, 1, 43, 1, 45, 1, 33, -1, 1, 93, -1, 1, 123, -1, 1, 125, -1,
  1, 59, -1, 1, 10, -1, 2, 105, 102, 2, 100, 111, 3, 102, 111, 114, 5, 98, 114, 101, 97, 107, 8,
  99, 111, 110, 116, 105, 110, 117, 101, 6, 100, 101, 108, 101, 116, 101, 5, 112, 114, 105, 110,
  116, 6, 112, 114, 105, 110, 116, 102, 4, 110, 101, 120, 116, 8, 110, 101, 120, 116, 102, 105,
  108, 101, 6, 114, 101, 116, 117, 114, 110, 4, 101, 120, 105, 116, -1, 5, 119, 104, 105, 108, 101,
  -1, 4, 101, 108, 115, 101, -1, 3, 97, 110, 100, 5, 99, 111, 109, 112, 108, 6, 108, 115, 104, 105,
  102, 116, 2, 111, 114, 6, 114, 115, 104, 105, 102, 116, 3, 120, 111, 114, 5, 99, 108, 111, 115,
  101, 6, 115, 121, 115, 116, 101, 109, 6, 102, 102, 108, 117, 115, 104, 5, 97, 116, 97, 110, 50,
  3, 99, 111, 115, 3, 101, 120, 112, 3, 105, 110, 116, 3, 108, 111, 103, 4, 114, 97, 110, 100, 3,
  115, 105, 110, 4, 115, 113, 114, 116, 5, 115, 114, 97, 110, 100, 6, 103, 101, 110, 115, 117, 98,
  4, 103, 115, 117, 98, 5, 105, 110, 100, 101, 120, 5, 109, 97, 116, 99, 104, 5, 115, 112, 108,
  105, 116, 7, 115, 112, 114, 105, 110, 116, 102, 3, 115, 117, 98, 6, 115, 117, 98, 115, 116, 114,
  7, 115, 121, 115, 116, 105, 109, 101, 8, 115, 116, 114, 102, 116, 105, 109, 101, 6, 109, 107,
  116, 105, 109, 101, 7, 116, 111, 108, 111, 119, 101, 114, 7, 116, 111, 117, 112, 112, 101, 114,
  -1, 6, 108, 101, 110, 103, 116, 104, -1, 7, 103, 101, 116, 108, 105, 110, 101, -1, 4, 102, 117,
  110, 99, 8, 102, 117, 110, 99, 116, 105, 111, 110, -1, 5, 66, 69, 71, 73, 78, -1, 3, 69, 78, 68,
  0,
];
static mut tokeninfo: [uint32_t; 100] = [
  0i32 as uint32_t,
  0i32 as uint32_t,
  OC_REGEXP as libc::c_int as uint32_t,
  (0x20000i32 | 0x80000i32 | 'a' as i32) as uint32_t,
  (0x20000i32 | 0x80000i32 | 'w' as i32) as uint32_t,
  (0x20000i32 | 0x80000i32 | '|' as i32) as uint32_t,
  (OC_UNARY as libc::c_int | 0x20000i32 | 9i32 << 24i32 | 'p' as i32) as uint32_t,
  (OC_UNARY as libc::c_int | 0x20000i32 | 9i32 << 24i32 | 'm' as i32) as uint32_t,
  (OC_UNARY as libc::c_int | 0x20000i32 | 9i32 << 24i32 | 'P' as i32) as uint32_t,
  (OC_UNARY as libc::c_int | 0x20000i32 | 9i32 << 24i32 | 'M' as i32) as uint32_t,
  (OC_FIELD as libc::c_int | 0x20000i32 | 5i32 << 24i32) as uint32_t,
  (OC_COMPARE as libc::c_int | (0x10000i32 | 0x20000i32) | 39i32 << 24i32 | 5i32) as uint32_t,
  (OC_MOVE as libc::c_int | (0x10000i32 | 0x20000i32) | 74i32 << 24i32) as uint32_t,
  (OC_REPLACE as libc::c_int
    | (0x10000i32 | 0x100000i32 | 0x20000i32)
    | 74i32 << 24i32
    | '+' as i32) as uint32_t,
  (OC_REPLACE as libc::c_int
    | (0x10000i32 | 0x100000i32 | 0x20000i32)
    | 74i32 << 24i32
    | '-' as i32) as uint32_t,
  (OC_REPLACE as libc::c_int
    | (0x10000i32 | 0x100000i32 | 0x20000i32)
    | 74i32 << 24i32
    | '*' as i32) as uint32_t,
  (OC_REPLACE as libc::c_int
    | (0x10000i32 | 0x100000i32 | 0x20000i32)
    | 74i32 << 24i32
    | '/' as i32) as uint32_t,
  (OC_REPLACE as libc::c_int
    | (0x10000i32 | 0x100000i32 | 0x20000i32)
    | 74i32 << 24i32
    | '%' as i32) as uint32_t,
  (OC_REPLACE as libc::c_int
    | (0x10000i32 | 0x100000i32 | 0x20000i32)
    | 74i32 << 24i32
    | '&' as i32) as uint32_t,
  (OC_BINARY as libc::c_int | (0x10000i32 | 0x100000i32 | 0x20000i32) | 29i32 << 24i32 | '+' as i32)
    as uint32_t,
  (OC_BINARY as libc::c_int | (0x10000i32 | 0x100000i32 | 0x20000i32) | 29i32 << 24i32 | '-' as i32)
    as uint32_t,
  (OC_REPLACE as libc::c_int
    | (0x10000i32 | 0x100000i32 | 0x20000i32)
    | 74i32 << 24i32
    | '&' as i32) as uint32_t,
  (OC_BINARY as libc::c_int | (0x10000i32 | 0x100000i32 | 0x20000i32) | 15i32 << 24i32 | '&' as i32)
    as uint32_t,
  (OC_BINARY as libc::c_int | (0x10000i32 | 0x100000i32 | 0x20000i32) | 25i32 << 24i32 | '/' as i32)
    as uint32_t,
  (OC_BINARY as libc::c_int | (0x10000i32 | 0x100000i32 | 0x20000i32) | 25i32 << 24i32 | '%' as i32)
    as uint32_t,
  (OC_BINARY as libc::c_int | (0x10000i32 | 0x100000i32 | 0x20000i32) | 15i32 << 24i32 | '&' as i32)
    as uint32_t,
  (OC_BINARY as libc::c_int | (0x10000i32 | 0x100000i32 | 0x20000i32) | 25i32 << 24i32 | '*' as i32)
    as uint32_t,
  (OC_COMPARE as libc::c_int | (0x10000i32 | 0x20000i32) | 39i32 << 24i32 | 4i32) as uint32_t,
  (OC_COMPARE as libc::c_int | (0x10000i32 | 0x20000i32) | 39i32 << 24i32 | 3i32) as uint32_t,
  (OC_COMPARE as libc::c_int | (0x10000i32 | 0x20000i32) | 39i32 << 24i32 | 0i32) as uint32_t,
  (OC_COMPARE as libc::c_int | (0x10000i32 | 0x20000i32) | 39i32 << 24i32 | 1i32) as uint32_t,
  (OC_COMPARE as libc::c_int | (0x10000i32 | 0x20000i32) | 39i32 << 24i32 | 2i32) as uint32_t,
  (OC_MATCH as libc::c_int | (0x10000i32 | 0x40000i32) | 45i32 << 24i32 | '!' as i32) as uint32_t,
  (OC_MATCH as libc::c_int | (0x10000i32 | 0x40000i32) | 45i32 << 24i32 | '~' as i32) as uint32_t,
  (OC_LAND as libc::c_int | 0x10000i32 | 55i32 << 24i32) as uint32_t,
  (OC_LOR as libc::c_int | 0x10000i32 | 59i32 << 24i32) as uint32_t,
  (OC_TERNARY as libc::c_int | 0x10000i32 | 64i32 << 24i32 | '?' as i32) as uint32_t,
  (OC_COLON as libc::c_int | 0i32 | 67i32 << 24i32 | ':' as i32) as uint32_t,
  (OC_IN as libc::c_int | (0x10000i32 | 0x40000i32 | 0x20000i32) | 49i32 << 24i32) as uint32_t,
  (OC_COMMA as libc::c_int | (0x10000i32 | 0x40000i32 | 0x20000i32 | 0x80000i32) | 80i32 << 24i32)
    as uint32_t,
  (OC_PGETLINE as libc::c_int | (0x10000i32 | 0x40000i32 | 0x20000i32) | 37i32 << 24i32)
    as uint32_t,
  (OC_UNARY as libc::c_int | 0x20000i32 | 19i32 << 24i32 | '+' as i32) as uint32_t,
  (OC_UNARY as libc::c_int | 0x20000i32 | 19i32 << 24i32 | '-' as i32) as uint32_t,
  (OC_UNARY as libc::c_int | 0x20000i32 | 19i32 << 24i32 | '!' as i32) as uint32_t,
  0i32 as uint32_t,
  0i32 as uint32_t,
  0i32 as uint32_t,
  0i32 as uint32_t,
  0i32 as uint32_t,
  ST_IF as libc::c_int as uint32_t,
  ST_DO as libc::c_int as uint32_t,
  ST_FOR as libc::c_int as uint32_t,
  OC_BREAK as libc::c_int as uint32_t,
  OC_CONTINUE as libc::c_int as uint32_t,
  (OC_DELETE as libc::c_int | (0x10000i32 | 0x100000i32 | 0x400000i32)) as uint32_t,
  OC_PRINT as libc::c_int as uint32_t,
  OC_PRINTF as libc::c_int as uint32_t,
  OC_NEXT as libc::c_int as uint32_t,
  OC_NEXTFILE as libc::c_int as uint32_t,
  (OC_RETURN as libc::c_int | 0x10000i32) as uint32_t,
  (OC_EXIT as libc::c_int | (0x10000i32 | 0x100000i32)) as uint32_t,
  ST_WHILE as libc::c_int as uint32_t,
  0i32 as uint32_t,
  (OC_BUILTIN as libc::c_int | B_an as libc::c_int | 0x83i32 << 24i32) as uint32_t,
  (OC_BUILTIN as libc::c_int | B_co as libc::c_int | 0x41i32 << 24i32) as uint32_t,
  (OC_BUILTIN as libc::c_int | B_ls as libc::c_int | 0x83i32 << 24i32) as uint32_t,
  (OC_BUILTIN as libc::c_int | B_or as libc::c_int | 0x83i32 << 24i32) as uint32_t,
  (OC_BUILTIN as libc::c_int | B_rs as libc::c_int | 0x83i32 << 24i32) as uint32_t,
  (OC_BUILTIN as libc::c_int | B_xo as libc::c_int | 0x83i32 << 24i32) as uint32_t,
  (OC_FBLTIN as libc::c_int | (0x10000i32 | 0x40000i32) | F_cl as libc::c_int) as uint32_t,
  (OC_FBLTIN as libc::c_int | (0x10000i32 | 0x40000i32) | F_sy as libc::c_int) as uint32_t,
  (OC_FBLTIN as libc::c_int | (0x10000i32 | 0x40000i32) | F_ff as libc::c_int) as uint32_t,
  (OC_BUILTIN as libc::c_int | B_a2 as libc::c_int | 0x83i32 << 24i32) as uint32_t,
  (OC_FBLTIN as libc::c_int | (0x10000i32 | 0x100000i32) | F_co as libc::c_int) as uint32_t,
  (OC_FBLTIN as libc::c_int | (0x10000i32 | 0x100000i32) | F_ex as libc::c_int) as uint32_t,
  (OC_FBLTIN as libc::c_int | (0x10000i32 | 0x100000i32) | F_in as libc::c_int) as uint32_t,
  (OC_FBLTIN as libc::c_int | (0x10000i32 | 0x100000i32) | F_lg as libc::c_int) as uint32_t,
  (OC_FBLTIN as libc::c_int | F_rn as libc::c_int) as uint32_t,
  (OC_FBLTIN as libc::c_int | (0x10000i32 | 0x100000i32) | F_si as libc::c_int) as uint32_t,
  (OC_FBLTIN as libc::c_int | (0x10000i32 | 0x100000i32) | F_sq as libc::c_int) as uint32_t,
  (OC_FBLTIN as libc::c_int | (0x10000i32 | 0x100000i32) | F_sr as libc::c_int) as uint32_t,
  (OC_BUILTIN as libc::c_int | B_ge as libc::c_int | 0xd6i32 << 24i32) as uint32_t,
  (OC_BUILTIN as libc::c_int | B_gs as libc::c_int | 0xb6i32 << 24i32) as uint32_t,
  (OC_BUILTIN as libc::c_int | B_ix as libc::c_int | 0x9bi32 << 24i32) as uint32_t,
  (OC_BUILTIN as libc::c_int | B_ma as libc::c_int | 0x89i32 << 24i32) as uint32_t,
  (OC_BUILTIN as libc::c_int | B_sp as libc::c_int | 0x8bi32 << 24i32) as uint32_t,
  OC_SPRINTF as libc::c_int as uint32_t,
  (OC_BUILTIN as libc::c_int | B_su as libc::c_int | 0xb6i32 << 24i32) as uint32_t,
  (OC_BUILTIN as libc::c_int | B_ss as libc::c_int | 0x8fi32 << 24i32) as uint32_t,
  (OC_FBLTIN as libc::c_int | F_ti as libc::c_int) as uint32_t,
  (OC_BUILTIN as libc::c_int | B_ti as libc::c_int | 0xbi32 << 24i32) as uint32_t,
  (OC_BUILTIN as libc::c_int | B_mt as libc::c_int | 0xbi32 << 24i32) as uint32_t,
  (OC_BUILTIN as libc::c_int | B_lo as libc::c_int | 0x49i32 << 24i32) as uint32_t,
  (OC_BUILTIN as libc::c_int | B_up as libc::c_int | 0x49i32 << 24i32) as uint32_t,
  (OC_FBLTIN as libc::c_int | (0x10000i32 | 0x40000i32) | F_le as libc::c_int) as uint32_t,
  (OC_GETLINE as libc::c_int | (0x10000i32 | 0x40000i32 | 0x20000i32) | 0i32 << 24i32) as uint32_t,
  0i32 as uint32_t,
  0i32 as uint32_t,
  0i32 as uint32_t,
  0i32 as uint32_t,
];
static mut vNames: [libc::c_char; 107] = [
  67, 79, 78, 86, 70, 77, 84, 0, 79, 70, 77, 84, 0, 70, 83, 0, 42, 79, 70, 83, 0, 79, 82, 83, 0,
  82, 83, 0, 42, 82, 84, 0, 70, 73, 76, 69, 78, 65, 77, 69, 0, 83, 85, 66, 83, 69, 80, 0, 36, 0,
  42, 65, 82, 71, 73, 78, 68, 0, 65, 82, 71, 67, 0, 65, 82, 71, 86, 0, 69, 82, 82, 78, 79, 0, 70,
  78, 82, 0, 78, 82, 0, 78, 70, 0, 42, 73, 71, 78, 79, 82, 69, 67, 65, 83, 69, 0, 42, 69, 78, 86,
  73, 82, 79, 78, 0, 0, 0,
];
static mut vValues: [libc::c_char; 25] = [
  37, 46, 54, 103, 0, 37, 46, 54, 103, 0, 32, 0, 32, 0, 10, 0, 10, 0, 0, 0, 28, 0, 0, -1, 0,
];
static mut PRIMES: [uint16_t; 5] = [
  251i32 as uint16_t,
  1021i32 as uint16_t,
  4093i32 as uint16_t,
  16381i32 as uint16_t,
  65521i32 as uint16_t,
];
/* ---- error handling ---- */
static mut EMSG_INTERNAL_ERROR: [libc::c_char; 15] = [
  73, 110, 116, 101, 114, 110, 97, 108, 32, 101, 114, 114, 111, 114, 0,
];
static mut EMSG_UNEXP_EOS: [libc::c_char; 25] = [
  85, 110, 101, 120, 112, 101, 99, 116, 101, 100, 32, 101, 110, 100, 32, 111, 102, 32, 115, 116,
  114, 105, 110, 103, 0,
];
static mut EMSG_UNEXP_TOKEN: [libc::c_char; 17] = [
  85, 110, 101, 120, 112, 101, 99, 116, 101, 100, 32, 116, 111, 107, 101, 110, 0,
];
static mut EMSG_DIV_BY_ZERO: [libc::c_char; 17] = [
  68, 105, 118, 105, 115, 105, 111, 110, 32, 98, 121, 32, 122, 101, 114, 111, 0,
];
static mut EMSG_INV_FMT: [libc::c_char; 25] = [
  73, 110, 118, 97, 108, 105, 100, 32, 102, 111, 114, 109, 97, 116, 32, 115, 112, 101, 99, 105,
  102, 105, 101, 114, 0,
];
static mut EMSG_TOO_FEW_ARGS: [libc::c_char; 18] = [
  84, 111, 111, 32, 102, 101, 119, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 0,
];
static mut EMSG_NOT_ARRAY: [libc::c_char; 13] =
  [78, 111, 116, 32, 97, 110, 32, 97, 114, 114, 97, 121, 0];
static mut EMSG_POSSIBLE_ERROR: [libc::c_char; 22] = [
  80, 111, 115, 115, 105, 98, 108, 101, 32, 115, 121, 110, 116, 97, 120, 32, 101, 114, 114, 111,
  114, 0,
];
static mut EMSG_UNDEF_FUNC: [libc::c_char; 27] = [
  67, 97, 108, 108, 32, 116, 111, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 102, 117,
  110, 99, 116, 105, 111, 110, 0,
];
static mut EMSG_NEGATIVE_FIELD: [libc::c_char; 25] = [
  65, 99, 99, 101, 115, 115, 32, 116, 111, 32, 110, 101, 103, 97, 116, 105, 118, 101, 32, 102, 105,
  101, 108, 100, 0,
];
unsafe extern "C" fn zero_out_var(mut vp: *mut var) {
  memset(
    vp as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<var>() as libc::c_ulong,
  );
}
unsafe extern "C" fn syntax_error(mut message: *const libc::c_char) -> ! {
  bb_error_msg_and_die(
    b"%s:%i: %s\x00" as *const u8 as *const libc::c_char,
    (*ptr_to_globals.offset(-1i32 as isize)).g_progname,
    (*ptr_to_globals.offset(-1i32 as isize)).g_lineno,
    message,
  );
}
/* ---- hash stuff ---- */
unsafe extern "C" fn hashidx(mut name: *const libc::c_char) -> libc::c_uint {
  let mut idx: libc::c_uint = 0i32 as libc::c_uint;
  while *name != 0 {
    let fresh0 = name;
    name = name.offset(1);
    idx = (*fresh0 as libc::c_uint)
      .wrapping_add(idx << 6i32)
      .wrapping_sub(idx)
  }
  return idx;
}
/* create new hash */
unsafe extern "C" fn hash_init() -> *mut xhash {
  let mut newhash: *mut xhash = 0 as *mut xhash;
  newhash = xzalloc(::std::mem::size_of::<xhash>() as libc::c_ulong) as *mut xhash;
  (*newhash).csize = 61i32 as libc::c_uint;
  (*newhash).items = xzalloc(
    (61i32 as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<*mut hash_item_s>() as libc::c_ulong),
  ) as *mut *mut hash_item_s;
  return newhash;
}
/* find item in hash, return ptr to data, NULL if not found */
unsafe extern "C" fn hash_search(
  mut hash: *mut xhash,
  mut name: *const libc::c_char,
) -> *mut libc::c_void {
  let mut hi: *mut hash_item = 0 as *mut hash_item;
  hi = *(*hash)
    .items
    .offset(hashidx(name).wrapping_rem((*hash).csize) as isize);
  while !hi.is_null() {
    if strcmp((*hi).name.as_mut_ptr(), name) == 0i32 {
      return &mut (*hi).data as *mut C2RustUnnamed_0 as *mut libc::c_void;
    }
    hi = (*hi).next
  }
  return 0 as *mut libc::c_void;
}
/* grow hash if it becomes too big */
unsafe extern "C" fn hash_rebuild(mut hash: *mut xhash) {
  let mut newsize: libc::c_uint = 0;
  let mut i: libc::c_uint = 0;
  let mut idx: libc::c_uint = 0;
  let mut newitems: *mut *mut hash_item = 0 as *mut *mut hash_item;
  let mut hi: *mut hash_item = 0 as *mut hash_item;
  let mut thi: *mut hash_item = 0 as *mut hash_item;
  if (*hash).nprime
    == (::std::mem::size_of::<[uint16_t; 5]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<uint16_t>() as libc::c_ulong) as libc::c_uint
  {
    return;
  }
  let fresh1 = (*hash).nprime;
  (*hash).nprime = (*hash).nprime.wrapping_add(1);
  newsize = PRIMES[fresh1 as usize] as libc::c_uint;
  newitems = xzalloc(
    (newsize as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<*mut hash_item>() as libc::c_ulong),
  ) as *mut *mut hash_item;
  i = 0i32 as libc::c_uint;
  while i < (*hash).csize {
    hi = *(*hash).items.offset(i as isize);
    while !hi.is_null() {
      thi = hi;
      hi = (*thi).next;
      idx = hashidx((*thi).name.as_mut_ptr()).wrapping_rem(newsize);
      (*thi).next = *newitems.offset(idx as isize);
      let ref mut fresh2 = *newitems.offset(idx as isize);
      *fresh2 = thi
    }
    i = i.wrapping_add(1)
  }
  free((*hash).items as *mut libc::c_void);
  (*hash).csize = newsize;
  (*hash).items = newitems;
}
/* find item in hash, add it if necessary. Return ptr to data */
unsafe extern "C" fn hash_find(
  mut hash: *mut xhash,
  mut name: *const libc::c_char,
) -> *mut libc::c_void {
  let mut hi: *mut hash_item = 0 as *mut hash_item;
  let mut idx: libc::c_uint = 0;
  let mut l: libc::c_int = 0;
  hi = hash_search(hash, name) as *mut hash_item;
  if hi.is_null() {
    (*hash).nel = (*hash).nel.wrapping_add(1);
    if (*hash).nel.wrapping_div((*hash).csize) > 10i32 as libc::c_uint {
      hash_rebuild(hash);
    }
    l = strlen(name).wrapping_add(1i32 as libc::c_ulong) as libc::c_int;
    hi = xzalloc(
      (::std::mem::size_of::<hash_item>() as libc::c_ulong).wrapping_add(l as libc::c_ulong),
    ) as *mut hash_item;
    strcpy((*hi).name.as_mut_ptr(), name);
    idx = hashidx(name).wrapping_rem((*hash).csize);
    (*hi).next = *(*hash).items.offset(idx as isize);
    let ref mut fresh3 = *(*hash).items.offset(idx as isize);
    *fresh3 = hi;
    (*hash).glen = (*hash).glen.wrapping_add(l as libc::c_uint)
  }
  return &mut (*hi).data as *mut C2RustUnnamed_0 as *mut libc::c_void;
}
unsafe extern "C" fn hash_remove(mut hash: *mut xhash, mut name: *const libc::c_char) {
  let mut hi: *mut hash_item = 0 as *mut hash_item;
  let mut phi: *mut *mut hash_item = 0 as *mut *mut hash_item;
  phi = &mut *(*hash).items.offset(
    (hashidx as unsafe extern "C" fn(_: *const libc::c_char) -> libc::c_uint)(name)
      .wrapping_rem((*hash).csize) as isize,
  ) as *mut *mut hash_item_s;
  while !(*phi).is_null() {
    hi = *phi;
    if strcmp((*hi).name.as_mut_ptr(), name) == 0i32 {
      (*hash).glen = ((*hash).glen as libc::c_ulong)
        .wrapping_sub(strlen(name).wrapping_add(1i32 as libc::c_ulong))
        as libc::c_uint as libc::c_uint;
      (*hash).nel = (*hash).nel.wrapping_sub(1);
      *phi = (*hi).next;
      free(hi as *mut libc::c_void);
      break;
    } else {
      phi = &mut (*hi).next
    }
  }
}
/* ------ some useful functions ------ */
unsafe extern "C" fn skip_spaces(mut p: *mut libc::c_char) -> *mut libc::c_char {
  loop {
    if *p as libc::c_int == '\\' as i32 && *p.offset(1) as libc::c_int == '\n' as i32 {
      p = p.offset(1);
      let ref mut fresh4 = (*(ptr_to_globals as *mut globals2)).t_lineno;
      *fresh4 += 1
    } else if *p as libc::c_int != ' ' as i32 && *p as libc::c_int != '\t' as i32 {
      break;
    }
    p = p.offset(1)
  }
  return p;
}
/* returns old *s, advances *s past word and terminating NUL */
unsafe extern "C" fn nextword(mut s: *mut *mut libc::c_char) -> *mut libc::c_char {
  let mut p: *mut libc::c_char = *s;
  loop {
    let fresh5 = *s;
    *s = (*s).offset(1);
    if !(*fresh5 as libc::c_int != '\u{0}' as i32) {
      break;
    }
  }
  return p;
}
unsafe extern "C" fn nextchar(mut s: *mut *mut libc::c_char) -> libc::c_char {
  let mut c: libc::c_char = 0;
  let mut pps: *mut libc::c_char = 0 as *mut libc::c_char;
  let fresh6 = *s;
  *s = (*s).offset(1);
  c = *fresh6;
  pps = *s;
  if c as libc::c_int == '\\' as i32 {
    c = bb_process_escape_sequence(s as *mut *const libc::c_char)
  }
  /* Example awk statement:
   * s = "abc\"def"
   * we must treat \" as "
   */

  if c as libc::c_int == '\\' as i32 && *s == pps {
    /* unrecognized \z? */
    c = **s;
    if c != 0 {
      *s = (*s).offset(1)
    } /* yes, fetch z */
    /* advance unless z = NUL */
  }
  return c;
}
/* TODO: merge with strcpy_and_process_escape_sequences()?
 */
unsafe extern "C" fn unescape_string_in_place(mut s1: *mut libc::c_char) {
  let mut s: *mut libc::c_char = s1;
  loop {
    *s1 = nextchar(&mut s);
    if !(*s1 as libc::c_int != '\u{0}' as i32) {
      break;
    }
    s1 = s1.offset(1)
  }
}
#[inline(always)]
unsafe extern "C" fn isalnum_(mut c: libc::c_int) -> libc::c_int {
  return (bb_ascii_isalnum(c as libc::c_uchar) != 0 || c == '_' as i32) as libc::c_int;
}
unsafe extern "C" fn my_strtod(mut pp: *mut *mut libc::c_char) -> libc::c_double {
  let mut cp: *mut libc::c_char = *pp;
  if 1i32 != 0 && *cp.offset(0) as libc::c_int == '0' as i32 {
    /* Might be hex or octal integer: 0x123abc or 07777 */
    let mut c: libc::c_char = (*cp.offset(1) as libc::c_int | 0x20i32) as libc::c_char;
    if c as libc::c_int == 'x' as i32
      || (*cp.offset(1) as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32
    {
      let mut ull: libc::c_ulonglong = strtoull(cp, pp, 0i32);
      if c as libc::c_int == 'x' as i32 {
        return ull as libc::c_double;
      }
      c = **pp;
      if !((c as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32)
        && c as libc::c_int != '.' as i32
      {
        return ull as libc::c_double;
      }
      /* else: it may be a floating number. Examples:
       * 009.123 (*pp points to '9')
       * 000.123 (*pp points to '.')
       * fall through to strtod.
       */
    }
  }
  return strtod(cp, pp);
}
/* -------- working with variables (set/get/copy/etc) -------- */
unsafe extern "C" fn iamarray(mut v: *mut var) -> *mut xhash {
  let mut a: *mut var = v;
  while (*a).type_0 & 0x2000i32 as libc::c_uint != 0 {
    a = (*a).x.parent
  }
  if (*a).type_0 & 0x2i32 as libc::c_uint == 0 {
    (*a).type_0 |= 0x2i32 as libc::c_uint;
    (*a).x.array = hash_init()
  }
  return (*a).x.array;
}
unsafe extern "C" fn clear_array(mut array: *mut xhash) {
  let mut i: libc::c_uint = 0;
  let mut hi: *mut hash_item = 0 as *mut hash_item;
  let mut thi: *mut hash_item = 0 as *mut hash_item;
  i = 0i32 as libc::c_uint;
  while i < (*array).csize {
    hi = *(*array).items.offset(i as isize);
    while !hi.is_null() {
      thi = hi;
      hi = (*hi).next;
      free((*thi).data.v.string as *mut libc::c_void);
      free(thi as *mut libc::c_void);
    }
    let ref mut fresh7 = *(*array).items.offset(i as isize);
    *fresh7 = 0 as *mut hash_item_s;
    i = i.wrapping_add(1)
  }
  (*array).nel = 0i32 as libc::c_uint;
  (*array).glen = (*array).nel;
}
/* clear a variable */
unsafe extern "C" fn clrvar(mut v: *mut var) -> *mut var {
  if (*v).type_0 & 0x1000i32 as libc::c_uint == 0 {
    free((*v).string as *mut libc::c_void);
  }
  (*v).type_0 &= (0x2i32 | 0x400i32 | 0x800i32 | 0x2000i32 | 0x4000i32) as libc::c_uint;
  (*v).type_0 |= 0x4000i32 as libc::c_uint;
  (*v).string = 0 as *mut libc::c_char;
  return v;
}
/* assign string value to variable */
unsafe extern "C" fn setvar_p(mut v: *mut var, mut value: *mut libc::c_char) -> *mut var {
  clrvar(v);
  (*v).string = value;
  handle_special(v);
  return v;
}
/* same as setvar_p but make a copy of string */
unsafe extern "C" fn setvar_s(mut v: *mut var, mut value: *const libc::c_char) -> *mut var {
  return setvar_p(
    v,
    if !value.is_null() && *value as libc::c_int != 0 {
      xstrdup(value)
    } else {
      0 as *mut libc::c_char
    },
  );
}
/* same as setvar_s but sets USER flag */
unsafe extern "C" fn setvar_u(mut v: *mut var, mut value: *const libc::c_char) -> *mut var {
  v = setvar_s(v, value);
  (*v).type_0 |= 0x200i32 as libc::c_uint;
  return v;
}
/* set array element to user string */
unsafe extern "C" fn setari_u(mut a: *mut var, mut idx: libc::c_int, mut s: *const libc::c_char) {
  let mut v: *mut var = 0 as *mut var;
  v = hash_find(iamarray(a), itoa(idx)) as *mut var;
  setvar_u(v, s);
}
/* assign numeric value to variable */
unsafe extern "C" fn setvar_i(mut v: *mut var, mut value: libc::c_double) -> *mut var {
  clrvar(v);
  (*v).type_0 |= 0x1i32 as libc::c_uint;
  (*v).number = value;
  handle_special(v);
  return v;
}
unsafe extern "C" fn getvar_s(mut v: *mut var) -> *const libc::c_char {
  /* if v is numeric and has no cached string, convert it to string */
  if (*v).type_0 & (0x1i32 | 0x100i32) as libc::c_uint == 0x1i32 as libc::c_uint {
    fmt_num(
      (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
      240i32,
      getvar_s((*(ptr_to_globals as *mut globals2)).intvar[CONVFMT as libc::c_int as usize]),
      (*v).number,
      1i32,
    );
    (*v).string = xstrdup((*ptr_to_globals.offset(-1i32 as isize)).g_buf);
    (*v).type_0 |= 0x100i32 as libc::c_uint
  }
  return if (*v).string.is_null() {
    b"\x00" as *const u8 as *const libc::c_char
  } else {
    (*v).string
  };
}
unsafe extern "C" fn getvar_i(mut v: *mut var) -> libc::c_double {
  let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
  if (*v).type_0 & (0x1i32 | 0x100i32) as libc::c_uint == 0i32 as libc::c_uint {
    (*v).number = 0i32 as libc::c_double;
    s = (*v).string;
    if !s.is_null() && *s as libc::c_int != 0 {
      (*v).number = my_strtod(&mut s);
      if (*v).type_0 & 0x200i32 as libc::c_uint != 0 {
        s = skip_spaces(s);
        if *s as libc::c_int != '\u{0}' as i32 {
          (*v).type_0 &= !0x200i32 as libc::c_uint
        }
      }
    } else {
      (*v).type_0 &= !0x200i32 as libc::c_uint
    }
    (*v).type_0 |= 0x100i32 as libc::c_uint
  }
  return (*v).number;
}
/* Used for operands of bitwise ops */
unsafe extern "C" fn getvar_i_int(mut v: *mut var) -> libc::c_ulong {
  let mut d: libc::c_double = getvar_i(v);
  /* Casting doubles to longs is undefined for values outside
   * of target type range. Try to widen it as much as possible */
  if d >= 0i32 as libc::c_double {
    return d as libc::c_ulong;
  }
  /* Why? Think about d == -4294967295.0 (assuming 32bit longs) */
  return -(-d as libc::c_ulong as libc::c_long) as libc::c_ulong;
}
unsafe extern "C" fn copyvar(mut dest: *mut var, mut src: *const var) -> *mut var {
  if dest != src as *mut var {
    clrvar(dest);
    (*dest).type_0 |= (*src).type_0
      & !(0x2i32 | 0x400i32 | 0x800i32 | 0x2000i32 | 0x4000i32 | 0x1000i32) as libc::c_uint;
    (*dest).number = (*src).number;
    if !(*src).string.is_null() {
      (*dest).string = xstrdup((*src).string)
    }
  }
  handle_special(dest);
  return dest;
}
unsafe extern "C" fn incvar(mut v: *mut var) -> *mut var {
  return setvar_i(v, getvar_i(v) + 1.0f64);
}
/* return true if v is number or numeric string */
unsafe extern "C" fn is_numeric(mut v: *mut var) -> libc::c_int {
  getvar_i(v);
  return (((*v).type_0 ^ 0x4000i32 as libc::c_uint)
    & (0x1i32 | 0x200i32 | 0x4000i32) as libc::c_uint) as libc::c_int;
}
/* return 1 when value of v corresponds to true, 0 otherwise */
unsafe extern "C" fn istrue(mut v: *mut var) -> libc::c_int {
  if is_numeric(v) != 0 {
    return ((*v).number != 0i32 as libc::c_double) as libc::c_int;
  }
  return (!(*v).string.is_null() && *(*v).string.offset(0) as libc::c_int != 0) as libc::c_int;
}
/* temporary variables allocator. Last allocated should be first freed */
unsafe extern "C" fn nvalloc(mut n: libc::c_int) -> *mut var {
  let mut pb: *mut nvblock = 0 as *mut nvblock;
  let mut v: *mut var = 0 as *mut var;
  let mut r: *mut var = 0 as *mut var;
  let mut size: libc::c_int = 0;
  while !(*ptr_to_globals.offset(-1i32 as isize)).g_cb.is_null() {
    pb = (*ptr_to_globals.offset(-1i32 as isize)).g_cb;
    if (*(*ptr_to_globals.offset(-1i32 as isize)).g_cb)
      .pos
      .wrapping_offset_from(
        (*(*ptr_to_globals.offset(-1i32 as isize)).g_cb)
          .nv
          .as_mut_ptr(),
      ) as libc::c_long
      + n as libc::c_long
      <= (*(*ptr_to_globals.offset(-1i32 as isize)).g_cb).size as libc::c_long
    {
      break;
    }
    let ref mut fresh8 = (*ptr_to_globals.offset(-1i32 as isize)).g_cb;
    *fresh8 = (*(*ptr_to_globals.offset(-1i32 as isize)).g_cb).next
  }
  if (*ptr_to_globals.offset(-1i32 as isize)).g_cb.is_null() {
    size = if n <= 64i32 { 64i32 } else { n };
    let ref mut fresh9 = (*ptr_to_globals.offset(-1i32 as isize)).g_cb;
    *fresh9 = xzalloc(
      (::std::mem::size_of::<nvblock>() as libc::c_ulong).wrapping_add(
        (size as libc::c_ulong).wrapping_mul(::std::mem::size_of::<var>() as libc::c_ulong),
      ),
    ) as *mut nvblock;
    (*(*ptr_to_globals.offset(-1i32 as isize)).g_cb).size = size;
    let ref mut fresh10 = (*(*ptr_to_globals.offset(-1i32 as isize)).g_cb).pos;
    *fresh10 = (*(*ptr_to_globals.offset(-1i32 as isize)).g_cb)
      .nv
      .as_mut_ptr();
    let ref mut fresh11 = (*(*ptr_to_globals.offset(-1i32 as isize)).g_cb).prev;
    *fresh11 = pb;
    /*g_cb->next = NULL; - xzalloc did it */
    if !pb.is_null() {
      (*pb).next = (*ptr_to_globals.offset(-1i32 as isize)).g_cb
    }
  }
  r = (*(*ptr_to_globals.offset(-1i32 as isize)).g_cb).pos;
  v = r;
  let ref mut fresh12 = (*(*ptr_to_globals.offset(-1i32 as isize)).g_cb).pos;
  *fresh12 = (*fresh12).offset(n as isize);
  while v < (*(*ptr_to_globals.offset(-1i32 as isize)).g_cb).pos {
    (*v).type_0 = 0i32 as libc::c_uint;
    (*v).string = 0 as *mut libc::c_char;
    v = v.offset(1)
  }
  return r;
}
unsafe extern "C" fn nvfree(mut v: *mut var) {
  let mut p: *mut var = 0 as *mut var;
  if v
    < (*(*ptr_to_globals.offset(-1i32 as isize)).g_cb)
      .nv
      .as_mut_ptr()
    || v >= (*(*ptr_to_globals.offset(-1i32 as isize)).g_cb).pos
  {
    syntax_error(EMSG_INTERNAL_ERROR.as_ptr());
  }
  p = v;
  while p < (*(*ptr_to_globals.offset(-1i32 as isize)).g_cb).pos {
    if (*p).type_0 & (0x2i32 | 0x2000i32) as libc::c_uint == 0x2i32 as libc::c_uint {
      clear_array(iamarray(p));
      free((*(*p).x.array).items as *mut libc::c_void);
      free((*p).x.array as *mut libc::c_void);
    }
    if (*p).type_0 & 0x800i32 as libc::c_uint != 0 {
      let mut n: *mut walker_list = 0 as *mut walker_list;
      let mut w: *mut walker_list = (*p).x.walker;
      (*p).x.walker = 0 as *mut walker_list;
      while !w.is_null() {
        n = (*w).prev;
        free(w as *mut libc::c_void);
        w = n
      }
    }
    clrvar(p);
    p = p.offset(1)
  }
  let ref mut fresh13 = (*(*ptr_to_globals.offset(-1i32 as isize)).g_cb).pos;
  *fresh13 = v;
  while !(*(*ptr_to_globals.offset(-1i32 as isize)).g_cb)
    .prev
    .is_null()
    && (*(*ptr_to_globals.offset(-1i32 as isize)).g_cb).pos
      == (*(*ptr_to_globals.offset(-1i32 as isize)).g_cb)
        .nv
        .as_mut_ptr()
  {
    let ref mut fresh14 = (*ptr_to_globals.offset(-1i32 as isize)).g_cb;
    *fresh14 = (*(*ptr_to_globals.offset(-1i32 as isize)).g_cb).prev
  }
}
/* ------- awk program text parsing ------- */
/* Parse next token pointed by global pos, place results into global ttt.
 * If token isn't expected, give away. Return token class
 */
unsafe extern "C" fn next_token(mut expected: uint32_t) -> uint32_t {
  /* Initialized to TC_OPTERM: */
  let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut tl: *const libc::c_char = 0 as *const libc::c_char;
  let mut tc: uint32_t = 0;
  let mut ti: *const uint32_t = 0 as *const uint32_t;
  if (*ptr_to_globals.offset(-1i32 as isize)).t_rollback != 0 {
    (*ptr_to_globals.offset(-1i32 as isize)).t_rollback = 0i32 as smallint
  } else if (*(ptr_to_globals as *mut globals2)).next_token__concat_inserted != 0 {
    (*(ptr_to_globals as *mut globals2)).next_token__concat_inserted = 0i32 as smallint;
    (*(ptr_to_globals as *mut globals2)).t_tclass =
      (*(ptr_to_globals as *mut globals2)).next_token__save_tclass;
    (*(ptr_to_globals as *mut globals2)).t_info =
      (*(ptr_to_globals as *mut globals2)).next_token__save_info
  } else {
    let mut current_block_102: u64;
    p = (*ptr_to_globals.offset(-1i32 as isize)).g_pos;
    loop {
      p = skip_spaces(p);
      (*ptr_to_globals.offset(-1i32 as isize)).g_lineno =
        (*(ptr_to_globals as *mut globals2)).t_lineno;
      if *p as libc::c_int == '#' as i32 {
        while *p as libc::c_int != '\n' as i32 && *p as libc::c_int != '\u{0}' as i32 {
          p = p.offset(1)
        }
      }
      if *p as libc::c_int == '\n' as i32 {
        let ref mut fresh15 = (*(ptr_to_globals as *mut globals2)).t_lineno;
        *fresh15 += 1
      }
      if *p as libc::c_int == '\u{0}' as i32 {
        tc = (1i32 << 25i32) as uint32_t
      } else if *p as libc::c_int == '\"' as i32 {
        /* it's a string */
        p = p.offset(1);
        s = p;
        let ref mut fresh16 = (*(ptr_to_globals as *mut globals2)).t_string;
        *fresh16 = s;
        while *p as libc::c_int != '\"' as i32 {
          let mut pp: *mut libc::c_char = 0 as *mut libc::c_char;
          if *p as libc::c_int == '\u{0}' as i32 || *p as libc::c_int == '\n' as i32 {
            syntax_error(EMSG_UNEXP_EOS.as_ptr());
          }
          pp = p;
          let fresh17 = s;
          s = s.offset(1);
          *fresh17 = nextchar(&mut pp);
          p = pp
        }
        p = p.offset(1);
        *s = '\u{0}' as i32 as libc::c_char;
        tc = (1i32 << 29i32) as uint32_t
      } else if expected & (1i32 << 2i32) as libc::c_uint != 0 && *p as libc::c_int == '/' as i32 {
        /* it's regexp */
        p = p.offset(1);
        s = p;
        let ref mut fresh18 = (*(ptr_to_globals as *mut globals2)).t_string;
        *fresh18 = s;
        while *p as libc::c_int != '/' as i32 {
          if *p as libc::c_int == '\u{0}' as i32 || *p as libc::c_int == '\n' as i32 {
            syntax_error(EMSG_UNEXP_EOS.as_ptr());
          }
          let fresh19 = p;
          p = p.offset(1);
          *s = *fresh19;
          let fresh20 = s;
          s = s.offset(1);
          if *fresh20 as libc::c_int == '\\' as i32 {
            let mut pp_0: *mut libc::c_char = p;
            *s.offset(-1i32 as isize) = bb_process_escape_sequence(
              &mut pp_0 as *mut *mut libc::c_char as *mut *const libc::c_char,
            );
            if *p as libc::c_int == '\\' as i32 {
              let fresh21 = s;
              s = s.offset(1);
              *fresh21 = '\\' as i32 as libc::c_char
            }
            if pp_0 == p {
              let fresh22 = p;
              p = p.offset(1);
              let fresh23 = s;
              s = s.offset(1);
              *fresh23 = *fresh22
            } else {
              p = pp_0
            }
          }
        }
        p = p.offset(1);
        *s = '\u{0}' as i32 as libc::c_char;
        tc = (1i32 << 2i32) as uint32_t
      } else if *p as libc::c_int == '.' as i32
        || (*p as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32
      {
        /* it's a number */
        let mut pp_1: *mut libc::c_char = p;
        (*ptr_to_globals.offset(-1i32 as isize)).t_double = my_strtod(&mut pp_1);
        p = pp_1;
        if *p as libc::c_int == '.' as i32 {
          syntax_error(EMSG_UNEXP_TOKEN.as_ptr());
        }
        tc = (1i32 << 30i32) as uint32_t
      } else {
        /* search for something known */
        tl = tokenlist.as_ptr();
        tc = 0x1i32 as uint32_t;
        ti = tokeninfo.as_ptr();
        loop {
          if !(*tl != 0) {
            current_block_102 = 10435735846551762309;
            break;
          }
          let fresh24 = tl;
          tl = tl.offset(1);
          let mut l: libc::c_int = *fresh24 as libc::c_uchar as libc::c_int;
          if l == '\u{ff}' as i32 as libc::c_uchar as libc::c_int {
            tc <<= 1i32
          } else if tc
            & (expected
              | (1i32 << 7i32
                | (1i32 << 16i32 | 1i32 << 17i32)
                | 1i32 << 18i32
                | 1i32 << 19i32
                | 1i32 << 20i32
                | 1i32 << 21i32
                | 1i32 << 22i32
                | 1i32 << 23i32
                | 1i32 << 24i32) as libc::c_uint
              | (1i32 << 15i32) as libc::c_uint)
            != 0
            && strncmp(p, tl, l as libc::c_ulong) == 0i32
            && !(tc
              & (1i32 << 7i32
                | (1i32 << 16i32 | 1i32 << 17i32)
                | 1i32 << 18i32
                | 1i32 << 19i32
                | 1i32 << 20i32
                | 1i32 << 21i32
                | 1i32 << 22i32
                | 1i32 << 23i32
                | 1i32 << 24i32) as libc::c_uint
              != 0
              && isalnum_(*p.offset(l as isize) as libc::c_int) != 0)
          {
            /* if token class is expected,
             * token matches,
             * and it's not a longer word,
             */
            /* then this is what we are looking for */
            (*(ptr_to_globals as *mut globals2)).t_info = *ti;
            p = p.offset(l as isize);
            current_block_102 = 11983909263168414716;
            break;
          } else {
            ti = ti.offset(1);
            tl = tl.offset(l as isize)
          }
        }
        match current_block_102 {
          11983909263168414716 => {}
          _ => {
            /* not a known token */
            /* is it a name? (var/array/function) */
            if isalnum_(*p as libc::c_int) == 0 {
              syntax_error(EMSG_UNEXP_TOKEN.as_ptr()); /* no */
            }
            /* yes */
            p = p.offset(-1);
            let ref mut fresh25 = (*(ptr_to_globals as *mut globals2)).t_string;
            *fresh25 = p;
            loop {
              p = p.offset(1);
              if !(isalnum_(*p as libc::c_int) != 0) {
                break;
              }
              *p.offset(-1i32 as isize) = *p
            }
            *p.offset(-1i32 as isize) = '\u{0}' as i32 as libc::c_char;
            tc = (1i32 << 26i32) as uint32_t;
            /* also consume whitespace between functionname and bracket */
            if expected & (1i32 << 26i32) as libc::c_uint == 0
              || expected & (1i32 << 27i32) as libc::c_uint != 0
            {
              p = skip_spaces(p)
            }
            if *p as libc::c_int == '(' as i32 {
              tc = (1i32 << 28i32) as uint32_t
            } else if *p as libc::c_int == '[' as i32 {
              p = p.offset(1);
              tc = (1i32 << 27i32) as uint32_t
            }
          }
        }
      }
      let ref mut fresh26 = (*ptr_to_globals.offset(-1i32 as isize)).g_pos;
      *fresh26 = p;
      /* skipping newlines in some cases */
      if !((*(ptr_to_globals as *mut globals2)).next_token__ltclass
        & (1i32 << 8i32
          | 1i32 << 12i32
          | 1i32 << 13i32
          | (1i32 << 6i32 | 1i32 << 8i32 | 1i32 << 9i32 | 1i32 << 7i32)
          | (1i32 << 14i32 | 1i32 << 15i32)) as libc::c_uint
        != 0
        && tc & (1i32 << 15i32) as libc::c_uint != 0)
      {
        break;
      }
    }
    /* insert concatenation operator when needed */
    if (*(ptr_to_globals as *mut globals2)).next_token__ltclass
      & (1i32 << 26i32
        | 1i32 << 11i32
        | 1i32 << 1i32
        | 1i32 << 29i32
        | 1i32 << 30i32
        | 1i32 << 4i32) as libc::c_uint
      != 0
      && tc
        & (1i32 << 26i32
          | 1i32 << 27i32
          | 1i32 << 28i32
          | 1i32 << 19i32
          | 1i32 << 20i32
          | 1i32 << 21i32
          | 1i32 << 0i32
          | 1i32 << 29i32
          | 1i32 << 30i32
          | (1i32 << 5i32 | 1i32 << 10i32)) as libc::c_uint
        != 0
      && expected & (1i32 << 6i32 | 1i32 << 8i32 | 1i32 << 9i32 | 1i32 << 7i32) as libc::c_uint != 0
    {
      (*(ptr_to_globals as *mut globals2)).next_token__concat_inserted = 1i32 as smallint;
      (*(ptr_to_globals as *mut globals2)).next_token__save_tclass = tc;
      (*(ptr_to_globals as *mut globals2)).next_token__save_info =
        (*(ptr_to_globals as *mut globals2)).t_info;
      tc = (1i32 << 6i32 | 1i32 << 8i32 | 1i32 << 9i32 | 1i32 << 7i32) as uint32_t;
      (*(ptr_to_globals as *mut globals2)).t_info = (OC_CONCAT as libc::c_int
        | (0x10000i32 | 0x40000i32 | 0x20000i32 | 0x80000i32)
        | 35i32 << 24i32) as uint32_t
    }
    (*(ptr_to_globals as *mut globals2)).t_tclass = tc
  }
  (*(ptr_to_globals as *mut globals2)).next_token__ltclass =
    (*(ptr_to_globals as *mut globals2)).t_tclass;
  /* Are we ready for this? */
  if (*(ptr_to_globals as *mut globals2)).next_token__ltclass & expected == 0 {
    syntax_error(
      if (*(ptr_to_globals as *mut globals2)).next_token__ltclass
        & (1i32 << 15i32 | 1i32 << 25i32) as libc::c_uint
        != 0
      {
        EMSG_UNEXP_EOS.as_ptr()
      } else {
        EMSG_UNEXP_TOKEN.as_ptr()
      },
    );
  }
  return (*(ptr_to_globals as *mut globals2)).next_token__ltclass;
}
unsafe extern "C" fn rollback_token() {
  (*ptr_to_globals.offset(-1i32 as isize)).t_rollback = 1i32 as smallint;
}
unsafe extern "C" fn new_node(mut info: uint32_t) -> *mut node {
  let mut n: *mut node = 0 as *mut node;
  n = xzalloc(::std::mem::size_of::<node>() as libc::c_ulong) as *mut node;
  (*n).info = info;
  (*n).lineno = (*ptr_to_globals.offset(-1i32 as isize)).g_lineno as libc::c_uint;
  return n;
}
unsafe extern "C" fn mk_re_node(
  mut s: *const libc::c_char,
  mut n: *mut node,
  mut re: *mut regex_t,
) {
  (*n).info = OC_REGEXP as libc::c_int as uint32_t;
  (*n).l.re = re;
  (*n).r.ire = re.offset(1);
  xregcomp(re, s, 1i32);
  xregcomp(re.offset(1), s, 1i32 | 1i32 << 1i32);
}
unsafe extern "C" fn condition() -> *mut node {
  next_token((1i32 << 0i32) as uint32_t);
  return parse_expr((1i32 << 1i32) as uint32_t);
}
/* parse expression terminated by given argument, return ptr
 * to built subtree. Terminator is eaten by parse_expr */
unsafe extern "C" fn parse_expr(mut iexp: uint32_t) -> *mut node {
  let mut sn: node = node {
    info: 0,
    lineno: 0,
    l: C2RustUnnamed_3 {
      n: 0 as *mut node_s,
    },
    r: C2RustUnnamed_2 {
      n: 0 as *mut node_s,
    },
    a: C2RustUnnamed_1 {
      n: 0 as *mut node_s,
    },
  };
  let mut cn: *mut node = &mut sn;
  let mut vn: *mut node = 0 as *mut node;
  let mut glptr: *mut node = 0 as *mut node;
  let mut tc: uint32_t = 0;
  let mut xtc: uint32_t = 0;
  let mut v: *mut var = 0 as *mut var;
  sn.info = 0x7f000000i32 as uint32_t;
  glptr = 0 as *mut node;
  sn.a.n = glptr;
  sn.r.n = sn.a.n;
  xtc = (1i32 << 26i32
    | 1i32 << 27i32
    | 1i32 << 28i32
    | 1i32 << 19i32
    | 1i32 << 20i32
    | 1i32 << 21i32
    | 1i32 << 0i32
    | 1i32 << 29i32
    | 1i32 << 30i32
    | (1i32 << 5i32 | 1i32 << 10i32)
    | 1i32 << 2i32) as libc::c_uint
    | iexp;
  loop {
    tc = next_token(xtc);
    if !(tc & iexp == 0) {
      break;
    }
    if !glptr.is_null()
      && (*(ptr_to_globals as *mut globals2)).t_info
        == (OC_COMPARE as libc::c_int | (0x10000i32 | 0x20000i32) | 39i32 << 24i32 | 2i32)
          as libc::c_uint
    {
      /* input redirection (<) attached to glptr node */
      (*glptr).l.n = new_node(
        (OC_CONCAT as libc::c_int
          | (0x10000i32 | 0x40000i32 | 0x20000i32 | 0x80000i32)
          | 37i32 << 24i32) as uint32_t,
      );
      cn = (*glptr).l.n;
      (*cn).a.n = glptr;
      xtc = (1i32 << 26i32
        | 1i32 << 27i32
        | 1i32 << 28i32
        | 1i32 << 19i32
        | 1i32 << 20i32
        | 1i32 << 21i32
        | 1i32 << 0i32
        | 1i32 << 29i32
        | 1i32 << 30i32
        | (1i32 << 5i32 | 1i32 << 10i32)) as uint32_t;
      glptr = 0 as *mut node
    } else if tc
      & (1i32 << 6i32 | 1i32 << 8i32 | 1i32 << 9i32 | 1i32 << 7i32 | 1i32 << 4i32) as libc::c_uint
      != 0
    {
      /* for binary and postfix-unary operators, jump back over
       * previous operators with higher priority */
      vn = cn;
      while (*(ptr_to_globals as *mut globals2)).t_info & 0x7f000000i32 as libc::c_uint
        > (*(*vn).a.n).info & 0x7e000000i32 as libc::c_uint
        || (*(ptr_to_globals as *mut globals2)).t_info == (*vn).info
          && (*(ptr_to_globals as *mut globals2)).t_info & 0xff00i32 as libc::c_uint
            == OC_COLON as libc::c_int as libc::c_uint
      {
        vn = (*vn).a.n;
        if (*vn).a.n.is_null() {
          syntax_error(EMSG_UNEXP_TOKEN.as_ptr());
        }
      }
      if (*(ptr_to_globals as *mut globals2)).t_info & 0xff00i32 as libc::c_uint
        == OC_TERNARY as libc::c_int as libc::c_uint
      {
        let ref mut fresh27 = (*(ptr_to_globals as *mut globals2)).t_info;
        *fresh27 = (*fresh27 as libc::c_uint).wrapping_add((6i32 << 24i32) as libc::c_uint)
          as uint32_t as uint32_t
      }
      (*(*vn).a.n).r.n = new_node((*(ptr_to_globals as *mut globals2)).t_info);
      cn = (*(*vn).a.n).r.n;
      (*cn).a.n = (*vn).a.n;
      if tc & (1i32 << 6i32 | 1i32 << 8i32 | 1i32 << 9i32 | 1i32 << 7i32) as libc::c_uint != 0 {
        (*cn).l.n = vn;
        xtc = (1i32 << 26i32
          | 1i32 << 27i32
          | 1i32 << 28i32
          | 1i32 << 19i32
          | 1i32 << 20i32
          | 1i32 << 21i32
          | 1i32 << 0i32
          | 1i32 << 29i32
          | 1i32 << 30i32
          | (1i32 << 5i32 | 1i32 << 10i32)
          | 1i32 << 2i32) as uint32_t;
        if (*(ptr_to_globals as *mut globals2)).t_info & 0xff00i32 as libc::c_uint
          == OC_PGETLINE as libc::c_int as libc::c_uint
        {
          /* it's a pipe */
          next_token((1i32 << 21i32) as uint32_t);
          /* give maximum priority to this pipe */
          (*cn).info &= !0x7f000000i32 as libc::c_uint;
          xtc = (1i32 << 26i32
            | 1i32 << 27i32
            | 1i32 << 28i32
            | 1i32 << 19i32
            | 1i32 << 20i32
            | 1i32 << 21i32
            | 1i32 << 0i32
            | 1i32 << 29i32
            | 1i32 << 30i32
            | (1i32 << 5i32 | 1i32 << 10i32)
            | (1i32 << 6i32 | 1i32 << 8i32 | 1i32 << 9i32 | 1i32 << 7i32))
            as libc::c_uint
            | iexp
        }
      } else {
        (*cn).r.n = vn;
        xtc = (1i32 << 26i32
          | 1i32 << 27i32
          | 1i32 << 28i32
          | 1i32 << 19i32
          | 1i32 << 20i32
          | 1i32 << 21i32
          | 1i32 << 0i32
          | 1i32 << 29i32
          | 1i32 << 30i32
          | (1i32 << 5i32 | 1i32 << 10i32)
          | (1i32 << 6i32 | 1i32 << 8i32 | 1i32 << 9i32 | 1i32 << 7i32))
          as libc::c_uint
          | iexp
      }
      (*vn).a.n = cn
    } else {
      /* for operands and prefix-unary operators, attach them
       * to last node */
      vn = cn;
      (*vn).r.n = new_node((*(ptr_to_globals as *mut globals2)).t_info);
      cn = (*vn).r.n;
      (*cn).a.n = vn;
      xtc = (1i32 << 26i32
        | 1i32 << 27i32
        | 1i32 << 28i32
        | 1i32 << 19i32
        | 1i32 << 20i32
        | 1i32 << 21i32
        | 1i32 << 0i32
        | 1i32 << 29i32
        | 1i32 << 30i32
        | (1i32 << 5i32 | 1i32 << 10i32)
        | 1i32 << 2i32) as uint32_t;
      if tc
        & (1i32 << 26i32
          | 1i32 << 27i32
          | 1i32 << 28i32
          | 1i32 << 19i32
          | 1i32 << 20i32
          | 1i32 << 21i32
          | 1i32 << 0i32
          | 1i32 << 29i32
          | 1i32 << 30i32
          | 1i32 << 2i32) as libc::c_uint
        != 0
      {
        xtc = (1i32 << 5i32
          | 1i32 << 10i32
          | 1i32 << 4i32
          | (1i32 << 6i32 | 1i32 << 8i32 | 1i32 << 9i32 | 1i32 << 7i32)
          | (1i32 << 26i32
            | 1i32 << 27i32
            | 1i32 << 28i32
            | 1i32 << 19i32
            | 1i32 << 20i32
            | 1i32 << 21i32
            | 1i32 << 0i32
            | 1i32 << 29i32
            | 1i32 << 30i32)) as libc::c_uint
          | iexp;
        /* one should be very careful with switch on tclass -
         * only simple tclasses should be used! */
        match tc {
          67108864 | 134217728 => {
            (*cn).info = OC_VAR as libc::c_int as uint32_t;
            v = hash_search(
              (*ptr_to_globals.offset(-1i32 as isize)).ahash,
              (*(ptr_to_globals as *mut globals2)).t_string,
            ) as *mut var;
            if !v.is_null() {
              (*cn).info = OC_FNARG as libc::c_int as uint32_t;
              (*cn).l.aidx = (*v).x.aidx
            } else {
              (*cn).l.v = hash_find(
                (*ptr_to_globals.offset(-1i32 as isize)).vhash,
                (*(ptr_to_globals as *mut globals2)).t_string,
              ) as *mut var
            }
            if tc & (1i32 << 27i32) as libc::c_uint != 0 {
              (*cn).info |= (0x20000i32 | 0x80000i32) as libc::c_uint;
              (*cn).r.n = parse_expr((1i32 << 11i32) as uint32_t)
            }
          }
          1073741824 | 536870912 => {
            (*cn).info = OC_VAR as libc::c_int as uint32_t;
            (*cn).l.v = xzalloc(::std::mem::size_of::<var>() as libc::c_ulong) as *mut var;
            v = (*cn).l.v;
            if tc & (1i32 << 30i32) as libc::c_uint != 0 {
              setvar_i(v, (*ptr_to_globals.offset(-1i32 as isize)).t_double);
            } else {
              setvar_s(v, (*(ptr_to_globals as *mut globals2)).t_string);
            }
          }
          4 => {
            mk_re_node(
              (*(ptr_to_globals as *mut globals2)).t_string,
              cn,
              xzalloc(
                (::std::mem::size_of::<regex_t>() as libc::c_ulong)
                  .wrapping_mul(2i32 as libc::c_ulong),
              ) as *mut regex_t,
            );
          }
          268435456 => {
            (*cn).info = OC_FUNC as libc::c_int as uint32_t;
            (*cn).r.f = hash_find(
              (*ptr_to_globals.offset(-1i32 as isize)).fnhash,
              (*(ptr_to_globals as *mut globals2)).t_string,
            ) as *mut func;
            (*cn).l.n = condition()
          }
          1 => {
            (*vn).r.n = parse_expr((1i32 << 1i32) as uint32_t);
            cn = (*vn).r.n;
            if cn.is_null() {
              syntax_error(b"Empty sequence\x00" as *const u8 as *const libc::c_char);
            }
            (*cn).a.n = vn
          }
          2097152 => {
            glptr = cn;
            xtc = (1i32 << 26i32
              | 1i32 << 27i32
              | 1i32 << 28i32
              | 1i32 << 19i32
              | 1i32 << 20i32
              | 1i32 << 21i32
              | 1i32 << 0i32
              | 1i32 << 29i32
              | 1i32 << 30i32
              | (1i32 << 5i32 | 1i32 << 10i32)
              | (1i32 << 6i32 | 1i32 << 8i32 | 1i32 << 9i32 | 1i32 << 7i32))
              as libc::c_uint
              | iexp
          }
          524288 => (*cn).l.n = condition(),
          1048576 => {
            next_token(
              (1i32 << 0i32 | (1i32 << 14i32 | 1i32 << 15i32) | 1i32 << 13i32) as uint32_t,
            );
            rollback_token();
            if (*(ptr_to_globals as *mut globals2)).t_tclass & (1i32 << 0i32) as libc::c_uint != 0 {
              /* It was a "(" token. Handle just like TC_BUILTIN */
              (*cn).l.n = condition()
            }
          }
          _ => {}
        }
      }
    }
  }
  return sn.r.n;
}
/* add node to chain. Return ptr to alloc'd node */
unsafe extern "C" fn chain_node(mut info: uint32_t) -> *mut node {
  let mut n: *mut node = 0 as *mut node;
  if (*(*ptr_to_globals.offset(-1i32 as isize)).seq)
    .first
    .is_null()
  {
    let ref mut fresh28 = (*(*ptr_to_globals.offset(-1i32 as isize)).seq).last;
    *fresh28 = new_node(0i32 as uint32_t);
    let ref mut fresh29 = (*(*ptr_to_globals.offset(-1i32 as isize)).seq).first;
    *fresh29 = *fresh28
  }
  if (*(*ptr_to_globals.offset(-1i32 as isize)).seq).programname
    != (*ptr_to_globals.offset(-1i32 as isize)).g_progname
  {
    let ref mut fresh30 = (*(*ptr_to_globals.offset(-1i32 as isize)).seq).programname;
    *fresh30 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
    n = chain_node(OC_NEWSOURCE as libc::c_int as uint32_t);
    (*n).l.new_progname = xstrdup((*ptr_to_globals.offset(-1i32 as isize)).g_progname)
  }
  n = (*(*ptr_to_globals.offset(-1i32 as isize)).seq).last;
  (*n).info = info;
  (*n).a.n = new_node(OC_DONE as libc::c_int as uint32_t);
  let ref mut fresh31 = (*(*ptr_to_globals.offset(-1i32 as isize)).seq).last;
  *fresh31 = (*n).a.n;
  return n;
}
unsafe extern "C" fn chain_expr(mut info: uint32_t) {
  let mut n: *mut node = 0 as *mut node;
  n = chain_node(info);
  (*n).l.n = parse_expr((1i32 << 14i32 | 1i32 << 15i32 | 1i32 << 13i32) as uint32_t);
  if info & 0x400000i32 as libc::c_uint != 0 && (*n).l.n.is_null() {
    syntax_error(EMSG_TOO_FEW_ARGS.as_ptr());
  }
  if (*(ptr_to_globals as *mut globals2)).t_tclass & (1i32 << 13i32) as libc::c_uint != 0 {
    rollback_token();
  };
}
unsafe extern "C" fn chain_loop(mut nn: *mut node) -> *mut node {
  let mut n: *mut node = 0 as *mut node;
  let mut n2: *mut node = 0 as *mut node;
  let mut save_brk: *mut node = 0 as *mut node;
  let mut save_cont: *mut node = 0 as *mut node;
  save_brk = (*ptr_to_globals.offset(-1i32 as isize)).break_ptr;
  save_cont = (*ptr_to_globals.offset(-1i32 as isize)).continue_ptr;
  n = chain_node((OC_BR as libc::c_int | 0x10000i32) as uint32_t);
  let ref mut fresh32 = (*ptr_to_globals.offset(-1i32 as isize)).continue_ptr;
  *fresh32 = new_node(OC_EXEC as libc::c_int as uint32_t);
  let ref mut fresh33 = (*ptr_to_globals.offset(-1i32 as isize)).break_ptr;
  *fresh33 = new_node(OC_EXEC as libc::c_int as uint32_t);
  chain_group();
  n2 = chain_node((OC_EXEC as libc::c_int | 0x10000i32) as uint32_t);
  (*n2).l.n = nn;
  (*n2).a.n = n;
  let ref mut fresh34 = (*(*ptr_to_globals.offset(-1i32 as isize)).continue_ptr).a.n;
  *fresh34 = n2;
  (*n).r.n = (*(*ptr_to_globals.offset(-1i32 as isize)).seq).last;
  let ref mut fresh35 = (*(*ptr_to_globals.offset(-1i32 as isize)).break_ptr).a.n;
  *fresh35 = (*n).r.n;
  let ref mut fresh36 = (*ptr_to_globals.offset(-1i32 as isize)).continue_ptr;
  *fresh36 = save_cont;
  let ref mut fresh37 = (*ptr_to_globals.offset(-1i32 as isize)).break_ptr;
  *fresh37 = save_brk;
  return n;
}
/* parse group and attach it to chain */
unsafe extern "C" fn chain_group() {
  let mut c: uint32_t = 0;
  let mut n: *mut node = 0 as *mut node;
  let mut n2: *mut node = 0 as *mut node;
  let mut n3: *mut node = 0 as *mut node;
  loop {
    c = next_token(
      (1i32 << 26i32
        | 1i32 << 27i32
        | 1i32 << 28i32
        | 1i32 << 19i32
        | 1i32 << 20i32
        | 1i32 << 21i32
        | 1i32 << 0i32
        | 1i32 << 29i32
        | 1i32 << 30i32
        | (1i32 << 5i32 | 1i32 << 10i32)
        | 1i32 << 2i32
        | (1i32 << 14i32 | 1i32 << 15i32)
        | (1i32 << 16i32 | 1i32 << 17i32)
        | 1i32 << 12i32) as uint32_t,
    );
    if !(c & (1i32 << 15i32) as libc::c_uint != 0) {
      break;
    }
  }
  if c & (1i32 << 12i32) as libc::c_uint != 0 {
    while next_token(
      (1i32 << 26i32
        | 1i32 << 27i32
        | 1i32 << 28i32
        | 1i32 << 19i32
        | 1i32 << 20i32
        | 1i32 << 21i32
        | 1i32 << 0i32
        | 1i32 << 29i32
        | 1i32 << 30i32
        | (1i32 << 5i32 | 1i32 << 10i32)
        | 1i32 << 2i32
        | (1i32 << 14i32 | 1i32 << 15i32)
        | (1i32 << 16i32 | 1i32 << 17i32)
        | 1i32 << 12i32
        | 1i32 << 13i32) as uint32_t,
    ) != (1i32 << 13i32) as libc::c_uint
    {
      if (*(ptr_to_globals as *mut globals2)).t_tclass & (1i32 << 15i32) as libc::c_uint != 0 {
        continue;
      }
      rollback_token();
      chain_group();
    }
  } else if c
    & (1i32 << 26i32
      | 1i32 << 27i32
      | 1i32 << 28i32
      | 1i32 << 19i32
      | 1i32 << 20i32
      | 1i32 << 21i32
      | 1i32 << 0i32
      | 1i32 << 29i32
      | 1i32 << 30i32
      | (1i32 << 5i32 | 1i32 << 10i32)
      | 1i32 << 2i32
      | (1i32 << 14i32 | 1i32 << 15i32)) as libc::c_uint
    != 0
  {
    rollback_token();
    chain_expr((OC_EXEC as libc::c_int | 0x10000i32) as uint32_t);
  } else {
    /* TC_STATEMNT */
    match (*(ptr_to_globals as *mut globals2)).t_info & 0xff00i32 as libc::c_uint {
      12288 => {
        n = chain_node((OC_BR as libc::c_int | 0x10000i32) as uint32_t); /* for (;;) */
        (*n).l.n = condition();
        chain_group();
        n2 = chain_node(OC_EXEC as libc::c_int as uint32_t);
        (*n).r.n = (*(*ptr_to_globals.offset(-1i32 as isize)).seq).last;
        if next_token(
          (1i32 << 26i32
            | 1i32 << 27i32
            | 1i32 << 28i32
            | 1i32 << 19i32
            | 1i32 << 20i32
            | 1i32 << 21i32
            | 1i32 << 0i32
            | 1i32 << 29i32
            | 1i32 << 30i32
            | (1i32 << 5i32 | 1i32 << 10i32)
            | 1i32 << 2i32
            | (1i32 << 14i32 | 1i32 << 15i32)
            | (1i32 << 16i32 | 1i32 << 17i32)
            | 1i32 << 12i32
            | 1i32 << 13i32
            | 1i32 << 18i32) as uint32_t,
        ) == (1i32 << 18i32) as libc::c_uint
        {
          chain_group();
          (*n2).a.n = (*(*ptr_to_globals.offset(-1i32 as isize)).seq).last
        } else {
          rollback_token();
        }
      }
      13056 => {
        n2 = condition();
        n = chain_loop(0 as *mut node);
        (*n).l.n = n2
      }
      12544 => {
        n2 = chain_node(OC_EXEC as libc::c_int as uint32_t);
        n = chain_loop(0 as *mut node);
        (*n2).a.n = (*n).a.n;
        next_token((1i32 << 17i32) as uint32_t);
        (*n).l.n = condition()
      }
      12800 => {
        next_token((1i32 << 0i32) as uint32_t);
        n2 = parse_expr((1i32 << 14i32 | 1i32 << 1i32) as uint32_t);
        if (*(ptr_to_globals as *mut globals2)).t_tclass & (1i32 << 1i32) as libc::c_uint != 0 {
          /* for-in */
          if n2.is_null()
            || (*n2).info & 0xff00i32 as libc::c_uint != OC_IN as libc::c_int as libc::c_uint
          {
            syntax_error(EMSG_UNEXP_TOKEN.as_ptr());
          }
          n = chain_node((OC_WALKINIT as libc::c_int | (0x10000i32 | 0x20000i32)) as uint32_t);
          (*n).l.n = (*n2).l.n;
          (*n).r.n = (*n2).r.n;
          n = chain_loop(0 as *mut node);
          (*n).info = (OC_WALKNEXT as libc::c_int | 0x10000i32) as uint32_t;
          (*n).l.n = (*n2).l.n
        } else {
          n = chain_node((OC_EXEC as libc::c_int | 0x10000i32) as uint32_t);
          (*n).l.n = n2;
          n2 = parse_expr((1i32 << 14i32) as uint32_t);
          n3 = parse_expr((1i32 << 1i32) as uint32_t);
          n = chain_loop(n3);
          (*n).l.n = n2;
          if n2.is_null() {
            (*n).info = OC_EXEC as libc::c_int as uint32_t
          }
        }
      }
      1024 | 1280 => {
        n = chain_node((*(ptr_to_globals as *mut globals2)).t_info);
        (*n).l.n =
          parse_expr((1i32 << 14i32 | 1i32 << 15i32 | 1i32 << 3i32 | 1i32 << 13i32) as uint32_t);
        if (*(ptr_to_globals as *mut globals2)).t_tclass & (1i32 << 3i32) as libc::c_uint != 0 {
          (*n).info |= (*(ptr_to_globals as *mut globals2)).t_info;
          (*n).r.n = parse_expr((1i32 << 14i32 | 1i32 << 15i32 | 1i32 << 13i32) as uint32_t)
        }
        if (*(ptr_to_globals as *mut globals2)).t_tclass & (1i32 << 13i32) as libc::c_uint != 0 {
          rollback_token();
        }
      }
      2048 => {
        n = chain_node(OC_EXEC as libc::c_int as uint32_t);
        (*n).a.n = (*ptr_to_globals.offset(-1i32 as isize)).break_ptr;
        chain_expr((*(ptr_to_globals as *mut globals2)).t_info);
      }
      2304 => {
        n = chain_node(OC_EXEC as libc::c_int as uint32_t);
        (*n).a.n = (*ptr_to_globals.offset(-1i32 as isize)).continue_ptr;
        chain_expr((*(ptr_to_globals as *mut globals2)).t_info);
      }
      _ => {
        /* delete, next, nextfile, return, exit */
        chain_expr((*(ptr_to_globals as *mut globals2)).t_info);
      }
    }
  };
}
unsafe extern "C" fn parse_program(mut p: *mut libc::c_char) {
  let mut tclass: uint32_t = 0;
  let mut cn: *mut node = 0 as *mut node;
  let mut f: *mut func = 0 as *mut func;
  let mut v: *mut var = 0 as *mut var;
  let ref mut fresh38 = (*ptr_to_globals.offset(-1i32 as isize)).g_pos;
  *fresh38 = p;
  (*(ptr_to_globals as *mut globals2)).t_lineno = 1i32;
  loop {
    tclass = next_token(
      (1i32 << 25i32
        | (1i32 << 26i32
          | 1i32 << 27i32
          | 1i32 << 28i32
          | 1i32 << 19i32
          | 1i32 << 20i32
          | 1i32 << 21i32
          | 1i32 << 0i32
          | 1i32 << 29i32
          | 1i32 << 30i32
          | (1i32 << 5i32 | 1i32 << 10i32)
          | 1i32 << 2i32)
        | 1i32 << 12i32
        | (1i32 << 14i32 | 1i32 << 15i32)
        | 1i32 << 23i32
        | 1i32 << 24i32
        | 1i32 << 22i32) as uint32_t,
    );
    if !(tclass != (1i32 << 25i32) as libc::c_uint) {
      break;
    }
    if tclass & (1i32 << 14i32 | 1i32 << 15i32) as libc::c_uint != 0 {
      continue;
    }
    let ref mut fresh39 = (*ptr_to_globals.offset(-1i32 as isize)).seq;
    *fresh39 = &mut (*ptr_to_globals.offset(-1i32 as isize)).mainseq;
    if tclass & (1i32 << 23i32) as libc::c_uint != 0 {
      let ref mut fresh40 = (*ptr_to_globals.offset(-1i32 as isize)).seq;
      *fresh40 = &mut (*ptr_to_globals.offset(-1i32 as isize)).beginseq;
      chain_group();
    } else if tclass & (1i32 << 24i32) as libc::c_uint != 0 {
      let ref mut fresh41 = (*ptr_to_globals.offset(-1i32 as isize)).seq;
      *fresh41 = &mut (*ptr_to_globals.offset(-1i32 as isize)).endseq;
      chain_group();
    } else if tclass & (1i32 << 22i32) as libc::c_uint != 0 {
      next_token((1i32 << 28i32) as uint32_t);
      let ref mut fresh42 = (*ptr_to_globals.offset(-1i32 as isize)).g_pos;
      *fresh42 = (*fresh42).offset(1);
      f = hash_find(
        (*ptr_to_globals.offset(-1i32 as isize)).fnhash,
        (*(ptr_to_globals as *mut globals2)).t_string,
      ) as *mut func;
      (*f).body.first = 0 as *mut node_s;
      (*f).nargs = 0i32 as libc::c_uint;
      /* Match func arg list: a comma sep list of >= 0 args, and a close paren */
      while next_token((1i32 << 26i32 | 1i32 << 1i32 | 1i32 << 8i32) as uint32_t) != 0 {
        /* Either an empty arg list, or trailing comma from prev iter
         * must be followed by an arg */
        if (*f).nargs == 0i32 as libc::c_uint
          && (*(ptr_to_globals as *mut globals2)).t_tclass == (1i32 << 1i32) as libc::c_uint
        {
          break;
        }
        /* TC_SEQSTART/TC_COMMA must be followed by TC_VARIABLE */
        if (*(ptr_to_globals as *mut globals2)).t_tclass != (1i32 << 26i32) as libc::c_uint {
          syntax_error(EMSG_UNEXP_TOKEN.as_ptr());
        }
        v = hash_find(
          (*ptr_to_globals.offset(-1i32 as isize)).ahash,
          (*(ptr_to_globals as *mut globals2)).t_string,
        ) as *mut var;
        let fresh43 = (*f).nargs;
        (*f).nargs = (*f).nargs.wrapping_add(1);
        (*v).x.aidx = fresh43 as libc::c_int;
        /* Arg followed either by end of arg list or 1 comma */
        if next_token((1i32 << 8i32 | 1i32 << 1i32) as uint32_t) & (1i32 << 1i32) as libc::c_uint
          != 0
        {
          break;
        }
        if (*(ptr_to_globals as *mut globals2)).t_tclass != (1i32 << 8i32) as libc::c_uint {
          syntax_error(EMSG_UNEXP_TOKEN.as_ptr());
        }
      }
      let ref mut fresh44 = (*ptr_to_globals.offset(-1i32 as isize)).seq;
      *fresh44 = &mut (*f).body;
      chain_group();
      clear_array((*ptr_to_globals.offset(-1i32 as isize)).ahash);
    } else if tclass
      & (1i32 << 26i32
        | 1i32 << 27i32
        | 1i32 << 28i32
        | 1i32 << 19i32
        | 1i32 << 20i32
        | 1i32 << 21i32
        | 1i32 << 0i32
        | 1i32 << 29i32
        | 1i32 << 30i32
        | (1i32 << 5i32 | 1i32 << 10i32)
        | 1i32 << 2i32) as libc::c_uint
      != 0
    {
      rollback_token();
      cn = chain_node(OC_TEST as libc::c_int as uint32_t);
      (*cn).l.n =
        parse_expr((1i32 << 14i32 | 1i32 << 15i32 | 1i32 << 25i32 | 1i32 << 12i32) as uint32_t);
      if (*(ptr_to_globals as *mut globals2)).t_tclass & (1i32 << 12i32) as libc::c_uint != 0 {
        rollback_token();
        chain_group();
      } else {
        chain_node(OC_PRINT as libc::c_int as uint32_t);
      }
      (*cn).r.n = (*ptr_to_globals.offset(-1i32 as isize)).mainseq.last
    } else {
      /* if (tclass & TC_GRPSTART) */
      rollback_token();
      chain_group();
    }
  }
}
/* -------- program execution part -------- */
unsafe extern "C" fn mk_splitter(mut s: *const libc::c_char, mut spl: *mut tsplitter) -> *mut node {
  let mut re: *mut regex_t = 0 as *mut regex_t;
  let mut ire: *mut regex_t = 0 as *mut regex_t;
  let mut n: *mut node = 0 as *mut node;
  re = &mut *(*spl).re.as_mut_ptr().offset(0) as *mut regex_t;
  ire = &mut *(*spl).re.as_mut_ptr().offset(1) as *mut regex_t;
  n = &mut (*spl).n;
  if (*n).info & 0xff00i32 as libc::c_uint == OC_REGEXP as libc::c_int as libc::c_uint {
    regfree(re);
    regfree(ire);
    // TODO: nuke ire, use re+1?
  }
  if *s.offset(0) as libc::c_int != 0 && *s.offset(1) as libc::c_int != 0 {
    /* strlen(s) > 1 */
    mk_re_node(s, n, re);
  } else {
    (*n).info = *s.offset(0) as uint32_t
  }
  return n;
}
/* use node as a regular expression. Supplied with node ptr and regex_t
 * storage space. Return ptr to regex (if result points to preg, it should
 * be later regfree'd manually
 */
unsafe extern "C" fn as_regex(mut op: *mut node, mut preg: *mut regex_t) -> *mut regex_t {
  let mut cflags: libc::c_int = 0;
  let mut v: *mut var = 0 as *mut var;
  let mut s: *const libc::c_char = 0 as *const libc::c_char;
  if (*op).info & 0xff00i32 as libc::c_uint == OC_REGEXP as libc::c_int as libc::c_uint {
    return if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
      (*op).r.ire
    } else {
      (*op).l.re
    };
  }
  v = nvalloc(1i32);
  s = getvar_s(evaluate(op, v));
  cflags = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
    (1i32) | 1i32 << 1i32
  } else {
    1i32
  };
  /* Testcase where REG_EXTENDED fails (unpaired '{'):
   * echo Hi | awk 'gsub("@(samp|code|file)\{","");'
   * gawk 3.1.5 eats this. We revert to ~REG_EXTENDED
   * (maybe gsub is not supposed to use REG_EXTENDED?).
   */
  if regcomp(preg, s, cflags) != 0 {
    cflags &= !1i32;
    xregcomp(preg, s, cflags);
  }
  nvfree(v);
  return preg;
}
/* gradually increasing buffer.
 * note that we reallocate even if n == old_size,
 * and thus there is at least one extra allocated byte.
 */
unsafe extern "C" fn qrealloc(
  mut b: *mut libc::c_char,
  mut n: libc::c_int,
  mut size: *mut libc::c_int,
) -> *mut libc::c_char {
  if b.is_null() || n >= *size {
    *size = n + (n >> 1i32) + 80i32;
    b = xrealloc(b as *mut libc::c_void, *size as size_t) as *mut libc::c_char
  }
  return b;
}
/* resize field storage space */
unsafe extern "C" fn fsrealloc(mut size: libc::c_int) {
  let mut i: libc::c_int = 0;
  if size >= (*ptr_to_globals.offset(-1i32 as isize)).maxfields {
    i = (*ptr_to_globals.offset(-1i32 as isize)).maxfields;
    (*ptr_to_globals.offset(-1i32 as isize)).maxfields = size + 16i32;
    let ref mut fresh45 = (*ptr_to_globals.offset(-1i32 as isize)).Fields;
    *fresh45 = xrealloc(
      (*ptr_to_globals.offset(-1i32 as isize)).Fields as *mut libc::c_void,
      ((*ptr_to_globals.offset(-1i32 as isize)).maxfields as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<var>() as libc::c_ulong),
    ) as *mut var;
    while i < (*ptr_to_globals.offset(-1i32 as isize)).maxfields {
      (*(*ptr_to_globals.offset(-1i32 as isize))
        .Fields
        .offset(i as isize))
      .type_0 = 0x400i32 as libc::c_uint;
      let ref mut fresh46 = (*(*ptr_to_globals.offset(-1i32 as isize))
        .Fields
        .offset(i as isize))
      .string;
      *fresh46 = 0 as *mut libc::c_char;
      i += 1
    }
  }
  /* if size < nfields, clear extra field variables */
  i = size; // TODO: why [2]? [1] is enough...
  while i < (*ptr_to_globals.offset(-1i32 as isize)).nfields {
    clrvar(
      (*ptr_to_globals.offset(-1i32 as isize))
        .Fields
        .offset(i as isize),
    );
    i += 1
  }
  (*ptr_to_globals.offset(-1i32 as isize)).nfields = size;
}
unsafe extern "C" fn awk_split(
  mut s: *const libc::c_char,
  mut spl: *mut node,
  mut slist: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut l: libc::c_int = 0;
  let mut n: libc::c_int = 0;
  let mut c: [libc::c_char; 4] = [0; 4];
  let mut s1: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut pmatch: [regmatch_t; 2] = [regmatch_t { rm_so: 0, rm_eo: 0 }; 2];
  /* in worst case, each char would be a separate field */
  s1 = xzalloc(
    strlen(s)
      .wrapping_mul(2i32 as libc::c_ulong)
      .wrapping_add(3i32 as libc::c_ulong),
  ) as *mut libc::c_char;
  *slist = s1;
  strcpy(s1, s);
  c[1] = (*spl).info as libc::c_char;
  c[0] = c[1];
  c[3] = '\u{0}' as i32 as libc::c_char;
  c[2] = c[3];
  if *getvar_s((*(ptr_to_globals as *mut globals2)).intvar[RS as libc::c_int as usize])
    as libc::c_int
    == '\u{0}' as i32
  {
    c[2] = '\n' as i32 as libc::c_char
  }
  n = 0i32;
  if (*spl).info & 0xff00i32 as libc::c_uint == OC_REGEXP as libc::c_int as libc::c_uint {
    /* regex split */
    if *s == 0 {
      return n;
    } /* "": zero fields */
    n += 1; /* at least one field will be there */
    loop {
      l = strcspn(s, c.as_mut_ptr().offset(2)) as libc::c_int; /* len till next NUL or \n */
      if regexec(
        if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
          (*spl).r.ire
        } else {
          (*spl).l.re
        },
        s,
        1i32 as size_t,
        pmatch.as_mut_ptr(),
        0i32,
      ) == 0i32
        && pmatch[0].rm_so <= l
      {
        l = pmatch[0].rm_so;
        if pmatch[0].rm_eo == 0i32 {
          l += 1;
          pmatch[0].rm_eo += 1
        }
        n += 1
      /* we saw yet another delimiter */
      } else {
        pmatch[0].rm_eo = l;
        if *s.offset(l as isize) != 0 {
          pmatch[0].rm_eo += 1
        }
      }
      memcpy(
        s1 as *mut libc::c_void,
        s as *const libc::c_void,
        l as libc::c_ulong,
      );
      loop
      /* make sure we remove *all* of the separator chars */
      {
        *s1.offset(l as isize) = '\u{0}' as i32 as libc::c_char;
        l += 1;
        if !(l < pmatch[0].rm_eo) {
          break;
        }
      }
      nextword(&mut s1);
      s = s.offset(pmatch[0].rm_eo as isize);
      if !(*s != 0) {
        break;
      }
    }
    return n;
  }
  if c[0] as libc::c_int == '\u{0}' as i32 {
    /* null split */
    while *s != 0 {
      let fresh47 = s;
      s = s.offset(1);
      let fresh48 = s1;
      s1 = s1.offset(1);
      *fresh48 = *fresh47;
      let fresh49 = s1;
      s1 = s1.offset(1);
      *fresh49 = '\u{0}' as i32 as libc::c_char;
      n += 1
    }
    return n;
  }
  if c[0] as libc::c_int != ' ' as i32 {
    /* single-character split */
    if (*ptr_to_globals.offset(-1i32 as isize)).icase != 0 {
      c[0] = bb_ascii_toupper(c[0] as libc::c_uchar) as libc::c_char;
      c[1] = bb_ascii_tolower(c[1] as libc::c_uchar) as libc::c_char
    }
    if *s1 != 0 {
      n += 1
    }
    loop {
      s1 = strpbrk(s1, c.as_mut_ptr());
      if s1.is_null() {
        break;
      }
      let fresh50 = s1;
      s1 = s1.offset(1);
      *fresh50 = '\u{0}' as i32 as libc::c_char;
      n += 1
    }
    return n;
  }
  /* space split */
  while *s != 0 {
    s = skip_whitespace(s);
    if *s == 0 {
      break;
    }
    n += 1;
    while *s as libc::c_int != 0
      && ({
        let mut bb__isspace: libc::c_uchar = (*s as libc::c_int - 9i32) as libc::c_uchar;
        (bb__isspace as libc::c_int == ' ' as i32 - 9i32
          || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
      }) == 0
    {
      let fresh51 = s;
      s = s.offset(1);
      let fresh52 = s1;
      s1 = s1.offset(1);
      *fresh52 = *fresh51
    }
    let fresh53 = s1;
    s1 = s1.offset(1);
    *fresh53 = '\u{0}' as i32 as libc::c_char
  }
  return n;
}
unsafe extern "C" fn split_f0() {
  /* static char *fstrings; */
  let mut i: libc::c_int = 0;
  let mut n: libc::c_int = 0;
  let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
  if (*ptr_to_globals.offset(-1i32 as isize)).is_f0_split != 0 {
    return;
  }
  (*ptr_to_globals.offset(-1i32 as isize)).is_f0_split = 1i32 as smallint;
  free((*(ptr_to_globals as *mut globals2)).split_f0__fstrings as *mut libc::c_void);
  fsrealloc(0i32);
  n = awk_split(
    getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]),
    &mut (*(ptr_to_globals as *mut globals2)).fsplitter.n,
    &mut (*(ptr_to_globals as *mut globals2)).split_f0__fstrings,
  );
  fsrealloc(n);
  s = (*(ptr_to_globals as *mut globals2)).split_f0__fstrings;
  i = 0i32;
  while i < n {
    let ref mut fresh54 = (*(*ptr_to_globals.offset(-1i32 as isize))
      .Fields
      .offset(i as isize))
    .string;
    *fresh54 = nextword(&mut s);
    (*(*ptr_to_globals.offset(-1i32 as isize))
      .Fields
      .offset(i as isize))
    .type_0 |= (0x1000i32 | 0x200i32 | 0x4000i32) as libc::c_uint;
    i += 1
  }
  /* set NF manually to avoid side effects */
  clrvar((*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize]);
  (*(*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize]).type_0 =
    (0x1i32 | 0x400i32) as libc::c_uint;
  (*(*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize]).number =
    (*ptr_to_globals.offset(-1i32 as isize)).nfields as libc::c_double;
}
/* For debug. nm --size-sort awk.o | grep -vi ' [tr] ' */
/*char G1size[sizeof(G1)]; - 0x74 */
/*char Gsize[sizeof(G)]; - 0x1c4 */
/* Trying to keep most of members accessible with short offsets: */
/*char Gofs_seed[offsetof(struct globals2, evaluate__seed)]; - 0x90 */
/* function prototypes */
/* perform additional actions when some internal variables changed */
unsafe extern "C" fn handle_special(mut v: *mut var) {
  let mut n: libc::c_int = 0;
  let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut sep: *const libc::c_char = 0 as *const libc::c_char;
  let mut s: *const libc::c_char = 0 as *const libc::c_char;
  let mut sl: libc::c_int = 0;
  let mut l: libc::c_int = 0;
  let mut len: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  let mut bsize: libc::c_int = 0;
  if (*v).type_0 & 0x400i32 as libc::c_uint == 0 {
    return;
  }
  if v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
    n = getvar_i(v) as libc::c_int;
    if n < 0i32 {
      syntax_error(b"NF set to negative value\x00" as *const u8 as *const libc::c_char);
    }
    fsrealloc(n);
    /* recalculate $0 */
    sep = getvar_s((*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize]);
    sl = strlen(sep) as libc::c_int;
    b = 0 as *mut libc::c_char;
    len = 0i32;
    i = 0i32;
    while i < n {
      s = getvar_s(
        &mut *(*ptr_to_globals.offset(-1i32 as isize))
          .Fields
          .offset(i as isize),
      );
      l = strlen(s) as libc::c_int;
      if !b.is_null() {
        memcpy(
          b.offset(len as isize) as *mut libc::c_void,
          sep as *const libc::c_void,
          sl as libc::c_ulong,
        );
        len += sl
      }
      b = qrealloc(b, len + l + sl, &mut bsize);
      memcpy(
        b.offset(len as isize) as *mut libc::c_void,
        s as *const libc::c_void,
        l as libc::c_ulong,
      );
      len += l;
      i += 1
    }
    if !b.is_null() {
      *b.offset(len as isize) = '\u{0}' as i32 as libc::c_char
    }
    setvar_p(
      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
      b,
    );
    (*ptr_to_globals.offset(-1i32 as isize)).is_f0_split = 1i32 as smallint
  } else if v == (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize] {
    (*ptr_to_globals.offset(-1i32 as isize)).is_f0_split = 0i32 as smallint
  } else if v == (*(ptr_to_globals as *mut globals2)).intvar[FS as libc::c_int as usize] {
    /*
     * The POSIX-2008 standard says that changing FS should have no effect on the
     * current input line, but only on the next one. The language is:
     *
     * > Before the first reference to a field in the record is evaluated, the record
     * > shall be split into fields, according to the rules in Regular Expressions,
     * > using the value of FS that was current at the time the record was read.
     *
     * So, split up current line before assignment to FS:
     */
    split_f0(); /* $n */
    mk_splitter(
      getvar_s(v),
      &mut (*(ptr_to_globals as *mut globals2)).fsplitter,
    );
  } else if v == (*(ptr_to_globals as *mut globals2)).intvar[RS as libc::c_int as usize] {
    mk_splitter(
      getvar_s(v),
      &mut (*(ptr_to_globals as *mut globals2)).rsplitter,
    );
  } else if v == (*(ptr_to_globals as *mut globals2)).intvar[IGNORECASE as libc::c_int as usize] {
    (*ptr_to_globals.offset(-1i32 as isize)).icase = istrue(v) as smallint
  } else {
    n = getvar_i((*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize])
      as libc::c_int;
    setvar_i(
      (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize],
      if n as libc::c_long
        > v.wrapping_offset_from((*ptr_to_globals.offset(-1i32 as isize)).Fields) as libc::c_long
      {
        n as libc::c_long
      } else {
        (v.wrapping_offset_from((*ptr_to_globals.offset(-1i32 as isize)).Fields) as libc::c_long)
          + 1i32 as libc::c_long
      } as libc::c_double,
    );
    /* right here v is invalid. Just to note... */
  };
}
/* step through func/builtin/etc arguments */
unsafe extern "C" fn nextarg(mut pn: *mut *mut node) -> *mut node {
  let mut n: *mut node = 0 as *mut node; /* why + 1? */
  n = *pn;
  if !n.is_null()
    && (*n).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
  {
    *pn = (*n).r.n;
    n = (*n).l.n
  } else {
    *pn = 0 as *mut node
  }
  return n;
}
unsafe extern "C" fn hashwalk_init(mut v: *mut var, mut array: *mut xhash) {
  let mut hi: *mut hash_item = 0 as *mut hash_item;
  let mut i: libc::c_uint = 0;
  let mut w: *mut walker_list = 0 as *mut walker_list;
  let mut prev_walker: *mut walker_list = 0 as *mut walker_list;
  if (*v).type_0 & 0x800i32 as libc::c_uint != 0 {
    prev_walker = (*v).x.walker
  } else {
    (*v).type_0 |= 0x800i32 as libc::c_uint;
    prev_walker = 0 as *mut walker_list
  }
  (*v).x.walker = xzalloc(
    (::std::mem::size_of::<walker_list>() as libc::c_ulong)
      .wrapping_add((*array).glen as libc::c_ulong)
      .wrapping_add(1i32 as libc::c_ulong),
  ) as *mut walker_list;
  w = (*v).x.walker;
  (*w).end = (*w).wbuf.as_mut_ptr();
  (*w).cur = (*w).end;
  (*w).prev = prev_walker;
  i = 0i32 as libc::c_uint;
  while i < (*array).csize {
    hi = *(*array).items.offset(i as isize);
    while !hi.is_null() {
      strcpy((*w).end, (*hi).name.as_mut_ptr());
      nextword(&mut (*w).end);
      hi = (*hi).next
    }
    i = i.wrapping_add(1)
  }
}
unsafe extern "C" fn hashwalk_next(mut v: *mut var) -> libc::c_int {
  let mut w: *mut walker_list = (*v).x.walker;
  if (*w).cur >= (*w).end {
    let mut prev_walker: *mut walker_list = (*w).prev;
    free(w as *mut libc::c_void);
    (*v).x.walker = prev_walker;
    return 0i32;
  }
  setvar_s(v, nextword(&mut (*w).cur));
  return 1i32;
}
/* evaluate node, return 1 when result is true, 0 otherwise */
unsafe extern "C" fn ptest(mut pattern: *mut node) -> libc::c_int {
  /* ptest__v is "static": to save stack space? */
  return istrue(evaluate(
    pattern,
    &mut (*(ptr_to_globals as *mut globals2)).ptest__v,
  ));
}
/* read next record from stream rsm into a variable v */
unsafe extern "C" fn awk_getline(mut rsm: *mut rstream, mut v: *mut var) -> libc::c_int {
  let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut pmatch: [regmatch_t; 2] = [regmatch_t { rm_so: 0, rm_eo: 0 }; 2];
  let mut size: libc::c_int = 0;
  let mut a: libc::c_int = 0;
  let mut p: libc::c_int = 0;
  let mut pp: libc::c_int = 0i32;
  let mut fd: libc::c_int = 0;
  let mut so: libc::c_int = 0;
  let mut eo: libc::c_int = 0;
  let mut r: libc::c_int = 0;
  let mut rp: libc::c_int = 0;
  let mut c: libc::c_char = 0;
  let mut m: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
  /* we're using our own buffer since we need access to accumulating
   * characters
   */
  fd = fileno_unlocked((*rsm).F);
  m = (*rsm).buffer;
  a = (*rsm).adv;
  p = (*rsm).pos;
  size = (*rsm).size;
  c = (*(ptr_to_globals as *mut globals2)).rsplitter.n.info as libc::c_char;
  rp = 0i32;
  if m.is_null() {
    m = qrealloc(m, 256i32, &mut size)
  }
  loop {
    b = m.offset(a as isize);
    eo = p;
    so = eo;
    r = 1i32;
    if p > 0i32 {
      if (*(ptr_to_globals as *mut globals2)).rsplitter.n.info & 0xff00i32 as libc::c_uint
        == OC_REGEXP as libc::c_int as libc::c_uint
      {
        if regexec(
          if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
            (*(ptr_to_globals as *mut globals2)).rsplitter.n.r.ire
          } else {
            (*(ptr_to_globals as *mut globals2)).rsplitter.n.l.re
          },
          b,
          1i32 as size_t,
          pmatch.as_mut_ptr(),
          0i32,
        ) == 0i32
        {
          so = pmatch[0].rm_so;
          eo = pmatch[0].rm_eo;
          if *b.offset(eo as isize) as libc::c_int != '\u{0}' as i32 {
            break;
          }
        }
      } else if c as libc::c_int != '\u{0}' as i32 {
        s = strchr(b.offset(pp as isize), c as libc::c_int);
        if s.is_null() {
          s = memchr(
            b.offset(pp as isize) as *const libc::c_void,
            '\u{0}' as i32,
            (p - pp) as libc::c_ulong,
          ) as *mut libc::c_char
        }
        if !s.is_null() {
          eo = s.wrapping_offset_from(b) as libc::c_long as libc::c_int;
          so = eo;
          eo += 1;
          break;
        }
      } else {
        while *b.offset(rp as isize) as libc::c_int == '\n' as i32 {
          rp += 1
        }
        s = strstr(
          b.offset(rp as isize),
          b"\n\n\x00" as *const u8 as *const libc::c_char,
        );
        if !s.is_null() {
          eo = s.wrapping_offset_from(b) as libc::c_long as libc::c_int;
          so = eo;
          while *b.offset(eo as isize) as libc::c_int == '\n' as i32 {
            eo += 1
          }
          if *b.offset(eo as isize) as libc::c_int != '\u{0}' as i32 {
            break;
          }
        }
      }
    }
    if a > 0i32 {
      memmove(
        m as *mut libc::c_void,
        m.offset(a as isize) as *const libc::c_void,
        (p + 1i32) as libc::c_ulong,
      );
      b = m;
      a = 0i32
    }
    m = qrealloc(m, a + p + 128i32, &mut size);
    b = m.offset(a as isize);
    pp = p;
    p = (p as libc::c_long
      + safe_read(
        fd,
        b.offset(p as isize) as *mut libc::c_void,
        (size - p - 1i32) as size_t,
      )) as libc::c_int;
    if p < pp {
      p = 0i32;
      r = 0i32;
      setvar_i(
        (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
        *bb_errno as libc::c_double,
      );
    }
    *b.offset(p as isize) = '\u{0}' as i32 as libc::c_char;
    if !(p > pp) {
      break;
    }
  }
  if p == 0i32 {
    r -= 1
  } else {
    c = *b.offset(so as isize);
    *b.offset(so as isize) = '\u{0}' as i32 as libc::c_char;
    setvar_s(v, b.offset(rp as isize));
    (*v).type_0 |= 0x200i32 as libc::c_uint;
    *b.offset(so as isize) = c;
    c = *b.offset(eo as isize);
    *b.offset(eo as isize) = '\u{0}' as i32 as libc::c_char;
    setvar_s(
      (*(ptr_to_globals as *mut globals2)).intvar[RT as libc::c_int as usize],
      b.offset(so as isize),
    );
    *b.offset(eo as isize) = c
  }
  (*rsm).buffer = m;
  (*rsm).adv = a + eo;
  (*rsm).pos = p - eo;
  (*rsm).size = size;
  return r;
}
unsafe extern "C" fn fmt_num(
  mut b: *mut libc::c_char,
  mut size: libc::c_int,
  mut format: *const libc::c_char,
  mut n: libc::c_double,
  mut int_as_int: libc::c_int,
) -> libc::c_int {
  let mut r: libc::c_int = 0i32;
  let mut c: libc::c_char = 0;
  let mut s: *const libc::c_char = format;
  if int_as_int != 0 && n == n as libc::c_longlong as libc::c_double {
    r = snprintf(
      b,
      size as libc::c_ulong,
      b"%lld\x00" as *const u8 as *const libc::c_char,
      n as libc::c_longlong,
    )
  } else {
    loop {
      c = *s;
      if !(c as libc::c_int != 0 && {
        s = s.offset(1);
        (*s as libc::c_int) != 0
      }) {
        break;
      }
    }
    if !strchr(
      b"diouxX\x00" as *const u8 as *const libc::c_char,
      c as libc::c_int,
    )
    .is_null()
    {
      r = snprintf(b, size as libc::c_ulong, format, n as libc::c_int)
    } else if !strchr(
      b"eEfgG\x00" as *const u8 as *const libc::c_char,
      c as libc::c_int,
    )
    .is_null()
    {
      r = snprintf(b, size as libc::c_ulong, format, n)
    } else {
      syntax_error(EMSG_INV_FMT.as_ptr());
    }
  }
  return r;
}
/* formatted output into an allocated buffer, return ptr to buffer */
unsafe extern "C" fn awk_printf(mut n: *mut node) -> *mut libc::c_char {
  let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut fmt: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut f: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut s1: *const libc::c_char = 0 as *const libc::c_char;
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  let mut incr: libc::c_int = 0;
  let mut bsize: libc::c_int = 0;
  let mut c: libc::c_char = 0;
  let mut c1: libc::c_char = 0;
  let mut v: *mut var = 0 as *mut var;
  let mut arg: *mut var = 0 as *mut var;
  v = nvalloc(1i32);
  f = xstrdup(getvar_s(evaluate(nextarg(&mut n), v)));
  fmt = f;
  i = 0i32;
  while *f != 0 {
    s = f;
    while *f as libc::c_int != 0
      && (*f as libc::c_int != '%' as i32 || {
        f = f.offset(1);
        (*f as libc::c_int) == '%' as i32
      })
    {
      f = f.offset(1)
    }
    while *f as libc::c_int != 0
      && !(((*f as libc::c_int | 0x20i32) - 'a' as i32) as libc::c_uchar as libc::c_int
        <= 'z' as i32 - 'a' as i32)
    {
      if *f as libc::c_int == '*' as i32 {
        syntax_error(b"%*x formats are not supported\x00" as *const u8 as *const libc::c_char);
      }
      f = f.offset(1)
    }
    incr = (f.wrapping_offset_from(s) as libc::c_long + 240i32 as libc::c_long) as libc::c_int;
    b = qrealloc(b, incr + i, &mut bsize);
    c = *f;
    if c as libc::c_int != '\u{0}' as i32 {
      f = f.offset(1)
    }
    c1 = *f;
    *f = '\u{0}' as i32 as libc::c_char;
    arg = evaluate(nextarg(&mut n), v);
    j = i;
    if c as libc::c_int == 'c' as i32 || c == 0 {
      i += sprintf(
        b.offset(i as isize),
        s,
        if is_numeric(arg) != 0 {
          getvar_i(arg) as libc::c_char as libc::c_int
        } else {
          *getvar_s(arg) as libc::c_int
        },
      )
    } else if c as libc::c_int == 's' as i32 {
      s1 = getvar_s(arg);
      b = qrealloc(
        b,
        ((incr + i) as libc::c_ulong).wrapping_add(strlen(s1)) as libc::c_int,
        &mut bsize,
      );
      i += sprintf(b.offset(i as isize), s, s1)
    } else {
      i += fmt_num(b.offset(i as isize), incr, s, getvar_i(arg), 0i32)
    }
    *f = c1;
    /* if there was an error while sprintf, return value is negative */
    if i < j {
      i = j
    }
  }
  free(fmt as *mut libc::c_void);
  nvfree(v);
  b = xrealloc(b as *mut libc::c_void, (i + 1i32) as size_t) as *mut libc::c_char;
  *b.offset(i as isize) = '\u{0}' as i32 as libc::c_char;
  return b;
}
/* Common substitution routine.
 * Replace (nm)'th substring of (src) that matches (rn) with (repl),
 * store result into (dest), return number of substitutions.
 * If nm = 0, replace all matches.
 * If src or dst is NULL, use $0.
 * If subexp != 0, enable subexpression matching (\1-\9).
 */
unsafe extern "C" fn awk_sub(
  mut rn: *mut node,
  mut repl: *const libc::c_char,
  mut nm: libc::c_int,
  mut src: *mut var,
  mut dest: *mut var,
  mut subexp: libc::c_int,
) -> libc::c_int {
  let mut current_block: u64;
  let mut resbuf: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut sp: *const libc::c_char = 0 as *const libc::c_char;
  let mut match_no: libc::c_int = 0;
  let mut residx: libc::c_int = 0;
  let mut replen: libc::c_int = 0;
  let mut resbufsize: libc::c_int = 0;
  let mut regexec_flags: libc::c_int = 0;
  let mut pmatch: [regmatch_t; 10] = [regmatch_t { rm_so: 0, rm_eo: 0 }; 10];
  let mut sreg: regex_t = regex_t {
    buffer: 0 as *mut libc::c_uchar,
    allocated: 0,
    used: 0,
    syntax: 0,
    fastmap: 0 as *mut libc::c_char,
    translate: 0 as *mut libc::c_uchar,
    re_nsub: 0,
    can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
    c2rust_padding: [0; 7],
  };
  let mut regex: *mut regex_t = 0 as *mut regex_t;
  resbuf = 0 as *mut libc::c_char;
  residx = 0i32;
  match_no = 0i32;
  regexec_flags = 0i32;
  regex = as_regex(rn, &mut sreg);
  sp = getvar_s(if !src.is_null() {
    src
  } else {
    (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
  });
  replen = strlen(repl) as libc::c_int;
  loop {
    if !(regexec(
      regex,
      sp,
      10i32 as size_t,
      pmatch.as_mut_ptr(),
      regexec_flags,
    ) == 0i32)
    {
      current_block = 6476622998065200121;
      break;
    }
    let mut so: libc::c_int = pmatch[0].rm_so;
    let mut eo: libc::c_int = pmatch[0].rm_eo;
    //bb_error_msg("match %u: [%u,%u] '%s'%p", match_no+1, so, eo, sp,sp);
    resbuf = qrealloc(resbuf, residx + eo + replen, &mut resbufsize);
    memcpy(
      resbuf.offset(residx as isize) as *mut libc::c_void,
      sp as *const libc::c_void,
      eo as libc::c_ulong,
    );
    residx += eo;
    match_no += 1;
    if match_no >= nm {
      let mut s: *const libc::c_char = 0 as *const libc::c_char;
      let mut nbs: libc::c_int = 0;
      /* replace */
      residx -= eo - so;
      nbs = 0i32;
      s = repl;
      while *s != 0 {
        let fresh55 = residx;
        residx = residx + 1;
        let ref mut fresh56 = *resbuf.offset(fresh55 as isize);
        *fresh56 = *s;
        let mut c: libc::c_char = *fresh56;
        if c as libc::c_int == '\\' as i32 {
          nbs += 1
        } else {
          if c as libc::c_int == '&' as i32
            || subexp != 0 && c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32
          {
            let mut j: libc::c_int = 0;
            residx -= nbs + 3i32 >> 1i32;
            j = 0i32;
            if c as libc::c_int != '&' as i32 {
              j = c as libc::c_int - '0' as i32;
              nbs += 1
            }
            if nbs % 2i32 != 0 {
              let fresh57 = residx;
              residx = residx + 1;
              *resbuf.offset(fresh57 as isize) = c
            } else {
              let mut n: libc::c_int = pmatch[j as usize].rm_eo - pmatch[j as usize].rm_so;
              resbuf = qrealloc(resbuf, residx + replen + n, &mut resbufsize);
              memcpy(
                resbuf.offset(residx as isize) as *mut libc::c_void,
                sp.offset(pmatch[j as usize].rm_so as isize) as *const libc::c_void,
                n as libc::c_ulong,
              );
              residx += n
            }
          }
          nbs = 0i32
        }
        s = s.offset(1)
      }
    }
    regexec_flags = 1i32;
    sp = sp.offset(eo as isize);
    if match_no == nm {
      current_block = 6476622998065200121;
      break;
    }
    if !(eo == so) {
      continue;
    }
    /* Empty match (e.g. "b*" will match anywhere).
     * Advance by one char. */
    //BUG (bug 1333):
    //gsub(/\<b*/,"") on "abc" will reach this point, advance to "bc"
    //... and will erroneously match "b" even though it is NOT at the word start.
    //we need REG_NOTBOW but it does not exist...
    //TODO: if EXTRA_COMPAT=y, use GNU matching and re_search,
    //it should be able to do it correctly.
    /* Subtle: this is safe only because
     * qrealloc allocated at least one extra byte */
    *resbuf.offset(residx as isize) = *sp;
    if *sp as libc::c_int == '\u{0}' as i32 {
      current_block = 9111172387726641803;
      break;
    }
    sp = sp.offset(1);
    residx += 1
  }
  match current_block {
    6476622998065200121 => {
      resbuf = qrealloc(
        resbuf,
        (residx as libc::c_ulong).wrapping_add(strlen(sp)) as libc::c_int,
        &mut resbufsize,
      );
      strcpy(resbuf.offset(residx as isize), sp);
    }
    _ => {}
  }
  //bb_error_msg("end sp:'%s'%p", sp,sp);
  setvar_p(
    if !dest.is_null() {
      dest
    } else {
      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
    },
    resbuf,
  );
  if regex == &mut sreg as *mut regex_t {
    regfree(regex);
  }
  return match_no;
}
#[inline(never)]
unsafe extern "C" fn do_mktime(mut ds: *const libc::c_char) -> libc::c_int {
  let mut then: tm = tm {
    tm_sec: 0,
    tm_min: 0,
    tm_hour: 0,
    tm_mday: 0,
    tm_mon: 0,
    tm_year: 0,
    tm_wday: 0,
    tm_yday: 0,
    tm_isdst: 0,
    tm_gmtoff: 0,
    tm_zone: 0 as *const libc::c_char,
  };
  let mut count: libc::c_int = 0;
  /*memset(&then, 0, sizeof(then)); - not needed */
  then.tm_isdst = -1i32; /* default is unknown */
  /* manpage of mktime says these fields are ints,
   * so we can sscanf stuff directly into them */
  count = sscanf(
    ds,
    b"%u %u %u %u %u %u %d\x00" as *const u8 as *const libc::c_char,
    &mut then.tm_year as *mut libc::c_int,
    &mut then.tm_mon as *mut libc::c_int,
    &mut then.tm_mday as *mut libc::c_int,
    &mut then.tm_hour as *mut libc::c_int,
    &mut then.tm_min as *mut libc::c_int,
    &mut then.tm_sec as *mut libc::c_int,
    &mut then.tm_isdst as *mut libc::c_int,
  );
  if count < 6i32
    || (then.tm_mon as libc::c_uint) < 1i32 as libc::c_uint
    || (then.tm_year as libc::c_uint) < 1900i32 as libc::c_uint
  {
    return -1i32;
  }
  then.tm_mon -= 1i32;
  then.tm_year -= 1900i32;
  return mktime(&mut then) as libc::c_int;
}
#[inline(never)]
unsafe extern "C" fn exec_builtin(mut op: *mut node, mut res: *mut var) -> *mut var {
  let mut tv: *mut var = 0 as *mut var;
  let mut an: [*mut node; 4] = [0 as *mut node; 4];
  let mut av: [*mut var; 4] = [0 as *mut var; 4];
  let mut as_0: [*const libc::c_char; 4] = [0 as *const libc::c_char; 4];
  let mut pmatch: [regmatch_t; 2] = [regmatch_t { rm_so: 0, rm_eo: 0 }; 2];
  let mut sreg: regex_t = regex_t {
    buffer: 0 as *mut libc::c_uchar,
    allocated: 0,
    used: 0,
    syntax: 0,
    fastmap: 0 as *mut libc::c_char,
    translate: 0 as *mut libc::c_uchar,
    re_nsub: 0,
    can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
    c2rust_padding: [0; 7],
  };
  let mut re: *mut regex_t = 0 as *mut regex_t;
  let mut spl: *mut node = 0 as *mut node;
  let mut isr: uint32_t = 0;
  let mut info: uint32_t = 0;
  let mut nargs: libc::c_int = 0;
  let mut tt: time_t = 0;
  let mut i: libc::c_int = 0;
  let mut l: libc::c_int = 0;
  let mut ll: libc::c_int = 0;
  let mut n: libc::c_int = 0;
  tv = nvalloc(4i32);
  info = (*op).info;
  isr = info;
  op = (*op).l.n;
  av[3] = 0 as *mut var;
  av[2] = av[3];
  i = 0i32;
  while i < 4i32 && !op.is_null() {
    an[i as usize] = nextarg(&mut op);
    if isr & 0x9000000i32 as libc::c_uint != 0 {
      av[i as usize] = evaluate(an[i as usize], &mut *tv.offset(i as isize))
    }
    if isr & 0x8000000i32 as libc::c_uint != 0 {
      as_0[i as usize] = getvar_s(av[i as usize])
    }
    isr >>= 1i32;
    i += 1
  }
  nargs = i;
  if (nargs as uint32_t) < info >> 30i32 {
    syntax_error(EMSG_TOO_FEW_ARGS.as_ptr());
  }
  info &= 0x7fi32 as libc::c_uint;
  match info {
    0 => {
      setvar_i(res, atan2(getvar_i(av[0]), getvar_i(av[1])));
    }
    3 => {
      let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
      let mut s1: *mut libc::c_char = 0 as *mut libc::c_char;
      if nargs > 2i32 {
        spl =
          if (*an[2]).info & 0xff00i32 as libc::c_uint == OC_REGEXP as libc::c_int as libc::c_uint {
            an[2]
          } else {
            mk_splitter(
              getvar_s(evaluate(an[2], &mut *tv.offset(2))),
              &mut (*(ptr_to_globals as *mut globals2)).exec_builtin__tspl,
            )
          }
      } else {
        spl = &mut (*(ptr_to_globals as *mut globals2)).fsplitter.n
      }
      n = awk_split(as_0[0], spl, &mut s);
      s1 = s;
      clear_array(iamarray(av[1]));
      i = 1i32;
      while i <= n {
        setari_u(av[1], i, nextword(&mut s));
        i += 1
      }
      free(s1 as *mut libc::c_void);
      setvar_i(res, n as libc::c_double);
    }
    4 => {
      let mut s_0: *mut libc::c_char = 0 as *mut libc::c_char;
      l = strlen(as_0[0]) as libc::c_int;
      i = (getvar_i(av[1]) - 1i32 as libc::c_double) as libc::c_int;
      if i > l {
        i = l
      }
      if i < 0i32 {
        i = 0i32
      }
      n = if nargs > 2i32 {
        getvar_i(av[2])
      } else {
        (l - i) as libc::c_double
      } as libc::c_int;
      if n < 0i32 {
        n = 0i32
      }
      s_0 = xstrndup(as_0[0].offset(i as isize), n);
      setvar_p(res, s_0);
    }
    12 => {
      /* Bitwise ops must assume that operands are unsigned. GNU Awk 3.1.5:
       * awk '{ print or(-1,1) }' gives "4.29497e+09", not "-2.xxxe+09" */
      setvar_i(
        res,
        (getvar_i_int(av[0]) & getvar_i_int(av[1])) as libc::c_double,
      );
    }
    13 => {
      setvar_i(res, !getvar_i_int(av[0]) as libc::c_double);
    }
    14 => {
      setvar_i(
        res,
        (getvar_i_int(av[0]) << getvar_i_int(av[1])) as libc::c_double,
      );
    }
    15 => {
      setvar_i(
        res,
        (getvar_i_int(av[0]) | getvar_i_int(av[1])) as libc::c_double,
      );
    }
    16 => {
      setvar_i(
        res,
        (getvar_i_int(av[0]) >> getvar_i_int(av[1])) as libc::c_double,
      );
    }
    17 => {
      setvar_i(
        res,
        (getvar_i_int(av[0]) ^ getvar_i_int(av[1])) as libc::c_double,
      );
    }
    7 | 8 => {
      let mut s_1: *mut libc::c_char = 0 as *mut libc::c_char;
      let mut s1_0: *mut libc::c_char = 0 as *mut libc::c_char;
      s_1 = xstrdup(as_0[0]);
      s1_0 = s_1;
      while *s1_0 != 0 {
        //*s1 = (info == B_up) ? toupper(*s1) : tolower(*s1);
        if ((*s1_0 as libc::c_int | 0x20i32) - 'a' as i32) as libc::c_uchar as libc::c_int
          <= 'z' as i32 - 'a' as i32
        {
          *s1_0 = if info == B_up as libc::c_int as libc::c_uint {
            (*s1_0 as libc::c_int) & 0xdfi32
          } else {
            (*s1_0 as libc::c_int) | 0x20i32
          } as libc::c_char
        }
        s1_0 = s1_0.offset(1)
      }
      setvar_p(res, s_1);
    }
    1 => {
      n = 0i32;
      ll = strlen(as_0[1]) as libc::c_int;
      l = strlen(as_0[0]).wrapping_sub(ll as libc::c_ulong) as libc::c_int;
      if ll > 0i32 && l >= 0i32 {
        if (*ptr_to_globals.offset(-1i32 as isize)).icase == 0 {
          let mut s_2: *mut libc::c_char = strstr(as_0[0], as_0[1]);
          if !s_2.is_null() {
            n = (s_2.wrapping_offset_from(as_0[0]) as libc::c_long + 1i32 as libc::c_long)
              as libc::c_int
          }
        } else {
          /* this piece of code is terribly slow and
           * really should be rewritten
           */
          i = 0i32;
          while i <= l {
            if strncasecmp(as_0[0].offset(i as isize), as_0[1], ll as libc::c_ulong) == 0i32 {
              n = i + 1i32;
              break;
            } else {
              i += 1
            }
          }
        }
      }
      setvar_i(res, n as libc::c_double);
    }
    5 => {
      if nargs > 1i32 {
        tt = getvar_i(av[1]) as time_t
      } else {
        time(&mut tt);
      }
      //s = (nargs > 0) ? as[0] : "%a %b %d %H:%M:%S %Z %Y";
      i = strftime(
        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
        240i32 as size_t,
        if nargs > 0i32 {
          as_0[0]
        } else {
          b"%a %b %d %H:%M:%S %Z %Y\x00" as *const u8 as *const libc::c_char
        },
        localtime(&mut tt),
      ) as libc::c_int;
      *(*ptr_to_globals.offset(-1i32 as isize))
        .g_buf
        .offset(i as isize) = '\u{0}' as i32 as libc::c_char;
      setvar_s(res, (*ptr_to_globals.offset(-1i32 as isize)).g_buf);
    }
    6 => {
      setvar_i(res, do_mktime(as_0[0]) as libc::c_double);
    }
    2 => {
      re = as_regex(an[1], &mut sreg);
      n = regexec(re, as_0[0], 1i32 as size_t, pmatch.as_mut_ptr(), 0i32);
      if n == 0i32 {
        pmatch[0].rm_so += 1;
        pmatch[0].rm_eo += 1
      } else {
        pmatch[0].rm_so = 0i32;
        pmatch[0].rm_eo = -1i32
      }
      setvar_i(
        hash_find(
          (*ptr_to_globals.offset(-1i32 as isize)).vhash,
          b"RSTART\x00" as *const u8 as *const libc::c_char,
        ) as *mut var,
        pmatch[0].rm_so as libc::c_double,
      );
      setvar_i(
        hash_find(
          (*ptr_to_globals.offset(-1i32 as isize)).vhash,
          b"RLENGTH\x00" as *const u8 as *const libc::c_char,
        ) as *mut var,
        (pmatch[0].rm_eo - pmatch[0].rm_so) as libc::c_double,
      );
      setvar_i(res, pmatch[0].rm_so as libc::c_double);
      if re == &mut sreg as *mut regex_t {
        regfree(re);
      }
    }
    9 => {
      awk_sub(
        an[0],
        as_0[1],
        getvar_i(av[2]) as libc::c_int,
        av[3],
        res,
        1i32,
      );
    }
    10 => {
      setvar_i(
        res,
        awk_sub(an[0], as_0[1], 0i32, av[2], av[2], 0i32) as libc::c_double,
      );
    }
    11 => {
      setvar_i(
        res,
        awk_sub(an[0], as_0[1], 1i32, av[2], av[2], 0i32) as libc::c_double,
      );
    }
    _ => {}
  }
  nvfree(tv);
  return res;
}
/*
 * Evaluate node - the heart of the program. Supplied with subtree
 * and place where to store result. returns ptr to result.
 */
unsafe extern "C" fn evaluate(mut op: *mut node, mut res: *mut var) -> *mut var {
  let mut current_block: u64;
  /* This procedure is recursive so we should count every byte */
  /* seed is initialized to 1 */
  let mut v1: *mut var = 0 as *mut var; /* while (op) */
  if op.is_null() {
    return setvar_s(res, 0 as *const libc::c_char);
  } /* for compiler */
  v1 = nvalloc(2i32);
  while !op.is_null() {
    let mut L: C2RustUnnamed_10 = C2RustUnnamed_10 {
      v: 0 as *mut var,
      s: 0 as *const libc::c_char,
    };
    L = L;
    let mut R: C2RustUnnamed_9 = C2RustUnnamed_9 {
      v: 0 as *mut var,
      s: 0 as *const libc::c_char,
    };
    R = R;
    let mut L_d: libc::c_double = 0.;
    L_d = L_d;
    let mut opinfo: uint32_t = 0;
    let mut opn: libc::c_int = 0;
    let mut op1: *mut node = 0 as *mut node;
    opinfo = (*op).info;
    opn = (opinfo & 0x7fi32 as libc::c_uint) as libc::c_int;
    (*ptr_to_globals.offset(-1i32 as isize)).g_lineno = (*op).lineno as libc::c_int;
    op1 = (*op).l.n;
    /* "delete" is special:
     * "delete array[var--]" must evaluate index expr only once,
     * must not evaluate it in "execute inevitable things" part.
     */
    if (opinfo & 0xff00i32 as libc::c_uint) >> 8i32
      == (OC_DELETE as libc::c_int >> 8i32) as libc::c_uint
    {
      let mut info: uint32_t = (*op1).info & 0xff00i32 as libc::c_uint;
      let mut v: *mut var = 0 as *mut var;
      if info == OC_VAR as libc::c_int as libc::c_uint {
        v = (*op1).l.v
      } else if info == OC_FNARG as libc::c_int as libc::c_uint {
        v = &mut *(*(ptr_to_globals as *mut globals2))
          .evaluate__fnargs
          .offset((*op1).l.aidx as isize) as *mut var
      } else {
        syntax_error(EMSG_NOT_ARRAY.as_ptr());
      }
      if !(*op1).r.n.is_null() {
        /* array ref? */
        let mut s: *const libc::c_char = 0 as *const libc::c_char;
        s = getvar_s(evaluate((*op1).r.n, v1));
        hash_remove(iamarray(v), s);
      } else {
        clear_array(iamarray(v));
      }
    } else {
      /* execute inevitable things */
      if opinfo & 0x10000i32 as libc::c_uint != 0 {
        L.v = evaluate(op1, v1)
      } /* switch */
      if opinfo & 0x20000i32 as libc::c_uint != 0 {
        R.v = evaluate((*op).r.n, v1.offset(1))
      } /* for compiler */
      if opinfo & 0x40000i32 as libc::c_uint != 0 {
        L.s = getvar_s(L.v)
      } /* switch */
      if opinfo & 0x80000i32 as libc::c_uint != 0 {
        R.s = getvar_s(R.v)
      }
      if opinfo & 0x100000i32 as libc::c_uint != 0 {
        L_d = getvar_i(L.v)
      }
      match (opinfo & 0xff00i32 as libc::c_uint) >> 8i32 {
        13 => {
          current_block = 2121877899178614704;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    /* Can be NULL if open failed. Example:
                     * getline line <"doesnt_exist";
                     * close("doesnt_exist"); <--- here rsm->F is NULL
                     */
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                    /* not xfopen! */
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              /* The body might be empty, still has to eval the args */
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 =>
            /* -- recursive node type -- */
            {
              L.v = (*op).l.v; /* OC_PRINTF */
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 =>
            /* concatenation (" ") and index joining (",") */
            {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char; /* for compiler */
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 =>
            /* -- iterative node type -- */
            /* test pattern */
            {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                /* it's range pattern */
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 =>
            /* branch, used in if-else and various loops */
            {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 =>
            /* initialize for-in loop */
            {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 =>
            /* get next array item */
            {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 =>
            /* case XC( OC_DELETE ): - moved to happen before arg evaluation */
            {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              /* if source is a temporary string, jusk relink it to dest */
              //Disabled: if R.v is numeric but happens to have cached R.v->string,
              //then L.v ends up being a string, which is wrong
              //			if (R.v == v1+1 && R.v->string) {
              //				res = setvar_p(L.v, R.v->string);
              //				R.v->string = NULL;
              //			} else {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        2 => {}
        7 => {
          current_block = 17287781741814388593;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        6 => {
          current_block = 9714677826615582715;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        14 => {
          current_block = 2037600786294156004;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        4 | 5 => {
          current_block = 12961834331865314435;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        3 => {
          current_block = 7638386993052269879;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        35 => {
          current_block = 10991484589513526581;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        12 => {
          current_block = 16882676863205622236;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        11 => {
          current_block = 18411366856620127559;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        40 => {
          current_block = 11069739612517487711;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        10 => {
          current_block = 18312077952280535951;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        39 => {
          current_block = 1013506999122146761;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        24 => {
          current_block = 15456862084301247793;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        27 => {
          current_block = 14112124086624267904;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        33 => {
          current_block = 13483122344860452139;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        30 => {
          current_block = 6927004879276236994;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        31 => {
          current_block = 13326139174796812312;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        37 => {
          current_block = 717755337672912042;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        25 => {
          current_block = 11227437541145425351;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        26 | 32 => {
          current_block = 7848525887314104415;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        22 => {
          current_block = 8679519573445464994;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        17 => {
          current_block = 3570306954818144852;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        36 => {
          current_block = 3537788755567081353;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        38 => {
          current_block = 4001239642700071046;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        23 => {
          current_block = 17870747410601324163;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        21 | 19 => {
          current_block = 8483315232868171348;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        28 => {
          current_block = 8739897491286011536;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        29 => {
          current_block = 10704547640368762168;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        16 | 34 => {
          current_block = 6344031133004074802;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        20 => {
          current_block = 4347182642961927639;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
        _ => {
          current_block = 16214842883115607608;
          match current_block {
            17870747410601324163 => {
              let mut i_1: libc::c_int = getvar_i(R.v) as libc::c_int;
              if i_1 < 0i32 {
                syntax_error(EMSG_NEGATIVE_FIELD.as_ptr());
              }
              if i_1 == 0i32 {
                res = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
              } else {
                split_f0();
                if i_1 > (*ptr_to_globals.offset(-1i32 as isize)).nfields {
                  fsrealloc(i_1);
                }
                res = &mut *(*ptr_to_globals.offset(-1i32 as isize))
                  .Fields
                  .offset((i_1 - 1i32) as isize) as *mut var
              }
              current_block = 261410684697878013;
            }
            4001239642700071046 => {
              let mut Ld: libc::c_double = 0.;
              let mut R_d_0: libc::c_double = 0.;
              R_d_0 = getvar_i(R.v);
              Ld = R_d_0;
              match opn {
                80 => {
                  R_d_0 += 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                112 => {
                  R_d_0 += 1.;
                  current_block = 12160612325011165107;
                }
                77 => {
                  R_d_0 -= 1.;
                  Ld = R_d_0;
                  current_block = 12160612325011165107;
                }
                109 => {
                  R_d_0 -= 1.;
                  current_block = 12160612325011165107;
                }
                33 => {
                  Ld = (istrue(R.v) == 0) as libc::c_int as libc::c_double;
                  current_block = 12941848718694358059;
                }
                45 => {
                  Ld = -R_d_0;
                  current_block = 12941848718694358059;
                }
                _ => {
                  current_block = 12941848718694358059;
                }
              }
              match current_block {
                12160612325011165107 => {
                  setvar_i(R.v, R_d_0);
                }
                _ => {}
              }
              setvar_i(res, Ld);
              current_block = 261410684697878013;
            }
            8679519573445464994 => {
              let mut R_d: libc::c_double = 0.;
              R_d = R_d;
              let mut current_block_210: u64;
              match opn {
                0 => R_d = L_d as libc::c_longlong as libc::c_double,
                1 => R_d = rand() as libc::c_double / 2147483647i32 as libc::c_double,
                2 => R_d = cos(L_d),
                3 => R_d = exp(L_d),
                4 => R_d = log(L_d),
                5 => R_d = sin(L_d),
                6 => R_d = sqrt(L_d),
                7 => {
                  R_d = (*(ptr_to_globals as *mut globals2)).evaluate__seed as libc::c_double;
                  (*(ptr_to_globals as *mut globals2)).evaluate__seed = if !op1.is_null() {
                    L_d as libc::c_uint
                  } else {
                    time(0 as *mut time_t) as libc::c_uint
                  };
                  srand((*(ptr_to_globals as *mut globals2)).evaluate__seed);
                }
                8 => R_d = time(0 as *mut time_t) as libc::c_double,
                9 => {
                  if op1.is_null() {
                    L.s = getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    );
                    current_block_210 = 12433871959304513155;
                  } else if (*L.v).type_0 & 0x2i32 as libc::c_uint != 0 {
                    R_d = (*(*L.v).x.array).nel as libc::c_double;
                    current_block_210 = 2603304679309198903;
                  } else {
                    current_block_210 = 12433871959304513155;
                  }
                  match current_block_210 {
                    2603304679309198903 => {}
                    _ => R_d = strlen(L.s) as libc::c_double,
                  }
                }
                10 => {
                  fflush_all();
                  R_d = if 1i32 != 0 && !L.s.is_null() && *L.s as libc::c_int != 0 {
                    (system(L.s)) >> 8i32
                  } else {
                    0i32
                  } as libc::c_double
                }
                11 => {
                  if op1.is_null() {
                    fflush(stdout);
                  } else if !L.s.is_null() && *L.s as libc::c_int != 0 {
                    let mut rsm_1: *mut rstream =
                      hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                        as *mut rstream;
                    fflush((*rsm_1).F);
                  } else {
                    fflush_all();
                  }
                }
                12 => {
                  let mut rsm_2: *mut rstream = 0 as *mut rstream;
                  let mut err: libc::c_int = 0i32;
                  rsm_2 = hash_search((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s)
                    as *mut rstream;
                  if !rsm_2.is_null() {
                    if !(*rsm_2).F.is_null() {
                      err = if (*rsm_2).is_pipe as libc::c_int != 0 {
                        pclose((*rsm_2).F)
                      } else {
                        fclose((*rsm_2).F)
                      }
                    }
                    free((*rsm_2).buffer as *mut libc::c_void);
                    hash_remove((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s);
                  }
                  if err != 0 {
                    setvar_i(
                      (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                      *bb_errno as libc::c_double,
                    );
                  }
                  R_d = err as libc::c_double
                }
                _ => {}
              }
              setvar_i(res, R_d);
              current_block = 261410684697878013;
            }
            7848525887314104415 => {
              let mut rsm_0: *mut rstream = 0 as *mut rstream;
              let mut i_0: libc::c_int = 0;
              if !op1.is_null() {
                rsm_0 =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, L.s) as *mut rstream;
                if (*rsm_0).F.is_null() {
                  if opinfo & 0xff00i32 as libc::c_uint
                    == OC_PGETLINE as libc::c_int as libc::c_uint
                  {
                    (*rsm_0).F = popen(L.s, b"r\x00" as *const u8 as *const libc::c_char);
                    (*rsm_0).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm_0).F = fopen_for_read(L.s)
                  }
                }
              } else {
                if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
                  let ref mut fresh62 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
                  *fresh62 = next_input_file()
                }
                rsm_0 = (*ptr_to_globals.offset(-1i32 as isize)).iF
              }
              if rsm_0.is_null() || (*rsm_0).F.is_null() {
                setvar_i(
                  (*(ptr_to_globals as *mut globals2)).intvar[ERRNO as libc::c_int as usize],
                  *bb_errno as libc::c_double,
                );
                setvar_i(res, -1i32 as libc::c_double);
              } else {
                if (*op).r.n.is_null() {
                  R.v = (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]
                }
                i_0 = awk_getline(rsm_0, R.v);
                if i_0 > 0i32 && op1.is_null() {
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
                  incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
                }
                setvar_i(res, i_0 as libc::c_double);
              }
              current_block = 261410684697878013;
            }
            11227437541145425351 => {
              let mut vbeg: *mut var = 0 as *mut var;
              let mut v_1: *mut var = 0 as *mut var;
              let mut sv_progname: *const libc::c_char = 0 as *const libc::c_char;
              if (*(*op).r.n).info == 0 && (*(*op).r.f).body.first.is_null() {
                syntax_error(EMSG_UNDEF_FUNC.as_ptr());
              }
              v_1 = nvalloc((*(*op).r.f).nargs.wrapping_add(1i32 as libc::c_uint) as libc::c_int);
              vbeg = v_1;
              while !op1.is_null() {
                let mut arg: *mut var = evaluate(nextarg(&mut op1), v1);
                copyvar(v_1, arg);
                (*v_1).type_0 |= 0x2000i32 as libc::c_uint;
                (*v_1).x.parent = arg;
                v_1 = v_1.offset(1);
                if v_1.wrapping_offset_from(vbeg) as libc::c_long
                  >= (*(*op).r.f).nargs as libc::c_long
                {
                  break;
                }
              }
              v_1 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              let ref mut fresh59 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh59 = vbeg;
              sv_progname = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              res = evaluate((*(*op).r.f).body.first, res);
              let ref mut fresh60 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh60 = sv_progname;
              nvfree((*(ptr_to_globals as *mut globals2)).evaluate__fnargs);
              let ref mut fresh61 = (*(ptr_to_globals as *mut globals2)).evaluate__fnargs;
              *fresh61 = v_1;
              current_block = 261410684697878013;
            }
            717755337672912042 => {
              if (*(*op).r.n).info & 0xff00i32 as libc::c_uint
                != OC_COLON as libc::c_int as libc::c_uint
              {
                syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
              }
              res = evaluate(
                if istrue(L.v) != 0 {
                  (*(*op).r.n).l.n
                } else {
                  (*(*op).r.n).r.n
                },
                res,
              );
              current_block = 261410684697878013;
            }
            13483122344860452139 => {
              op1 = op;
              L.s =
                getvar_s((*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize]);
              current_block = 9650150017893459424;
            }
            1013506999122146761 => {
              L.v = (*op).l.v;
              if L.v == (*(ptr_to_globals as *mut globals2)).intvar[NF as libc::c_int as usize] {
                split_f0();
              }
              current_block = 11966806193788542454;
            }
            12961834331865314435 => {
              let mut F: *mut FILE = stdout;
              if !(*op).r.n.is_null() {
                let mut rsm: *mut rstream =
                  hash_find((*ptr_to_globals.offset(-1i32 as isize)).fdhash, R.s) as *mut rstream;
                if (*rsm).F.is_null() {
                  if opn == '|' as i32 {
                    (*rsm).F = popen(R.s, b"w\x00" as *const u8 as *const libc::c_char);
                    if (*rsm).F.is_null() {
                      bb_simple_perror_msg_and_die(
                        b"popen\x00" as *const u8 as *const libc::c_char,
                      );
                    }
                    (*rsm).is_pipe = 1i32 as smallint
                  } else {
                    (*rsm).F = xfopen(
                      R.s,
                      if opn == 'w' as i32 {
                        b"w\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"a\x00" as *const u8 as *const libc::c_char
                      },
                    )
                  }
                }
                F = (*rsm).F
              }
              if opinfo & 0xff00i32 as libc::c_uint == OC_PRINT as libc::c_int as libc::c_uint {
                if op1.is_null() {
                  fputs_unlocked(
                    getvar_s(
                      (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
                    ),
                    F,
                  );
                } else {
                  while !op1.is_null() {
                    let mut v_0: *mut var = evaluate(nextarg(&mut op1), v1);
                    if (*v_0).type_0 & 0x1i32 as libc::c_uint != 0 {
                      fmt_num(
                        (*ptr_to_globals.offset(-1i32 as isize)).g_buf,
                        240i32,
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFMT as libc::c_int as usize],
                        ),
                        getvar_i(v_0),
                        1i32,
                      );
                      fputs_unlocked((*ptr_to_globals.offset(-1i32 as isize)).g_buf, F);
                    } else {
                      fputs_unlocked(getvar_s(v_0), F);
                    }
                    if !op1.is_null() {
                      fputs_unlocked(
                        getvar_s(
                          (*(ptr_to_globals as *mut globals2)).intvar[OFS as libc::c_int as usize],
                        ),
                        F,
                      );
                    }
                  }
                }
                fputs_unlocked(
                  getvar_s(
                    (*(ptr_to_globals as *mut globals2)).intvar[ORS as libc::c_int as usize],
                  ),
                  F,
                );
              } else {
                let mut s_0: *mut libc::c_char = awk_printf(op1);
                fputs_unlocked(s_0, F);
                free(s_0 as *mut libc::c_void);
              }
              fflush(F);
              current_block = 261410684697878013;
            }
            8483315232868171348 => {
              let mut sep: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
              if opinfo & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint {
                sep = getvar_s(
                  (*(ptr_to_globals as *mut globals2)).intvar[SUBSEP as libc::c_int as usize],
                )
              }
              setvar_p(
                res,
                xasprintf(
                  b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                  L.s,
                  sep,
                  R.s,
                ),
              );
              current_block = 261410684697878013;
            }
            6344031133004074802 => {
              let mut R_d_1: libc::c_double = getvar_i(R.v);
              match opn {
                43 => L_d += R_d_1,
                45 => L_d -= R_d_1,
                42 => L_d *= R_d_1,
                47 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d /= R_d_1
                }
                38 => L_d = pow(L_d, R_d_1),
                37 => {
                  if R_d_1 == 0i32 as libc::c_double {
                    syntax_error(EMSG_DIV_BY_ZERO.as_ptr());
                  }
                  L_d -= (L_d / R_d_1) as libc::c_longlong as libc::c_double * R_d_1
                }
                _ => {}
              }
              res = setvar_i(
                if opinfo & 0xff00i32 as libc::c_uint == OC_BINARY as libc::c_int as libc::c_uint {
                  res
                } else {
                  L.v
                },
                L_d,
              );
              current_block = 261410684697878013;
            }
            4347182642961927639 => {
              let mut i_2: libc::c_int = 0;
              i_2 = i_2;
              let mut Ld_0: libc::c_double = 0.;
              if is_numeric(L.v) != 0 && is_numeric(R.v) != 0 {
                Ld_0 = getvar_i(L.v) - getvar_i(R.v)
              } else {
                let mut l: *const libc::c_char = getvar_s(L.v);
                let mut r: *const libc::c_char = getvar_s(R.v);
                Ld_0 = if (*ptr_to_globals.offset(-1i32 as isize)).icase as libc::c_int != 0 {
                  strcasecmp(l, r)
                } else {
                  strcmp(l, r)
                } as libc::c_double
              }
              match opn & 0xfei32 {
                0 => i_2 = (Ld_0 > 0i32 as libc::c_double) as libc::c_int,
                2 => i_2 = (Ld_0 >= 0i32 as libc::c_double) as libc::c_int,
                4 => i_2 = (Ld_0 == 0i32 as libc::c_double) as libc::c_int,
                _ => {}
              }
              setvar_i(
                res,
                ((i_2 == 0i32) as libc::c_int ^ opn & 1i32) as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            2121877899178614704 => {
              if (*op1).info & 0xff00i32 as libc::c_uint == OC_COMMA as libc::c_int as libc::c_uint
              {
                if opinfo & 0x200000i32 as libc::c_uint != 0 || ptest((*op1).l.n) != 0 {
                  (*op).info |= 0x200000i32 as libc::c_uint;
                  if ptest((*op1).r.n) != 0 {
                    (*op).info &= !0x200000i32 as libc::c_uint
                  }
                  op = (*op).a.n
                } else {
                  op = (*op).r.n
                }
              } else {
                op = if ptest(op1) != 0 {
                  (*op).a.n
                } else {
                  (*op).r.n
                }
              }
              current_block = 261410684697878013;
            }
            17287781741814388593 => {
              op = if istrue(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            9714677826615582715 => {
              hashwalk_init(L.v, iamarray(R.v));
              current_block = 261410684697878013;
            }
            2037600786294156004 => {
              op = if hashwalk_next(L.v) != 0 {
                (*op).a.n
              } else {
                (*op).r.n
              };
              current_block = 261410684697878013;
            }
            7638386993052269879 => {
              let ref mut fresh58 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
              *fresh58 = (*op).l.new_progname;
              current_block = 261410684697878013;
            }
            10991484589513526581 => {
              copyvar(res, L.v);
              current_block = 261410684697878013;
            }
            16882676863205622236 => {
              (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 1i32 as smallint;
              current_block = 18411366856620127559;
            }
            18312077952280535951 => {
              awk_exit(L_d as libc::c_int);
            }
            15456862084301247793 => {
              L.v = &mut *(*(ptr_to_globals as *mut globals2))
                .evaluate__fnargs
                .offset((*op).l.aidx as isize) as *mut var;
              current_block = 11966806193788542454;
            }
            14112124086624267904 => {
              setvar_i(
                res,
                if !hash_search(iamarray(R.v), L.s).is_null() {
                  1i32
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            6927004879276236994 => {
              op1 = (*op).r.n;
              current_block = 9650150017893459424;
            }
            13326139174796812312 => {
              res = copyvar(L.v, R.v);
              current_block = 261410684697878013;
            }
            3570306954818144852 => {
              res = exec_builtin(op, res);
              current_block = 261410684697878013;
            }
            3537788755567081353 => {
              setvar_p(res, awk_printf(op1));
              current_block = 261410684697878013;
            }
            8739897491286011536 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  ptest((*op).r.n)
                } else {
                  0i32
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            10704547640368762168 => {
              setvar_i(
                res,
                if istrue(L.v) != 0 {
                  1i32
                } else {
                  ptest((*op).r.n)
                } as libc::c_double,
              );
              current_block = 261410684697878013;
            }
            16214842883115607608 => {
              syntax_error(EMSG_POSSIBLE_ERROR.as_ptr());
            }
            _ => {}
          }
          match current_block {
            261410684697878013 => {}
            _ => {
              match current_block {
                18411366856620127559 => {
                  (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 1i32 as smallint;
                  current_block = 11069739612517487711;
                }
                9650150017893459424 => {
                  let mut re: *mut regex_t = as_regex(
                    op1,
                    &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg,
                  );
                  let mut i: libc::c_int =
                    regexec(re, L.s, 0i32 as size_t, 0 as *mut regmatch_t, 0i32);
                  if re == &mut (*(ptr_to_globals as *mut globals2)).evaluate__sreg as *mut regex_t
                  {
                    regfree(re);
                  }
                  setvar_i(
                    res,
                    ((i == 0i32) as libc::c_int ^ (opn == '!' as i32) as libc::c_int)
                      as libc::c_double,
                  );
                  current_block = 261410684697878013;
                }
                11966806193788542454 => {
                  res = if !(*op).r.n.is_null() {
                    hash_find(iamarray(L.v), R.s) as *mut var
                  } else {
                    L.v
                  };
                  current_block = 261410684697878013;
                }
                _ => {}
              }
              match current_block {
                261410684697878013 => {}
                _ => {
                  clrvar(res);
                }
              }
            }
          }
        }
      }
    }
    //			}
    if opinfo & 0xff00i32 as libc::c_uint <= 0x600i32 as libc::c_uint {
      op = (*op).a.n
    }
    if opinfo & 0xff00i32 as libc::c_uint >= 0x1000i32 as libc::c_uint {
      break;
    }
    if (*ptr_to_globals.offset(-1i32 as isize)).nextrec != 0 {
      break;
    }
  }
  nvfree(v1);
  return res;
}
/* -------- main & co. -------- */
unsafe extern "C" fn awk_exit(mut r: libc::c_int) -> ! {
  let mut tv: var = var {
    type_0: 0,
    number: 0.,
    string: 0 as *mut libc::c_char,
    x: C2RustUnnamed { aidx: 0 },
  };
  let mut i: libc::c_uint = 0;
  let mut hi: *mut hash_item = 0 as *mut hash_item;
  zero_out_var(&mut tv);
  if (*ptr_to_globals.offset(-1i32 as isize)).exiting == 0 {
    (*ptr_to_globals.offset(-1i32 as isize)).exiting = 1i32 as smallint;
    (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 0i32 as smallint;
    evaluate(
      (*ptr_to_globals.offset(-1i32 as isize)).endseq.first,
      &mut tv,
    );
  }
  /* waiting for children */
  i = 0i32 as libc::c_uint;
  while i < (*(*ptr_to_globals.offset(-1i32 as isize)).fdhash).csize {
    hi = *(*(*ptr_to_globals.offset(-1i32 as isize)).fdhash)
      .items
      .offset(i as isize);
    while !hi.is_null() {
      if !(*hi).data.rs.F.is_null() && (*hi).data.rs.is_pipe as libc::c_int != 0 {
        pclose((*hi).data.rs.F);
      }
      hi = (*hi).next
    }
    i = i.wrapping_add(1)
  }
  exit(r);
}
/* if expr looks like "var=value", perform assignment and return 1,
 * otherwise return 0 */
unsafe extern "C" fn is_assignment(mut expr: *const libc::c_char) -> libc::c_int {
  let mut exprc: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
  if isalnum_(*expr as libc::c_int) == 0 || {
    val = strchr(expr, '=' as i32);
    val.is_null()
  } {
    return 0i32;
  }
  exprc = xstrdup(expr);
  val = exprc.offset(val.wrapping_offset_from(expr) as libc::c_long as isize);
  let fresh63 = val;
  val = val.offset(1);
  *fresh63 = '\u{0}' as i32 as libc::c_char;
  unescape_string_in_place(val);
  setvar_u(
    hash_find((*ptr_to_globals.offset(-1i32 as isize)).vhash, exprc) as *mut var,
    val,
  );
  free(exprc as *mut libc::c_void);
  return 1i32;
}
/* switch to next input file */
unsafe extern "C" fn next_input_file() -> *mut rstream {
  let mut F: *mut FILE = 0 as *mut FILE; /* cheat */
  let mut fname: *const libc::c_char = 0 as *const libc::c_char;
  let mut ind: *const libc::c_char = 0 as *const libc::c_char;
  if !(*(ptr_to_globals as *mut globals2))
    .next_input_file__rsm
    .F
    .is_null()
  {
    fclose((*(ptr_to_globals as *mut globals2)).next_input_file__rsm.F);
  }
  let ref mut fresh64 = (*(ptr_to_globals as *mut globals2)).next_input_file__rsm.F;
  *fresh64 = 0 as *mut FILE;
  let ref mut fresh65 = (*(ptr_to_globals as *mut globals2))
    .next_input_file__rsm
    .adv;
  *fresh65 = 0i32;
  (*(ptr_to_globals as *mut globals2))
    .next_input_file__rsm
    .pos = *fresh65;
  loop {
    if getvar_i((*(ptr_to_globals as *mut globals2)).intvar[ARGIND as libc::c_int as usize])
      + 1i32 as libc::c_double
      >= getvar_i((*(ptr_to_globals as *mut globals2)).intvar[ARGC as libc::c_int as usize])
    {
      if (*(ptr_to_globals as *mut globals2)).next_input_file__files_happen != 0 {
        return 0 as *mut rstream;
      }
      fname = b"-\x00" as *const u8 as *const libc::c_char;
      F = stdin;
      break;
    } else {
      ind = getvar_s(incvar(
        (*(ptr_to_globals as *mut globals2)).intvar[ARGIND as libc::c_int as usize],
      ));
      fname = getvar_s(hash_find(
        iamarray((*(ptr_to_globals as *mut globals2)).intvar[ARGV as libc::c_int as usize]),
        ind,
      ) as *mut var);
      if !(!fname.is_null() && *fname as libc::c_int != 0 && is_assignment(fname) == 0) {
        continue;
      }
      F = xfopen_stdin(fname);
      break;
    }
  }
  (*(ptr_to_globals as *mut globals2)).next_input_file__files_happen = 1i32 as smallint;
  setvar_s(
    (*(ptr_to_globals as *mut globals2)).intvar[FILENAME as libc::c_int as usize],
    fname,
  );
  let ref mut fresh66 = (*(ptr_to_globals as *mut globals2)).next_input_file__rsm.F;
  *fresh66 = F;
  return &mut (*(ptr_to_globals as *mut globals2)).next_input_file__rsm;
}
#[no_mangle]
pub unsafe extern "C" fn awk_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opt: libc::c_uint = 0;
  let mut opt_F: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut list_v: *mut llist_t = 0 as *mut llist_t;
  let mut list_f: *mut llist_t = 0 as *mut llist_t;
  let mut list_e: *mut llist_t = 0 as *mut llist_t;
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  let mut v: *mut var = 0 as *mut var;
  let mut tv: var = var {
    type_0: 0,
    number: 0.,
    string: 0 as *mut libc::c_char,
    x: C2RustUnnamed { aidx: 0 },
  };
  let mut envp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut vnames: *mut libc::c_char = vNames.as_ptr() as *mut libc::c_char;
  let mut vvalues: *mut libc::c_char = vValues.as_ptr() as *mut libc::c_char;
  let ref mut fresh67 =
    *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
      as *mut *mut globals);
  *fresh67 = (xzalloc(
    (::std::mem::size_of::<globals>() as libc::c_ulong)
      .wrapping_add(::std::mem::size_of::<globals2>() as libc::c_ulong),
  ) as *mut libc::c_char)
    .offset(::std::mem::size_of::<globals>() as libc::c_ulong as isize)
    as *mut libc::c_void as *mut globals;
  asm!("" : : : "memory" : "volatile");
  (*(ptr_to_globals as *mut globals2)).next_token__ltclass =
    (1i32 << 14i32 | 1i32 << 15i32) as uint32_t;
  (*(ptr_to_globals as *mut globals2)).evaluate__seed = 1i32 as libc::c_uint;
  /* Undo busybox.c, or else strtod may eat ','! This breaks parsing:
   * $1,$2 == '$1,' '$2', NOT '$1' ',' '$2' */
  zero_out_var(&mut tv);
  /* allocate global buffer */
  let ref mut fresh68 = (*ptr_to_globals.offset(-1i32 as isize)).g_buf;
  *fresh68 = xmalloc((240i32 + 1i32) as size_t) as *mut libc::c_char;
  let ref mut fresh69 = (*ptr_to_globals.offset(-1i32 as isize)).vhash;
  *fresh69 = hash_init();
  let ref mut fresh70 = (*ptr_to_globals.offset(-1i32 as isize)).ahash;
  *fresh70 = hash_init();
  let ref mut fresh71 = (*ptr_to_globals.offset(-1i32 as isize)).fdhash;
  *fresh71 = hash_init();
  let ref mut fresh72 = (*ptr_to_globals.offset(-1i32 as isize)).fnhash;
  *fresh72 = hash_init();
  /* initialize variables */
  i = 0i32;
  while *vnames != 0 {
    v = hash_find(
      (*ptr_to_globals.offset(-1i32 as isize)).vhash,
      nextword(&mut vnames),
    ) as *mut var;
    let ref mut fresh73 = (*(ptr_to_globals as *mut globals2)).intvar[i as usize];
    *fresh73 = v;
    if *vvalues as libc::c_int != '\u{ff}' as i32 {
      setvar_s(v, nextword(&mut vvalues));
    } else {
      setvar_i(v, 0i32 as libc::c_double);
    }
    if *vnames as libc::c_int == '*' as i32 {
      (*v).type_0 |= 0x400i32 as libc::c_uint;
      vnames = vnames.offset(1)
    }
    i += 1
  }
  handle_special((*(ptr_to_globals as *mut globals2)).intvar[FS as libc::c_int as usize]);
  handle_special((*(ptr_to_globals as *mut globals2)).intvar[RS as libc::c_int as usize]);
  let ref mut fresh74 = (*(hash_find(
    (*ptr_to_globals.offset(-1i32 as isize)).fdhash,
    b"/dev/stdin\x00" as *const u8 as *const libc::c_char,
  ) as *mut rstream))
    .F;
  *fresh74 = stdin;
  let ref mut fresh75 = (*(hash_find(
    (*ptr_to_globals.offset(-1i32 as isize)).fdhash,
    b"/dev/stdout\x00" as *const u8 as *const libc::c_char,
  ) as *mut rstream))
    .F;
  *fresh75 = stdout;
  let ref mut fresh76 = (*(hash_find(
    (*ptr_to_globals.offset(-1i32 as isize)).fdhash,
    b"/dev/stderr\x00" as *const u8 as *const libc::c_char,
  ) as *mut rstream))
    .F;
  *fresh76 = stderr;
  /* Huh, people report that sometimes environ is NULL. Oh well. */
  if !environ.is_null() {
    envp = environ;
    while !(*envp).is_null() {
      /* environ is writable, thus we don't strdup it needlessly */
      let mut s: *mut libc::c_char = *envp;
      let mut s1: *mut libc::c_char = strchr(s, '=' as i32);
      if !s1.is_null() {
        *s1 = '\u{0}' as i32 as libc::c_char;
        /* Both findvar and setvar_u take const char*
         * as 2nd arg -> environment is not trashed */
        setvar_u(
          hash_find(
            iamarray((*(ptr_to_globals as *mut globals2)).intvar[ENVIRON as libc::c_int as usize]),
            s,
          ) as *mut var,
          s1.offset(1),
        );
        *s1 = '=' as i32 as libc::c_char
      }
      envp = envp.offset(1)
    }
  }
  opt = getopt32(
    argv,
    b"+F:v:*f:*e:*W:\x00" as *const u8 as *const libc::c_char,
    &mut opt_F as *mut *mut libc::c_char,
    &mut list_v as *mut *mut llist_t,
    &mut list_f as *mut *mut llist_t,
    &mut list_e as *mut *mut llist_t,
    0 as *mut libc::c_void,
  );
  argv = argv.offset(optind as isize);
  //argc -= optind;
  if opt & OPT_W as libc::c_int as libc::c_uint != 0 {
    bb_simple_error_msg(b"warning: option -W is ignored\x00" as *const u8 as *const libc::c_char);
  }
  if opt & OPT_F as libc::c_int as libc::c_uint != 0 {
    unescape_string_in_place(opt_F);
    setvar_s(
      (*(ptr_to_globals as *mut globals2)).intvar[FS as libc::c_int as usize],
      opt_F,
    );
  }
  while !list_v.is_null() {
    if is_assignment(llist_pop(&mut list_v) as *const libc::c_char) == 0 {
      bb_show_usage();
    }
  }
  while !list_f.is_null() {
    let mut s_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut from_file: *mut FILE = 0 as *mut FILE;
    let ref mut fresh77 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
    *fresh77 = llist_pop(&mut list_f) as *const libc::c_char;
    from_file = xfopen_stdin((*ptr_to_globals.offset(-1i32 as isize)).g_progname);
    /* one byte is reserved for some trick in next_token */
    j = 1i32;
    i = j;
    while j > 0i32 {
      s_0 = xrealloc(s_0 as *mut libc::c_void, (i + 4096i32) as size_t) as *mut libc::c_char;
      j = fread(
        s_0.offset(i as isize) as *mut libc::c_void,
        1i32 as size_t,
        4094i32 as size_t,
        from_file,
      ) as libc::c_int;
      i += j
    }
    *s_0.offset(i as isize) = '\u{0}' as i32 as libc::c_char;
    fclose(from_file);
    parse_program(s_0.offset(1));
    free(s_0 as *mut libc::c_void);
  }
  let ref mut fresh78 = (*ptr_to_globals.offset(-1i32 as isize)).g_progname;
  *fresh78 = b"cmd. line\x00" as *const u8 as *const libc::c_char;
  while !list_e.is_null() {
    parse_program(llist_pop(&mut list_e) as *mut libc::c_char);
  }
  if opt & (OPT_f as libc::c_int | OPT_e as libc::c_int) as libc::c_uint == 0 {
    if (*argv).is_null() {
      bb_show_usage();
    }
    let fresh79 = argv;
    argv = argv.offset(1);
    parse_program(*fresh79);
  }
  /* fill in ARGV array */
  setari_u(
    (*(ptr_to_globals as *mut globals2)).intvar[ARGV as libc::c_int as usize],
    0i32,
    b"awk\x00" as *const u8 as *const libc::c_char,
  );
  i = 0i32;
  while !(*argv).is_null() {
    i += 1;
    let fresh80 = argv;
    argv = argv.offset(1);
    setari_u(
      (*(ptr_to_globals as *mut globals2)).intvar[ARGV as libc::c_int as usize],
      i,
      *fresh80,
    );
  }
  setvar_i(
    (*(ptr_to_globals as *mut globals2)).intvar[ARGC as libc::c_int as usize],
    (i + 1i32) as libc::c_double,
  );
  evaluate(
    (*ptr_to_globals.offset(-1i32 as isize)).beginseq.first,
    &mut tv,
  );
  if (*ptr_to_globals.offset(-1i32 as isize))
    .mainseq
    .first
    .is_null()
    && (*ptr_to_globals.offset(-1i32 as isize))
      .endseq
      .first
      .is_null()
  {
    awk_exit(0i32);
  }
  /* input file could already be opened in BEGIN block */
  if (*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
    let ref mut fresh81 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
    *fresh81 = next_input_file()
  }
  /* passing through input files */
  while !(*ptr_to_globals.offset(-1i32 as isize)).iF.is_null() {
    (*ptr_to_globals.offset(-1i32 as isize)).nextfile = 0i32 as smallint;
    setvar_i(
      (*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize],
      0i32 as libc::c_double,
    );
    loop {
      i = awk_getline(
        (*ptr_to_globals.offset(-1i32 as isize)).iF,
        (*(ptr_to_globals as *mut globals2)).intvar[F0 as libc::c_int as usize],
      );
      if !(i > 0i32) {
        break;
      }
      (*ptr_to_globals.offset(-1i32 as isize)).nextrec = 0i32 as smallint;
      incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as libc::c_int as usize]);
      incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as libc::c_int as usize]);
      evaluate(
        (*ptr_to_globals.offset(-1i32 as isize)).mainseq.first,
        &mut tv,
      );
      if (*ptr_to_globals.offset(-1i32 as isize)).nextfile != 0 {
        break;
      }
    }
    if i < 0i32 {
      syntax_error(strerror(*bb_errno));
    }
    let ref mut fresh82 = (*ptr_to_globals.offset(-1i32 as isize)).iF;
    *fresh82 = next_input_file()
  }
  awk_exit(0i32);
  /*return 0;*/
}
