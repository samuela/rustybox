use crate::archival::libarchive::bb_archive::archive_handle_t;
use crate::libbb::llist::llist_t;
use crate::libbb::ptr_to_globals::bb_errno;
use crate::libbb::xfuncs_printf::xmalloc;
use crate::librb::md5_ctx_t;
use crate::librb::size_t;
use c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;
use libc;
use libc::access;
use libc::atoi;
use libc::close;
use libc::fclose;
use libc::fprintf;
use libc::free;
use libc::lstat;
use libc::open;
use libc::printf;
use libc::puts;
use libc::rename;
use libc::rmdir;
use libc::sprintf;
use libc::stat;
use libc::strchr;
use libc::strcmp;
use libc::strrchr;
use libc::strstr;
use libc::system;
use libc::unlink;
use libc::FILE;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;

  #[no_mangle]
  fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;

  #[no_mangle]
  fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;

  #[no_mangle]
  fn strtok_r(
    __s: *mut libc::c_char,
    __delim: *const libc::c_char,
    __save_ptr: *mut *mut libc::c_char,
  ) -> *mut libc::c_char;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  static mut option_mask32: u32;

  #[no_mangle]
  static ptr_to_globals: *mut globals;

  #[no_mangle]
  fn fnmatch(
    __pattern: *const libc::c_char,
    __name: *const libc::c_char,
    __flags: libc::c_int,
  ) -> libc::c_int;

}

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

// Defined in archival/dpkg.c

#[repr(C)]
#[derive(Copy, Clone)]
pub struct globals {
  pub name_hashtable: [*mut libc::c_char; 16382],
  pub package_hashtable: [*mut common_node_t; 10008],
  pub status_hashtable: [*mut status_node_t; 8192],
}
pub type status_node_t = status_node_s;

#[repr(C)]
#[derive(Copy, Clone, BitfieldStruct)]
pub struct status_node_s {
  #[bitfield(name = "package", ty = "libc::c_uint", bits = "0..=15")]
  #[bitfield(name = "status", ty = "libc::c_uint", bits = "16..=31")]
  pub package_status: [u8; 4],
}
pub type common_node_t = common_node_s;

#[repr(C)]
#[derive(Copy, Clone, BitfieldStruct)]
pub struct common_node_s {
  #[bitfield(name = "name", ty = "libc::c_uint", bits = "0..=15")]
  #[bitfield(name = "version", ty = "libc::c_uint", bits = "16..=31")]
  #[bitfield(name = "num_of_edges", ty = "libc::c_uint", bits = "32..=47")]
  pub name_version_num_of_edges: [u8; 6],
  #[bitfield(padding)]
  pub c2rust_padding: [u8; 2],
  pub edge: *mut *mut edge_t,
}
pub type edge_t = edge_s;

#[repr(C)]
#[derive(Copy, Clone, BitfieldStruct)]
pub struct edge_s {
  #[bitfield(padding)]
  pub c2rust_padding: [u8; 1],
  #[bitfield(name = "operator", ty = "libc::c_uint", bits = "0..=3")]
  #[bitfield(name = "type_0", ty = "libc::c_uint", bits = "4..=7")]
  #[bitfield(name = "name", ty = "libc::c_uint", bits = "8..=23")]
  #[bitfield(name = "version", ty = "libc::c_uint", bits = "32..=47")]
  pub operator_type_0_name_version: [u8; 5],
  #[bitfield(padding)]
  pub c2rust_padding_0: [u8; 2],
}

/* Even numbers are for 'extras', like ored dependencies or null */
pub type edge_type_e = libc::c_uint;
pub const EDGE_ENHANCES: edge_type_e = 15;
pub const EDGE_RECOMMENDS: edge_type_e = 13;
pub const EDGE_SUGGESTS: edge_type_e = 11;
pub const EDGE_CONFLICTS: edge_type_e = 9;
pub const EDGE_PROVIDES: edge_type_e = 7;
pub const EDGE_REPLACES: edge_type_e = 5;
pub const EDGE_OR_DEPENDS: edge_type_e = 4;
pub const EDGE_DEPENDS: edge_type_e = 3;
pub const EDGE_OR_PRE_DEPENDS: edge_type_e = 2;
pub const EDGE_PRE_DEPENDS: edge_type_e = 1;
pub const EDGE_NULL: edge_type_e = 0;
pub type operator_e = libc::c_uint;
pub const VER_ANY: operator_e = 6;
pub const VER_MORE_EQUAL: operator_e = 5;
pub const VER_MORE: operator_e = 4;
pub const VER_LESS_EQUAL: operator_e = 3;
pub const VER_LESS: operator_e = 2;
pub const VER_EQUAL: operator_e = 1;
pub const VER_NULL: operator_e = 0;

#[repr(C)]
#[derive(Copy, Clone, BitfieldStruct)]
pub struct deb_file_s {
  pub control_file: *mut libc::c_char,
  pub filename: *mut libc::c_char,
  #[bitfield(name = "package", ty = "libc::c_uint", bits = "0..=15")]
  pub package: [u8; 2],
  #[bitfield(padding)]
  pub c2rust_padding: [u8; 6],
}
pub type deb_file_t = deb_file_s;
pub type C2RustUnnamed = libc::c_uint;
pub const OPT_force_confold: C2RustUnnamed = 512;
pub const OPT_force_confnew: C2RustUnnamed = 256;
pub const OPT_force_ignore_depends: C2RustUnnamed = 128;
/* was:14 */
/* Options */
pub const OPT_force: C2RustUnnamed = 64;
pub const OPTMASK_cmd: C2RustUnnamed = 63;
pub const OPT_unpack: C2RustUnnamed = 32;
pub const OPT_remove: C2RustUnnamed = 16;
pub const OPT_purge: C2RustUnnamed = 8;
pub const OPT_list_installed: C2RustUnnamed = 4;
pub const OPT_install: C2RustUnnamed = 2;
/* Commands */
pub const OPT_configure: C2RustUnnamed = 1;
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
unsafe extern "C" fn make_hash(
  mut key: *const libc::c_char,
  mut start: *mut libc::c_uint,
  mut decrement: *mut libc::c_uint,
  hash_prime: libc::c_int,
) {
  let mut hash_num: libc::c_ulong = *key.offset(0) as libc::c_ulong;
  let mut len: libc::c_int = strlen(key) as libc::c_int;
  let mut i: libc::c_int = 0;
  /* Maybe i should have uses a "proper" hashing algorithm here instead
   * of making one up myself, seems to be working ok though. */
  i = 1i32;
  while i < len {
    /* shifts the ascii based value and adds it to previous value
     * shift amount is mod 24 because long int is 32 bit and data
     * to be shifted is 8, don't want to shift data to where it has
     * no effect */
    hash_num = hash_num.wrapping_add(
      ((*key.offset(i as isize) as libc::c_int + *key.offset((i - 1i32) as isize) as libc::c_int)
        << *key.offset(i as isize) as libc::c_int * i % 24i32) as libc::c_ulong,
    );
    i += 1
  }
  *start = (hash_num as libc::c_uint).wrapping_rem(hash_prime as libc::c_uint);
  *decrement = (1i32 as libc::c_uint as libc::c_ulong)
    .wrapping_add(hash_num.wrapping_rem((hash_prime - 1i32) as libc::c_ulong))
    as libc::c_uint;
}
/* this adds the key to the hash table */
unsafe extern "C" fn search_name_hashtable(mut key: *const libc::c_char) -> libc::c_int {
  let mut probe_address: libc::c_uint = 0;
  let mut probe_decrement: libc::c_uint = 0;
  make_hash(key, &mut probe_address, &mut probe_decrement, 16381i32);
  while !(*ptr_to_globals).name_hashtable[probe_address as usize].is_null() {
    if strcmp(
      (*ptr_to_globals).name_hashtable[probe_address as usize],
      key,
    ) == 0
    {
      return probe_address as libc::c_int;
    }
    probe_address = probe_address.wrapping_sub(probe_decrement);
    if (probe_address as libc::c_int) < 0 {
      probe_address = probe_address.wrapping_add(16381i32 as libc::c_uint)
    }
  }
  (*ptr_to_globals).name_hashtable[probe_address as usize] =
    crate::libbb::xfuncs_printf::xstrdup(key);
  return probe_address as libc::c_int;
}
/* this DOESN'T add the key to the hashtable
 * TODO make it consistent with search_name_hashtable
 */
