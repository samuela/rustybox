use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn rewind(__stream: *mut FILE);
  #[no_mangle]
  fn fileno_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn setgroups(__n: size_t, __groups: *const __gid_t) -> libc::c_int;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn overlapping_strcpy(dst: *mut libc::c_char, src: *const libc::c_char);
  #[no_mangle]
  fn skip_whitespace(_: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_strtoll(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_longlong;
  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  #[no_mangle]
  fn xrealloc(old: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn nth_string(strings: *const libc::c_char, n: libc::c_int) -> *const libc::c_char;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn xmalloc_fgetline(file: *mut FILE) -> *mut libc::c_char;
  #[no_mangle]
  fn close_on_exec_on(fd: libc::c_int);
  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn utoa(n: libc::c_uint) -> *mut libc::c_char;
  #[no_mangle]
  fn xrealloc_vector_helper(
    vector: *mut libc::c_void,
    sizeof_and_shift: libc::c_uint,
    idx: libc::c_int,
  ) -> *mut libc::c_void;
}
pub type __uint8_t = libc::c_uchar;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
use crate::librb::uint8_t;
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
use crate::librb::size_t;
use crate::librb::gid_t;
use crate::librb::uid_t;



use crate::librb::FILE;
use crate::librb::passwd;
use crate::librb::group;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passdb {
  pub filename: *const libc::c_char,
  pub def: [libc::c_char; 9],
  pub off: [uint8_t; 9],
  pub numfields: uint8_t,
  pub size_of: uint8_t,
  pub fp: *mut FILE,
  pub malloced: *mut libc::c_char,
}
/* We avoid having big global data. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statics {
  pub db: [passdb; 3],
  pub tokenize_end: *mut libc::c_char,
  pub string_size: libc::c_uint,
}
/* vi: set sw=4 ts=4: */
/*
 * Copyright (C) 2014 Tito Ragusa <farmatito@tiscali.it>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY!!
 *
 * Rewrite of some parts. Main differences are:
 *
 * 1) the buffer for getpwuid, getgrgid, getpwnam, getgrnam is dynamically
 *    allocated.
 *    If ENABLE_FEATURE_CLEAN_UP is set the buffers are freed at program
 *    exit using the atexit function to make valgrind happy.
 * 2) the passwd/group files:
 *      a) must contain the expected number of fields (as per count of field
 *         delimiters ":") or we will complain with a error message.
 *      b) leading and trailing whitespace in fields is stripped.
 *      c) some fields are not allowed to be empty (e.g. username, uid/gid),
 *         and in this case NULL is returned and errno is set to EINVAL.
 *         This behaviour could be easily changed by modifying PW_DEF, GR_DEF,
 *         SP_DEF strings (uppercase makes a field mandatory).
 *      d) the string representing uid/gid must be convertible by strtoXX
 *         functions, or errno is set to EINVAL.
 *      e) leading and trailing whitespace in group member names is stripped.
 * 3) the internal function for getgrouplist uses dynamically allocated buffer.
 * 4) at the moment only the functions really used by busybox code are
 *    implemented, if you need a particular missing function it should be
 *    easy to write it by using the internal common code.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct const_passdb {
  pub filename: *const libc::c_char,
  pub def: [libc::c_char; 9],
  pub off: [uint8_t; 9],
  pub numfields: uint8_t,
  pub size_of: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spwd {
  pub sp_namp: *mut libc::c_char,
  pub sp_pwdp: *mut libc::c_char,
  pub sp_lstchg: libc::c_long,
  pub sp_min: libc::c_long,
  pub sp_max: libc::c_long,
  pub sp_warn: libc::c_long,
  pub sp_inact: libc::c_long,
  pub sp_expire: libc::c_long,
  pub sp_flag: libc::c_ulong,
}
/* vi: set sw=4 ts=4: */
/*
 * ascii-to-numbers implementations for busybox
 *
 * Copyright (C) 2003  Manuel Novoa III  <mjn3@codepoet.org>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* Provides extern declarations of functions */
/* Unsigned long long functions always exist */
/* Provides inline definitions of functions */
/* (useful for mapping them to the type of the same width) */
/* If long == long long, then just map them one-to-one */
/* Same for int -> [long] long */
/* Specialized */
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
#[inline(always)]
unsafe extern "C" fn bb_strtol(
  mut arg: *const libc::c_char,
  mut endp: *mut *mut libc::c_char,
  mut base: libc::c_int,
) -> libc::c_long {
  return bb_strtoll(arg, endp, base) as libc::c_long;
}
// Initialized in run_static_initializers
static mut const_pw_db: const_passdb = const_passdb {
  filename: 0 as *const libc::c_char,
  def: [0; 9],
  off: [0; 9],
  numfields: 0,
  size_of: 0,
};
// Initialized in run_static_initializers
static mut const_gr_db: const_passdb = const_passdb {
  filename: 0 as *const libc::c_char,
  def: [0; 9],
  off: [0; 9],
  numfields: 0,
  size_of: 0,
};
// Initialized in run_static_initializers
static mut const_sp_db: const_passdb = const_passdb {
  filename: 0 as *const libc::c_char,
  def: [0; 9],
  off: [0; 9],
  numfields: 0,
  size_of: 0,
};
static mut ptr_to_statics: *mut statics = 0 as *const statics as *mut statics;
unsafe extern "C" fn get_S() -> *mut statics {
  if ptr_to_statics.is_null() {
    ptr_to_statics = xzalloc(::std::mem::size_of::<statics>() as libc::c_ulong) as *mut statics;
    memcpy(
      &mut *(*ptr_to_statics).db.as_mut_ptr().offset(0) as *mut passdb as *mut libc::c_void,
      &const_pw_db as *const const_passdb as *const libc::c_void,
      ::std::mem::size_of::<const_passdb>() as libc::c_ulong,
    );
    memcpy(
      &mut *(*ptr_to_statics).db.as_mut_ptr().offset(1) as *mut passdb as *mut libc::c_void,
      &const_gr_db as *const const_passdb as *const libc::c_void,
      ::std::mem::size_of::<const_passdb>() as libc::c_ulong,
    );
    memcpy(
      &mut *(*ptr_to_statics).db.as_mut_ptr().offset(2) as *mut passdb as *mut libc::c_void,
      &const_sp_db as *const const_passdb as *const libc::c_void,
      ::std::mem::size_of::<const_passdb>() as libc::c_ulong,
    );
  }
  return ptr_to_statics;
}
/* Internal functions */
/* Divide the passwd/group/shadow record in fields
 * by substituting the given delimiter
 * e.g. ':' or ',' with '\0'.
 * Returns the number of fields found.
 * Strips leading and trailing whitespace in fields.
 */
unsafe extern "C" fn tokenize(mut buffer: *mut libc::c_char, mut ch: libc::c_int) -> libc::c_int {
  let mut p: *mut libc::c_char = buffer;
  let mut s: *mut libc::c_char = p;
  let mut num_fields: libc::c_int = 0i32;
  loop {
    if ({
      let mut bb__isblank: libc::c_uchar = *s as libc::c_uchar;
      (bb__isblank as libc::c_int == ' ' as i32 || bb__isblank as libc::c_int == '\t' as i32)
        as libc::c_int
    }) != 0
    {
      overlapping_strcpy(s, skip_whitespace(s));
    }
    if *p as libc::c_int == ch || *p as libc::c_int == '\u{0}' as i32 {
      let mut end: *mut libc::c_char = p;
      while p != s
        && ({
          let mut bb__isblank: libc::c_uchar = *p.offset(-1i32 as isize) as libc::c_uchar;
          (bb__isblank as libc::c_int == ' ' as i32 || bb__isblank as libc::c_int == '\t' as i32)
            as libc::c_int
        }) != 0
      {
        p = p.offset(-1)
      }
      if p != end {
        overlapping_strcpy(p, end);
      }
      num_fields += 1;
      if *end as libc::c_int == '\u{0}' as i32 {
        (*ptr_to_statics).tokenize_end = p.offset(1);
        return num_fields;
      }
      *p = '\u{0}' as i32 as libc::c_char;
      s = p.offset(1)
    }
    p = p.offset(1)
  }
}
/* Returns !NULL on success and matching line broken up in fields by '\0' in buf.
 * We require the expected number of fields to be found.
 */
unsafe extern "C" fn parse_common(
  mut fp: *mut FILE,
  mut db: *mut passdb,
  mut key: *const libc::c_char,
  mut field_pos: libc::c_int,
) -> *mut libc::c_char {
  let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
  loop {
    buf = xmalloc_fgetline(fp);
    if buf.is_null() {
      break;
    }
    /* Skip empty lines, comment lines */
    if !(*buf.offset(0) as libc::c_int == '\u{0}' as i32
      || *buf.offset(0) as libc::c_int == '#' as i32)
    {
      if tokenize(buf, ':' as i32) != (*db).numfields as libc::c_int {
        /* number of fields is wrong */
        bb_error_msg(
          b"%s: bad record\x00" as *const u8 as *const libc::c_char,
          (*db).filename,
        );
      } else if field_pos == -1i32 {
        /* no key specified: sequential read, return a record */
        break;
      } else if strcmp(key, nth_string(buf, field_pos)) == 0i32 {
        break;
      }
    }
    free(buf as *mut libc::c_void);
  }
  /* record found */
  (*ptr_to_statics).string_size =
    (*ptr_to_statics).tokenize_end.wrapping_offset_from(buf) as libc::c_long as libc::c_uint;
  /*
   * Ugly hack: group db requires additional buffer space
   * for members[] array. If there is only one group, we need space
   * for 3 pointers: alignment padding, group name, NULL.
   * +1 for every additional group.
   */
  if !buf.is_null()
    && (*db).numfields as libc::c_ulong
      == (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong)
  {
    /* if we read group file... */
    let mut cnt: libc::c_int = 3i32;
    let mut p: *mut libc::c_char = buf;
    while p < (*ptr_to_statics).tokenize_end {
      let fresh0 = p;
      p = p.offset(1);
      if *fresh0 as libc::c_int == ',' as i32 {
        cnt += 1
      }
    }
    (*ptr_to_statics).string_size = ((*ptr_to_statics).string_size as libc::c_ulong).wrapping_add(
      (cnt as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as libc::c_uint as libc::c_uint;
    //bb_error_msg("+%d words = %u key:%s buf:'%s'", cnt, S.string_size, key, buf);
    buf = xrealloc(
      buf as *mut libc::c_void,
      (*ptr_to_statics).string_size as size_t,
    ) as *mut libc::c_char
  }
  return buf;
}
unsafe extern "C" fn parse_file(
  mut db: *mut passdb,
  mut key: *const libc::c_char,
  mut field_pos: libc::c_int,
) -> *mut libc::c_char {
  let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut fp: *mut FILE = fopen_for_read((*db).filename);
  if !fp.is_null() {
    buf = parse_common(fp, db, key, field_pos);
    fclose(fp);
  }
  return buf;
}
/* Convert passwd/group/shadow file record in buffer to a struct */
unsafe extern "C" fn convert_to_struct(
  mut db: *mut passdb,
  mut buffer: *mut libc::c_char,
  mut result: *mut libc::c_void,
) -> *mut libc::c_void {
  let mut def: *const libc::c_char = (*db).def.as_mut_ptr();
  let mut off: *const uint8_t = (*db).off.as_mut_ptr();
  /* For consistency, zero out all fields */
  memset(result, 0i32, (*db).size_of as libc::c_ulong);
  loop {
    let fresh1 = off;
    off = off.offset(1);
    let mut member: *mut libc::c_void =
      (result as *mut libc::c_char).offset(*fresh1 as libc::c_int as isize) as *mut libc::c_void;
    if *def as libc::c_int | 0x20i32 == 's' as i32 {
      /* s or S */
      let ref mut fresh2 = *(member as *mut *mut libc::c_char);
      *fresh2 = buffer;
      if *buffer.offset(0) == 0 && *def as libc::c_int == 'S' as i32 {
        *bb_errno = 22i32
      }
    }
    if *def as libc::c_int == 'I' as i32 {
      *(member as *mut libc::c_int) =
        bb_strtou(buffer, 0 as *mut *mut libc::c_char, 10i32) as libc::c_int
    }
    if *def as libc::c_int == 'l' as i32 {
      let mut n: libc::c_long = -1i32 as libc::c_long;
      if *buffer.offset(0) != 0 {
        n = bb_strtol(buffer, 0 as *mut *mut libc::c_char, 10i32)
      }
      *(member as *mut libc::c_long) = n
    }
    if *def as libc::c_int == 'm' as i32 {
      let mut members: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
      let mut i: libc::c_int = tokenize(buffer, ',' as i32);
      /* Store members[] after buffer's end.
       * This is safe ONLY because there is a hack
       * in parse_common() which allocates additional space
       * at the end of malloced buffer!
       */
      members = (((*ptr_to_statics).tokenize_end as intptr_t as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
        & -(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong as intptr_t)
          as libc::c_ulong) as *mut *mut libc::c_char;
      let ref mut fresh3 = (*(result as *mut group)).gr_mem;
      *fresh3 = members;
      loop {
        i -= 1;
        if !(i >= 0i32) {
          break;
        }
        if *buffer.offset(0) != 0 {
          let fresh4 = members;
          members = members.offset(1);
          *fresh4 = buffer
          // bb_error_msg("member[]='%s'", buffer);
        }
        buffer = buffer.offset(strlen(buffer).wrapping_add(1i32 as libc::c_ulong) as isize)
      }
      *members = 0 as *mut libc::c_char
    }
    /* def "r" does nothing */
    def = def.offset(1);
    if *def as libc::c_uchar as libc::c_int <= ' ' as i32 as libc::c_uchar as libc::c_int {
      break;
    }
    buffer = buffer.offset(strlen(buffer).wrapping_add(1i32 as libc::c_ulong) as isize)
  }
  if *bb_errno != 0 {
    result = 0 as *mut libc::c_void
  }
  return result;
}
unsafe extern "C" fn massage_data_for_r_func(
  mut db: *mut passdb,
  mut buffer: *mut libc::c_char,
  mut buflen: size_t,
  mut result: *mut *mut libc::c_void,
  mut buf: *mut libc::c_char,
) -> libc::c_int {
  let mut result_buf: *mut libc::c_void = *result;
  *result = 0 as *mut libc::c_void;
  if !buf.is_null() {
    if (*ptr_to_statics).string_size as libc::c_ulong > buflen {
      *bb_errno = 34i32
    } else {
      memcpy(
        buffer as *mut libc::c_void,
        buf as *const libc::c_void,
        (*ptr_to_statics).string_size as libc::c_ulong,
      );
      *result = convert_to_struct(db, buffer, result_buf)
    }
    free(buf as *mut libc::c_void);
  }
  /* "The reentrant functions return zero on success.
   * In case of error, an error number is returned."
   * NB: not finding the record is also a "success" here:
   */
  return *bb_errno;
}
unsafe extern "C" fn massage_data_for_non_r_func(
  mut db: *mut passdb,
  mut buf: *mut libc::c_char,
) -> *mut libc::c_void {
  if buf.is_null() {
    return 0 as *mut libc::c_void;
  }
  free((*db).malloced as *mut libc::c_void);
  /* We enlarge buf and move string data up, freeing space
   * for struct passwd/group/spwd at the beginning. This way,
   * entire result of getXXnam is in a single malloced block.
   * This enables easy creation of xmalloc_getpwnam() API.
   */
  buf = xrealloc(
    buf as *mut libc::c_void,
    ((*db).size_of as libc::c_uint).wrapping_add((*ptr_to_statics).string_size) as size_t,
  ) as *mut libc::c_char;
  (*db).malloced = buf;
  memmove(
    buf.offset((*db).size_of as libc::c_int as isize) as *mut libc::c_void,
    buf as *const libc::c_void,
    (*ptr_to_statics).string_size as libc::c_ulong,
  );
  return convert_to_struct(
    db,
    buf.offset((*db).size_of as libc::c_int as isize),
    buf as *mut libc::c_void,
  );
}
/* ***** getXXnam/id_r */
unsafe extern "C" fn getXXnam_r(
  mut name: *const libc::c_char,
  mut db_and_field_pos: uintptr_t,
  mut buffer: *mut libc::c_char,
  mut buflen: size_t,
  mut result: *mut libc::c_void,
) -> libc::c_int {
  let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut db: *mut passdb = &mut *(*(get_S as unsafe extern "C" fn() -> *mut statics)())
    .db
    .as_mut_ptr()
    .offset((db_and_field_pos >> 2i32) as isize) as *mut passdb;
  buf = parse_file(db, name, 0i32);
  /* "db_and_field_pos & 3" is commented out since so far we don't implement
   * getXXXid_r() functions which would use that to pass 2 here */
  return massage_data_for_r_func(db, buffer, buflen, result as *mut *mut libc::c_void, buf);
}
/* Reentrant versions of some of the functions above. */
#[no_mangle]
pub unsafe extern "C" fn bb_internal_getpwnam_r(
  mut name: *const libc::c_char,
  mut struct_buf: *mut passwd,
  mut buffer: *mut libc::c_char,
  mut buflen: size_t,
  mut result: *mut *mut passwd,
) -> libc::c_int {
  /* Why the "store buffer address in result" trick?
   * This way, getXXnam_r has the same ABI signature as getpwnam_r,
   * hopefully compiler can optimize tail call better in this case.
   */
  *result = struct_buf;
  return getXXnam_r(
    name,
    ((0i32 << 2i32) + 0i32) as uintptr_t,
    buffer,
    buflen,
    result as *mut libc::c_void,
  );
}
/* vi: set sw=4 ts=4: */
/* Copyright (C) 1996, 1997, 1998, 1999 Free Software Foundation, Inc.
This file is part of the GNU C Library.

The GNU C Library is free software; you can redistribute it and/or
modify it under the terms of the GNU Lesser General Public
License as published by the Free Software Foundation; either
version 2.1 of the License, or (at your option) any later version.

The GNU C Library is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
Lesser General Public License for more details.

You should have received a copy of the GNU Lesser General Public
License along with the GNU C Library; if not, write to the Free
Software Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA
02111-1307 USA.  */
/* Declaration of types and functions for shadow password suite */
/* Structure of the password file */
/* Login name */
/* Encrypted password */
/* Date of last change */
/* Minimum number of days between changes */
/* Maximum number of days between changes */
/* Number of days to warn user to change the password */
/* Number of days the account may be inactive */
/* Number of days since 1970-01-01 until account expires */
/* Reserved */
/* All function names below should be remapped by #defines above
 * in order to not collide with libc names. */
#[no_mangle]
pub unsafe extern "C" fn bb_internal_getspnam_r(
  mut name: *const libc::c_char,
  mut struct_buf: *mut spwd,
  mut buffer: *mut libc::c_char,
  mut buflen: size_t,
  mut result: *mut *mut spwd,
) -> libc::c_int {
  *result = struct_buf;
  return getXXnam_r(
    name,
    ((2i32 << 2i32) + 0i32) as uintptr_t,
    buffer,
    buflen,
    result as *mut libc::c_void,
  );
}
/* ***** getXXent */
unsafe extern "C" fn getXXent(mut db_idx: uintptr_t) -> *mut libc::c_void {
  let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut db: *mut passdb = &mut *(*(get_S as unsafe extern "C" fn() -> *mut statics)())
    .db
    .as_mut_ptr()
    .offset(db_idx as isize) as *mut passdb;
  if (*db).fp.is_null() {
    (*db).fp = fopen_for_read((*db).filename);
    if (*db).fp.is_null() {
      return 0 as *mut libc::c_void;
    }
    close_on_exec_on(fileno_unlocked((*db).fp));
  }
  buf = parse_common((*db).fp, db, 0 as *const libc::c_char, -1i32);
  return massage_data_for_non_r_func(db, buf);
}
#[no_mangle]
pub unsafe extern "C" fn bb_internal_getpwent() -> *mut passwd {
  return getXXent(0i32 as uintptr_t) as *mut passwd;
}
/* ***** getXXnam/id */
unsafe extern "C" fn getXXnam(
  mut name: *const libc::c_char,
  mut db_and_field_pos: libc::c_uint,
) -> *mut libc::c_void {
  let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut db: *mut passdb = &mut *(*(get_S as unsafe extern "C" fn() -> *mut statics)())
    .db
    .as_mut_ptr()
    .offset((db_and_field_pos >> 2i32) as isize) as *mut passdb;
  buf = parse_file(
    db,
    name,
    (db_and_field_pos & 3i32 as libc::c_uint) as libc::c_int,
  );
  return massage_data_for_non_r_func(db, buf);
}
/* Search for an entry with a matching username.  */
#[no_mangle]
pub unsafe extern "C" fn bb_internal_getpwnam(mut name: *const libc::c_char) -> *mut passwd {
  return getXXnam(name, ((0i32 << 2i32) + 0i32) as libc::c_uint) as *mut passwd;
}
#[no_mangle]
pub unsafe extern "C" fn bb_internal_getgrnam(mut name: *const libc::c_char) -> *mut group {
  return getXXnam(name, ((1i32 << 2i32) + 0i32) as libc::c_uint) as *mut group;
}
/* Read an entry from the password-file stream, opening it if necessary.  */
/* Search for an entry with a matching user ID.  */
#[no_mangle]
pub unsafe extern "C" fn bb_internal_getpwuid(mut id: uid_t) -> *mut passwd {
  return getXXnam(utoa(id), ((0i32 << 2i32) + 2i32) as libc::c_uint) as *mut passwd;
}
#[no_mangle]
pub unsafe extern "C" fn bb_internal_getgrgid(mut id: gid_t) -> *mut group {
  return getXXnam(utoa(id), ((1i32 << 2i32) + 2i32) as libc::c_uint) as *mut group;
}
/* Close the password-file stream.  */
/* ***** end/setXXend */
#[no_mangle]
pub unsafe extern "C" fn bb_internal_endpwent() {
  if !ptr_to_statics.is_null() && !(*ptr_to_statics).db[0].fp.is_null() {
    fclose((*ptr_to_statics).db[0].fp);
    (*ptr_to_statics).db[0].fp = 0 as *mut FILE
  };
}
/* vi: set sw=4 ts=4: */
/* Copyright (C) 1991,92,95,96,97,98,99,2001 Free Software Foundation, Inc.
This file is part of the GNU C Library.

The GNU C Library is free software; you can redistribute it and/or
modify it under the terms of the GNU Lesser General Public
License as published by the Free Software Foundation; either
version 2.1 of the License, or (at your option) any later version.

The GNU C Library is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
Lesser General Public License for more details.

You should have received a copy of the GNU Lesser General Public
License along with the GNU C Library; if not, write to the Free
Software Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA
02111-1307 USA.  */
/*
 * POSIX Standard: 9.2.2 User Database Access	<pwd.h>
 */
/* This file is #included after #include <pwd.h>
 * We will use libc-defined structures, but will #define function names
 * so that function calls are directed to bb_internal_XXX replacements
 */
/* All function names below should be remapped by #defines above
 * in order to not collide with libc names. */
/* Rewind the password-file stream.  */
#[no_mangle]
pub unsafe extern "C" fn bb_internal_setpwent() {
  if !ptr_to_statics.is_null() && !(*ptr_to_statics).db[0].fp.is_null() {
    rewind((*ptr_to_statics).db[0].fp);
  };
}
#[no_mangle]
pub unsafe extern "C" fn bb_internal_endgrent() {
  if !ptr_to_statics.is_null() && !(*ptr_to_statics).db[1].fp.is_null() {
    fclose((*ptr_to_statics).db[1].fp);
    (*ptr_to_statics).db[1].fp = 0 as *mut FILE
  };
}
/* ***** initgroups and getgrouplist */
unsafe extern "C" fn getgrouplist_internal(
  mut ngroups_ptr: *mut libc::c_int,
  mut user: *const libc::c_char,
  mut gid: gid_t,
) -> *mut gid_t {
  let mut fp: *mut FILE = 0 as *mut FILE;
  let mut group_list: *mut gid_t = 0 as *mut gid_t;
  let mut ngroups: libc::c_int = 0;
  /* We alloc space for 8 gids at a time. */
  group_list =
    xzalloc((8i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<gid_t>() as libc::c_ulong))
      as *mut gid_t;
  *group_list.offset(0) = gid;
  ngroups = 1i32;
  fp = fopen_for_read(b"/etc/group\x00" as *const u8 as *const libc::c_char);
  if !fp.is_null() {
    let mut db: *mut passdb = &mut *(*(get_S as unsafe extern "C" fn() -> *mut statics)())
      .db
      .as_mut_ptr()
      .offset(1) as *mut passdb;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
      buf = parse_common(fp, db, 0 as *const libc::c_char, -1i32);
      if buf.is_null() {
        break;
      }
      let mut m: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
      let mut group: group = group {
        gr_name: 0 as *mut libc::c_char,
        gr_passwd: 0 as *mut libc::c_char,
        gr_gid: 0,
        gr_mem: 0 as *mut *mut libc::c_char,
      };
      if !convert_to_struct(db, buf, &mut group as *mut group as *mut libc::c_void).is_null() {
        if !(group.gr_gid == gid) {
          m = group.gr_mem;
          while !(*m).is_null() {
            if strcmp(*m, user) != 0i32 {
              m = m.offset(1)
            } else {
              group_list = xrealloc_vector_helper(
                group_list as *mut libc::c_void,
                ((::std::mem::size_of::<gid_t>() as libc::c_ulong) << 8i32)
                  .wrapping_add(3i32 as libc::c_ulong) as libc::c_uint,
                ngroups,
              ) as *mut gid_t;
              let fresh5 = ngroups;
              ngroups = ngroups + 1;
              *group_list.offset(fresh5 as isize) = group.gr_gid;
              break;
            }
          }
        }
      }
      free(buf as *mut libc::c_void);
    }
    fclose(fp);
  }
  *ngroups_ptr = ngroups;
  return group_list;
}
/* Initialize the group set for the current user
by reading the group database and using all groups
of which USER is a member.  Also include GROUP.  */
#[no_mangle]
pub unsafe extern "C" fn bb_internal_initgroups(
  mut user: *const libc::c_char,
  mut gid: gid_t,
) -> libc::c_int {
  let mut ngroups: libc::c_int = 0;
  let mut group_list: *mut gid_t = getgrouplist_internal(&mut ngroups, user, gid);
  ngroups = setgroups(ngroups as size_t, group_list);
  free(group_list as *mut libc::c_void);
  return ngroups;
}
/* vi: set sw=4 ts=4: */
/* Copyright (C) 1991,92,95,96,97,98,99,2000,01 Free Software Foundation, Inc.
  This file is part of the GNU C Library.

  The GNU C Library is free software; you can redistribute it and/or
  modify it under the terms of the GNU Lesser General Public
  License as published by the Free Software Foundation; either
  version 2.1 of the License, or (at your option) any later version.

  The GNU C Library is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
  Lesser General Public License for more details.

  You should have received a copy of the GNU Lesser General Public
  License along with the GNU C Library; if not, write to the Free
  Software Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA
  02111-1307 USA.
*/
/*
 * POSIX Standard: 9.2.1 Group Database Access	<grp.h>
 */
/* This file is #included after #include <grp.h>
 * We will use libc-defined structures, but will #define function names
 * so that function calls are directed to bb_internal_XXX replacements
 */
/* All function names below should be remapped by #defines above
 * in order to not collide with libc names. */
/* Close the group-file stream.  */
/* Search for an entry with a matching group ID.  */
/* Search for an entry with a matching group name.  */
/* Reentrant versions of some of the functions above. */
/* Store at most *NGROUPS members of the group set for USER into
*GROUPS.  Also include GROUP.  The actual number of groups found is
returned in *NGROUPS.  Return -1 if the if *NGROUPS is too small.  */
#[no_mangle]
pub unsafe extern "C" fn bb_internal_getgrouplist(
  mut user: *const libc::c_char,
  mut gid: gid_t,
  mut groups: *mut gid_t,
  mut ngroups: *mut libc::c_int,
) -> libc::c_int {
  let mut ngroups_old: libc::c_int = *ngroups;
  let mut group_list: *mut gid_t = getgrouplist_internal(ngroups, user, gid);
  if *ngroups <= ngroups_old {
    ngroups_old = *ngroups;
    memcpy(
      groups as *mut libc::c_void,
      group_list as *const libc::c_void,
      (ngroups_old as libc::c_ulong).wrapping_mul(::std::mem::size_of::<gid_t>() as libc::c_ulong),
    );
  } else {
    ngroups_old = -1i32
  }
  free(group_list as *mut libc::c_void);
  return ngroups_old;
}
unsafe extern "C" fn run_static_initializers() {
  const_pw_db = {
    let mut init = const_passdb {
      filename: b"/etc/passwd\x00" as *const u8 as *const libc::c_char,
      def: *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"SsIIsss\x00\x00"),
      off: [
        0u64 as uint8_t,
        8u64 as uint8_t,
        16u64 as uint8_t,
        20u64 as uint8_t,
        24u64 as uint8_t,
        32u64 as uint8_t,
        40u64 as uint8_t,
        0,
        0,
      ],
      numfields: (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong) as uint8_t,
      size_of: ::std::mem::size_of::<passwd>() as libc::c_ulong as uint8_t,
    };
    init
  };
  const_gr_db = {
    let mut init = const_passdb {
      filename: b"/etc/group\x00" as *const u8 as *const libc::c_char,
      def: *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"SsIm\x00\x00\x00\x00\x00"),
      off: [
        0u64 as uint8_t,
        8u64 as uint8_t,
        16u64 as uint8_t,
        24u64 as uint8_t,
        0,
        0,
        0,
        0,
        0,
      ],
      numfields: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong) as uint8_t,
      size_of: ::std::mem::size_of::<group>() as libc::c_ulong as uint8_t,
    };
    init
  };
  const_sp_db = {
    let mut init = const_passdb {
      filename: b"/etc/shadow\x00" as *const u8 as *const libc::c_char,
      def: *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"Ssllllllr"),
      off: [
        0u64 as uint8_t,
        8u64 as uint8_t,
        16u64 as uint8_t,
        24u64 as uint8_t,
        32u64 as uint8_t,
        40u64 as uint8_t,
        48u64 as uint8_t,
        56u64 as uint8_t,
        64u64 as uint8_t,
      ],
      numfields: (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong) as uint8_t,
      size_of: ::std::mem::size_of::<spwd>() as libc::c_ulong as uint8_t,
    };
    init
  }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
