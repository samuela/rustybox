use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn syscall(__sysno: libc::c_long, _: ...) -> libc::c_long;
  #[no_mangle]
  fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn strerror(_: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn strsep(__stringp: *mut *mut libc::c_char, __delim: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc_open_zipped_read_close(
    fname: *const libc::c_char,
    maxsz_p: *mut size_t,
  ) -> *mut libc::c_void;
  #[no_mangle]
  fn llist_add_to_end(list_head: *mut *mut llist_t, data: *mut libc::c_void);
}
use crate::libbb::llist::llist_t;
use crate::librb::size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct module_entry {
  pub next: *mut module_entry,
  pub name: *mut libc::c_char,
  pub modname: *mut libc::c_char,
  pub deps: *mut llist_t,
  pub realnames: *mut llist_t,
  pub flags: libc::c_uint,
  pub probed_name: *const libc::c_char,
  pub options: *mut libc::c_char,
  pub aliases: *mut llist_t,
  pub symbols: *mut llist_t,
  pub dnext: *mut module_entry,
  pub dprev: *mut module_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct module_db {
  pub buckets: [*mut module_entry; 256],
}
/*
 * Common modutils related functions for busybox
 *
 * Copyright (C) 2008 by Timo Teras <timo.teras@iki.fi>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
unsafe extern "C" fn helper_get_module(
  mut db: *mut module_db,
  mut module: *const libc::c_char,
  mut create: libc::c_int,
) -> *mut module_entry {
  let mut modname: [libc::c_char; 256] = [0; 256];
  let mut e: *mut module_entry = 0 as *mut module_entry;
  let mut i: libc::c_uint = 0;
  let mut hash: libc::c_uint = 0;
  filename2modname(module, modname.as_mut_ptr());
  hash = 0i32 as libc::c_uint;
  i = 0i32 as libc::c_uint;
  while modname[i as usize] != 0 {
    hash = (hash << 5i32)
      .wrapping_add(hash)
      .wrapping_add(modname[i as usize] as libc::c_uint);
    i = i.wrapping_add(1)
  }
  hash = hash.wrapping_rem(256i32 as libc::c_uint);
  e = (*db).buckets[hash as usize];
  while !e.is_null() {
    if strcmp((*e).modname, modname.as_mut_ptr()) == 0i32 {
      return e;
    }
    e = (*e).next
  }
  if create == 0 {
    return 0 as *mut module_entry;
  }
  e = xzalloc(::std::mem::size_of::<module_entry>() as libc::c_ulong) as *mut module_entry;
  (*e).modname = xstrdup(modname.as_mut_ptr());
  (*e).next = (*db).buckets[hash as usize];
  (*db).buckets[hash as usize] = e;
  (*e).dprev = e;
  (*e).dnext = (*e).dprev;
  return e;
}
#[no_mangle]
pub unsafe extern "C" fn moddb_get(
  mut db: *mut module_db,
  mut module: *const libc::c_char,
) -> *mut module_entry {
  return helper_get_module(db, module, 0i32);
}
#[no_mangle]
pub unsafe extern "C" fn moddb_get_or_create(
  mut db: *mut module_db,
  mut module: *const libc::c_char,
) -> *mut module_entry {
  return helper_get_module(db, module, 1i32);
}
#[no_mangle]
pub unsafe extern "C" fn moddb_free(mut db: *mut module_db) {
  let mut e: *mut module_entry = 0 as *mut module_entry;
  let mut n: *mut module_entry = 0 as *mut module_entry;
  let mut i: libc::c_uint = 0;
  i = 0i32 as libc::c_uint;
  while i < 256i32 as libc::c_uint {
    e = (*db).buckets[i as usize];
    while !e.is_null() {
      n = (*e).next;
      free((*e).name as *mut libc::c_void);
      free((*e).modname as *mut libc::c_void);
      free(e as *mut libc::c_void);
      e = n
    }
    i = i.wrapping_add(1)
  }
}
#[no_mangle]
pub unsafe extern "C" fn replace(
  mut s: *mut libc::c_char,
  mut what: libc::c_char,
  mut with: libc::c_char,
) {
  while *s != 0 {
    if what as libc::c_int == *s as libc::c_int {
      *s = with
    }
    s = s.offset(1)
  }
}
#[no_mangle]
pub unsafe extern "C" fn string_to_llist(
  mut string: *mut libc::c_char,
  mut llist: *mut *mut llist_t,
  mut delim: *const libc::c_char,
) -> libc::c_int {
  let mut tok: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut len: libc::c_int = 0i32;
  loop {
    tok = strsep(&mut string, delim);
    if tok.is_null() {
      break;
    }
    if *tok.offset(0) as libc::c_int == '\u{0}' as i32 {
      continue;
    }
    llist_add_to_end(llist, xstrdup(tok) as *mut libc::c_void);
    len = (len as libc::c_ulong).wrapping_add(strlen(tok)) as libc::c_int as libc::c_int
  }
  return len;
}
#[no_mangle]
pub unsafe extern "C" fn filename2modname(
  mut filename: *const libc::c_char,
  mut modname: *mut libc::c_char,
) -> *mut libc::c_char {
  let mut local_modname: [libc::c_char; 256] = [0; 256];
  let mut i: libc::c_int = 0;
  let mut from: *const libc::c_char = 0 as *const libc::c_char;
  if filename.is_null() {
    return 0 as *mut libc::c_char;
  }
  if modname.is_null() {
    modname = local_modname.as_mut_ptr()
  }
  // Disabled since otherwise "modprobe dir/name" would work
  // as if it is "modprobe name". It is unclear why
  // 'basenamization' was here in the first place.
  //from = bb_get_last_path_component_nostrip(filename);
  from = filename;
  i = 0i32;
  while i < 256i32 - 1i32
    && *from.offset(i as isize) as libc::c_int != '\u{0}' as i32
    && *from.offset(i as isize) as libc::c_int != '.' as i32
  {
    *modname.offset(i as isize) = if *from.offset(i as isize) as libc::c_int == '-' as i32 {
      '_' as i32
    } else {
      *from.offset(i as isize) as libc::c_int
    } as libc::c_char;
    i += 1
  }
  *modname.offset(i as isize) = '\u{0}' as i32 as libc::c_char;
  if modname == local_modname.as_mut_ptr() {
    return xstrdup(modname);
  }
  return modname;
}
#[no_mangle]
pub unsafe extern "C" fn parse_cmdline_module_options(
  mut argv: *mut *mut libc::c_char,
  mut quote_spaces: libc::c_int,
) -> *mut libc::c_char {
  let mut options: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut optlen: libc::c_int = 0;
  options = xzalloc(1i32 as size_t) as *mut libc::c_char;
  optlen = 0i32;
  loop {
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
    let mut fmt: *const libc::c_char = 0 as *const libc::c_char;
    let mut var: *const libc::c_char = 0 as *const libc::c_char;
    let mut val: *const libc::c_char = 0 as *const libc::c_char;
    var = *argv;
    options = xrealloc(
      options as *mut libc::c_void,
      ((optlen + 2i32) as libc::c_ulong)
        .wrapping_add(strlen(var))
        .wrapping_add(2i32 as libc::c_ulong),
    ) as *mut libc::c_char;
    fmt = b"%.*s%s \x00" as *const u8 as *const libc::c_char;
    val = strchrnul(var, '=' as i32);
    if quote_spaces != 0 {
      /*
       * modprobe (module-init-tools version 3.11.1) compat:
       * quote only value:
       * var="val with spaces", not "var=val with spaces"
       * (note: var *name* is not checked for spaces!)
       */
      if *val != 0 {
        /* has var=val format. skip '=' */
        val = val.offset(1);
        if !strchr(val, ' ' as i32).is_null() {
          fmt = b"%.*s\"%s\" \x00" as *const u8 as *const libc::c_char
        }
      }
    }
    optlen += sprintf(
      options.offset(optlen as isize),
      fmt,
      val.wrapping_offset_from(var) as libc::c_long as libc::c_int,
      var,
      val,
    )
  }
  /* Remove trailing space. Disabled */
  /* if (optlen != 0) options[optlen-1] = '\0'; */
  return options;
}
/* Return:
 * 0 on success,
 * -errno on open/read error,
 * errno on init_module() error
 */
