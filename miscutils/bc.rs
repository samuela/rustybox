use libc;
extern "C" {
  #[no_mangle]
  fn isatty(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  static mut optarg: *mut libc::c_char;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn getopt(
    ___argc: libc::c_int,
    ___argv: *const *mut libc::c_char,
    __shortopts: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  static mut stdin: *mut FILE;
  #[no_mangle]
  static mut stdout: *mut FILE;
  #[no_mangle]
  static mut stderr: *mut FILE;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn ferror_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn exit(_: libc::c_int) -> !;
  #[no_mangle]
  fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn llabs(_: libc::c_longlong) -> libc::c_longlong;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  static ptr_to_globals: *mut globals;
  #[no_mangle]
  static const_int_0: libc::c_int;
  #[no_mangle]
  static bb_hexdigits_upcase: [libc::c_char; 0];
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
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
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn signal_SA_RESTART_empty_mask(
    sig: libc::c_int,
    handler: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
  );
  #[no_mangle]
  static mut bb_got_signal: smallint;
  #[no_mangle]
  fn record_signo(signo: libc::c_int);
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn xfopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  #[no_mangle]
  static mut option_mask32: uint32_t;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> uint32_t;
  #[no_mangle]
  fn xfunc_die() -> !;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_perror_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_verror_msg(s: *const libc::c_char, p: ::std::ffi::VaList, strerr: *const libc::c_char);
  #[no_mangle]
  fn new_line_input_t(flags: libc::c_int) -> *mut line_input_t;
  #[no_mangle]
  fn read_line_input(
    st: *mut line_input_t,
    prompt: *const libc::c_char,
    command: *mut libc::c_char,
    maxsize: libc::c_int,
  ) -> libc::c_int;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
  pub gp_offset: libc::c_uint,
  pub fp_offset: libc::c_uint,
  pub overflow_arg_area: *mut libc::c_void,
  pub reg_save_area: *mut libc::c_void,
}
pub type __int8_t = libc::c_schar;


pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;

pub type int8_t = __int8_t;
use crate::librb::uint8_t;
use crate::librb::uint32_t;
/* NB: unaligned parameter should be a pointer, aligned one -
 * a lvalue. This makes it more likely to not swap them by mistake
 */
/* #elif ... - add your favorite arch today! */
/* Unaligned, fixed-endian accessors */
/* unxz needs an aligned fixed-endian accessor.
 * (however, the compiler does not realize it's aligned, the cast is still necessary)
 */
/* ---- Size-saving "small" ints (arch-dependent) ----------- */
/* add other arches which benefit from this... */
use crate::librb::smallint;
use crate::librb::ssize_t;
use crate::librb::size_t;



use crate::librb::FILE;
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
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
pub type C2RustUnnamed = libc::c_uint;
pub const FOR_SHELL: C2RustUnnamed = 7;
pub const WITH_PATH_LOOKUP: C2RustUnnamed = 16;
pub const VI_MODE: C2RustUnnamed = 0;
pub const USERNAME_COMPLETION: C2RustUnnamed = 4;
pub const TAB_COMPLETION: C2RustUnnamed = 2;
pub const DO_HISTORY: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub prs: BcParse,
  pub err_line: size_t,
  pub input_buffer: BcVec,
  pub ttyin: smallint,
  pub prog: BcProgram,
  pub files: BcVec,
  pub env_args: *mut libc::c_char,
  pub line_input_state: *mut line_input_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BcVec {
  pub v: *mut libc::c_char,
  pub len: size_t,
  pub cap: size_t,
  pub size: size_t,
  pub dtor: BcVecFree,
}
pub type BcVecFree = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BcProgram {
  pub len: size_t,
  pub nchars: size_t,
  pub scale: size_t,
  pub ib_t: size_t,
  pub ob_t: size_t,
  pub results: BcVec,
  pub exestack: BcVec,
  pub fns: BcVec,
  pub fn_map: BcVec,
  pub vars: BcVec,
  pub var_map: BcVec,
  pub arrs: BcVec,
  pub arr_map: BcVec,
  pub strs: BcVec,
  pub consts: BcVec,
  pub zero: BcNum,
  pub one: BcNum,
  pub last: BcNum,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BcNum {
  pub num: *mut BcDig,
  pub rdx: size_t,
  pub len: size_t,
  pub cap: size_t,
  pub neg: bool,
}
pub type BcDig = libc::c_schar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BcParse {
  pub lex: smallint,
  pub lex_last: smallint,
  pub lex_line: size_t,
  pub lex_inbuf: *const libc::c_char,
  pub lex_next_at: *const libc::c_char,
  pub lex_filename: *const libc::c_char,
  pub lex_input_fp: *mut FILE,
  pub lex_strnumbuf: BcVec,
  pub func: *mut BcFunc,
  pub fidx: size_t,
  pub in_funcdef: size_t,
  pub exits: BcVec,
  pub conds: BcVec,
  pub ops: BcVec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BcFunc {
  pub code: BcVec,
  pub labels: BcVec,
  pub autos: BcVec,
  pub strs: BcVec,
  pub consts: BcVec,
  pub nparams: size_t,
  pub voidfunc: bool,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const COMMON_BUFSIZE: C2RustUnnamed_0 = 1024;
pub type BcStatus = libc::c_uint;
pub const BC_STATUS_FAILURE: BcStatus = 1;
pub const BC_STATUS_SUCCESS: BcStatus = 0;
pub type BcInst = libc::c_int;
pub const DC_INST_INVALID: BcInst = -1;
pub const DC_INST_NQUIT: BcInst = 66;
pub const DC_INST_QUIT: BcInst = 65;
pub const DC_INST_PUSH_TO_VAR: BcInst = 64;
pub const DC_INST_PUSH_VAR: BcInst = 63;
pub const DC_INST_LOAD: BcInst = 62;
pub const DC_INST_SWAP: BcInst = 61;
pub const DC_INST_DUPLICATE: BcInst = 60;
pub const DC_INST_STACK_LEN: BcInst = 59;
pub const DC_INST_CLEAR_STACK: BcInst = 58;
pub const DC_INST_PRINT_STACK: BcInst = 57;
pub const DC_INST_PRINT_STREAM: BcInst = 56;
pub const DC_INST_ASCIIFY: BcInst = 55;
pub const DC_INST_EXEC_COND: BcInst = 54;
pub const DC_INST_EXECUTE: BcInst = 53;
pub const DC_INST_DIVMOD: BcInst = 52;
pub const DC_INST_MODEXP: BcInst = 51;
pub const DC_INST_POP_EXEC: BcInst = 50;
pub const XC_INST_POP: BcInst = 49;
pub const XC_INST_RET: BcInst = 48;
pub const BC_INST_RET0: BcInst = 47;
pub const BC_INST_CALL: BcInst = 46;
pub const BC_INST_JUMP_ZERO: BcInst = 45;
pub const BC_INST_JUMP: BcInst = 44;
pub const BC_INST_HALT: BcInst = 43;
pub const XC_INST_PRINT_STR: BcInst = 42;
pub const XC_INST_STR: BcInst = 41;
pub const XC_INST_PRINT_POP: BcInst = 40;
pub const XC_INST_PRINT: BcInst = 39;
pub const XC_INST_SQRT: BcInst = 38;
pub const XC_INST_READ: BcInst = 37;
pub const XC_INST_LENGTH: BcInst = 36;
pub const BC_INST_LAST: BcInst = 35;
pub const XC_INST_SCALE: BcInst = 34;
pub const XC_INST_OBASE: BcInst = 33;
pub const XC_INST_IBASE: BcInst = 32;
pub const XC_INST_SCALE_FUNC: BcInst = 31;
pub const XC_INST_ARRAY: BcInst = 30;
pub const XC_INST_ARRAY_ELEM: BcInst = 29;
pub const XC_INST_VAR: BcInst = 28;
pub const XC_INST_NUM: BcInst = 27;
pub const XC_INST_ASSIGN: BcInst = 26;
pub const BC_INST_ASSIGN_MINUS: BcInst = 25;
pub const BC_INST_ASSIGN_PLUS: BcInst = 24;
pub const BC_INST_ASSIGN_MODULUS: BcInst = 23;
pub const BC_INST_ASSIGN_DIVIDE: BcInst = 22;
pub const BC_INST_ASSIGN_MULTIPLY: BcInst = 21;
pub const BC_INST_ASSIGN_POWER: BcInst = 20;
pub const XC_INST_BOOL_AND: BcInst = 19;
pub const XC_INST_BOOL_OR: BcInst = 18;
pub const XC_INST_BOOL_NOT: BcInst = 17;
pub const XC_INST_MINUS: BcInst = 16;
pub const XC_INST_PLUS: BcInst = 15;
pub const XC_INST_MODULUS: BcInst = 14;
pub const XC_INST_DIVIDE: BcInst = 13;
pub const XC_INST_MULTIPLY: BcInst = 12;
pub const XC_INST_POWER: BcInst = 11;
pub const XC_INST_REL_GT: BcInst = 10;
pub const XC_INST_REL_LT: BcInst = 9;
pub const XC_INST_REL_NE: BcInst = 8;
pub const XC_INST_REL_GE: BcInst = 7;
pub const XC_INST_REL_LE: BcInst = 6;
pub const XC_INST_REL_EQ: BcInst = 5;
pub const XC_INST_NEG: BcInst = 4;
pub const BC_INST_DEC_POST: BcInst = 3;
pub const BC_INST_INC_POST: BcInst = 2;
pub const BC_INST_DEC_PRE: BcInst = 1;
pub const BC_INST_INC_PRE: BcInst = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BcId {
  pub name: *mut libc::c_char,
  pub idx: size_t,
}
pub type BcResultType = libc::c_uint;
pub const BC_RESULT_ONE: BcResultType = 11;
pub const XC_RESULT_CONSTANT: BcResultType = 10;
pub const BC_RESULT_LAST: BcResultType = 9;
pub const XC_RESULT_SCALE: BcResultType = 8;
pub const XC_RESULT_OBASE: BcResultType = 7;
pub const XC_RESULT_IBASE: BcResultType = 6;
pub const XC_RESULT_STR: BcResultType = 5;
pub const XC_RESULT_ARRAY: BcResultType = 4;
pub const XC_RESULT_ARRAY_ELEM: BcResultType = 3;
pub const XC_RESULT_VAR: BcResultType = 2;
pub const BC_RESULT_VOID: BcResultType = 1;
pub const XC_RESULT_TEMP: BcResultType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union BcResultData {
  pub n: BcNum,
  pub v: BcVec,
  pub id: BcId,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BcResult {
  pub t: BcResultType,
  pub d: BcResultData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BcInstPtr {
  pub func: size_t,
  pub inst_idx: size_t,
}
pub type BcType = libc::c_uint;
pub const BC_TYPE_REF: BcType = 2;
pub const BC_TYPE_ARRAY: BcType = 1;
pub const BC_TYPE_VAR: BcType = 0;
pub type BcLexType = libc::c_uint;
pub const DC_LEX_SCALE_FACTOR: BcLexType = 55;
pub const DC_LEX_NQUIT: BcLexType = 54;
pub const DC_LEX_PRINT_POP: BcLexType = 53;
pub const DC_LEX_STORE_PUSH: BcLexType = 52;
pub const DC_LEX_LOAD_POP: BcLexType = 51;
pub const DC_LEX_LOAD: BcLexType = 50;
pub const DC_LEX_STORE_SCALE: BcLexType = 49;
pub const DC_LEX_STORE_OBASE: BcLexType = 48;
pub const DC_LEX_STORE_IBASE: BcLexType = 47;
pub const DC_LEX_PRINT_STREAM: BcLexType = 46;
pub const DC_LEX_ASCIIFY: BcLexType = 45;
pub const DC_LEX_POP: BcLexType = 44;
pub const DC_LEX_SWAP: BcLexType = 43;
pub const DC_LEX_DUPLICATE: BcLexType = 42;
pub const DC_LEX_STACK_LEVEL: BcLexType = 41;
pub const DC_LEX_CLEAR_STACK: BcLexType = 40;
pub const DC_LEX_PRINT_STACK: BcLexType = 39;
pub const DC_LEX_EXECUTE: BcLexType = 38;
pub const DC_LEX_ELSE: BcLexType = 37;
pub const DC_LEX_COLON: BcLexType = 36;
pub const DC_LEX_OP_DIVMOD: BcLexType = 35;
pub const DC_LEX_OP_MODEXP: BcLexType = 34;
pub const DC_LEX_EQ_NO_REG: BcLexType = 33;
pub const DC_LEX_LBRACE: BcLexType = 32;
pub const DC_LEX_SQRT: BcLexType = 31;
pub const DC_LEX_QUIT: BcLexType = 30;
pub const DC_LEX_PRINT: BcLexType = 29;
pub const DC_LEX_LENGTH: BcLexType = 28;
pub const DC_LEX_OBASE: BcLexType = 27;
pub const DC_LEX_SCALE: BcLexType = 26;
pub const DC_LEX_IBASE: BcLexType = 25;
pub const DC_LEX_READ: BcLexType = 24;
pub const DC_LEX_SCOLON: BcLexType = 23;
pub const DC_LEX_LPAREN: BcLexType = 22;
pub const DC_LEX_OP_ASSIGN: BcLexType = 21;
pub const DC_LEX_OP_BOOL_NOT: BcLexType = 20;
pub const BC_LEX_KEY_WHILE: BcLexType = 59;
pub const BC_LEX_KEY_SQRT: BcLexType = 58;
pub const BC_LEX_KEY_SCALE: BcLexType = 57;
pub const BC_LEX_KEY_RETURN: BcLexType = 56;
pub const BC_LEX_KEY_READ: BcLexType = 55;
pub const BC_LEX_KEY_QUIT: BcLexType = 54;
pub const BC_LEX_KEY_PRINT: BcLexType = 53;
pub const BC_LEX_KEY_LIMITS: BcLexType = 52;
pub const BC_LEX_KEY_LENGTH: BcLexType = 51;
pub const BC_LEX_KEY_LAST: BcLexType = 50;
pub const BC_LEX_KEY_IF: BcLexType = 49;
pub const BC_LEX_KEY_OBASE: BcLexType = 48;
pub const BC_LEX_KEY_IBASE: BcLexType = 47;
pub const BC_LEX_KEY_HALT: BcLexType = 46;
pub const BC_LEX_KEY_FOR: BcLexType = 45;
pub const BC_LEX_KEY_ELSE: BcLexType = 44;
pub const BC_LEX_KEY_DEFINE: BcLexType = 43;
pub const BC_LEX_KEY_CONTINUE: BcLexType = 42;
pub const BC_LEX_KEY_BREAK: BcLexType = 41;
pub const BC_LEX_KEY_AUTO: BcLexType = 40;
pub const BC_LEX_KEY_1st_keyword: BcLexType = 40;
pub const BC_LEX_RBRACE: BcLexType = 39;
pub const BC_LEX_SCOLON: BcLexType = 38;
pub const BC_LEX_LBRACE: BcLexType = 37;
pub const BC_LEX_RBRACKET: BcLexType = 36;
pub const BC_LEX_COMMA: BcLexType = 35;
pub const BC_LEX_LBRACKET: BcLexType = 34;
pub const BC_LEX_RPAREN: BcLexType = 33;
pub const BC_LEX_LPAREN: BcLexType = 32;
pub const BC_LEX_OP_DEC: BcLexType = 31;
pub const BC_LEX_OP_INC: BcLexType = 30;
pub const BC_LEX_OP_ASSIGN: BcLexType = 29;
pub const BC_LEX_OP_ASSIGN_MINUS: BcLexType = 28;
pub const BC_LEX_OP_ASSIGN_PLUS: BcLexType = 27;
pub const BC_LEX_OP_ASSIGN_MODULUS: BcLexType = 26;
pub const BC_LEX_OP_ASSIGN_DIVIDE: BcLexType = 25;
pub const BC_LEX_OP_ASSIGN_MULTIPLY: BcLexType = 24;
pub const BC_LEX_OP_ASSIGN_POWER: BcLexType = 23;
pub const BC_LEX_OP_BOOL_AND: BcLexType = 22;
pub const BC_LEX_OP_BOOL_OR: BcLexType = 21;
pub const BC_LEX_OP_BOOL_NOT: BcLexType = 20;
pub const XC_LEX_OP_last: BcLexType = 19;
pub const XC_LEX_OP_MINUS: BcLexType = 19;
pub const XC_LEX_OP_PLUS: BcLexType = 18;
pub const XC_LEX_OP_MODULUS: BcLexType = 17;
pub const XC_LEX_OP_DIVIDE: BcLexType = 16;
pub const XC_LEX_OP_MULTIPLY: BcLexType = 15;
pub const XC_LEX_OP_POWER: BcLexType = 14;
pub const XC_LEX_OP_REL_GT: BcLexType = 13;
pub const XC_LEX_OP_REL_LT: BcLexType = 12;
pub const XC_LEX_OP_REL_NE: BcLexType = 11;
pub const XC_LEX_OP_REL_GE: BcLexType = 10;
pub const XC_LEX_OP_REL_LE: BcLexType = 9;
pub const XC_LEX_OP_REL_EQ: BcLexType = 8;
pub const XC_LEX_NEG: BcLexType = 7;
pub const XC_LEX_1st_op: BcLexType = 7;
pub const XC_LEX_NUMBER: BcLexType = 6;
pub const XC_LEX_NAME: BcLexType = 5;
pub const XC_LEX_STR: BcLexType = 4;
pub const XC_LEX_WHITESPACE: BcLexType = 3;
pub const XC_LEX_NLINE: BcLexType = 2;
pub const XC_LEX_INVALID: BcLexType = 1;
pub const XC_LEX_EOF: BcLexType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BcLexKeyword {
  pub name8: [libc::c_char; 8],
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const POSIX_KWORD_MASK: C2RustUnnamed_1 = 1002411;
pub type C2RustUnnamed_2 = libc::c_ulong;
pub const BC_PARSE_EXPRS_BITS: C2RustUnnamed_2 = 472174290611994592;
pub type BcNumBinaryOp =
  Option<unsafe extern "C" fn(_: *mut BcNum, _: *mut BcNum, _: *mut BcNum, _: size_t) -> BcStatus>;
pub type BcNumDigitOp = Option<unsafe extern "C" fn(_: size_t, _: size_t, _: bool) -> ()>;
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
static mut bc_lex_kws: [BcLexKeyword; 20] = [
  {
    let mut init = BcLexKeyword {
      name8: [97, 117, 116, 111, 0, 0, 0, 0],
    };
    init
  },
  {
    let mut init = BcLexKeyword {
      name8: [98, 114, 101, 97, 107, 0, 0, 0],
    };
    init
  },
  {
    let mut init = BcLexKeyword {
      name8: [99, 111, 110, 116, 105, 110, 117, 101],
    };
    init
  },
  {
    let mut init = BcLexKeyword {
      name8: [100, 101, 102, 105, 110, 101, 0, 0],
    };
    init
  },
  {
    let mut init = BcLexKeyword {
      name8: [101, 108, 115, 101, 0, 0, 0, 0],
    };
    init
  },
  {
    let mut init = BcLexKeyword {
      name8: [102, 111, 114, 0, 0, 0, 0, 0],
    };
    init
  },
  {
    let mut init = BcLexKeyword {
      name8: [104, 97, 108, 116, 0, 0, 0, 0],
    };
    init
  },
  {
    let mut init = BcLexKeyword {
      name8: [105, 98, 97, 115, 101, 0, 0, 0],
    };
    init
  },
  {
    let mut init = BcLexKeyword {
      name8: [111, 98, 97, 115, 101, 0, 0, 0],
    };
    init
  },
  {
    let mut init = BcLexKeyword {
      name8: [105, 102, 0, 0, 0, 0, 0, 0],
    };
    init
  },
  {
    let mut init = BcLexKeyword {
      name8: [108, 97, 115, 116, 0, 0, 0, 0],
    };
    init
  },
  {
    let mut init = BcLexKeyword {
      name8: [108, 101, 110, 103, 116, 104, 0, 0],
    };
    init
  },
  {
    let mut init = BcLexKeyword {
      name8: [108, 105, 109, 105, 116, 115, 0, 0],
    };
    init
  },
  {
    let mut init = BcLexKeyword {
      name8: [112, 114, 105, 110, 116, 0, 0, 0],
    };
    init
  },
  {
    let mut init = BcLexKeyword {
      name8: [113, 117, 105, 116, 0, 0, 0, 0],
    };
    init
  },
  {
    let mut init = BcLexKeyword {
      name8: [114, 101, 97, 100, 0, 0, 0, 0],
    };
    init
  },
  {
    let mut init = BcLexKeyword {
      name8: [114, 101, 116, 117, 114, 110, 0, 0],
    };
    init
  },
  {
    let mut init = BcLexKeyword {
      name8: [115, 99, 97, 108, 101, 0, 0, 0],
    };
    init
  },
  {
    let mut init = BcLexKeyword {
      name8: [115, 113, 114, 116, 0, 0, 0, 0],
    };
    init
  },
  {
    let mut init = BcLexKeyword {
      name8: [119, 104, 105, 108, 101, 0, 0, 0],
    };
    init
  },
];
#[inline(always)]
unsafe extern "C" fn lex_allowed_in_bc_expr(mut i: libc::c_uint) -> libc::c_long {
  return (BC_PARSE_EXPRS_BITS as libc::c_ulong & 1u64 << i) as libc::c_long;
}
static mut bc_ops_prec_and_assoc: [uint8_t; 25] = [
  (0i32 * 0x10i32 + 1i32) as uint8_t,
  (1i32 * 0x10i32 + 6i32) as uint8_t,
  (1i32 * 0x10i32 + 6i32) as uint8_t,
  (1i32 * 0x10i32 + 6i32) as uint8_t,
  (1i32 * 0x10i32 + 6i32) as uint8_t,
  (1i32 * 0x10i32 + 6i32) as uint8_t,
  (1i32 * 0x10i32 + 6i32) as uint8_t,
  (0i32 * 0x10i32 + 2i32) as uint8_t,
  (1i32 * 0x10i32 + 3i32) as uint8_t,
  (1i32 * 0x10i32 + 3i32) as uint8_t,
  (1i32 * 0x10i32 + 3i32) as uint8_t,
  (1i32 * 0x10i32 + 4i32) as uint8_t,
  (1i32 * 0x10i32 + 4i32) as uint8_t,
  (0i32 * 0x10i32 + 1i32) as uint8_t,
  (1i32 * 0x10i32 + 7i32) as uint8_t,
  (1i32 * 0x10i32 + 7i32) as uint8_t,
  (0i32 * 0x10i32 + 5i32) as uint8_t,
  (0i32 * 0x10i32 + 5i32) as uint8_t,
  (0i32 * 0x10i32 + 5i32) as uint8_t,
  (0i32 * 0x10i32 + 5i32) as uint8_t,
  (0i32 * 0x10i32 + 5i32) as uint8_t,
  (0i32 * 0x10i32 + 5i32) as uint8_t,
  (0i32 * 0x10i32 + 5i32) as uint8_t,
  (0i32 * 0x10i32 + 0i32) as uint8_t,
  (0i32 * 0x10i32 + 0i32) as uint8_t,
];
static mut dc_char_to_LEX: [uint8_t; 90] = [
  XC_LEX_OP_MODULUS as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  DC_LEX_LPAREN as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_OP_MULTIPLY as libc::c_int as uint8_t,
  XC_LEX_OP_PLUS as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_OP_MINUS as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_OP_DIVIDE as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  DC_LEX_COLON as libc::c_int as uint8_t,
  DC_LEX_SCOLON as libc::c_int as uint8_t,
  XC_LEX_OP_REL_GT as libc::c_int as uint8_t,
  XC_LEX_OP_REL_EQ as libc::c_int as uint8_t,
  XC_LEX_OP_REL_LT as libc::c_int as uint8_t,
  DC_LEX_READ as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  DC_LEX_EQ_NO_REG as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  DC_LEX_IBASE as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  DC_LEX_SCALE as libc::c_int as uint8_t,
  DC_LEX_LOAD_POP as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  DC_LEX_OP_BOOL_NOT as libc::c_int as uint8_t,
  DC_LEX_OBASE as libc::c_int as uint8_t,
  DC_LEX_PRINT_STREAM as libc::c_int as uint8_t,
  DC_LEX_NQUIT as libc::c_int as uint8_t,
  DC_LEX_POP as libc::c_int as uint8_t,
  DC_LEX_STORE_PUSH as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  DC_LEX_SCALE_FACTOR as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  DC_LEX_LENGTH as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_OP_POWER as libc::c_int as uint8_t,
  XC_LEX_NEG as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  DC_LEX_ASCIIFY as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  DC_LEX_CLEAR_STACK as libc::c_int as uint8_t,
  DC_LEX_DUPLICATE as libc::c_int as uint8_t,
  DC_LEX_ELSE as libc::c_int as uint8_t,
  DC_LEX_PRINT_STACK as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  DC_LEX_STORE_IBASE as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  DC_LEX_STORE_SCALE as libc::c_int as uint8_t,
  DC_LEX_LOAD as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  DC_LEX_PRINT_POP as libc::c_int as uint8_t,
  DC_LEX_STORE_OBASE as libc::c_int as uint8_t,
  DC_LEX_PRINT as libc::c_int as uint8_t,
  DC_LEX_QUIT as libc::c_int as uint8_t,
  DC_LEX_SWAP as libc::c_int as uint8_t,
  DC_LEX_OP_ASSIGN as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  DC_LEX_SQRT as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  DC_LEX_EXECUTE as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  DC_LEX_STACK_LEVEL as libc::c_int as uint8_t,
  DC_LEX_LBRACE as libc::c_int as uint8_t,
  DC_LEX_OP_MODEXP as libc::c_int as uint8_t,
  XC_LEX_INVALID as libc::c_int as uint8_t,
  DC_LEX_OP_DIVMOD as libc::c_int as uint8_t,
];
static mut dc_LEX_to_INST: [int8_t; 42] = [
  XC_INST_POWER as libc::c_int as int8_t,
  XC_INST_MULTIPLY as libc::c_int as int8_t,
  XC_INST_DIVIDE as libc::c_int as int8_t,
  XC_INST_MODULUS as libc::c_int as int8_t,
  XC_INST_PLUS as libc::c_int as int8_t,
  XC_INST_MINUS as libc::c_int as int8_t,
  XC_INST_BOOL_NOT as libc::c_int as int8_t,
  DC_INST_INVALID as libc::c_int as int8_t,
  XC_INST_REL_GT as libc::c_int as int8_t,
  DC_INST_INVALID as libc::c_int as int8_t,
  DC_INST_INVALID as libc::c_int as int8_t,
  XC_INST_IBASE as libc::c_int as int8_t,
  XC_INST_SCALE as libc::c_int as int8_t,
  XC_INST_OBASE as libc::c_int as int8_t,
  XC_INST_LENGTH as libc::c_int as int8_t,
  XC_INST_PRINT as libc::c_int as int8_t,
  DC_INST_QUIT as libc::c_int as int8_t,
  XC_INST_SQRT as libc::c_int as int8_t,
  XC_INST_REL_GE as libc::c_int as int8_t,
  XC_INST_REL_EQ as libc::c_int as int8_t,
  DC_INST_MODEXP as libc::c_int as int8_t,
  DC_INST_DIVMOD as libc::c_int as int8_t,
  DC_INST_INVALID as libc::c_int as int8_t,
  DC_INST_INVALID as libc::c_int as int8_t,
  DC_INST_EXECUTE as libc::c_int as int8_t,
  DC_INST_PRINT_STACK as libc::c_int as int8_t,
  DC_INST_CLEAR_STACK as libc::c_int as int8_t,
  DC_INST_STACK_LEN as libc::c_int as int8_t,
  DC_INST_DUPLICATE as libc::c_int as int8_t,
  DC_INST_SWAP as libc::c_int as int8_t,
  XC_INST_POP as libc::c_int as int8_t,
  DC_INST_ASCIIFY as libc::c_int as int8_t,
  DC_INST_PRINT_STREAM as libc::c_int as int8_t,
  DC_INST_INVALID as libc::c_int as int8_t,
  DC_INST_INVALID as libc::c_int as int8_t,
  DC_INST_INVALID as libc::c_int as int8_t,
  DC_INST_INVALID as libc::c_int as int8_t,
  DC_INST_INVALID as libc::c_int as int8_t,
  DC_INST_INVALID as libc::c_int as int8_t,
  XC_INST_PRINT as libc::c_int as int8_t,
  DC_INST_NQUIT as libc::c_int as int8_t,
  XC_INST_SCALE_FUNC as libc::c_int as int8_t,
];
// Unused apart from "limits" message. Just show a "biggish number" there.
//#define BC_MAX_EXP      ((unsigned long) LONG_MAX)
//#define BC_MAX_VARS     ((unsigned long) SIZE_MAX - 1)
// In configurations where errors abort instead of propagating error
// return code up the call chain, functions returning BC_STATUS
// actually don't return anything, they always succeed and return "void".
// A macro wrapper is provided, which makes this statement work:
//  s = zbc_func(...)
// and makes it visible to the compiler that s is always zero,
// allowing compiler to optimize dead code after the statement.
//
// To make code more readable, each such function has a "z"
// ("always returning zero") prefix, i.e. zbc_foo or zdc_foo.
//
/*nothing*/
/*nothing*/
//
// Utility routines
//
unsafe extern "C" fn fflush_and_check() {
  fflush_all(); // for compiler
  if ferror_unlocked(stdout) != 0 || ferror_unlocked(stderr) != 0 {
    bb_simple_perror_msg_and_die(b"output error\x00" as *const u8 as *const libc::c_char);
  };
}
unsafe extern "C" fn quit() -> ! {
  if ferror_unlocked(stdin) != 0 {
    bb_simple_perror_msg_and_die(b"input error\x00" as *const u8 as *const libc::c_char);
  }
  fflush_and_check();
  exit(0i32);
}
unsafe extern "C" fn bc_verror_msg(mut fmt: *const libc::c_char, mut p: ::std::ffi::VaList) {
  let mut sv: *const libc::c_char = 0 as *const libc::c_char;
  sv = sv;
  if !(*ptr_to_globals).prs.lex_filename.is_null() {
    sv = applet_name;
    applet_name = xasprintf(
      b"%s: %s:%lu\x00" as *const u8 as *const libc::c_char,
      applet_name,
      (*ptr_to_globals).prs.lex_filename,
      (*ptr_to_globals).err_line,
    )
  }
  bb_verror_msg(fmt, p.as_va_list(), 0 as *const libc::c_char);
  if !(*ptr_to_globals).prs.lex_filename.is_null() {
    free(applet_name as *mut libc::c_char as *mut libc::c_void);
    applet_name = sv
  };
}
#[inline(never)]
unsafe extern "C" fn bc_error_fmt(mut fmt: *const libc::c_char, mut args: ...) -> libc::c_int {
  let mut p: ::std::ffi::VaListImpl;
  p = args.clone();
  bc_verror_msg(fmt, p.as_va_list());
  if 0i32 != 0 || (*ptr_to_globals).ttyin as libc::c_int != 0 {
    return BC_STATUS_FAILURE as libc::c_int;
  }
  exit(1i32);
}
#[inline(never)]
unsafe extern "C" fn zbc_posix_error_fmt(mut fmt: *const libc::c_char, mut args: ...) -> BcStatus {
  let mut p: ::std::ffi::VaListImpl;
  // Are non-POSIX constructs totally ok?
  if option_mask32 & (1i32 << 2i32 | 1i32 << 0i32) as libc::c_uint == 0 {
    return BC_STATUS_SUCCESS;
  } // yes
  p = args.clone();
  bc_verror_msg(fmt, p.as_va_list());
  // Do we treat non-POSIX constructs as errors?
  if option_mask32 & (1i32 << 2i32) as libc::c_uint == 0 {
    return BC_STATUS_SUCCESS;
  } // no, it's a warning
  if 0i32 != 0 || (*ptr_to_globals).ttyin as libc::c_int != 0 {
    return BC_STATUS_FAILURE;
  }
  exit(1i32);
}
// We use error functions with "return bc_error(FMT[, PARAMS])" idiom.
// This idiom begs for tail-call optimization, but for it to work,
// function must not have caller-cleaned parameters on stack.
// Unfortunately, vararg function API does exactly that on most arches.
// Thus, use these shims for the cases when we have no vararg PARAMS:
unsafe extern "C" fn bc_error(mut msg: *const libc::c_char) -> libc::c_int {
  return bc_error_fmt(b"%s\x00" as *const u8 as *const libc::c_char, msg);
}
unsafe extern "C" fn bc_error_at(mut msg: *const libc::c_char) -> libc::c_int {
  let mut err_at: *const libc::c_char = (*ptr_to_globals).prs.lex_next_at;
  if !err_at.is_null() {
    return bc_error_fmt(
      b"%s at \'%.*s\'\x00" as *const u8 as *const libc::c_char,
      msg,
      strchrnul(err_at, '\n' as i32).wrapping_offset_from(err_at) as libc::c_long as libc::c_int,
      err_at,
    );
  }
  return bc_error_fmt(b"%s\x00" as *const u8 as *const libc::c_char, msg);
}
unsafe extern "C" fn bc_error_bad_character(mut c: libc::c_char) -> libc::c_int {
  if c == 0 {
    return bc_error(b"NUL character\x00" as *const u8 as *const libc::c_char);
  }
  return bc_error_fmt(
    b"bad character \'%c\'\x00" as *const u8 as *const libc::c_char,
    c as libc::c_int,
  );
}
unsafe extern "C" fn bc_error_bad_function_definition() -> libc::c_int {
  return bc_error_at(b"bad function definition\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn bc_error_bad_expression() -> libc::c_int {
  return bc_error_at(b"bad expression\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn bc_error_bad_assignment() -> libc::c_int {
  return bc_error_at(
    b"bad assignment: left side must be variable or array element\x00" as *const u8
      as *const libc::c_char,
  );
}
unsafe extern "C" fn bc_error_bad_token() -> libc::c_int {
  return bc_error_at(b"bad token\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn bc_error_stack_has_too_few_elements() -> libc::c_int {
  return bc_error(b"stack has too few elements\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn bc_error_variable_is_wrong_type() -> libc::c_int {
  return bc_error(b"variable is wrong type\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn zbc_POSIX_requires(mut msg: *const libc::c_char) -> BcStatus {
  return zbc_posix_error_fmt(
    b"POSIX requires %s\x00" as *const u8 as *const libc::c_char,
    msg,
  );
}
unsafe extern "C" fn zbc_POSIX_does_not_allow(mut msg: *const libc::c_char) -> BcStatus {
  return zbc_posix_error_fmt(
    b"%s%s\x00" as *const u8 as *const libc::c_char,
    b"POSIX does not allow \x00" as *const u8 as *const libc::c_char,
    msg,
  );
}
unsafe extern "C" fn zbc_POSIX_does_not_allow_bool_ops_this_is_bad(
  mut msg: *const libc::c_char,
) -> BcStatus {
  return zbc_posix_error_fmt(
    b"%s%s %s\x00" as *const u8 as *const libc::c_char,
    b"POSIX does not allow \x00" as *const u8 as *const libc::c_char,
    b"boolean operators; this is bad:\x00" as *const u8 as *const libc::c_char,
    msg,
  );
}
unsafe extern "C" fn zbc_POSIX_does_not_allow_empty_X_expression_in_for(
  mut msg: *const libc::c_char,
) -> BcStatus {
  return zbc_posix_error_fmt(
    b"%san empty %s expression in \'for()\'\x00" as *const u8 as *const libc::c_char,
    b"POSIX does not allow \x00" as *const u8 as *const libc::c_char,
    msg,
  );
}
unsafe extern "C" fn bc_vec_grow(mut v: *mut BcVec, mut n: size_t) {
  let mut cap: size_t = (*v).cap.wrapping_mul(2i32 as libc::c_ulong);
  while cap < (*v).len.wrapping_add(n) {
    cap = (cap as libc::c_ulong).wrapping_mul(2i32 as libc::c_ulong) as size_t as size_t
  }
  (*v).v = xrealloc((*v).v as *mut libc::c_void, (*v).size.wrapping_mul(cap)) as *mut libc::c_char;
  (*v).cap = cap;
}
unsafe extern "C" fn bc_vec_init(mut v: *mut BcVec, mut esize: size_t, mut dtor: BcVecFree) {
  (*v).size = esize;
  (*v).cap = (1i32 << 5i32) as size_t;
  (*v).len = 0i32 as size_t;
  (*v).dtor = dtor;
  (*v).v = xmalloc(esize.wrapping_mul((1i32 << 5i32) as libc::c_ulong)) as *mut libc::c_char;
}
unsafe extern "C" fn bc_char_vec_init(mut v: *mut BcVec) {
  bc_vec_init(
    v,
    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
    None,
  );
}
unsafe extern "C" fn bc_vec_expand(mut v: *mut BcVec, mut req: size_t) {
  if (*v).cap < req {
    (*v).v =
      xrealloc((*v).v as *mut libc::c_void, (*v).size.wrapping_mul(req)) as *mut libc::c_char;
    (*v).cap = req
  };
}
unsafe extern "C" fn bc_vec_pop(mut v: *mut BcVec) {
  (*v).len = (*v).len.wrapping_sub(1);
  if (*v).dtor.is_some() {
    (*v).dtor.expect("non-null function pointer")(
      (*v).v.offset((*v).size.wrapping_mul((*v).len) as isize) as *mut libc::c_void,
    );
  };
}
unsafe extern "C" fn bc_vec_npop(mut v: *mut BcVec, mut n: size_t) {
  if (*v).dtor.is_none() {
    (*v).len = ((*v).len as libc::c_ulong).wrapping_sub(n) as size_t as size_t
  } else {
    let mut len: size_t = (*v).len.wrapping_sub(n);
    while (*v).len > len {
      (*v).len = (*v).len.wrapping_sub(1);
      (*v).dtor.expect("non-null function pointer")(
        (*v).v.offset((*v).size.wrapping_mul((*v).len) as isize) as *mut libc::c_void,
      );
    }
  };
}
unsafe extern "C" fn bc_vec_pop_all(mut v: *mut BcVec) {
  bc_vec_npop(v, (*v).len);
}
unsafe extern "C" fn bc_vec_npush(
  mut v: *mut BcVec,
  mut n: size_t,
  mut data: *const libc::c_void,
) -> size_t {
  let mut len: size_t = (*v).len;
  if len.wrapping_add(n) > (*v).cap {
    bc_vec_grow(v, n);
  }
  memmove(
    (*v).v.offset((*v).size.wrapping_mul(len) as isize) as *mut libc::c_void,
    data,
    (*v).size.wrapping_mul(n),
  );
  (*v).len = len.wrapping_add(n);
  return len;
}
unsafe extern "C" fn bc_vec_push(mut v: *mut BcVec, mut data: *const libc::c_void) -> size_t {
  return bc_vec_npush(v, 1i32 as size_t, data);
  //size_t len = v->len;
  //if (len >= v->cap) bc_vec_grow(v, 1);
  //memmove(v->v + (v->size * len), data, v->size);
  //v->len = len + 1;
  //return len;
}
// G.prog.results often needs "pop old operand, push result" idiom.
// Can do this without a few extra ops
unsafe extern "C" fn bc_result_pop_and_push(mut data: *const libc::c_void) -> size_t {
  let mut v: *mut BcVec = &mut (*ptr_to_globals).prog.results;
  let mut last: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut len: size_t = (*v).len.wrapping_sub(1i32 as libc::c_ulong);
  last = (*v).v.offset((*v).size.wrapping_mul(len) as isize);
  if (*v).dtor.is_some() {
    (*v).dtor.expect("non-null function pointer")(last as *mut libc::c_void);
  }
  memmove(last as *mut libc::c_void, data, (*v).size);
  return len;
}
unsafe extern "C" fn bc_vec_pushByte(mut v: *mut BcVec, mut data: libc::c_char) -> size_t {
  return bc_vec_push(v, &mut data as *mut libc::c_char as *const libc::c_void);
}
unsafe extern "C" fn bc_vec_pushZeroByte(mut v: *mut BcVec) -> size_t {
  //return bc_vec_pushByte(v, '\0');
  // better:
  return bc_vec_push(v, &const_int_0 as *const libc::c_int as *const libc::c_void);
}
unsafe extern "C" fn bc_vec_pushAt(
  mut v: *mut BcVec,
  mut data: *const libc::c_void,
  mut idx: size_t,
) {
  if idx == (*v).len {
    bc_vec_push(v, data);
  } else {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*v).len == (*v).cap {
      bc_vec_grow(v, 1i32 as size_t);
    }
    ptr = (*v).v.offset((*v).size.wrapping_mul(idx) as isize);
    let fresh0 = (*v).len;
    (*v).len = (*v).len.wrapping_add(1);
    memmove(
      ptr.offset((*v).size as isize) as *mut libc::c_void,
      ptr as *const libc::c_void,
      (*v).size.wrapping_mul(fresh0.wrapping_sub(idx)),
    );
    memmove(ptr as *mut libc::c_void, data, (*v).size);
  };
}
unsafe extern "C" fn bc_vec_string(
  mut v: *mut BcVec,
  mut len: size_t,
  mut str: *const libc::c_char,
) {
  bc_vec_pop_all(v);
  bc_vec_expand(v, len.wrapping_add(1i32 as libc::c_ulong));
  memcpy((*v).v as *mut libc::c_void, str as *const libc::c_void, len);
  (*v).len = len;
  bc_vec_pushZeroByte(v);
}
unsafe extern "C" fn bc_vec_item(mut v: *const BcVec, mut idx: size_t) -> *mut libc::c_void {
  return (*v).v.offset((*v).size.wrapping_mul(idx) as isize) as *mut libc::c_void;
}
unsafe extern "C" fn bc_vec_item_rev(mut v: *const BcVec, mut idx: size_t) -> *mut libc::c_void {
  return (*v).v.offset(
    (*v).size.wrapping_mul(
      (*v)
        .len
        .wrapping_sub(idx)
        .wrapping_sub(1i32 as libc::c_ulong),
    ) as isize,
  ) as *mut libc::c_void;
}
unsafe extern "C" fn bc_vec_top(mut v: *const BcVec) -> *mut libc::c_void {
  return (*v).v.offset(
    (*v)
      .size
      .wrapping_mul((*v).len.wrapping_sub(1i32 as libc::c_ulong)) as isize,
  ) as *mut libc::c_void;
}
unsafe extern "C" fn bc_vec_free(mut vec: *mut libc::c_void) {
  let mut v: *mut BcVec = vec as *mut BcVec;
  bc_vec_pop_all(v);
  free((*v).v as *mut libc::c_void);
}
unsafe extern "C" fn xc_program_func(mut idx: size_t) -> *mut BcFunc {
  return bc_vec_item(&mut (*ptr_to_globals).prog.fns, idx) as *mut BcFunc;
}
// BC_PROG_MAIN is zeroth element, so:
unsafe extern "C" fn bc_program_current_func() -> *mut BcFunc {
  let mut ip: *mut BcInstPtr = bc_vec_top(&mut (*ptr_to_globals).prog.exestack) as *mut BcInstPtr; // "was not inserted"
  let mut func: *mut BcFunc = xc_program_func((*ip).func);
  return func;
}
unsafe extern "C" fn xc_program_str(mut idx: size_t) -> *mut *mut libc::c_char {
  if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(0) as libc::c_int == 'b' as i32) {
    let mut func: *mut BcFunc = bc_program_current_func();
    return bc_vec_item(&mut (*func).strs, idx) as *mut *mut libc::c_char;
  }
  return bc_vec_item(&mut (*ptr_to_globals).prog.strs, idx) as *mut *mut libc::c_char;
}
unsafe extern "C" fn xc_program_const(mut idx: size_t) -> *mut *mut libc::c_char {
  if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(0) as libc::c_int == 'b' as i32) {
    let mut func: *mut BcFunc = bc_program_current_func();
    return bc_vec_item(&mut (*func).consts, idx) as *mut *mut libc::c_char;
  }
  return bc_vec_item(&mut (*ptr_to_globals).prog.consts, idx) as *mut *mut libc::c_char;
}
unsafe extern "C" fn bc_id_cmp(
  mut e1: *const libc::c_void,
  mut e2: *const libc::c_void,
) -> libc::c_int {
  return strcmp((*(e1 as *const BcId)).name, (*(e2 as *const BcId)).name);
}
unsafe extern "C" fn bc_id_free(mut id: *mut libc::c_void) {
  free((*(id as *mut BcId)).name as *mut libc::c_void);
}
unsafe extern "C" fn bc_map_find_ge(mut v: *const BcVec, mut ptr: *const libc::c_void) -> size_t {
  let mut low: size_t = 0i32 as size_t;
  let mut high: size_t = (*v).len;
  while low < high {
    let mut mid: size_t = low.wrapping_add(high).wrapping_div(2i32 as libc::c_ulong);
    let mut id: *mut BcId = bc_vec_item(v, mid) as *mut BcId;
    let mut result: libc::c_int = bc_id_cmp(ptr, id as *const libc::c_void);
    if result == 0i32 {
      return mid;
    }
    if result < 0i32 {
      high = mid
    } else {
      low = mid.wrapping_add(1i32 as libc::c_ulong)
    }
  }
  return low;
}
unsafe extern "C" fn bc_map_insert(
  mut v: *mut BcVec,
  mut ptr: *const libc::c_void,
  mut i: *mut size_t,
) -> libc::c_int {
  *i = bc_map_find_ge(v, ptr);
  let mut n: size_t = *i;
  if n == (*v).len {
    bc_vec_push(v, ptr);
  } else if bc_id_cmp(ptr, bc_vec_item(v, n)) == 0 {
    return 0i32;
  } else {
    bc_vec_pushAt(v, ptr, n);
  }
  return 1i32;
  // "was inserted"
}
unsafe extern "C" fn bc_map_find_exact(
  mut v: *const BcVec,
  mut ptr: *const libc::c_void,
) -> size_t {
  let mut i: size_t = bc_map_find_ge(v, ptr);
  if i >= (*v).len {
    return -1i32 as size_t;
  }
  return if bc_id_cmp(ptr, bc_vec_item(v, i)) != 0 {
    -1i32 as size_t
  } else {
    i
  };
}
unsafe extern "C" fn bc_num_setToZero(mut n: *mut BcNum, mut scale: size_t) {
  (*n).len = 0i32 as size_t;
  (*n).neg = 0i32 != 0;
  (*n).rdx = scale;
}
unsafe extern "C" fn bc_num_zero(mut n: *mut BcNum) {
  bc_num_setToZero(n, 0i32 as size_t);
}
unsafe extern "C" fn bc_num_one(mut n: *mut BcNum) {
  bc_num_setToZero(n, 0i32 as size_t);
  (*n).len = 1i32 as size_t;
  *(*n).num.offset(0) = 1i32 as BcDig;
}
// Note: this also sets BcNum to zero
unsafe extern "C" fn bc_num_init(mut n: *mut BcNum, mut req: size_t) {
  req = if req >= 16i32 as libc::c_ulong {
    req
  } else {
    16i32 as libc::c_ulong
  };
  //memset(n, 0, sizeof(BcNum)); - cleared by assignments below
  (*n).num = xmalloc(req) as *mut BcDig;
  (*n).cap = req;
  (*n).rdx = 0i32 as size_t;
  (*n).len = 0i32 as size_t;
  (*n).neg = 0i32 != 0;
}
unsafe extern "C" fn bc_num_init_DEF_SIZE(mut n: *mut BcNum) {
  bc_num_init(n, 16i32 as size_t);
}
unsafe extern "C" fn bc_num_expand(mut n: *mut BcNum, mut req: size_t) {
  req = if req >= 16i32 as libc::c_ulong {
    req
  } else {
    16i32 as libc::c_ulong
  };
  if req > (*n).cap {
    (*n).num = xrealloc((*n).num as *mut libc::c_void, req) as *mut BcDig;
    (*n).cap = req
  };
}
unsafe extern "C" fn bc_num_free(mut num: *mut libc::c_void) {
  free((*(num as *mut BcNum)).num as *mut libc::c_void);
}
unsafe extern "C" fn bc_num_copy(mut d: *mut BcNum, mut s: *mut BcNum) {
  if d != s {
    bc_num_expand(d, (*s).cap);
    (*d).len = (*s).len;
    (*d).neg = (*s).neg;
    (*d).rdx = (*s).rdx;
    memcpy(
      (*d).num as *mut libc::c_void,
      (*s).num as *const libc::c_void,
      (::std::mem::size_of::<BcDig>() as libc::c_ulong).wrapping_mul((*d).len),
    );
  };
}
unsafe extern "C" fn zbc_num_ulong_abs(
  mut n: *mut BcNum,
  mut result_p: *mut libc::c_ulong,
) -> BcStatus {
  let mut i: size_t = 0;
  let mut result: libc::c_ulong = 0;
  result = 0i32 as libc::c_ulong;
  i = (*n).len;
  while i > (*n).rdx {
    let mut prev: libc::c_ulong = result;
    i = i.wrapping_sub(1);
    result = result
      .wrapping_mul(10i32 as libc::c_ulong)
      .wrapping_add(*(*n).num.offset(i as isize) as libc::c_ulong);
    // Even overflowed N*10 can still satisfy N*10>=N. For example,
    //    0x1ff00000 * 10 is 0x13f600000,
    // or 0x3f600000 truncated to 32 bits. Which is larger.
    // However, (N*10)/8 < N check is always correct.
    if result.wrapping_div(8i32 as libc::c_ulong) < prev {
      return bc_error(b"overflow\x00" as *const u8 as *const libc::c_char) as BcStatus;
    }
  }
  *result_p = result;
  return BC_STATUS_SUCCESS;
}
unsafe extern "C" fn zbc_num_ulong(
  mut n: *mut BcNum,
  mut result_p: *mut libc::c_ulong,
) -> BcStatus {
  if (*n).neg {
    return bc_error(b"negative number\x00" as *const u8 as *const libc::c_char) as BcStatus;
  }
  return zbc_num_ulong_abs(n, result_p);
}
// minimum BC_NUM_DEF_SIZE, so that bc_num_expand() in bc_num_ulong2num()
// would not hit realloc() code path - not good if num[] is not malloced
unsafe extern "C" fn bc_num_ulong2num(mut n: *mut BcNum, mut val: libc::c_ulong) {
  let mut ptr: *mut BcDig = 0 as *mut BcDig;
  bc_num_zero(n);
  if val == 0i32 as libc::c_ulong {
    return;
  }
  bc_num_expand(n, if 20i32 > 16i32 { 20i32 } else { 16i32 } as size_t);
  ptr = (*n).num;
  loop {
    (*n).len = (*n).len.wrapping_add(1);
    let fresh1 = ptr;
    ptr = ptr.offset(1);
    *fresh1 = val.wrapping_rem(10i32 as libc::c_ulong) as BcDig;
    val = val.wrapping_div(10i32 as libc::c_ulong);
    if val == 0i32 as libc::c_ulong {
      break;
    }
  }
}
unsafe extern "C" fn bc_num_subArrays(mut a: *mut BcDig, mut b: *mut BcDig, mut len: size_t) {
  let mut i: size_t = 0;
  let mut j: size_t = 0;
  i = 0i32 as size_t;
  while i < len {
    let ref mut fresh2 = *a.offset(i as isize);
    *fresh2 = (*fresh2 as libc::c_int - *b.offset(i as isize) as libc::c_int) as BcDig;
    j = i;
    while (*a.offset(j as isize) as libc::c_int) < 0i32 {
      let fresh3 = j;
      j = j.wrapping_add(1);
      let ref mut fresh4 = *a.offset(fresh3 as isize);
      *fresh4 = (*fresh4 as libc::c_int + 10i32) as BcDig;
      let ref mut fresh5 = *a.offset(j as isize);
      *fresh5 = (*fresh5 as libc::c_int - 1i32) as BcDig
    }
    i = i.wrapping_add(1)
  }
}
unsafe extern "C" fn bc_num_compare(
  mut a: *mut BcDig,
  mut b: *mut BcDig,
  mut len: size_t,
) -> ssize_t {
  let mut i: size_t = len;
  loop {
    let mut c: libc::c_int = 0;
    if i == 0i32 as libc::c_ulong {
      return 0i32 as ssize_t;
    }
    i = i.wrapping_sub(1);
    c = *a.offset(i as isize) as libc::c_int - *b.offset(i as isize) as libc::c_int;
    if c != 0i32 {
      i = i.wrapping_add(1);
      if c < 0i32 {
        return i.wrapping_neg() as ssize_t;
      }
      return i as ssize_t;
    }
  }
}
//#define BC_NUM_AREQ(a, b)       (BC_MAX((a)->rdx, (b)->rdx) + BC_MAX(BC_NUM_INT(a), BC_NUM_INT(b)) + 1)
unsafe extern "C" fn BC_NUM_AREQ(mut a: *mut BcNum, mut b: *mut BcNum) -> size_t {
  return (if (*a).rdx > (*b).rdx {
    (*a).rdx
  } else {
    (*b).rdx
  })
  .wrapping_add(
    if (*a).len.wrapping_sub((*a).rdx) > (*b).len.wrapping_sub((*b).rdx) {
      (*a).len.wrapping_sub((*a).rdx)
    } else {
      (*b).len.wrapping_sub((*b).rdx)
    },
  )
  .wrapping_add(1i32 as libc::c_ulong);
}
//#define BC_NUM_MREQ(a, b, scale) (BC_NUM_INT(a) + BC_NUM_INT(b) + BC_MAX((scale), (a)->rdx + (b)->rdx) + 1)
unsafe extern "C" fn BC_NUM_MREQ(
  mut a: *mut BcNum,
  mut b: *mut BcNum,
  mut scale: size_t,
) -> size_t {
  return (*a)
    .len
    .wrapping_sub((*a).rdx)
    .wrapping_add((*b).len.wrapping_sub((*b).rdx))
    .wrapping_add(if scale > (*a).rdx.wrapping_add((*b).rdx) {
      scale
    } else {
      (*a).rdx.wrapping_add((*b).rdx)
    })
    .wrapping_add(1i32 as libc::c_ulong);
}
unsafe extern "C" fn bc_num_cmp(mut a: *mut BcNum, mut b: *mut BcNum) -> ssize_t {
  let mut i: size_t = 0;
  let mut min: size_t = 0;
  let mut a_int: size_t = 0;
  let mut b_int: size_t = 0;
  let mut diff: size_t = 0;
  let mut max_num: *mut BcDig = 0 as *mut BcDig;
  let mut min_num: *mut BcDig = 0 as *mut BcDig;
  let mut a_max: bool = false;
  let mut neg: bool = false;
  let mut cmp: ssize_t = 0;
  if a == b {
    return 0i32 as ssize_t;
  }
  if (*a).len == 0i32 as libc::c_ulong {
    return (((*b).len != 0) as libc::c_int as ssize_t ^ -(!(*b).neg as libc::c_int as ssize_t))
      + !(*b).neg as libc::c_int as libc::c_long;
  }
  if (*b).len == 0i32 as libc::c_ulong {
    return (1i32 as ssize_t ^ -((*a).neg as ssize_t)) + (*a).neg as libc::c_long;
  }
  if (*a).neg as libc::c_int != (*b).neg as libc::c_int {
    // signs of a and b differ
    // +a,-b = a>b = 1 or -a,+b = a<b = -1
    return ((*b).neg as libc::c_int - (*a).neg as libc::c_int) as ssize_t;
  } // 1 if both negative, 0 if both positive
  neg = (*a).neg;
  a_int = (*a).len.wrapping_sub((*a).rdx);
  b_int = (*b).len.wrapping_sub((*b).rdx);
  a_int = (a_int as libc::c_ulong).wrapping_sub(b_int) as size_t as size_t;
  if a_int != 0i32 as libc::c_ulong {
    return a_int as ssize_t;
  }
  a_max = (*a).rdx > (*b).rdx;
  if a_max {
    min = (*b).rdx;
    diff = (*a).rdx.wrapping_sub((*b).rdx);
    max_num = (*a).num.offset(diff as isize);
    min_num = (*b).num
  // neg = (a_max == neg); - NOP (maps 1->1 and 0->0)
  } else {
    min = (*a).rdx;
    diff = (*b).rdx.wrapping_sub((*a).rdx);
    max_num = (*b).num.offset(diff as isize);
    min_num = (*a).num;
    neg = !neg
    // same as "neg = (a_max == neg)"
  }
  cmp = bc_num_compare(max_num, min_num, b_int.wrapping_add(min));
  if cmp != 0i32 as libc::c_long {
    return (cmp ^ -(neg as ssize_t)) + neg as libc::c_long;
  }
  max_num = max_num.offset(-(diff as isize));
  i = diff.wrapping_sub(1i32 as libc::c_ulong);
  while i < diff {
    if *max_num.offset(i as isize) != 0 {
      return (1i32 as ssize_t ^ -(neg as ssize_t)) + neg as libc::c_long;
    }
    i = i.wrapping_sub(1)
  }
  return 0i32 as ssize_t;
}
unsafe extern "C" fn bc_num_truncate(mut n: *mut BcNum, mut places: size_t) {
  if places == 0i32 as libc::c_ulong {
    return;
  }
  (*n).rdx = ((*n).rdx as libc::c_ulong).wrapping_sub(places) as size_t as size_t;
  if (*n).len != 0i32 as libc::c_ulong {
    (*n).len = ((*n).len as libc::c_ulong).wrapping_sub(places) as size_t as size_t;
    memmove(
      (*n).num as *mut libc::c_void,
      (*n).num.offset(places as isize) as *const libc::c_void,
      (*n)
        .len
        .wrapping_mul(::std::mem::size_of::<BcDig>() as libc::c_ulong),
    );
  };
}
unsafe extern "C" fn bc_num_extend(mut n: *mut BcNum, mut places: size_t) {
  let mut len: size_t = (*n).len.wrapping_add(places);
  if places != 0i32 as libc::c_ulong {
    if (*n).cap < len {
      bc_num_expand(n, len);
    }
    memmove(
      (*n).num.offset(places as isize) as *mut libc::c_void,
      (*n).num as *const libc::c_void,
      (::std::mem::size_of::<BcDig>() as libc::c_ulong).wrapping_mul((*n).len),
    );
    memset(
      (*n).num as *mut libc::c_void,
      0i32,
      (::std::mem::size_of::<BcDig>() as libc::c_ulong).wrapping_mul(places),
    );
    (*n).len = ((*n).len as libc::c_ulong).wrapping_add(places) as size_t as size_t;
    (*n).rdx = ((*n).rdx as libc::c_ulong).wrapping_add(places) as size_t as size_t
  };
}
unsafe extern "C" fn bc_num_clean(mut n: *mut BcNum) {
  while (*n).len > 0i32 as libc::c_ulong
    && *(*n)
      .num
      .offset((*n).len.wrapping_sub(1i32 as libc::c_ulong) as isize) as libc::c_int
      == 0i32
  {
    (*n).len = (*n).len.wrapping_sub(1)
  }
  if (*n).len == 0i32 as libc::c_ulong {
    (*n).neg = 0i32 != 0
  } else if (*n).len < (*n).rdx {
    (*n).len = (*n).rdx
  };
}
unsafe extern "C" fn bc_num_retireMul(
  mut n: *mut BcNum,
  mut scale: size_t,
  mut neg1: bool,
  mut neg2: bool,
) {
  if (*n).rdx < scale {
    bc_num_extend(n, scale.wrapping_sub((*n).rdx));
  } else {
    bc_num_truncate(n, (*n).rdx.wrapping_sub(scale));
  }
  bc_num_clean(n);
  if (*n).len != 0i32 as libc::c_ulong {
    (*n).neg = !neg1 as libc::c_int != !neg2 as libc::c_int
  };
}
unsafe extern "C" fn bc_num_split(
  mut n: *mut BcNum,
  mut idx: size_t,
  mut a: *mut BcNum,
  mut b: *mut BcNum,
) {
  if idx < (*n).len {
    (*b).len = (*n).len.wrapping_sub(idx);
    (*a).len = idx;
    (*b).rdx = 0i32 as size_t;
    (*a).rdx = (*b).rdx;
    memcpy(
      (*b).num as *mut libc::c_void,
      (*n).num.offset(idx as isize) as *const libc::c_void,
      (*b)
        .len
        .wrapping_mul(::std::mem::size_of::<BcDig>() as libc::c_ulong),
    );
    memcpy(
      (*a).num as *mut libc::c_void,
      (*n).num as *const libc::c_void,
      idx.wrapping_mul(::std::mem::size_of::<BcDig>() as libc::c_ulong),
    );
  } else {
    bc_num_zero(b);
    bc_num_copy(a, n);
  }
  bc_num_clean(a);
  bc_num_clean(b);
}
unsafe extern "C" fn zbc_num_shift(mut n: *mut BcNum, mut places: size_t) -> BcStatus {
  if places == 0i32 as libc::c_ulong || (*n).len == 0i32 as libc::c_ulong {
    return BC_STATUS_SUCCESS;
  }
  // This check makes sense only if size_t is (much) larger than BC_MAX_NUM.
  if 18446744073709551615u64
    > ((2147483647i32 as libc::c_uint)
      .wrapping_mul(2u32)
      .wrapping_add(1u32)
      .wrapping_sub(1i32 as libc::c_uint)
      | 0xffi32 as libc::c_uint) as libc::c_ulong
  {
    if places.wrapping_add((*n).len)
      > (2147483647i32 as libc::c_uint)
        .wrapping_mul(2u32)
        .wrapping_add(1u32)
        .wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong
    {
      return bc_error(
        b"number too long: must be [1,4294967294]\x00" as *const u8 as *const libc::c_char,
      ) as BcStatus;
    }
  }
  if (*n).rdx >= places {
    (*n).rdx = ((*n).rdx as libc::c_ulong).wrapping_sub(places) as size_t as size_t
  } else {
    bc_num_extend(n, places.wrapping_sub((*n).rdx));
    (*n).rdx = 0i32 as size_t
  }
  bc_num_clean(n);
  return BC_STATUS_SUCCESS;
}
unsafe extern "C" fn zbc_num_binary(
  mut a: *mut BcNum,
  mut b: *mut BcNum,
  mut c: *mut BcNum,
  mut scale: size_t,
  mut op: BcNumBinaryOp,
  mut req: size_t,
) -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut num2: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut ptr_a: *mut BcNum = 0 as *mut BcNum;
  let mut ptr_b: *mut BcNum = 0 as *mut BcNum;
  let mut init: bool = 0i32 != 0;
  if c == a {
    ptr_a = &mut num2;
    memcpy(
      ptr_a as *mut libc::c_void,
      c as *const libc::c_void,
      ::std::mem::size_of::<BcNum>() as libc::c_ulong,
    );
    init = 1i32 != 0
  } else {
    ptr_a = a
  }
  if c == b {
    ptr_b = &mut num2;
    if c != a {
      memcpy(
        ptr_b as *mut libc::c_void,
        c as *const libc::c_void,
        ::std::mem::size_of::<BcNum>() as libc::c_ulong,
      );
      init = 1i32 != 0
    }
  } else {
    ptr_b = b
  }
  if init {
    bc_num_init(c, req);
  } else {
    bc_num_expand(c, req);
  }
  s = BC_STATUS_SUCCESS;
  s = op.expect("non-null function pointer")(ptr_a, ptr_b, c, scale);
  if init {
    bc_num_free(&mut num2 as *mut BcNum as *mut libc::c_void);
  }
  return s;
}
unsafe extern "C" fn zbc_num_add(
  mut a: *mut BcNum,
  mut b: *mut BcNum,
  mut c: *mut BcNum,
  mut _scale: size_t,
) -> BcStatus {
  let mut op: BcNumBinaryOp = if !(*a).neg as libc::c_int == !(*b).neg as libc::c_int {
    Some(
      zbc_num_a
        as unsafe extern "C" fn(_: *mut BcNum, _: *mut BcNum, _: *mut BcNum, _: size_t) -> BcStatus,
    )
  } else {
    Some(
      zbc_num_s
        as unsafe extern "C" fn(_: *mut BcNum, _: *mut BcNum, _: *mut BcNum, _: size_t) -> BcStatus,
    )
  };
  return zbc_num_binary(a, b, c, 0i32 as size_t, op, BC_NUM_AREQ(a, b));
}
unsafe extern "C" fn zbc_num_sub(
  mut a: *mut BcNum,
  mut b: *mut BcNum,
  mut c: *mut BcNum,
  mut _scale: size_t,
) -> BcStatus {
  let mut op: BcNumBinaryOp = if !(*a).neg as libc::c_int == !(*b).neg as libc::c_int {
    Some(
      zbc_num_s
        as unsafe extern "C" fn(_: *mut BcNum, _: *mut BcNum, _: *mut BcNum, _: size_t) -> BcStatus,
    )
  } else {
    Some(
      zbc_num_a
        as unsafe extern "C" fn(_: *mut BcNum, _: *mut BcNum, _: *mut BcNum, _: size_t) -> BcStatus,
    )
  };
  return zbc_num_binary(a, b, c, 1i32 as size_t, op, BC_NUM_AREQ(a, b));
}
unsafe extern "C" fn zbc_num_mul(
  mut a: *mut BcNum,
  mut b: *mut BcNum,
  mut c: *mut BcNum,
  mut scale: size_t,
) -> BcStatus {
  let mut req: size_t = BC_NUM_MREQ(a, b, scale);
  return zbc_num_binary(
    a,
    b,
    c,
    scale,
    Some(
      zbc_num_m
        as unsafe extern "C" fn(_: *mut BcNum, _: *mut BcNum, _: *mut BcNum, _: size_t) -> BcStatus,
    ),
    req,
  );
}
unsafe extern "C" fn zbc_num_div(
  mut a: *mut BcNum,
  mut b: *mut BcNum,
  mut c: *mut BcNum,
  mut scale: size_t,
) -> BcStatus {
  let mut req: size_t = BC_NUM_MREQ(a, b, scale);
  return zbc_num_binary(
    a,
    b,
    c,
    scale,
    Some(
      zbc_num_d
        as unsafe extern "C" fn(_: *mut BcNum, _: *mut BcNum, _: *mut BcNum, _: size_t) -> BcStatus,
    ),
    req,
  );
}
unsafe extern "C" fn zbc_num_mod(
  mut a: *mut BcNum,
  mut b: *mut BcNum,
  mut c: *mut BcNum,
  mut scale: size_t,
) -> BcStatus {
  let mut req: size_t = BC_NUM_MREQ(a, b, scale);
  return zbc_num_binary(
    a,
    b,
    c,
    scale,
    Some(
      zbc_num_rem
        as unsafe extern "C" fn(_: *mut BcNum, _: *mut BcNum, _: *mut BcNum, _: size_t) -> BcStatus,
    ),
    req,
  );
}
unsafe extern "C" fn zbc_num_pow(
  mut a: *mut BcNum,
  mut b: *mut BcNum,
  mut c: *mut BcNum,
  mut scale: size_t,
) -> BcStatus {
  return zbc_num_binary(
    a,
    b,
    c,
    scale,
    Some(
      zbc_num_p
        as unsafe extern "C" fn(_: *mut BcNum, _: *mut BcNum, _: *mut BcNum, _: size_t) -> BcStatus,
    ),
    (*a)
      .len
      .wrapping_mul((*b).len)
      .wrapping_add(1i32 as libc::c_ulong),
  );
}

static mut zxc_program_ops: [BcNumBinaryOp; 6] = [
  Some(
    zbc_num_pow
      as unsafe extern "C" fn(_: *mut BcNum, _: *mut BcNum, _: *mut BcNum, _: size_t) -> BcStatus,
  ),
  Some(
    zbc_num_mul
      as unsafe extern "C" fn(_: *mut BcNum, _: *mut BcNum, _: *mut BcNum, _: size_t) -> BcStatus,
  ),
  Some(
    zbc_num_div
      as unsafe extern "C" fn(_: *mut BcNum, _: *mut BcNum, _: *mut BcNum, _: size_t) -> BcStatus,
  ),
  Some(
    zbc_num_mod
      as unsafe extern "C" fn(_: *mut BcNum, _: *mut BcNum, _: *mut BcNum, _: size_t) -> BcStatus,
  ),
  Some(
    zbc_num_add
      as unsafe extern "C" fn(_: *mut BcNum, _: *mut BcNum, _: *mut BcNum, _: size_t) -> BcStatus,
  ),
  Some(
    zbc_num_sub
      as unsafe extern "C" fn(_: *mut BcNum, _: *mut BcNum, _: *mut BcNum, _: size_t) -> BcStatus,
  ),
];

unsafe extern "C" fn zbc_num_inv(
  mut a: *mut BcNum,
  mut b: *mut BcNum,
  mut scale: size_t,
) -> BcStatus {
  let mut one: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut num: [BcDig; 2] = [0; 2];
  one.cap = 2i32 as size_t;
  one.num = num.as_mut_ptr();
  bc_num_one(&mut one);
  return zbc_num_div(&mut one, a, b, scale);
}
unsafe extern "C" fn zbc_num_a(
  mut a: *mut BcNum,
  mut b: *mut BcNum,
  mut c: *mut BcNum,
  mut sub: size_t,
) -> BcStatus {
  let mut ptr: *mut BcDig = 0 as *mut BcDig;
  let mut ptr_a: *mut BcDig = 0 as *mut BcDig;
  let mut ptr_b: *mut BcDig = 0 as *mut BcDig;
  let mut ptr_c: *mut BcDig = 0 as *mut BcDig;
  let mut i: size_t = 0;
  let mut max: size_t = 0;
  let mut min_rdx: size_t = 0;
  let mut min_int: size_t = 0;
  let mut diff: size_t = 0;
  let mut a_int: size_t = 0;
  let mut b_int: size_t = 0;
  let mut carry: libc::c_uint = 0;
  // Because this function doesn't need to use scale (per the bc spec),
  // I am hijacking it to say whether it's doing an add or a subtract.
  if (*a).len == 0i32 as libc::c_ulong {
    bc_num_copy(c, b);
    if sub != 0 && (*c).len != 0 {
      (*c).neg = !(*c).neg
    }
    return BC_STATUS_SUCCESS;
  }
  if (*b).len == 0i32 as libc::c_ulong {
    bc_num_copy(c, a);
    return BC_STATUS_SUCCESS;
  }
  (*c).neg = (*a).neg;
  (*c).rdx = if (*a).rdx > (*b).rdx {
    (*a).rdx
  } else {
    (*b).rdx
  };
  min_rdx = if (*a).rdx < (*b).rdx {
    (*a).rdx
  } else {
    (*b).rdx
  };
  (*c).len = 0i32 as size_t;
  if (*a).rdx > (*b).rdx {
    diff = (*a).rdx.wrapping_sub((*b).rdx);
    ptr = (*a).num;
    ptr_a = (*a).num.offset(diff as isize);
    ptr_b = (*b).num
  } else {
    diff = (*b).rdx.wrapping_sub((*a).rdx);
    ptr = (*b).num;
    ptr_a = (*a).num;
    ptr_b = (*b).num.offset(diff as isize)
  }
  ptr_c = (*c).num;
  i = 0i32 as size_t;
  while i < diff {
    *ptr_c.offset(i as isize) = *ptr.offset(i as isize);
    i = i.wrapping_add(1);
    (*c).len = (*c).len.wrapping_add(1)
  }
  ptr_c = ptr_c.offset(diff as isize);
  a_int = (*a).len.wrapping_sub((*a).rdx);
  b_int = (*b).len.wrapping_sub((*b).rdx);
  if a_int > b_int {
    min_int = b_int;
    max = a_int;
    ptr = ptr_a
  } else {
    min_int = a_int;
    max = b_int;
    ptr = ptr_b
  }
  carry = 0i32 as libc::c_uint;
  i = 0i32 as size_t;
  while i < min_rdx.wrapping_add(min_int) {
    let mut in_0: libc::c_uint = (*ptr_a.offset(i as isize) as libc::c_uint)
      .wrapping_add(*ptr_b.offset(i as isize) as libc::c_uint)
      .wrapping_add(carry);
    carry = in_0.wrapping_div(10i32 as libc::c_uint);
    *ptr_c.offset(i as isize) = in_0.wrapping_rem(10i32 as libc::c_uint) as BcDig;
    i = i.wrapping_add(1)
  }
  while i < max.wrapping_add(min_rdx) {
    let mut in_1: libc::c_uint = (*ptr.offset(i as isize) as libc::c_uint).wrapping_add(carry);
    carry = in_1.wrapping_div(10i32 as libc::c_uint);
    *ptr_c.offset(i as isize) = in_1.wrapping_rem(10i32 as libc::c_uint) as BcDig;
    i = i.wrapping_add(1)
  }
  (*c).len = ((*c).len as libc::c_ulong).wrapping_add(i) as size_t as size_t;
  if carry != 0i32 as libc::c_uint {
    let fresh6 = (*c).len;
    (*c).len = (*c).len.wrapping_add(1);
    *(*c).num.offset(fresh6 as isize) = carry as BcDig
  }
  return BC_STATUS_SUCCESS;
  // can't make void, see zbc_num_binary()
}
unsafe extern "C" fn zbc_num_s(
  mut a: *mut BcNum,
  mut b: *mut BcNum,
  mut c: *mut BcNum,
  mut sub: size_t,
) -> BcStatus {
  let mut cmp: ssize_t = 0;
  let mut minuend: *mut BcNum = 0 as *mut BcNum;
  let mut subtrahend: *mut BcNum = 0 as *mut BcNum;
  let mut start: size_t = 0;
  let mut aneg: bool = false;
  let mut bneg: bool = false;
  let mut neg: bool = false;
  // Because this function doesn't need to use scale (per the bc spec),
  // I am hijacking it to say whether it's doing an add or a subtract.
  if (*a).len == 0i32 as libc::c_ulong {
    bc_num_copy(c, b);
    if sub != 0 && (*c).len != 0 {
      (*c).neg = !(*c).neg
    }
    return BC_STATUS_SUCCESS;
  }
  if (*b).len == 0i32 as libc::c_ulong {
    bc_num_copy(c, a);
    return BC_STATUS_SUCCESS;
  }
  aneg = (*a).neg;
  bneg = (*b).neg;
  (*b).neg = 0i32 != 0;
  (*a).neg = (*b).neg;
  cmp = bc_num_cmp(a, b);
  (*a).neg = aneg;
  (*b).neg = bneg;
  if cmp == 0i32 as libc::c_long {
    bc_num_setToZero(
      c,
      if (*a).rdx > (*b).rdx {
        (*a).rdx
      } else {
        (*b).rdx
      },
    );
    return BC_STATUS_SUCCESS;
  }
  if cmp > 0i32 as libc::c_long {
    neg = (*a).neg;
    minuend = a;
    subtrahend = b
  } else {
    neg = (*b).neg;
    if sub != 0 {
      neg = !neg
    }
    minuend = b;
    subtrahend = a
  }
  bc_num_copy(c, minuend);
  (*c).neg = neg;
  if (*c).rdx < (*subtrahend).rdx {
    bc_num_extend(c, (*subtrahend).rdx.wrapping_sub((*c).rdx));
    start = 0i32 as size_t
  } else {
    start = (*c).rdx.wrapping_sub((*subtrahend).rdx)
  }
  bc_num_subArrays(
    (*c).num.offset(start as isize),
    (*subtrahend).num,
    (*subtrahend).len,
  );
  bc_num_clean(c);
  return BC_STATUS_SUCCESS;
  // can't make void, see zbc_num_binary()
}
unsafe extern "C" fn zbc_num_k(
  mut a: *mut BcNum,
  mut b: *mut BcNum,
  mut c: *mut BcNum,
) -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut max: size_t = if (*a).len > (*b).len {
    (*a).len
  } else {
    (*b).len
  };
  let mut max2: size_t = max
    .wrapping_add(1i32 as libc::c_ulong)
    .wrapping_div(2i32 as libc::c_ulong);
  let mut l1: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut h1: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut l2: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut h2: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut m2: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut m1: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut z0: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut z1: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut z2: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut temp: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut aone: bool = false;
  if (*a).len == 0i32 as libc::c_ulong || (*b).len == 0i32 as libc::c_ulong {
    bc_num_zero(c);
    return BC_STATUS_SUCCESS;
  }
  aone = (*a).len == 1i32 as libc::c_ulong
    && (*a).rdx == 0i32 as libc::c_ulong
    && *(*a).num.offset(0) as libc::c_int == 1i32;
  if aone as libc::c_int != 0
    || (*b).len == 1i32 as libc::c_ulong
      && (*b).rdx == 0i32 as libc::c_ulong
      && *(*b).num.offset(0) as libc::c_int == 1i32
  {
    bc_num_copy(c, if aone as libc::c_int != 0 { b } else { a });
    return BC_STATUS_SUCCESS;
  }
  if (*a).len.wrapping_add((*b).len) < 32i32 as libc::c_ulong
    || (*a).len < 32i32 as libc::c_ulong
    || (*b).len < 32i32 as libc::c_ulong
  {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut len: size_t = 0;
    bc_num_expand(
      c,
      (*a)
        .len
        .wrapping_add((*b).len)
        .wrapping_add(1i32 as libc::c_ulong),
    );
    memset(
      (*c).num as *mut libc::c_void,
      0i32,
      (::std::mem::size_of::<BcDig>() as libc::c_ulong).wrapping_mul((*c).cap),
    );
    len = 0i32 as size_t;
    (*c).len = len;
    i = 0i32 as size_t;
    while i < (*b).len {
      let mut carry: libc::c_uint = 0i32 as libc::c_uint;
      j = 0i32 as size_t;
      while j < (*a).len {
        let mut in_0: libc::c_uint = *(*c).num.offset(i.wrapping_add(j) as isize) as libc::c_uint;
        in_0 = in_0.wrapping_add(
          (*(*a).num.offset(j as isize) as libc::c_uint)
            .wrapping_mul(*(*b).num.offset(i as isize) as libc::c_uint)
            .wrapping_add(carry),
        );
        // note: compilers prefer _unsigned_ div/const
        carry = in_0.wrapping_div(10i32 as libc::c_uint);
        *(*c).num.offset(i.wrapping_add(j) as isize) =
          in_0.wrapping_rem(10i32 as libc::c_uint) as BcDig;
        j = j.wrapping_add(1)
      }
      let ref mut fresh7 = *(*c).num.offset(i.wrapping_add(j) as isize);
      *fresh7 = (*fresh7 as libc::c_int + carry as BcDig as libc::c_int) as BcDig;
      len = if len
        > i
          .wrapping_add(j)
          .wrapping_add((carry != 0) as libc::c_int as libc::c_ulong)
      {
        len
      } else {
        i.wrapping_add(j)
          .wrapping_add((carry != 0) as libc::c_int as libc::c_ulong)
      };
      // a=2^1000000
      // a*a <- without check below, this will not be interruptible
      if bb_got_signal != 0 {
        return BC_STATUS_FAILURE;
      }
      i = i.wrapping_add(1)
    }
    (*c).len = len;
    return BC_STATUS_SUCCESS;
  }
  bc_num_init(&mut l1, max);
  bc_num_init(&mut h1, max);
  bc_num_init(&mut l2, max);
  bc_num_init(&mut h2, max);
  bc_num_init(&mut m1, max);
  bc_num_init(&mut m2, max);
  bc_num_init(&mut z0, max);
  bc_num_init(&mut z1, max);
  bc_num_init(&mut z2, max);
  bc_num_init(&mut temp, max.wrapping_add(max));
  bc_num_split(a, max2, &mut l1, &mut h1);
  bc_num_split(b, max2, &mut l2, &mut h2);
  s = zbc_num_add(&mut h1, &mut l1, &mut m1, 0i32 as size_t);
  if !(s as u64 != 0) {
    s = zbc_num_add(&mut h2, &mut l2, &mut m2, 0i32 as size_t);
    if !(s as u64 != 0) {
      s = zbc_num_k(&mut h1, &mut h2, &mut z0);
      if !(s as u64 != 0) {
        s = zbc_num_k(&mut m1, &mut m2, &mut z1);
        if !(s as u64 != 0) {
          s = zbc_num_k(&mut l1, &mut l2, &mut z2);
          if !(s as u64 != 0) {
            s = zbc_num_sub(&mut z1, &mut z0, &mut temp, 0i32 as size_t);
            if !(s as u64 != 0) {
              s = zbc_num_sub(&mut temp, &mut z2, &mut z1, 0i32 as size_t);
              if !(s as u64 != 0) {
                s = zbc_num_shift(&mut z0, max2.wrapping_mul(2i32 as libc::c_ulong));
                if !(s as u64 != 0) {
                  s = zbc_num_shift(&mut z1, max2);
                  if !(s as u64 != 0) {
                    s = zbc_num_add(&mut z0, &mut z1, &mut temp, 0i32 as size_t);
                    if !(s as u64 != 0) {
                      s = zbc_num_add(&mut temp, &mut z2, c, 0i32 as size_t)
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  bc_num_free(&mut temp as *mut BcNum as *mut libc::c_void);
  bc_num_free(&mut z2 as *mut BcNum as *mut libc::c_void);
  bc_num_free(&mut z1 as *mut BcNum as *mut libc::c_void);
  bc_num_free(&mut z0 as *mut BcNum as *mut libc::c_void);
  bc_num_free(&mut m2 as *mut BcNum as *mut libc::c_void);
  bc_num_free(&mut m1 as *mut BcNum as *mut libc::c_void);
  bc_num_free(&mut h2 as *mut BcNum as *mut libc::c_void);
  bc_num_free(&mut l2 as *mut BcNum as *mut libc::c_void);
  bc_num_free(&mut h1 as *mut BcNum as *mut libc::c_void);
  bc_num_free(&mut l1 as *mut BcNum as *mut libc::c_void);
  return s;
}
unsafe extern "C" fn zbc_num_m(
  mut a: *mut BcNum,
  mut b: *mut BcNum,
  mut c: *mut BcNum,
  mut scale: size_t,
) -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut cpa: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut cpb: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut maxrdx: size_t = if (*a).rdx > (*b).rdx {
    (*a).rdx
  } else {
    (*b).rdx
  };
  scale = if scale > (*a).rdx { scale } else { (*a).rdx };
  scale = if scale > (*b).rdx { scale } else { (*b).rdx };
  scale = if (*a).rdx.wrapping_add((*b).rdx) < scale {
    (*a).rdx.wrapping_add((*b).rdx)
  } else {
    scale
  };
  maxrdx = if maxrdx > scale { maxrdx } else { scale };
  bc_num_init(&mut cpa, (*a).len);
  bc_num_init(&mut cpb, (*b).len);
  bc_num_copy(&mut cpa, a);
  bc_num_copy(&mut cpb, b);
  cpb.neg = 0i32 != 0;
  cpa.neg = cpb.neg;
  s = zbc_num_shift(&mut cpa, maxrdx);
  if !(s as u64 != 0) {
    s = zbc_num_shift(&mut cpb, maxrdx);
    if !(s as u64 != 0) {
      s = zbc_num_k(&mut cpa, &mut cpb, c);
      if !(s as u64 != 0) {
        maxrdx = (maxrdx as libc::c_ulong).wrapping_add(scale) as size_t as size_t;
        bc_num_expand(c, (*c).len.wrapping_add(maxrdx));
        if (*c).len < maxrdx {
          memset(
            (*c).num.offset((*c).len as isize) as *mut libc::c_void,
            0i32,
            (*c)
              .cap
              .wrapping_sub((*c).len)
              .wrapping_mul(::std::mem::size_of::<BcDig>() as libc::c_ulong),
          );
          (*c).len = ((*c).len as libc::c_ulong).wrapping_add(maxrdx) as size_t as size_t
        }
        (*c).rdx = maxrdx;
        bc_num_retireMul(c, scale, (*a).neg, (*b).neg);
      }
    }
  }
  bc_num_free(&mut cpb as *mut BcNum as *mut libc::c_void);
  bc_num_free(&mut cpa as *mut BcNum as *mut libc::c_void);
  return s;
}
unsafe extern "C" fn zbc_num_d(
  mut a: *mut BcNum,
  mut b: *mut BcNum,
  mut c: *mut BcNum,
  mut scale: size_t,
) -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut len: size_t = 0;
  let mut end: size_t = 0;
  let mut i: size_t = 0;
  let mut cp: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  if (*b).len == 0i32 as libc::c_ulong {
    return bc_error(b"divide by zero\x00" as *const u8 as *const libc::c_char) as BcStatus;
  }
  if (*a).len == 0i32 as libc::c_ulong {
    bc_num_setToZero(c, scale);
    return BC_STATUS_SUCCESS;
  }
  if (*b).len == 1i32 as libc::c_ulong
    && (*b).rdx == 0i32 as libc::c_ulong
    && *(*b).num.offset(0) as libc::c_int == 1i32
  {
    bc_num_copy(c, a);
    bc_num_retireMul(c, scale, (*a).neg, (*b).neg);
    return BC_STATUS_SUCCESS;
  }
  bc_num_init(&mut cp, BC_NUM_MREQ(a, b, scale));
  bc_num_copy(&mut cp, a);
  len = (*b).len;
  if len > cp.len {
    bc_num_expand(&mut cp, len.wrapping_add(2i32 as libc::c_ulong));
    bc_num_extend(&mut cp, len.wrapping_sub(cp.len));
  }
  if (*b).rdx > cp.rdx {
    bc_num_extend(&mut cp, (*b).rdx.wrapping_sub(cp.rdx));
  }
  cp.rdx = (cp.rdx as libc::c_ulong).wrapping_sub((*b).rdx) as size_t as size_t;
  if scale > cp.rdx {
    bc_num_extend(&mut cp, scale.wrapping_sub(cp.rdx));
  }
  if (*b).rdx == (*b).len {
    while !(len == 0i32 as libc::c_ulong) {
      len = len.wrapping_sub(1);
      if *(*b).num.offset(len as isize) as libc::c_int != 0i32 {
        break;
      }
    }
    len = len.wrapping_add(1)
  }
  if cp.cap == cp.len {
    bc_num_expand(&mut cp, cp.len.wrapping_add(1i32 as libc::c_ulong));
  }
  // We want an extra zero in front to make things simpler.
  let fresh8 = cp.len;
  cp.len = cp.len.wrapping_add(1);
  *cp.num.offset(fresh8 as isize) = 0i32 as BcDig;
  end = cp.len.wrapping_sub(len);
  bc_num_expand(c, cp.len);
  bc_num_zero(c);
  memset(
    (*c).num.offset(end as isize) as *mut libc::c_void,
    0i32,
    (*c)
      .cap
      .wrapping_sub(end)
      .wrapping_mul(::std::mem::size_of::<BcDig>() as libc::c_ulong),
  );
  (*c).rdx = cp.rdx;
  (*c).len = cp.len;
  s = BC_STATUS_SUCCESS;
  i = end.wrapping_sub(1i32 as libc::c_ulong);
  while i < end {
    let mut n: *mut BcDig = 0 as *mut BcDig;
    let mut q: BcDig = 0;
    n = cp.num.offset(i as isize);
    q = 0i32 as BcDig;
    while *n.offset(len as isize) as libc::c_int != 0i32
      || bc_num_compare(n, (*b).num, len) >= 0i32 as libc::c_long
    {
      bc_num_subArrays(n, (*b).num, len);
      q += 1
    }
    *(*c).num.offset(i as isize) = q;
    // a=2^100000
    // scale=40000
    // 1/a <- without check below, this will not be interruptible
    if bb_got_signal != 0 {
      s = BC_STATUS_FAILURE;
      break;
    } else {
      i = i.wrapping_sub(1)
    }
  }
  bc_num_retireMul(c, scale, (*a).neg, (*b).neg);
  bc_num_free(&mut cp as *mut BcNum as *mut libc::c_void);
  return s;
}
unsafe extern "C" fn zbc_num_r(
  mut a: *mut BcNum,
  mut b: *mut BcNum,
  mut c: *mut BcNum,
  mut d: *mut BcNum,
  mut scale: size_t,
  mut ts: size_t,
) -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut temp: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut neg: bool = false;
  if (*b).len == 0i32 as libc::c_ulong {
    return bc_error(b"divide by zero\x00" as *const u8 as *const libc::c_char) as BcStatus;
  }
  if (*a).len == 0i32 as libc::c_ulong {
    bc_num_setToZero(d, ts);
    return BC_STATUS_SUCCESS;
  }
  bc_num_init(&mut temp, (*d).cap);
  s = zbc_num_d(a, b, c, scale);
  if !(s as u64 != 0) {
    if scale != 0i32 as libc::c_ulong {
      scale = ts
    }
    s = zbc_num_m(c, b, &mut temp, scale);
    if !(s as u64 != 0) {
      s = zbc_num_sub(a, &mut temp, d, scale);
      if !(s as u64 != 0) {
        if ts > (*d).rdx && (*d).len != 0 {
          bc_num_extend(d, ts.wrapping_sub((*d).rdx));
        }
        neg = (*d).neg;
        bc_num_retireMul(d, ts, (*a).neg, (*b).neg);
        (*d).neg = neg
      }
    }
  }
  bc_num_free(&mut temp as *mut BcNum as *mut libc::c_void);
  return s;
}
unsafe extern "C" fn zbc_num_rem(
  mut a: *mut BcNum,
  mut b: *mut BcNum,
  mut c: *mut BcNum,
  mut scale: size_t,
) -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut c1: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut ts: size_t = if scale.wrapping_add((*b).rdx) > (*a).rdx {
    scale.wrapping_add((*b).rdx)
  } else {
    (*a).rdx
  };
  let mut len: size_t = BC_NUM_MREQ(a, b, ts);
  bc_num_init(&mut c1, len);
  s = zbc_num_r(a, b, &mut c1, c, scale, ts);
  bc_num_free(&mut c1 as *mut BcNum as *mut libc::c_void);
  return s;
}
unsafe extern "C" fn zbc_num_p(
  mut a: *mut BcNum,
  mut b: *mut BcNum,
  mut c: *mut BcNum,
  mut scale: size_t,
) -> BcStatus {
  let mut current_block: u64;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut copy: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut pow: libc::c_ulong = 0;
  let mut i: size_t = 0;
  let mut powrdx: size_t = 0;
  let mut resrdx: size_t = 0;
  let mut neg: bool = false;
  // GNU bc does not allow 2^2.0 - we do
  i = 0i32 as size_t;
  while i < (*b).rdx {
    if *(*b).num.offset(i as isize) as libc::c_int != 0i32 {
      return bc_error(b"not an integer\x00" as *const u8 as *const libc::c_char) as BcStatus;
    }
    i = i.wrapping_add(1)
  }
  if (*b).len == 0i32 as libc::c_ulong {
    bc_num_one(c);
    return BC_STATUS_SUCCESS;
  }
  if (*a).len == 0i32 as libc::c_ulong {
    bc_num_setToZero(c, scale);
    return BC_STATUS_SUCCESS;
  }
  if (*b).len == 1i32 as libc::c_ulong
    && (*b).rdx == 0i32 as libc::c_ulong
    && *(*b).num.offset(0) as libc::c_int == 1i32
  {
    if !(*b).neg {
      bc_num_copy(c, a);
    } else {
      s = zbc_num_inv(a, c, scale)
    }
    return s;
  }
  neg = (*b).neg;
  s = zbc_num_ulong_abs(b, &mut pow);
  if s as u64 != 0 {
    return s;
  }
  // b is not used beyond this point
  bc_num_init(&mut copy, (*a).len);
  bc_num_copy(&mut copy, a);
  if !neg {
    if (*a).rdx > scale {
      scale = (*a).rdx
    }
    if (*a).rdx.wrapping_mul(pow) < scale {
      scale = (*a).rdx.wrapping_mul(pow)
    }
  }
  powrdx = (*a).rdx;
  loop {
    if !(pow & 1i32 as libc::c_ulong == 0) {
      current_block = 14434620278749266018;
      break;
    }
    powrdx <<= 1i32;
    s = zbc_num_mul(&mut copy, &mut copy, &mut copy, powrdx);
    if s as u64 != 0 {
      current_block = 177946457580043942;
      break;
    }
    pow >>= 1i32
    // Not needed: zbc_num_mul() has a check for ^C:
    //if (G_interrupt) {
    //	s = BC_STATUS_FAILURE;
    //	goto err;
    //}
  }
  match current_block {
    14434620278749266018 => {
      bc_num_copy(c, &mut copy);
      resrdx = powrdx;
      pow >>= 1i32;
      loop {
        if !(pow != 0i32 as libc::c_ulong) {
          current_block = 15004371738079956865;
          break;
        }
        powrdx <<= 1i32;
        s = zbc_num_mul(&mut copy, &mut copy, &mut copy, powrdx);
        if s as u64 != 0 {
          current_block = 177946457580043942;
          break;
        }
        if pow & 1i32 as libc::c_ulong != 0 {
          resrdx = (resrdx as libc::c_ulong).wrapping_add(powrdx) as size_t as size_t;
          s = zbc_num_mul(c, &mut copy, c, resrdx);
          if s as u64 != 0 {
            current_block = 177946457580043942;
            break;
          }
        }
        pow >>= 1i32
        // Not needed: zbc_num_mul() has a check for ^C:
        //if (G_interrupt) {
        //	s = BC_STATUS_FAILURE;
        //	goto err;
        //}
      }
      match current_block {
        177946457580043942 => {}
        _ => {
          if neg {
            s = zbc_num_inv(c, c, scale);
            if s as u64 != 0 {
              current_block = 177946457580043942;
            } else {
              current_block = 2989495919056355252;
            }
          } else {
            current_block = 2989495919056355252;
          }
          match current_block {
            177946457580043942 => {}
            _ => {
              if (*c).rdx > scale {
                bc_num_truncate(c, (*c).rdx.wrapping_sub(scale));
              }
              // We can't use bc_num_clean() here.
              i = 0i32 as size_t;
              loop {
                if !(i < (*c).len) {
                  current_block = 8151474771948790331;
                  break;
                }
                if *(*c).num.offset(i as isize) as libc::c_int != 0i32 {
                  current_block = 177946457580043942;
                  break;
                }
                i = i.wrapping_add(1)
              }
              match current_block {
                177946457580043942 => {}
                _ => {
                  bc_num_setToZero(c, scale);
                }
              }
            }
          }
        }
      }
    }
    _ => {}
  }
  bc_num_free(&mut copy as *mut BcNum as *mut libc::c_void);
  return s;
}
unsafe extern "C" fn zbc_num_sqrt(
  mut a: *mut BcNum,
  mut b: *mut BcNum,
  mut scale: size_t,
) -> BcStatus {
  let mut current_block: u64;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut num1: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut num2: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut half: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut f: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut fprime: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut x0: *mut BcNum = 0 as *mut BcNum;
  let mut x1: *mut BcNum = 0 as *mut BcNum;
  let mut temp: *mut BcNum = 0 as *mut BcNum;
  let mut half_digs: [BcDig; 1] = [0; 1];
  let mut pow: size_t = 0;
  let mut len: size_t = 0;
  let mut digs: size_t = 0;
  let mut digs1: size_t = 0;
  let mut resrdx: size_t = 0;
  let mut req: size_t = 0;
  let mut times: size_t = 0i32 as size_t;
  let mut cmp: ssize_t = 1i32 as ssize_t;
  let mut cmp1: ssize_t = 9223372036854775807i64;
  let mut cmp2: ssize_t = 9223372036854775807i64;
  req = (if scale > (*a).rdx { scale } else { (*a).rdx })
    .wrapping_add(
      (*a)
        .len
        .wrapping_sub((*a).rdx)
        .wrapping_add(1i32 as libc::c_ulong)
        >> 1i32,
    )
    .wrapping_add(1i32 as libc::c_ulong);
  bc_num_expand(b, req);
  if (*a).len == 0i32 as libc::c_ulong {
    bc_num_setToZero(b, scale);
    return BC_STATUS_SUCCESS;
  }
  if (*a).neg {
    return bc_error(b"negative number\x00" as *const u8 as *const libc::c_char) as BcStatus;
  }
  if (*a).len == 1i32 as libc::c_ulong
    && (*a).rdx == 0i32 as libc::c_ulong
    && *(*a).num.offset(0) as libc::c_int == 1i32
  {
    bc_num_one(b);
    bc_num_extend(b, scale);
    return BC_STATUS_SUCCESS;
  }
  scale = (if scale > (*a).rdx { scale } else { (*a).rdx }).wrapping_add(1i32 as libc::c_ulong);
  len = (*a).len.wrapping_add(scale);
  bc_num_init(&mut num1, len);
  bc_num_init(&mut num2, len);
  half.cap = (::std::mem::size_of::<[BcDig; 1]>() as libc::c_ulong)
    .wrapping_div(::std::mem::size_of::<BcDig>() as libc::c_ulong) as libc::c_uint
    as size_t;
  half.num = half_digs.as_mut_ptr();
  bc_num_one(&mut half);
  half_digs[0] = 5i32 as BcDig;
  half.rdx = 1i32 as size_t;
  bc_num_init(&mut f, len);
  bc_num_init(&mut fprime, len);
  x0 = &mut num1;
  x1 = &mut num2;
  bc_num_one(x0);
  pow = (*a).len.wrapping_sub((*a).rdx);
  if pow != 0 {
    if pow & 1i32 as libc::c_ulong != 0 {
      *(*x0).num.offset(0) = 2i32 as BcDig
    } else {
      *(*x0).num.offset(0) = 6i32 as BcDig
    }
    pow = (pow as libc::c_ulong)
      .wrapping_sub((2i32 as libc::c_ulong).wrapping_sub(pow & 1i32 as libc::c_ulong))
      as size_t as size_t;
    bc_num_extend(x0, pow);
    // Make sure to move the radix back.
    (*x0).rdx = ((*x0).rdx as libc::c_ulong).wrapping_sub(pow) as size_t as size_t
  }
  digs1 = 0i32 as size_t;
  digs = digs1;
  (*x0).rdx = digs;
  resrdx = scale.wrapping_add(2i32 as libc::c_ulong);
  len = (*x0)
    .len
    .wrapping_sub((*x0).rdx)
    .wrapping_add(resrdx)
    .wrapping_sub(1i32 as libc::c_ulong);
  loop {
    if !(cmp != 0i32 as libc::c_long || digs < len) {
      current_block = 13826291924415791078;
      break;
    }
    s = zbc_num_div(a, x0, &mut f, resrdx);
    if s as u64 != 0 {
      current_block = 1137939974041476113;
      break;
    }
    s = zbc_num_add(x0, &mut f, &mut fprime, resrdx);
    if s as u64 != 0 {
      current_block = 1137939974041476113;
      break;
    }
    s = zbc_num_mul(&mut fprime, &mut half, x1, resrdx);
    if s as u64 != 0 {
      current_block = 1137939974041476113;
      break;
    }
    cmp = bc_num_cmp(x1, x0);
    digs = ((*x1).len as libc::c_ulonglong)
      .wrapping_sub(llabs(cmp as libc::c_longlong) as libc::c_ulonglong) as size_t;
    if cmp == cmp2 && digs == digs1 {
      times = (times as libc::c_ulong).wrapping_add(1i32 as libc::c_ulong) as size_t as size_t
    } else {
      times = 0i32 as size_t
    }
    resrdx = (resrdx as libc::c_ulong)
      .wrapping_add((times > 4i32 as libc::c_ulong) as libc::c_int as libc::c_ulong)
      as size_t as size_t;
    cmp2 = cmp1;
    cmp1 = cmp;
    digs1 = digs;
    temp = x0;
    x0 = x1;
    x1 = temp
  }
  match current_block {
    13826291924415791078 => {
      bc_num_copy(b, x0);
      scale = (scale as libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong) as size_t as size_t;
      if (*b).rdx > scale {
        bc_num_truncate(b, (*b).rdx.wrapping_sub(scale));
      }
    }
    _ => {}
  }
  bc_num_free(&mut fprime as *mut BcNum as *mut libc::c_void);
  bc_num_free(&mut f as *mut BcNum as *mut libc::c_void);
  bc_num_free(&mut num2 as *mut BcNum as *mut libc::c_void);
  bc_num_free(&mut num1 as *mut BcNum as *mut libc::c_void);
  return s;
}
unsafe extern "C" fn zbc_num_divmod(
  mut a: *mut BcNum,
  mut b: *mut BcNum,
  mut c: *mut BcNum,
  mut d: *mut BcNum,
  mut scale: size_t,
) -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut num2: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut ptr_a: *mut BcNum = 0 as *mut BcNum;
  let mut init: bool = 0i32 != 0;
  let mut ts: size_t = if scale.wrapping_add((*b).rdx) > (*a).rdx {
    scale.wrapping_add((*b).rdx)
  } else {
    (*a).rdx
  };
  let mut len: size_t = BC_NUM_MREQ(a, b, ts);
  if c == a {
    memcpy(
      &mut num2 as *mut BcNum as *mut libc::c_void,
      c as *const libc::c_void,
      ::std::mem::size_of::<BcNum>() as libc::c_ulong,
    );
    ptr_a = &mut num2;
    bc_num_init(c, len);
    init = 1i32 != 0
  } else {
    ptr_a = a;
    bc_num_expand(c, len);
  }
  s = zbc_num_r(ptr_a, b, c, d, scale, ts);
  if init {
    bc_num_free(&mut num2 as *mut BcNum as *mut libc::c_void);
  }
  return s;
}
unsafe extern "C" fn zdc_num_modexp(
  mut a: *mut BcNum,
  mut b: *mut BcNum,
  mut c: *mut BcNum,
  mut d: *mut BcNum,
) -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut base: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut exp: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut two: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut temp: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut two_digs: [BcDig; 1] = [0; 1];
  if (*c).len == 0i32 as libc::c_ulong {
    return bc_error(b"divide by zero\x00" as *const u8 as *const libc::c_char) as BcStatus;
  }
  if (*a).rdx != 0 || (*b).rdx != 0 || (*c).rdx != 0 {
    return bc_error(b"not an integer\x00" as *const u8 as *const libc::c_char) as BcStatus;
  }
  if (*b).neg {
    return bc_error(b"negative number\x00" as *const u8 as *const libc::c_char) as BcStatus;
  }
  bc_num_expand(d, (*c).len);
  bc_num_init(&mut base, (*c).len);
  bc_num_init(&mut exp, (*b).len);
  bc_num_init(&mut temp, (*b).len);
  two.cap = (::std::mem::size_of::<[BcDig; 1]>() as libc::c_ulong)
    .wrapping_div(::std::mem::size_of::<BcDig>() as libc::c_ulong) as libc::c_uint
    as size_t;
  two.num = two_digs.as_mut_ptr();
  bc_num_one(&mut two);
  two_digs[0] = 2i32 as BcDig;
  bc_num_one(d);
  s = zbc_num_rem(a, c, &mut base, 0i32 as size_t);
  if !(s as u64 != 0) {
    bc_num_copy(&mut exp, b);
    while exp.len != 0i32 as libc::c_ulong {
      s = zbc_num_divmod(&mut exp, &mut two, &mut exp, &mut temp, 0i32 as size_t);
      if s as u64 != 0 {
        break;
      }
      if temp.len == 1i32 as libc::c_ulong
        && temp.rdx == 0i32 as libc::c_ulong
        && *temp.num.offset(0) as libc::c_int == 1i32
      {
        s = zbc_num_mul(d, &mut base, &mut temp, 0i32 as size_t);
        if s as u64 != 0 {
          break;
        }
        s = zbc_num_rem(&mut temp, c, d, 0i32 as size_t);
        if s as u64 != 0 {
          break;
        }
      }
      s = zbc_num_mul(&mut base, &mut base, &mut temp, 0i32 as size_t);
      if s as u64 != 0 {
        break;
      }
      s = zbc_num_rem(&mut temp, c, &mut base, 0i32 as size_t);
      if s as u64 != 0 {
        break;
      }
    }
  }
  bc_num_free(&mut temp as *mut BcNum as *mut libc::c_void);
  bc_num_free(&mut exp as *mut BcNum as *mut libc::c_void);
  bc_num_free(&mut base as *mut BcNum as *mut libc::c_void);
  return s;
}
// ENABLE_DC
unsafe extern "C" fn bc_string_free(mut string: *mut libc::c_void) {
  free(*(string as *mut *mut libc::c_char) as *mut libc::c_void);
}
unsafe extern "C" fn bc_func_init(mut f: *mut BcFunc) {
  bc_char_vec_init(&mut (*f).code);
  bc_vec_init(
    &mut (*f).labels,
    ::std::mem::size_of::<size_t>() as libc::c_ulong,
    None,
  );
  bc_vec_init(
    &mut (*f).autos,
    ::std::mem::size_of::<BcId>() as libc::c_ulong,
    Some(bc_id_free as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
  );
  bc_vec_init(
    &mut (*f).strs,
    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    Some(bc_string_free as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
  );
  bc_vec_init(
    &mut (*f).consts,
    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    Some(bc_string_free as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
  );
  (*f).nparams = 0i32 as size_t;
}
unsafe extern "C" fn bc_func_free(mut func: *mut libc::c_void) {
  let mut f: *mut BcFunc = func as *mut BcFunc;
  bc_vec_free(&mut (*f).code as *mut BcVec as *mut libc::c_void);
  bc_vec_free(&mut (*f).labels as *mut BcVec as *mut libc::c_void);
  bc_vec_free(&mut (*f).autos as *mut BcVec as *mut libc::c_void);
  bc_vec_free(&mut (*f).strs as *mut BcVec as *mut libc::c_void);
  bc_vec_free(&mut (*f).consts as *mut BcVec as *mut libc::c_void);
}
unsafe extern "C" fn bc_array_init(mut a: *mut BcVec, mut nums: bool) {
  if nums {
    bc_vec_init(
      a,
      ::std::mem::size_of::<BcNum>() as libc::c_ulong,
      Some(bc_num_free as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
    );
  } else {
    bc_vec_init(
      a,
      ::std::mem::size_of::<BcVec>() as libc::c_ulong,
      Some(bc_vec_free as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
    );
  }
  bc_array_expand(a, 1i32 as size_t);
}
unsafe extern "C" fn bc_array_expand(mut a: *mut BcVec, mut len: size_t) {
  if (*a).dtor == Some(bc_num_free as unsafe extern "C" fn(_: *mut libc::c_void) -> ()) {
    // && a->size == sizeof(BcNum) - always true
    let mut n: BcNum = BcNum {
      num: 0 as *mut BcDig,
      rdx: 0,
      len: 0,
      cap: 0,
      neg: false,
    };
    while len > (*a).len {
      bc_num_init_DEF_SIZE(&mut n);
      bc_vec_push(a, &mut n as *mut BcNum as *const libc::c_void);
    }
  } else {
    let mut v: BcVec = BcVec {
      v: 0 as *mut libc::c_char,
      len: 0,
      cap: 0,
      size: 0,
      dtor: None,
    };
    while len > (*a).len {
      bc_array_init(&mut v, 1i32 != 0);
      bc_vec_push(a, &mut v as *mut BcVec as *const libc::c_void);
    }
  };
}
unsafe extern "C" fn bc_array_copy(mut d: *mut BcVec, mut s: *const BcVec) {
  let mut dnum: *mut BcNum = 0 as *mut BcNum;
  let mut snum: *mut BcNum = 0 as *mut BcNum;
  let mut i: size_t = 0;
  bc_vec_pop_all(d);
  bc_vec_expand(d, (*s).cap);
  (*d).len = (*s).len;
  dnum = (*d).v as *mut libc::c_void as *mut BcNum;
  snum = (*s).v as *mut libc::c_void as *mut BcNum;
  i = 0i32 as size_t;
  while i < (*s).len {
    bc_num_init(dnum, (*snum).len);
    bc_num_copy(dnum, snum);
    i = i.wrapping_add(1);
    dnum = dnum.offset(1);
    snum = snum.offset(1)
  }
}
unsafe extern "C" fn dc_result_copy(mut d: *mut BcResult, mut src: *mut BcResult) {
  (*d).t = (*src).t;
  match (*d).t as libc::c_uint {
    0 | 6 | 8 | 7 => {
      bc_num_init(&mut (*d).d.n, (*src).d.n.len);
      bc_num_copy(&mut (*d).d.n, &mut (*src).d.n);
    }
    2 | 4 | 3 => (*d).d.id.name = xstrdup((*src).d.id.name),
    10 | 5 => {
      memcpy(
        &mut (*d).d.n as *mut BcNum as *mut libc::c_void,
        &mut (*src).d.n as *mut BcNum as *const libc::c_void,
        ::std::mem::size_of::<BcNum>() as libc::c_ulong,
      );
    }
    _ => {}
  };
}
// ENABLE_DC
unsafe extern "C" fn bc_result_free(mut result: *mut libc::c_void) {
  let mut r: *mut BcResult = result as *mut BcResult;
  match (*r).t as libc::c_uint {
    0 | 1 | 6 | 8 | 7 => {
      bc_num_free(&mut (*r).d.n as *mut BcNum as *mut libc::c_void);
    }
    2 | 4 | 3 => {
      free((*r).d.id.name as *mut libc::c_void);
    }
    _ => {}
  };
}
unsafe extern "C" fn bad_input_byte(mut c: libc::c_char) -> libc::c_int {
  if (c as libc::c_int) < ' ' as i32
    && c as libc::c_int != '\t' as i32
    && c as libc::c_int != '\r' as i32
    && c as libc::c_int != '\n' as i32
    || c as libc::c_int > 0x7ei32
  {
    bc_error_fmt(
      b"illegal character 0x%02x\x00" as *const u8 as *const libc::c_char,
      c as libc::c_int,
    );
    return 1i32;
  }
  return 0i32;
}
unsafe extern "C" fn xc_read_line(mut vec: *mut BcVec, mut fp: *mut FILE) {
  let mut c_0: libc::c_int = 0;
  let mut bad_chars: bool = false;
  let mut n: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  let mut current_block: u64;
  'c_18400: loop
  // stdin
  // ignore entire line, get another one
  {
    bc_vec_pop_all(vec);
    fflush_and_check();
    if bb_got_signal != 0 {
      // ^C was pressed
      current_block = 7042277567724652491;
    } else {
      current_block = 6937071982253665452;
    }
    loop {
      match current_block {
        7042277567724652491 =>
        // ^C
        {
          if fp != stdin {
            // ^C while running a script (bc SCRIPT): die.
            // We do not return to interactive prompt:
            // user might be running us from a shell,
            // and SCRIPT might be intended to terminate
            // (e.g. contain a "halt" stmt).
            // ^C dropping user into a bc prompt instead of
            // the shell would be unexpected.
            xfunc_die();
          }
          // ^C while interactive input
          bb_got_signal = 0i32 as smallint;
          // GNU bc says "interrupted execution."
          // GNU dc says "Interrupt!"
          fputs_unlocked(
            b"\ninterrupted execution\n\x00" as *const u8 as *const libc::c_char,
            stderr,
          );
          current_block = 6937071982253665452;
        }
        _ => {
          if (*ptr_to_globals).ttyin as libc::c_int != 0 && fp == stdin {
            n = 0;
            i = 0;
            n = read_line_input(
              (*ptr_to_globals).line_input_state,
              b"\x00" as *const u8 as *const libc::c_char,
              bb_common_bufsiz1.as_mut_ptr(),
              COMMON_BUFSIZE as libc::c_int,
            );
            if n <= 0i32 {
              // read errors or EOF, or ^D, or ^C
              if n == 0i32 {
                current_block = 7042277567724652491; // ^D or EOF (or error)
                continue;
              }
              bc_vec_pushZeroByte(vec);
              return;
            } else {
              i = 0i32;
              current_block = 12039483399334584727;
              break;
            }
          } else {
            c_0 = 0;
            bad_chars = 0i32 != 0;
            current_block = 3595087818270943215;
            break;
          }
        }
      }
    }
    loop {
      match current_block {
        12039483399334584727 => {
          let fresh9 = i;
          i = i + 1;
          let mut c: libc::c_char = *bb_common_bufsiz1.as_mut_ptr().offset(fresh9 as isize);
          if c as libc::c_int == '\u{0}' as i32 {
            bc_vec_string(vec, n as size_t, bb_common_bufsiz1.as_mut_ptr());
            break 'c_18400;
          } else if bad_input_byte(c) != 0 {
            continue 'c_18400;
          } else {
            current_block = 12039483399334584727;
          }
        }
        _ => {
          if bb_got_signal != 0 {
            // ^C was pressed: ignore entire line, get another one
            continue 'c_18400;
          } else {
            c_0 = getc_unlocked(fp);
            if c_0 == '\u{0}' as i32 {
              current_block = 3595087818270943215;
              continue;
            }
            if c_0 == -1i32 {
              if ferror_unlocked(fp) != 0 {
                bb_simple_perror_msg_and_die(
                  b"input error\x00" as *const u8 as *const libc::c_char,
                );
              }
              break;
            } else {
              bad_chars = (bad_chars as libc::c_int | bad_input_byte(c_0 as libc::c_char)) != 0;
              bc_vec_pushByte(vec, c_0 as libc::c_char);
              if c_0 != '\n' as i32 {
                current_block = 3595087818270943215;
              } else {
                break;
              }
            }
          }
        }
      }
    }
    // Note: EOF does not append '\n'
    if bad_chars {
      // Bad chars on this line
      if (*ptr_to_globals).prs.lex_filename.is_null() {
        continue;
      }
      bb_perror_msg_and_die(
        b"file \'%s\' is not text\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).prs.lex_filename,
      );
    } else {
      bc_vec_pushZeroByte(vec);
      break;
    }
  }
}
//
// Parsing routines
//
// "Input numbers may contain the characters 0-9 and A-Z.
// (Note: They must be capitals.  Lower case letters are variable names.)
// Single digit numbers always have the value of the digit regardless of
// the value of ibase. (i.e. A = 10.) For multi-digit numbers, bc changes
// all input digits greater or equal to ibase to the value of ibase-1.
// This makes the number ZZZ always be the largest 3 digit number of the
// input base."
unsafe extern "C" fn xc_num_strValid(mut val: *const libc::c_char) -> bool {
  let mut radix: bool = 0i32 != 0;
  loop {
    let fresh10 = val;
    val = val.offset(1);
    let mut c: BcDig = *fresh10 as BcDig;
    if c as libc::c_int == '\u{0}' as i32 {
      break;
    }
    if c as libc::c_int == '.' as i32 {
      if radix {
        return 0i32 != 0;
      }
      radix = 1i32 != 0
    } else if ((c as libc::c_int) < '0' as i32 || c as libc::c_int > '9' as i32)
      && ((c as libc::c_int) < 'A' as i32 || c as libc::c_int > 'Z' as i32)
    {
      return 0i32 != 0;
    }
  }
  return 1i32 != 0;
}
// Note: n is already "bc_num_zero()"ed,
// leading zeroes in "val" are removed
unsafe extern "C" fn bc_num_parseDecimal(mut n: *mut BcNum, mut val: *const libc::c_char) {
  let mut len: size_t = 0; // +1 for e.g. "A" converting into 10
  let mut i: size_t = 0;
  let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
  len = strlen(val);
  if len == 0i32 as libc::c_ulong {
    return;
  }
  bc_num_expand(n, len.wrapping_add(1i32 as libc::c_ulong));
  ptr = strchr(val, '.' as i32);
  (*n).rdx = 0i32 as size_t;
  if !ptr.is_null() {
    (*n).rdx =
      val.offset(len as isize).wrapping_offset_from(ptr.offset(1)) as libc::c_long as size_t
  }
  i = 0i32 as size_t;
  while *val.offset(i as isize) != 0 {
    if *val.offset(i as isize) as libc::c_int != '0' as i32
      && *val.offset(i as isize) as libc::c_int != '.' as i32
    {
      // Not entirely zero value - convert it, and exit
      if len == 1i32 as libc::c_ulong {
        let mut c: libc::c_uint = (*val.offset(0) as libc::c_int - '0' as i32) as libc::c_uint;
        (*n).len = 1i32 as size_t;
        if c > 9i32 as libc::c_uint {
          // A-Z => 10-36
          (*n).len = 2i32 as size_t;
          c = c.wrapping_sub(('A' as i32 - '9' as i32 - 1i32) as libc::c_uint);
          *(*n).num.offset(1) = c.wrapping_div(10i32 as libc::c_uint) as BcDig;
          c = c.wrapping_rem(10i32 as libc::c_uint)
        }
        *(*n).num.offset(0) = c as BcDig;
        break;
      } else {
        i = len.wrapping_sub(1i32 as libc::c_ulong);
        's_110: loop {
          let mut c_0: libc::c_char =
            (*val.offset(i as isize) as libc::c_int - '0' as i32) as libc::c_char;
          if c_0 as libc::c_int > 9i32 {
            // A-Z => 9
            c_0 = 9i32 as libc::c_char
          }
          *(*n).num.offset((*n).len as isize) = c_0 as BcDig;
          (*n).len = (*n).len.wrapping_add(1);
          loop {
            if i == 0i32 as libc::c_ulong {
              break 's_110;
            }
            i = i.wrapping_sub(1);
            if !(*val.offset(i as isize) as libc::c_int == '.' as i32) {
              break;
            }
          }
        }
        break;
      }
    } else {
      i = i.wrapping_add(1)
    }
  }
  // if for() exits without hitting if(), the value is entirely zero
}
// Note: n is already "bc_num_zero()"ed,
// leading zeroes in "val" are removed
unsafe extern "C" fn bc_num_parseBase(
  mut n: *mut BcNum,
  mut val: *const libc::c_char,
  mut base_t: libc::c_uint,
) {
  let mut current_block: u64;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut mult: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut result: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut temp: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut base: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut temp_digs: [BcDig; 20] = [0; 20];
  let mut base_digs: [BcDig; 20] = [0; 20];
  let mut digits: size_t = 0;
  bc_num_init_DEF_SIZE(&mut mult);
  temp.cap = (::std::mem::size_of::<[BcDig; 20]>() as libc::c_ulong)
    .wrapping_div(::std::mem::size_of::<BcDig>() as libc::c_ulong) as libc::c_uint
    as size_t;
  temp.num = temp_digs.as_mut_ptr();
  base.cap = (::std::mem::size_of::<[BcDig; 20]>() as libc::c_ulong)
    .wrapping_div(::std::mem::size_of::<BcDig>() as libc::c_ulong) as libc::c_uint
    as size_t;
  base.num = base_digs.as_mut_ptr();
  bc_num_ulong2num(&mut base, base_t as libc::c_ulong);
  base_t = base_t.wrapping_sub(1);
  loop {
    let mut v: libc::c_uint = 0;
    let mut c: libc::c_char = 0;
    let fresh11 = val;
    val = val.offset(1);
    c = *fresh11;
    if c as libc::c_int == '\u{0}' as i32 {
      current_block = 6365745064265978751;
      break;
    }
    if c as libc::c_int == '.' as i32 {
      current_block = 11298138898191919651;
      break;
    }
    v = if c as libc::c_int <= '9' as i32 {
      (c as libc::c_int) - '0' as i32
    } else {
      (c as libc::c_int - 'A' as i32) + 10i32
    } as libc::c_uint;
    if v > base_t {
      v = base_t
    }
    s = zbc_num_mul(n, &mut base, &mut mult, 0i32 as size_t);
    if s as u64 != 0 {
      current_block = 6365745064265978751;
      break;
    }
    bc_num_ulong2num(&mut temp, v as libc::c_ulong);
    s = zbc_num_add(&mut mult, &mut temp, n, 0i32 as size_t);
    if s as u64 != 0 {
      current_block = 6365745064265978751;
      break;
    }
  }
  match current_block {
    11298138898191919651 => {
      bc_num_init(&mut result, base.len);
      //bc_num_zero(&result); - already is
      bc_num_one(&mut mult);
      digits = 0i32 as size_t;
      loop {
        let mut v_0: libc::c_uint = 0;
        let mut c_0: libc::c_char = 0;
        let fresh12 = val;
        val = val.offset(1);
        c_0 = *fresh12;
        if c_0 as libc::c_int == '\u{0}' as i32 {
          current_block = 18435049525520518667;
          break;
        }
        digits = digits.wrapping_add(1);
        v_0 = if c_0 as libc::c_int <= '9' as i32 {
          (c_0 as libc::c_int) - '0' as i32
        } else {
          (c_0 as libc::c_int - 'A' as i32) + 10i32
        } as libc::c_uint;
        if v_0 > base_t {
          v_0 = base_t
        }
        s = zbc_num_mul(&mut result, &mut base, &mut result, 0i32 as size_t);
        if s as u64 != 0 {
          current_block = 2795181157028703675;
          break;
        }
        bc_num_ulong2num(&mut temp, v_0 as libc::c_ulong);
        s = zbc_num_add(&mut result, &mut temp, &mut result, 0i32 as size_t);
        if s as u64 != 0 {
          current_block = 2795181157028703675;
          break;
        }
        s = zbc_num_mul(&mut mult, &mut base, &mut mult, 0i32 as size_t);
        if s as u64 != 0 {
          current_block = 2795181157028703675;
          break;
        }
      }
      match current_block {
        18435049525520518667 => {
          s = zbc_num_div(&mut result, &mut mult, &mut result, digits);
          if !(s as u64 != 0) {
            s = zbc_num_add(n, &mut result, n, digits);
            if !(s as u64 != 0) {
              if (*n).len != 0i32 as libc::c_ulong {
                if (*n).rdx < digits {
                  bc_num_extend(n, digits.wrapping_sub((*n).rdx));
                }
              } else {
                bc_num_zero(n);
              }
            }
          }
        }
        _ => {}
      }
      bc_num_free(&mut result as *mut BcNum as *mut libc::c_void);
    }
    _ => {}
  }
  bc_num_free(&mut mult as *mut BcNum as *mut libc::c_void);
}
unsafe extern "C" fn zxc_num_parse(
  mut n: *mut BcNum,
  mut val: *const libc::c_char,
  mut base_t: libc::c_uint,
) -> BcStatus {
  let mut i: size_t = 0;
  if !xc_num_strValid(val) {
    return bc_error(b"bad number string\x00" as *const u8 as *const libc::c_char) as BcStatus;
  }
  bc_num_zero(n);
  while *val as libc::c_int == '0' as i32 {
    val = val.offset(1)
  }
  i = 0i32 as size_t;
  loop {
    if *val.offset(i as isize) as libc::c_int == '\u{0}' as i32 {
      return BC_STATUS_SUCCESS;
    }
    if *val.offset(i as isize) as libc::c_int != '.' as i32
      && *val.offset(i as isize) as libc::c_int != '0' as i32
    {
      break;
    }
    i = i.wrapping_add(1)
  }
  if base_t == 10i32 as libc::c_uint || *val.offset(1) as libc::c_int == '\u{0}' as i32 {
    // Decimal, or single-digit number
    bc_num_parseDecimal(n, val);
  } else {
    bc_num_parseBase(n, val, base_t);
  }
  return BC_STATUS_SUCCESS;
}
// p->lex_inbuf points to the current string to be parsed.
// if p->lex_inbuf points to '\0', it's either EOF or it points after
// last processed line's terminating '\n' (and more reading needs to be done
// to get next character).
//
// If you are in a situation where that is a possibility, call peek_inbuf().
// If necessary, it performs more reading and changes p->lex_inbuf,
// then it returns *p->lex_inbuf (which will be '\0' only if it's EOF).
// After it, just referencing *p->lex_inbuf is valid, and if it wasn't '\0',
// it's ok to do p->lex_inbuf++ once without end-of-buffer checking.
//
// eat_inbuf() is equvalent to "peek_inbuf(); if (c) p->lex_inbuf++":
// it returns current char and advances the pointer (if not EOF).
// After eat_inbuf(), referencing p->lex_inbuf[-1] and *p->lex_inbuf is valid.
//
// In many cases, you can use fast *p->lex_inbuf instead of peek_inbuf():
// unless prev char might have been '\n', *p->lex_inbuf is '\0' ONLY
// on real EOF, not end-of-buffer.
//
// bc cases to test interactively:
// 1 #comment\  - prints "1<newline>" at once (comment is not continued)
// 1 #comment/* - prints "1<newline>" at once
// 1 #comment"  - prints "1<newline>" at once
// 1\#comment   - error at once (\ is not a line continuation)
// 1 + /*"*/2   - prints "3<newline>" at once
// 1 + /*#*/2   - prints "3<newline>" at once
// "str\"       - prints "str\" at once
// "str#"       - prints "str#" at once
// "str/*"      - prints "str/*" at once
// "str#\       - waits for second line
// end"         - ...prints "str#\<newline>end"
unsafe extern "C" fn peek_inbuf() -> libc::c_char {
  if *(*ptr_to_globals).prs.lex_inbuf as libc::c_int == '\u{0}' as i32
    && !(*ptr_to_globals).prs.lex_input_fp.is_null()
  {
    xc_read_line(
      &mut (*ptr_to_globals).input_buffer,
      (*ptr_to_globals).prs.lex_input_fp,
    );
    (*ptr_to_globals).prs.lex_inbuf = (*ptr_to_globals).input_buffer.v;
    if (*ptr_to_globals).input_buffer.len <= 1i32 as libc::c_ulong {
      // on EOF, len is 1 (NUL byte)
      (*ptr_to_globals).prs.lex_input_fp = 0 as *mut FILE
    }
  }
  return *(*ptr_to_globals).prs.lex_inbuf;
}
unsafe extern "C" fn eat_inbuf() -> libc::c_char {
  let mut c: libc::c_char = peek_inbuf();
  if c != 0 {
    (*ptr_to_globals).prs.lex_inbuf = (*ptr_to_globals).prs.lex_inbuf.offset(1)
  }
  return c;
}
unsafe extern "C" fn xc_lex_lineComment() {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut c: libc::c_char = 0;
  // Try: echo -n '#foo' | bc
  (*p).lex = XC_LEX_WHITESPACE as libc::c_int as smallint;
  loop
  // Not peek_inbuf(): we depend on input being done in whole lines:
  // '\0' which isn't the EOF can only be seen after '\n'.
  {
    c = *(*p).lex_inbuf;
    if !(c as libc::c_int != '\n' as i32 && c as libc::c_int != '\u{0}' as i32) {
      break;
    }
    (*p).lex_inbuf = (*p).lex_inbuf.offset(1)
  }
}
unsafe extern "C" fn xc_lex_whitespace() {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  (*p).lex = XC_LEX_WHITESPACE as libc::c_int as smallint;
  loop
  // We depend here on input being done in whole lines:
  // '\0' which isn't the EOF can only be seen after '\n'.
  {
    let mut c: libc::c_char = *(*p).lex_inbuf;
    if c as libc::c_int == '\n' as i32 {
      break;
    }
    if ({
      let mut bb__isspace: libc::c_uchar = (c as libc::c_int - 9i32) as libc::c_uchar;
      (bb__isspace as libc::c_int == ' ' as i32 - 9i32
        || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
    }) == 0
    {
      break;
    }
    (*p).lex_inbuf = (*p).lex_inbuf.offset(1)
  }
}
unsafe extern "C" fn zxc_lex_number(mut last: libc::c_char) -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut pt: bool = false;
  let mut last_valid_ch: libc::c_char = 0;
  bc_vec_pop_all(&mut (*p).lex_strnumbuf);
  bc_vec_pushByte(&mut (*p).lex_strnumbuf, last);
  // bc: "Input numbers may contain the characters 0-9 and A-Z.
  // (Note: They must be capitals.  Lower case letters are variable names.)
  // Single digit numbers always have the value of the digit regardless of
  // the value of ibase. (i.e. A = 10.) For multi-digit numbers, bc changes
  // all input digits greater or equal to ibase to the value of ibase-1.
  // This makes the number ZZZ always be the largest 3 digit number of the
  // input base."
  // dc only allows A-F, the rules about single-char and multi-char are the same.
  last_valid_ch = if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(0) as libc::c_int == 'b' as i32)
  {
    'Z' as i32
  } else {
    'F' as i32
  } as libc::c_char;
  pt = last as libc::c_int == '.' as i32;
  (*p).lex = XC_LEX_NUMBER as libc::c_int as smallint;
  's_36: loop
  // We depend here on input being done in whole lines:
  // '\0' which isn't the EOF can only be seen after '\n'.
  {
    let mut c: libc::c_char = *(*p).lex_inbuf; // force next line to be read
    loop {
      if c as libc::c_int == '\u{0}' as i32 {
        break 's_36;
      }
      if !(c as libc::c_int == '\\' as i32
        && *(*p).lex_inbuf.offset(1) as libc::c_int == '\n' as i32)
      {
        break;
      }
      (*p).lex_inbuf = (*p).lex_inbuf.offset(2);
      (*p).lex_line = (*p).lex_line.wrapping_add(1);
      c = peek_inbuf()
    }
    if !((c as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32)
      && ((c as libc::c_int) < 'A' as i32 || c as libc::c_int > last_valid_ch as libc::c_int)
    {
      if c as libc::c_int != '.' as i32 {
        break;
      }
      // if '.' was already seen, stop on second one:
      if pt {
        break;
      }
      pt = 1i32 != 0
    }
    // c is one of "0-9A-Z."
    last = c;
    bc_vec_push(
      &mut (*p).lex_strnumbuf,
      (*p).lex_inbuf as *const libc::c_void,
    );
    (*p).lex_inbuf = (*p).lex_inbuf.offset(1)
  }
  if last as libc::c_int == '.' as i32 {
    // remove trailing '.' if any
    bc_vec_pop(&mut (*p).lex_strnumbuf);
  }
  bc_vec_pushZeroByte(&mut (*p).lex_strnumbuf);
  (*ptr_to_globals).err_line = (*ptr_to_globals).prs.lex_line;
  return BC_STATUS_SUCCESS;
}
unsafe extern "C" fn xc_lex_name() {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut i: size_t = 0;
  let mut buf: *const libc::c_char = 0 as *const libc::c_char;
  (*p).lex = XC_LEX_NAME as libc::c_int as smallint;
  // Since names can't cross lines with \<newline>,
  // we depend on the fact that whole line is in the buffer
  i = 0i32 as size_t;
  buf = (*p).lex_inbuf.offset(-1);
  loop {
    let mut c: libc::c_char = *buf.offset(i as isize);
    if ((c as libc::c_int) < 'a' as i32 || c as libc::c_int > 'z' as i32)
      && !((c as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32)
      && c as libc::c_int != '_' as i32
    {
      break;
    }
    i = i.wrapping_add(1)
  }
  // We do not protect against people with gigabyte-long names
  bc_vec_string(&mut (*p).lex_strnumbuf, i, buf);
  // Increment the index. We minus 1 because it has already been incremented.
  (*p).lex_inbuf = (*p)
    .lex_inbuf
    .offset(i.wrapping_sub(1i32 as libc::c_ulong) as isize);
  //return BC_STATUS_SUCCESS;
}
unsafe extern "C" fn zxc_lex_next() -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  (*ptr_to_globals).err_line = (*p).lex_line;
  (*p).lex_last = (*p).lex;
  //why?
  //	if (p->lex_last == XC_LEX_EOF)
  //		RETURN_STATUS(bc_error("end of file"));
  // Loop until failure or we don't have whitespace. This
  // is so the parser doesn't get inundated with whitespace.
  // Comments are also XC_LEX_WHITESPACE tokens and eaten here.
  s = BC_STATUS_SUCCESS;
  loop {
    if *(*p).lex_inbuf as libc::c_int == '\u{0}' as i32 {
      (*p).lex = XC_LEX_EOF as libc::c_int as smallint;
      if peek_inbuf() as libc::c_int == '\u{0}' as i32 {
        return BC_STATUS_SUCCESS;
      }
    }
    (*p).lex_next_at = (*p).lex_inbuf;
    if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(0) as libc::c_int == 'b' as i32) {
      s = zbc_lex_token()
    } else {
      s = zdc_lex_token()
    }
    if !(s as u64 == 0 && (*p).lex as libc::c_int == XC_LEX_WHITESPACE as libc::c_int) {
      break;
    }
  }
  return s;
}
unsafe extern "C" fn zbc_lex_skip_if_at_NLINE() -> BcStatus {
  if (*ptr_to_globals).prs.lex as libc::c_int == XC_LEX_NLINE as libc::c_int {
    return zxc_lex_next();
  }
  return BC_STATUS_SUCCESS;
}
unsafe extern "C" fn zbc_lex_next_and_skip_NLINE() -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  s = zxc_lex_next();
  if s as u64 != 0 {
    return s;
  }
  // if(cond)<newline>stmt is accepted too (but not 2+ newlines)
  s = zbc_lex_skip_if_at_NLINE(); // "ifz" does not match "if" keyword, "if." does
  return s;
}
unsafe extern "C" fn zbc_lex_identifier() -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut i: libc::c_uint = 0;
  let mut buf: *const libc::c_char = (*p).lex_inbuf.offset(-1);
  let mut current_block_9: u64;
  i = 0i32 as libc::c_uint;
  while i
    < (::std::mem::size_of::<[BcLexKeyword; 20]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<BcLexKeyword>() as libc::c_ulong) as libc::c_uint
  {
    let mut keyword8: *const libc::c_char = bc_lex_kws[i as usize].name8.as_ptr();
    let mut j: libc::c_uint = 0i32 as libc::c_uint;
    loop {
      if !(*buf.offset(j as isize) as libc::c_int != '\u{0}' as i32
        && *buf.offset(j as isize) as libc::c_int == *keyword8.offset(j as isize) as libc::c_int)
      {
        current_block_9 = 3276175668257526147;
        break;
      }
      j = j.wrapping_add(1);
      if j == 8i32 as libc::c_uint {
        current_block_9 = 12777431847750420019;
        break;
      }
    }
    match current_block_9 {
      3276175668257526147 => {
        if *keyword8.offset(j as isize) as libc::c_int != '\u{0}' as i32 {
          current_block_9 = 12675440807659640239;
        } else {
          current_block_9 = 12777431847750420019;
        }
      }
      _ => {}
    }
    match current_block_9 {
      12777431847750420019 =>
      // buf starts with keyword bc_lex_kws[i]
      {
        if !(bb_ascii_isalnum(*buf.offset(j as isize) as libc::c_uchar) != 0
          || *buf.offset(j as isize) as libc::c_int == '_' as i32)
        {
          (*p).lex =
            (BC_LEX_KEY_1st_keyword as libc::c_int as libc::c_uint).wrapping_add(i) as smallint;
          if 1i32 << i & POSIX_KWORD_MASK as libc::c_int == 0 {
            s = zbc_posix_error_fmt(
              b"%sthe \'%.8s\' keyword\x00" as *const u8 as *const libc::c_char,
              b"POSIX does not allow \x00" as *const u8 as *const libc::c_char,
              bc_lex_kws[i as usize].name8.as_ptr(),
            );
            if s as u64 != 0 {
              return s;
            }
          }
          // We minus 1 because the index has already been incremented.
          (*p).lex_inbuf = (*p)
            .lex_inbuf
            .offset(j.wrapping_sub(1i32 as libc::c_uint) as isize);
          return BC_STATUS_SUCCESS;
        }
      }
      _ => {}
    }
    i = i.wrapping_add(1)
  }
  xc_lex_name();
  s = BC_STATUS_SUCCESS;
  if (*p).lex_strnumbuf.len > 2i32 as libc::c_ulong {
    // Prevent this:
    // >>> qwe=1
    // bc: POSIX only allows one character names; this is bad: 'qwe=1
    // '
    let mut len: libc::c_uint =
      strchrnul(buf, '\n' as i32).wrapping_offset_from(buf) as libc::c_long as libc::c_uint; // strings can cross lines
    s = zbc_posix_error_fmt(
      b"POSIX only allows one character names; this is bad: \'%.*s\'\x00" as *const u8
        as *const libc::c_char,
      len,
      buf,
    )
  } // else store "without" value
  return s;
}
unsafe extern "C" fn zbc_lex_string() -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  (*p).lex = XC_LEX_STR as libc::c_int as smallint;
  bc_vec_pop_all(&mut (*p).lex_strnumbuf);
  loop {
    let mut c: libc::c_char = peek_inbuf();
    if c as libc::c_int == '\u{0}' as i32 {
      return bc_error(b"unterminated string\x00" as *const u8 as *const libc::c_char) as BcStatus;
    }
    if c as libc::c_int == '\"' as i32 {
      break;
    }
    if c as libc::c_int == '\n' as i32 {
      (*p).lex_line = (*p).lex_line.wrapping_add(1)
    }
    bc_vec_push(
      &mut (*p).lex_strnumbuf,
      (*p).lex_inbuf as *const libc::c_void,
    );
    (*p).lex_inbuf = (*p).lex_inbuf.offset(1)
  }
  bc_vec_pushZeroByte(&mut (*p).lex_strnumbuf);
  (*p).lex_inbuf = (*p).lex_inbuf.offset(1);
  (*ptr_to_globals).err_line = (*p).lex_line;
  return BC_STATUS_SUCCESS;
}
unsafe extern "C" fn parse_lex_by_checking_eq_sign(mut with_and_without: libc::c_uint) {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  if *(*p).lex_inbuf as libc::c_int == '=' as i32 {
    // ^^^ not using peek_inbuf() since '==' etc can't be split across lines
    (*p).lex_inbuf = (*p).lex_inbuf.offset(1);
    with_and_without >>= 8i32
    // store "with" value
  }
  (*p).lex = (with_and_without & 0xffi32 as libc::c_uint) as smallint;
}
unsafe extern "C" fn zbc_lex_comment() -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  (*p).lex = XC_LEX_WHITESPACE as libc::c_int as smallint;
  's_14: loop
  // here lex_inbuf is at '*' of opening comment delimiter
  {
    let mut c: libc::c_char = 0; // no need to peek_inbuf()
    (*p).lex_inbuf = (*p).lex_inbuf.offset(1); // skip trailing '/'
    c = peek_inbuf();
    while c as libc::c_int == '*' as i32 {
      (*p).lex_inbuf = (*p).lex_inbuf.offset(1);
      c = *(*p).lex_inbuf;
      if c as libc::c_int == '/' as i32 {
        break 's_14;
      }
    }
    if c as libc::c_int == '\u{0}' as i32 {
      return bc_error(b"unterminated comment\x00" as *const u8 as *const libc::c_char) as BcStatus;
    }
    if c as libc::c_int == '\n' as i32 {
      (*p).lex_line = (*p).lex_line.wrapping_add(1)
    }
  }
  (*p).lex_inbuf = (*p).lex_inbuf.offset(1);
  (*ptr_to_globals).err_line = (*p).lex_line;
  return BC_STATUS_SUCCESS;
}
unsafe extern "C" fn zbc_lex_token() -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut c: libc::c_char = eat_inbuf();
  let mut c2: libc::c_char = 0;
  // This is the workhorse of the lexer.
  match c as libc::c_int {
    10 => {
      //	case '\0': // probably never reached
      //		p->lex_inbuf--;
      //		p->lex = XC_LEX_EOF;
      //		break;
      (*p).lex_line = (*p).lex_line.wrapping_add(1);
      (*p).lex = XC_LEX_NLINE as libc::c_int as smallint
    }
    9 | 11 | 12 | 13 | 32 => {
      xc_lex_whitespace();
    }
    33 => {
      parse_lex_by_checking_eq_sign(
        ((XC_LEX_OP_REL_NE as libc::c_int) << 8i32 | BC_LEX_OP_BOOL_NOT as libc::c_int)
          as libc::c_uint,
      );
      if (*p).lex as libc::c_int == BC_LEX_OP_BOOL_NOT as libc::c_int {
        s = zbc_POSIX_does_not_allow_bool_ops_this_is_bad(
          b"!\x00" as *const u8 as *const libc::c_char,
        );
        if s as u64 != 0 {
          return s;
        }
      }
    }
    34 => s = zbc_lex_string(),
    35 => {
      s =
        zbc_POSIX_does_not_allow(b"\'#\' script comments\x00" as *const u8 as *const libc::c_char);
      if s as u64 != 0 {
        return s;
      }
      xc_lex_lineComment();
    }
    37 => {
      parse_lex_by_checking_eq_sign(
        ((BC_LEX_OP_ASSIGN_MODULUS as libc::c_int) << 8i32 | XC_LEX_OP_MODULUS as libc::c_int)
          as libc::c_uint,
      );
    }
    38 => {
      c2 = *(*p).lex_inbuf;
      if c2 as libc::c_int == '&' as i32 {
        s = zbc_POSIX_does_not_allow_bool_ops_this_is_bad(
          b"&&\x00" as *const u8 as *const libc::c_char,
        );
        if s as u64 != 0 {
          return s;
        }
        (*p).lex_inbuf = (*p).lex_inbuf.offset(1);
        (*p).lex = BC_LEX_OP_BOOL_AND as libc::c_int as smallint
      } else {
        (*p).lex = XC_LEX_INVALID as libc::c_int as smallint;
        s = bc_error_bad_character('&' as i32 as libc::c_char) as BcStatus
      }
    }
    40 | 41 => {
      (*p).lex =
        (c as libc::c_int - '(' as i32 + BC_LEX_LPAREN as libc::c_int) as BcLexType as smallint
    }
    42 => {
      parse_lex_by_checking_eq_sign(
        ((BC_LEX_OP_ASSIGN_MULTIPLY as libc::c_int) << 8i32 | XC_LEX_OP_MULTIPLY as libc::c_int)
          as libc::c_uint,
      );
    }
    43 => {
      c2 = *(*p).lex_inbuf;
      if c2 as libc::c_int == '+' as i32 {
        (*p).lex_inbuf = (*p).lex_inbuf.offset(1);
        (*p).lex = BC_LEX_OP_INC as libc::c_int as smallint
      } else {
        parse_lex_by_checking_eq_sign(
          ((BC_LEX_OP_ASSIGN_PLUS as libc::c_int) << 8i32 | XC_LEX_OP_PLUS as libc::c_int)
            as libc::c_uint,
        );
      }
    }
    44 => (*p).lex = BC_LEX_COMMA as libc::c_int as smallint,
    45 => {
      c2 = *(*p).lex_inbuf;
      if c2 as libc::c_int == '-' as i32 {
        (*p).lex_inbuf = (*p).lex_inbuf.offset(1);
        (*p).lex = BC_LEX_OP_DEC as libc::c_int as smallint
      } else {
        parse_lex_by_checking_eq_sign(
          ((BC_LEX_OP_ASSIGN_MINUS as libc::c_int) << 8i32 | XC_LEX_OP_MINUS as libc::c_int)
            as libc::c_uint,
        );
      }
    }
    46 => {
      if (*(*p).lex_inbuf as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
        s = zxc_lex_number(c)
      } else {
        (*p).lex = BC_LEX_KEY_LAST as libc::c_int as smallint;
        s = zbc_POSIX_does_not_allow(b"\'.\' as \'last\'\x00" as *const u8 as *const libc::c_char)
      }
    }
    47 => {
      c2 = *(*p).lex_inbuf;
      if c2 as libc::c_int == '*' as i32 {
        s = zbc_lex_comment()
      } else {
        parse_lex_by_checking_eq_sign(
          ((BC_LEX_OP_ASSIGN_DIVIDE as libc::c_int) << 8i32 | XC_LEX_OP_DIVIDE as libc::c_int)
            as libc::c_uint,
        );
      }
    }
    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72
    | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => {
      s = zxc_lex_number(c)
    }
    59 => (*p).lex = BC_LEX_SCOLON as libc::c_int as smallint,
    60 => {
      parse_lex_by_checking_eq_sign(
        ((XC_LEX_OP_REL_LE as libc::c_int) << 8i32 | XC_LEX_OP_REL_LT as libc::c_int)
          as libc::c_uint,
      );
    }
    61 => {
      parse_lex_by_checking_eq_sign(
        ((XC_LEX_OP_REL_EQ as libc::c_int) << 8i32 | BC_LEX_OP_ASSIGN as libc::c_int)
          as libc::c_uint,
      );
    }
    62 => {
      parse_lex_by_checking_eq_sign(
        ((XC_LEX_OP_REL_GE as libc::c_int) << 8i32 | XC_LEX_OP_REL_GT as libc::c_int)
          as libc::c_uint,
      );
    }
    91 | 93 => {
      (*p).lex =
        (c as libc::c_int - '[' as i32 + BC_LEX_LBRACKET as libc::c_int) as BcLexType as smallint
    }
    92 => {
      if *(*p).lex_inbuf as libc::c_int == '\n' as i32 {
        (*p).lex = XC_LEX_WHITESPACE as libc::c_int as smallint;
        (*p).lex_inbuf = (*p).lex_inbuf.offset(1)
      } else {
        s = bc_error_bad_character(c) as BcStatus
      }
    }
    94 => {
      parse_lex_by_checking_eq_sign(
        ((BC_LEX_OP_ASSIGN_POWER as libc::c_int) << 8i32 | XC_LEX_OP_POWER as libc::c_int)
          as libc::c_uint,
      );
    }
    97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112
    | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 => s = zbc_lex_identifier(),
    123 | 125 => {
      (*p).lex =
        (c as libc::c_int - '{' as i32 + BC_LEX_LBRACE as libc::c_int) as BcLexType as smallint
    }
    124 => {
      c2 = *(*p).lex_inbuf;
      if c2 as libc::c_int == '|' as i32 {
        s = zbc_POSIX_does_not_allow_bool_ops_this_is_bad(
          b"||\x00" as *const u8 as *const libc::c_char,
        );
        if s as u64 != 0 {
          return s;
        }
        (*p).lex_inbuf = (*p).lex_inbuf.offset(1);
        (*p).lex = BC_LEX_OP_BOOL_OR as libc::c_int as smallint
      } else {
        (*p).lex = XC_LEX_INVALID as libc::c_int as smallint;
        s = bc_error_bad_character(c) as BcStatus
      }
    }
    _ => {
      (*p).lex = XC_LEX_INVALID as libc::c_int as smallint;
      s = bc_error_bad_character(c) as BcStatus
    }
  }
  return s;
}
// ENABLE_BC
unsafe extern "C" fn zdc_lex_register() -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs; // eats whitespace (but not newline)
  if 1i32 != 0
    && option_mask32 & ((1i32 << 6i32) * 1i32) as libc::c_uint != 0
    && ({
      let mut bb__isspace: libc::c_uchar = (*(*p).lex_inbuf as libc::c_int - 9i32) as libc::c_uchar; // xc_lex_name() expects this
      (bb__isspace as libc::c_int == ' ' as i32 - 9i32
        || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
    }) != 0
  {
    xc_lex_whitespace(); // skip trailing ']'
    (*p).lex_inbuf = (*p).lex_inbuf.offset(1);
    xc_lex_name();
  } else {
    bc_vec_pop_all(&mut (*p).lex_strnumbuf);
    let fresh13 = (*p).lex_inbuf;
    (*p).lex_inbuf = (*p).lex_inbuf.offset(1);
    bc_vec_push(&mut (*p).lex_strnumbuf, fresh13 as *const libc::c_void);
    bc_vec_pushZeroByte(&mut (*p).lex_strnumbuf);
    (*p).lex = XC_LEX_NAME as libc::c_int as smallint
  }
  return BC_STATUS_SUCCESS;
}
unsafe extern "C" fn zdc_lex_string() -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut depth: size_t = 0;
  (*p).lex = XC_LEX_STR as libc::c_int as smallint;
  bc_vec_pop_all(&mut (*p).lex_strnumbuf);
  depth = 1i32 as size_t;
  loop {
    let mut c: libc::c_char = peek_inbuf();
    if c as libc::c_int == '\u{0}' as i32 {
      return bc_error(b"unterminated string\x00" as *const u8 as *const libc::c_char) as BcStatus;
    }
    if c as libc::c_int == '[' as i32 {
      depth = depth.wrapping_add(1)
    }
    if c as libc::c_int == ']' as i32 {
      depth = depth.wrapping_sub(1);
      if depth == 0i32 as libc::c_ulong {
        break;
      }
    }
    if c as libc::c_int == '\n' as i32 {
      (*p).lex_line = (*p).lex_line.wrapping_add(1)
    }
    bc_vec_push(
      &mut (*p).lex_strnumbuf,
      (*p).lex_inbuf as *const libc::c_void,
    );
    (*p).lex_inbuf = (*p).lex_inbuf.offset(1)
  }
  bc_vec_pushZeroByte(&mut (*p).lex_strnumbuf);
  (*p).lex_inbuf = (*p).lex_inbuf.offset(1);
  (*ptr_to_globals).err_line = (*p).lex_line;
  return BC_STATUS_SUCCESS;
}
unsafe extern "C" fn zdc_lex_token() -> BcStatus {
  //BcLexType - should be this type, but narrower type saves size:
  static mut dc_lex_regs: [uint8_t; 13] = [
    XC_LEX_OP_REL_EQ as libc::c_int as uint8_t,
    XC_LEX_OP_REL_LE as libc::c_int as uint8_t,
    XC_LEX_OP_REL_GE as libc::c_int as uint8_t,
    XC_LEX_OP_REL_NE as libc::c_int as uint8_t,
    XC_LEX_OP_REL_LT as libc::c_int as uint8_t,
    XC_LEX_OP_REL_GT as libc::c_int as uint8_t,
    DC_LEX_SCOLON as libc::c_int as uint8_t,
    DC_LEX_COLON as libc::c_int as uint8_t,
    DC_LEX_ELSE as libc::c_int as uint8_t,
    DC_LEX_LOAD as libc::c_int as uint8_t,
    DC_LEX_LOAD_POP as libc::c_int as uint8_t,
    DC_LEX_OP_ASSIGN as libc::c_int as uint8_t,
    DC_LEX_STORE_PUSH as libc::c_int as uint8_t,
  ];
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut c: libc::c_char = 0;
  let mut c2: libc::c_char = 0;
  let mut i: size_t = 0;
  i = 0i32 as size_t;
  while i
    < (::std::mem::size_of::<[uint8_t; 13]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<uint8_t>() as libc::c_ulong) as libc::c_uint
      as libc::c_ulong
  {
    if (*p).lex_last as libc::c_int == dc_lex_regs[i as usize] as libc::c_int {
      return zdc_lex_register();
    }
    i = i.wrapping_add(1)
  }
  s = BC_STATUS_SUCCESS;
  c = eat_inbuf();
  if c as libc::c_int >= '%' as i32 && c as libc::c_int <= '~' as i32 && {
    (*p).lex = dc_char_to_LEX[(c as libc::c_int - '%' as i32) as usize] as smallint;
    ((*p).lex as libc::c_int) != XC_LEX_INVALID as libc::c_int
  } {
    return s;
  }
  // This is the workhorse of the lexer.
  match c as libc::c_int {
    10 => {
      //	case '\0': // probably never reached
      //		p->lex = XC_LEX_EOF;
      //		break;
      // '\n' is XC_LEX_NLINE, not XC_LEX_WHITESPACE
      // (and "case '\n':" is not just empty here)
      // only to allow interactive dc have a way to exit
      // "parse" stage of "parse,execute" loop
      // on <enter>, not on _next_ token (which would mean
      // commands are not executed on pressing <enter>).
      // IOW: typing "1p<enter>" should print "1" _at once_,
      // not after some more input.
      (*p).lex_line = (*p).lex_line.wrapping_add(1);
      (*p).lex = XC_LEX_NLINE as libc::c_int as smallint
    }
    9 | 11 | 12 | 13 | 32 => {
      xc_lex_whitespace();
    }
    33 => {
      c2 = *(*p).lex_inbuf;
      if c2 as libc::c_int == '=' as i32 {
        (*p).lex = XC_LEX_OP_REL_NE as libc::c_int as smallint
      } else if c2 as libc::c_int == '<' as i32 {
        (*p).lex = XC_LEX_OP_REL_LE as libc::c_int as smallint
      } else if c2 as libc::c_int == '>' as i32 {
        (*p).lex = XC_LEX_OP_REL_GE as libc::c_int as smallint
      } else {
        return bc_error_bad_character(c) as BcStatus;
      }
      (*p).lex_inbuf = (*p).lex_inbuf.offset(1)
    }
    35 => {
      xc_lex_lineComment();
    }
    46 => {
      if (*(*p).lex_inbuf as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
        s = zxc_lex_number(c)
      } else {
        s = bc_error_bad_character(c) as BcStatus
      }
    }
    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 65 | 66 | 67 | 68 | 69 | 70 => {
      s = zxc_lex_number(c)
    }
    91 => s = zdc_lex_string(),
    _ => {
      (*p).lex = XC_LEX_INVALID as libc::c_int as smallint;
      s = bc_error_bad_character(c) as BcStatus
    }
  }
  return s;
}
// ENABLE_DC
unsafe extern "C" fn xc_parse_push(mut i: libc::c_uint) {
  let mut code: *mut BcVec = &mut (*(*ptr_to_globals).prs.func).code;
  bc_vec_pushByte(code, i as uint8_t as libc::c_char);
}
unsafe extern "C" fn xc_parse_pushName(mut name: *mut libc::c_char) {
  let mut code: *mut BcVec = &mut (*(*ptr_to_globals).prs.func).code;
  let mut pos: size_t = (*code).len;
  let mut len: size_t = strlen(name).wrapping_add(1i32 as libc::c_ulong);
  bc_vec_expand(code, pos.wrapping_add(len));
  strcpy((*code).v.offset(pos as isize), name);
  (*code).len = pos.wrapping_add(len);
}
unsafe extern "C" fn bc_vec_pushIndex(mut v: *mut BcVec, mut idx: size_t) {
  let mut mask: size_t = 0;
  let mut amt: libc::c_uint = 0;
  if idx
    < (0x100i32 as libc::c_ulong).wrapping_sub(::std::mem::size_of::<size_t>() as libc::c_ulong)
  {
    bc_vec_pushByte(v, idx as libc::c_char);
    return;
  }
  mask = (0xffi32 as size_t)
    << (::std::mem::size_of::<size_t>() as libc::c_ulong)
      .wrapping_mul(8i32 as libc::c_ulong)
      .wrapping_sub(8i32 as libc::c_ulong);
  amt = ::std::mem::size_of::<size_t>() as libc::c_ulong as libc::c_uint;
  while !(idx & mask != 0) {
    mask >>= 8i32;
    amt = amt.wrapping_sub(1)
  }
  // amt is at least 1 here - "one byte of length data follows"
  bc_vec_pushByte(
    v,
    (0x100i32 as libc::c_ulong)
      .wrapping_sub(::std::mem::size_of::<size_t>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong)
      .wrapping_add(amt as libc::c_ulong) as libc::c_char,
  );
  loop {
    bc_vec_pushByte(v, idx as libc::c_uchar as libc::c_char);
    idx >>= 8i32;
    if !(idx != 0i32 as libc::c_ulong) {
      break;
    }
  }
}
unsafe extern "C" fn xc_parse_pushIndex(mut idx: size_t) {
  bc_vec_pushIndex(&mut (*(*ptr_to_globals).prs.func).code, idx);
}
unsafe extern "C" fn xc_parse_pushInst_and_Index(mut inst: libc::c_uint, mut idx: size_t) {
  xc_parse_push(inst);
  xc_parse_pushIndex(idx);
}
unsafe extern "C" fn bc_parse_pushJUMP(mut idx: size_t) {
  xc_parse_pushInst_and_Index(BC_INST_JUMP as libc::c_int as libc::c_uint, idx);
}
unsafe extern "C" fn bc_parse_pushJUMP_ZERO(mut idx: size_t) {
  xc_parse_pushInst_and_Index(BC_INST_JUMP_ZERO as libc::c_int as libc::c_uint, idx);
}
unsafe extern "C" fn zbc_parse_pushSTR() -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut str: *mut libc::c_char = xstrdup((*p).lex_strnumbuf.v);
  xc_parse_pushInst_and_Index(
    XC_INST_STR as libc::c_int as libc::c_uint,
    (*(*p).func).strs.len,
  );
  bc_vec_push(
    &mut (*(*p).func).strs,
    &mut str as *mut *mut libc::c_char as *const libc::c_void,
  );
  return zxc_lex_next();
}
unsafe extern "C" fn xc_parse_pushNUM() {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut num: *mut libc::c_char = xstrdup((*p).lex_strnumbuf.v);
  let mut idx: size_t = bc_vec_push(
    if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(0) as libc::c_int == 'b' as i32) {
      &mut (*(*p).func).consts
    } else {
      &mut (*ptr_to_globals).prog.consts
    },
    &mut num as *mut *mut libc::c_char as *const libc::c_void,
  );
  xc_parse_pushInst_and_Index(XC_INST_NUM as libc::c_int as libc::c_uint, idx);
}
unsafe extern "C" fn zxc_parse_text_init(mut text: *const libc::c_char) -> BcStatus {
  (*ptr_to_globals).prs.func = xc_program_func((*ptr_to_globals).prs.fidx);
  (*ptr_to_globals).prs.lex_inbuf = text;
  (*ptr_to_globals).prs.lex_last = XC_LEX_INVALID as libc::c_int as smallint;
  (*ptr_to_globals).prs.lex = (*ptr_to_globals).prs.lex_last;
  return zxc_lex_next();
}
// Called when parsing or execution detects a failure,
// resets execution structures.
unsafe extern "C" fn xc_program_reset() {
  let mut f: *mut BcFunc = 0 as *mut BcFunc;
  let mut ip: *mut BcInstPtr = 0 as *mut BcInstPtr;
  bc_vec_npop(
    &mut (*ptr_to_globals).prog.exestack,
    (*ptr_to_globals)
      .prog
      .exestack
      .len
      .wrapping_sub(1i32 as libc::c_ulong),
  );
  bc_vec_pop_all(&mut (*ptr_to_globals).prog.results);
  f = (*ptr_to_globals).prog.fns.v as *mut BcFunc;
  ip = bc_vec_top(&mut (*ptr_to_globals).prog.exestack) as *mut BcInstPtr;
  (*ip).inst_idx = (*f).code.len;
}
// Called when parsing code detects a failure,
// resets parsing structures.
unsafe extern "C" fn xc_parse_reset() {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  if (*p).fidx != 0i32 as libc::c_ulong {
    bc_func_free((*p).func as *mut libc::c_void);
    bc_func_init((*p).func);
    (*p).fidx = 0i32 as size_t;
    (*p).func = (*ptr_to_globals).prog.fns.v as *mut BcFunc
  }
  (*p).lex_inbuf = (*p).lex_inbuf.offset(strlen((*p).lex_inbuf) as isize);
  (*p).lex = XC_LEX_EOF as libc::c_int as smallint;
  bc_vec_pop_all(&mut (*p).exits);
  bc_vec_pop_all(&mut (*p).conds);
  bc_vec_pop_all(&mut (*p).ops);
  xc_program_reset();
}
unsafe extern "C" fn xc_parse_free() {
  bc_vec_free(&mut (*ptr_to_globals).prs.exits as *mut BcVec as *mut libc::c_void);
  bc_vec_free(&mut (*ptr_to_globals).prs.conds as *mut BcVec as *mut libc::c_void);
  bc_vec_free(&mut (*ptr_to_globals).prs.ops as *mut BcVec as *mut libc::c_void);
  bc_vec_free(&mut (*ptr_to_globals).prs.lex_strnumbuf as *mut BcVec as *mut libc::c_void);
}
unsafe extern "C" fn xc_parse_create(mut fidx: size_t) {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  memset(
    p as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<BcParse>() as libc::c_ulong,
  );
  bc_char_vec_init(&mut (*p).lex_strnumbuf);
  bc_vec_init(
    &mut (*p).exits,
    ::std::mem::size_of::<size_t>() as libc::c_ulong,
    None,
  );
  bc_vec_init(
    &mut (*p).conds,
    ::std::mem::size_of::<size_t>() as libc::c_ulong,
    None,
  );
  bc_vec_init(
    &mut (*p).ops,
    ::std::mem::size_of::<BcLexType>() as libc::c_ulong,
    None,
  );
  (*p).fidx = fidx;
  (*p).func = xc_program_func(fidx);
}
unsafe extern "C" fn xc_program_add_fn() {
  //size_t idx;
  let mut f: BcFunc = BcFunc {
    code: BcVec {
      v: 0 as *mut libc::c_char,
      len: 0,
      cap: 0,
      size: 0,
      dtor: None,
    },
    labels: BcVec {
      v: 0 as *mut libc::c_char,
      len: 0,
      cap: 0,
      size: 0,
      dtor: None,
    },
    autos: BcVec {
      v: 0 as *mut libc::c_char,
      len: 0,
      cap: 0,
      size: 0,
      dtor: None,
    },
    strs: BcVec {
      v: 0 as *mut libc::c_char,
      len: 0,
      cap: 0,
      size: 0,
      dtor: None,
    },
    consts: BcVec {
      v: 0 as *mut libc::c_char,
      len: 0,
      cap: 0,
      size: 0,
      dtor: None,
    },
    nparams: 0,
    voidfunc: false,
  };
  bc_func_init(&mut f);
  //idx =
  bc_vec_push(
    &mut (*ptr_to_globals).prog.fns,
    &mut f as *mut BcFunc as *const libc::c_void,
  );
  //return idx;
}
// Note: takes ownership of 'name' (must be malloced)
unsafe extern "C" fn bc_program_addFunc(mut name: *mut libc::c_char) -> size_t {
  let mut idx: size_t = 0;
  let mut entry: BcId = BcId {
    name: 0 as *mut libc::c_char,
    idx: 0,
  };
  let mut entry_ptr: *mut BcId = 0 as *mut BcId;
  let mut inserted: libc::c_int = 0;
  entry.name = name;
  entry.idx = (*ptr_to_globals).prog.fns.len;
  inserted = bc_map_insert(
    &mut (*ptr_to_globals).prog.fn_map,
    &mut entry as *mut BcId as *const libc::c_void,
    &mut idx,
  );
  if inserted == 0 {
    free(name as *mut libc::c_void);
  }
  entry_ptr = bc_vec_item(&mut (*ptr_to_globals).prog.fn_map, idx) as *mut BcId;
  idx = (*entry_ptr).idx;
  if inserted == 0 {
    // There is already a function with this name.
    // It'll be redefined now, clear old definition.
    let mut func: *mut BcFunc = xc_program_func((*entry_ptr).idx);
    bc_func_free(func as *mut libc::c_void);
    bc_func_init(func);
  } else {
    xc_program_add_fn();
  }
  return idx;
}
unsafe extern "C" fn zbc_parse_stmt() -> BcStatus {
  return zbc_parse_stmt_possibly_auto(0i32 != 0);
}
unsafe extern "C" fn zbc_parse_stmt_allow_NLINE_before(
  mut after_X: *const libc::c_char,
) -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  // "if(cond)<newline>stmt" is accepted too, but not 2+ newlines.
  // Same for "else", "while()", "for()".
  let mut s: BcStatus = zbc_lex_next_and_skip_NLINE();
  if s as u64 != 0 {
    return s;
  }
  if (*p).lex as libc::c_int == XC_LEX_NLINE as libc::c_int {
    return bc_error_fmt(
      b"no statement after \'%s\'\x00" as *const u8 as *const libc::c_char,
      after_X,
    ) as BcStatus;
  }
  return zbc_parse_stmt();
}
unsafe extern "C" fn bc_parse_operator(
  mut type_0: BcLexType,
  mut start: size_t,
  mut nexprs: *mut size_t,
) {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut l: libc::c_char = 0;
  let mut r: libc::c_char = (bc_ops_prec_and_assoc
    [(type_0 as libc::c_uint).wrapping_sub(XC_LEX_1st_op as libc::c_int as libc::c_uint) as usize]
    as libc::c_int
    & 0xfi32) as libc::c_char;
  let mut left: bool = bc_ops_prec_and_assoc
    [(type_0 as libc::c_uint).wrapping_sub(XC_LEX_1st_op as libc::c_int as libc::c_uint) as usize]
    as libc::c_int
    & 0x10i32
    != 0;
  while (*p).ops.len > start {
    let mut t: BcLexType = *(bc_vec_top(&mut (*p).ops) as *mut BcLexType);
    if t as libc::c_uint == BC_LEX_LPAREN as libc::c_int as libc::c_uint {
      break;
    }
    l = (bc_ops_prec_and_assoc
      [(t as libc::c_uint).wrapping_sub(XC_LEX_1st_op as libc::c_int as libc::c_uint) as usize]
      as libc::c_int
      & 0xfi32) as libc::c_char;
    if l as libc::c_int >= r as libc::c_int && (l as libc::c_int != r as libc::c_int || !left) {
      break;
    }
    xc_parse_push(
      (t as libc::c_uint)
        .wrapping_sub(XC_LEX_OP_POWER as libc::c_int as libc::c_uint)
        .wrapping_add(XC_INST_POWER as libc::c_int as libc::c_uint) as libc::c_char
        as libc::c_uint,
    );
    bc_vec_pop(&mut (*p).ops);
    *nexprs = (*nexprs as libc::c_ulong).wrapping_sub(
      (t as libc::c_uint != BC_LEX_OP_BOOL_NOT as libc::c_int as libc::c_uint
        && t as libc::c_uint != XC_LEX_NEG as libc::c_int as libc::c_uint) as libc::c_int
        as libc::c_ulong,
    ) as size_t as size_t
  }
  bc_vec_push(
    &mut (*p).ops,
    &mut type_0 as *mut BcLexType as *const libc::c_void,
  );
}
unsafe extern "C" fn zbc_parse_rightParen(mut ops_bgn: size_t, mut nexs: *mut size_t) -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut top: BcLexType = XC_LEX_EOF;
  if (*p).ops.len <= ops_bgn {
    return bc_error_bad_expression() as BcStatus;
  }
  top = *(bc_vec_top(&mut (*p).ops) as *mut BcLexType);
  while top as libc::c_uint != BC_LEX_LPAREN as libc::c_int as libc::c_uint {
    xc_parse_push(
      (top as libc::c_uint)
        .wrapping_sub(XC_LEX_OP_POWER as libc::c_int as libc::c_uint)
        .wrapping_add(XC_INST_POWER as libc::c_int as libc::c_uint) as libc::c_char
        as libc::c_uint,
    );
    bc_vec_pop(&mut (*p).ops);
    *nexs = (*nexs as libc::c_ulong).wrapping_sub(
      (top as libc::c_uint != BC_LEX_OP_BOOL_NOT as libc::c_int as libc::c_uint
        && top as libc::c_uint != XC_LEX_NEG as libc::c_int as libc::c_uint) as libc::c_int
        as libc::c_ulong,
    ) as size_t as size_t;
    if (*p).ops.len <= ops_bgn {
      return bc_error_bad_expression() as BcStatus;
    }
    top = *(bc_vec_top(&mut (*p).ops) as *mut BcLexType)
  }
  bc_vec_pop(&mut (*p).ops);
  return BC_STATUS_SUCCESS;
}
unsafe extern "C" fn zbc_parse_params(mut flags: uint8_t) -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut nparams: size_t = 0;
  flags = (flags as libc::c_int & !(1i32 << 1i32 | 1i32 << 0i32) | 1i32 << 2i32) as uint8_t;
  s = zxc_lex_next();
  if s as u64 != 0 {
    return s;
  }
  nparams = 0i32 as size_t;
  if (*p).lex as libc::c_int != BC_LEX_RPAREN as libc::c_int {
    loop {
      s = zbc_parse_expr(flags);
      if s as u64 != 0 {
        return s;
      }
      nparams = nparams.wrapping_add(1);
      if (*p).lex as libc::c_int != BC_LEX_COMMA as libc::c_int {
        if (*p).lex as libc::c_int == BC_LEX_RPAREN as libc::c_int {
          break;
        }
        return bc_error_bad_token() as BcStatus;
      } else {
        s = zxc_lex_next();
        if s as u64 != 0 {
          return s;
        }
      }
    }
  }
  xc_parse_pushInst_and_Index(BC_INST_CALL as libc::c_int as libc::c_uint, nparams);
  return BC_STATUS_SUCCESS;
}
// Note: takes ownership of 'name' (must be malloced)
unsafe extern "C" fn zbc_parse_call(mut name: *mut libc::c_char, mut flags: uint8_t) -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut entry: BcId = BcId {
    name: 0 as *mut libc::c_char,
    idx: 0,
  };
  let mut entry_ptr: *mut BcId = 0 as *mut BcId;
  let mut idx: size_t = 0;
  entry.name = name;
  s = zbc_parse_params(flags);
  if !(s as u64 != 0) {
    if (*p).lex as libc::c_int != BC_LEX_RPAREN as libc::c_int {
      s = bc_error_bad_token() as BcStatus
    } else {
      idx = bc_map_find_exact(
        &mut (*ptr_to_globals).prog.fn_map,
        &mut entry as *mut BcId as *const libc::c_void,
      );
      if idx == -1i32 as size_t {
        // No such function exists, create an empty one
        bc_program_addFunc(name);
        idx = bc_map_find_exact(
          &mut (*ptr_to_globals).prog.fn_map,
          &mut entry as *mut BcId as *const libc::c_void,
        )
      } else {
        free(name as *mut libc::c_void);
      }
      entry_ptr = bc_vec_item(&mut (*ptr_to_globals).prog.fn_map, idx) as *mut BcId;
      xc_parse_pushIndex((*entry_ptr).idx);
      return zxc_lex_next();
    }
  }
  free(name as *mut libc::c_void);
  return s;
}
unsafe extern "C" fn zbc_parse_name(mut type_0: *mut BcInst, mut flags: uint8_t) -> BcStatus {
  let mut current_block: u64;
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
  name = xstrdup((*p).lex_strnumbuf.v);
  s = zxc_lex_next();
  if !(s as u64 != 0) {
    if (*p).lex as libc::c_int == BC_LEX_LBRACKET as libc::c_int {
      s = zxc_lex_next();
      if s as u64 != 0 {
        current_block = 11734377364214890191;
      } else {
        if (*p).lex as libc::c_int == BC_LEX_RBRACKET as libc::c_int {
          if flags as libc::c_int & 1i32 << 2i32 == 0 {
            s = bc_error_bad_expression() as BcStatus;
            current_block = 11734377364214890191;
          } else {
            *type_0 = XC_INST_ARRAY;
            current_block = 5143058163439228106;
          }
        } else {
          *type_0 = XC_INST_ARRAY_ELEM;
          flags = (flags as libc::c_int & !(1i32 << 1i32 | 1i32 << 0i32)) as uint8_t;
          s = zbc_parse_expr(flags);
          if s as u64 != 0 {
            current_block = 11734377364214890191;
          } else {
            current_block = 5143058163439228106;
          }
        }
        match current_block {
          11734377364214890191 => {}
          _ => {
            s = zxc_lex_next();
            if s as u64 != 0 {
              current_block = 11734377364214890191;
            } else {
              xc_parse_push(*type_0 as libc::c_uint);
              xc_parse_pushName(name);
              free(name as *mut libc::c_void);
              current_block = 5494826135382683477;
            }
          }
        }
      }
    } else if (*p).lex as libc::c_int == BC_LEX_LPAREN as libc::c_int {
      if flags as libc::c_int & 1i32 << 3i32 != 0 {
        s = bc_error_bad_token() as BcStatus;
        current_block = 11734377364214890191;
      } else {
        *type_0 = BC_INST_CALL;
        s = zbc_parse_call(name, flags);
        current_block = 5494826135382683477;
      }
    } else {
      *type_0 = XC_INST_VAR;
      xc_parse_push(XC_INST_VAR as libc::c_int as libc::c_uint);
      xc_parse_pushName(name);
      free(name as *mut libc::c_void);
      current_block = 5494826135382683477;
    }
    match current_block {
      11734377364214890191 => {}
      _ => return s,
    }
  }
  free(name as *mut libc::c_void);
  return s;
}
unsafe extern "C" fn zbc_parse_read() -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  s = zxc_lex_next();
  if s as u64 != 0 {
    return s;
  }
  if (*p).lex as libc::c_int != BC_LEX_LPAREN as libc::c_int {
    return bc_error_bad_token() as BcStatus;
  }
  s = zxc_lex_next();
  if s as u64 != 0 {
    return s;
  }
  if (*p).lex as libc::c_int != BC_LEX_RPAREN as libc::c_int {
    return bc_error_bad_token() as BcStatus;
  }
  xc_parse_push(XC_INST_READ as libc::c_int as libc::c_uint);
  return s;
}
unsafe extern "C" fn zbc_parse_builtin(
  mut type_0: BcLexType,
  mut flags: uint8_t,
  mut prev: *mut BcInst,
) -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  s = zxc_lex_next();
  if s as u64 != 0 {
    return s;
  }
  if (*p).lex as libc::c_int != BC_LEX_LPAREN as libc::c_int {
    return bc_error_bad_token() as BcStatus;
  }
  flags = (flags as libc::c_int & !(1i32 << 1i32 | 1i32 << 0i32) | 1i32 << 2i32) as uint8_t;
  s = zxc_lex_next();
  if s as u64 != 0 {
    return s;
  }
  s = zbc_parse_expr(flags);
  if s as u64 != 0 {
    return s;
  }
  if (*p).lex as libc::c_int != BC_LEX_RPAREN as libc::c_int {
    return bc_error_bad_token() as BcStatus;
  }
  *prev = if type_0 as libc::c_uint == BC_LEX_KEY_LENGTH as libc::c_int as libc::c_uint {
    XC_INST_LENGTH as libc::c_int
  } else {
    XC_INST_SQRT as libc::c_int
  } as BcInst;
  xc_parse_push(*prev as libc::c_uint);
  return s;
}
unsafe extern "C" fn zbc_parse_scale(mut type_0: *mut BcInst, mut flags: uint8_t) -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  s = zxc_lex_next();
  if s as u64 != 0 {
    return s;
  }
  if (*p).lex as libc::c_int != BC_LEX_LPAREN as libc::c_int {
    *type_0 = XC_INST_SCALE;
    xc_parse_push(XC_INST_SCALE as libc::c_int as libc::c_uint);
    return BC_STATUS_SUCCESS;
  }
  *type_0 = XC_INST_SCALE_FUNC;
  flags = (flags as libc::c_int & !(1i32 << 1i32 | 1i32 << 0i32)) as uint8_t;
  s = zxc_lex_next();
  if s as u64 != 0 {
    return s;
  }
  s = zbc_parse_expr(flags);
  if s as u64 != 0 {
    return s;
  }
  if (*p).lex as libc::c_int != BC_LEX_RPAREN as libc::c_int {
    return bc_error_bad_token() as BcStatus;
  }
  xc_parse_push(XC_INST_SCALE_FUNC as libc::c_int as libc::c_uint);
  return zxc_lex_next();
}
unsafe extern "C" fn zbc_parse_incdec(
  mut prev: *mut BcInst,
  mut nexs: *mut size_t,
  mut flags: uint8_t,
) -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut type_0: BcLexType = XC_LEX_EOF;
  let mut inst: libc::c_char = 0;
  let mut etype: BcInst = *prev;
  if etype as libc::c_int == XC_INST_VAR as libc::c_int
    || etype as libc::c_int == XC_INST_ARRAY_ELEM as libc::c_int
    || etype as libc::c_int == XC_INST_SCALE as libc::c_int
    || etype as libc::c_int == BC_INST_LAST as libc::c_int
    || etype as libc::c_int == XC_INST_IBASE as libc::c_int
    || etype as libc::c_int == XC_INST_OBASE as libc::c_int
  {
    inst = (BC_INST_INC_POST as libc::c_int
      + ((*p).lex as libc::c_int != BC_LEX_OP_INC as libc::c_int) as libc::c_int)
      as libc::c_char;
    *prev = inst as BcInst;
    xc_parse_push(inst as libc::c_uint);
    s = zxc_lex_next()
  } else {
    inst = (BC_INST_INC_PRE as libc::c_int
      + ((*p).lex as libc::c_int != BC_LEX_OP_INC as libc::c_int) as libc::c_int)
      as libc::c_char;
    *prev = inst as BcInst;
    s = zxc_lex_next();
    if s as u64 != 0 {
      return s;
    }
    type_0 = (*p).lex as BcLexType;
    // Because we parse the next part of the expression
    // right here, we need to increment this.
    *nexs = (*nexs).wrapping_add(1i32 as libc::c_ulong);
    match type_0 as libc::c_uint {
      5 => s = zbc_parse_name(prev, (flags as libc::c_int | 1i32 << 3i32) as uint8_t),
      47 | 50 | 48 => {
        xc_parse_push(
          (type_0 as libc::c_uint)
            .wrapping_sub(BC_LEX_KEY_IBASE as libc::c_int as libc::c_uint)
            .wrapping_add(XC_INST_IBASE as libc::c_int as libc::c_uint),
        );
        s = zxc_lex_next()
      }
      57 => {
        s = zxc_lex_next();
        if s as u64 != 0 {
          return s;
        }
        if (*p).lex as libc::c_int == BC_LEX_LPAREN as libc::c_int {
          s = bc_error_bad_token() as BcStatus
        } else {
          xc_parse_push(XC_INST_SCALE as libc::c_int as libc::c_uint);
        }
      }
      _ => s = bc_error_bad_token() as BcStatus,
    }
    if s as u64 == 0 {
      xc_parse_push(inst as libc::c_uint);
    }
  }
  return s;
}
unsafe extern "C" fn bc_parse_inst_isLeaf(mut p: BcInst) -> libc::c_int {
  return (p as libc::c_int >= XC_INST_NUM as libc::c_int
    && p as libc::c_int <= XC_INST_SQRT as libc::c_int
    || p as libc::c_int == BC_INST_INC_POST as libc::c_int
    || p as libc::c_int == BC_INST_DEC_POST as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn zbc_parse_minus(
  mut prev: *mut BcInst,
  mut ops_bgn: size_t,
  mut rparen: bool,
  mut bin_last: bool,
  mut nexprs: *mut size_t,
) -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut type_0: BcLexType = XC_LEX_EOF;
  s = zxc_lex_next();
  if s as u64 != 0 {
    return s;
  }
  type_0 = if !bin_last && (rparen as libc::c_int != 0 || bc_parse_inst_isLeaf(*prev) != 0) {
    XC_LEX_OP_MINUS as libc::c_int
  } else {
    XC_LEX_NEG as libc::c_int
  } as BcLexType;
  *prev = (type_0 as libc::c_uint)
    .wrapping_sub(XC_LEX_OP_POWER as libc::c_int as libc::c_uint)
    .wrapping_add(XC_INST_POWER as libc::c_int as libc::c_uint) as libc::c_char as BcInst;
  // We can just push onto the op stack because this is the largest
  // precedence operator that gets pushed. Inc/dec does not.
  if type_0 as libc::c_uint != XC_LEX_OP_MINUS as libc::c_int as libc::c_uint {
    bc_vec_push(
      &mut (*p).ops,
      &mut type_0 as *mut BcLexType as *const libc::c_void,
    );
  } else {
    bc_parse_operator(type_0, ops_bgn, nexprs);
  }
  return s;
}
unsafe extern "C" fn zbc_parse_print() -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut type_0: BcLexType = XC_LEX_EOF;
  loop {
    s = zxc_lex_next();
    if s as u64 != 0 {
      return s;
    }
    type_0 = (*p).lex as BcLexType;
    if type_0 as libc::c_uint == XC_LEX_STR as libc::c_int as libc::c_uint {
      s = zbc_parse_pushSTR()
    } else {
      s = zbc_parse_expr(0i32 as uint8_t)
    }
    if s as u64 != 0 {
      return s;
    }
    xc_parse_push(XC_INST_PRINT_POP as libc::c_int as libc::c_uint);
    if (*p).lex as libc::c_int != BC_LEX_COMMA as libc::c_int {
      break;
    }
  }
  return s;
}
unsafe extern "C" fn zbc_parse_return() -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut t: BcLexType = XC_LEX_EOF;
  s = zxc_lex_next();
  if s as u64 != 0 {
    return s;
  }
  t = (*p).lex as BcLexType;
  if t as libc::c_uint == XC_LEX_NLINE as libc::c_int as libc::c_uint
    || t as libc::c_uint == BC_LEX_SCOLON as libc::c_int as libc::c_uint
    || t as libc::c_uint == BC_LEX_RBRACE as libc::c_int as libc::c_uint
  {
    xc_parse_push(BC_INST_RET0 as libc::c_int as libc::c_uint);
  } else {
    //TODO: if (p->func->voidfunc) ERROR
    s = zbc_parse_expr(0i32 as uint8_t);
    if s as u64 != 0 {
      return s;
    }
    if t as libc::c_uint != BC_LEX_LPAREN as libc::c_int as libc::c_uint
      || (*p).lex_last as libc::c_int != BC_LEX_RPAREN as libc::c_int
    {
      // example: "return (a) + b"
      s = zbc_POSIX_requires(
        b"parentheses around return expressions\x00" as *const u8 as *const libc::c_char,
      );
      if s as u64 != 0 {
        return s;
      }
    }
    xc_parse_push(XC_INST_RET as libc::c_int as libc::c_uint);
  }
  return s;
}
unsafe extern "C" fn rewrite_label_to_current(mut idx: size_t) {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut label: *mut size_t = bc_vec_item(&mut (*(*p).func).labels, idx) as *mut size_t;
  *label = (*(*p).func).code.len;
}
unsafe extern "C" fn zbc_parse_if() -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut ip_idx: size_t = 0;
  s = zxc_lex_next();
  if s as u64 != 0 {
    return s;
  }
  if (*p).lex as libc::c_int != BC_LEX_LPAREN as libc::c_int {
    return bc_error_bad_token() as BcStatus;
  }
  s = zxc_lex_next();
  if s as u64 != 0 {
    return s;
  }
  s = zbc_parse_expr((1i32 << 0i32) as uint8_t);
  if s as u64 != 0 {
    return s;
  }
  if (*p).lex as libc::c_int != BC_LEX_RPAREN as libc::c_int {
    return bc_error_bad_token() as BcStatus;
  }
  // Encode "if zero, jump to ..."
  // Pushed value (destination of the jump) is uninitialized,
  // will be rewritten to be address of "end of if()" or of "else".
  ip_idx = bc_vec_push(
    &mut (*(*p).func).labels,
    &mut ip_idx as *mut size_t as *const libc::c_void,
  );
  bc_parse_pushJUMP_ZERO(ip_idx);
  s = zbc_parse_stmt_allow_NLINE_before(bc_lex_kws[9].name8.as_ptr());
  if s as u64 != 0 {
    return s;
  }
  if (*p).lex as libc::c_int == BC_LEX_KEY_ELSE as libc::c_int {
    let mut ip2_idx: size_t = 0;
    // Encode "after then_stmt, jump to end of if()"
    ip2_idx = bc_vec_push(
      &mut (*(*p).func).labels,
      &mut ip2_idx as *mut size_t as *const libc::c_void,
    );
    bc_parse_pushJUMP(ip2_idx);
    rewrite_label_to_current(ip_idx);
    ip_idx = ip2_idx;
    s = zbc_parse_stmt_allow_NLINE_before(bc_lex_kws[4].name8.as_ptr());
    if s as u64 != 0 {
      return s;
    }
  }
  rewrite_label_to_current(ip_idx);
  return s;
}
unsafe extern "C" fn zbc_parse_while() -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut cond_idx: size_t = 0;
  let mut ip_idx: size_t = 0;
  s = zxc_lex_next();
  if s as u64 != 0 {
    return s;
  }
  if (*p).lex as libc::c_int != BC_LEX_LPAREN as libc::c_int {
    return bc_error_bad_token() as BcStatus;
  }
  s = zxc_lex_next();
  if s as u64 != 0 {
    return s;
  }
  cond_idx = bc_vec_push(
    &mut (*(*p).func).labels,
    &mut (*(*p).func).code.len as *mut size_t as *const libc::c_void,
  );
  ip_idx = cond_idx.wrapping_add(1i32 as libc::c_ulong);
  bc_vec_push(
    &mut (*p).conds,
    &mut cond_idx as *mut size_t as *const libc::c_void,
  );
  bc_vec_push(
    &mut (*p).exits,
    &mut ip_idx as *mut size_t as *const libc::c_void,
  );
  bc_vec_push(
    &mut (*(*p).func).labels,
    &mut ip_idx as *mut size_t as *const libc::c_void,
  );
  s = zbc_parse_expr((1i32 << 0i32) as uint8_t);
  if s as u64 != 0 {
    return s;
  }
  if (*p).lex as libc::c_int != BC_LEX_RPAREN as libc::c_int {
    return bc_error_bad_token() as BcStatus;
  }
  bc_parse_pushJUMP_ZERO(ip_idx);
  s = zbc_parse_stmt_allow_NLINE_before(bc_lex_kws[19].name8.as_ptr());
  if s as u64 != 0 {
    return s;
  }
  bc_parse_pushJUMP(cond_idx);
  rewrite_label_to_current(ip_idx);
  bc_vec_pop(&mut (*p).exits);
  bc_vec_pop(&mut (*p).conds);
  return s;
}
unsafe extern "C" fn zbc_parse_for() -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut cond_idx: size_t = 0;
  let mut exit_idx: size_t = 0;
  let mut body_idx: size_t = 0;
  let mut update_idx: size_t = 0;
  s = zxc_lex_next();
  if s as u64 != 0 {
    return s;
  }
  if (*p).lex as libc::c_int != BC_LEX_LPAREN as libc::c_int {
    return bc_error_bad_token() as BcStatus;
  }
  s = zxc_lex_next();
  if s as u64 != 0 {
    return s;
  }
  if (*p).lex as libc::c_int != BC_LEX_SCOLON as libc::c_int {
    s = zbc_parse_expr(0i32 as uint8_t);
    xc_parse_push(XC_INST_POP as libc::c_int as libc::c_uint);
    if s as u64 != 0 {
      return s;
    }
  } else {
    s = zbc_POSIX_does_not_allow_empty_X_expression_in_for(
      b"init\x00" as *const u8 as *const libc::c_char,
    );
    if s as u64 != 0 {
      return s;
    }
  }
  if (*p).lex as libc::c_int != BC_LEX_SCOLON as libc::c_int {
    return bc_error_bad_token() as BcStatus;
  }
  s = zxc_lex_next();
  if s as u64 != 0 {
    return s;
  }
  cond_idx = bc_vec_push(
    &mut (*(*p).func).labels,
    &mut (*(*p).func).code.len as *mut size_t as *const libc::c_void,
  );
  update_idx = cond_idx.wrapping_add(1i32 as libc::c_ulong);
  body_idx = update_idx.wrapping_add(1i32 as libc::c_ulong);
  exit_idx = body_idx.wrapping_add(1i32 as libc::c_ulong);
  if (*p).lex as libc::c_int != BC_LEX_SCOLON as libc::c_int {
    s = zbc_parse_expr((1i32 << 0i32) as uint8_t)
  } else {
    // Set this for the next call to xc_parse_pushNUM().
    // This is safe to set because the current token is a semicolon,
    // which has no string requirement.
    bc_vec_string(
      &mut (*p).lex_strnumbuf,
      1i32 as size_t,
      b"1\x00" as *const u8 as *const libc::c_char,
    );
    xc_parse_pushNUM();
    s = zbc_POSIX_does_not_allow_empty_X_expression_in_for(
      b"condition\x00" as *const u8 as *const libc::c_char,
    )
  }
  if s as u64 != 0 {
    return s;
  }
  if (*p).lex as libc::c_int != BC_LEX_SCOLON as libc::c_int {
    return bc_error_bad_token() as BcStatus;
  }
  s = zxc_lex_next();
  if s as u64 != 0 {
    return s;
  }
  bc_parse_pushJUMP_ZERO(exit_idx);
  bc_parse_pushJUMP(body_idx);
  bc_vec_push(
    &mut (*p).conds,
    &mut update_idx as *mut size_t as *const libc::c_void,
  );
  bc_vec_push(
    &mut (*(*p).func).labels,
    &mut (*(*p).func).code.len as *mut size_t as *const libc::c_void,
  );
  if (*p).lex as libc::c_int != BC_LEX_RPAREN as libc::c_int {
    s = zbc_parse_expr(0i32 as uint8_t);
    if s as u64 != 0 {
      return s;
    }
    if (*p).lex as libc::c_int != BC_LEX_RPAREN as libc::c_int {
      return bc_error_bad_token() as BcStatus;
    }
    xc_parse_push(XC_INST_POP as libc::c_int as libc::c_uint);
  } else {
    s = zbc_POSIX_does_not_allow_empty_X_expression_in_for(
      b"update\x00" as *const u8 as *const libc::c_char,
    );
    if s as u64 != 0 {
      return s;
    }
  }
  bc_parse_pushJUMP(cond_idx);
  bc_vec_push(
    &mut (*(*p).func).labels,
    &mut (*(*p).func).code.len as *mut size_t as *const libc::c_void,
  );
  bc_vec_push(
    &mut (*p).exits,
    &mut exit_idx as *mut size_t as *const libc::c_void,
  );
  bc_vec_push(
    &mut (*(*p).func).labels,
    &mut exit_idx as *mut size_t as *const libc::c_void,
  );
  s = zbc_parse_stmt_allow_NLINE_before(bc_lex_kws[5].name8.as_ptr());
  if s as u64 != 0 {
    return s;
  }
  bc_parse_pushJUMP(update_idx);
  rewrite_label_to_current(exit_idx);
  bc_vec_pop(&mut (*p).exits);
  bc_vec_pop(&mut (*p).conds);
  return BC_STATUS_SUCCESS;
}
unsafe extern "C" fn zbc_parse_break_or_continue(mut type_0: BcLexType) -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut i: size_t = 0;
  if type_0 as libc::c_uint == BC_LEX_KEY_BREAK as libc::c_int as libc::c_uint {
    if (*p).exits.len == 0i32 as libc::c_ulong {
      // none of the enclosing blocks is a loop
      return bc_error_bad_token() as BcStatus;
    }
    i = *(bc_vec_top(&mut (*p).exits) as *mut size_t)
  } else {
    i = *(bc_vec_top(&mut (*p).conds) as *mut size_t)
  }
  bc_parse_pushJUMP(i);
  return zxc_lex_next();
}
unsafe extern "C" fn zbc_func_insert(
  mut f: *mut BcFunc,
  mut name: *mut libc::c_char,
  mut type_0: BcType,
) -> BcStatus {
  let mut autoid: *mut BcId = 0 as *mut BcId;
  let mut a: BcId = BcId {
    name: 0 as *mut libc::c_char,
    idx: 0,
  };
  let mut i: size_t = 0;
  autoid = (*f).autos.v as *mut libc::c_void as *mut BcId;
  i = 0i32 as size_t;
  while i < (*f).autos.len {
    if strcmp(name, (*autoid).name) == 0i32
      && type_0 as libc::c_uint == (*autoid).idx as BcType as libc::c_uint
    {
      return bc_error(
        b"duplicate function parameter or auto name\x00" as *const u8 as *const libc::c_char,
      ) as BcStatus;
    }
    i = i.wrapping_add(1);
    autoid = autoid.offset(1)
  }
  a.idx = type_0 as size_t;
  a.name = name;
  bc_vec_push(&mut (*f).autos, &mut a as *mut BcId as *const libc::c_void);
  return BC_STATUS_SUCCESS;
}
unsafe extern "C" fn zbc_parse_funcdef() -> BcStatus {
  let mut current_block: u64;
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut comma: bool = false;
  let mut voidfunc: bool = false;
  let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
  s = zxc_lex_next();
  if s as u64 != 0 {
    return s;
  }
  if (*p).lex as libc::c_int != XC_LEX_NAME as libc::c_int {
    return bc_error_bad_function_definition() as BcStatus;
  }
  // To be maximally both POSIX and GNU-compatible,
  // "void" is not treated as a normal keyword:
  // you can have variable named "void", and even a function
  // named "void": "define void() { return 6; }" is ok.
  // _Only_ "define void f() ..." syntax treats "void"
  // specially.
  voidfunc = strcmp(
    (*p).lex_strnumbuf.v,
    b"void\x00" as *const u8 as *const libc::c_char,
  ) == 0i32;
  s = zxc_lex_next();
  if s as u64 != 0 {
    return s;
  }
  voidfunc = voidfunc as libc::c_int != 0 && (*p).lex as libc::c_int == XC_LEX_NAME as libc::c_int;
  if voidfunc {
    s = zxc_lex_next();
    if s as u64 != 0 {
      return s;
    }
  }
  if (*p).lex as libc::c_int != BC_LEX_LPAREN as libc::c_int {
    return bc_error_bad_function_definition() as BcStatus;
  }
  (*p).fidx = bc_program_addFunc(xstrdup((*p).lex_strnumbuf.v));
  (*p).func = xc_program_func((*p).fidx);
  (*(*p).func).voidfunc = voidfunc;
  s = zxc_lex_next();
  if s as u64 != 0 {
    return s;
  }
  comma = 0i32 != 0;
  loop {
    if !((*p).lex as libc::c_int != BC_LEX_RPAREN as libc::c_int) {
      current_block = 16415152177862271243;
      break;
    }
    let mut t: BcType = BC_TYPE_VAR;
    if (*p).lex as libc::c_int == XC_LEX_OP_MULTIPLY as libc::c_int {
      t = BC_TYPE_REF;
      s = zxc_lex_next();
      if s as u64 != 0 {
        return s;
      }
      s = zbc_POSIX_does_not_allow(b"references\x00" as *const u8 as *const libc::c_char);
      if s as u64 != 0 {
        return s;
      }
    }
    if (*p).lex as libc::c_int != XC_LEX_NAME as libc::c_int {
      return bc_error_bad_function_definition() as BcStatus;
    }
    (*(*p).func).nparams = (*(*p).func).nparams.wrapping_add(1);
    name = xstrdup((*p).lex_strnumbuf.v);
    s = zxc_lex_next();
    if s as u64 != 0 {
      current_block = 479107131381816815;
      break;
    }
    if (*p).lex as libc::c_int == BC_LEX_LBRACKET as libc::c_int {
      if t as libc::c_uint == BC_TYPE_VAR as libc::c_int as libc::c_uint {
        t = BC_TYPE_ARRAY
      }
      s = zxc_lex_next();
      if s as u64 != 0 {
        current_block = 479107131381816815;
        break;
      }
      if (*p).lex as libc::c_int != BC_LEX_RBRACKET as libc::c_int {
        s = bc_error_bad_function_definition() as BcStatus;
        current_block = 479107131381816815;
        break;
      } else {
        s = zxc_lex_next();
        if s as u64 != 0 {
          current_block = 479107131381816815;
          break;
        }
      }
    } else if t as libc::c_uint == BC_TYPE_REF as libc::c_int as libc::c_uint {
      s = bc_error_at(b"vars can\'t be references\x00" as *const u8 as *const libc::c_char)
        as BcStatus;
      current_block = 479107131381816815;
      break;
    }
    comma = (*p).lex as libc::c_int == BC_LEX_COMMA as libc::c_int;
    if comma {
      s = zxc_lex_next();
      if s as u64 != 0 {
        current_block = 479107131381816815;
        break;
      }
    }
    s = zbc_func_insert((*p).func, name, t);
    if s as u64 != 0 {
      current_block = 479107131381816815;
      break;
    }
  }
  match current_block {
    479107131381816815 => {
      free(name as *mut libc::c_void);
      return s;
    }
    _ => {
      if comma {
        return bc_error_bad_function_definition() as BcStatus;
      }
      s = zxc_lex_next();
      if s as u64 != 0 {
        return s;
      }
      if (*p).lex as libc::c_int != BC_LEX_LBRACE as libc::c_int {
        s = zbc_POSIX_requires(
          b"the left brace be on the same line as the function header\x00" as *const u8
            as *const libc::c_char,
        );
        if s as u64 != 0 {
          return s;
        }
      }
      // Prevent "define z()<newline>" from being interpreted as function with empty stmt as body
      s = zbc_lex_skip_if_at_NLINE();
      if s as u64 != 0 {
        return s;
      }
      // GNU bc requires a {} block even if function body has single stmt, enforce this
      if (*p).lex as libc::c_int != BC_LEX_LBRACE as libc::c_int {
        return bc_error(b"function { body } expected\x00" as *const u8 as *const libc::c_char)
          as BcStatus;
      } // to determine whether "return" stmt is allowed, and such
      (*p).in_funcdef = (*p).in_funcdef.wrapping_add(1);
      s = zbc_parse_stmt_possibly_auto(1i32 != 0);
      (*p).in_funcdef = (*p).in_funcdef.wrapping_sub(1);
      if s as u64 != 0 {
        return s;
      }
      xc_parse_push(BC_INST_RET0 as libc::c_int as libc::c_uint);
      // Subsequent code generation is into main program
      (*p).fidx = 0i32 as size_t; // skip comma
      (*p).func = (*ptr_to_globals).prog.fns.v as *mut BcFunc;
      return s;
    }
  };
}
unsafe extern "C" fn zbc_parse_auto() -> BcStatus {
  let mut current_block: u64;
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
  s = zxc_lex_next();
  if s as u64 != 0 {
    return s;
  }
  loop {
    let mut t: BcType = BC_TYPE_VAR;
    if (*p).lex as libc::c_int != XC_LEX_NAME as libc::c_int {
      return bc_error_at(b"bad \'auto\' syntax\x00" as *const u8 as *const libc::c_char)
        as BcStatus;
    }
    name = xstrdup((*p).lex_strnumbuf.v);
    s = zxc_lex_next();
    if s as u64 != 0 {
      current_block = 3153325098279936700;
      break;
    }
    t = BC_TYPE_VAR;
    if (*p).lex as libc::c_int == BC_LEX_LBRACKET as libc::c_int {
      t = BC_TYPE_ARRAY;
      s = zxc_lex_next();
      if s as u64 != 0 {
        current_block = 3153325098279936700;
        break;
      }
      if (*p).lex as libc::c_int != BC_LEX_RBRACKET as libc::c_int {
        s = bc_error_at(b"bad \'auto\' syntax\x00" as *const u8 as *const libc::c_char) as BcStatus;
        current_block = 3153325098279936700;
        break;
      } else {
        s = zxc_lex_next();
        if s as u64 != 0 {
          current_block = 3153325098279936700;
          break;
        }
      }
    }
    s = zbc_func_insert((*p).func, name, t);
    if s as u64 != 0 {
      current_block = 3153325098279936700;
      break;
    }
    if (*p).lex as libc::c_int == XC_LEX_NLINE as libc::c_int
      || (*p).lex as libc::c_int == BC_LEX_SCOLON as libc::c_int
    {
      current_block = 17788412896529399552;
      break;
    }
    if (*p).lex as libc::c_int != BC_LEX_COMMA as libc::c_int {
      return bc_error_at(b"bad \'auto\' syntax\x00" as *const u8 as *const libc::c_char)
        as BcStatus;
    }
    s = zxc_lex_next();
    if s as u64 != 0 {
      return s;
    }
  }
  match current_block {
    17788412896529399552 =>
    //|| p->lex == BC_LEX_RBRACE // allow "define f() {auto a}"
    {
      return BC_STATUS_SUCCESS
    }
    _ => {
      free(name as *mut libc::c_void);
      return s;
    }
  };
}
unsafe extern "C" fn zbc_parse_stmt_possibly_auto(mut auto_allowed: bool) -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  if (*p).lex as libc::c_int == XC_LEX_NLINE as libc::c_int {
    return s;
  }
  if (*p).lex as libc::c_int == BC_LEX_SCOLON as libc::c_int {
    return s;
  }
  if (*p).lex as libc::c_int == BC_LEX_LBRACE as libc::c_int {
    loop {
      s = zxc_lex_next();
      if s as u64 != 0 {
        return s;
      }
      if !((*p).lex as libc::c_int == XC_LEX_NLINE as libc::c_int) {
        break;
      }
    }
    if auto_allowed as libc::c_int != 0 && (*p).lex as libc::c_int == BC_LEX_KEY_AUTO as libc::c_int
    {
      s = zbc_parse_auto();
      if s as u64 != 0 {
        return s;
      }
    }
    while (*p).lex as libc::c_int != BC_LEX_RBRACE as libc::c_int {
      s = zbc_parse_stmt();
      if s as u64 != 0 {
        return s;
      }
      // Check that next token is a correct stmt delimiter -
      // disallows "print 1 print 2" and such.
      if (*p).lex as libc::c_int == BC_LEX_RBRACE as libc::c_int {
        break;
      }
      if (*p).lex as libc::c_int != BC_LEX_SCOLON as libc::c_int
        && (*p).lex as libc::c_int != XC_LEX_NLINE as libc::c_int
      {
        return bc_error_at(b"bad statement terminator\x00" as *const u8 as *const libc::c_char)
          as BcStatus;
      }
      s = zxc_lex_next();
      if s as u64 != 0 {
        return s;
      }
    }
    s = zxc_lex_next();
    return s;
  }
  match (*p).lex as libc::c_int {
    19 | 30 | 31 | 20 | 32 | 5 | 6 | 47 | 50 | 51 | 48 | 55 | 57 | 58 => {
      s = zbc_parse_expr((1i32 << 1i32) as uint8_t)
    }
    4 => {
      s = zbc_parse_pushSTR();
      xc_parse_push(XC_INST_PRINT_STR as libc::c_int as libc::c_uint);
    }
    41 | 42 => s = zbc_parse_break_or_continue((*p).lex as BcLexType),
    45 => s = zbc_parse_for(),
    46 => {
      xc_parse_push(BC_INST_HALT as libc::c_int as libc::c_uint);
      s = zxc_lex_next()
    }
    49 => s = zbc_parse_if(),
    52 => {
      // "limits" is a compile-time command,
      // the output is produced at _parse time_.
      printf(b"BC_BASE_MAX     = 999\nBC_DIM_MAX      = 2147483647\nBC_SCALE_MAX    = 4294967295\nBC_STRING_MAX   = 4294967294\nMAX Exponent    = 999999999\nNumber of vars  = 999999999\n\x00"
                       as *const u8 as *const libc::c_char);
      s = zxc_lex_next()
    }
    53 => s = zbc_parse_print(),
    54 => {
      // "quit" is a compile-time command. For example,
      // "if (0 == 1) quit" terminates when parsing the statement,
      // not when it is executed
      quit();
    }
    56 => {
      if (*p).in_funcdef == 0 {
        return bc_error(b"\'return\' not in a function\x00" as *const u8 as *const libc::c_char)
          as BcStatus;
      }
      s = zbc_parse_return()
    }
    59 => s = zbc_parse_while(),
    _ => s = bc_error_bad_token() as BcStatus,
  }
  return s;
}
unsafe extern "C" fn zbc_parse_stmt_or_funcdef() -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  //why?
  //	if (p->lex == XC_LEX_EOF)
  //		s = bc_error("end of file");
  //	else
  if (*p).lex as libc::c_int == BC_LEX_KEY_DEFINE as libc::c_int {
    s = zbc_parse_funcdef()
  } else {
    s = zbc_parse_stmt()
  }
  return s;
}
// We can calculate the conversion between tokens and exprs by subtracting the
// position of the first operator in the lex enum and adding the position of the
// first in the expr enum. Note: This only works for binary operators.
unsafe extern "C" fn zbc_parse_expr(mut flags: uint8_t) -> BcStatus {
  let mut current_block: u64;
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut prev: BcInst = XC_INST_PRINT;
  let mut nexprs: size_t = 0i32 as size_t;
  let mut ops_bgn: size_t = (*p).ops.len;
  let mut nparens: libc::c_uint = 0;
  let mut nrelops: libc::c_uint = 0;
  let mut paren_first: bool = false;
  let mut rprn: bool = false;
  let mut assign: bool = false;
  let mut bin_last: bool = false;
  let mut incdec: bool = false;
  paren_first = (*p).lex as libc::c_int == BC_LEX_LPAREN as libc::c_int;
  nrelops = 0i32 as libc::c_uint;
  nparens = nrelops;
  incdec = 0i32 != 0;
  assign = incdec;
  rprn = assign;
  bin_last = 1i32 != 0;
  loop {
    let mut get_token: bool = false;
    let mut s: BcStatus = BC_STATUS_SUCCESS;
    let mut t: BcLexType = (*p).lex as BcLexType;
    if lex_allowed_in_bc_expr(t as libc::c_uint) == 0 {
      break;
    }
    get_token = 0i32 != 0;
    s = BC_STATUS_SUCCESS;
    match t as libc::c_uint {
      30 | 31 => {
        if incdec {
          return bc_error_bad_assignment() as BcStatus;
        }
        s = zbc_parse_incdec(&mut prev, &mut nexprs, flags);
        incdec = 1i32 != 0;
        bin_last = 0i32 != 0;
        rprn = bin_last;
        current_block = 14483658890531361756;
      }
      19 => {
        s = zbc_parse_minus(&mut prev, ops_bgn, rprn, bin_last, &mut nexprs);
        rprn = 0i32 != 0;
        //get_token = false; - already is
        bin_last = prev as libc::c_int == XC_INST_MINUS as libc::c_int;
        if bin_last {
          incdec = 0i32 != 0
        }
        current_block = 14483658890531361756;
      }
      23 | 24 | 25 | 26 | 27 | 28 | 29 => {
        if prev as libc::c_int != XC_INST_VAR as libc::c_int
          && prev as libc::c_int != XC_INST_ARRAY_ELEM as libc::c_int
          && prev as libc::c_int != XC_INST_SCALE as libc::c_int
          && prev as libc::c_int != XC_INST_IBASE as libc::c_int
          && prev as libc::c_int != XC_INST_OBASE as libc::c_int
          && prev as libc::c_int != BC_INST_LAST as libc::c_int
        {
          return bc_error_bad_assignment() as BcStatus;
        }
        current_block = 3123434771885419771;
      }
      14 | 15 | 16 | 17 | 18 | 8 | 9 | 10 | 11 | 12 | 13 | 20 | 21 | 22 => {
        current_block = 3123434771885419771;
      }
      32 => {
        if !bin_last && (rprn as libc::c_int != 0 || bc_parse_inst_isLeaf(prev) != 0) {
          return bc_error_bad_expression() as BcStatus;
        }
        bc_vec_push(
          &mut (*p).ops,
          &mut t as *mut BcLexType as *const libc::c_void,
        );
        nparens = nparens.wrapping_add(1);
        get_token = 1i32 != 0;
        incdec = 0i32 != 0;
        rprn = incdec;
        current_block = 14483658890531361756;
      }
      33 => {
        //why?
        //			if (p->lex_last == BC_LEX_LPAREN) {
        //				RETURN_STATUS(bc_error_at("empty expression"));
        //			}
        if bin_last as libc::c_int != 0 || prev as libc::c_int == XC_INST_BOOL_NOT as libc::c_int {
          return bc_error_bad_expression() as BcStatus;
        }
        if nparens == 0i32 as libc::c_uint {
          break;
        }
        s = zbc_parse_rightParen(ops_bgn, &mut nexprs);
        nparens = nparens.wrapping_sub(1);
        get_token = 1i32 != 0;
        rprn = 1i32 != 0;
        incdec = 0i32 != 0;
        bin_last = incdec;
        current_block = 14483658890531361756;
      }
      5 => {
        if !bin_last && (rprn as libc::c_int != 0 || bc_parse_inst_isLeaf(prev) != 0) {
          return bc_error_bad_expression() as BcStatus;
        }
        s = zbc_parse_name(
          &mut prev,
          (flags as libc::c_int & !(1i32 << 3i32)) as uint8_t,
        );
        rprn = prev as libc::c_int == BC_INST_CALL as libc::c_int;
        bin_last = 0i32 != 0;
        //get_token = false; - already is
        nexprs = nexprs.wrapping_add(1);
        current_block = 14483658890531361756;
      }
      6 => {
        if !bin_last && (rprn as libc::c_int != 0 || bc_parse_inst_isLeaf(prev) != 0) {
          return bc_error_bad_expression() as BcStatus;
        }
        xc_parse_pushNUM();
        prev = XC_INST_NUM;
        get_token = 1i32 != 0;
        bin_last = 0i32 != 0;
        rprn = bin_last;
        nexprs = nexprs.wrapping_add(1);
        current_block = 14483658890531361756;
      }
      47 | 50 | 48 => {
        if !bin_last && (rprn as libc::c_int != 0 || bc_parse_inst_isLeaf(prev) != 0) {
          return bc_error_bad_expression() as BcStatus;
        }
        prev = (t as libc::c_uint)
          .wrapping_sub(BC_LEX_KEY_IBASE as libc::c_int as libc::c_uint)
          .wrapping_add(XC_INST_IBASE as libc::c_int as libc::c_uint) as libc::c_char
          as BcInst;
        xc_parse_push(prev as libc::c_char as libc::c_uint);
        get_token = 1i32 != 0;
        bin_last = 0i32 != 0;
        rprn = bin_last;
        nexprs = nexprs.wrapping_add(1);
        current_block = 14483658890531361756;
      }
      51 | 58 => {
        if !bin_last && (rprn as libc::c_int != 0 || bc_parse_inst_isLeaf(prev) != 0) {
          return bc_error_bad_expression() as BcStatus;
        }
        s = zbc_parse_builtin(t, flags, &mut prev);
        get_token = 1i32 != 0;
        incdec = 0i32 != 0;
        bin_last = incdec;
        rprn = bin_last;
        nexprs = nexprs.wrapping_add(1);
        current_block = 14483658890531361756;
      }
      55 => {
        if !bin_last && (rprn as libc::c_int != 0 || bc_parse_inst_isLeaf(prev) != 0) {
          return bc_error_bad_expression() as BcStatus;
        }
        s = zbc_parse_read();
        prev = XC_INST_READ;
        get_token = 1i32 != 0;
        incdec = 0i32 != 0;
        bin_last = incdec;
        rprn = bin_last;
        nexprs = nexprs.wrapping_add(1);
        current_block = 14483658890531361756;
      }
      57 => {
        if !bin_last && (rprn as libc::c_int != 0 || bc_parse_inst_isLeaf(prev) != 0) {
          return bc_error_bad_expression() as BcStatus;
        }
        s = zbc_parse_scale(&mut prev, flags);
        //get_token = false; - already is
        bin_last = 0i32 != 0;
        rprn = bin_last;
        nexprs = nexprs.wrapping_add(1);
        current_block = 14483658890531361756;
      }
      _ => return bc_error_bad_token() as BcStatus,
    }
    match current_block {
      3123434771885419771 =>
      // Fallthrough.
      {
        if t as libc::c_uint == BC_LEX_OP_BOOL_NOT as libc::c_int as libc::c_uint {
          if !bin_last && (*p).lex_last as libc::c_int != BC_LEX_OP_BOOL_NOT as libc::c_int {
            return bc_error_bad_expression() as BcStatus;
          }
        } else if prev as libc::c_int == XC_INST_BOOL_NOT as libc::c_int {
          return bc_error_bad_expression() as BcStatus;
        }
        nrelops = nrelops.wrapping_add(
          (t as libc::c_uint >= XC_LEX_OP_REL_EQ as libc::c_int as libc::c_uint
            && t as libc::c_uint <= XC_LEX_OP_REL_GT as libc::c_int as libc::c_uint)
            as libc::c_int as libc::c_uint,
        );
        prev = (t as libc::c_uint)
          .wrapping_sub(XC_LEX_OP_POWER as libc::c_int as libc::c_uint)
          .wrapping_add(XC_INST_POWER as libc::c_int as libc::c_uint) as libc::c_char
          as BcInst;
        bc_parse_operator(t, ops_bgn, &mut nexprs);
        incdec = 0i32 != 0;
        rprn = incdec;
        get_token = 1i32 != 0;
        bin_last = t as libc::c_uint != BC_LEX_OP_BOOL_NOT as libc::c_int as libc::c_uint
      }
      _ => {}
    }
    //get_token = false; - already is
    if s as libc::c_uint != 0 || bb_got_signal as libc::c_int != 0 {
      // error, or ^C: stop parsing
      return BC_STATUS_FAILURE;
    }
    if get_token {
      s = zxc_lex_next();
      if s as u64 != 0 {
        return s;
      }
    }
  }
  while (*p).ops.len > ops_bgn {
    let mut top: BcLexType = *(bc_vec_top(&mut (*p).ops) as *mut BcLexType);
    assign = top as libc::c_uint >= BC_LEX_OP_ASSIGN_POWER as libc::c_int as libc::c_uint
      && top as libc::c_uint <= BC_LEX_OP_ASSIGN as libc::c_int as libc::c_uint;
    if top as libc::c_uint == BC_LEX_LPAREN as libc::c_int as libc::c_uint
      || top as libc::c_uint == BC_LEX_RPAREN as libc::c_int as libc::c_uint
    {
      return bc_error_bad_expression() as BcStatus;
    }
    xc_parse_push(
      (top as libc::c_uint)
        .wrapping_sub(XC_LEX_OP_POWER as libc::c_int as libc::c_uint)
        .wrapping_add(XC_INST_POWER as libc::c_int as libc::c_uint) as libc::c_char
        as libc::c_uint,
    );
    nexprs = (nexprs as libc::c_ulong).wrapping_sub(
      (top as libc::c_uint != BC_LEX_OP_BOOL_NOT as libc::c_int as libc::c_uint
        && top as libc::c_uint != XC_LEX_NEG as libc::c_int as libc::c_uint) as libc::c_int
        as libc::c_ulong,
    ) as size_t as size_t;
    bc_vec_pop(&mut (*p).ops);
  }
  if prev as libc::c_int == XC_INST_BOOL_NOT as libc::c_int || nexprs != 1i32 as libc::c_ulong {
    return bc_error_bad_expression() as BcStatus;
  }
  if flags as libc::c_int & 1i32 << 0i32 == 0 && nrelops != 0 {
    let mut s_0: BcStatus = BC_STATUS_SUCCESS;
    s_0 = zbc_POSIX_does_not_allow(
      b"comparison operators outside if or loops\x00" as *const u8 as *const libc::c_char,
    );
    if s_0 as u64 != 0 {
      return s_0;
    }
  } else if flags as libc::c_int & 1i32 << 0i32 != 0 && nrelops > 1i32 as libc::c_uint {
    let mut s_1: BcStatus = BC_STATUS_SUCCESS;
    s_1 = zbc_POSIX_requires(
      b"exactly one comparison operator per condition\x00" as *const u8 as *const libc::c_char,
    );
    if s_1 as u64 != 0 {
      return s_1;
    }
  }
  if flags as libc::c_int & 1i32 << 1i32 != 0 {
    if paren_first as libc::c_int != 0 || !assign {
      xc_parse_push(XC_INST_PRINT as libc::c_int as libc::c_uint);
    }
    xc_parse_push(XC_INST_POP as libc::c_int as libc::c_uint);
  }
  return BC_STATUS_SUCCESS;
}
// ENABLE_BC
unsafe extern "C" fn zdc_parse_register() -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  s = zxc_lex_next();
  if s as u64 != 0 {
    return s;
  }
  if (*p).lex as libc::c_int != XC_LEX_NAME as libc::c_int {
    return bc_error_bad_token() as BcStatus;
  }
  xc_parse_pushName((*p).lex_strnumbuf.v);
  return s;
}
unsafe extern "C" fn dc_parse_string() {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut len: size_t = (*ptr_to_globals).prog.strs.len;
  str = xstrdup((*p).lex_strnumbuf.v);
  xc_parse_pushInst_and_Index(XC_INST_STR as libc::c_int as libc::c_uint, len);
  bc_vec_push(
    &mut (*ptr_to_globals).prog.strs,
    &mut str as *mut *mut libc::c_char as *const libc::c_void,
  );
  // Add an empty function so that if zdc_program_execStr ever needs to
  // parse the string into code (from the 'x' command) there's somewhere
  // to store the bytecode.
  xc_program_add_fn();
  (*p).func = xc_program_func((*p).fidx);
}
unsafe extern "C" fn zdc_parse_mem(mut inst: uint8_t, mut name: bool, mut store: bool) -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  xc_parse_push(inst as libc::c_uint);
  if name {
    s = zdc_parse_register();
    if s as u64 != 0 {
      return s;
    }
  }
  if store {
    xc_parse_push(DC_INST_SWAP as libc::c_int as libc::c_uint);
    xc_parse_push(XC_INST_ASSIGN as libc::c_int as libc::c_uint);
    xc_parse_push(XC_INST_POP as libc::c_int as libc::c_uint);
  }
  return BC_STATUS_SUCCESS;
}
unsafe extern "C" fn zdc_parse_cond(mut inst: uint8_t) -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  xc_parse_push(inst as libc::c_uint);
  xc_parse_push(DC_INST_EXEC_COND as libc::c_int as libc::c_uint);
  s = zdc_parse_register();
  if s as u64 != 0 {
    return s;
  }
  s = zxc_lex_next();
  if s as u64 != 0 {
    return s;
  }
  // Note that 'else' part can not be on the next line:
  // echo -e '[1p]sa [2p]sb 2 1>a eb' | dc - OK, prints "2"
  // echo -e '[1p]sa [2p]sb 2 1>a\neb' | dc - parse error
  if (*p).lex as libc::c_int == DC_LEX_ELSE as libc::c_int {
    s = zdc_parse_register();
    if s as u64 != 0 {
      return s;
    }
    s = zxc_lex_next()
  } else {
    xc_parse_push('\u{0}' as i32 as libc::c_uint);
  }
  return s;
}
unsafe extern "C" fn zdc_parse_token(mut t: BcLexType) -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut inst: uint8_t = 0;
  let mut assign: bool = false;
  let mut get_token: bool = false;
  s = BC_STATUS_SUCCESS;
  get_token = 1i32 != 0;
  match t as libc::c_uint {
    8 | 9 | 10 | 11 | 12 | 13 => {
      s = zdc_parse_cond(
        (t as libc::c_uint)
          .wrapping_sub(XC_LEX_OP_REL_EQ as libc::c_int as libc::c_uint)
          .wrapping_add(XC_INST_REL_EQ as libc::c_int as libc::c_uint) as uint8_t,
      );
      get_token = 0i32 != 0
    }
    23 | 36 => {
      s = zdc_parse_mem(
        XC_INST_ARRAY_ELEM as libc::c_int as uint8_t,
        1i32 != 0,
        t as libc::c_uint == DC_LEX_COLON as libc::c_int as libc::c_uint,
      )
    }
    4 => {
      dc_parse_string();
    }
    7 => {
      s = zxc_lex_next();
      if s as u64 != 0 {
        return s;
      }
      if (*ptr_to_globals).prs.lex as libc::c_int != XC_LEX_NUMBER as libc::c_int {
        return bc_error_bad_token() as BcStatus;
      }
      xc_parse_pushNUM();
      xc_parse_push(XC_INST_NEG as libc::c_int as libc::c_uint);
    }
    6 => {
      xc_parse_pushNUM();
    }
    24 => {
      xc_parse_push(XC_INST_READ as libc::c_int as libc::c_uint);
    }
    21 | 52 => {
      assign = t as libc::c_uint == DC_LEX_OP_ASSIGN as libc::c_int as libc::c_uint;
      inst = if assign as libc::c_int != 0 {
        XC_INST_VAR as libc::c_int
      } else {
        DC_INST_PUSH_TO_VAR as libc::c_int
      } as uint8_t;
      s = zdc_parse_mem(inst, 1i32 != 0, assign)
    }
    50 | 51 => {
      inst = if t as libc::c_uint == DC_LEX_LOAD_POP as libc::c_int as libc::c_uint {
        DC_INST_PUSH_VAR as libc::c_int
      } else {
        DC_INST_LOAD as libc::c_int
      } as uint8_t;
      s = zdc_parse_mem(inst, 1i32 != 0, 0i32 != 0)
    }
    47 | 49 | 48 => {
      inst = (t as libc::c_uint)
        .wrapping_sub(DC_LEX_STORE_IBASE as libc::c_int as libc::c_uint)
        .wrapping_add(XC_INST_IBASE as libc::c_int as libc::c_uint) as uint8_t;
      s = zdc_parse_mem(inst, 0i32 != 0, 1i32 != 0)
    }
    _ => return bc_error_bad_token() as BcStatus,
  }
  if s as u64 == 0 && get_token as libc::c_int != 0 {
    s = zxc_lex_next()
  }
  return s;
}
unsafe extern "C" fn zdc_parse_expr() -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  let mut i: libc::c_int = 0;
  if (*p).lex as libc::c_int == XC_LEX_NLINE as libc::c_int {
    return zxc_lex_next();
  }
  i = (*p).lex as libc::c_int - XC_LEX_OP_POWER as libc::c_int;
  if i >= 0i32 {
    let mut inst: BcInst = dc_LEX_to_INST[i as usize] as BcInst;
    if inst as libc::c_int != DC_INST_INVALID as libc::c_int {
      xc_parse_push(inst as libc::c_uint);
      return zxc_lex_next();
    }
  }
  return zdc_parse_token((*p).lex as BcLexType);
}
unsafe extern "C" fn zdc_parse_exprs_until_eof() -> BcStatus {
  let mut p: *mut BcParse = &mut (*ptr_to_globals).prs;
  while (*p).lex as libc::c_int != XC_LEX_EOF as libc::c_int {
    let mut s: BcStatus = zdc_parse_expr();
    if s as u64 != 0 {
      return s;
    }
  }
  return BC_STATUS_SUCCESS;
}
// ENABLE_DC
//
// Execution engine
//
unsafe extern "C" fn xc_program_index(mut code: *mut libc::c_char, mut bgn: *mut size_t) -> size_t {
  let mut bytes: *mut libc::c_uchar =
    code.offset(*bgn as isize) as *mut libc::c_void as *mut libc::c_uchar; // amt is 1 or more here
  let mut amt: libc::c_uint = 0;
  let mut i: libc::c_uint = 0;
  let mut res: size_t = 0;
  let fresh14 = bytes;
  bytes = bytes.offset(1);
  amt = *fresh14 as libc::c_uint;
  if (amt as libc::c_ulong)
    < (0x100i32 as libc::c_ulong).wrapping_sub(::std::mem::size_of::<size_t>() as libc::c_ulong)
  {
    *bgn = (*bgn as libc::c_ulong).wrapping_add(1i32 as libc::c_ulong) as size_t as size_t;
    return amt as size_t;
  }
  amt = (amt as libc::c_ulong).wrapping_sub(
    (0x100i32 as libc::c_ulong)
      .wrapping_sub(::std::mem::size_of::<size_t>() as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong),
  ) as libc::c_uint as libc::c_uint;
  *bgn = (*bgn as libc::c_ulong)
    .wrapping_add(amt.wrapping_add(1i32 as libc::c_uint) as libc::c_ulong) as size_t
    as size_t;
  res = 0i32 as size_t;
  i = 0i32 as libc::c_uint;
  loop {
    let fresh15 = bytes;
    bytes = bytes.offset(1);
    res |= (*fresh15 as size_t) << i;
    i = i.wrapping_add(8i32 as libc::c_uint);
    amt = amt.wrapping_sub(1);
    if !(amt != 0i32 as libc::c_uint) {
      break;
    }
  }
  return res;
}
unsafe extern "C" fn xc_program_name(
  mut code: *mut libc::c_char,
  mut bgn: *mut size_t,
) -> *mut libc::c_char {
  code = code.offset(*bgn as isize);
  *bgn = (*bgn as libc::c_ulong).wrapping_add(strlen(code).wrapping_add(1i32 as libc::c_ulong))
    as size_t as size_t;
  return xstrdup(code);
}
unsafe extern "C" fn xc_program_dereference(mut vec: *mut BcVec) -> *mut BcVec {
  let mut v: *mut BcVec = 0 as *mut BcVec;
  let mut vidx: size_t = 0;
  let mut nidx: size_t = 0;
  let mut i: size_t = 0i32 as size_t;
  //assert(vec->size == sizeof(uint8_t));
  vidx = xc_program_index((*vec).v, &mut i);
  nidx = xc_program_index((*vec).v, &mut i);
  v = bc_vec_item(&mut (*ptr_to_globals).prog.arrs, vidx) as *mut BcVec;
  v = bc_vec_item(v, nidx) as *mut BcVec;
  //assert(v->size != sizeof(uint8_t));
  return v; // 1 if insertion was successful
}
unsafe extern "C" fn xc_program_search(
  mut id: *mut libc::c_char,
  mut type_0: BcType,
) -> *mut BcVec {
  let mut e: BcId = BcId {
    name: 0 as *mut libc::c_char,
    idx: 0,
  };
  let mut ptr: *mut BcId = 0 as *mut BcId;
  let mut v: *mut BcVec = 0 as *mut BcVec;
  let mut map: *mut BcVec = 0 as *mut BcVec;
  let mut i: size_t = 0;
  let mut new: libc::c_int = 0;
  let mut var: bool = type_0 as libc::c_uint == BC_TYPE_VAR as libc::c_int as libc::c_uint;
  v = if var as libc::c_int != 0 {
    &mut (*ptr_to_globals).prog.vars
  } else {
    &mut (*ptr_to_globals).prog.arrs
  };
  map = if var as libc::c_int != 0 {
    &mut (*ptr_to_globals).prog.var_map
  } else {
    &mut (*ptr_to_globals).prog.arr_map
  };
  e.name = id;
  e.idx = (*v).len;
  new = bc_map_insert(map, &mut e as *mut BcId as *const libc::c_void, &mut i);
  if new != 0 {
    let mut v2: BcVec = BcVec {
      v: 0 as *mut libc::c_char,
      len: 0,
      cap: 0,
      size: 0,
      dtor: None,
    };
    bc_array_init(&mut v2, var);
    bc_vec_push(v, &mut v2 as *mut BcVec as *const libc::c_void);
  }
  ptr = bc_vec_item(map, i) as *mut BcId;
  if new != 0 {
    (*ptr).name = xstrdup(e.name)
  }
  return bc_vec_item(v, (*ptr).idx) as *mut BcVec;
}
// 'num' need not be initialized on entry
unsafe extern "C" fn zxc_program_num(mut r: *mut BcResult, mut num: *mut *mut BcNum) -> BcStatus {
  match (*r).t as libc::c_uint {
    5 | 0 | 1 | 6 | 8 | 7 => *num = &mut (*r).d.n,
    10 => {
      let mut s: BcStatus = BC_STATUS_SUCCESS;
      let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
      let mut len: size_t = 0;
      str = *xc_program_const((*r).d.id.idx);
      len = strlen(str);
      bc_num_init(&mut (*r).d.n, len);
      s = zxc_num_parse(
        &mut (*r).d.n,
        str,
        (*ptr_to_globals).prog.ib_t as libc::c_uint,
      );
      if s as u64 != 0 {
        bc_num_free(&mut (*r).d.n as *mut BcNum as *mut libc::c_void);
        return s;
      }
      *num = &mut (*r).d.n;
      (*r).t = XC_RESULT_TEMP
    }
    2 | 4 | 3 => {
      let mut type_0: BcType =
        if (*r).t as libc::c_uint == XC_RESULT_VAR as libc::c_int as libc::c_uint {
          BC_TYPE_VAR as libc::c_int
        } else {
          BC_TYPE_ARRAY as libc::c_int
        } as BcType;
      let mut v: *mut BcVec = xc_program_search((*r).d.id.name, type_0);
      let mut p: *mut libc::c_void = bc_vec_top(v);
      if (*r).t as libc::c_uint == XC_RESULT_ARRAY_ELEM as libc::c_int as libc::c_uint {
        let mut idx: size_t = (*r).d.id.idx;
        v = p as *mut BcVec;
        if (*v).size == ::std::mem::size_of::<uint8_t>() as libc::c_ulong {
          v = xc_program_dereference(v)
        }
        //assert(v->size == sizeof(BcNum));
        if (*v).len <= idx {
          bc_array_expand(v, idx.wrapping_add(1i32 as libc::c_ulong));
        }
        *num = bc_vec_item(v, idx) as *mut BcNum
      } else {
        *num = p as *mut BcNum
      }
    }
    9 => *num = &mut (*ptr_to_globals).prog.last,
    11 => *num = &mut (*ptr_to_globals).prog.one,
    _ => {
      // Testing the theory that dc does not reach LAST/ONE
      bb_error_msg_and_die(
        b"BUG:%d\x00" as *const u8 as *const libc::c_char,
        (*r).t as libc::c_uint,
      );
    }
  }
  return BC_STATUS_SUCCESS;
}
unsafe extern "C" fn zxc_program_binOpPrep(
  mut l: *mut *mut BcResult,
  mut ln: *mut *mut BcNum,
  mut r: *mut *mut BcResult,
  mut rn: *mut *mut BcNum,
  mut assign: bool,
) -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut lt: BcResultType = XC_RESULT_TEMP;
  let mut rt: BcResultType = XC_RESULT_TEMP;
  if !((*ptr_to_globals).prog.results.len > 1i32 as size_t) {
    return bc_error_stack_has_too_few_elements() as BcStatus;
  }
  *r = bc_vec_item_rev(&mut (*ptr_to_globals).prog.results, 0i32 as size_t) as *mut BcResult;
  *l = bc_vec_item_rev(&mut (*ptr_to_globals).prog.results, 1i32 as size_t) as *mut BcResult;
  s = zxc_program_num(*l, ln);
  if s as u64 != 0 {
    return s;
  }
  s = zxc_program_num(*r, rn);
  if s as u64 != 0 {
    return s;
  }
  lt = (**l).t;
  rt = (**r).t;
  // We run this again under these conditions in case any vector has been
  // reallocated out from under the BcNums or arrays we had.
  if lt as libc::c_uint == rt as libc::c_uint
    && (lt as libc::c_uint == XC_RESULT_VAR as libc::c_int as libc::c_uint
      || lt as libc::c_uint == XC_RESULT_ARRAY_ELEM as libc::c_int as libc::c_uint)
  {
    s = zxc_program_num(*l, ln);
    if s as u64 != 0 {
      return s;
    }
  }
  if !((**l).t as libc::c_uint != XC_RESULT_ARRAY as libc::c_int as libc::c_uint
    && (**l).t as libc::c_uint != XC_RESULT_STR as libc::c_int as libc::c_uint
    && !((**ln).num.is_null() && (**ln).cap == 0))
    && (!assign || (**l).t as libc::c_uint != XC_RESULT_VAR as libc::c_int as libc::c_uint)
  {
    return bc_error_variable_is_wrong_type() as BcStatus;
  }
  if !assign
    && !((**r).t as libc::c_uint != XC_RESULT_ARRAY as libc::c_int as libc::c_uint
      && (**r).t as libc::c_uint != XC_RESULT_STR as libc::c_int as libc::c_uint
      && !((**ln).num.is_null() && (**ln).cap == 0))
  {
    return bc_error_variable_is_wrong_type() as BcStatus;
  }
  return s;
}
unsafe extern "C" fn xc_program_binOpRetire(mut r: *mut BcResult) {
  (*r).t = XC_RESULT_TEMP;
  bc_vec_pop(&mut (*ptr_to_globals).prog.results);
  bc_result_pop_and_push(r as *const libc::c_void);
}
// Note: *r and *n need not be initialized by caller
unsafe extern "C" fn zxc_program_prep(
  mut r: *mut *mut BcResult,
  mut n: *mut *mut BcNum,
) -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS; // struct copy
  if !((*ptr_to_globals).prog.results.len > 0i32 as size_t) {
    return bc_error_stack_has_too_few_elements() as BcStatus;
  }
  *r = bc_vec_top(&mut (*ptr_to_globals).prog.results) as *mut BcResult;
  s = zxc_program_num(*r, n);
  if s as u64 != 0 {
    return s;
  }
  if !((**r).t as libc::c_uint != XC_RESULT_ARRAY as libc::c_int as libc::c_uint
    && (**r).t as libc::c_uint != XC_RESULT_STR as libc::c_int as libc::c_uint
    && !((**n).num.is_null() && (**n).cap == 0))
  {
    return bc_error_variable_is_wrong_type() as BcStatus;
  }
  return s;
}
unsafe extern "C" fn xc_program_retire(mut r: *mut BcResult, mut t: BcResultType) {
  (*r).t = t;
  bc_result_pop_and_push(r as *const libc::c_void);
}
unsafe extern "C" fn zxc_program_op(mut inst: libc::c_char) -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut opd1: *mut BcResult = 0 as *mut BcResult;
  let mut opd2: *mut BcResult = 0 as *mut BcResult;
  let mut res: BcResult = BcResult {
    t: XC_RESULT_TEMP,
    d: BcResultData {
      n: BcNum {
        num: 0 as *mut BcDig,
        rdx: 0,
        len: 0,
        cap: 0,
        neg: false,
      },
    },
  };
  let mut n1: *mut BcNum = 0 as *mut BcNum;
  let mut n2: *mut BcNum = 0 as *mut BcNum;
  s = zxc_program_binOpPrep(&mut opd1, &mut n1, &mut opd2, &mut n2, 0i32 != 0);
  if s as u64 != 0 {
    return s;
  }
  bc_num_init_DEF_SIZE(&mut res.d.n);
  s = BC_STATUS_SUCCESS;
  s = zxc_program_ops[(inst as libc::c_int - XC_INST_POWER as libc::c_int) as usize]
    .expect("non-null function pointer")(n1, n2, &mut res.d.n, (*ptr_to_globals).prog.scale);
  if s as u64 != 0 {
    bc_num_free(&mut res.d.n as *mut BcNum as *mut libc::c_void);
    return s;
  } else {
    xc_program_binOpRetire(&mut res);
    return s;
  };
}
unsafe extern "C" fn zxc_program_read() -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut sv_parse: BcParse = BcParse {
    lex: 0,
    lex_last: 0,
    lex_line: 0,
    lex_inbuf: 0 as *const libc::c_char,
    lex_next_at: 0 as *const libc::c_char,
    lex_filename: 0 as *const libc::c_char,
    lex_input_fp: 0 as *mut FILE,
    lex_strnumbuf: BcVec {
      v: 0 as *mut libc::c_char,
      len: 0,
      cap: 0,
      size: 0,
      dtor: None,
    },
    func: 0 as *mut BcFunc,
    fidx: 0,
    in_funcdef: 0,
    exits: BcVec {
      v: 0 as *mut libc::c_char,
      len: 0,
      cap: 0,
      size: 0,
      dtor: None,
    },
    conds: BcVec {
      v: 0 as *mut libc::c_char,
      len: 0,
      cap: 0,
      size: 0,
      dtor: None,
    },
    ops: BcVec {
      v: 0 as *mut libc::c_char,
      len: 0,
      cap: 0,
      size: 0,
      dtor: None,
    },
  };
  let mut buf: BcVec = BcVec {
    v: 0 as *mut libc::c_char,
    len: 0,
    cap: 0,
    size: 0,
    dtor: None,
  };
  let mut ip: BcInstPtr = BcInstPtr {
    func: 0,
    inst_idx: 0,
  };
  let mut f: *mut BcFunc = 0 as *mut BcFunc;
  bc_char_vec_init(&mut buf);
  xc_read_line(&mut buf, stdin);
  f = xc_program_func(1i32 as size_t);
  bc_vec_pop_all(&mut (*f).code);
  sv_parse = (*ptr_to_globals).prs;
  xc_parse_create(1i32 as size_t);
  //G.err_line = G.prs.lex_line = 1; - not needed, error line info is not printed for read()
  s = zxc_parse_text_init(buf.v); // struct copy
  if !(s as u64 != 0) {
    if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(0) as libc::c_int == 'b' as i32) {
      s = zbc_parse_expr(0i32 as uint8_t)
    } else {
      s = zdc_parse_exprs_until_eof()
    }
    if !(s as u64 != 0) {
      if (*ptr_to_globals).prs.lex as libc::c_int != XC_LEX_NLINE as libc::c_int
        && (*ptr_to_globals).prs.lex as libc::c_int != XC_LEX_EOF as libc::c_int
      {
        s =
          bc_error_at(b"bad read() expression\x00" as *const u8 as *const libc::c_char) as BcStatus
      } else {
        xc_parse_push(XC_INST_RET as libc::c_int as libc::c_uint);
        ip.func = 1i32 as size_t;
        ip.inst_idx = 0i32 as size_t;
        bc_vec_push(
          &mut (*ptr_to_globals).prog.exestack,
          &mut ip as *mut BcInstPtr as *const libc::c_void,
        );
      }
    }
  }
  xc_parse_free();
  (*ptr_to_globals).prs = sv_parse;
  bc_vec_free(&mut buf as *mut BcVec as *mut libc::c_void);
  return s;
}
unsafe extern "C" fn xc_program_printString(mut str: *const libc::c_char) {
  if *str.offset(0) == 0
    && (1i32 != 0 && (1i32 == 0 || *applet_name.offset(0) as libc::c_int != 'b' as i32))
  {
    // Example: echo '[]ap' | dc
    // should print two bytes: 0x00, 0x0A
    bb_putchar('\u{0}' as i32); // note: if c is NUL, n = \0 at end of esc
    return;
  }
  while *str != 0 {
    let fresh16 = str;
    str = str.offset(1);
    let mut c: libc::c_char = *fresh16;
    if c as libc::c_int == '\\' as i32 {
      static mut esc: [libc::c_char; 9] = [110, 97, 98, 102, 114, 116, 101, 92, 0];
      let mut n: *mut libc::c_char = 0 as *mut libc::c_char;
      let fresh17 = str;
      str = str.offset(1);
      c = *fresh17;
      n = strchr(esc.as_ptr(), c as libc::c_int);
      if n.is_null() || c == 0 {
        // Just print the backslash and following character
        bb_putchar('\\' as i32);
        (*ptr_to_globals).prog.nchars = (*ptr_to_globals).prog.nchars.wrapping_add(1);
        // But if we're at the end of the string, stop
        if c == 0 {
          break;
        }
      } else {
        if n.wrapping_offset_from(esc.as_ptr()) as libc::c_long == 0i32 as libc::c_long {
          // "\n" ?
          (*ptr_to_globals).prog.nchars = 18446744073709551615u64
        }
        c = (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(
          b"\n\x07\x08\x0c\r\t\\\\\\\x00",
        ))[n.wrapping_offset_from(esc.as_ptr()) as libc::c_long as usize]
        //   n a b f r t   e \   \<end of line>
      }
    }
    putchar_unlocked(c as libc::c_int);
    (*ptr_to_globals).prog.nchars = (*ptr_to_globals).prog.nchars.wrapping_add(1)
  }
}
unsafe extern "C" fn bc_num_printNewline() {
  if (*ptr_to_globals).prog.nchars
    == (*ptr_to_globals)
      .prog
      .len
      .wrapping_sub(1i32 as libc::c_ulong)
  {
    bb_putchar('\\' as i32);
    bb_putchar('\n' as i32);
    (*ptr_to_globals).prog.nchars = 0i32 as size_t
  };
}
unsafe extern "C" fn dc_num_printChar(mut num: size_t, mut width: size_t, mut _radix: bool) {
  bb_putchar(num as libc::c_char as libc::c_int);
  (*ptr_to_globals).prog.nchars =
    ((*ptr_to_globals).prog.nchars as libc::c_ulong).wrapping_add(width) as size_t as size_t;
}
unsafe extern "C" fn bc_num_printDigits(mut num: size_t, mut width: size_t, mut radix: bool) {
  let mut exp: size_t = 0;
  let mut pow: size_t = 0;
  bc_num_printNewline();
  bb_putchar(if radix as libc::c_int != 0 {
    '.' as i32
  } else {
    ' ' as i32
  });
  (*ptr_to_globals).prog.nchars = (*ptr_to_globals).prog.nchars.wrapping_add(1);
  bc_num_printNewline();
  exp = 0i32 as size_t;
  pow = 1i32 as size_t;
  while exp < width.wrapping_sub(1i32 as libc::c_ulong) {
    exp = exp.wrapping_add(1);
    pow = (pow as libc::c_ulong).wrapping_mul(10i32 as libc::c_ulong) as size_t as size_t
  }
  exp = 0i32 as size_t;
  while exp < width {
    let mut dig: size_t = 0;
    bc_num_printNewline();
    dig = num.wrapping_div(pow);
    num = (num as libc::c_ulong).wrapping_sub(dig.wrapping_mul(pow)) as size_t as size_t;
    bb_putchar(dig as libc::c_char as libc::c_int + '0' as i32);
    pow = (pow as libc::c_ulong).wrapping_div(10i32 as libc::c_ulong) as size_t as size_t;
    (*ptr_to_globals).prog.nchars = (*ptr_to_globals).prog.nchars.wrapping_add(1);
    exp = exp.wrapping_add(1)
  }
}
unsafe extern "C" fn bc_num_printHex(mut num: size_t, mut width: size_t, mut radix: bool) {
  if radix {
    bc_num_printNewline();
    bb_putchar('.' as i32);
    (*ptr_to_globals).prog.nchars = (*ptr_to_globals).prog.nchars.wrapping_add(1)
  }
  bc_num_printNewline();
  bb_putchar(*bb_hexdigits_upcase.as_ptr().offset(num as isize) as libc::c_int);
  (*ptr_to_globals).prog.nchars =
    ((*ptr_to_globals).prog.nchars as libc::c_ulong).wrapping_add(width) as size_t as size_t;
}
unsafe extern "C" fn bc_num_printDecimal(mut n: *mut BcNum) {
  let mut i: size_t = 0;
  let mut rdx: size_t = (*n).rdx.wrapping_sub(1i32 as libc::c_ulong);
  if (*n).neg {
    bb_putchar('-' as i32);
    (*ptr_to_globals).prog.nchars = (*ptr_to_globals).prog.nchars.wrapping_add(1)
  }
  i = (*n).len.wrapping_sub(1i32 as libc::c_ulong);
  while i < (*n).len {
    bc_num_printHex(
      *(*n).num.offset(i as isize) as size_t,
      1i32 as size_t,
      i == rdx,
    );
    i = i.wrapping_sub(1)
  }
}
unsafe extern "C" fn zxc_num_printNum(
  mut n: *mut BcNum,
  mut base_t: libc::c_uint,
  mut width: size_t,
  mut print: BcNumDigitOp,
) -> BcStatus {
  let mut current_block: u64;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut stack: BcVec = BcVec {
    v: 0 as *mut libc::c_char,
    len: 0,
    cap: 0,
    size: 0,
    dtor: None,
  };
  let mut base: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut base_digs: [BcDig; 20] = [0; 20];
  let mut intp: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut fracp: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut digit: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut frac_len: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut dig: libc::c_ulong = 0;
  let mut ptr: *mut libc::c_ulong = 0 as *mut libc::c_ulong;
  let mut i: size_t = 0;
  let mut radix: bool = false;
  if (*n).len == 0i32 as libc::c_ulong {
    print.expect("non-null function pointer")(0i32 as size_t, width, 0i32 != 0);
    return BC_STATUS_SUCCESS;
  }
  bc_vec_init(
    &mut stack,
    ::std::mem::size_of::<libc::c_long>() as libc::c_ulong,
    None,
  );
  bc_num_init(&mut intp, (*n).len);
  bc_num_init(&mut fracp, (*n).rdx);
  bc_num_init(&mut digit, width);
  bc_num_init(&mut frac_len, (*n).len.wrapping_sub((*n).rdx));
  bc_num_copy(&mut intp, n);
  bc_num_one(&mut frac_len);
  base.cap = (::std::mem::size_of::<[BcDig; 20]>() as libc::c_ulong)
    .wrapping_div(::std::mem::size_of::<BcDig>() as libc::c_ulong) as libc::c_uint
    as size_t;
  base.num = base_digs.as_mut_ptr();
  bc_num_ulong2num(&mut base, base_t as libc::c_ulong);
  bc_num_truncate(&mut intp, intp.rdx);
  s = zbc_num_sub(n, &mut intp, &mut fracp, 0i32 as size_t);
  if s as u64 != 0 {
    current_block = 15705001814182199965;
  } else {
    current_block = 2370887241019905314;
  }
  loop {
    match current_block {
      15705001814182199965 => {
        bc_num_free(&mut frac_len as *mut BcNum as *mut libc::c_void);
        break;
      }
      _ => {
        if intp.len != 0i32 as libc::c_ulong {
          s = zbc_num_divmod(&mut intp, &mut base, &mut intp, &mut digit, 0i32 as size_t);
          if s as u64 != 0 {
            current_block = 15705001814182199965;
            continue;
          }
          s = zbc_num_ulong(&mut digit, &mut dig);
          if s as u64 != 0 {
            current_block = 15705001814182199965;
            continue;
          }
          bc_vec_push(
            &mut stack,
            &mut dig as *mut libc::c_ulong as *const libc::c_void,
          );
          current_block = 2370887241019905314;
        } else {
          i = 0i32 as size_t;
          while i < stack.len {
            ptr = bc_vec_item_rev(&mut stack, i) as *mut libc::c_ulong;
            print.expect("non-null function pointer")(*ptr, width, 0i32 != 0);
            i = i.wrapping_add(1)
          }
          if (*n).rdx == 0 {
            current_block = 15705001814182199965;
            continue;
          }
          radix = 1i32 != 0;
          loop {
            if !(frac_len.len <= (*n).rdx) {
              current_block = 15705001814182199965;
              break;
            }
            s = zbc_num_mul(&mut fracp, &mut base, &mut fracp, (*n).rdx);
            if s as u64 != 0 {
              current_block = 15705001814182199965;
              break;
            }
            s = zbc_num_ulong(&mut fracp, &mut dig);
            if s as u64 != 0 {
              current_block = 15705001814182199965;
              break;
            }
            bc_num_ulong2num(&mut intp, dig);
            s = zbc_num_sub(&mut fracp, &mut intp, &mut fracp, 0i32 as size_t);
            if s as u64 != 0 {
              current_block = 15705001814182199965;
              break;
            }
            print.expect("non-null function pointer")(dig, width, radix);
            s = zbc_num_mul(&mut frac_len, &mut base, &mut frac_len, 0i32 as size_t);
            if s as u64 != 0 {
              current_block = 15705001814182199965;
              break;
            }
            radix = 0i32 != 0
          }
        }
      }
    }
  }
  bc_num_free(&mut digit as *mut BcNum as *mut libc::c_void);
  bc_num_free(&mut fracp as *mut BcNum as *mut libc::c_void);
  bc_num_free(&mut intp as *mut BcNum as *mut libc::c_void);
  bc_vec_free(&mut stack as *mut BcVec as *mut libc::c_void);
  return s;
}
unsafe extern "C" fn zxc_num_printBase(mut n: *mut BcNum) -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut width: size_t = 0;
  let mut print: BcNumDigitOp = None;
  let mut neg: bool = (*n).neg;
  if neg {
    bb_putchar('-' as i32);
    (*ptr_to_globals).prog.nchars = (*ptr_to_globals).prog.nchars.wrapping_add(1)
  }
  (*n).neg = 0i32 != 0;
  if (*ptr_to_globals).prog.ob_t <= 16i32 as libc::c_ulong {
    width = 1i32 as size_t;
    print = Some(bc_num_printHex as unsafe extern "C" fn(_: size_t, _: size_t, _: bool) -> ())
  } else {
    let mut i: libc::c_uint = (*ptr_to_globals)
      .prog
      .ob_t
      .wrapping_sub(1i32 as libc::c_ulong) as libc::c_uint;
    width = 0i32 as size_t;
    loop {
      width = width.wrapping_add(1);
      i = i.wrapping_div(10i32 as libc::c_uint);
      if i == 0i32 as libc::c_uint {
        break;
      }
    }
    print = Some(bc_num_printDigits as unsafe extern "C" fn(_: size_t, _: size_t, _: bool) -> ())
  }
  s = zxc_num_printNum(n, (*ptr_to_globals).prog.ob_t as libc::c_uint, width, print);
  (*n).neg = neg;
  return s;
}
unsafe extern "C" fn zxc_num_print(mut n: *mut BcNum, mut newline: bool) -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  bc_num_printNewline();
  if (*n).len == 0i32 as libc::c_ulong {
    bb_putchar('0' as i32);
    (*ptr_to_globals).prog.nchars = (*ptr_to_globals).prog.nchars.wrapping_add(1)
  } else if (*ptr_to_globals).prog.ob_t == 10i32 as libc::c_ulong {
    bc_num_printDecimal(n);
  } else {
    s = zxc_num_printBase(n)
  }
  if newline {
    bb_putchar('\n' as i32);
    (*ptr_to_globals).prog.nchars = 0i32 as size_t
  }
  return s;
}
unsafe extern "C" fn xc_program_print(mut inst: libc::c_char, mut idx: size_t) -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut r: *mut BcResult = 0 as *mut BcResult;
  let mut num: *mut BcNum = 0 as *mut BcNum;
  if !((*ptr_to_globals).prog.results.len > idx) {
    return bc_error_stack_has_too_few_elements() as BcStatus;
  }
  r = bc_vec_item_rev(&mut (*ptr_to_globals).prog.results, idx) as *mut BcResult;
  if inst as libc::c_int == XC_INST_PRINT as libc::c_int
    && (*r).t as libc::c_uint == BC_RESULT_VOID as libc::c_int as libc::c_uint
  {
    // void function's result on stack, ignore
    return BC_STATUS_SUCCESS;
  }
  s = zxc_program_num(r, &mut num);
  if s as u64 != 0 {
    return s;
  }
  if (*r).t as libc::c_uint != XC_RESULT_ARRAY as libc::c_int as libc::c_uint
    && (*r).t as libc::c_uint != XC_RESULT_STR as libc::c_int as libc::c_uint
    && !((*num).num.is_null() && (*num).cap == 0)
  {
    s = zxc_num_print(num, inst as libc::c_int == XC_INST_PRINT as libc::c_int);
    if s as u64 == 0
      && (1i32 != 0 && (1i32 == 0 || *applet_name.offset(0) as libc::c_int == 'b' as i32))
    {
      bc_num_copy(&mut (*ptr_to_globals).prog.last, num);
    }
  } else {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    idx = if (*r).t as libc::c_uint == XC_RESULT_STR as libc::c_int as libc::c_uint {
      (*r).d.id.idx
    } else {
      (*num).rdx
    };
    str = *xc_program_str(idx);
    if inst as libc::c_int == XC_INST_PRINT_STR as libc::c_int {
      let mut nl: *mut libc::c_char = 0 as *mut libc::c_char;
      (*ptr_to_globals).prog.nchars = ((*ptr_to_globals).prog.nchars as libc::c_ulong)
        .wrapping_add(printf(b"%s\x00" as *const u8 as *const libc::c_char, str) as libc::c_ulong)
        as size_t as size_t;
      nl = strrchr(str, '\n' as i32);
      if !nl.is_null() {
        (*ptr_to_globals).prog.nchars = strlen(nl.offset(1))
      }
    } else {
      xc_program_printString(str);
      if inst as libc::c_int == XC_INST_PRINT as libc::c_int {
        bb_putchar('\n' as i32);
      }
    }
  }
  if s as u64 == 0 && inst as libc::c_int != XC_INST_PRINT as libc::c_int {
    bc_vec_pop(&mut (*ptr_to_globals).prog.results);
  }
  return s;
}
unsafe extern "C" fn zxc_program_negate() -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut res: BcResult = BcResult {
    t: XC_RESULT_TEMP,
    d: BcResultData {
      n: BcNum {
        num: 0 as *mut BcDig,
        rdx: 0,
        len: 0,
        cap: 0,
        neg: false,
      },
    },
  };
  let mut ptr: *mut BcResult = 0 as *mut BcResult;
  let mut num: *mut BcNum = 0 as *mut BcNum;
  s = zxc_program_prep(&mut ptr, &mut num);
  if s as u64 != 0 {
    return s;
  }
  bc_num_init(&mut res.d.n, (*num).len);
  bc_num_copy(&mut res.d.n, num);
  if res.d.n.len != 0 {
    res.d.n.neg = !res.d.n.neg
  }
  xc_program_retire(&mut res, XC_RESULT_TEMP);
  return s;
}
unsafe extern "C" fn zxc_program_logical(mut inst: libc::c_char) -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut opd1: *mut BcResult = 0 as *mut BcResult;
  let mut opd2: *mut BcResult = 0 as *mut BcResult;
  let mut res: BcResult = BcResult {
    t: XC_RESULT_TEMP,
    d: BcResultData {
      n: BcNum {
        num: 0 as *mut BcDig,
        rdx: 0,
        len: 0,
        cap: 0,
        neg: false,
      },
    },
  };
  let mut n1: *mut BcNum = 0 as *mut BcNum;
  let mut n2: *mut BcNum = 0 as *mut BcNum;
  let mut cond: ssize_t = 0;
  s = zxc_program_binOpPrep(&mut opd1, &mut n1, &mut opd2, &mut n2, 0i32 != 0);
  if s as u64 != 0 {
    return s;
  }
  bc_num_init_DEF_SIZE(&mut res.d.n);
  if inst as libc::c_int == XC_INST_BOOL_AND as libc::c_int {
    cond = (bc_num_cmp(n1, &mut (*ptr_to_globals).prog.zero) != 0
      && bc_num_cmp(n2, &mut (*ptr_to_globals).prog.zero) != 0) as libc::c_int as ssize_t
  } else if inst as libc::c_int == XC_INST_BOOL_OR as libc::c_int {
    cond = (bc_num_cmp(n1, &mut (*ptr_to_globals).prog.zero) != 0
      || bc_num_cmp(n2, &mut (*ptr_to_globals).prog.zero) != 0) as libc::c_int as ssize_t
  } else {
    cond = bc_num_cmp(n1, n2);
    match inst as libc::c_int {
      5 => cond = (cond == 0i32 as libc::c_long) as libc::c_int as ssize_t,
      6 => cond = (cond <= 0i32 as libc::c_long) as libc::c_int as ssize_t,
      7 => cond = (cond >= 0i32 as libc::c_long) as libc::c_int as ssize_t,
      9 => cond = (cond < 0i32 as libc::c_long) as libc::c_int as ssize_t,
      10 => cond = (cond > 0i32 as libc::c_long) as libc::c_int as ssize_t,
      _ => {}
    }
  }
  if cond != 0 {
    bc_num_one(&mut res.d.n);
  }
  //else bc_num_zero(&res.d.n); - already is
  xc_program_binOpRetire(&mut res);
  return s;
}
unsafe extern "C" fn zdc_program_assignStr(
  mut r: *mut BcResult,
  mut v: *mut BcVec,
  mut push: bool,
) -> BcStatus {
  let mut n2: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut res: BcResult = BcResult {
    t: XC_RESULT_TEMP,
    d: BcResultData {
      n: BcNum {
        num: 0 as *mut BcDig,
        rdx: 0,
        len: 0,
        cap: 0,
        neg: false,
      },
    },
  };
  memset(
    &mut n2 as *mut BcNum as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<BcNum>() as libc::c_ulong,
  );
  res.d.id.idx = (*r).d.id.idx;
  n2.rdx = res.d.id.idx;
  res.t = XC_RESULT_STR;
  if !push {
    if !((*ptr_to_globals).prog.results.len > 1i32 as size_t) {
      return bc_error_stack_has_too_few_elements() as BcStatus;
    }
    bc_vec_pop(v);
    bc_vec_pop(&mut (*ptr_to_globals).prog.results);
  }
  bc_result_pop_and_push(&mut res as *mut BcResult as *const libc::c_void);
  bc_vec_push(v, &mut n2 as *mut BcNum as *const libc::c_void);
  return BC_STATUS_SUCCESS;
}
// ENABLE_DC
unsafe extern "C" fn zxc_program_popResultAndCopyToVar(
  mut name: *mut libc::c_char,
  mut t: BcType,
) -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut ptr: *mut BcResult = 0 as *mut BcResult;
  let mut r: BcResult = BcResult {
    t: XC_RESULT_TEMP,
    d: BcResultData {
      n: BcNum {
        num: 0 as *mut BcDig,
        rdx: 0,
        len: 0,
        cap: 0,
        neg: false,
      },
    },
  };
  let mut vec: *mut BcVec = 0 as *mut BcVec;
  let mut n: *mut BcNum = 0 as *mut BcNum;
  let mut var: bool = t as libc::c_uint == BC_TYPE_VAR as libc::c_int as libc::c_uint;
  if !((*ptr_to_globals).prog.results.len > 0i32 as size_t) {
    return bc_error_stack_has_too_few_elements() as BcStatus;
  }
  ptr = bc_vec_top(&mut (*ptr_to_globals).prog.results) as *mut BcResult;
  if ((*ptr).t as libc::c_uint == XC_RESULT_ARRAY as libc::c_int as libc::c_uint) as libc::c_int
    == var as libc::c_int
  {
    return bc_error_variable_is_wrong_type() as BcStatus;
  }
  vec = xc_program_search(name, t);
  if (*ptr).t as libc::c_uint == XC_RESULT_STR as libc::c_int as libc::c_uint {
    if !var {
      return bc_error_variable_is_wrong_type() as BcStatus;
    }
    return zdc_program_assignStr(ptr, vec, 1i32 != 0);
  }
  s = zxc_program_num(ptr, &mut n);
  if s as u64 != 0 {
    return s;
  }
  // Do this once more to make sure that pointers were not invalidated.
  vec = xc_program_search(name, t);
  if var {
    bc_num_init_DEF_SIZE(&mut r.d.n);
    bc_num_copy(&mut r.d.n, n);
  } else {
    let mut v: *mut BcVec = n as *mut BcVec;
    let mut ref_0: bool = false;
    let mut ref_size: bool = false;
    ref_0 = (*v).size == ::std::mem::size_of::<BcVec>() as libc::c_ulong
      && t as libc::c_uint != BC_TYPE_ARRAY as libc::c_int as libc::c_uint;
    ref_size = (*v).size == ::std::mem::size_of::<uint8_t>() as libc::c_ulong;
    if ref_0 as libc::c_int != 0
      || ref_size as libc::c_int != 0
        && t as libc::c_uint == BC_TYPE_REF as libc::c_int as libc::c_uint
    {
      bc_vec_init(
        &mut r.d.v,
        ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
        None,
      );
      if ref_0 {
        let mut vidx: size_t = 0;
        let mut idx: size_t = 0;
        let mut id: BcId = BcId {
          name: 0 as *mut libc::c_char,
          idx: 0,
        };
        id.name = (*ptr).d.id.name;
        v = xc_program_search((*ptr).d.id.name, BC_TYPE_REF);
        // Make sure the pointer was not invalidated.
        vec = xc_program_search(name, t);
        vidx = bc_map_find_exact(
          &mut (*ptr_to_globals).prog.arr_map,
          &mut id as *mut BcId as *const libc::c_void,
        );
        //assert(vidx != BC_VEC_INVALID_IDX);
        vidx = (*(bc_vec_item(&mut (*ptr_to_globals).prog.arr_map, vidx) as *mut BcId)).idx;
        idx = (*v).len.wrapping_sub(1i32 as libc::c_ulong);
        bc_vec_pushIndex(&mut r.d.v, vidx);
        bc_vec_pushIndex(&mut r.d.v, idx);
      } else {
        // If we get here, we are copying a ref to a ref.
        bc_vec_npush(&mut r.d.v, (*v).len, (*v).v as *const libc::c_void);
      }
    } else {
      if ref_size as libc::c_int != 0
        && t as libc::c_uint != BC_TYPE_REF as libc::c_int as libc::c_uint
      {
        v = xc_program_dereference(v)
      }
      bc_array_init(&mut r.d.v, 1i32 != 0);
      bc_array_copy(&mut r.d.v, v);
    }
  }
  // We need to return early.
  bc_vec_push(vec, &mut r.d as *mut BcResultData as *const libc::c_void);
  bc_vec_pop(&mut (*ptr_to_globals).prog.results);
  return s;
}
unsafe extern "C" fn zxc_program_assign(mut inst: libc::c_char) -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut left: *mut BcResult = 0 as *mut BcResult;
  let mut right: *mut BcResult = 0 as *mut BcResult;
  let mut res: BcResult = BcResult {
    t: XC_RESULT_TEMP,
    d: BcResultData {
      n: BcNum {
        num: 0 as *mut BcDig,
        rdx: 0,
        len: 0,
        cap: 0,
        neg: false,
      },
    },
  };
  let mut l: *mut BcNum = 0 as *mut BcNum;
  let mut r: *mut BcNum = 0 as *mut BcNum;
  let mut assign: bool = inst as libc::c_int == XC_INST_ASSIGN as libc::c_int;
  let mut ib: bool = false;
  let mut sc: bool = false;
  s = zxc_program_binOpPrep(&mut left, &mut l, &mut right, &mut r, assign);
  if s as u64 != 0 {
    return s;
  }
  ib = (*left).t as libc::c_uint == XC_RESULT_IBASE as libc::c_int as libc::c_uint;
  sc = (*left).t as libc::c_uint == XC_RESULT_SCALE as libc::c_int as libc::c_uint;
  if (*right).t as libc::c_uint == XC_RESULT_STR as libc::c_int as libc::c_uint {
    let mut v: *mut BcVec = 0 as *mut BcVec;
    if (*left).t as libc::c_uint != XC_RESULT_VAR as libc::c_int as libc::c_uint {
      return bc_error_variable_is_wrong_type() as BcStatus;
    }
    v = xc_program_search((*left).d.id.name, BC_TYPE_VAR);
    return zdc_program_assignStr(right, v, 0i32 != 0);
  }
  if (*left).t as libc::c_uint == XC_RESULT_CONSTANT as libc::c_int as libc::c_uint
    || (*left).t as libc::c_uint == XC_RESULT_TEMP as libc::c_int as libc::c_uint
    || (*left).t as libc::c_uint == BC_RESULT_VOID as libc::c_int as libc::c_uint
  {
    return bc_error_bad_assignment() as BcStatus;
  }
  if assign {
    bc_num_copy(l, r);
  } else {
    s = BC_STATUS_SUCCESS;
    s = zxc_program_ops[(inst as libc::c_int - BC_INST_ASSIGN_POWER as libc::c_int) as usize]
      .expect("non-null function pointer")(l, r, l, (*ptr_to_globals).prog.scale)
  }
  if s as u64 != 0 {
    return s;
  }
  if ib as libc::c_int != 0
    || sc as libc::c_int != 0
    || (*left).t as libc::c_uint == XC_RESULT_OBASE as libc::c_int as libc::c_uint
  {
    static mut msg: [*const libc::c_char; 3] = [
      b"bad ibase; must be [2,16]\x00" as *const u8 as *const libc::c_char,
      b"bad obase; must be [2,999]\x00" as *const u8 as *const libc::c_char,
      b"bad scale; must be [0,4294967295]\x00" as *const u8 as *const libc::c_char,
    ];
    let mut ptr: *mut size_t = 0 as *mut size_t;
    let mut max: size_t = 0;
    let mut val: libc::c_ulong = 0;
    s = zbc_num_ulong(l, &mut val);
    if s as u64 != 0 {
      return s;
    }
    s = ((*left).t as libc::c_uint).wrapping_sub(XC_RESULT_IBASE as libc::c_int as libc::c_uint)
      as BcStatus;
    if sc {
      max = (2147483647i32 as libc::c_uint)
        .wrapping_mul(2u32)
        .wrapping_add(1u32) as size_t;
      ptr = &mut (*ptr_to_globals).prog.scale
    } else {
      if val < 2i32 as libc::c_ulong {
        return bc_error(msg[s as usize]) as BcStatus;
      }
      max = if ib as libc::c_int != 0 {
        36i32 as libc::c_uint
      } else {
        999i32 as libc::c_uint
      } as size_t;
      ptr = if ib as libc::c_int != 0 {
        &mut (*ptr_to_globals).prog.ib_t
      } else {
        &mut (*ptr_to_globals).prog.ob_t
      }
    }
    if val > max {
      return bc_error(msg[s as usize]) as BcStatus;
    }
    *ptr = val;
    s = BC_STATUS_SUCCESS
  }
  bc_num_init(&mut res.d.n, (*l).len);
  bc_num_copy(&mut res.d.n, l);
  xc_program_binOpRetire(&mut res);
  return s;
}
unsafe extern "C" fn xc_program_pushVar(
  mut code: *mut libc::c_char,
  mut bgn: *mut size_t,
  mut pop: bool,
  mut copy: bool,
) -> BcStatus {
  let mut r: BcResult = BcResult {
    t: XC_RESULT_TEMP,
    d: BcResultData {
      n: BcNum {
        num: 0 as *mut BcDig,
        rdx: 0,
        len: 0,
        cap: 0,
        neg: false,
      },
    },
  };
  let mut name: *mut libc::c_char = xc_program_name(code, bgn);
  r.t = XC_RESULT_VAR;
  r.d.id.name = name;
  if pop as libc::c_int != 0 || copy as libc::c_int != 0 {
    let mut v: *mut BcVec = xc_program_search(name, BC_TYPE_VAR);
    let mut num: *mut BcNum = bc_vec_top(v) as *mut BcNum;
    free(name as *mut libc::c_void);
    if !((*v).len > (1i32 - copy as libc::c_int) as size_t) {
      return bc_error_stack_has_too_few_elements() as BcStatus;
    }
    if !((*num).num.is_null() && (*num).cap == 0) {
      r.t = XC_RESULT_TEMP;
      bc_num_init_DEF_SIZE(&mut r.d.n);
      bc_num_copy(&mut r.d.n, num);
    } else {
      r.t = XC_RESULT_STR;
      r.d.id.idx = (*num).rdx
    }
    if !copy {
      bc_vec_pop(v);
    }
  }
  // ENABLE_DC
  bc_vec_push(
    &mut (*ptr_to_globals).prog.results,
    &mut r as *mut BcResult as *const libc::c_void,
  );
  return BC_STATUS_SUCCESS;
}
unsafe extern "C" fn zbc_program_pushArray(
  mut code: *mut libc::c_char,
  mut bgn: *mut size_t,
  mut inst: libc::c_char,
) -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut r: BcResult = BcResult {
    t: XC_RESULT_TEMP,
    d: BcResultData {
      n: BcNum {
        num: 0 as *mut BcDig,
        rdx: 0,
        len: 0,
        cap: 0,
        neg: false,
      },
    },
  };
  let mut num: *mut BcNum = 0 as *mut BcNum;
  r.d.id.name = xc_program_name(code, bgn);
  if inst as libc::c_int == XC_INST_ARRAY as libc::c_int {
    r.t = XC_RESULT_ARRAY;
    bc_vec_push(
      &mut (*ptr_to_globals).prog.results,
      &mut r as *mut BcResult as *const libc::c_void,
    );
  } else {
    let mut operand: *mut BcResult = 0 as *mut BcResult;
    let mut temp: libc::c_ulong = 0;
    s = zxc_program_prep(&mut operand, &mut num);
    if !(s as u64 != 0) {
      s = zbc_num_ulong(num, &mut temp);
      if !(s as u64 != 0) {
        if temp > 2147483647i32 as libc::c_uint as libc::c_ulong {
          s = bc_error(
            b"array too long; must be [1,2147483647]\x00" as *const u8 as *const libc::c_char,
          ) as BcStatus
        } else {
          r.d.id.idx = temp;
          xc_program_retire(&mut r, XC_RESULT_ARRAY_ELEM);
        }
      }
    }
  }
  if s as u64 != 0 {
    free(r.d.id.name as *mut libc::c_void);
  }
  return s;
}
unsafe extern "C" fn zbc_program_incdec(mut inst: libc::c_char) -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut ptr: *mut BcResult = 0 as *mut BcResult;
  let mut res: BcResult = BcResult {
    t: XC_RESULT_TEMP,
    d: BcResultData {
      n: BcNum {
        num: 0 as *mut BcDig,
        rdx: 0,
        len: 0,
        cap: 0,
        neg: false,
      },
    },
  };
  let mut copy: BcResult = BcResult {
    t: XC_RESULT_TEMP,
    d: BcResultData {
      n: BcNum {
        num: 0 as *mut BcDig,
        rdx: 0,
        len: 0,
        cap: 0,
        neg: false,
      },
    },
  };
  let mut num: *mut BcNum = 0 as *mut BcNum;
  let mut inst2: libc::c_char = inst;
  s = zxc_program_prep(&mut ptr, &mut num);
  if s as u64 != 0 {
    return s;
  }
  if inst as libc::c_int == BC_INST_INC_POST as libc::c_int
    || inst as libc::c_int == BC_INST_DEC_POST as libc::c_int
  {
    copy.t = XC_RESULT_TEMP;
    bc_num_init(&mut copy.d.n, (*num).len);
    bc_num_copy(&mut copy.d.n, num);
  }
  res.t = BC_RESULT_ONE;
  inst = if inst as libc::c_int == BC_INST_INC_PRE as libc::c_int
    || inst as libc::c_int == BC_INST_INC_POST as libc::c_int
  {
    BC_INST_ASSIGN_PLUS as libc::c_int
  } else {
    BC_INST_ASSIGN_MINUS as libc::c_int
  } as libc::c_char;
  bc_vec_push(
    &mut (*ptr_to_globals).prog.results,
    &mut res as *mut BcResult as *const libc::c_void,
  );
  s = zxc_program_assign(inst);
  if s as u64 != 0 {
    return s;
  }
  if inst2 as libc::c_int == BC_INST_INC_POST as libc::c_int
    || inst2 as libc::c_int == BC_INST_DEC_POST as libc::c_int
  {
    bc_result_pop_and_push(&mut copy as *mut BcResult as *const libc::c_void);
  }
  return s;
}
unsafe extern "C" fn zbc_program_call(
  mut code: *mut libc::c_char,
  mut idx: *mut size_t,
) -> BcStatus {
  let mut ip: BcInstPtr = BcInstPtr {
    func: 0,
    inst_idx: 0,
  };
  let mut i: size_t = 0;
  let mut nparams: size_t = 0;
  let mut a: *mut BcId = 0 as *mut BcId;
  let mut func: *mut BcFunc = 0 as *mut BcFunc;
  nparams = xc_program_index(code, idx);
  ip.func = xc_program_index(code, idx);
  func = xc_program_func(ip.func);
  if (*func).code.len == 0i32 as libc::c_ulong {
    return bc_error(b"undefined function\x00" as *const u8 as *const libc::c_char) as BcStatus;
  }
  if nparams != (*func).nparams {
    return bc_error_fmt(
      b"function has %u parameters, but called with %u\x00" as *const u8 as *const libc::c_char,
      (*func).nparams,
      nparams,
    ) as BcStatus;
  }
  ip.inst_idx = 0i32 as size_t;
  i = 0i32 as size_t;
  while i < nparams {
    let mut arg: *mut BcResult = 0 as *mut BcResult;
    let mut s: BcStatus = BC_STATUS_SUCCESS;
    let mut arr: bool = false;
    a = bc_vec_item(
      &mut (*func).autos,
      nparams.wrapping_sub(1i32 as libc::c_ulong).wrapping_sub(i),
    ) as *mut BcId;
    arg = bc_vec_top(&mut (*ptr_to_globals).prog.results) as *mut BcResult;
    arr = (*a).idx == BC_TYPE_ARRAY as libc::c_int as libc::c_ulong
      || (*a).idx == BC_TYPE_REF as libc::c_int as libc::c_ulong;
    if arr as libc::c_int
      != ((*arg).t as libc::c_uint == XC_RESULT_ARRAY as libc::c_int as libc::c_uint) as libc::c_int
    {
      // array/variable mismatch
      // || arg->t == XC_RESULT_STR - impossible, f("str") is not a legal syntax (strings are not bc expressions)
      return bc_error_variable_is_wrong_type() as BcStatus;
    }
    s = zxc_program_popResultAndCopyToVar((*a).name, (*a).idx as BcType);
    if s as u64 != 0 {
      return s;
    }
    i = i.wrapping_add(1)
  }
  a = bc_vec_item(&mut (*func).autos, i) as *mut BcId;
  while i < (*func).autos.len {
    let mut v: *mut BcVec = 0 as *mut BcVec;
    v = xc_program_search((*a).name, (*a).idx as BcType);
    if (*a).idx == BC_TYPE_VAR as libc::c_int as libc::c_ulong {
      let mut n2: BcNum = BcNum {
        num: 0 as *mut BcDig,
        rdx: 0,
        len: 0,
        cap: 0,
        neg: false,
      };
      bc_num_init_DEF_SIZE(&mut n2);
      bc_vec_push(v, &mut n2 as *mut BcNum as *const libc::c_void);
    } else {
      //assert(a->idx == BC_TYPE_ARRAY);
      let mut v2: BcVec = BcVec {
        v: 0 as *mut libc::c_char,
        len: 0,
        cap: 0,
        size: 0,
        dtor: None,
      };
      bc_array_init(&mut v2, 1i32 != 0);
      bc_vec_push(v, &mut v2 as *mut BcVec as *const libc::c_void);
    }
    i = i.wrapping_add(1);
    a = a.offset(1)
  }
  bc_vec_push(
    &mut (*ptr_to_globals).prog.exestack,
    &mut ip as *mut BcInstPtr as *const libc::c_void,
  );
  return BC_STATUS_SUCCESS;
}
unsafe extern "C" fn zbc_program_return(mut inst: libc::c_char) -> BcStatus {
  let mut res: BcResult = BcResult {
    t: XC_RESULT_TEMP,
    d: BcResultData {
      n: BcNum {
        num: 0 as *mut BcDig,
        rdx: 0,
        len: 0,
        cap: 0,
        neg: false,
      },
    },
  };
  let mut f: *mut BcFunc = 0 as *mut BcFunc;
  let mut a: *mut BcId = 0 as *mut BcId;
  let mut i: size_t = 0;
  let mut ip: *mut BcInstPtr = bc_vec_top(&mut (*ptr_to_globals).prog.exestack) as *mut BcInstPtr;
  f = xc_program_func((*ip).func);
  res.t = XC_RESULT_TEMP;
  if inst as libc::c_int == XC_INST_RET as libc::c_int {
    // bc needs this for e.g. RESULT_CONSTANT ("return 5")
    // because bc constants are per-function.
    // TODO: maybe avoid if value is already RESULT_TEMP?
    let mut s: BcStatus = BC_STATUS_SUCCESS;
    let mut num: *mut BcNum = 0 as *mut BcNum;
    let mut operand: *mut BcResult =
      bc_vec_top(&mut (*ptr_to_globals).prog.results) as *mut BcResult;
    s = zxc_program_num(operand, &mut num);
    if s as u64 != 0 {
      return s;
    }
    bc_num_init(&mut res.d.n, (*num).len);
    bc_num_copy(&mut res.d.n, num);
    bc_vec_pop(&mut (*ptr_to_globals).prog.results);
  } else {
    if (*f).voidfunc {
      res.t = BC_RESULT_VOID
    }
    bc_num_init_DEF_SIZE(&mut res.d.n);
    //bc_num_zero(&res.d.n); - already is
  }
  bc_vec_push(
    &mut (*ptr_to_globals).prog.results,
    &mut res as *mut BcResult as *const libc::c_void,
  );
  bc_vec_pop(&mut (*ptr_to_globals).prog.exestack);
  // We need to pop arguments as well, so this takes that into account.
  a = (*f).autos.v as *mut libc::c_void as *mut BcId;
  i = 0i32 as size_t;
  while i < (*f).autos.len {
    let mut v: *mut BcVec = 0 as *mut BcVec;
    v = xc_program_search((*a).name, (*a).idx as BcType);
    bc_vec_pop(v);
    i = i.wrapping_add(1);
    a = a.offset(1)
  }
  return BC_STATUS_SUCCESS;
}
// ENABLE_BC
unsafe extern "C" fn xc_program_scale(mut n: *mut BcNum) -> libc::c_ulong {
  return (*n).rdx;
}
unsafe extern "C" fn xc_program_len(mut n: *mut BcNum) -> libc::c_ulong {
  let mut len: size_t = (*n).len;
  if (*n).rdx != len {
    return len;
  }
  while !(len == 0i32 as libc::c_ulong) {
    len = len.wrapping_sub(1);
    if *(*n).num.offset(len as isize) as libc::c_int != 0i32 {
      break;
    }
  }
  return len;
}
unsafe extern "C" fn zxc_program_builtin(mut inst: libc::c_char) -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut opnd: *mut BcResult = 0 as *mut BcResult;
  let mut num: *mut BcNum = 0 as *mut BcNum;
  let mut res: BcResult = BcResult {
    t: XC_RESULT_TEMP,
    d: BcResultData {
      n: BcNum {
        num: 0 as *mut BcDig,
        rdx: 0,
        len: 0,
        cap: 0,
        neg: false,
      },
    },
  };
  let mut len: bool = inst as libc::c_int == XC_INST_LENGTH as libc::c_int;
  if !((*ptr_to_globals).prog.results.len > 0i32 as size_t) {
    return bc_error_stack_has_too_few_elements() as BcStatus;
  }
  opnd = bc_vec_top(&mut (*ptr_to_globals).prog.results) as *mut BcResult;
  s = zxc_program_num(opnd, &mut num);
  if s as u64 != 0 {
    return s;
  }
  if !((*opnd).t as libc::c_uint != XC_RESULT_ARRAY as libc::c_int as libc::c_uint
    && (*opnd).t as libc::c_uint != XC_RESULT_STR as libc::c_int as libc::c_uint
    && !((*num).num.is_null() && (*num).cap == 0))
    && !len
  {
    return bc_error_variable_is_wrong_type() as BcStatus;
  }
  bc_num_init_DEF_SIZE(&mut res.d.n);
  if inst as libc::c_int == XC_INST_SQRT as libc::c_int {
    s = zbc_num_sqrt(num, &mut res.d.n, (*ptr_to_globals).prog.scale)
  } else if len as libc::c_int != 0i32
    && (*opnd).t as libc::c_uint == XC_RESULT_ARRAY as libc::c_int as libc::c_uint
  {
    bc_num_ulong2num(&mut res.d.n, (*(num as *mut BcVec)).len);
  } else if len as libc::c_int != 0i32
    && !((*opnd).t as libc::c_uint != XC_RESULT_ARRAY as libc::c_int as libc::c_uint
      && (*opnd).t as libc::c_uint != XC_RESULT_STR as libc::c_int as libc::c_uint
      && !((*num).num.is_null() && (*num).cap == 0))
  {
    let mut str: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut idx: size_t =
      if (*opnd).t as libc::c_uint == XC_RESULT_STR as libc::c_int as libc::c_uint {
        (*opnd).d.id.idx
      } else {
        (*num).rdx
      };
    str = xc_program_str(idx);
    bc_num_ulong2num(&mut res.d.n, strlen(*str));
  } else {
    bc_num_ulong2num(
      &mut res.d.n,
      if len as libc::c_int != 0 {
        xc_program_len(num)
      } else {
        xc_program_scale(num)
      },
    );
  }
  xc_program_retire(&mut res, XC_RESULT_TEMP);
  return s;
}
unsafe extern "C" fn zdc_program_divmod() -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut opd1: *mut BcResult = 0 as *mut BcResult;
  let mut opd2: *mut BcResult = 0 as *mut BcResult;
  let mut res: BcResult = BcResult {
    t: XC_RESULT_TEMP,
    d: BcResultData {
      n: BcNum {
        num: 0 as *mut BcDig,
        rdx: 0,
        len: 0,
        cap: 0,
        neg: false,
      },
    },
  };
  let mut res2: BcResult = BcResult {
    t: XC_RESULT_TEMP,
    d: BcResultData {
      n: BcNum {
        num: 0 as *mut BcDig,
        rdx: 0,
        len: 0,
        cap: 0,
        neg: false,
      },
    },
  };
  let mut n1: *mut BcNum = 0 as *mut BcNum;
  let mut n2: *mut BcNum = 0 as *mut BcNum;
  s = zxc_program_binOpPrep(&mut opd1, &mut n1, &mut opd2, &mut n2, 0i32 != 0);
  if s as u64 != 0 {
    return s;
  }
  bc_num_init_DEF_SIZE(&mut res.d.n);
  bc_num_init(&mut res2.d.n, (*n2).len);
  s = zbc_num_divmod(
    n1,
    n2,
    &mut res2.d.n,
    &mut res.d.n,
    (*ptr_to_globals).prog.scale,
  );
  if s as u64 != 0 {
    bc_num_free(&mut res2.d.n as *mut BcNum as *mut libc::c_void);
    bc_num_free(&mut res.d.n as *mut BcNum as *mut libc::c_void);
    return s;
  } else {
    xc_program_binOpRetire(&mut res2);
    res.t = XC_RESULT_TEMP;
    bc_vec_push(
      &mut (*ptr_to_globals).prog.results,
      &mut res as *mut BcResult as *const libc::c_void,
    );
    return s;
  };
}
unsafe extern "C" fn zdc_program_modexp() -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut r1: *mut BcResult = 0 as *mut BcResult;
  let mut r2: *mut BcResult = 0 as *mut BcResult;
  let mut r3: *mut BcResult = 0 as *mut BcResult;
  let mut res: BcResult = BcResult {
    t: XC_RESULT_TEMP,
    d: BcResultData {
      n: BcNum {
        num: 0 as *mut BcDig,
        rdx: 0,
        len: 0,
        cap: 0,
        neg: false,
      },
    },
  };
  let mut n1: *mut BcNum = 0 as *mut BcNum;
  let mut n2: *mut BcNum = 0 as *mut BcNum;
  let mut n3: *mut BcNum = 0 as *mut BcNum;
  if !((*ptr_to_globals).prog.results.len > 2i32 as size_t) {
    return bc_error_stack_has_too_few_elements() as BcStatus;
  }
  s = zxc_program_binOpPrep(&mut r2, &mut n2, &mut r3, &mut n3, 0i32 != 0);
  if s as u64 != 0 {
    return s;
  }
  r1 = bc_vec_item_rev(&mut (*ptr_to_globals).prog.results, 2i32 as size_t) as *mut BcResult;
  s = zxc_program_num(r1, &mut n1);
  if s as u64 != 0 {
    return s;
  }
  if !((*r1).t as libc::c_uint != XC_RESULT_ARRAY as libc::c_int as libc::c_uint
    && (*r1).t as libc::c_uint != XC_RESULT_STR as libc::c_int as libc::c_uint
    && !((*n1).num.is_null() && (*n1).cap == 0))
  {
    return bc_error_variable_is_wrong_type() as BcStatus;
  }
  // Make sure that the values have their pointers updated, if necessary.
  if (*r1).t as libc::c_uint == XC_RESULT_VAR as libc::c_int as libc::c_uint
    || (*r1).t as libc::c_uint == XC_RESULT_ARRAY_ELEM as libc::c_int as libc::c_uint
  {
    if (*r1).t as libc::c_uint == (*r2).t as libc::c_uint {
      s = zxc_program_num(r2, &mut n2);
      if s as u64 != 0 {
        return s;
      }
    }
    if (*r1).t as libc::c_uint == (*r3).t as libc::c_uint {
      s = zxc_program_num(r3, &mut n3);
      if s as u64 != 0 {
        return s;
      }
    }
  }
  bc_num_init(&mut res.d.n, (*n3).len);
  s = zdc_num_modexp(n1, n2, n3, &mut res.d.n);
  if s as u64 != 0 {
    bc_num_free(&mut res.d.n as *mut BcNum as *mut libc::c_void);
    return s;
  } else {
    bc_vec_pop(&mut (*ptr_to_globals).prog.results);
    xc_program_binOpRetire(&mut res);
    return s;
  };
}
unsafe extern "C" fn dc_program_stackLen() {
  let mut res: BcResult = BcResult {
    t: XC_RESULT_TEMP,
    d: BcResultData {
      n: BcNum {
        num: 0 as *mut BcDig,
        rdx: 0,
        len: 0,
        cap: 0,
        neg: false,
      },
    },
  };
  let mut len: size_t = (*ptr_to_globals).prog.results.len;
  res.t = XC_RESULT_TEMP;
  bc_num_init_DEF_SIZE(&mut res.d.n);
  bc_num_ulong2num(&mut res.d.n, len);
  bc_vec_push(
    &mut (*ptr_to_globals).prog.results,
    &mut res as *mut BcResult as *const libc::c_void,
  );
}
unsafe extern "C" fn zdc_program_asciify() -> BcStatus {
  let mut current_block: u64;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut r: *mut BcResult = 0 as *mut BcResult;
  let mut res: BcResult = BcResult {
    t: XC_RESULT_TEMP,
    d: BcResultData {
      n: BcNum {
        num: 0 as *mut BcDig,
        rdx: 0,
        len: 0,
        cap: 0,
        neg: false,
      },
    },
  };
  let mut num: *mut BcNum = 0 as *mut BcNum;
  let mut n: BcNum = BcNum {
    num: 0 as *mut BcDig,
    rdx: 0,
    len: 0,
    cap: 0,
    neg: false,
  };
  let mut strs: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut c: libc::c_char = 0;
  let mut idx: size_t = 0;
  if !((*ptr_to_globals).prog.results.len > 0i32 as size_t) {
    return bc_error_stack_has_too_few_elements() as BcStatus;
  }
  r = bc_vec_top(&mut (*ptr_to_globals).prog.results) as *mut BcResult;
  s = zxc_program_num(r, &mut num);
  if s as u64 != 0 {
    return s;
  }
  if (*r).t as libc::c_uint != XC_RESULT_ARRAY as libc::c_int as libc::c_uint
    && (*r).t as libc::c_uint != XC_RESULT_STR as libc::c_int as libc::c_uint
    && !((*num).num.is_null() && (*num).cap == 0)
  {
    let mut val: libc::c_ulong = 0;
    let mut strmb: BcNum = BcNum {
      num: 0 as *mut BcDig,
      rdx: 0,
      len: 0,
      cap: 0,
      neg: false,
    };
    let mut strmb_digs: [BcDig; 20] = [0; 20];
    bc_num_init_DEF_SIZE(&mut n);
    bc_num_copy(&mut n, num);
    bc_num_truncate(&mut n, n.rdx);
    strmb.cap = (::std::mem::size_of::<[BcDig; 20]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<BcDig>() as libc::c_ulong) as libc::c_uint
      as size_t;
    strmb.num = strmb_digs.as_mut_ptr();
    bc_num_ulong2num(&mut strmb, 0x100i32 as libc::c_ulong);
    s = zbc_num_mod(&mut n, &mut strmb, &mut n, 0i32 as size_t);
    if s as u64 != 0 {
      current_block = 14386233464246175769;
    } else {
      s = zbc_num_ulong(&mut n, &mut val);
      if s as u64 != 0 {
        current_block = 14386233464246175769;
      } else {
        c = val as libc::c_char;
        bc_num_free(&mut n as *mut BcNum as *mut libc::c_void);
        current_block = 7056779235015430508;
      }
    }
    match current_block {
      7056779235015430508 => {}
      _ => {
        bc_num_free(&mut n as *mut BcNum as *mut libc::c_void);
        return s;
      }
    }
  } else {
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    idx = if (*r).t as libc::c_uint == XC_RESULT_STR as libc::c_int as libc::c_uint {
      (*r).d.id.idx
    } else {
      (*num).rdx
    };
    sp = *xc_program_str(idx);
    c = *sp.offset(0)
  }
  strs = (*ptr_to_globals).prog.strs.v as *mut libc::c_void as *mut *mut libc::c_char;
  idx = 0i32 as size_t;
  loop {
    if !(idx < (*ptr_to_globals).prog.strs.len) {
      current_block = 2891135413264362348;
      break;
    }
    if *(*strs.offset(idx as isize)).offset(0) as libc::c_int == c as libc::c_int
      && *(*strs.offset(idx as isize)).offset(1) as libc::c_int == '\u{0}' as i32
    {
      current_block = 15217880022857650315;
      break;
    }
    idx = idx.wrapping_add(1)
  }
  match current_block {
    2891135413264362348 => {
      str = xzalloc(2i32 as size_t) as *mut libc::c_char;
      *str.offset(0) = c;
      //str[1] = '\0'; - already is
      idx = bc_vec_push(
        &mut (*ptr_to_globals).prog.strs,
        &mut str as *mut *mut libc::c_char as *const libc::c_void,
      );
      // Add an empty function so that if zdc_program_execStr ever needs to
      // parse the string into code (from the 'x' command) there's somewhere
      // to store the bytecode.
      xc_program_add_fn(); // for compiler
    }
    _ => {}
  } // struct copy
  res.t = XC_RESULT_STR; // struct copy
  res.d.id.idx = idx;
  bc_result_pop_and_push(&mut res as *mut BcResult as *const libc::c_void);
  return BC_STATUS_SUCCESS;
}
unsafe extern "C" fn zdc_program_printStream() -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut r: *mut BcResult = 0 as *mut BcResult;
  let mut n: *mut BcNum = 0 as *mut BcNum;
  let mut idx: size_t = 0;
  if !((*ptr_to_globals).prog.results.len > 0i32 as size_t) {
    return bc_error_stack_has_too_few_elements() as BcStatus;
  }
  r = bc_vec_top(&mut (*ptr_to_globals).prog.results) as *mut BcResult;
  s = zxc_program_num(r, &mut n);
  if s as u64 != 0 {
    return s;
  }
  if (*r).t as libc::c_uint != XC_RESULT_ARRAY as libc::c_int as libc::c_uint
    && (*r).t as libc::c_uint != XC_RESULT_STR as libc::c_int as libc::c_uint
    && !((*n).num.is_null() && (*n).cap == 0)
  {
    s = zxc_num_printNum(
      n,
      0x100i32 as libc::c_uint,
      1i32 as size_t,
      Some(dc_num_printChar as unsafe extern "C" fn(_: size_t, _: size_t, _: bool) -> ()),
    )
  } else {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    idx = if (*r).t as libc::c_uint == XC_RESULT_STR as libc::c_int as libc::c_uint {
      (*r).d.id.idx
    } else {
      (*n).rdx
    };
    str = *xc_program_str(idx);
    fputs_unlocked(str, stdout);
  }
  return s;
}
unsafe extern "C" fn zdc_program_nquit() -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut opnd: *mut BcResult = 0 as *mut BcResult;
  let mut num: *mut BcNum = 0 as *mut BcNum;
  let mut val: libc::c_ulong = 0;
  s = zxc_program_prep(&mut opnd, &mut num);
  if s as u64 != 0 {
    return s;
  }
  s = zbc_num_ulong(num, &mut val);
  if s as u64 != 0 {
    return s;
  }
  bc_vec_pop(&mut (*ptr_to_globals).prog.results);
  if (*ptr_to_globals).prog.exestack.len < val {
    return bc_error_stack_has_too_few_elements() as BcStatus;
  }
  if (*ptr_to_globals).prog.exestack.len == val {
    quit();
  }
  bc_vec_npop(&mut (*ptr_to_globals).prog.exestack, val);
  return s;
}
unsafe extern "C" fn zdc_program_execStr(
  mut code: *mut libc::c_char,
  mut bgn: *mut size_t,
  mut cond: bool,
) -> BcStatus {
  let mut current_block: u64;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut r: *mut BcResult = 0 as *mut BcResult;
  let mut f: *mut BcFunc = 0 as *mut BcFunc;
  let mut ip: BcInstPtr = BcInstPtr {
    func: 0,
    inst_idx: 0,
  };
  let mut fidx: size_t = 0;
  let mut sidx: size_t = 0;
  if !((*ptr_to_globals).prog.results.len > 0i32 as size_t) {
    return bc_error_stack_has_too_few_elements() as BcStatus;
  }
  r = bc_vec_top(&mut (*ptr_to_globals).prog.results) as *mut BcResult;
  if cond {
    let mut n: *mut BcNum = 0 as *mut BcNum;
    n = n;
    let mut exec: bool = false;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut then_name: *mut libc::c_char = xc_program_name(code, bgn);
    let mut else_name: *mut libc::c_char = 0 as *mut libc::c_char;
    if *code.offset(*bgn as isize) as libc::c_int == '\u{0}' as i32 {
      *bgn = (*bgn as libc::c_ulong).wrapping_add(1i32 as libc::c_ulong) as size_t as size_t
    } else {
      else_name = xc_program_name(code, bgn)
    }
    exec = (*r).d.n.len != 0i32 as libc::c_ulong;
    name = then_name;
    if !exec && !else_name.is_null() {
      exec = 1i32 != 0;
      name = else_name
    }
    if exec {
      let mut v: *mut BcVec = 0 as *mut BcVec;
      v = xc_program_search(name, BC_TYPE_VAR);
      n = bc_vec_top(v) as *mut BcNum
    }
    free(then_name as *mut libc::c_void);
    free(else_name as *mut libc::c_void);
    if !exec {
      current_block = 5837603160108855842;
    } else if !((*n).num.is_null() && (*n).cap == 0) {
      s = bc_error_variable_is_wrong_type() as BcStatus;
      current_block = 5837603160108855842;
    } else {
      sidx = (*n).rdx;
      current_block = 11048769245176032998;
    }
  } else if (*r).t as libc::c_uint == XC_RESULT_STR as libc::c_int as libc::c_uint {
    sidx = (*r).d.id.idx;
    current_block = 11048769245176032998;
  } else if (*r).t as libc::c_uint == XC_RESULT_VAR as libc::c_int as libc::c_uint {
    let mut n_0: *mut BcNum = 0 as *mut BcNum;
    s = zxc_program_num(r, &mut n_0);
    if s as libc::c_uint != 0 || !((*n_0).num.is_null() && (*n_0).cap == 0) {
      current_block = 5837603160108855842;
    } else {
      sidx = (*n_0).rdx;
      current_block = 11048769245176032998;
    }
  } else {
    current_block = 12412625468482137631;
  }
  match current_block {
    11048769245176032998 => {
      fidx = sidx.wrapping_add(2i32 as libc::c_ulong);
      f = xc_program_func(fidx);
      if (*f).code.len == 0i32 as libc::c_ulong {
        let mut sv_parse: BcParse = BcParse {
          lex: 0,
          lex_last: 0,
          lex_line: 0,
          lex_inbuf: 0 as *const libc::c_char,
          lex_next_at: 0 as *const libc::c_char,
          lex_filename: 0 as *const libc::c_char,
          lex_input_fp: 0 as *mut FILE,
          lex_strnumbuf: BcVec {
            v: 0 as *mut libc::c_char,
            len: 0,
            cap: 0,
            size: 0,
            dtor: None,
          },
          func: 0 as *mut BcFunc,
          fidx: 0,
          in_funcdef: 0,
          exits: BcVec {
            v: 0 as *mut libc::c_char,
            len: 0,
            cap: 0,
            size: 0,
            dtor: None,
          },
          conds: BcVec {
            v: 0 as *mut libc::c_char,
            len: 0,
            cap: 0,
            size: 0,
            dtor: None,
          },
          ops: BcVec {
            v: 0 as *mut libc::c_char,
            len: 0,
            cap: 0,
            size: 0,
            dtor: None,
          },
        };
        let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
        sv_parse = (*ptr_to_globals).prs;
        xc_parse_create(fidx);
        str = *xc_program_str(sidx);
        s = zxc_parse_text_init(str);
        if s as u64 != 0 {
          current_block = 16162562597699186501;
        } else {
          s = zdc_parse_exprs_until_eof();
          if s as u64 != 0 {
            current_block = 16162562597699186501;
          } else {
            xc_parse_push(DC_INST_POP_EXEC as libc::c_int as libc::c_uint);
            if (*ptr_to_globals).prs.lex as libc::c_int != XC_LEX_EOF as libc::c_int {
              s = bc_error_bad_expression() as BcStatus
            }
            xc_parse_free();
            (*ptr_to_globals).prs = sv_parse;
            if s as u64 != 0 {
              current_block = 16162562597699186501;
            } else {
              current_block = 10930818133215224067;
            }
          }
        }
        match current_block {
          10930818133215224067 => {}
          _ => {
            bc_vec_pop_all(&mut (*f).code);
            current_block = 5837603160108855842;
          }
        }
      } else {
        current_block = 10930818133215224067;
      }
      match current_block {
        5837603160108855842 => {}
        _ => {
          ip.inst_idx = 0i32 as size_t;
          ip.func = fidx;
          bc_vec_pop(&mut (*ptr_to_globals).prog.results);
          bc_vec_push(
            &mut (*ptr_to_globals).prog.exestack,
            &mut ip as *mut BcInstPtr as *const libc::c_void,
          );
          return BC_STATUS_SUCCESS;
        }
      }
    }
    _ => {}
  }
  match current_block {
    5837603160108855842 => {
      bc_vec_pop(&mut (*ptr_to_globals).prog.results);
    }
    _ => {}
  }
  return s;
}
// ENABLE_DC
unsafe extern "C" fn xc_program_pushGlobal(mut inst: libc::c_char) {
  let mut res: BcResult = BcResult {
    t: XC_RESULT_TEMP,
    d: BcResultData {
      n: BcNum {
        num: 0 as *mut BcDig,
        rdx: 0,
        len: 0,
        cap: 0,
        neg: false,
      },
    },
  };
  let mut val: libc::c_ulong = 0;
  res.t = (inst as libc::c_int - XC_INST_IBASE as libc::c_int + XC_RESULT_IBASE as libc::c_int)
    as BcResultType;
  if inst as libc::c_int == XC_INST_IBASE as libc::c_int {
    val = (*ptr_to_globals).prog.ib_t
  } else if inst as libc::c_int == XC_INST_SCALE as libc::c_int {
    val = (*ptr_to_globals).prog.scale
  } else {
    val = (*ptr_to_globals).prog.ob_t
  }
  bc_num_init_DEF_SIZE(&mut res.d.n);
  bc_num_ulong2num(&mut res.d.n, val);
  bc_vec_push(
    &mut (*ptr_to_globals).prog.results,
    &mut res as *mut BcResult as *const libc::c_void,
  );
}
unsafe extern "C" fn zxc_program_exec() -> BcStatus {
  let mut current_block: u64;
  let mut r: BcResult = BcResult {
    t: XC_RESULT_TEMP,
    d: BcResultData {
      n: BcNum {
        num: 0 as *mut BcDig,
        rdx: 0,
        len: 0,
        cap: 0,
        neg: false,
      },
    },
  };
  let mut ptr: *mut BcResult = 0 as *mut BcResult;
  let mut ip: *mut BcInstPtr = bc_vec_top(&mut (*ptr_to_globals).prog.exestack) as *mut BcInstPtr;
  let mut func: *mut BcFunc = xc_program_func((*ip).func);
  let mut code: *mut libc::c_char = (*func).code.v;
  while (*ip).inst_idx < (*func).code.len {
    let mut s: BcStatus = BC_STATUS_SUCCESS;
    let fresh18 = (*ip).inst_idx;
    (*ip).inst_idx = (*ip).inst_idx.wrapping_add(1);
    let mut inst: libc::c_char = *code.offset(fresh18 as isize);
    match inst as libc::c_int {
      48 => {
        if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(0) as libc::c_int != 'b' as i32) {
          // end of '?' reached
          bc_vec_pop(&mut (*ptr_to_globals).prog.exestack);
          current_block = 8122785073741256984;
        } else {
          current_block = 13536709405535804910;
        }
      }
      47 => {
        current_block = 13536709405535804910;
      }
      45 => {
        let mut num: *mut BcNum = 0 as *mut BcNum;
        let mut zero: bool = false;
        s = zxc_program_prep(&mut ptr, &mut num);
        if s as u64 != 0 {
          return s;
        }
        zero = bc_num_cmp(num, &mut (*ptr_to_globals).prog.zero) == 0i32 as libc::c_long;
        bc_vec_pop(&mut (*ptr_to_globals).prog.results);
        if !zero {
          xc_program_index(code, &mut (*ip).inst_idx);
          current_block = 4299703460566765016;
        } else {
          current_block = 18317007320854588510;
        }
        // else: fall through
      }
      44 => {
        current_block = 18317007320854588510;
      }
      46 => {
        s = zbc_program_call(code, &mut (*ip).inst_idx);
        current_block = 8122785073741256984;
      }
      0 | 1 | 2 | 3 => {
        s = zbc_program_incdec(inst);
        current_block = 4299703460566765016;
      }
      43 => {
        quit();
      }
      18 | 19 | 5 | 6 | 7 | 8 | 9 | 10 => {
        // ENABLE_BC
        s = zxc_program_logical(inst);
        current_block = 4299703460566765016;
      }
      37 => {
        s = zxc_program_read();
        current_block = 8122785073741256984;
      }
      28 => {
        s = xc_program_pushVar(code, &mut (*ip).inst_idx, 0i32 != 0, 0i32 != 0);
        current_block = 4299703460566765016;
      }
      29 | 30 => {
        s = zbc_program_pushArray(code, &mut (*ip).inst_idx, inst);
        current_block = 4299703460566765016;
      }
      35 => {
        r.t = BC_RESULT_LAST;
        bc_vec_push(
          &mut (*ptr_to_globals).prog.results,
          &mut r as *mut BcResult as *const libc::c_void,
        );
        current_block = 4299703460566765016;
      }
      32 | 33 | 34 => {
        xc_program_pushGlobal(inst);
        current_block = 4299703460566765016;
      }
      31 | 36 | 38 => {
        s = zxc_program_builtin(inst);
        current_block = 4299703460566765016;
      }
      27 => {
        r.t = XC_RESULT_CONSTANT;
        r.d.id.idx = xc_program_index(code, &mut (*ip).inst_idx);
        bc_vec_push(
          &mut (*ptr_to_globals).prog.results,
          &mut r as *mut BcResult as *const libc::c_void,
        );
        current_block = 4299703460566765016;
      }
      49 => {
        if !((*ptr_to_globals).prog.results.len > 0i32 as size_t) {
          s = bc_error_stack_has_too_few_elements() as BcStatus
        } else {
          bc_vec_pop(&mut (*ptr_to_globals).prog.results);
        }
        current_block = 4299703460566765016;
      }
      39 | 40 | 42 => {
        s = xc_program_print(inst, 0i32 as size_t);
        current_block = 4299703460566765016;
      }
      41 => {
        r.t = XC_RESULT_STR;
        r.d.id.idx = xc_program_index(code, &mut (*ip).inst_idx);
        bc_vec_push(
          &mut (*ptr_to_globals).prog.results,
          &mut r as *mut BcResult as *const libc::c_void,
        );
        current_block = 4299703460566765016;
      }
      11 | 12 | 13 | 14 | 15 | 16 => {
        s = zxc_program_op(inst);
        current_block = 4299703460566765016;
      }
      17 => {
        let mut num_0: *mut BcNum = 0 as *mut BcNum;
        s = zxc_program_prep(&mut ptr, &mut num_0);
        if s as u64 != 0 {
          return s;
        }
        bc_num_init_DEF_SIZE(&mut r.d.n);
        if bc_num_cmp(num_0, &mut (*ptr_to_globals).prog.zero) == 0i32 as libc::c_long {
          bc_num_one(&mut r.d.n);
        }
        //else bc_num_zero(&r.d.n); - already is
        xc_program_retire(&mut r, XC_RESULT_TEMP);
        current_block = 4299703460566765016;
      }
      4 => {
        s = zxc_program_negate();
        current_block = 4299703460566765016;
      }
      20 | 21 | 22 | 23 | 24 | 25 | 26 => {
        s = zxc_program_assign(inst);
        current_block = 4299703460566765016;
      }
      50 => {
        bc_vec_pop(&mut (*ptr_to_globals).prog.exestack);
        current_block = 8122785073741256984;
      }
      51 => {
        s = zdc_program_modexp();
        current_block = 4299703460566765016;
      }
      52 => {
        s = zdc_program_divmod();
        current_block = 4299703460566765016;
      }
      53 | 54 => {
        s = zdc_program_execStr(
          code,
          &mut (*ip).inst_idx,
          inst as libc::c_int == DC_INST_EXEC_COND as libc::c_int,
        );
        current_block = 8122785073741256984;
      }
      57 => {
        let mut idx_0: size_t = 0;
        idx_0 = 0i32 as size_t;
        while idx_0 < (*ptr_to_globals).prog.results.len {
          s = xc_program_print(XC_INST_PRINT as libc::c_int as libc::c_char, idx_0);
          if s as u64 != 0 {
            break;
          }
          idx_0 = idx_0.wrapping_add(1)
        }
        current_block = 4299703460566765016;
      }
      58 => {
        bc_vec_pop_all(&mut (*ptr_to_globals).prog.results);
        current_block = 4299703460566765016;
      }
      59 => {
        dc_program_stackLen();
        current_block = 4299703460566765016;
      }
      60 => {
        if !((*ptr_to_globals).prog.results.len > 0i32 as size_t) {
          return bc_error_stack_has_too_few_elements() as BcStatus;
        }
        ptr = bc_vec_top(&mut (*ptr_to_globals).prog.results) as *mut BcResult;
        dc_result_copy(&mut r, ptr);
        bc_vec_push(
          &mut (*ptr_to_globals).prog.results,
          &mut r as *mut BcResult as *const libc::c_void,
        );
        current_block = 4299703460566765016;
      }
      61 => {
        let mut ptr2: *mut BcResult = 0 as *mut BcResult;
        if !((*ptr_to_globals).prog.results.len > 1i32 as size_t) {
          return bc_error_stack_has_too_few_elements() as BcStatus;
        }
        ptr = bc_vec_item_rev(&mut (*ptr_to_globals).prog.results, 0i32 as size_t) as *mut BcResult;
        ptr2 =
          bc_vec_item_rev(&mut (*ptr_to_globals).prog.results, 1i32 as size_t) as *mut BcResult;
        memcpy(
          &mut r as *mut BcResult as *mut libc::c_void,
          ptr as *const libc::c_void,
          ::std::mem::size_of::<BcResult>() as libc::c_ulong,
        );
        memcpy(
          ptr as *mut libc::c_void,
          ptr2 as *const libc::c_void,
          ::std::mem::size_of::<BcResult>() as libc::c_ulong,
        );
        memcpy(
          ptr2 as *mut libc::c_void,
          &mut r as *mut BcResult as *const libc::c_void,
          ::std::mem::size_of::<BcResult>() as libc::c_ulong,
        );
        current_block = 4299703460566765016;
      }
      55 => {
        s = zdc_program_asciify();
        current_block = 4299703460566765016;
      }
      56 => {
        s = zdc_program_printStream();
        current_block = 4299703460566765016;
      }
      62 | 63 => {
        let mut copy: bool = inst as libc::c_int == DC_INST_LOAD as libc::c_int;
        s = xc_program_pushVar(code, &mut (*ip).inst_idx, 1i32 != 0, copy);
        current_block = 4299703460566765016;
      }
      64 => {
        let mut name: *mut libc::c_char = xc_program_name(code, &mut (*ip).inst_idx);
        s = zxc_program_popResultAndCopyToVar(name, BC_TYPE_VAR);
        free(name as *mut libc::c_void);
        current_block = 4299703460566765016;
      }
      65 => {
        if (*ptr_to_globals).prog.exestack.len <= 2i32 as libc::c_ulong {
          quit();
        }
        bc_vec_npop(&mut (*ptr_to_globals).prog.exestack, 2i32 as size_t);
        current_block = 8122785073741256984;
      }
      66 => {
        s = zdc_program_nquit();
        current_block = 8122785073741256984;
      }
      _ => {
        current_block = 4299703460566765016;
      }
    }
    match current_block {
      18317007320854588510 => {
        let mut idx: size_t = xc_program_index(code, &mut (*ip).inst_idx);
        let mut addr: *mut size_t = bc_vec_item(&mut (*func).labels, idx) as *mut size_t;
        (*ip).inst_idx = *addr;
        current_block = 4299703460566765016;
      }
      13536709405535804910 =>
      // bc: fall through
      {
        s = zbc_program_return(inst);
        current_block = 8122785073741256984;
      }
      _ => {}
    }
    match current_block {
      8122785073741256984 =>
      //goto read_updated_ip; - just fall through to it
      // ENABLE_DC
      // Instruction stack has changed, read new pointers
      {
        ip = bc_vec_top(&mut (*ptr_to_globals).prog.exestack) as *mut BcInstPtr; // does the first zxc_lex_next()
        func = xc_program_func((*ip).func);
        code = (*func).code.v
      }
      _ => {}
    }
    if s as libc::c_uint != 0 || bb_got_signal as libc::c_int != 0 {
      xc_program_reset();
      return s;
    }
    fflush_and_check();
  }
  return BC_STATUS_SUCCESS;
}
unsafe extern "C" fn xc_vm_envLen(mut var: *const libc::c_char) -> libc::c_uint {
  let mut lenv: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut len: libc::c_uint = 0;
  lenv = getenv(var);
  len = 69i32 as libc::c_uint;
  if lenv.is_null() {
    return len;
  }
  len = bb_strtou(lenv, 0 as *mut *mut libc::c_char, 10i32).wrapping_sub(1i32 as libc::c_uint);
  if *bb_errno != 0 || len < 2i32 as libc::c_uint || len >= 2147483647i32 as libc::c_uint {
    len = 69i32 as libc::c_uint
  }
  return len;
}
unsafe extern "C" fn zxc_vm_process(mut text: *const libc::c_char) -> BcStatus {
  let mut current_block: u64;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  s = zxc_parse_text_init(text);
  if s as u64 != 0 {
    return s;
  }
  while (*ptr_to_globals).prs.lex as libc::c_int != XC_LEX_EOF as libc::c_int {
    let mut ip: *mut BcInstPtr = 0 as *mut BcInstPtr;
    let mut f: *mut BcFunc = 0 as *mut BcFunc;
    if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(0) as libc::c_int == 'b' as i32) {
      s = zbc_parse_stmt_or_funcdef();
      if s as u64 != 0 {
        current_block = 16954902497165895845;
      } else if (*ptr_to_globals).prs.lex as libc::c_int != BC_LEX_SCOLON as libc::c_int
        && (*ptr_to_globals).prs.lex as libc::c_int != XC_LEX_NLINE as libc::c_int
        && (*ptr_to_globals).prs.lex as libc::c_int != XC_LEX_EOF as libc::c_int
      {
        bc_error_at(b"bad statement terminator\x00" as *const u8 as *const libc::c_char);
        current_block = 16954902497165895845;
      } else {
        current_block = 10048703153582371463;
      }
    // The above logic is fragile. Check these examples:
    // - interactive read() still works
    } else {
      s = zdc_parse_expr();
      current_block = 10048703153582371463;
    }
    match current_block {
      10048703153582371463 => {
        if !(s as libc::c_uint != 0 || bb_got_signal as libc::c_int != 0) {
          s = zxc_program_exec();
          if s as u64 != 0 {
            xc_program_reset();
            break;
          } else {
            ip = (*ptr_to_globals).prog.exestack.v as *mut libc::c_void as *mut BcInstPtr;
            if (*ptr_to_globals).prog.exestack.len != 1i32 as libc::c_ulong {
              // Check that next token is a correct stmt delimiter -
              // disallows "print 1 print 2" and such.
              // should have only main's IP
              bb_simple_error_msg_and_die(
                b"BUG:call stack\x00" as *const u8 as *const libc::c_char,
              );
            }
            if (*ip).func != 0i32 as libc::c_ulong {
              bb_simple_error_msg_and_die(b"BUG:not MAIN\x00" as *const u8 as *const libc::c_char);
            }
            f = (*ptr_to_globals).prog.fns.v as *mut BcFunc;
            // bc discards strings, constants and code after each
            // top-level statement in the "main program".
            // This prevents "yes 1 | bc" from growing its memory
            // without bound. This can be done because data stack
            // is empty and thus can't hold any references to
            // strings or constants, there is no generated code
            // which can hold references (after we discard one
            // we just executed). Code of functions can have references,
            // but bc stores function strings/constants in per-function
            // storage.
            if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(0) as libc::c_int == 'b' as i32) {
              if (*ptr_to_globals).prog.results.len != 0i32 as libc::c_ulong {
                // should be empty
                bb_simple_error_msg_and_die(
                  b"BUG:data stack\x00" as *const u8 as *const libc::c_char,
                );
              }
              bc_vec_pop_all(&mut (*f).strs);
              bc_vec_pop_all(&mut (*f).consts);
              // We are at SCOLON/NLINE, skip it:
              s = zxc_lex_next();
              if s as u64 != 0 {
                current_block = 16954902497165895845;
              } else {
                current_block = 5529461102203738653;
              }
            } else {
              if (*ptr_to_globals).prog.results.len == 0i32 as libc::c_ulong
                && (*ptr_to_globals).prog.vars.len == 0i32 as libc::c_ulong
              {
                // If stack is empty and no registers exist (TODO: or they are all empty),
                // we can get rid of accumulated strings and constants.
                // In this example dc process should not grow
                // its memory consumption with time:
                // yes 1pc | dc
                bc_vec_pop_all(&mut (*ptr_to_globals).prog.strs);
                bc_vec_pop_all(&mut (*ptr_to_globals).prog.consts);
              }
              current_block = 5529461102203738653;
              // The code is discarded always (below), thus this example
              // should also not grow its memory consumption with time,
              // even though its data stack is not empty:
              // { echo 1; yes dk; } | dc
            }
            match current_block {
              16954902497165895845 => {}
              _ => {
                // We drop generated and executed code for both bc and dc:
                bc_vec_pop_all(&mut (*f).code); // includes xc_program_reset()
                (*ip).inst_idx = 0i32 as size_t;
                continue;
              }
            }
          }
        }
      }
      _ => {}
    }
    xc_parse_reset();
    return BC_STATUS_FAILURE;
  }
  return s;
}
unsafe extern "C" fn zxc_vm_execute_FILE(
  mut fp: *mut FILE,
  mut filename: *const libc::c_char,
) -> BcStatus {
  // So far bc/dc have no way to include a file from another file,
  // therefore we know G.prs.lex_filename == NULL on entry
  //const char *sv_file;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  (*ptr_to_globals).prs.lex_filename = filename;
  (*ptr_to_globals).prs.lex_input_fp = fp;
  (*ptr_to_globals).prs.lex_line = 1i32 as size_t;
  (*ptr_to_globals).err_line = (*ptr_to_globals).prs.lex_line;
  loop {
    s = zxc_vm_process(b"\x00" as *const u8 as *const libc::c_char);
    if !((*ptr_to_globals).prs.lex_input_fp == stdin) {
      break;
    }
    // We do not stop looping on errors here if reading stdin.
    // Example: start interactive bc and enter "return".
    // It should say "'return' not in a function"
    // but should not exit.
  }
  (*ptr_to_globals).prs.lex_filename = 0 as *const libc::c_char;
  return s;
}
unsafe extern "C" fn zxc_vm_file(mut file: *const libc::c_char) -> BcStatus {
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut fp: *mut FILE = 0 as *mut FILE;
  fp = xfopen_for_read(file);
  s = zxc_vm_execute_FILE(fp, file);
  fclose(fp);
  return s;
}
unsafe extern "C" fn bc_vm_info() {
  printf(b"%s 1.32.0.git\nAdapted from https://github.com/gavinhoward/bc\nOriginal code (c) 2018 Gavin D. Howard and contributors\n\x00"
               as *const u8 as *const libc::c_char, applet_name);
}
unsafe extern "C" fn bc_args(mut argv: *mut *mut libc::c_char) {
  let mut opts: libc::c_uint = 0;
  let mut i: libc::c_int = 0;
  optind = 0i32;
  option_mask32 |=
        getopt32long(argv, b"wvsqli\x00" as *const u8 as *const libc::c_char,
                     b"warn\x00\x00wversion\x00\x00vstandard\x00\x00squiet\x00\x00qmathlib\x00\x00linteractive\x00\x00i\x00"
                         as *const u8 as *const libc::c_char);
  opts = option_mask32;
  if !getenv(b"POSIXLY_CORRECT\x00" as *const u8 as *const libc::c_char).is_null() {
    option_mask32 |= (1i32 << 2i32) as libc::c_uint
  }
  if opts & (1i32 << 1i32) as libc::c_uint != 0 {
    bc_vm_info();
    exit(0i32);
  }
  i = optind;
  while !(*argv.offset(i as isize)).is_null() {
    bc_vec_push(
      &mut (*ptr_to_globals).files,
      argv.offset(i as isize) as *const libc::c_void,
    );
    i += 1
  }
}
unsafe extern "C" fn bc_vm_envArgs() {
  let mut v: BcVec = BcVec {
    v: 0 as *mut libc::c_char,
    len: 0,
    cap: 0,
    size: 0,
    dtor: None,
  };
  let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut env_args: *mut libc::c_char =
    getenv(b"BC_ENV_ARGS\x00" as *const u8 as *const libc::c_char);
  if env_args.is_null() {
    return;
  }
  (*ptr_to_globals).env_args = xstrdup(env_args);
  buf = (*ptr_to_globals).env_args;
  bc_vec_init(
    &mut v,
    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    None,
  );
  loop {
    buf = skip_whitespace(buf);
    if !(*buf as libc::c_int != '\u{0}' as i32) {
      break;
    }
    bc_vec_push(
      &mut v,
      &mut buf as *mut *mut libc::c_char as *const libc::c_void,
    );
    buf = skip_non_whitespace(buf);
    if *buf == 0 {
      break;
    }
    let fresh19 = buf;
    buf = buf.offset(1);
    *fresh19 = '\u{0}' as i32 as libc::c_char
  }
  // NULL terminate, and pass argv[] so that first arg is argv[1]
  if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
    == ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong
  {
    bc_vec_push(
      &mut v,
      &const_int_0 as *const libc::c_int as *const libc::c_void,
    );
  } else {
    static mut nullptr: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    bc_vec_push(
      &mut v,
      &nullptr as *const *mut libc::c_char as *const libc::c_void,
    );
  }
  bc_args((v.v as *mut *mut libc::c_char).offset(-1));
  bc_vec_free(&mut v as *mut BcVec as *mut libc::c_void);
}
static mut bc_lib: [libc::c_char; 1584] = [
  115, 99, 97, 108, 101, 61, 50, 48, 10, 100, 101, 102, 105, 110, 101, 32, 101, 40, 120, 41, 123,
  10, 97, 117, 116, 111, 32, 98, 44, 115, 44, 110, 44, 114, 44, 100, 44, 105, 44, 112, 44, 102, 44,
  118, 10, 98, 61, 105, 98, 97, 115, 101, 10, 105, 98, 97, 115, 101, 61, 65, 10, 105, 102, 40, 120,
  60, 48, 41, 123, 10, 110, 61, 49, 10, 120, 61, 45, 120, 10, 125, 10, 115, 61, 115, 99, 97, 108,
  101, 10, 114, 61, 54, 43, 115, 43, 46, 52, 52, 42, 120, 10, 115, 99, 97, 108, 101, 61, 115, 99,
  97, 108, 101, 40, 120, 41, 43, 49, 10, 119, 104, 105, 108, 101, 40, 120, 62, 49, 41, 123, 10,
  100, 43, 61, 49, 10, 120, 47, 61, 50, 10, 115, 99, 97, 108, 101, 43, 61, 49, 10, 125, 10, 115,
  99, 97, 108, 101, 61, 114, 10, 114, 61, 120, 43, 49, 10, 112, 61, 120, 10, 102, 61, 118, 61, 49,
  10, 102, 111, 114, 40, 105, 61, 50, 59, 118, 59, 43, 43, 105, 41, 123, 10, 112, 42, 61, 120, 10,
  102, 42, 61, 105, 10, 118, 61, 112, 47, 102, 10, 114, 43, 61, 118, 10, 125, 10, 119, 104, 105,
  108, 101, 40, 100, 45, 45, 41, 114, 42, 61, 114, 10, 115, 99, 97, 108, 101, 61, 115, 10, 105, 98,
  97, 115, 101, 61, 98, 10, 105, 102, 40, 110, 41, 114, 101, 116, 117, 114, 110, 40, 49, 47, 114,
  41, 10, 114, 101, 116, 117, 114, 110, 40, 114, 47, 49, 41, 10, 125, 10, 100, 101, 102, 105, 110,
  101, 32, 108, 40, 120, 41, 123, 10, 97, 117, 116, 111, 32, 98, 44, 115, 44, 114, 44, 112, 44, 97,
  44, 113, 44, 105, 44, 118, 10, 98, 61, 105, 98, 97, 115, 101, 10, 105, 98, 97, 115, 101, 61, 65,
  10, 105, 102, 40, 120, 60, 61, 48, 41, 123, 10, 114, 61, 40, 49, 45, 49, 48, 94, 115, 99, 97,
  108, 101, 41, 47, 49, 10, 105, 98, 97, 115, 101, 61, 98, 10, 114, 101, 116, 117, 114, 110, 40,
  114, 41, 10, 125, 10, 115, 61, 115, 99, 97, 108, 101, 10, 115, 99, 97, 108, 101, 43, 61, 54, 10,
  112, 61, 50, 10, 119, 104, 105, 108, 101, 40, 120, 62, 61, 50, 41, 123, 10, 112, 42, 61, 50, 10,
  120, 61, 115, 113, 114, 116, 40, 120, 41, 10, 125, 10, 119, 104, 105, 108, 101, 40, 120, 60, 61,
  46, 53, 41, 123, 10, 112, 42, 61, 50, 10, 120, 61, 115, 113, 114, 116, 40, 120, 41, 10, 125, 10,
  114, 61, 97, 61, 40, 120, 45, 49, 41, 47, 40, 120, 43, 49, 41, 10, 113, 61, 97, 42, 97, 10, 118,
  61, 49, 10, 102, 111, 114, 40, 105, 61, 51, 59, 118, 59, 105, 43, 61, 50, 41, 123, 10, 97, 42,
  61, 113, 10, 118, 61, 97, 47, 105, 10, 114, 43, 61, 118, 10, 125, 10, 114, 42, 61, 112, 10, 115,
  99, 97, 108, 101, 61, 115, 10, 105, 98, 97, 115, 101, 61, 98, 10, 114, 101, 116, 117, 114, 110,
  40, 114, 47, 49, 41, 10, 125, 10, 100, 101, 102, 105, 110, 101, 32, 115, 40, 120, 41, 123, 10,
  97, 117, 116, 111, 32, 98, 44, 115, 44, 114, 44, 97, 44, 113, 44, 105, 10, 105, 102, 40, 120, 60,
  48, 41, 114, 101, 116, 117, 114, 110, 40, 45, 115, 40, 45, 120, 41, 41, 10, 98, 61, 105, 98, 97,
  115, 101, 10, 105, 98, 97, 115, 101, 61, 65, 10, 115, 61, 115, 99, 97, 108, 101, 10, 115, 99, 97,
  108, 101, 61, 49, 46, 49, 42, 115, 43, 50, 10, 97, 61, 97, 40, 49, 41, 10, 115, 99, 97, 108, 101,
  61, 48, 10, 113, 61, 40, 120, 47, 97, 43, 50, 41, 47, 52, 10, 120, 45, 61, 52, 42, 113, 42, 97,
  10, 105, 102, 40, 113, 37, 50, 41, 120, 61, 45, 120, 10, 115, 99, 97, 108, 101, 61, 115, 43, 50,
  10, 114, 61, 97, 61, 120, 10, 113, 61, 45, 120, 42, 120, 10, 102, 111, 114, 40, 105, 61, 51, 59,
  97, 59, 105, 43, 61, 50, 41, 123, 10, 97, 42, 61, 113, 47, 40, 105, 42, 40, 105, 45, 49, 41, 41,
  10, 114, 43, 61, 97, 10, 125, 10, 115, 99, 97, 108, 101, 61, 115, 10, 105, 98, 97, 115, 101, 61,
  98, 10, 114, 101, 116, 117, 114, 110, 40, 114, 47, 49, 41, 10, 125, 10, 100, 101, 102, 105, 110,
  101, 32, 99, 40, 120, 41, 123, 10, 97, 117, 116, 111, 32, 98, 44, 115, 10, 98, 61, 105, 98, 97,
  115, 101, 10, 105, 98, 97, 115, 101, 61, 65, 10, 115, 61, 115, 99, 97, 108, 101, 10, 115, 99, 97,
  108, 101, 42, 61, 49, 46, 50, 10, 120, 61, 115, 40, 50, 42, 97, 40, 49, 41, 43, 120, 41, 10, 115,
  99, 97, 108, 101, 61, 115, 10, 105, 98, 97, 115, 101, 61, 98, 10, 114, 101, 116, 117, 114, 110,
  40, 120, 47, 49, 41, 10, 125, 10, 100, 101, 102, 105, 110, 101, 32, 97, 40, 120, 41, 123, 10, 97,
  117, 116, 111, 32, 98, 44, 115, 44, 114, 44, 110, 44, 97, 44, 109, 44, 116, 44, 102, 44, 105, 44,
  117, 10, 98, 61, 105, 98, 97, 115, 101, 10, 105, 98, 97, 115, 101, 61, 65, 10, 110, 61, 49, 10,
  105, 102, 40, 120, 60, 48, 41, 123, 10, 110, 61, 45, 49, 10, 120, 61, 45, 120, 10, 125, 10, 105,
  102, 40, 115, 99, 97, 108, 101, 60, 54, 53, 41, 123, 10, 105, 102, 40, 120, 61, 61, 49, 41, 114,
  101, 116, 117, 114, 110, 40, 46, 55, 56, 53, 51, 57, 56, 49, 54, 51, 51, 57, 55, 52, 52, 56, 51,
  48, 57, 54, 49, 53, 54, 54, 48, 56, 52, 53, 56, 49, 57, 56, 55, 53, 55, 50, 49, 48, 52, 57, 50,
  57, 50, 51, 52, 57, 56, 52, 51, 55, 55, 54, 52, 53, 53, 50, 52, 51, 55, 51, 54, 49, 52, 56, 48,
  47, 110, 41, 10, 105, 102, 40, 120, 61, 61, 46, 50, 41, 114, 101, 116, 117, 114, 110, 40, 46, 49,
  57, 55, 51, 57, 53, 53, 53, 57, 56, 52, 57, 56, 56, 48, 55, 53, 56, 51, 55, 48, 48, 52, 57, 55,
  54, 53, 49, 57, 52, 55, 57, 48, 50, 57, 51, 52, 52, 55, 53, 56, 53, 49, 48, 51, 55, 56, 55, 56,
  53, 50, 49, 48, 49, 53, 49, 55, 54, 56, 56, 57, 52, 48, 50, 47, 110, 41, 10, 125, 10, 115, 61,
  115, 99, 97, 108, 101, 10, 105, 102, 40, 120, 62, 46, 50, 41, 123, 10, 115, 99, 97, 108, 101, 43,
  61, 53, 10, 97, 61, 97, 40, 46, 50, 41, 10, 125, 10, 115, 99, 97, 108, 101, 61, 115, 43, 51, 10,
  119, 104, 105, 108, 101, 40, 120, 62, 46, 50, 41, 123, 10, 109, 43, 61, 49, 10, 120, 61, 40, 120,
  45, 46, 50, 41, 47, 40, 49, 43, 46, 50, 42, 120, 41, 10, 125, 10, 114, 61, 117, 61, 120, 10, 102,
  61, 45, 120, 42, 120, 10, 116, 61, 49, 10, 102, 111, 114, 40, 105, 61, 51, 59, 116, 59, 105, 43,
  61, 50, 41, 123, 10, 117, 42, 61, 102, 10, 116, 61, 117, 47, 105, 10, 114, 43, 61, 116, 10, 125,
  10, 115, 99, 97, 108, 101, 61, 115, 10, 105, 98, 97, 115, 101, 61, 98, 10, 114, 101, 116, 117,
  114, 110, 40, 40, 109, 42, 97, 43, 114, 41, 47, 110, 41, 10, 125, 10, 100, 101, 102, 105, 110,
  101, 32, 106, 40, 110, 44, 120, 41, 123, 10, 97, 117, 116, 111, 32, 98, 44, 115, 44, 111, 44, 97,
  44, 105, 44, 118, 44, 102, 10, 98, 61, 105, 98, 97, 115, 101, 10, 105, 98, 97, 115, 101, 61, 65,
  10, 115, 61, 115, 99, 97, 108, 101, 10, 115, 99, 97, 108, 101, 61, 48, 10, 110, 47, 61, 49, 10,
  105, 102, 40, 110, 60, 48, 41, 123, 10, 110, 61, 45, 110, 10, 111, 61, 110, 37, 50, 10, 125, 10,
  97, 61, 49, 10, 102, 111, 114, 40, 105, 61, 50, 59, 105, 60, 61, 110, 59, 43, 43, 105, 41, 97,
  42, 61, 105, 10, 115, 99, 97, 108, 101, 61, 49, 46, 53, 42, 115, 10, 97, 61, 40, 120, 94, 110,
  41, 47, 50, 94, 110, 47, 97, 10, 114, 61, 118, 61, 49, 10, 102, 61, 45, 120, 42, 120, 47, 52, 10,
  115, 99, 97, 108, 101, 43, 61, 108, 101, 110, 103, 116, 104, 40, 97, 41, 45, 115, 99, 97, 108,
  101, 40, 97, 41, 10, 102, 111, 114, 40, 105, 61, 49, 59, 118, 59, 43, 43, 105, 41, 123, 10, 118,
  61, 118, 42, 102, 47, 105, 47, 40, 110, 43, 105, 41, 10, 114, 43, 61, 118, 10, 125, 10, 115, 99,
  97, 108, 101, 61, 115, 10, 105, 98, 97, 115, 101, 61, 98, 10, 105, 102, 40, 111, 41, 97, 61, 45,
  97, 10, 114, 101, 116, 117, 114, 110, 40, 97, 42, 114, 47, 49, 41, 10, 125, 0,
];
// ENABLE_BC
unsafe extern "C" fn zxc_vm_exec() -> BcStatus {
  let mut fname: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
  let mut s: BcStatus = BC_STATUS_SUCCESS;
  let mut i: size_t = 0;
  if option_mask32 & (1i32 << 4i32) as libc::c_uint != 0 {
    // We know that internal library is not buggy,
    // thus error checking is normally disabled.
    s = zxc_vm_process(bc_lib.as_ptr());
    if 0i32 != 0 && s as libc::c_uint != 0 {
      return s;
    }
  }
  s = BC_STATUS_SUCCESS;
  fname = (*ptr_to_globals).files.v as *mut libc::c_void as *mut *mut libc::c_char;
  i = 0i32 as size_t;
  while i < (*ptr_to_globals).files.len {
    let fresh20 = fname;
    fname = fname.offset(1);
    s = zxc_vm_file(*fresh20);
    if 0i32 != 0 && (*ptr_to_globals).ttyin == 0 && s as libc::c_uint != 0 {
      // Debug config, non-interactive mode:
      // return all the way back to main.
      // Non-debug builds do not come here
      // in non-interactive mode, they exit.
      return s;
    }
    i = i.wrapping_add(1)
  }
  if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(0) as libc::c_int == 'b' as i32)
    || option_mask32 & ((1i32 << 5i32) * 1i32) as libc::c_uint != 0
  {
    s = zxc_vm_execute_FILE(stdin, 0 as *const libc::c_char)
  }
  return s;
}
unsafe extern "C" fn xc_program_init() {
  let mut ip: BcInstPtr = BcInstPtr {
    func: 0,
    inst_idx: 0,
  };
  // memset(&G.prog, 0, sizeof(G.prog)); - already is
  memset(
    &mut ip as *mut BcInstPtr as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<BcInstPtr>() as libc::c_ulong,
  );
  // G.prog.nchars = G.prog.scale = 0; - already is
  (*ptr_to_globals).prog.ib_t = 10i32 as size_t;
  (*ptr_to_globals).prog.ob_t = 10i32 as size_t;
  bc_num_init_DEF_SIZE(&mut (*ptr_to_globals).prog.last);
  //IF_BC(bc_num_zero(&G.prog.last);) - already is
  //bc_num_init_DEF_SIZE(&G.prog.zero); - not needed
  //bc_num_zero(&G.prog.zero); - already is
  bc_num_init_DEF_SIZE(&mut (*ptr_to_globals).prog.one);
  bc_num_one(&mut (*ptr_to_globals).prog.one);
  bc_vec_init(
    &mut (*ptr_to_globals).prog.fns,
    ::std::mem::size_of::<BcFunc>() as libc::c_ulong,
    Some(bc_func_free as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
  );
  bc_vec_init(
    &mut (*ptr_to_globals).prog.fn_map,
    ::std::mem::size_of::<BcId>() as libc::c_ulong,
    Some(bc_id_free as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
  );
  if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(0) as libc::c_int == 'b' as i32) {
    // Names are chosen simply to be distinct and never match
    // a valid function name (and be short)
    bc_program_addFunc(xstrdup(b"\x00" as *const u8 as *const libc::c_char));
    bc_program_addFunc(xstrdup(b"1\x00" as *const u8 as *const libc::c_char)); // func #0: main
                                                                               // func #1: for read()
  } else {
    // in dc, functions have no names
    xc_program_add_fn();
    xc_program_add_fn();
  }
  bc_vec_init(
    &mut (*ptr_to_globals).prog.vars,
    ::std::mem::size_of::<BcVec>() as libc::c_ulong,
    Some(bc_vec_free as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
  );
  bc_vec_init(
    &mut (*ptr_to_globals).prog.var_map,
    ::std::mem::size_of::<BcId>() as libc::c_ulong,
    Some(bc_id_free as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
  );
  bc_vec_init(
    &mut (*ptr_to_globals).prog.arrs,
    ::std::mem::size_of::<BcVec>() as libc::c_ulong,
    Some(bc_vec_free as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
  );
  bc_vec_init(
    &mut (*ptr_to_globals).prog.arr_map,
    ::std::mem::size_of::<BcId>() as libc::c_ulong,
    Some(bc_id_free as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
  );
  bc_vec_init(
    &mut (*ptr_to_globals).prog.strs,
    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    Some(bc_string_free as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
  );
  bc_vec_init(
    &mut (*ptr_to_globals).prog.consts,
    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    Some(bc_string_free as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
  );
  bc_vec_init(
    &mut (*ptr_to_globals).prog.results,
    ::std::mem::size_of::<BcResult>() as libc::c_ulong,
    Some(bc_result_free as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
  );
  bc_vec_init(
    &mut (*ptr_to_globals).prog.exestack,
    ::std::mem::size_of::<BcInstPtr>() as libc::c_ulong,
    None,
  );
  bc_vec_push(
    &mut (*ptr_to_globals).prog.exestack,
    &mut ip as *mut BcInstPtr as *const libc::c_void,
  );
  bc_char_vec_init(&mut (*ptr_to_globals).input_buffer);
}
unsafe extern "C" fn xc_vm_init(mut env_len: *const libc::c_char) -> libc::c_int {
  (*ptr_to_globals).prog.len = xc_vm_envLen(env_len) as size_t;
  (*ptr_to_globals).line_input_state = new_line_input_t(DO_HISTORY as libc::c_int);
  bc_vec_init(
    &mut (*ptr_to_globals).files,
    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    None,
  );
  xc_program_init();
  if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(0) as libc::c_int == 'b' as i32) {
    bc_vm_envArgs();
  }
  xc_parse_create(0i32 as size_t);
  //TODO: in GNU bc, the check is (isatty(0) && isatty(1)),
  //-i option unconditionally enables this regardless of isatty():
  if isatty(0i32) != 0 {
    (*ptr_to_globals).ttyin = 1i32 as smallint;
    // "tty"
    signal_SA_RESTART_empty_mask(
      2i32,
      Some(record_signo as unsafe extern "C" fn(_: libc::c_int) -> ()),
    );
    return 1i32;
  }
  return 0i32;
  // With SA_RESTART, most system calls will restart
  // (IOW: they won't fail with EINTR).
  // In particular, this means ^C won't cause
  // stdout to get into "error state" if SIGINT hits
  // within write() syscall.
  //
  // The downside is that ^C while tty input is taken
  // will only be handled after [Enter] since read()
  // from stdin is not interrupted by ^C either,
  // it restarts, thus fgetc() does not return on ^C.
  // (This problem manifests only if line editing is disabled)
  // Without SA_RESTART, this exhibits a bug:
  // "while (1) print 1" and try ^C-ing it.
  // Intermittently, instead of returning to input line,
  // you'll get "output error: Interrupted system call"
  // and exit.
  //signal_no_SA_RESTART_empty_mask(SIGINT, record_signo);
  // "not a tty"
}
unsafe extern "C" fn xc_vm_run() -> BcStatus {
  let mut st: BcStatus = zxc_vm_exec();
  return st;
}
#[no_mangle]
pub unsafe extern "C" fn bc_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut is_tty: libc::c_int = 0;
  let ref mut fresh21 =
    *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
      as *mut *mut globals);
  *fresh21 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  is_tty = xc_vm_init(b"BC_LINE_LENGTH\x00" as *const u8 as *const libc::c_char);
  bc_args(argv);
  if is_tty != 0 && option_mask32 & (1i32 << 3i32) as libc::c_uint == 0 {
    bc_vm_info();
  }
  return xc_vm_run() as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dc_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut noscript: libc::c_int = 0;
  let ref mut fresh22 =
    *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
      as *mut *mut globals);
  *fresh22 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  // TODO: dc (GNU bc 1.07.1) 1.4.1 seems to use width
  // 1 char wider than bc from the same package.
  // Both default width, and xC_LINE_LENGTH=N are wider:
  // "DC_LINE_LENGTH=5 dc -e'123456 p'" prints:
  //	|1234\   |
  //	|56      |
  // "echo '123456' | BC_LINE_LENGTH=5 bc" prints:
  //	|123\    |
  //	|456     |
  // Do the same, or it's a bug?
  xc_vm_init(b"DC_LINE_LENGTH\x00" as *const u8 as *const libc::c_char);
  // Run -e'SCRIPT' and -fFILE in order of appearance, then handle FILEs
  noscript = (1i32 << 5i32) * 1i32; // set BC_FLAG_I if we need to interpret stdin
  loop {
    let mut n: libc::c_int = getopt(argc, argv, b"e:f:x\x00" as *const u8 as *const libc::c_char);
    if n <= 0i32 {
      break;
    }
    match n {
      101 => {
        noscript = 0i32;
        n = zxc_vm_process(optarg) as libc::c_int;
        if n != 0 {
          return n;
        }
      }
      102 => {
        noscript = 0i32;
        n = zxc_vm_file(optarg) as libc::c_int;
        if n != 0 {
          return n;
        }
      }
      120 => option_mask32 |= ((1i32 << 6i32) * 1i32) as libc::c_uint,
      _ => {
        bb_show_usage();
      }
    }
  }
  argv = argv.offset(optind as isize);
  while !(*argv).is_null() {
    noscript = 0i32;
    let fresh23 = argv;
    argv = argv.offset(1);
    bc_vec_push(&mut (*ptr_to_globals).files, fresh23 as *const libc::c_void);
  }
  option_mask32 |= noscript as libc::c_uint;
  return xc_vm_run() as libc::c_int;
}
// DC_BIG
