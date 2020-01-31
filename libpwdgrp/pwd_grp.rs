use crate::libbb::ptr_to_globals::bb_errno;
use crate::libbb::skip_whitespace::skip_whitespace;
use crate::librb::size_t;
use libc;
use libc::fclose;
use libc::free;
use libc::gid_t;
use libc::group;
use libc::passwd;
use libc::strcmp;
use libc::uid_t;
use libc::FILE;
extern "C" {

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
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn setgroups(__n: size_t, __groups: *const gid_t) -> libc::c_int;

}

pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct passdb {
  pub filename: *const libc::c_char,
  pub def: [libc::c_char; 9],
  pub off: [u8; 9],
  pub numfields: u8,
  pub size_of: u8,
  pub fp: *mut FILE,
  pub malloced: *mut libc::c_char,
}

/* We avoid having big global data. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct statics {
  pub db: [passdb; 3],
  pub tokenize_end: *mut libc::c_char,
  pub string_size: libc::c_uint,
}

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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct const_passdb {
  pub filename: *const libc::c_char,
  pub def: [libc::c_char; 9],
  pub off: [u8; 9],
  pub numfields: u8,
  pub size_of: u8,
}

use crate::librb::spwd;

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
unsafe fn bb_strtol(
  mut arg: *const libc::c_char,
  mut endp: *mut *mut libc::c_char,
  mut base: libc::c_int,
) -> libc::c_long {
  return crate::libbb::bb_strtonum::bb_strtoll(arg, endp, base) as libc::c_long;
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
static mut ptr_to_statics: *mut statics = std::ptr::null_mut();
unsafe fn get_S() -> *mut statics {
  if ptr_to_statics.is_null() {
    ptr_to_statics =
      crate::libbb::xfuncs_printf::xzalloc(::std::mem::size_of::<statics>() as libc::c_ulong)
        as *mut statics;
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
unsafe fn tokenize(mut buffer: *mut libc::c_char, mut ch: libc::c_int) -> libc::c_int {
  let mut p: *mut libc::c_char = buffer;
  let mut s: *mut libc::c_char = p;
  let mut num_fields: libc::c_int = 0;
  loop {
    if ({
      let mut bb__isblank: libc::c_uchar = *s as libc::c_uchar;
      (bb__isblank as libc::c_int == ' ' as i32 || bb__isblank as libc::c_int == '\t' as i32)
        as libc::c_int
    }) != 0
    {
      crate::libbb::safe_strncpy::overlapping_strcpy(s, skip_whitespace(s));
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
        crate::libbb::safe_strncpy::overlapping_strcpy(p, end);
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
unsafe fn parse_common(
  mut fp: *mut FILE,
  mut db: *mut passdb,
  mut key: *const libc::c_char,
  mut field_pos: libc::c_int,
) -> *mut libc::c_char {
  let mut buf: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  loop {
    buf = crate::libbb::get_line_from_file::xmalloc_fgetline(fp);
    if buf.is_null() {
      break;
    }
    /* Skip empty lines, comment lines */
    if !(*buf.offset(0) as libc::c_int == '\u{0}' as i32
      || *buf.offset(0) as libc::c_int == '#' as i32)
    {
      if tokenize(buf, ':' as i32) != (*db).numfields as libc::c_int {
        /* number of fields is wrong */
        crate::libbb::verror_msg::bb_error_msg(
          b"%s: bad record\x00" as *const u8 as *const libc::c_char,
          (*db).filename,
        );
      } else if field_pos == -1i32 {
        /* no key specified: sequential read, return a record */
        break;
      } else if strcmp(
        key,
        crate::libbb::compare_string_array::nth_string(buf, field_pos),
      ) == 0
      {
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
    buf = crate::libbb::xfuncs_printf::xrealloc(
      buf as *mut libc::c_void,
      (*ptr_to_statics).string_size as size_t,
    ) as *mut libc::c_char
  }
  return buf;
}
unsafe fn parse_file(
  mut db: *mut passdb,
  mut key: *const libc::c_char,
  mut field_pos: libc::c_int,
) -> *mut libc::c_char {
  let mut buf: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut fp: *mut FILE = crate::libbb::wfopen::fopen_for_read((*db).filename);
  if !fp.is_null() {
    buf = parse_common(fp, db, key, field_pos);
    fclose(fp);
  }
  return buf;
}
/* Convert passwd/group/shadow file record in buffer to a struct */
unsafe fn convert_to_struct(
  mut db: *mut passdb,
  mut buffer: *mut libc::c_char,
  mut result: *mut libc::c_void,
) -> *mut libc::c_void {
  let mut def: *const libc::c_char = (*db).def.as_mut_ptr();
  let mut off: *const u8 = (*db).off.as_mut_ptr();
  /* For consistency, zero out all fields */
  memset(result, 0, (*db).size_of as libc::c_ulong);
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
        crate::libbb::bb_strtonum::bb_strtou(buffer, 0 as *mut *mut libc::c_char, 10i32)
          as libc::c_int
    }
    if *def as libc::c_int == 'l' as i32 {
      let mut n: libc::c_long = -1i32 as libc::c_long;
      if *buffer.offset(0) != 0 {
        n = bb_strtol(buffer, 0 as *mut *mut libc::c_char, 10i32)
      }
      *(member as *mut libc::c_long) = n
    }
    if *def as libc::c_int == 'm' as i32 {
      let mut members: *mut *mut libc::c_char = std::ptr::null_mut();
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
        if !(i >= 0) {
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
      *members = std::ptr::null_mut::<libc::c_char>()
    }
    /* def "r" does nothing */
    def = def.offset(1);
    if *def as libc::c_uchar as libc::c_int <= ' ' as i32 as libc::c_uchar as libc::c_int {
      break;
    }
    buffer = buffer.offset(strlen(buffer).wrapping_add(1i32 as libc::c_ulong) as isize)
  }
  if *bb_errno != 0 {
    result = std::ptr::null_mut()
  }
  return result;
}
unsafe fn massage_data_for_r_func(
  mut db: *mut passdb,
  mut buffer: *mut libc::c_char,
  mut buflen: size_t,
  mut result: *mut *mut libc::c_void,
  mut buf: *mut libc::c_char,
) -> libc::c_int {
  let mut result_buf: *mut libc::c_void = *result;
  *result = std::ptr::null_mut();
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
unsafe fn massage_data_for_non_r_func(
  mut db: *mut passdb,
  mut buf: *mut libc::c_char,
) -> *mut libc::c_void {
  if buf.is_null() {
    return std::ptr::null_mut();
  }
  free((*db).malloced as *mut libc::c_void);
  /* We enlarge buf and move string data up, freeing space
   * for struct passwd/group/spwd at the beginning. This way,
   * entire result of getXXnam is in a single malloced block.
   * This enables easy creation of xmalloc_getpwnam() API.
   */
  buf = crate::libbb::xfuncs_printf::xrealloc(
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
unsafe fn getXXnam_r(
  mut name: *const libc::c_char,
  mut db_and_field_pos: uintptr_t,
  mut buffer: *mut libc::c_char,
  mut buflen: size_t,
  mut result: *mut libc::c_void,
) -> libc::c_int {
  let mut buf: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut db: *mut passdb = &mut *(*(get_S as unsafe fn() -> *mut statics)())
    .db
    .as_mut_ptr()
    .offset((db_and_field_pos >> 2i32) as isize) as *mut passdb;
  buf = parse_file(db, name, 0);
  /* "db_and_field_pos & 3" is commented out since so far we don't implement
   * getXXXid_r() functions which would use that to pass 2 here */
  return massage_data_for_r_func(db, buffer, buflen, result as *mut *mut libc::c_void, buf);
}
/* Reentrant versions of some of the functions above. */
pub unsafe fn bb_internal_getpwnam_r(
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
    ((0i32 << 2i32) + 0) as uintptr_t,
    buffer,
    buflen,
    result as *mut libc::c_void,
  );
}

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
pub unsafe fn bb_internal_getspnam_r(
  mut name: *const libc::c_char,
  mut struct_buf: *mut spwd,
  mut buffer: *mut libc::c_char,
  mut buflen: size_t,
  mut result: *mut *mut spwd,
) -> libc::c_int {
  *result = struct_buf;
  return getXXnam_r(
    name,
    ((2i32 << 2i32) + 0) as uintptr_t,
    buffer,
    buflen,
    result as *mut libc::c_void,
  );
}
/* ***** getXXent */
unsafe fn getXXent(mut db_idx: uintptr_t) -> *mut libc::c_void {
  let mut buf: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut db: *mut passdb = &mut *(*(get_S as unsafe fn() -> *mut statics)())
    .db
    .as_mut_ptr()
    .offset(db_idx as isize) as *mut passdb;
  if (*db).fp.is_null() {
    (*db).fp = crate::libbb::wfopen::fopen_for_read((*db).filename);
    if (*db).fp.is_null() {
      return std::ptr::null_mut();
    }
    crate::libbb::xfuncs::close_on_exec_on(fileno_unlocked((*db).fp));
  }
  buf = parse_common((*db).fp, db, 0 as *const libc::c_char, -1i32);
  return massage_data_for_non_r_func(db, buf);
}
pub unsafe fn bb_internal_getpwent() -> *mut passwd {
  return getXXent(0i32 as uintptr_t) as *mut passwd;
}
/* ***** getXXnam/id */
unsafe fn getXXnam(
  mut name: *const libc::c_char,
  mut db_and_field_pos: libc::c_uint,
) -> *mut libc::c_void {
  let mut buf: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut db: *mut passdb = &mut *(*(get_S as unsafe fn() -> *mut statics)())
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
pub unsafe fn bb_internal_getpwnam(mut name: *const libc::c_char) -> *mut passwd {
  return getXXnam(name, ((0i32 << 2i32) + 0) as libc::c_uint) as *mut passwd;
}
pub unsafe fn bb_internal_getgrnam(mut name: *const libc::c_char) -> *mut group {
  return getXXnam(name, ((1i32 << 2i32) + 0) as libc::c_uint) as *mut group;
}
/* Read an entry from the password-file stream, opening it if necessary.  */
/* Search for an entry with a matching user ID.  */
pub unsafe fn bb_internal_getpwuid(mut id: uid_t) -> *mut passwd {
  return getXXnam(
    crate::libbb::xfuncs::utoa(id),
    ((0i32 << 2i32) + 2i32) as libc::c_uint,
  ) as *mut passwd;
}
pub unsafe fn bb_internal_getgrgid(mut id: gid_t) -> *mut group {
  return getXXnam(
    crate::libbb::xfuncs::utoa(id),
    ((1i32 << 2i32) + 2i32) as libc::c_uint,
  ) as *mut group;
}
/* Close the password-file stream.  */
/* ***** end/setXXend */
pub unsafe fn bb_internal_endpwent() {
  if !ptr_to_statics.is_null() && !(*ptr_to_statics).db[0].fp.is_null() {
    fclose((*ptr_to_statics).db[0].fp);
    (*ptr_to_statics).db[0].fp = std::ptr::null_mut()
  };
}

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
pub unsafe fn bb_internal_setpwent() {
  if !ptr_to_statics.is_null() && !(*ptr_to_statics).db[0].fp.is_null() {
    rewind((*ptr_to_statics).db[0].fp);
  };
}
pub unsafe fn bb_internal_endgrent() {
  if !ptr_to_statics.is_null() && !(*ptr_to_statics).db[1].fp.is_null() {
    fclose((*ptr_to_statics).db[1].fp);
    (*ptr_to_statics).db[1].fp = std::ptr::null_mut()
  };
}
/* ***** initgroups and getgrouplist */
unsafe fn getgrouplist_internal(
  mut ngroups_ptr: *mut libc::c_int,
  mut user: *const libc::c_char,
  mut gid: gid_t,
) -> *mut gid_t {
  let mut fp: *mut FILE = std::ptr::null_mut();
  let mut group_list: *mut gid_t = std::ptr::null_mut();
  let mut ngroups: libc::c_int = 0;
  /* We alloc space for 8 gids at a time. */
  group_list = crate::libbb::xfuncs_printf::xzalloc(
    (8i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<gid_t>() as libc::c_ulong),
  ) as *mut gid_t;
  *group_list.offset(0) = gid;
  ngroups = 1i32;
  fp = crate::libbb::wfopen::fopen_for_read(b"/etc/group\x00" as *const u8 as *const libc::c_char);
  if !fp.is_null() {
    let mut db: *mut passdb = &mut *(*(get_S as unsafe fn() -> *mut statics)())
      .db
      .as_mut_ptr()
      .offset(1) as *mut passdb;
    let mut buf: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    loop {
      buf = parse_common(fp, db, 0 as *const libc::c_char, -1i32);
      if buf.is_null() {
        break;
      }
      let mut m: *mut *mut libc::c_char = std::ptr::null_mut();
      let mut group: group = std::mem::zeroed();
      if !convert_to_struct(db, buf, &mut group as *mut group as *mut libc::c_void).is_null() {
        if !(group.gr_gid == gid) {
          m = group.gr_mem;
          while !(*m).is_null() {
            if strcmp(*m, user) != 0 {
              m = m.offset(1)
            } else {
              group_list = crate::libbb::xrealloc_vector::xrealloc_vector_helper(
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
pub unsafe fn bb_internal_initgroups(mut user: *const libc::c_char, mut gid: gid_t) -> libc::c_int {
  let mut ngroups: libc::c_int = 0;
  let mut group_list: *mut gid_t = getgrouplist_internal(&mut ngroups, user, gid);
  ngroups = setgroups(ngroups as size_t, group_list);
  free(group_list as *mut libc::c_void);
  return ngroups;
}

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
pub unsafe fn bb_internal_getgrouplist(
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
unsafe fn run_static_initializers() {
  const_pw_db = {
    let mut init = const_passdb {
      filename: b"/etc/passwd\x00" as *const u8 as *const libc::c_char,
      def: *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"SsIIsss\x00\x00"),
      off: [
        0u64 as u8,
        8u64 as u8,
        16u64 as u8,
        20u64 as u8,
        24u64 as u8,
        32u64 as u8,
        40u64 as u8,
        0,
        0,
      ],
      numfields: (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong) as u8,
      size_of: ::std::mem::size_of::<passwd>() as libc::c_ulong as u8,
    };
    init
  };
  const_gr_db = {
    let mut init = const_passdb {
      filename: b"/etc/group\x00" as *const u8 as *const libc::c_char,
      def: *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"SsIm\x00\x00\x00\x00\x00"),
      off: [
        0u64 as u8,
        8u64 as u8,
        16u64 as u8,
        24u64 as u8,
        0,
        0,
        0,
        0,
        0,
      ],
      numfields: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong) as u8,
      size_of: ::std::mem::size_of::<group>() as libc::c_ulong as u8,
    };
    init
  };
  const_sp_db = {
    let mut init = const_passdb {
      filename: b"/etc/shadow\x00" as *const u8 as *const libc::c_char,
      def: *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"Ssllllllr"),
      off: [
        0u64 as u8,
        8u64 as u8,
        16u64 as u8,
        24u64 as u8,
        32u64 as u8,
        40u64 as u8,
        48u64 as u8,
        56u64 as u8,
        64u64 as u8,
      ],
      numfields: (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong) as u8,
      size_of: ::std::mem::size_of::<spwd>() as libc::c_ulong as u8,
    };
    init
  }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [run_static_initializers];