#[no_mangle]
pub unsafe extern "C" fn bb_init_module(
  mut filename: *const libc::c_char,
  mut options: *const libc::c_char,
) -> libc::c_int {
  let mut image_size: size_t = 0;
  let mut image: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut rc: libc::c_int = 0;
  let mut mmaped: bool = false;
  if options.is_null() {
    options = b"\x00" as *const u8 as *const libc::c_char
  }
  //TODO: audit bb_init_module_24 to match error code convention
  /*
   * First we try finit_module if available.  Some kernels are configured
   * to only allow loading of modules off of secure storage (like a read-
   * only rootfs) which needs the finit_module call.  If it fails, we fall
   * back to normal module loading to support compressed modules.
   */
  let mut fd: libc::c_int = open(filename, 0i32 | 0o2000000i32); /* may be changed by e.g. open errors below */
  if fd >= 0i32 {
    rc =
      (syscall(313i32 as libc::c_long, fd, options, 0i32) !=0) as libc::c_int;
    close(fd);
    if rc == 0i32 {
      return rc;
    }
  }
  image_size = (2147483647i32 - 4095i32) as size_t;
  mmaped = 0i32 != 0;
  image = 0 as *mut libc::c_char;
  if !image.is_null() {
    mmaped = 1i32 != 0
  } else {
    *bb_errno = 12i32;
    image = xmalloc_open_zipped_read_close(filename, &mut image_size) as *mut libc::c_char;
    if image.is_null() {
      return -*bb_errno;
    }
  }
  *bb_errno = 0i32;
  syscall(175i32 as libc::c_long, image, image_size, options);
  rc = *bb_errno;
  if mmaped {
    munmap(image as *mut libc::c_void, image_size);
  } else {
    free(image as *mut libc::c_void);
  }
  return rc;
}
#[no_mangle]
pub unsafe extern "C" fn bb_delete_module(
  mut module: *const libc::c_char,
  mut flags: libc::c_uint,
) -> libc::c_int {
  *bb_errno = 0i32;
  syscall(176i32 as libc::c_long, module, flags);
  return *bb_errno;
}
/*
 * Common modutils related functions for busybox
 *
 * Copyright (C) 2008 by Timo Teras <timo.teras@iki.fi>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* linux/include/linux/module.h has 64, but this is also used
 * internally for the maximum alias name length, which can be quite long */