unsafe extern "C" fn search_status_hashtable(mut key: *const libc::c_char) -> libc::c_uint {
  let mut probe_address: libc::c_uint = 0;
  let mut probe_decrement: libc::c_uint = 0;
  make_hash(key, &mut probe_address, &mut probe_decrement, 8191i32);
  while !(*ptr_to_globals).status_hashtable[probe_address as usize].is_null() {
    if strcmp(
      key,
      (*ptr_to_globals).name_hashtable[(*(*ptr_to_globals).package_hashtable
        [(*(*ptr_to_globals).status_hashtable[probe_address as usize]).package() as usize])
        .name() as usize],
    ) == 0
    {
      break;
    }
    probe_address = probe_address.wrapping_sub(probe_decrement);
    if (probe_address as libc::c_int) < 0 {
      probe_address = probe_address.wrapping_add(8191i32 as libc::c_uint)
    }
  }
  return probe_address;
}
unsafe extern "C" fn order(mut x: libc::c_char) -> libc::c_int {
  return if x as libc::c_int == '~' as i32 {
    -1i32
  } else if x as libc::c_int == '\u{0}' as i32 {
    0
  } else if (x as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
    0
  } else if ((x as libc::c_int | 0x20i32) - 'a' as i32) as libc::c_uchar as libc::c_int
    <= 'z' as i32 - 'a' as i32
  {
    x as libc::c_int
  } else {
    (x as libc::c_uchar as libc::c_int) + 256i32
  };
}
/* This code is taken from dpkg and modified slightly to work with busybox */
unsafe extern "C" fn version_compare_part(
  mut val: *const libc::c_char,
  mut ref_0: *const libc::c_char,
) -> libc::c_int {
  if val.is_null() {
    val = b"\x00" as *const u8 as *const libc::c_char
  }
  if ref_0.is_null() {
    ref_0 = b"\x00" as *const u8 as *const libc::c_char
  }
  while *val as libc::c_int != 0 || *ref_0 as libc::c_int != 0 {
    let mut first_diff: libc::c_int = 0;
    while *val as libc::c_int != 0
      && !((*val as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32)
      || *ref_0 as libc::c_int != 0
        && !((*ref_0 as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32)
    {
      let mut vc: libc::c_int = order(*val);
      let mut rc: libc::c_int = order(*ref_0);
      if vc != rc {
        return vc - rc;
      }
      val = val.offset(1);
      ref_0 = ref_0.offset(1)
    }
    while *val as libc::c_int == '0' as i32 {
      val = val.offset(1)
    }
    while *ref_0 as libc::c_int == '0' as i32 {
      ref_0 = ref_0.offset(1)
    }
    first_diff = 0;
    while (*val as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32
      && (*ref_0 as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32
    {
      if first_diff == 0 {
        first_diff = *val as libc::c_int - *ref_0 as libc::c_int
      }
      val = val.offset(1);
      ref_0 = ref_0.offset(1)
    }
    if (*val as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
      return 1i32;
    }
    if (*ref_0 as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int <= 9i32 {
      return -1i32;
    }
    if first_diff != 0 {
      return first_diff;
    }
  }
  return 0;
}
/* if ver1 < ver2 return -1,
 * if ver1 = ver2 return 0,
 * if ver1 > ver2 return 1,
 */
unsafe extern "C" fn version_compare(ver1: libc::c_uint, ver2: libc::c_uint) -> libc::c_int {
  let mut ch_ver1: *mut libc::c_char = (*ptr_to_globals).name_hashtable[ver1 as usize];
  let mut ch_ver2: *mut libc::c_char = (*ptr_to_globals).name_hashtable[ver2 as usize];
  let mut epoch1: libc::c_uint = 0 as libc::c_uint;
  let mut epoch2: libc::c_uint = 0 as libc::c_uint;
  let mut colon: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut deb_ver1: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut deb_ver2: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut upstream_ver1: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut upstream_ver2: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut result: libc::c_int = 0;
  /* Compare epoch */
  colon = strchr(ch_ver1, ':' as i32);
  if !colon.is_null() {
    epoch1 = atoi(ch_ver1) as libc::c_uint;
    ch_ver1 = colon.offset(1)
  }
  colon = strchr(ch_ver2, ':' as i32);
  if !colon.is_null() {
    epoch2 = atoi(ch_ver2) as libc::c_uint;
    ch_ver2 = colon.offset(1)
  }
  if epoch1 < epoch2 {
    return -1i32;
  }
  if epoch1 > epoch2 {
    return 1i32;
  }
  /* Compare upstream version */
  upstream_ver1 = crate::libbb::xfuncs_printf::xstrdup(ch_ver1);
  upstream_ver2 = crate::libbb::xfuncs_printf::xstrdup(ch_ver2);
  /* Chop off debian version, and store for later use */
  deb_ver1 = strrchr(upstream_ver1, '-' as i32);
  deb_ver2 = strrchr(upstream_ver2, '-' as i32);
  if !deb_ver1.is_null() {
    let fresh0 = deb_ver1;
    deb_ver1 = deb_ver1.offset(1);
    *fresh0 = '\u{0}' as i32 as libc::c_char
  }
  if !deb_ver2.is_null() {
    let fresh1 = deb_ver2;
    deb_ver2 = deb_ver2.offset(1);
    *fresh1 = '\u{0}' as i32 as libc::c_char
  }
  result = version_compare_part(upstream_ver1, upstream_ver2);
  if result == 0 {
    /* Compare debian versions */
    result = version_compare_part(deb_ver1, deb_ver2)
  }
  free(upstream_ver1 as *mut libc::c_void);
  free(upstream_ver2 as *mut libc::c_void);
  return result;
}
unsafe extern "C" fn test_version(
  version1: libc::c_uint,
  version2: libc::c_uint,
  operator: libc::c_uint,
) -> libc::c_int {
  let version_result: libc::c_int = version_compare(version1, version2);
  match operator {
    6 => return 1i32,
    1 => return (version_result == 0) as libc::c_int,
    2 => return (version_result < 0) as libc::c_int,
    3 => return (version_result <= 0) as libc::c_int,
    4 => return (version_result > 0) as libc::c_int,
    5 => return (version_result >= 0) as libc::c_int,
    _ => {}
  }
  return 0;
}
unsafe extern "C" fn search_package_hashtable(
  name: libc::c_uint,
  version: libc::c_uint,
  operator: libc::c_uint,
) -> libc::c_int {
  let mut probe_address: libc::c_uint = 0;
  let mut probe_decrement: libc::c_uint = 0;
  make_hash(
    (*ptr_to_globals).name_hashtable[name as usize],
    &mut probe_address,
    &mut probe_decrement,
    10007i32,
  );
  while !(*ptr_to_globals).package_hashtable[probe_address as usize].is_null() {
    if (*(*ptr_to_globals).package_hashtable[probe_address as usize]).name() == name {
      if operator == VER_ANY as libc::c_int as libc::c_uint {
        return probe_address as libc::c_int;
      }
      if test_version(
        (*(*ptr_to_globals).package_hashtable[probe_address as usize]).version(),
        version,
        operator,
      ) != 0
      {
        return probe_address as libc::c_int;
      }
    }
    probe_address = probe_address.wrapping_sub(probe_decrement);
    if (probe_address as libc::c_int) < 0 {
      probe_address = probe_address.wrapping_add(10007i32 as libc::c_uint)
    }
  }
  return probe_address as libc::c_int;
}
/*
 * This function searches through the entire package_hashtable looking
 * for a package which provides "needle". It returns the index into
 * the package_hashtable for the providing package.
 *
 * needle is the index into name_hashtable of the package we are
 * looking for.
 *
 * start_at is the index in the package_hashtable to start looking
 * at. If start_at is -1 then start at the beginning. This is to allow
 * for repeated searches since more than one package might provide
 * needle.
 *
 * FIXME: I don't think this is very efficient, but I thought I'd keep
 * it simple for now until it proves to be a problem.
 */
unsafe extern "C" fn search_for_provides(
  mut needle: libc::c_int,
  mut start_at: libc::c_int,
) -> libc::c_int {
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  let mut p: *mut common_node_t = std::ptr::null_mut();
  i = start_at + 1i32;
  while i < 10007i32 {
    p = (*ptr_to_globals).package_hashtable[i as usize];
    if !p.is_null() {
      j = 0;
      while j < (*p).num_of_edges() as libc::c_int {
        if (**(*p).edge.offset(j as isize)).type_0() as libc::c_int == EDGE_PROVIDES as libc::c_int
          && (**(*p).edge.offset(j as isize)).name() as libc::c_int == needle
        {
          return i;
        }
        j += 1
      }
    }
    i += 1
  }
  return -1i32;
}
/*
 * Add an edge to a node
 */
unsafe extern "C" fn add_edge_to_node(mut node: *mut common_node_t, mut edge: *mut edge_t) {
  (*node).edge = crate::libbb::xrealloc_vector::xrealloc_vector_helper(
    (*node).edge as *mut libc::c_void,
    ((::std::mem::size_of::<*mut edge_t>() as libc::c_ulong) << 8i32)
      .wrapping_add(2i32 as libc::c_ulong) as libc::c_uint,
    (*node).num_of_edges() as libc::c_int,
  ) as *mut *mut edge_t;
  let ref mut fresh2 = (*node).num_of_edges();
  let fresh3 = *fresh2;
  *fresh2 = (*fresh2).wrapping_add(1);
  let ref mut fresh4 = *(*node).edge.offset(fresh3 as isize);
  *fresh4 = edge;
}
/*
 * Create one new node and one new edge for every dependency.
 *
 * Dependencies which contain multiple alternatives are represented as
 * an EDGE_OR_PRE_DEPENDS or EDGE_OR_DEPENDS node, followed by a
 * number of EDGE_PRE_DEPENDS or EDGE_DEPENDS nodes. The name field of
 * the OR edge contains the full dependency string while the version
 * field contains the number of EDGE nodes which follow as part of
 * this alternative.
 */
unsafe extern "C" fn add_split_dependencies(
  mut parent_node: *mut common_node_t,
  mut whole_line: *const libc::c_char,
  mut edge_type: libc::c_uint,
) {
  let mut line: *mut libc::c_char = crate::libbb::xfuncs_printf::xstrdup(whole_line);
  let mut line2: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut line_ptr1: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut line_ptr2: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut field: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut field2: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut version: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut edge: *mut edge_t = std::ptr::null_mut();
  let mut or_edge: *mut edge_t = std::ptr::null_mut();
  let mut offset_ch: libc::c_int = 0;
  field = strtok_r(
    line,
    b",\x00" as *const u8 as *const libc::c_char,
    &mut line_ptr1,
  );
  loop {
    /* skip leading spaces */
    field = field.offset(strspn(field, b" \x00" as *const u8 as *const libc::c_char) as isize);
    line2 = crate::libbb::xfuncs_printf::xstrdup(field);
    field2 = strtok_r(
      line2,
      b"|\x00" as *const u8 as *const libc::c_char,
      &mut line_ptr2,
    );
    or_edge = std::ptr::null_mut();
    if (edge_type == EDGE_DEPENDS as libc::c_int as libc::c_uint
      || edge_type == EDGE_PRE_DEPENDS as libc::c_int as libc::c_uint)
      && strcmp(field, field2) != 0
    {
      or_edge =
        crate::libbb::xfuncs_printf::xzalloc(::std::mem::size_of::<edge_t>() as libc::c_ulong)
          as *mut edge_t;
      (*or_edge).set_type_0(edge_type.wrapping_add(1i32 as libc::c_uint));
      (*or_edge).set_name(search_name_hashtable(field) as libc::c_uint);
      //or_edge->version = 0; // tracks the number of alternatives
      add_edge_to_node(parent_node, or_edge);
    }
    loop {
      edge = xmalloc(::std::mem::size_of::<edge_t>() as libc::c_ulong) as *mut edge_t;
      (*edge).set_type_0(edge_type);
      /* Skip any extra leading spaces */
      field2 = field2.offset(strspn(field2, b" \x00" as *const u8 as *const libc::c_char) as isize);
      /* Get dependency version info */
      version = strchr(field2, '(' as i32);
      if version.is_null() {
        (*edge).set_operator(VER_ANY as libc::c_int as libc::c_uint);
        /* Get the versions hash number, adding it if the number isn't already in there */
        (*edge).set_version(
          search_name_hashtable(b"ANY\x00" as *const u8 as *const libc::c_char) as libc::c_uint,
        )
      } else {
        /* Skip leading ' ' or '(' */
        version =
          version.offset(strspn(version, b" (\x00" as *const u8 as *const libc::c_char) as isize);
        /* Calculate length of any operator characters */
        offset_ch = strspn(version, b"<=>\x00" as *const u8 as *const libc::c_char) as libc::c_int;
        /* Determine operator */
        if offset_ch > 0 {
          if strncmp(
            version,
            b"=\x00" as *const u8 as *const libc::c_char,
            offset_ch as libc::c_ulong,
          ) == 0
          {
            (*edge).set_operator(VER_EQUAL as libc::c_int as libc::c_uint)
          } else if strncmp(
            version,
            b"<<\x00" as *const u8 as *const libc::c_char,
            offset_ch as libc::c_ulong,
          ) == 0
          {
            (*edge).set_operator(VER_LESS as libc::c_int as libc::c_uint)
          } else if strncmp(
            version,
            b"<=\x00" as *const u8 as *const libc::c_char,
            offset_ch as libc::c_ulong,
          ) == 0
          {
            (*edge).set_operator(VER_LESS_EQUAL as libc::c_int as libc::c_uint)
          } else if strncmp(
            version,
            b">>\x00" as *const u8 as *const libc::c_char,
            offset_ch as libc::c_ulong,
          ) == 0
          {
            (*edge).set_operator(VER_MORE as libc::c_int as libc::c_uint)
          } else if strncmp(
            version,
            b">=\x00" as *const u8 as *const libc::c_char,
            offset_ch as libc::c_ulong,
          ) == 0
          {
            (*edge).set_operator(VER_MORE_EQUAL as libc::c_int as libc::c_uint)
          } else {
            crate::libbb::verror_msg::bb_simple_error_msg_and_die(
              b"illegal operator\x00" as *const u8 as *const libc::c_char,
            );
          }
        }
        /* skip to start of version numbers */
        version = version.offset(offset_ch as isize);
        version =
          version.offset(strspn(version, b" \x00" as *const u8 as *const libc::c_char) as isize);
        /* Truncate version at trailing ' ' or ')' */
        *version.offset(strcspn(version, b" )\x00" as *const u8 as *const libc::c_char) as isize) =
          '\u{0}' as i32 as libc::c_char;
        /* Get the versions hash number, adding it if the number isn't already in there */
        (*edge).set_version(search_name_hashtable(version) as libc::c_uint)
      }
      /* Get the dependency name */
      *field2.offset(strcspn(field2, b" (\x00" as *const u8 as *const libc::c_char) as isize) =
        '\u{0}' as i32 as libc::c_char;
      (*edge).set_name(search_name_hashtable(field2) as libc::c_uint);
      if !or_edge.is_null() {
        (*or_edge).set_version((*or_edge).version() + 1)
      }
      add_edge_to_node(parent_node, edge);
      field2 = strtok_r(
        std::ptr::null_mut::<libc::c_char>(),
        b"|\x00" as *const u8 as *const libc::c_char,
        &mut line_ptr2,
      );
      if field2.is_null() {
        break;
      }
    }
    free(line2 as *mut libc::c_void);
    field = strtok_r(
      std::ptr::null_mut::<libc::c_char>(),
      b",\x00" as *const u8 as *const libc::c_char,
      &mut line_ptr1,
    );
    if field.is_null() {
      break;
    }
  }
  free(line as *mut libc::c_void);
}
unsafe extern "C" fn free_package(mut node: *mut common_node_t) {
  let mut i: libc::c_uint = 0;
  if !node.is_null() {
    i = 0 as libc::c_uint;
    while i < (*node).num_of_edges() {
      free(*(*node).edge.offset(i as isize) as *mut libc::c_void);
      i = i.wrapping_add(1)
    }
    free((*node).edge as *mut libc::c_void);
    free(node as *mut libc::c_void);
  };
}
/*
 * Gets the next package field from package_buffer, separated into the field name
 * and field value, it returns the int offset to the first character of the next field
 */
unsafe extern "C" fn read_package_field(
  mut package_buffer: *const libc::c_char,
  mut field_name: *mut *mut libc::c_char,
  mut field_value: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut offset_name_start: libc::c_int = 0;
  let mut offset_name_end: libc::c_int = 0;
  let mut offset_value_start: libc::c_int = 0;
  let mut offset_value_end: libc::c_int = 0;
  let mut offset: libc::c_int = 0;
  let mut next_offset: libc::c_int = 0;
  let mut name_length: libc::c_int = 0;
  let mut value_length: libc::c_int = 0;
  let mut exit_flag: libc::c_int = 0;
  if package_buffer.is_null() {
    *field_name = std::ptr::null_mut::<libc::c_char>();
    *field_value = std::ptr::null_mut::<libc::c_char>();
    return -1i32;
  }
  loop {
    next_offset = offset + 1i32;
    let mut current_block_17: u64;
    match *package_buffer.offset(offset as isize) as libc::c_int {
      0 => {
        exit_flag = 1i32;
        current_block_17 = 15345278821338558188;
      }
      58 => {
        if offset_name_end == 0 {
          offset_name_end = offset;
          offset_value_start = next_offset
        }
        current_block_17 = 15345278821338558188;
      }
      10 => {
        /* TODO: The char next_offset may be out of bounds */
        if *package_buffer.offset(next_offset as isize) as libc::c_int != ' ' as i32 {
          exit_flag = 1i32;
          current_block_17 = 15345278821338558188;
        } else {
          current_block_17 = 11692205478773272193;
        }
      }
      9 | 32 => {
        current_block_17 = 11692205478773272193;
      }
      _ => {
        current_block_17 = 15345278821338558188;
      }
    }
    match current_block_17 {
      11692205478773272193 => {
        /* increment the value start point if its a just filler */
        if offset_name_start == offset {
          offset_name_start += 1
        }
        if offset_value_start == offset {
          offset_value_start += 1
        }
      }
      _ => {}
    }
    if exit_flag != 0 {
      /* Check that the names are valid */
      offset_value_end = offset;
      name_length = offset_name_end - offset_name_start;
      value_length = offset_value_end - offset_value_start;
      if name_length == 0 {
        break;
      }
      if name_length > 0 && value_length > 0 {
        break;
      }
      /* If not valid, start fresh with next field */
      exit_flag = 0;
      offset_name_start = offset + 1i32;
      offset_name_end = 0;
      offset_value_start = offset + 1i32;
      offset_value_end = offset + 1i32;
      offset += 1
    }
    offset += 1
  }
  *field_name = std::ptr::null_mut::<libc::c_char>();
  if name_length != 0 {
    *field_name = crate::libbb::xfuncs_printf::xstrndup(
      &*package_buffer.offset(offset_name_start as isize),
      name_length,
    )
  }
  *field_value = std::ptr::null_mut::<libc::c_char>();
  if value_length > 0 {
    *field_value = crate::libbb::xfuncs_printf::xstrndup(
      &*package_buffer.offset(offset_value_start as isize),
      value_length,
    )
  }
  return next_offset;
}
unsafe extern "C" fn fill_package_struct(mut control_buffer: *mut libc::c_char) -> libc::c_uint {
  static mut field_names: [libc::c_char; 94] = [
    80, 97, 99, 107, 97, 103, 101, 0, 86, 101, 114, 115, 105, 111, 110, 0, 80, 114, 101, 45, 68,
    101, 112, 101, 110, 100, 115, 0, 68, 101, 112, 101, 110, 100, 115, 0, 82, 101, 112, 108, 97,
    99, 101, 115, 0, 80, 114, 111, 118, 105, 100, 101, 115, 0, 67, 111, 110, 102, 108, 105, 99,
    116, 115, 0, 83, 117, 103, 103, 101, 115, 116, 115, 0, 82, 101, 99, 111, 109, 109, 101, 110,
    100, 115, 0, 69, 110, 104, 97, 110, 99, 101, 115, 0, 0,
  ];
  let mut new_node: *mut common_node_t =
    crate::libbb::xfuncs_printf::xzalloc(::std::mem::size_of::<common_node_t>() as libc::c_ulong)
      as *mut common_node_t;
  let mut field_name: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut field_value: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut field_start: libc::c_int = 0;
  let mut num: libc::c_int = -1i32;
  let mut buffer_length: libc::c_int = strlen(control_buffer) as libc::c_int;
  (*new_node).set_version(
    search_name_hashtable(b"unknown\x00" as *const u8 as *const libc::c_char) as libc::c_uint,
  );
  while field_start < buffer_length {
    let mut field_num: libc::c_uint = 0;
    field_start += read_package_field(
      &mut *control_buffer.offset(field_start as isize),
      &mut field_name,
      &mut field_value,
    );
    if !field_name.is_null() {
      field_num =
        crate::libbb::compare_string_array::index_in_strings(field_names.as_ptr(), field_name)
          as libc::c_uint;
      match field_num {
        0 => {
          /* Package */
          (*new_node).set_name(search_name_hashtable(field_value) as libc::c_uint)
        }
        1 => {
          /* Version */
          (*new_node).set_version(search_name_hashtable(field_value) as libc::c_uint)
        }
        2 => {
          /* Pre-Depends */
          add_split_dependencies(
            new_node,
            field_value,
            EDGE_PRE_DEPENDS as libc::c_int as libc::c_uint,
          );
        }
        3 => {
          /* Depends */
          add_split_dependencies(
            new_node,
            field_value,
            EDGE_DEPENDS as libc::c_int as libc::c_uint,
          );
        }
        4 => {
          /* Replaces */
          add_split_dependencies(
            new_node,
            field_value,
            EDGE_REPLACES as libc::c_int as libc::c_uint,
          );
        }
        5 => {
          /* Provides */
          add_split_dependencies(
            new_node,
            field_value,
            EDGE_PROVIDES as libc::c_int as libc::c_uint,
          );
        }
        6 => {
          /* Conflicts */
          add_split_dependencies(
            new_node,
            field_value,
            EDGE_CONFLICTS as libc::c_int as libc::c_uint,
          );
        }
        7 => {
          /* Suggests */
          add_split_dependencies(
            new_node,
            field_value,
            EDGE_SUGGESTS as libc::c_int as libc::c_uint,
          );
        }
        8 => {
          /* Recommends */
          add_split_dependencies(
            new_node,
            field_value,
            EDGE_RECOMMENDS as libc::c_int as libc::c_uint,
          );
        }
        9 => {
          /* Enhances */
          add_split_dependencies(
            new_node,
            field_value,
            EDGE_ENHANCES as libc::c_int as libc::c_uint,
          );
        }
        _ => {}
      }
    }
    free(field_name as *mut libc::c_void);
    free(field_value as *mut libc::c_void);
  }
  if (*new_node).version() as libc::c_int
    == search_name_hashtable(b"unknown\x00" as *const u8 as *const libc::c_char)
  {
    free_package(new_node);
    return -1i32 as libc::c_uint;
  }
  num = search_package_hashtable(
    (*new_node).name(),
    (*new_node).version(),
    VER_EQUAL as libc::c_int as libc::c_uint,
  );
  free_package((*ptr_to_globals).package_hashtable[num as usize]);
  (*ptr_to_globals).package_hashtable[num as usize] = new_node;
  return num as libc::c_uint;
}
/* if num = 1, it returns the want status, 2 returns flag, 3 returns status */
unsafe extern "C" fn get_status(status_node: libc::c_uint, num: libc::c_int) -> libc::c_uint {
  let mut status_string: *mut libc::c_char = (*ptr_to_globals).name_hashtable
    [(*(*ptr_to_globals).status_hashtable[status_node as usize]).status() as usize];
  let mut state_sub_string: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut state_sub_num: libc::c_uint = 0;
  let mut len: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  /* set tmp_string to point to the start of the word number */
  i = 1i32;
  while i < num {
    /* skip past a word */
    status_string = status_string
      .offset(strcspn(status_string, b" \x00" as *const u8 as *const libc::c_char) as isize);
    /* skip past the separating spaces */
    status_string = status_string
      .offset(strspn(status_string, b" \x00" as *const u8 as *const libc::c_char) as isize);
    i += 1
  }
  len = strcspn(
    status_string,
    b" \n\x00" as *const u8 as *const libc::c_char,
  ) as libc::c_int;
  state_sub_string = crate::libbb::xfuncs_printf::xstrndup(status_string, len);
  state_sub_num = search_name_hashtable(state_sub_string) as libc::c_uint;
  free(state_sub_string as *mut libc::c_void);
  return state_sub_num;
}
unsafe extern "C" fn set_status(
  status_node_num: libc::c_uint,
  mut new_value: *const libc::c_char,
  position: libc::c_int,
) {
  let new_value_num: libc::c_uint = search_name_hashtable(new_value) as libc::c_uint;
  let mut want: libc::c_uint = get_status(status_node_num, 1i32);
  let mut flag: libc::c_uint = get_status(status_node_num, 2i32);
  let mut status: libc::c_uint = get_status(status_node_num, 3i32);
  let mut new_status: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  match position {
    1 => want = new_value_num,
    2 => flag = new_value_num,
    3 => status = new_value_num,
    _ => {
      crate::libbb::verror_msg::bb_simple_error_msg_and_die(
        b"DEBUG ONLY: this shouldnt happen\x00" as *const u8 as *const libc::c_char,
      );
    }
  }
  new_status = crate::libbb::xfuncs_printf::xasprintf(
    b"%s %s %s\x00" as *const u8 as *const libc::c_char,
    (*ptr_to_globals).name_hashtable[want as usize],
    (*ptr_to_globals).name_hashtable[flag as usize],
    (*ptr_to_globals).name_hashtable[status as usize],
  );
  (*(*ptr_to_globals).status_hashtable[status_node_num as usize])
    .set_status(search_name_hashtable(new_status) as libc::c_uint);
  free(new_status as *mut libc::c_void);
}
unsafe extern "C" fn describe_status(mut status_num: libc::c_int) -> *const libc::c_char {
  let mut status_want: libc::c_int = 0;
  let mut status_state: libc::c_int = 0;
  if (*ptr_to_globals).status_hashtable[status_num as usize].is_null()
    || (*(*ptr_to_globals).status_hashtable[status_num as usize]).status() as libc::c_int == 0
  {
    return b"is not installed or flagged to be installed\x00" as *const u8 as *const libc::c_char;
  }
  status_want = get_status(status_num as libc::c_uint, 1i32) as libc::c_int;
  status_state = get_status(status_num as libc::c_uint, 3i32) as libc::c_int;
  if status_state == search_name_hashtable(b"installed\x00" as *const u8 as *const libc::c_char) {
    if status_want == search_name_hashtable(b"install\x00" as *const u8 as *const libc::c_char) {
      return b"is installed\x00" as *const u8 as *const libc::c_char;
    }
    if status_want == search_name_hashtable(b"deinstall\x00" as *const u8 as *const libc::c_char) {
      return b"is marked to be removed\x00" as *const u8 as *const libc::c_char;
    }
    if status_want == search_name_hashtable(b"purge\x00" as *const u8 as *const libc::c_char) {
      return b"is marked to be purged\x00" as *const u8 as *const libc::c_char;
    }
  }
  if status_want == search_name_hashtable(b"unknown\x00" as *const u8 as *const libc::c_char) {
    return b"is in an indeterminate state\x00" as *const u8 as *const libc::c_char;
  }
  if status_want == search_name_hashtable(b"install\x00" as *const u8 as *const libc::c_char) {
    return b"is marked to be installed\x00" as *const u8 as *const libc::c_char;
  }
  return b"is not installed or flagged to be installed\x00" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn index_status_file(mut filename: *const libc::c_char) {
  let mut status_file: *mut FILE = std::ptr::null_mut();
  let mut control_buffer: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut status_line: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut status_node: *mut status_node_t = std::ptr::null_mut();
  let mut status_num: libc::c_uint = 0;
  status_file = crate::libbb::wfopen::xfopen_for_read(filename);
  loop {
    control_buffer = crate::libbb::fgets_str::xmalloc_fgetline_str(
      status_file,
      b"\n\n\x00" as *const u8 as *const libc::c_char,
    );
    if control_buffer.is_null() {
      break;
    }
    let package_num: libc::c_uint = fill_package_struct(control_buffer);
    if package_num != -1i32 as libc::c_uint {
      status_node =
        xmalloc(::std::mem::size_of::<status_node_t>() as libc::c_ulong) as *mut status_node_t;
      /* fill_package_struct doesn't handle the status field */
      status_line = strstr(
        control_buffer,
        b"Status:\x00" as *const u8 as *const libc::c_char,
      );
      if !status_line.is_null() {
        status_line = status_line.offset(7);
        status_line = status_line.offset(strspn(
          status_line,
          b" \n\t\x00" as *const u8 as *const libc::c_char,
        ) as isize);
        status_line = crate::libbb::xfuncs_printf::xstrndup(
          status_line,
          strcspn(status_line, b"\n\x00" as *const u8 as *const libc::c_char) as libc::c_int,
        );
        (*status_node).set_status(search_name_hashtable(status_line) as libc::c_uint);
        free(status_line as *mut libc::c_void);
      }
      (*status_node).set_package(package_num);
      status_num = search_status_hashtable(
        (*ptr_to_globals).name_hashtable[(*(*ptr_to_globals).package_hashtable
          [(*status_node).package() as usize])
          .name() as usize],
      );
      (*ptr_to_globals).status_hashtable[status_num as usize] = status_node
    }
    free(control_buffer as *mut libc::c_void);
  }
  fclose(status_file);
}
unsafe extern "C" fn write_buffer_no_status(
  mut new_status_file: *mut FILE,
  mut control_buffer: *const libc::c_char,
) {
  let mut name: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut value: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut start: libc::c_int = 0;
  loop {
    start += read_package_field(
      &*control_buffer.offset(start as isize),
      &mut name,
      &mut value,
    );
    if name.is_null() {
      break;
    }
    if strcmp(name, b"Status\x00" as *const u8 as *const libc::c_char) != 0 {
      fprintf(
        new_status_file,
        b"%s: %s\n\x00" as *const u8 as *const libc::c_char,
        name,
        value,
      );
    }
  }
}
/* This could do with a cleanup */
unsafe extern "C" fn write_status_file(mut deb_file: *mut *mut deb_file_t) {
  let mut old_status_file: *mut FILE = crate::libbb::wfopen::xfopen_for_read(
    b"/var/lib/dpkg/status\x00" as *const u8 as *const libc::c_char,
  );
  let mut new_status_file: *mut FILE = crate::libbb::wfopen::xfopen_for_write(
    b"/var/lib/dpkg/status.udeb\x00" as *const u8 as *const libc::c_char,
  );
  let mut package_name: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut status_from_file: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut control_buffer: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut tmp_string: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut status_num: libc::c_int = 0;
  let mut field_start: libc::c_int = 0;
  let mut write_flag: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  loop
  /* Update previously known packages */
  {
    control_buffer = crate::libbb::fgets_str::xmalloc_fgetline_str(
      old_status_file,
      b"\n\n\x00" as *const u8 as *const libc::c_char,
    );
    if control_buffer.is_null() {
      break;
    }
    tmp_string = strstr(
      control_buffer,
      b"Package:\x00" as *const u8 as *const libc::c_char,
    );
    if tmp_string.is_null() {
      continue;
    }
    tmp_string = tmp_string.offset(8);
    tmp_string = tmp_string
      .offset(strspn(tmp_string, b" \n\t\x00" as *const u8 as *const libc::c_char) as isize);
    package_name = crate::libbb::xfuncs_printf::xstrndup(
      tmp_string,
      strcspn(tmp_string, b"\n\x00" as *const u8 as *const libc::c_char) as libc::c_int,
    );
    write_flag = 0;
    tmp_string = strstr(
      control_buffer,
      b"Status:\x00" as *const u8 as *const libc::c_char,
    );
    if !tmp_string.is_null() {
      /* Separate the status value from the control buffer */
      tmp_string = tmp_string.offset(7);
      tmp_string = tmp_string
        .offset(strspn(tmp_string, b" \n\t\x00" as *const u8 as *const libc::c_char) as isize);
      status_from_file = crate::libbb::xfuncs_printf::xstrndup(
        tmp_string,
        strcspn(tmp_string, b"\n\x00" as *const u8 as *const libc::c_char) as libc::c_int,
      )
    } else {
      status_from_file = std::ptr::null_mut::<libc::c_char>()
    }
    /* Find this package in the status hashtable */
    status_num = search_status_hashtable(package_name) as libc::c_int;
    if !(*ptr_to_globals).status_hashtable[status_num as usize].is_null() {
      let mut status_from_hashtable: *const libc::c_char = (*ptr_to_globals).name_hashtable
        [(*(*ptr_to_globals).status_hashtable[status_num as usize]).status() as usize];
      if strcmp(status_from_file, status_from_hashtable) != 0 {
        /* New status isn't exactly the same as old status */
        let state_status: libc::c_int = get_status(status_num as libc::c_uint, 3i32) as libc::c_int;
        if strcmp(
          b"installed\x00" as *const u8 as *const libc::c_char,
          (*ptr_to_globals).name_hashtable[state_status as usize],
        ) == 0
          || strcmp(
            b"unpacked\x00" as *const u8 as *const libc::c_char,
            (*ptr_to_globals).name_hashtable[state_status as usize],
          ) == 0
        {
          /* We need to add the control file from the package */
          i = 0;
          while !(*deb_file.offset(i as isize)).is_null() {
            if strcmp(
              package_name,
              (*ptr_to_globals).name_hashtable[(*(*ptr_to_globals).package_hashtable
                [(**deb_file.offset(i as isize)).package() as usize])
                .name() as usize],
            ) == 0
            {
              /* Write a status file entry with a modified status */
              /* remove trailing \n's */
              write_buffer_no_status(
                new_status_file,
                (**deb_file.offset(i as isize)).control_file,
              );
              set_status(
                status_num as libc::c_uint,
                b"ok\x00" as *const u8 as *const libc::c_char,
                2i32,
              );
              fprintf(
                new_status_file,
                b"Status: %s\n\n\x00" as *const u8 as *const libc::c_char,
                (*ptr_to_globals).name_hashtable
                  [(*(*ptr_to_globals).status_hashtable[status_num as usize]).status() as usize],
              );
              write_flag = 1i32;
              break;
            } else {
              i += 1
            }
          }
          /* This is temperary, debugging only */
          if (*deb_file.offset(i as isize)).is_null() {
            crate::libbb::verror_msg::bb_error_msg_and_die(b"ALERT: cannot find a control file, your status file may be broken, status may be incorrect for %s\x00"
                                                 as *const u8 as
                                                 *const libc::c_char,
                                             package_name);
          }
        } else if strcmp(
          b"not-installed\x00" as *const u8 as *const libc::c_char,
          (*ptr_to_globals).name_hashtable[state_status as usize],
        ) == 0
        {
          /* Only write the Package, Status, Priority and Section lines */
          fprintf(
            new_status_file,
            b"Package: %s\n\x00" as *const u8 as *const libc::c_char,
            package_name,
          );
          fprintf(
            new_status_file,
            b"Status: %s\n\x00" as *const u8 as *const libc::c_char,
            status_from_hashtable,
          );
          loop {
            let mut field_name: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
            let mut field_value: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
            field_start += read_package_field(
              &mut *control_buffer.offset(field_start as isize),
              &mut field_name,
              &mut field_value,
            );
            if field_name.is_null() {
              break;
            }
            if strcmp(
              field_name,
              b"Priority\x00" as *const u8 as *const libc::c_char,
            ) == 0
              || strcmp(
                field_name,
                b"Section\x00" as *const u8 as *const libc::c_char,
              ) == 0
            {
              fprintf(
                new_status_file,
                b"%s: %s\n\x00" as *const u8 as *const libc::c_char,
                field_name,
                field_value,
              );
            }
          }
          write_flag = 1i32;
          fputs_unlocked(
            b"\n\x00" as *const u8 as *const libc::c_char,
            new_status_file,
          );
        } else if strcmp(
          b"config-files\x00" as *const u8 as *const libc::c_char,
          (*ptr_to_globals).name_hashtable[state_status as usize],
        ) == 0
        {
          loop
          /* only change the status line */
          {
            let mut field_name_0: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
            let mut field_value_0: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
            field_start += read_package_field(
              &mut *control_buffer.offset(field_start as isize),
              &mut field_name_0,
              &mut field_value_0,
            );
            if field_name_0.is_null() {
              break;
            }
            /* Setup start point for next field */
            if strcmp(
              field_name_0,
              b"Status\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
              fprintf(
                new_status_file,
                b"Status: %s\n\x00" as *const u8 as *const libc::c_char,
                status_from_hashtable,
              );
            } else {
              fprintf(
                new_status_file,
                b"%s: %s\n\x00" as *const u8 as *const libc::c_char,
                field_name_0,
                field_value_0,
              );
            }
          }
          write_flag = 1i32;
          fputs_unlocked(
            b"\n\x00" as *const u8 as *const libc::c_char,
            new_status_file,
          );
        }
      }
    }
    /* If the package from the status file wasn't handle above, do it now*/
    if write_flag == 0 {
      fprintf(
        new_status_file,
        b"%s\n\n\x00" as *const u8 as *const libc::c_char,
        control_buffer,
      );
    }
    free(status_from_file as *mut libc::c_void);
    free(package_name as *mut libc::c_void);
    free(control_buffer as *mut libc::c_void);
  }
  /* Write any new packages */
  i = 0;
  while !(*deb_file.offset(i as isize)).is_null() {
    status_num = search_status_hashtable(
      (*ptr_to_globals).name_hashtable[(*(*ptr_to_globals).package_hashtable
        [(**deb_file.offset(i as isize)).package() as usize])
        .name() as usize],
    ) as libc::c_int;
    if strcmp(
      b"reinstreq\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).name_hashtable[get_status(status_num as libc::c_uint, 2i32) as usize],
    ) == 0
    {
      write_buffer_no_status(
        new_status_file,
        (**deb_file.offset(i as isize)).control_file,
      );
      set_status(
        status_num as libc::c_uint,
        b"ok\x00" as *const u8 as *const libc::c_char,
        2i32,
      );
      fprintf(
        new_status_file,
        b"Status: %s\n\n\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).name_hashtable
          [(*(*ptr_to_globals).status_hashtable[status_num as usize]).status() as usize],
      );
    }
    i += 1
  }
  fclose(old_status_file);
  fclose(new_status_file);
  /* Create a separate backfile to dpkg */
  if rename(
    b"/var/lib/dpkg/status\x00" as *const u8 as *const libc::c_char,
    b"/var/lib/dpkg/status.udeb.bak\x00" as *const u8 as *const libc::c_char,
  ) == -1i32
  {
    if *bb_errno != 2i32 {
      crate::libbb::verror_msg::bb_simple_error_msg_and_die(
        b"can\'t create backup status file\x00" as *const u8 as *const libc::c_char,
      );
    }
    /* Its ok if renaming the status file fails because status
     * file doesn't exist, maybe we are starting from scratch */
    crate::libbb::verror_msg::bb_simple_error_msg(
      b"no status file found, creating new one\x00" as *const u8 as *const libc::c_char,
    );
  }
  crate::libbb::xfuncs_printf::xrename(
    b"/var/lib/dpkg/status.udeb\x00" as *const u8 as *const libc::c_char,
    b"/var/lib/dpkg/status\x00" as *const u8 as *const libc::c_char,
  );
}
/* This function returns TRUE if the given package can satisfy a
 * dependency of type depend_type.
 *
 * A pre-depends is satisfied only if a package is already installed,
 * which a regular depends can be satisfied by a package which we want
 * to install.
 */
unsafe extern "C" fn package_satisfies_dependency(
  mut package: libc::c_int,
  mut depend_type: libc::c_int,
) -> libc::c_int {
  let mut status_num: libc::c_int = search_status_hashtable(
    (*ptr_to_globals).name_hashtable
      [(*(*ptr_to_globals).package_hashtable[package as usize]).name() as usize],
  ) as libc::c_int;
  /* status could be unknown if package is a pure virtual
   * provides which cannot satisfy any dependency by itself.
   */
  if (*ptr_to_globals).status_hashtable[status_num as usize].is_null() {
    return 0;
  }
  match depend_type {
    1 => {
      return (get_status(status_num as libc::c_uint, 3i32)
        == search_name_hashtable(b"installed\x00" as *const u8 as *const libc::c_char)
          as libc::c_uint) as libc::c_int
    }
    3 => {
      return (get_status(status_num as libc::c_uint, 1i32)
        == search_name_hashtable(b"install\x00" as *const u8 as *const libc::c_char)
          as libc::c_uint) as libc::c_int
    }
    _ => {}
  }
  return 0;
}
unsafe extern "C" fn check_deps(
  mut deb_file: *mut *mut deb_file_t,
  mut deb_start: libc::c_int,
) -> libc::c_int
/*, int dep_max_count - ?? */ {
  let mut conflicts: *mut libc::c_int = std::ptr::null_mut();
  let mut conflicts_num: libc::c_int = 0;
  let mut i: libc::c_int = deb_start;
  let mut j: libc::c_int = 0;
  /* Check for conflicts
   * TODO: TEST if conflicts with other packages to be installed
   *
   * Add install packages and the packages they provide
   * to the list of files to check conflicts for
   */
  /* Create array of package numbers to check against
   * installed package for conflicts*/
  while !(*deb_file.offset(i as isize)).is_null() {
    let package_num: libc::c_uint = (**deb_file.offset(i as isize)).package();
    conflicts = crate::libbb::xrealloc_vector::xrealloc_vector_helper(
      conflicts as *mut libc::c_void,
      ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong) << 8i32)
        .wrapping_add(2i32 as libc::c_ulong) as libc::c_uint,
      conflicts_num,
    ) as *mut libc::c_int;
    *conflicts.offset(conflicts_num as isize) = package_num as libc::c_int;
    conflicts_num += 1;
    /* add provides to conflicts list */
    j = 0;
    while j
      < (*(*ptr_to_globals).package_hashtable[package_num as usize]).num_of_edges() as libc::c_int
    {
      if (**(*(*ptr_to_globals).package_hashtable[package_num as usize])
        .edge
        .offset(j as isize))
      .type_0() as libc::c_int
        == EDGE_PROVIDES as libc::c_int
      {
        let conflicts_package_num: libc::c_int = search_package_hashtable(
          (**(*(*ptr_to_globals).package_hashtable[package_num as usize])
            .edge
            .offset(j as isize))
          .name(),
          (**(*(*ptr_to_globals).package_hashtable[package_num as usize])
            .edge
            .offset(j as isize))
          .version(),
          (**(*(*ptr_to_globals).package_hashtable[package_num as usize])
            .edge
            .offset(j as isize))
          .operator(),
        );
        if (*ptr_to_globals).package_hashtable[conflicts_package_num as usize].is_null() {
          /* create a new package */
          let mut new_node: *mut common_node_t = crate::libbb::xfuncs_printf::xzalloc(
            ::std::mem::size_of::<common_node_t>() as libc::c_ulong,
          ) as *mut common_node_t;
          (*new_node).set_name(
            (**(*(*ptr_to_globals).package_hashtable[package_num as usize])
              .edge
              .offset(j as isize))
            .name(),
          );
          (*new_node).set_version(
            (**(*(*ptr_to_globals).package_hashtable[package_num as usize])
              .edge
              .offset(j as isize))
            .version(),
          );
          (*ptr_to_globals).package_hashtable[conflicts_package_num as usize] = new_node
        }
        conflicts = crate::libbb::xrealloc_vector::xrealloc_vector_helper(
          conflicts as *mut libc::c_void,
          ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong) << 8i32)
            .wrapping_add(2i32 as libc::c_ulong) as libc::c_uint,
          conflicts_num,
        ) as *mut libc::c_int;
        *conflicts.offset(conflicts_num as isize) = conflicts_package_num;
        conflicts_num += 1
      }
      j += 1
    }
    i += 1
  }
  /* Check conflicts */
  i = 0;
  while !(*deb_file.offset(i as isize)).is_null() {
    let mut package_node: *const common_node_t =
      (*ptr_to_globals).package_hashtable[(**deb_file.offset(i as isize)).package() as usize];
    let mut status_num: libc::c_int = 0;
    status_num =
      search_status_hashtable((*ptr_to_globals).name_hashtable[(*package_node).name() as usize])
        as libc::c_int;
    if get_status(status_num as libc::c_uint, 3i32)
      == search_name_hashtable(b"installed\x00" as *const u8 as *const libc::c_char) as libc::c_uint
    {
      i += 1
    } else {
      j = 0;
      while j < (*package_node).num_of_edges() as libc::c_int {
        let mut package_edge: *const edge_t = *(*package_node).edge.offset(j as isize);
        if (*package_edge).type_0() as libc::c_int == EDGE_CONFLICTS as libc::c_int {
          let package_num_0: libc::c_uint = search_package_hashtable(
            (*package_edge).name(),
            (*package_edge).version(),
            (*package_edge).operator(),
          ) as libc::c_uint;
          let mut result: libc::c_int = 0;
          if !(*ptr_to_globals).package_hashtable[package_num_0 as usize].is_null() {
            status_num = search_status_hashtable(
              (*ptr_to_globals).name_hashtable
                [(*(*ptr_to_globals).package_hashtable[package_num_0 as usize]).name() as usize],
            ) as libc::c_int;
            if get_status(status_num as libc::c_uint, 1i32)
              == search_name_hashtable(b"install\x00" as *const u8 as *const libc::c_char)
                as libc::c_uint
            {
              result = test_version(
                (*(*ptr_to_globals).package_hashtable
                  [(**deb_file.offset(i as isize)).package() as usize])
                  .version(),
                (*package_edge).version(),
                (*package_edge).operator(),
              )
            }
          }
          if result != 0 {
            crate::libbb::verror_msg::bb_error_msg_and_die(
              b"package %s conflicts with %s\x00" as *const u8 as *const libc::c_char,
              (*ptr_to_globals).name_hashtable[(*package_node).name() as usize],
              (*ptr_to_globals).name_hashtable[(*package_edge).name() as usize],
            );
          }
        }
        j += 1
      }
      i += 1
    }
  }
  /* Check dependentcies */
  i = 0;
  while i < 10007i32 {
    let mut status_num_0: libc::c_int = 0;
    let mut number_of_alternatives: libc::c_int = 0;
    let mut root_of_alternatives: *const edge_t = std::ptr::null();
    let mut package_node_0: *const common_node_t = (*ptr_to_globals).package_hashtable[i as usize];
    /* If the package node does not exist then this
     * package is a virtual one. In which case there are
     * no dependencies to check.
     */
    if !package_node_0.is_null() {
      status_num_0 = search_status_hashtable(
        (*ptr_to_globals).name_hashtable[(*package_node_0).name() as usize],
      ) as libc::c_int;
      /* If there is no status then this package is a
       * virtual one provided by something else. In which
       * case there are no dependencies to check.
       */
      if !(*ptr_to_globals).status_hashtable[status_num_0 as usize].is_null() {
        /* If we don't want this package installed then we may
         * as well ignore it's dependencies.
         */
        if !(get_status(status_num_0 as libc::c_uint, 1i32)
          != search_name_hashtable(b"install\x00" as *const u8 as *const libc::c_char)
            as libc::c_uint)
        {
          /* This code is tested only for EDGE_DEPENDS, since I
           * have no suitable pre-depends available. There is no
           * reason that it shouldn't work though :-)
           */
          j = 0;
          while j < (*package_node_0).num_of_edges() as libc::c_int {
            let mut package_edge_0: *const edge_t = *(*package_node_0).edge.offset(j as isize);
            let mut package_num_1: libc::c_uint = 0;
            if (*package_edge_0).type_0() as libc::c_int == EDGE_OR_PRE_DEPENDS as libc::c_int
              || (*package_edge_0).type_0() as libc::c_int == EDGE_OR_DEPENDS as libc::c_int
            {
              /* start an EDGE_OR_ list */
              number_of_alternatives = (*package_edge_0).version() as libc::c_int;
              root_of_alternatives = package_edge_0
            } else {
              if number_of_alternatives == 0 {
                /* not in the middle of an EDGE_OR_ list */
                number_of_alternatives = 1i32;
                root_of_alternatives = std::ptr::null()
              }
              package_num_1 = search_package_hashtable(
                (*package_edge_0).name(),
                (*package_edge_0).version(),
                (*package_edge_0).operator(),
              ) as libc::c_uint;
              if (*package_edge_0).type_0() as libc::c_int == EDGE_PRE_DEPENDS as libc::c_int
                || (*package_edge_0).type_0() as libc::c_int == EDGE_DEPENDS as libc::c_int
              {
                let mut result_0: libc::c_int = 1i32;
                status_num_0 = 0;
                /* If we are inside an alternative then check
                 * this edge is the right type.
                 *
                 * EDGE_DEPENDS == OR_DEPENDS -1
                 * EDGE_PRE_DEPENDS == OR_PRE_DEPENDS -1
                 */
                if !root_of_alternatives.is_null()
                  && (*package_edge_0).type_0() as libc::c_int
                    != (*root_of_alternatives).type_0() as libc::c_int - 1i32
                {
                  crate::libbb::verror_msg::bb_error_msg_and_die(
                    b"fatal error, package dependencies corrupt: %d != %d - 1\x00" as *const u8
                      as *const libc::c_char,
                    (*package_edge_0).type_0() as libc::c_int,
                    (*root_of_alternatives).type_0() as libc::c_int,
                  );
                }
                if !(*ptr_to_globals).package_hashtable[package_num_1 as usize].is_null() {
                  result_0 = (package_satisfies_dependency(
                    package_num_1 as libc::c_int,
                    (*package_edge_0).type_0() as libc::c_int,
                  ) == 0) as libc::c_int
                }
                if result_0 != 0 {
                  /* check for other package which provide what we are looking for */
                  let mut provider: libc::c_int = -1i32;
                  loop {
                    provider =
                      search_for_provides((*package_edge_0).name() as libc::c_int, provider);
                    if !(provider > -1i32) {
                      break;
                    }
                    if (*ptr_to_globals).package_hashtable[provider as usize].is_null() {
                      puts(
                        b"Have a provider but no package information for it\x00" as *const u8
                          as *const libc::c_char,
                      );
                    } else {
                      result_0 = (package_satisfies_dependency(
                        provider,
                        (*package_edge_0).type_0() as libc::c_int,
                      ) == 0) as libc::c_int;
                      if result_0 == 0 {
                        break;
                      }
                    }
                  }
                }
                /* It must be already installed, or to be installed */
                number_of_alternatives -= 1;
                if result_0 != 0 && number_of_alternatives == 0 {
                  if !root_of_alternatives.is_null() {
                    crate::libbb::verror_msg::bb_error_msg_and_die(
                      b"package %s %sdepends on %s, which %s\x00" as *const u8
                        as *const libc::c_char,
                      (*ptr_to_globals).name_hashtable[(*package_node_0).name() as usize],
                      if (*package_edge_0).type_0() as libc::c_int
                        == EDGE_PRE_DEPENDS as libc::c_int
                      {
                        b"pre-\x00" as *const u8 as *const libc::c_char
                      } else {
                        b"\x00" as *const u8 as *const libc::c_char
                      },
                      (*ptr_to_globals).name_hashtable[(*root_of_alternatives).name() as usize],
                      b"cannot be satisfied\x00" as *const u8 as *const libc::c_char,
                    );
                  }
                  crate::libbb::verror_msg::bb_error_msg_and_die(
                    b"package %s %sdepends on %s, which %s\x00" as *const u8 as *const libc::c_char,
                    (*ptr_to_globals).name_hashtable[(*package_node_0).name() as usize],
                    if (*package_edge_0).type_0() as libc::c_int == EDGE_PRE_DEPENDS as libc::c_int
                    {
                      b"pre-\x00" as *const u8 as *const libc::c_char
                    } else {
                      b"\x00" as *const u8 as *const libc::c_char
                    },
                    (*ptr_to_globals).name_hashtable[(*package_edge_0).name() as usize],
                    describe_status(status_num_0),
                  );
                }
                if result_0 == 0 && number_of_alternatives != 0 {
                  /* we've found a package which
                   * satisfies the dependency,
                   * so skip over the rest of
                   * the alternatives.
                   */
                  j += number_of_alternatives;
                  number_of_alternatives = 0
                }
              }
            }
            j += 1
          }
        }
      }
    }
    i += 1
  }
  free(conflicts as *mut libc::c_void);
  return 1i32;
}
unsafe extern "C" fn create_list(mut filename: *const libc::c_char) -> *mut *mut libc::c_char {
  let mut list_stream: *mut FILE = std::ptr::null_mut();
  let mut file_list: *mut *mut libc::c_char = std::ptr::null_mut();
  let mut line: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut count: libc::c_int = 0;
  /* don't use [xw]fopen here, handle error ourself */
  list_stream = crate::libbb::wfopen::fopen_for_read(filename);
  if list_stream.is_null() {
    return std::ptr::null_mut();
  }
  file_list = std::ptr::null_mut();
  count = 0;
  loop {
    line = crate::libbb::get_line_from_file::xmalloc_fgetline(list_stream);
    if line.is_null() {
      break;
    }
    file_list = crate::libbb::xrealloc_vector::xrealloc_vector_helper(
      file_list as *mut libc::c_void,
      ((::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong) << 8i32)
        .wrapping_add(2i32 as libc::c_ulong) as libc::c_uint,
      count,
    ) as *mut *mut libc::c_char;
    let fresh5 = count;
    count += 1;
    let ref mut fresh6 = *file_list.offset(fresh5 as isize);
    *fresh6 = line
    /*file_list[count] = NULL; - xrealloc_vector did it */
  }
  fclose(list_stream);
  return file_list;
}
/* maybe i should try and hook this into remove_file.c somehow */
unsafe extern "C" fn remove_file_array(
  mut remove_names: *mut *mut libc::c_char,
  mut exclude_names: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut path_stat: stat = std::mem::zeroed(); /* not removed anything yet */
  let mut remove_flag: libc::c_int = 1i32;
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  if remove_names.is_null() {
    return 0;
  }
  let mut current_block_9: u64;
  i = 0;
  while !(*remove_names.offset(i as isize)).is_null() {
    if !exclude_names.is_null() {
      j = 0;
      loop {
        if (*exclude_names.offset(j as isize)).is_null() {
          current_block_9 = 5399440093318478209;
          break;
        }
        if strcmp(
          *remove_names.offset(i as isize),
          *exclude_names.offset(j as isize),
        ) == 0
        {
          current_block_9 = 820271813250567934;
          break;
        }
        j += 1
      }
    } else {
      current_block_9 = 5399440093318478209;
    }
    match current_block_9 {
      5399440093318478209 =>
      /* TODO: why we are checking lstat? we can just try rm/rmdir */
      {
        if !(lstat(*remove_names.offset(i as isize), &mut path_stat) < 0) {
          if path_stat.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint {
            remove_flag &= rmdir(*remove_names.offset(i as isize))
          /* 0 if no error */
          } else {
            remove_flag &= unlink(*remove_names.offset(i as isize))
            /* 0 if no error */
          }
        }
      }
      _ => {}
    }
    i += 1
  }
  return (remove_flag == 0) as libc::c_int;
}
unsafe extern "C" fn run_package_script_or_die(
  mut package_name: *const libc::c_char,
  mut script_type: *const libc::c_char,
) {
  let mut script_path: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut result: libc::c_int = 0;
  script_path = crate::libbb::xfuncs_printf::xasprintf(
    b"/var/lib/dpkg/info/%s.%s\x00" as *const u8 as *const libc::c_char,
    package_name,
    script_type,
  );
  /* If the file doesn't exist it isn't fatal */
  result = if access(script_path, 0) != 0 {
    0
  } else {
    system(script_path)
  };
  free(script_path as *mut libc::c_void);
  if result != 0 {
    crate::libbb::verror_msg::bb_error_msg_and_die(
      b"%s failed, exit code %d\x00" as *const u8 as *const libc::c_char,
      script_type,
      result,
    );
  };
}
/*
The policy manual defines what scripts get called when and with
what arguments. I realize that busybox does not support all of
these scenarios, but it does support some of them; it does not,
however, run them with any parameters in run_package_script_or_die().
Here are the scripts:

preinst install
preinst install <old_version>
preinst upgrade <old_version>
preinst abort_upgrade <new_version>
postinst configure <most_recent_version>
postinst abort-upgade <new_version>
postinst abort-remove
postinst abort-remove in-favour <package> <version>
postinst abort-deconfigure in-favor <failed_install_package> removing <conflicting_package> <version>
prerm remove
prerm upgrade <new_version>
prerm failed-upgrade <old_version>
prerm remove in-favor <package> <new_version>
prerm deconfigure in-favour <package> <version> removing <package> <version>
postrm remove
postrm purge
postrm upgrade <new_version>
postrm failed-upgrade <old_version>
postrm abort-install
postrm abort-install <old_version>
postrm abort-upgrade <old_version>
postrm disappear <overwriter> <version>
*/
static mut all_control_files: [*const libc::c_char; 10] = [
  b"preinst\x00" as *const u8 as *const libc::c_char,
  b"postinst\x00" as *const u8 as *const libc::c_char,
  b"prerm\x00" as *const u8 as *const libc::c_char,
  b"postrm\x00" as *const u8 as *const libc::c_char,
  b"list\x00" as *const u8 as *const libc::c_char,
  b"md5sums\x00" as *const u8 as *const libc::c_char,
  b"shlibs\x00" as *const u8 as *const libc::c_char,
  b"conffiles\x00" as *const u8 as *const libc::c_char,
  b"config\x00" as *const u8 as *const libc::c_char,
  b"templates\x00" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn all_control_list(
  mut package_name: *const libc::c_char,
) -> *mut *mut libc::c_char {
  let mut i: libc::c_uint = 0 as libc::c_uint;
  let mut remove_files: *mut *mut libc::c_char = std::ptr::null_mut();
  /* Create a list of all /var/lib/dpkg/info/<package> files */
  remove_files = crate::libbb::xfuncs_printf::xzalloc(
    (::std::mem::size_of::<[*const libc::c_char; 10]>() as libc::c_ulong)
      .wrapping_add(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
  ) as *mut *mut libc::c_char;
  while i
    < (::std::mem::size_of::<[*const libc::c_char; 10]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
      as libc::c_uint
  {
    let ref mut fresh7 = *remove_files.offset(i as isize);
    *fresh7 = crate::libbb::xfuncs_printf::xasprintf(
      b"/var/lib/dpkg/info/%s.%s\x00" as *const u8 as *const libc::c_char,
      package_name,
      all_control_files[i as usize],
    );
    i = i.wrapping_add(1)
  }
  return remove_files;
}
unsafe extern "C" fn free_array(mut array: *mut *mut libc::c_char) {
  if !array.is_null() {
    let mut i: libc::c_uint = 0 as libc::c_uint;
    while !(*array.offset(i as isize)).is_null() {
      free(*array.offset(i as isize) as *mut libc::c_void);
      i = i.wrapping_add(1)
    }
    free(array as *mut libc::c_void);
  };
}
/* This function lists information on the installed packages. It loops through
 * the status_hashtable to retrieve the info. This results in smaller code than
 * scanning the status file. The resulting list, however, is unsorted.
 */
unsafe extern "C" fn list_packages(mut pattern: *const libc::c_char) {
  let mut i: libc::c_int = 0;
  puts(b"    Name           Version\x00" as *const u8 as *const libc::c_char);
  puts(b"+++-==============-==============\x00" as *const u8 as *const libc::c_char);
  /* go through status hash, dereference package hash and finally strings */
  i = 0; /* status string */
  while i < 8191i32 + 1i32 {
    if !(*ptr_to_globals).status_hashtable[i as usize].is_null() {
      let mut stat_str: *const libc::c_char = std::ptr::null(); /* package name */
      let mut name_str: *const libc::c_char = std::ptr::null(); /* version */
      let mut vers_str: *const libc::c_char = std::ptr::null(); /* status abbreviations */
      let mut s1: libc::c_char = 0; /* space count */
      let mut s2: libc::c_char = 0;
      let mut spccnt: libc::c_int = 0;
      let mut j: libc::c_int = 0;
      stat_str = (*ptr_to_globals).name_hashtable
        [(*(*ptr_to_globals).status_hashtable[i as usize]).status() as usize];
      name_str = (*ptr_to_globals).name_hashtable[(*(*ptr_to_globals).package_hashtable
        [(*(*ptr_to_globals).status_hashtable[i as usize]).package() as usize])
        .name() as usize];
      vers_str = (*ptr_to_globals).name_hashtable[(*(*ptr_to_globals).package_hashtable
        [(*(*ptr_to_globals).status_hashtable[i as usize]).package() as usize])
        .version() as usize];
      if !(!pattern.is_null() && fnmatch(pattern, name_str, 0) != 0) {
        /* get abbreviation for status field 1 */
        s1 = if *stat_str.offset(0) as libc::c_int == 'i' as i32 {
          'i' as i32
        } else {
          'r' as i32
        } as libc::c_char;
        /* get abbreviation for status field 2 */
        j = 0;
        spccnt = 0;
        while *stat_str.offset(j as isize) as libc::c_int != 0 && spccnt < 2i32 {
          if *stat_str.offset(j as isize) as libc::c_int == ' ' as i32 {
            spccnt += 1
          }
          j += 1
        }
        s2 = *stat_str.offset(j as isize);
        /* print out the line formatted like Debian dpkg */
        printf(
          b"%c%c  %-14s %s\n\x00" as *const u8 as *const libc::c_char,
          s1 as libc::c_int,
          s2 as libc::c_int,
          name_str,
          vers_str,
        );
      }
    }
    i += 1
  }
}
unsafe extern "C" fn remove_package(package_num: libc::c_uint, mut noisy: libc::c_int) {
  let mut package_name: *const libc::c_char = (*ptr_to_globals).name_hashtable
    [(*(*ptr_to_globals).package_hashtable[package_num as usize]).name() as usize];
  let mut package_version: *const libc::c_char = (*ptr_to_globals).name_hashtable
    [(*(*ptr_to_globals).package_hashtable[package_num as usize]).version() as usize];
  let status_num: libc::c_uint = search_status_hashtable(package_name);
  let package_name_length: libc::c_int = strlen(package_name) as libc::c_int;
  let mut remove_files: *mut *mut libc::c_char = std::ptr::null_mut();
  let mut exclude_files: *mut *mut libc::c_char = std::ptr::null_mut();
  let vla = (package_name_length + 25i32) as usize;
  let mut list_name: Vec<libc::c_char> = ::std::vec::from_elem(0, vla);
  let vla_0 = (package_name_length + 30i32) as usize;
  let mut conffile_name: Vec<libc::c_char> = ::std::vec::from_elem(0, vla_0);
  if noisy != 0 {
    printf(
      b"Removing %s (%s)...\n\x00" as *const u8 as *const libc::c_char,
      package_name,
      package_version,
    );
  }
  /* Run prerm script */
  run_package_script_or_die(
    package_name,
    b"prerm\x00" as *const u8 as *const libc::c_char,
  );
  /* Create a list of files to remove, and a separate list of those to keep */
  sprintf(
    list_name.as_mut_ptr(),
    b"/var/lib/dpkg/info/%s.%s\x00" as *const u8 as *const libc::c_char,
    package_name,
    b"list\x00" as *const u8 as *const libc::c_char,
  );
  remove_files = create_list(list_name.as_mut_ptr());
  sprintf(
    conffile_name.as_mut_ptr(),
    b"/var/lib/dpkg/info/%s.%s\x00" as *const u8 as *const libc::c_char,
    package_name,
    b"conffiles\x00" as *const u8 as *const libc::c_char,
  );
  exclude_files = create_list(conffile_name.as_mut_ptr());
  /* Some directories can't be removed straight away, so do multiple passes */
  while remove_file_array(remove_files, exclude_files) != 0 {}
  free_array(exclude_files);
  free_array(remove_files);
  /* Create a list of files in /var/lib/dpkg/info/<package>.* to keep */
  exclude_files = crate::libbb::xfuncs_printf::xzalloc(
    (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
      .wrapping_mul(3i32 as libc::c_ulong),
  ) as *mut *mut libc::c_char;
  let ref mut fresh8 = *exclude_files.offset(0);
  *fresh8 = crate::libbb::xfuncs_printf::xstrdup(conffile_name.as_mut_ptr());
  let ref mut fresh9 = *exclude_files.offset(1);
  *fresh9 = crate::libbb::xfuncs_printf::xasprintf(
    b"/var/lib/dpkg/info/%s.%s\x00" as *const u8 as *const libc::c_char,
    package_name,
    b"postrm\x00" as *const u8 as *const libc::c_char,
  );
  /* Create a list of all /var/lib/dpkg/info/<package> files */
  remove_files = all_control_list(package_name);
  remove_file_array(remove_files, exclude_files);
  free_array(remove_files);
  free_array(exclude_files);
  /* rename <package>.conffiles to <package>.list
   * The conffiles control file isn't required in Debian packages, so don't
   * error out if it's missing.  */
  rename(conffile_name.as_mut_ptr(), list_name.as_mut_ptr());
  /* Change package status */
  set_status(
    status_num,
    b"config-files\x00" as *const u8 as *const libc::c_char,
    3i32,
  );
}
unsafe extern "C" fn purge_package(package_num: libc::c_uint) {
  let mut package_name: *const libc::c_char = (*ptr_to_globals).name_hashtable
    [(*(*ptr_to_globals).package_hashtable[package_num as usize]).name() as usize];
  let mut package_version: *const libc::c_char = (*ptr_to_globals).name_hashtable
    [(*(*ptr_to_globals).package_hashtable[package_num as usize]).version() as usize];
  let status_num: libc::c_uint = search_status_hashtable(package_name);
  let mut remove_files: *mut *mut libc::c_char = std::ptr::null_mut();
  let mut exclude_files: *mut *mut libc::c_char = std::ptr::null_mut();
  let vla = strlen(package_name).wrapping_add(25i32 as libc::c_ulong) as usize;
  let mut list_name: Vec<libc::c_char> = ::std::vec::from_elem(0, vla);
  printf(
    b"Purging %s (%s)...\n\x00" as *const u8 as *const libc::c_char,
    package_name,
    package_version,
  );
  /* Run prerm script */
  run_package_script_or_die(
    package_name,
    b"prerm\x00" as *const u8 as *const libc::c_char,
  );
  /* Create a list of files to remove */
  sprintf(
    list_name.as_mut_ptr(),
    b"/var/lib/dpkg/info/%s.%s\x00" as *const u8 as *const libc::c_char,
    package_name,
    b"list\x00" as *const u8 as *const libc::c_char,
  );
  remove_files = create_list(list_name.as_mut_ptr());
  /* Some directories cant be removed straight away, so do multiple passes */
  while remove_file_array(remove_files, 0 as *mut *mut libc::c_char) != 0 {}
  free_array(remove_files);
  /* Create a list of all /var/lib/dpkg/info/<package> files */
  remove_files = all_control_list(package_name);
  /* Delete all of them except the postrm script */
  exclude_files = crate::libbb::xfuncs_printf::xzalloc(
    (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
      .wrapping_mul(2i32 as libc::c_ulong),
  ) as *mut *mut libc::c_char;
  let ref mut fresh10 = *exclude_files.offset(0);
  *fresh10 = crate::libbb::xfuncs_printf::xasprintf(
    b"/var/lib/dpkg/info/%s.%s\x00" as *const u8 as *const libc::c_char,
    package_name,
    b"postrm\x00" as *const u8 as *const libc::c_char,
  );
  remove_file_array(remove_files, exclude_files);
  free_array(exclude_files);
  /* Run and remove postrm script */
  run_package_script_or_die(
    package_name,
    b"postrm\x00" as *const u8 as *const libc::c_char,
  );
  remove_file_array(remove_files, 0 as *mut *mut libc::c_char);
  free_array(remove_files);
  /* Change package status */
  set_status(
    status_num,
    b"not-installed\x00" as *const u8 as *const libc::c_char,
    3i32,
  );
}
unsafe extern "C" fn init_archive_deb_ar(
  mut filename: *const libc::c_char,
) -> *mut archive_handle_t {
  let mut ar_handle: *mut archive_handle_t = std::ptr::null_mut();
  /* Setup an ar archive handle that refers to the gzip sub archive */
  ar_handle = crate::archival::libarchive::init_handle::init_handle();
  (*ar_handle).filter = Some(
    crate::archival::libarchive::filter_accept_list_reassign::filter_accept_list_reassign
      as unsafe extern "C" fn(_: *mut archive_handle_t) -> libc::c_char,
  );
  (*ar_handle).src_fd = crate::libbb::xfuncs_printf::xopen(filename, 0);
  return ar_handle;
}
unsafe extern "C" fn init_archive_deb_control(mut ar_handle: *mut archive_handle_t) {
  let mut tar_handle: *mut archive_handle_t = std::ptr::null_mut();
  /* Setup the tar archive handle */
  tar_handle = crate::archival::libarchive::init_handle::init_handle();
  (*tar_handle).src_fd = (*ar_handle).src_fd;
  /* We don't care about data.tar.* or debian-binary, just control.tar.* */
  crate::libbb::llist::llist_add_to(
    &mut (*ar_handle).accept,
    b"control.tar\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      as *mut libc::c_void,
  );
  crate::libbb::llist::llist_add_to(
    &mut (*ar_handle).accept,
    b"control.tar.gz\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      as *mut libc::c_void,
  );
  crate::libbb::llist::llist_add_to(
    &mut (*ar_handle).accept,
    b"control.tar.bz2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      as *mut libc::c_void,
  );
  crate::libbb::llist::llist_add_to(
    &mut (*ar_handle).accept,
    b"control.tar.xz\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      as *mut libc::c_void,
  );
  /* Assign the tar handle as a subarchive of the ar handle */
  (*ar_handle).dpkg__sub_archive = tar_handle;
}
unsafe extern "C" fn init_archive_deb_data(mut ar_handle: *mut archive_handle_t) {
  let mut tar_handle: *mut archive_handle_t = std::ptr::null_mut();
  /* Setup the tar archive handle */
  tar_handle = crate::archival::libarchive::init_handle::init_handle();
  (*tar_handle).src_fd = (*ar_handle).src_fd;
  /* We don't care about control.tar.* or debian-binary, just data.tar.* */
  crate::libbb::llist::llist_add_to(
    &mut (*ar_handle).accept,
    b"data.tar\x00" as *const u8 as *const libc::c_char as *mut libc::c_char as *mut libc::c_void,
  );
  crate::libbb::llist::llist_add_to(
    &mut (*ar_handle).accept,
    b"data.tar.gz\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      as *mut libc::c_void,
  );
  crate::libbb::llist::llist_add_to(
    &mut (*ar_handle).accept,
    b"data.tar.bz2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      as *mut libc::c_void,
  );
  crate::libbb::llist::llist_add_to(
    &mut (*ar_handle).accept,
    b"data.tar.lzma\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      as *mut libc::c_void,
  );
  crate::libbb::llist::llist_add_to(
    &mut (*ar_handle).accept,
    b"data.tar.xz\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      as *mut libc::c_void,
  );
  /* Assign the tar handle as a subarchive of the ar handle */
  (*ar_handle).dpkg__sub_archive = tar_handle;
}
unsafe extern "C" fn data_extract_to_buffer(mut archive_handle: *mut archive_handle_t) {
  let mut size: libc::c_uint = (*(*archive_handle).file_header).size as libc::c_uint;
  (*archive_handle).dpkg__buffer =
    crate::libbb::xfuncs_printf::xzalloc(size.wrapping_add(1i32 as libc::c_uint) as size_t)
      as *mut libc::c_char;
  crate::libbb::read_printf::xread(
    (*archive_handle).src_fd,
    (*archive_handle).dpkg__buffer as *mut libc::c_void,
    size as size_t,
  );
}
unsafe extern "C" fn deb_extract_control_file_to_buffer(
  mut ar_handle: *mut archive_handle_t,
  mut myaccept: *mut llist_t,
) -> *mut libc::c_char {
  (*(*ar_handle).dpkg__sub_archive).action_data =
    Some(data_extract_to_buffer as unsafe extern "C" fn(_: *mut archive_handle_t) -> ());
  (*(*ar_handle).dpkg__sub_archive).accept = myaccept;
  (*(*ar_handle).dpkg__sub_archive).filter = Some(
    crate::archival::libarchive::filter_accept_list::filter_accept_list
      as unsafe extern "C" fn(_: *mut archive_handle_t) -> libc::c_char,
  );
  crate::archival::libarchive::unpack_ar_archive::unpack_ar_archive(ar_handle);
  close((*ar_handle).src_fd);
  return (*(*ar_handle).dpkg__sub_archive).dpkg__buffer;
}
unsafe extern "C" fn append_control_file_to_llist(
  mut package_name: *const libc::c_char,
  mut control_name: *const libc::c_char,
  mut ll: *mut *mut llist_t,
) {
  let mut fp: *mut FILE = std::ptr::null_mut();
  let mut filename: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut line: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  filename = crate::libbb::xfuncs_printf::xasprintf(
    b"/var/lib/dpkg/info/%s.%s\x00" as *const u8 as *const libc::c_char,
    package_name,
    control_name,
  );
  fp = crate::libbb::wfopen::fopen_for_read(filename);
  free(filename as *mut libc::c_void);
  if !fp.is_null() {
    loop {
      line = crate::libbb::get_line_from_file::xmalloc_fgetline(fp);
      if line.is_null() {
        break;
      }
      crate::libbb::llist::llist_add_to(ll, line as *mut libc::c_void);
    }
    fclose(fp);
  };
}
unsafe extern "C" fn filter_rename_config(
  mut archive_handle: *mut archive_handle_t,
) -> libc::c_char {
  let mut fd: libc::c_int = 0;
  let mut name_ptr: *mut libc::c_char = (*(*archive_handle).file_header).name.offset(1);
  /* Is this file marked as config file? */
  if crate::archival::libarchive::find_list_entry::find_list_entry(
    (*archive_handle).accept,
    name_ptr,
  )
  .is_null()
  {
    return 0 as libc::c_char;
  } /* no */
  fd = open(name_ptr, 0);
  if fd >= 0 {
    let mut md5: md5_ctx_t = md5_ctx_t {
      wbuffer: [0; 64],
      process_block: None,
      total64: 0,
      hash: [0; 8],
    };
    let mut md5line: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut buf: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut count: libc::c_int = 0;
    /* Calculate MD5 of existing file */
    buf = xmalloc(4096i32 as size_t) as *mut libc::c_char; /* using buf as result storage */
    crate::libbb::hash_md5_sha::md5_begin(&mut md5);
    loop {
      count = crate::libbb::read::safe_read(fd, buf as *mut libc::c_void, 4096i32 as size_t)
        as libc::c_int;
      if !(count > 0) {
        break;
      }
      crate::libbb::hash_md5_sha::md5_hash(&mut md5, buf as *const libc::c_void, count as size_t);
    }
    crate::libbb::hash_md5_sha::md5_end(&mut md5, buf as *mut libc::c_void);
    close(fd);
    md5line = xmalloc(
      ((16i32 * 2i32 + 2i32) as libc::c_ulong)
        .wrapping_add(strlen(name_ptr))
        .wrapping_add(1i32 as libc::c_ulong),
    ) as *mut libc::c_char;
    sprintf(
      crate::libbb::xfuncs::bin2hex(md5line, buf, 16i32),
      b"  %s\x00" as *const u8 as *const libc::c_char,
      name_ptr,
    );
    free(buf as *mut libc::c_void);
    /* Is it changed after install? */
    if crate::archival::libarchive::find_list_entry::find_list_entry(
      (*archive_handle).accept,
      md5line,
    )
    .is_null()
    {
      printf(
        b"Warning: Creating %s as %s.dpkg-new\n\x00" as *const u8 as *const libc::c_char,
        name_ptr,
        name_ptr,
      );
      (*(*archive_handle).file_header).name = crate::libbb::xfuncs_printf::xasprintf(
        b"%s.dpkg-new\x00" as *const u8 as *const libc::c_char,
        (*(*archive_handle).file_header).name,
      )
    }
    free(md5line as *mut libc::c_void);
  }
  return 0 as libc::c_char;
}
unsafe extern "C" fn data_extract_all_prefix(mut archive_handle: *mut archive_handle_t) {
  let mut name_ptr: *mut libc::c_char = (*(*archive_handle).file_header).name;
  /* Skip all leading "/" */
  while *name_ptr as libc::c_int == '/' as i32 {
    name_ptr = name_ptr.offset(1)
  }
  /* Skip all leading "./" and "../" */
  while *name_ptr.offset(0) as libc::c_int == '.' as i32 {
    if *name_ptr.offset(1) as libc::c_int == '.' as i32 {
      name_ptr = name_ptr.offset(1)
    }
    if *name_ptr.offset(1) as libc::c_int != '/' as i32 {
      break;
    }
    name_ptr = name_ptr.offset(2)
  }
  if *name_ptr.offset(0) as libc::c_int != '\u{0}' as i32 {
    (*(*archive_handle).file_header).name = crate::libbb::xfuncs_printf::xasprintf(
      b"%s%s\x00" as *const u8 as *const libc::c_char,
      (*archive_handle).dpkg__buffer,
      name_ptr,
    );
    crate::archival::libarchive::data_extract_all::data_extract_all(archive_handle);
    if fnmatch(
      b"*.dpkg-new\x00" as *const u8 as *const libc::c_char,
      (*(*archive_handle).file_header).name,
      0,
    ) == 0
    {
      /* remove .dpkg-new suffix */
      *(*(*archive_handle).file_header).name.offset(
        strlen((*(*archive_handle).file_header).name).wrapping_sub(9i32 as libc::c_ulong) as isize,
      ) = '\u{0}' as i32 as libc::c_char
    }
  };
}
unsafe extern "C" fn unpack_package(mut deb_file: *mut deb_file_t) {
  let mut package_name: *const libc::c_char = (*ptr_to_globals).name_hashtable
    [(*(*ptr_to_globals).package_hashtable[(*deb_file).package() as usize]).name() as usize];
  let status_num: libc::c_uint = search_status_hashtable(package_name);
  let status_package_num: libc::c_uint =
    (*(*ptr_to_globals).status_hashtable[status_num as usize]).package();
  let mut info_prefix: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut list_filename: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut archive_handle: *mut archive_handle_t = std::ptr::null_mut();
  let mut out_stream: *mut FILE = std::ptr::null_mut();
  let mut accept_list: *mut llist_t = std::ptr::null_mut();
  let mut conffile_list: *mut llist_t = std::ptr::null_mut();
  let mut i: libc::c_int = 0;
  /* If existing version, remove it first */
  conffile_list = std::ptr::null_mut();
  if strcmp(
    (*ptr_to_globals).name_hashtable[get_status(status_num, 3i32) as usize],
    b"installed\x00" as *const u8 as *const libc::c_char,
  ) == 0
  {
    /* Package is already installed, remove old version first */
    printf(
      b"Preparing to replace %s %s (using %s)...\n\x00" as *const u8 as *const libc::c_char,
      package_name,
      (*ptr_to_globals).name_hashtable
        [(*(*ptr_to_globals).package_hashtable[status_package_num as usize]).version() as usize],
      (*deb_file).filename,
    );
    /* Read md5sums from old package */
    if option_mask32 & OPT_force_confold as libc::c_int as libc::c_uint == 0 {
      append_control_file_to_llist(
        package_name,
        b"md5sums\x00" as *const u8 as *const libc::c_char,
        &mut conffile_list,
      );
    }
    remove_package(status_package_num, 0);
  } else {
    printf(
      b"Unpacking %s (from %s)...\n\x00" as *const u8 as *const libc::c_char,
      package_name,
      (*deb_file).filename,
    );
  }
  /* Extract control.tar.gz to /var/lib/dpkg/info/<package>.filename */
  info_prefix = crate::libbb::xfuncs_printf::xasprintf(
    b"/var/lib/dpkg/info/%s.%s\x00" as *const u8 as *const libc::c_char,
    package_name,
    b"\x00" as *const u8 as *const libc::c_char,
  );
  archive_handle = init_archive_deb_ar((*deb_file).filename);
  init_archive_deb_control(archive_handle);
  accept_list = std::ptr::null_mut();
  i = 0;
  while (i as libc::c_uint)
    < (::std::mem::size_of::<[*const libc::c_char; 10]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
      as libc::c_uint
  {
    let mut c: *mut libc::c_char = crate::libbb::xfuncs_printf::xasprintf(
      b"./%s\x00" as *const u8 as *const libc::c_char,
      all_control_files[i as usize],
    );
    crate::libbb::llist::llist_add_to(&mut accept_list, c as *mut libc::c_void);
    i += 1
  }
  (*(*archive_handle).dpkg__sub_archive).accept = accept_list;
  (*(*archive_handle).dpkg__sub_archive).filter = Some(
    crate::archival::libarchive::filter_accept_list::filter_accept_list
      as unsafe extern "C" fn(_: *mut archive_handle_t) -> libc::c_char,
  );
  (*(*archive_handle).dpkg__sub_archive).action_data =
    Some(data_extract_all_prefix as unsafe extern "C" fn(_: *mut archive_handle_t) -> ());
  (*(*archive_handle).dpkg__sub_archive).dpkg__buffer = info_prefix;
  (*(*archive_handle).dpkg__sub_archive).ah_flags |= (1i32 << 2i32) as libc::c_uint;
  crate::archival::libarchive::unpack_ar_archive::unpack_ar_archive(archive_handle);
  /* Run the preinst prior to extracting */
  run_package_script_or_die(
    package_name,
    b"preinst\x00" as *const u8 as *const libc::c_char,
  );
  /* Don't overwrite existing config files */
  if option_mask32 & OPT_force_confnew as libc::c_int as libc::c_uint == 0 {
    append_control_file_to_llist(
      package_name,
      b"conffiles\x00" as *const u8 as *const libc::c_char,
      &mut conffile_list,
    );
  }
  /* Extract data.tar.gz to the root directory */
  archive_handle = init_archive_deb_ar((*deb_file).filename);
  init_archive_deb_data(archive_handle);
  (*(*archive_handle).dpkg__sub_archive).accept = conffile_list;
  /* Why ARCHIVE_REMEMBER_NAMES?
   * We want names collected in ->passed list even if conffile_list
   * is NULL (otherwise get_header_tar may optimize name saving out):
   */
  (*(*archive_handle).dpkg__sub_archive).ah_flags |= (1i32 << 8i32 | 1i32 << 2i32) as libc::c_uint; /* huh? */
  (*(*archive_handle).dpkg__sub_archive).filter =
    Some(filter_rename_config as unsafe extern "C" fn(_: *mut archive_handle_t) -> libc::c_char);
  (*(*archive_handle).dpkg__sub_archive).action_data =
    Some(data_extract_all_prefix as unsafe extern "C" fn(_: *mut archive_handle_t) -> ());
  (*(*archive_handle).dpkg__sub_archive).dpkg__buffer =
    b"/\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
  crate::archival::libarchive::unpack_ar_archive::unpack_ar_archive(archive_handle);
  /* Create the list file */
  list_filename = crate::libbb::xfuncs_printf::xasprintf(
    b"/var/lib/dpkg/info/%s.%s\x00" as *const u8 as *const libc::c_char,
    package_name,
    b"list\x00" as *const u8 as *const libc::c_char,
  );
  out_stream = crate::libbb::wfopen::xfopen_for_write(list_filename);
  (*(*archive_handle).dpkg__sub_archive).passed =
    crate::libbb::llist::llist_rev((*(*archive_handle).dpkg__sub_archive).passed);
  while !(*(*archive_handle).dpkg__sub_archive).passed.is_null() {
    let mut filename: *mut libc::c_char =
      crate::libbb::llist::llist_pop(&mut (*(*archive_handle).dpkg__sub_archive).passed)
        as *mut libc::c_char;
    /* the leading . has been stripped by data_extract_all_prefix already */
    fprintf(
      out_stream,
      b"%s\n\x00" as *const u8 as *const libc::c_char,
      filename,
    );
    free(filename as *mut libc::c_void);
  }
  fclose(out_stream);
  /* change status */
  set_status(
    status_num,
    b"install\x00" as *const u8 as *const libc::c_char,
    1i32,
  );
  set_status(
    status_num,
    b"unpacked\x00" as *const u8 as *const libc::c_char,
    3i32,
  );
  free(info_prefix as *mut libc::c_void);
  free(list_filename as *mut libc::c_void);
}
unsafe extern "C" fn configure_package(mut deb_file: *mut deb_file_t) {
  let mut package_name: *const libc::c_char = (*ptr_to_globals).name_hashtable
    [(*(*ptr_to_globals).package_hashtable[(*deb_file).package() as usize]).name() as usize];
  let mut package_version: *const libc::c_char = (*ptr_to_globals).name_hashtable
    [(*(*ptr_to_globals).package_hashtable[(*deb_file).package() as usize]).version() as usize];
  let status_num: libc::c_int = search_status_hashtable(package_name) as libc::c_int;
  printf(
    b"Setting up %s (%s)...\n\x00" as *const u8 as *const libc::c_char,
    package_name,
    package_version,
  );
  /* Run the postinst script */
  /* TODO: handle failure gracefully */
  run_package_script_or_die(
    package_name,
    b"postinst\x00" as *const u8 as *const libc::c_char,
  );
  /* Change status to reflect success */
  set_status(
    status_num as libc::c_uint,
    b"install\x00" as *const u8 as *const libc::c_char,
    1i32,
  );
  set_status(
    status_num as libc::c_uint,
    b"installed\x00" as *const u8 as *const libc::c_char,
    3i32,
  );
}
#[no_mangle]
pub unsafe extern "C" fn dpkg_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut deb_file: *mut *mut deb_file_t = std::ptr::null_mut();
  let mut status_node: *mut status_node_t = std::ptr::null_mut();
  let mut str_f: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut opt: libc::c_int = 0;
  let mut package_num: libc::c_int = 0;
  let mut deb_count: libc::c_int = 0;
  let mut state_status: libc::c_int = 0;
  let mut status_num: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  static mut dpkg_longopts: [libc::c_char; 112] = [
    99, 111, 110, 102, 105, 103, 117, 114, 101, 0, 0, 67, 102, 111, 114, 99, 101, 0, 1, 70, 105,
    110, 115, 116, 97, 108, 108, 0, 0, 105, 108, 105, 115, 116, 0, 0, 108, 112, 117, 114, 103, 101,
    0, 0, 80, 114, 101, 109, 111, 118, 101, 0, 0, 114, 117, 110, 112, 97, 99, 107, 0, 0, 117, 102,
    111, 114, 99, 101, 45, 100, 101, 112, 101, 110, 100, 115, 0, 0, -1, 102, 111, 114, 99, 101, 45,
    99, 111, 110, 102, 110, 101, 119, 0, 0, -2, 102, 111, 114, 99, 101, 45, 99, 111, 110, 102, 111,
    108, 100, 0, 0, -3, 0,
  ];
  let ref mut fresh11 =
    *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
      as *mut *mut globals);
  *fresh11 = crate::libbb::xfuncs_printf::xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong)
    as *mut globals;
  asm!("" : : : "memory" : "volatile");
  opt = crate::libbb::getopt32::getopt32long(
    argv,
    b"CilPruF:\x00" as *const u8 as *const libc::c_char,
    dpkg_longopts.as_ptr(),
    &mut str_f as *mut *mut libc::c_char,
  ) as libc::c_int;
  argv = argv.offset(optind as isize);
  //if (opt & OPT_configure) ... // -C
  if opt & OPT_force as libc::c_int != 0 {
    // -F (--force in official dpkg)
    if strcmp(str_f, b"depends\x00" as *const u8 as *const libc::c_char) == 0 {
      opt |= OPT_force_ignore_depends as libc::c_int
    } else if strcmp(str_f, b"confnew\x00" as *const u8 as *const libc::c_char) == 0 {
      opt |= OPT_force_confnew as libc::c_int
    } else if strcmp(str_f, b"confold\x00" as *const u8 as *const libc::c_char) == 0 {
      opt |= OPT_force_confold as libc::c_int
    } else {
      crate::libbb::appletlib::bb_show_usage();
    }
    option_mask32 = opt as u32
  }
  //if (opt & OPT_install) ... // -i
  //if (opt & OPT_list_installed) ... // -l
  //if (opt & OPT_purge) ... // -P
  //if (opt & OPT_remove) ... // -r
  //if (opt & OPT_unpack) ... // -u (--unpack in official dpkg)
  if opt & OPTMASK_cmd as libc::c_int == 0
    || opt & OPTMASK_cmd as libc::c_int & (opt & OPTMASK_cmd as libc::c_int) - 1i32 != 0
  {
    /* more than one cmd */
    crate::libbb::appletlib::bb_show_usage();
  }
  /*	puts("(Reading database ... xxxxx files and directories installed.)"); */
  index_status_file(b"/var/lib/dpkg/status\x00" as *const u8 as *const libc::c_char);
  /* if the list action was given print the installed packages and exit */
  if opt & OPT_list_installed as libc::c_int != 0 {
    list_packages(*argv.offset(0)); /* param can be NULL */
    return 0;
  }
  /* Read arguments and store relevant info in structs */
  while !(*argv).is_null() {
    /* deb_count = nb_elem - 1 and we need nb_elem + 1 to allocate terminal node [NULL pointer] */
    deb_file = crate::libbb::xrealloc_vector::xrealloc_vector_helper(
      deb_file as *mut libc::c_void,
      ((::std::mem::size_of::<*mut deb_file_t>() as libc::c_ulong) << 8i32)
        .wrapping_add(2i32 as libc::c_ulong) as libc::c_uint,
      deb_count,
    ) as *mut *mut deb_file_t;
    let ref mut fresh12 = *deb_file.offset(deb_count as isize);
    *fresh12 =
      crate::libbb::xfuncs_printf::xzalloc(::std::mem::size_of::<deb_file_t>() as libc::c_ulong)
        as *mut deb_file_t;
    if opt & (OPT_install as libc::c_int | OPT_unpack as libc::c_int) != 0 {
      /* -i/-u: require filename */
      let mut archive_handle: *mut archive_handle_t = std::ptr::null_mut();
      let mut control_list: *mut llist_t = std::ptr::null_mut();
      /* Extract the control file */
      crate::libbb::llist::llist_add_to(
        &mut control_list,
        b"./control\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
          as *mut libc::c_void,
      );
      archive_handle = init_archive_deb_ar(*argv.offset(0));
      init_archive_deb_control(archive_handle);
      let ref mut fresh13 = (**deb_file.offset(deb_count as isize)).control_file;
      *fresh13 = deb_extract_control_file_to_buffer(archive_handle, control_list);
      if (**deb_file.offset(deb_count as isize))
        .control_file
        .is_null()
      {
        crate::libbb::verror_msg::bb_simple_error_msg_and_die(
          b"can\'t extract control file\x00" as *const u8 as *const libc::c_char,
        );
      }
      let ref mut fresh14 = (**deb_file.offset(deb_count as isize)).filename;
      *fresh14 = crate::libbb::xfuncs_printf::xstrdup(*argv.offset(0));
      package_num =
        fill_package_struct((**deb_file.offset(deb_count as isize)).control_file) as libc::c_int;
      if package_num == -1i32 {
        crate::libbb::verror_msg::bb_error_msg(
          b"invalid control file in %s\x00" as *const u8 as *const libc::c_char,
          *argv.offset(0),
        );
        argv = argv.offset(1);
        continue;
      } else {
        let ref mut fresh15 = **deb_file.offset(deb_count as isize);
        (*fresh15).set_package(package_num as libc::c_uint);
        /* Add the package to the status hashtable */
        if opt & (OPT_unpack as libc::c_int | OPT_install as libc::c_int) != 0 {
          /* Try and find a currently installed version of this package */
          status_num = search_status_hashtable(
            (*ptr_to_globals).name_hashtable[(*(*ptr_to_globals).package_hashtable
              [(**deb_file.offset(deb_count as isize)).package() as usize])
              .name() as usize],
          ) as libc::c_int;
          /* If no previous entry was found initialise a new entry */
          if (*ptr_to_globals).status_hashtable[status_num as usize].is_null()
            || (*(*ptr_to_globals).status_hashtable[status_num as usize]).status() as libc::c_int
              == 0
          {
            status_node = xmalloc(::std::mem::size_of::<status_node_t>() as libc::c_ulong)
              as *mut status_node_t;
            (*status_node).set_package((**deb_file.offset(deb_count as isize)).package());
            /* reinstreq isn't changed to "ok" until the package control info
             * is written to the status file*/
            (*status_node).set_status(search_name_hashtable(
              b"install reinstreq not-installed\x00" as *const u8 as *const libc::c_char,
            ) as libc::c_uint);
            (*ptr_to_globals).status_hashtable[status_num as usize] = status_node
          } else {
            set_status(
              status_num as libc::c_uint,
              b"install\x00" as *const u8 as *const libc::c_char,
              1i32,
            );
            set_status(
              status_num as libc::c_uint,
              b"reinstreq\x00" as *const u8 as *const libc::c_char,
              2i32,
            );
          }
        }
      }
    } else if opt
      & (OPT_configure as libc::c_int | OPT_purge as libc::c_int | OPT_remove as libc::c_int)
      != 0
    {
      /* -C/-p/-r: require package name */
      let ref mut fresh16 = **deb_file.offset(deb_count as isize);
      (*fresh16).set_package(search_package_hashtable(
        search_name_hashtable(*argv.offset(0)) as libc::c_uint,
        search_name_hashtable(b"ANY\x00" as *const u8 as *const libc::c_char) as libc::c_uint,
        VER_ANY as libc::c_int as libc::c_uint,
      ) as libc::c_uint);
      if (*ptr_to_globals).package_hashtable
        [(**deb_file.offset(deb_count as isize)).package() as usize]
        .is_null()
      {
        crate::libbb::verror_msg::bb_error_msg_and_die(
          b"package %s is uninstalled or unknown\x00" as *const u8 as *const libc::c_char,
          *argv.offset(0),
        );
      }
      package_num = (**deb_file.offset(deb_count as isize)).package() as libc::c_int;
      status_num = search_status_hashtable(
        (*ptr_to_globals).name_hashtable
          [(*(*ptr_to_globals).package_hashtable[package_num as usize]).name() as usize],
      ) as libc::c_int;
      state_status = get_status(status_num as libc::c_uint, 3i32) as libc::c_int;
      /* check package status is "installed" */
      if opt & OPT_remove as libc::c_int != 0 {
        if strcmp(
          (*ptr_to_globals).name_hashtable[state_status as usize],
          b"not-installed\x00" as *const u8 as *const libc::c_char,
        ) == 0
          || strcmp(
            (*ptr_to_globals).name_hashtable[state_status as usize],
            b"config-files\x00" as *const u8 as *const libc::c_char,
          ) == 0
        {
          crate::libbb::verror_msg::bb_error_msg_and_die(
            b"%s is already removed\x00" as *const u8 as *const libc::c_char,
            (*ptr_to_globals).name_hashtable
              [(*(*ptr_to_globals).package_hashtable[package_num as usize]).name() as usize],
          );
        }
        set_status(
          status_num as libc::c_uint,
          b"deinstall\x00" as *const u8 as *const libc::c_char,
          1i32,
        );
      } else if opt & OPT_purge as libc::c_int != 0 {
        /* if package status is "conf-files" then its ok */
        if strcmp(
          (*ptr_to_globals).name_hashtable[state_status as usize],
          b"not-installed\x00" as *const u8 as *const libc::c_char,
        ) == 0
        {
          crate::libbb::verror_msg::bb_error_msg_and_die(
            b"%s is already purged\x00" as *const u8 as *const libc::c_char,
            (*ptr_to_globals).name_hashtable
              [(*(*ptr_to_globals).package_hashtable[package_num as usize]).name() as usize],
          );
        }
        set_status(
          status_num as libc::c_uint,
          b"purge\x00" as *const u8 as *const libc::c_char,
          1i32,
        );
      }
    }
    deb_count += 1;
    argv = argv.offset(1)
  }
  if deb_count == 0 {
    crate::libbb::verror_msg::bb_simple_error_msg_and_die(
      b"no package files specified\x00" as *const u8 as *const libc::c_char,
    );
  }
  let ref mut fresh17 = *deb_file.offset(deb_count as isize);
  *fresh17 = std::ptr::null_mut();
  /* Check that the deb file arguments are installable */
  if opt & OPT_force_ignore_depends as libc::c_int == 0 {
    if check_deps(deb_file, 0) == 0 {
      crate::libbb::verror_msg::bb_simple_error_msg_and_die(
        b"dependency check failed\x00" as *const u8 as *const libc::c_char,
      );
    }
  }
  /* TODO: install or remove packages in the correct dependency order */
  i = 0;
  while i < deb_count {
    /* Remove or purge packages */
    if opt & OPT_remove as libc::c_int != 0 {
      remove_package((**deb_file.offset(i as isize)).package(), 1i32);
    } else if opt & OPT_purge as libc::c_int != 0 {
      purge_package((**deb_file.offset(i as isize)).package());
    } else if opt & OPT_unpack as libc::c_int != 0 {
      unpack_package(*deb_file.offset(i as isize));
    } else if opt & OPT_install as libc::c_int != 0 {
      unpack_package(*deb_file.offset(i as isize));
    /* package is configured in second pass below */
    } else if opt & OPT_configure as libc::c_int != 0 {
      configure_package(*deb_file.offset(i as isize));
    }
    i += 1
  }
  /* configure installed packages */
  if opt & OPT_install as libc::c_int != 0 {
    i = 0;
    while i < deb_count {
      configure_package(*deb_file.offset(i as isize));
      i += 1
    }
  }
  write_status_file(deb_file);
  return 0;
}
