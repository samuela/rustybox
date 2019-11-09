use libc;
use libc::unlink;


extern "C" {
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn itoa(n: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn config_open2(
    filename: *const libc::c_char,
    fopen_func: Option<unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE>,
  ) -> *mut parser_t;
  #[no_mangle]
  fn config_read(
    parser: *mut parser_t,
    tokens: *mut *mut libc::c_char,
    flags: libc::c_uint,
    delims: *const libc::c_char,
  ) -> libc::c_int;
  #[no_mangle]
  fn config_close(parser: *mut parser_t);
}

use crate::librb::size_t;


use libc::FILE;
pub type C2RustUnnamed = libc::c_uint;
pub const PARSE_NORMAL: C2RustUnnamed = 4653056;
pub const PARSE_WS_COMMENTS: C2RustUnnamed = 16777216;
pub const PARSE_ALT_COMMENTS: C2RustUnnamed = 8388608;
pub const PARSE_EOL_COMMENTS: C2RustUnnamed = 4194304;
pub const PARSE_KEEP_COPY: C2RustUnnamed = 2097152;
pub const PARSE_MIN_DIE: C2RustUnnamed = 1048576;
pub const PARSE_GREEDY: C2RustUnnamed = 262144;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rtnl_tab_t {
  pub cached_str: *const libc::c_char,
  pub cached_result: libc::c_uint,
  pub tab: [*const libc::c_char; 1024],
}
unsafe extern "C" fn rtnl_tab_initialize(
  mut file: *const libc::c_char,
  mut tab: *mut *const libc::c_char,
) {
  let mut token: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
  let mut fullname: [libc::c_char; 33] = [0; 33];
  let mut parser: *mut parser_t = 0 as *mut parser_t;
  sprintf(
    fullname.as_mut_ptr(),
    b"/etc/iproute2/rt_%s\x00" as *const u8 as *const libc::c_char,
    file,
  );
  parser = config_open2(
    fullname.as_mut_ptr(),
    Some(fopen_for_read as unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE),
  );
  while config_read(
    parser,
    token.as_mut_ptr(),
    (PARSE_NORMAL as libc::c_int | (2i32 & 0xffi32) << 8i32 | 2i32 & 0xffi32) as libc::c_uint,
    b"# \t\x00" as *const u8 as *const libc::c_char,
  ) != 0
  {
    let mut id: libc::c_uint = bb_strtou(token[0], 0 as *mut *mut libc::c_char, 0i32);
    if id > 1023i32 as libc::c_uint {
      bb_error_msg(
        b"database %s is corrupted at line %d\x00" as *const u8 as *const libc::c_char,
        file,
        (*parser).lineno,
      );
      break;
    } else {
      let ref mut fresh0 = *tab.offset(id as isize);
      *fresh0 = xstrdup(token[1])
    }
  }
  config_close(parser);
}
unsafe extern "C" fn rtnl_a2n(
  mut tab: *mut rtnl_tab_t,
  mut id: *mut u32,
  mut arg: *const libc::c_char,
  mut base: libc::c_int,
) -> libc::c_int {
  let mut i: libc::c_uint = 0;
  if !(*tab).cached_str.is_null() && strcmp((*tab).cached_str, arg) == 0i32 {
    *id = (*tab).cached_result;
    return 0i32;
  }
  i = 0i32 as libc::c_uint;
  while i <= 1023i32 as libc::c_uint {
    if !(*tab).tab[i as usize].is_null() && strcmp((*tab).tab[i as usize], arg) == 0i32 {
      (*tab).cached_str = (*tab).tab[i as usize];
      (*tab).cached_result = i;
      *id = i;
      return 0i32;
    }
    i = i.wrapping_add(1)
  }
  i = bb_strtou(arg, 0 as *mut *mut libc::c_char, base);
  if i > 1023i32 as libc::c_uint {
    return -1i32;
  }
  *id = i;
  return 0i32;
}
static mut rtnl_rtprot_tab: *mut rtnl_tab_t = 0 as *const rtnl_tab_t as *mut rtnl_tab_t;
unsafe extern "C" fn rtnl_rtprot_initialize() {
  static mut init_tab: [*const libc::c_char; 13] = [
    b"none\x00" as *const u8 as *const libc::c_char,
    b"redirect\x00" as *const u8 as *const libc::c_char,
    b"kernel\x00" as *const u8 as *const libc::c_char,
    b"boot\x00" as *const u8 as *const libc::c_char,
    b"static\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    b"gated\x00" as *const u8 as *const libc::c_char,
    b"ra\x00" as *const u8 as *const libc::c_char,
    b"mrt\x00" as *const u8 as *const libc::c_char,
    b"zebra\x00" as *const u8 as *const libc::c_char,
    b"bird\x00" as *const u8 as *const libc::c_char,
  ];
  if !rtnl_rtprot_tab.is_null() {
    return;
  }
  rtnl_rtprot_tab =
    xzalloc(::std::mem::size_of::<rtnl_tab_t>() as libc::c_ulong) as *mut rtnl_tab_t;
  memcpy(
    (*rtnl_rtprot_tab).tab.as_mut_ptr() as *mut libc::c_void,
    init_tab.as_ptr() as *const libc::c_void,
    ::std::mem::size_of::<[*const libc::c_char; 13]>() as libc::c_ulong,
  );
  rtnl_tab_initialize(
    b"protos\x00" as *const u8 as *const libc::c_char,
    (*rtnl_rtprot_tab).tab.as_mut_ptr(),
  );
}
/* UNUSED */
#[no_mangle]
pub unsafe extern "C" fn rtnl_rtprot_a2n(
  mut id: *mut u32,
  mut arg: *mut libc::c_char,
) -> libc::c_int {
  rtnl_rtprot_initialize();
  return rtnl_a2n(rtnl_rtprot_tab, id, arg, 0i32);
}
static mut rtnl_rtscope_tab: *mut rtnl_tab_t = 0 as *const rtnl_tab_t as *mut rtnl_tab_t;
unsafe extern "C" fn rtnl_rtscope_initialize() {
  if !rtnl_rtscope_tab.is_null() {
    return;
  }
  rtnl_rtscope_tab =
    xzalloc(::std::mem::size_of::<rtnl_tab_t>() as libc::c_ulong) as *mut rtnl_tab_t;
  (*rtnl_rtscope_tab).tab[0] = b"global\x00" as *const u8 as *const libc::c_char;
  (*rtnl_rtscope_tab).tab[255] = b"nowhere\x00" as *const u8 as *const libc::c_char;
  (*rtnl_rtscope_tab).tab[254] = b"host\x00" as *const u8 as *const libc::c_char;
  (*rtnl_rtscope_tab).tab[253] = b"link\x00" as *const u8 as *const libc::c_char;
  (*rtnl_rtscope_tab).tab[200] = b"site\x00" as *const u8 as *const libc::c_char;
  rtnl_tab_initialize(
    b"scopes\x00" as *const u8 as *const libc::c_char,
    (*rtnl_rtscope_tab).tab.as_mut_ptr(),
  );
}
#[no_mangle]
pub unsafe extern "C" fn rtnl_rtscope_n2a(mut id: libc::c_int) -> *const libc::c_char {
  if id < 0i32 || id > 1023i32 {
    return itoa(id);
  }
  rtnl_rtscope_initialize();
  if !(*rtnl_rtscope_tab).tab[id as usize].is_null() {
    return (*rtnl_rtscope_tab).tab[id as usize];
  }
  return itoa(id);
}
#[no_mangle]
pub unsafe extern "C" fn rtnl_rtscope_a2n(
  mut id: *mut u32,
  mut arg: *mut libc::c_char,
) -> libc::c_int {
  rtnl_rtscope_initialize();
  return rtnl_a2n(rtnl_rtscope_tab, id, arg, 0i32);
}
static mut rtnl_rtrealm_tab: *mut rtnl_tab_t = 0 as *const rtnl_tab_t as *mut rtnl_tab_t;
unsafe extern "C" fn rtnl_rtrealm_initialize() {
  if !rtnl_rtrealm_tab.is_null() {
    return;
  }
  rtnl_rtrealm_tab =
    xzalloc(::std::mem::size_of::<rtnl_tab_t>() as libc::c_ulong) as *mut rtnl_tab_t;
  (*rtnl_rtrealm_tab).tab[0] = b"unknown\x00" as *const u8 as *const libc::c_char;
  rtnl_tab_initialize(
    b"realms\x00" as *const u8 as *const libc::c_char,
    (*rtnl_rtrealm_tab).tab.as_mut_ptr(),
  );
}
#[no_mangle]
pub unsafe extern "C" fn rtnl_rtrealm_a2n(
  mut id: *mut u32,
  mut arg: *mut libc::c_char,
) -> libc::c_int {
  rtnl_rtrealm_initialize();
  return rtnl_a2n(rtnl_rtrealm_tab, id, arg, 0i32);
}
#[no_mangle]
pub unsafe extern "C" fn rtnl_rtrealm_n2a(mut id: libc::c_int) -> *const libc::c_char {
  if id < 0i32 || id > 1023i32 {
    return itoa(id);
  }
  rtnl_rtrealm_initialize();
  if !(*rtnl_rtrealm_tab).tab[id as usize].is_null() {
    return (*rtnl_rtrealm_tab).tab[id as usize];
  }
  return itoa(id);
}
static mut rtnl_rtdsfield_tab: *mut rtnl_tab_t = 0 as *const rtnl_tab_t as *mut rtnl_tab_t;
unsafe extern "C" fn rtnl_rtdsfield_initialize() {
  if !rtnl_rtdsfield_tab.is_null() {
    return;
  }
  rtnl_rtdsfield_tab =
    xzalloc(::std::mem::size_of::<rtnl_tab_t>() as libc::c_ulong) as *mut rtnl_tab_t;
  (*rtnl_rtdsfield_tab).tab[0] = b"0\x00" as *const u8 as *const libc::c_char;
  rtnl_tab_initialize(
    b"dsfield\x00" as *const u8 as *const libc::c_char,
    (*rtnl_rtdsfield_tab).tab.as_mut_ptr(),
  );
}
#[no_mangle]
pub unsafe extern "C" fn rtnl_dsfield_n2a(mut id: libc::c_int) -> *const libc::c_char {
  if id < 0i32 || id > 1023i32 {
    return itoa(id);
  }
  rtnl_rtdsfield_initialize();
  if !(*rtnl_rtdsfield_tab).tab[id as usize].is_null() {
    return (*rtnl_rtdsfield_tab).tab[id as usize];
  }
  return itoa(id);
}
#[no_mangle]
pub unsafe extern "C" fn rtnl_dsfield_a2n(
  mut id: *mut u32,
  mut arg: *mut libc::c_char,
) -> libc::c_int {
  rtnl_rtdsfield_initialize();
  return rtnl_a2n(rtnl_rtdsfield_tab, id, arg, 16i32);
}
static mut rtnl_rttable_tab: *mut rtnl_tab_t = 0 as *const rtnl_tab_t as *mut rtnl_tab_t;
unsafe extern "C" fn rtnl_rttable_initialize() {
  if !rtnl_rttable_tab.is_null() {
    return;
  }
  rtnl_rttable_tab =
    xzalloc(::std::mem::size_of::<rtnl_tab_t>() as libc::c_ulong) as *mut rtnl_tab_t;
  (*rtnl_rttable_tab).tab[0] = b"unspec\x00" as *const u8 as *const libc::c_char;
  (*rtnl_rttable_tab).tab[255] = b"local\x00" as *const u8 as *const libc::c_char;
  (*rtnl_rttable_tab).tab[254] = b"main\x00" as *const u8 as *const libc::c_char;
  (*rtnl_rttable_tab).tab[253] = b"default\x00" as *const u8 as *const libc::c_char;
  rtnl_tab_initialize(
    b"tables\x00" as *const u8 as *const libc::c_char,
    (*rtnl_rttable_tab).tab.as_mut_ptr(),
  );
}
#[no_mangle]
pub unsafe extern "C" fn rtnl_rttable_n2a(mut id: libc::c_int) -> *const libc::c_char {
  if id < 0i32 || id > 1023i32 {
    return itoa(id);
  }
  rtnl_rttable_initialize();
  if !(*rtnl_rttable_tab).tab[id as usize].is_null() {
    return (*rtnl_rttable_tab).tab[id as usize];
  }
  return itoa(id);
}
#[no_mangle]
pub unsafe extern "C" fn rtnl_rttable_a2n(
  mut id: *mut u32,
  mut arg: *mut libc::c_char,
) -> libc::c_int {
  rtnl_rttable_initialize();
  return rtnl_a2n(rtnl_rttable_tab, id, arg, 0i32);
}