/* verbatim as seen on cmdline */
/* options from config files */
/* insmod for 2.4 and modprobe's options (insmod 2.6 has no options at all): */
/* (was meant to support -o NAME) , NULL */
//INSMOD_OPT_OUTPUTNAME = (1 << x) - not supported yet
/* Return:
 * 0 on success,
 * -errno on open/read error,
 * errno on init_module() error
 */
/* Return:
 * 0 on success,
 * errno on init_module() error
 */
/* Translates error return to a string */
/* Note: not suitable for delete_module() errnos.
 * For them, probably only EWOULDBLOCK needs explaining:
 * "Other modules depend on us". So far we don't do such
 * translation and don't use moderror() for removal errors.
 */
#[no_mangle]
pub unsafe extern "C" fn moderror(mut err: libc::c_int) -> *const libc::c_char {
  match err {
    -1 => {
      /* btw: it's -EPERM */
      return b"no such module\x00" as *const u8 as *const libc::c_char;
    }
    8 => return b"invalid module format\x00" as *const u8 as *const libc::c_char,
    2 => {
      return b"unknown symbol in module, or unknown parameter\x00" as *const u8
        as *const libc::c_char
    }
    3 => return b"module has wrong symbol version\x00" as *const u8 as *const libc::c_char,
    38 => {
      return b"kernel does not support requested operation\x00" as *const u8 as *const libc::c_char
    }
    _ => {}
  }
  if err < 0i32 {
    /* should always be */
    err = -err
  }
  return strerror(err);
}
