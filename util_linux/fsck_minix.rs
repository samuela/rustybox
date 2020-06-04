use crate::libbb::appletlib::applet_name;
use crate::libbb::default_error_retval::xfunc_error_retval;
use crate::libbb::xfuncs_printf::xmalloc;
use crate::librb::size_t;
use crate::librb::smallint;
use libc;
use libc::isatty;
use libc::off_t;
use libc::printf;
use libc::puts;
use libc::sprintf;
use libc::ssize_t;
use libc::strcmp;
use libc::sync;
use libc::termios;
extern "C" {
  #[no_mangle]
  fn exit(_: libc::c_int) -> !;

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn getchar_unlocked() -> libc::c_int;

  #[no_mangle]
  fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  static mut option_mask32: u32;

  #[no_mangle]
  static bb_banner: [libc::c_char; 0];

  #[no_mangle]
  static ptr_to_globals: *mut globals;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct globals {
  pub version2: smallint,
  pub changed: smallint,
  pub errors_uncorrected: smallint,
  pub termios_set: smallint,
  pub dirsize: smallint,
  pub namelen: smallint,
  pub device_name: *const libc::c_char,
  pub directory: libc::c_int,
  pub regular: libc::c_int,
  pub blockdev: libc::c_int,
  pub chardev: libc::c_int,
  pub links: libc::c_int,
  pub symlinks: libc::c_int,
  pub total: libc::c_int,
  pub inode_buffer: *mut libc::c_char,
  pub inode_map: *mut libc::c_char,
  pub zone_map: *mut libc::c_char,
  pub inode_count: *mut libc::c_uchar,
  pub zone_count: *mut libc::c_uchar,
  pub name_depth: libc::c_int,
  pub name_component: [*mut libc::c_char; 33],
  pub sv_termios: termios,
  pub u: C2RustUnnamed,
  pub add_zone_ind_blk: [libc::c_char; 1024],
  pub add_zone_dind_blk: [libc::c_char; 1024],
  pub add_zone_tind_blk: [libc::c_char; 1024],
  pub check_file_blk: [libc::c_char; 1024],
  pub current_name: [libc::c_char; 8160],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed {
  pub superblock_buffer: [libc::c_char; 1024],
  pub Super: minix_superblock,
}

/*
 * minix superblock data on disk
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct minix_superblock {
  pub s_ninodes: u16,
  pub s_nzones: u16,
  pub s_imap_blocks: u16,
  pub s_zmap_blocks: u16,
  pub s_firstdatazone: u16,
  pub s_log_zone_size: u16,
  pub s_max_size: u32,
  pub s_magic: u16,
  pub s_state: u16,
  pub s_zones: u32,
}

/*
 * This is the original minix inode layout on disk.
 * Note the 8-bit gid and atime and ctime.
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct minix1_inode {
  pub i_mode: u16,
  pub i_uid: u16,
  pub i_size: u32,
  pub i_time: u32,
  pub i_gid: u8,
  pub i_nlinks: u8,
  pub i_zone: [u16; 9],
}

/*
 * The new minix inode has all the time entries, as well as
 * long block numbers and a third indirect block (7+1+1+1
 * instead of 7+1+1). Also, some previously 8-bit values are
 * now 16-bit. The inode is now 64 bytes instead of 32.
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct minix2_inode {
  pub i_mode: u16,
  pub i_nlinks: u16,
  pub i_uid: u16,
  pub i_gid: u16,
  pub i_size: u32,
  pub i_atime: u32,
  pub i_mtime: u32,
  pub i_ctime: u32,
  pub i_zone: [u32; 10],
}

/* Believe it or not, but mount.h has this one #defined */
pub type C2RustUnnamed_0 = libc::c_uint;
pub const MINIX2_INODES_PER_BLOCK: C2RustUnnamed_0 = 16;
pub const MINIX1_INODES_PER_BLOCK: C2RustUnnamed_0 = 32;
// pub const INODE_SIZE2: C2RustUnnamed_0 = 64;
/* fs has errors */
// pub const INODE_SIZE1: C2RustUnnamed_0 = 32;
/* clean fs */
pub const MINIX_ERROR_FS: C2RustUnnamed_0 = 2;
/* minix V2 fs, 30 char names */
pub const MINIX_VALID_FS: C2RustUnnamed_0 = 1;
/* minix V2 fs */
pub const MINIX2_SUPER_MAGIC2: C2RustUnnamed_0 = 9336;
/* minix fs, 30 char names */
/* bionic has this define */
pub const MINIX2_SUPER_MAGIC: C2RustUnnamed_0 = 9320;
/* original minix fs */
pub const MINIX1_SUPER_MAGIC2: C2RustUnnamed_0 = 5007;
pub const MINIX1_SUPER_MAGIC: C2RustUnnamed_0 = 4991;
// pub const MINIX_BAD_INO: C2RustUnnamed_0 = 2;
pub const MINIX_ROOT_INO: C2RustUnnamed_0 = 1;
// pub const BITS_PER_BLOCK: C2RustUnnamed_0 = 8192;
pub const BLOCK_SIZE: C2RustUnnamed_0 = 1024;

pub type C2RustUnnamed_1 = libc::c_uint;
pub const MAX_DEPTH: C2RustUnnamed_1 = 32;

pub type C2RustUnnamed_2 = libc::c_uint;
pub const dev_fd: C2RustUnnamed_2 = 3;
/*current_name[1] = '\0';*/

pub type C2RustUnnamed_3 = libc::c_uint;
pub const OPT_f: C2RustUnnamed_3 = 64;
pub const OPT_w: C2RustUnnamed_3 = 32;
pub const OPT_s: C2RustUnnamed_3 = 16;
pub const OPT_v: C2RustUnnamed_3 = 8;
pub const OPT_r: C2RustUnnamed_3 = 4;
pub const OPT_a: C2RustUnnamed_3 = 2;
pub const OPT_l: C2RustUnnamed_3 = 1;

#[inline(always)]
unsafe fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}

/* non-automatic repairs requested? */
/* gcc likes this more (code is smaller) than macro variant */
#[inline(always)]
unsafe fn div_roundup(mut size: libc::c_uint, mut n: libc::c_uint) -> libc::c_uint {
  return size
    .wrapping_add(n)
    .wrapping_sub(1i32 as libc::c_uint)
    .wrapping_div(n);
}

/* Before you ask "where they come from?": */
/* setbit/clrbit are supplied by sys/param.h */
unsafe fn minix_bit(mut a: *const libc::c_char, mut i: libc::c_uint) -> libc::c_int {
  return *a.offset((i >> 3i32) as isize) as libc::c_int & 1i32 << (i & 7i32 as libc::c_uint);
}

unsafe fn minix_setbit(mut a: *mut libc::c_char, mut i: libc::c_uint) {
  let ref mut fresh0 = *a.offset(i.wrapping_div(8i32 as libc::c_uint) as isize);
  *fresh0 = (*fresh0 as libc::c_int | 1i32 << i.wrapping_rem(8i32 as libc::c_uint)) as libc::c_char;
  (*ptr_to_globals).changed = 1i32 as smallint;
}

unsafe fn minix_clrbit(mut a: *mut libc::c_char, mut i: libc::c_uint) {
  let ref mut fresh1 = *a.offset(i.wrapping_div(8i32 as libc::c_uint) as isize);
  *fresh1 =
    (*fresh1 as libc::c_int & !(1i32 << i.wrapping_rem(8i32 as libc::c_uint))) as libc::c_char;
  (*ptr_to_globals).changed = 1i32 as smallint;
}

unsafe fn die(mut str: *const libc::c_char) -> ! {
  if (*ptr_to_globals).termios_set != 0 {
    crate::libbb::xfuncs::tcsetattr_stdin_TCSANOW(&mut (*ptr_to_globals).sv_termios);
  }
  crate::libbb::verror_msg::bb_simple_error_msg_and_die(str);
}

unsafe fn push_filename(mut name: *const libc::c_char) {
  //  /dir/dir/dir/file
  //  ^   ^   ^
  // [0] [1] [2] <-name_component[i]
  if (*ptr_to_globals).name_depth < MAX_DEPTH as libc::c_int {
    let mut len: libc::c_int = 0; /* tolower */
    let mut p: *mut libc::c_char =
      (*ptr_to_globals).name_component[(*ptr_to_globals).name_depth as usize];
    let fresh2 = p;
    p = p.offset(1);
    *fresh2 = '/' as i32 as libc::c_char;
    len = sprintf(
      p,
      b"%.*s\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).namelen as libc::c_int,
      name,
    );
    (*ptr_to_globals).name_component[((*ptr_to_globals).name_depth + 1i32) as usize] =
      p.offset(len as isize)
  }
  (*ptr_to_globals).name_depth += 1;
}

unsafe fn pop_filename() {
  (*ptr_to_globals).name_depth -= 1;
  if (*ptr_to_globals).name_depth < MAX_DEPTH as libc::c_int {
    *(*ptr_to_globals).name_component[(*ptr_to_globals).name_depth as usize] =
      '\u{0}' as i32 as libc::c_char;
    if (*ptr_to_globals).name_depth == 0 {
      (*ptr_to_globals).current_name[0] = '/' as i32 as libc::c_char;
      (*ptr_to_globals).current_name[1] = '\u{0}' as i32 as libc::c_char
    }
  };
}

unsafe fn ask(mut string: *const libc::c_char, mut def: libc::c_int) -> libc::c_int {
  let mut c: libc::c_int = 0;
  if option_mask32 & OPT_r as libc::c_int as libc::c_uint == 0 {
    crate::libbb::xfuncs_printf::bb_putchar('\n' as i32);
    (*ptr_to_globals).errors_uncorrected = 1i32 as smallint;
    return 0;
  }
  if option_mask32 & OPT_a as libc::c_int as libc::c_uint != 0 {
    crate::libbb::xfuncs_printf::bb_putchar('\n' as i32);
    if def == 0 {
      (*ptr_to_globals).errors_uncorrected = 1i32 as smallint
    }
    return def;
  }
  printf(
    if def != 0 {
      b"%s (y/n)? \x00" as *const u8 as *const libc::c_char
    } else {
      b"%s (n/y)? \x00" as *const u8 as *const libc::c_char
    },
    string,
  );
  loop {
    crate::libbb::xfuncs_printf::fflush_all();
    c = getchar_unlocked();
    if c == -1i32 {
      if def == 0 {
        (*ptr_to_globals).errors_uncorrected = 1i32 as smallint
      }
      return def;
    }
    if c == '\n' as i32 {
      break;
    }
    c |= 0x20i32;
    if c == 'y' as i32 {
      def = 1i32;
      break;
    } else {
      if !(c == 'n' as i32) {
        continue;
      }
      def = 0;
      break;
    }
  }
  if def != 0 {
    puts(b"y\x00" as *const u8 as *const libc::c_char);
  } else {
    puts(b"n\x00" as *const u8 as *const libc::c_char);
    (*ptr_to_globals).errors_uncorrected = 1i32 as smallint
  }
  return def;
}

/*
 * Make certain that we aren't checking a filesystem that is on a
 * mounted partition.  Code adapted from e2fsck, Copyright (C) 1993,
 * 1994 Theodore Ts'o.  Also licensed under GPL.
 */
unsafe fn check_mount() {
  if !crate::libbb::find_mount_point::find_mount_point((*ptr_to_globals).device_name, 0).is_null() {
    let mut cont: libc::c_int = 0;
    printf(
      b"%s is mounted. \x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).device_name,
    );
    cont = 0;
    if isatty(0i32) != 0 && isatty(1i32) != 0 {
      cont = ask(
        b"Do you really want to continue\x00" as *const u8 as *const libc::c_char,
        0,
      )
    }
    if cont == 0 {
      puts(b"Check aborted\x00" as *const u8 as *const libc::c_char);
      exit(0i32);
    }
  };
}

/*
 * check_zone_nr checks to see that *nr is a valid zone nr. If it
 * isn't, it will possibly be repaired. Check_zone_nr sets *corrected
 * if an error was corrected, and returns the zone (0 for no zone
 * or a bad zone-number).
 */
unsafe fn check_zone_nr2(mut nr: *mut u32, mut corrected: *mut smallint) -> libc::c_int {
  let mut msg: *const libc::c_char = std::ptr::null();
  if *nr == 0 {
    return 0;
  }
  if *nr < (*ptr_to_globals).u.Super.s_firstdatazone as libc::c_uint {
    msg = b"< FIRSTZONE\x00" as *const u8 as *const libc::c_char
  } else if *nr
    >= (if (*ptr_to_globals).version2 as libc::c_int != 0 {
      (*ptr_to_globals).u.Super.s_zones
    } else {
      (*ptr_to_globals).u.Super.s_nzones as libc::c_uint
    })
  {
    msg = b">= ZONES\x00" as *const u8 as *const libc::c_char
  } else {
    return *nr as libc::c_int;
  }
  printf(
    b"Zone nr %s in file \'%s\'. \x00" as *const u8 as *const libc::c_char,
    msg,
    (*ptr_to_globals).current_name.as_mut_ptr(),
  );
  if ask(
    b"Remove block\x00" as *const u8 as *const libc::c_char,
    1i32,
  ) != 0
  {
    *nr = 0 as u32;
    *corrected = 1i32 as smallint
  }
  return 0;
}

unsafe fn check_zone_nr(mut nr: *mut u16, mut corrected: *mut smallint) -> libc::c_int {
  let mut nr32: u32 = *nr as u32;
  let mut r: libc::c_int = check_zone_nr2(&mut nr32, corrected);
  *nr = nr32 as u16;
  return r;
}
/*
 * read-block reads block nr into the buffer at addr.
 */
unsafe fn read_block(mut nr: libc::c_uint, mut addr: *mut libc::c_void) {
  if nr == 0 {
    memset(addr, 0, BLOCK_SIZE as libc::c_int as libc::c_ulong);
    return;
  }
  crate::libbb::xfuncs_printf::xlseek(
    dev_fd as libc::c_int,
    (BLOCK_SIZE as libc::c_int as libc::c_uint).wrapping_mul(nr) as off_t,
    0,
  );
  if BLOCK_SIZE as libc::c_int as i64
    != crate::libbb::read::full_read(
      dev_fd as libc::c_int,
      addr,
      BLOCK_SIZE as libc::c_int as size_t,
    ) as i64
  {
    printf(
      b"%s: bad block %u in file \'%s\'\n\x00" as *const u8 as *const libc::c_char,
      b"read error\x00" as *const u8 as *const libc::c_char,
      nr,
      (*ptr_to_globals).current_name.as_mut_ptr(),
    );
    (*ptr_to_globals).errors_uncorrected = 1i32 as smallint;
    memset(addr, 0, BLOCK_SIZE as libc::c_int as libc::c_ulong);
  };
}

/*
 * write_block writes block nr to disk.
 */
unsafe fn write_block(mut nr: libc::c_uint, mut addr: *mut libc::c_void) {
  if nr == 0 {
    return;
  }
  if nr < (*ptr_to_globals).u.Super.s_firstdatazone as libc::c_uint
    || nr
      >= (if (*ptr_to_globals).version2 as libc::c_int != 0 {
        (*ptr_to_globals).u.Super.s_zones
      } else {
        (*ptr_to_globals).u.Super.s_nzones as libc::c_uint
      })
  {
    puts(
      b"Internal error: trying to write bad block\nWrite request ignored\x00" as *const u8
        as *const libc::c_char,
    );
    (*ptr_to_globals).errors_uncorrected = 1i32 as smallint;
    return;
  }
  crate::libbb::xfuncs_printf::xlseek(
    dev_fd as libc::c_int,
    (BLOCK_SIZE as libc::c_int as libc::c_uint).wrapping_mul(nr) as off_t,
    0,
  );
  if BLOCK_SIZE as libc::c_int as i64
    != crate::libbb::full_write::full_write(
      dev_fd as libc::c_int,
      addr,
      BLOCK_SIZE as libc::c_int as size_t,
    ) as i64
  {
    printf(
      b"%s: bad block %u in file \'%s\'\n\x00" as *const u8 as *const libc::c_char,
      b"write error\x00" as *const u8 as *const libc::c_char,
      nr,
      (*ptr_to_globals).current_name.as_mut_ptr(),
    );
    (*ptr_to_globals).errors_uncorrected = 1i32 as smallint
  };
}

/*
 * map_block calculates the absolute block nr of a block in a file.
 * It sets 'changed' if the inode has needed changing, and re-writes
 * any indirect blocks with errors.
 */
unsafe fn map_block(mut inode: *mut minix1_inode, mut blknr: libc::c_uint) -> libc::c_int {
  let mut ind: [u16; 512] = [0; 512]; /* double indirect */
  let mut block: libc::c_int = 0; /* triple indirect */
  let mut result: libc::c_int = 0; /* double indirect */
  let mut blk_chg: smallint = 0;
  if blknr < 7i32 as libc::c_uint {
    return check_zone_nr(
      (*inode).i_zone.as_mut_ptr().offset(blknr as isize),
      &mut (*ptr_to_globals).changed,
    );
  }
  blknr = blknr.wrapping_sub(7i32 as libc::c_uint);
  if blknr < 512i32 as libc::c_uint {
    block = check_zone_nr(
      (*inode).i_zone.as_mut_ptr().offset(7),
      &mut (*ptr_to_globals).changed,
    )
  } else {
    blknr = blknr.wrapping_sub(512i32 as libc::c_uint);
    block = check_zone_nr(
      (*inode).i_zone.as_mut_ptr().offset(8),
      &mut (*ptr_to_globals).changed,
    );
    read_block(block as libc::c_uint, ind.as_mut_ptr() as *mut libc::c_void);
    blk_chg = 0 as smallint;
    result = check_zone_nr(
      &mut *ind
        .as_mut_ptr()
        .offset(blknr.wrapping_div(512i32 as libc::c_uint) as isize),
      &mut blk_chg,
    );
    if blk_chg != 0 {
      write_block(block as libc::c_uint, ind.as_mut_ptr() as *mut libc::c_void);
    }
    block = result
  }
  read_block(block as libc::c_uint, ind.as_mut_ptr() as *mut libc::c_void);
  blk_chg = 0 as smallint;
  result = check_zone_nr(
    &mut *ind
      .as_mut_ptr()
      .offset(blknr.wrapping_rem(512i32 as libc::c_uint) as isize),
    &mut blk_chg,
  );
  if blk_chg != 0 {
    write_block(block as libc::c_uint, ind.as_mut_ptr() as *mut libc::c_void);
  }
  return result;
}

unsafe fn map_block2(mut inode: *mut minix2_inode, mut blknr: libc::c_uint) -> libc::c_int {
  let mut ind: [u32; 256] = [0; 256];
  let mut block: libc::c_int = 0;
  let mut result: libc::c_int = 0;
  let mut blk_chg: smallint = 0;
  if blknr < 7i32 as libc::c_uint {
    return check_zone_nr2(
      (*inode).i_zone.as_mut_ptr().offset(blknr as isize),
      &mut (*ptr_to_globals).changed,
    );
  }
  blknr = blknr.wrapping_sub(7i32 as libc::c_uint);
  if blknr < 256i32 as libc::c_uint {
    block = check_zone_nr2(
      (*inode).i_zone.as_mut_ptr().offset(7),
      &mut (*ptr_to_globals).changed,
    )
  } else {
    blknr = blknr.wrapping_sub(256i32 as libc::c_uint);
    if blknr < (256i32 * 256i32) as libc::c_uint {
      block = check_zone_nr2(
        (*inode).i_zone.as_mut_ptr().offset(8),
        &mut (*ptr_to_globals).changed,
      )
    } else {
      blknr = blknr.wrapping_sub((256i32 * 256i32) as libc::c_uint);
      block = check_zone_nr2(
        (*inode).i_zone.as_mut_ptr().offset(9),
        &mut (*ptr_to_globals).changed,
      );
      read_block(block as libc::c_uint, ind.as_mut_ptr() as *mut libc::c_void);
      blk_chg = 0 as smallint;
      result = check_zone_nr2(
        &mut *ind
          .as_mut_ptr()
          .offset(blknr.wrapping_div((256i32 * 256i32) as libc::c_uint) as isize),
        &mut blk_chg,
      );
      if blk_chg != 0 {
        write_block(block as libc::c_uint, ind.as_mut_ptr() as *mut libc::c_void);
      }
      block = result
    }
    read_block(block as libc::c_uint, ind.as_mut_ptr() as *mut libc::c_void);
    blk_chg = 0 as smallint;
    result = check_zone_nr2(
      &mut *ind.as_mut_ptr().offset(
        blknr
          .wrapping_div(256i32 as libc::c_uint)
          .wrapping_rem(256i32 as libc::c_uint) as isize,
      ),
      &mut blk_chg,
    );
    if blk_chg != 0 {
      write_block(block as libc::c_uint, ind.as_mut_ptr() as *mut libc::c_void);
    }
    block = result
  }
  read_block(block as libc::c_uint, ind.as_mut_ptr() as *mut libc::c_void);
  blk_chg = 0 as smallint;
  result = check_zone_nr2(
    &mut *ind
      .as_mut_ptr()
      .offset(blknr.wrapping_rem(256i32 as libc::c_uint) as isize),
    &mut blk_chg,
  );
  if blk_chg != 0 {
    write_block(block as libc::c_uint, ind.as_mut_ptr() as *mut libc::c_void);
  }
  return result;
}

unsafe fn write_superblock() {
  /*
   * Set the state of the filesystem based on whether or not there
   * are uncorrected errors.  The filesystem valid flag is
   * unconditionally set if we get this far.
   */
  (*ptr_to_globals).u.Super.s_state = ((*ptr_to_globals).u.Super.s_state as libc::c_int
    | (MINIX_VALID_FS as libc::c_int | MINIX_ERROR_FS as libc::c_int))
    as u16;
  if (*ptr_to_globals).errors_uncorrected == 0 {
    (*ptr_to_globals).u.Super.s_state =
      ((*ptr_to_globals).u.Super.s_state as libc::c_int & !(MINIX_ERROR_FS as libc::c_int)) as u16
  }
  crate::libbb::xfuncs_printf::xlseek(dev_fd as libc::c_int, BLOCK_SIZE as libc::c_int as off_t, 0);
  if BLOCK_SIZE as libc::c_int as i64
    != crate::libbb::full_write::full_write(
      dev_fd as libc::c_int,
      (*ptr_to_globals).u.superblock_buffer.as_mut_ptr() as *const libc::c_void,
      BLOCK_SIZE as libc::c_int as size_t,
    ) as i64
  {
    die(b"can\'t write superblock\x00" as *const u8 as *const libc::c_char);
  };
}

unsafe fn write_tables() {
  write_superblock();

  let imap_size = (*ptr_to_globals).u.Super.s_imap_blocks as isize * BLOCK_SIZE as isize;
  if imap_size
    != libc::write(
      dev_fd as libc::c_int,
      (*ptr_to_globals).inode_map as *const libc::c_void,
      imap_size as usize,
    )
  {
    die(b"can\'t write inode map\x00" as *const u8 as *const libc::c_char);
  }

  let zmap_size = (*ptr_to_globals).u.Super.s_zmap_blocks as isize * BLOCK_SIZE as isize;
  if zmap_size
    != libc::write(
      dev_fd as libc::c_int,
      (*ptr_to_globals).zone_map as *const libc::c_void,
      zmap_size as usize,
    )
  {
    die(b"can\'t write zone map\x00" as *const u8 as *const libc::c_char);
  }

  let minix_inodes_per_block = if (*ptr_to_globals).version2 != 0 {
    MINIX2_INODES_PER_BLOCK
  } else {
    MINIX1_INODES_PER_BLOCK
  };
  let inode_blocks = div_roundup(
    (*ptr_to_globals).u.Super.s_ninodes as u32,
    minix_inodes_per_block,
  );
  let inode_buffer_size = inode_blocks as isize * BLOCK_SIZE as isize;
  if inode_buffer_size
    != libc::write(
      dev_fd as libc::c_int,
      (*ptr_to_globals).inode_buffer as *const libc::c_void,
      inode_buffer_size as usize,
    )
  {
    die(b"can\'t write inodes\x00" as *const u8 as *const libc::c_char);
  };
}

unsafe fn get_dirsize() {
  let mut block: libc::c_int = 0;
  let mut blk: [libc::c_char; 1024] = [0; 1024];
  let mut size: libc::c_int = 0;
  if (*ptr_to_globals).version2 != 0 {
    block = (*((*ptr_to_globals).inode_buffer as *mut minix2_inode)
      .offset(-1)
      .offset(MINIX_ROOT_INO as libc::c_int as isize))
    .i_zone[0] as libc::c_int
  } else {
    block = (*((*ptr_to_globals).inode_buffer as *mut minix1_inode)
      .offset(-1)
      .offset(MINIX_ROOT_INO as libc::c_int as isize))
    .i_zone[0] as libc::c_int
  }
  read_block(block as libc::c_uint, blk.as_mut_ptr() as *mut libc::c_void);
  size = 16i32;
  while size < BLOCK_SIZE as libc::c_int {
    if strcmp(
      blk.as_mut_ptr().offset(size as isize).offset(2),
      b"..\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
      (*ptr_to_globals).dirsize = size as smallint;
      (*ptr_to_globals).namelen = (size - 2i32) as smallint;
      return;
    }
    size <<= 1i32
  }
  /* use defaults */
}

unsafe fn read_superblock() {
  crate::libbb::xfuncs_printf::xlseek(dev_fd as libc::c_int, BLOCK_SIZE as libc::c_int as off_t, 0);
  if BLOCK_SIZE
    != crate::libbb::read::full_read(
      dev_fd as libc::c_int,
      (*ptr_to_globals).u.superblock_buffer.as_mut_ptr() as *mut libc::c_void,
      BLOCK_SIZE as libc::c_int as size_t,
    ) as u32
  {
    die(b"can\'t read superblock\x00" as *const u8 as *const libc::c_char);
  }
  /* already initialized to:
  namelen = 14;
  dirsize = 16;
  version2 = 0;
  */
  if !((*ptr_to_globals).u.Super.s_magic as libc::c_int == MINIX1_SUPER_MAGIC as libc::c_int) {
    if (*ptr_to_globals).u.Super.s_magic as libc::c_int == MINIX1_SUPER_MAGIC2 as libc::c_int {
      (*ptr_to_globals).namelen = 30i32 as smallint;
      (*ptr_to_globals).dirsize = 32i32 as smallint
    } else if (*ptr_to_globals).u.Super.s_magic as libc::c_int == MINIX2_SUPER_MAGIC as libc::c_int
    {
      (*ptr_to_globals).version2 = 1i32 as smallint
    } else if (*ptr_to_globals).u.Super.s_magic as libc::c_int == MINIX2_SUPER_MAGIC2 as libc::c_int
    {
      (*ptr_to_globals).namelen = 30i32 as smallint;
      (*ptr_to_globals).dirsize = 32i32 as smallint;
      (*ptr_to_globals).version2 = 1i32 as smallint
    } else {
      die(b"bad magic number in superblock\x00" as *const u8 as *const libc::c_char);
    }
  }
  if (*ptr_to_globals).u.Super.s_log_zone_size as libc::c_uint != 0 as libc::c_uint
    || BLOCK_SIZE as libc::c_int != 1024i32
  {
    die(b"only 1k blocks/zones supported\x00" as *const u8 as *const libc::c_char);
  }
  if ((*ptr_to_globals).u.Super.s_imap_blocks as libc::c_uint)
    .wrapping_mul(BLOCK_SIZE as libc::c_int as libc::c_uint)
    .wrapping_mul(8i32 as libc::c_uint)
    < ((*ptr_to_globals).u.Super.s_ninodes as libc::c_uint).wrapping_add(1i32 as libc::c_uint)
  {
    die(b"bad s_imap_blocks field in superblock\x00" as *const u8 as *const libc::c_char);
  }
  if ((*ptr_to_globals).u.Super.s_zmap_blocks as libc::c_uint)
    .wrapping_mul(BLOCK_SIZE as libc::c_int as libc::c_uint)
    .wrapping_mul(8i32 as libc::c_uint)
    < (if (*ptr_to_globals).version2 as libc::c_int != 0 {
      (*ptr_to_globals).u.Super.s_zones
    } else {
      (*ptr_to_globals).u.Super.s_nzones as libc::c_uint
    })
    .wrapping_sub((*ptr_to_globals).u.Super.s_firstdatazone as libc::c_uint)
    .wrapping_add(1i32 as libc::c_uint)
  {
    die(b"bad s_zmap_blocks field in superblock\x00" as *const u8 as *const libc::c_char);
  };
}

unsafe fn read_tables() {
  (*ptr_to_globals).inode_map = crate::libbb::xfuncs_printf::xzalloc(
    ((*ptr_to_globals).u.Super.s_imap_blocks as libc::c_uint)
      .wrapping_mul(BLOCK_SIZE as libc::c_int as libc::c_uint) as size_t,
  ) as *mut libc::c_char;
  (*ptr_to_globals).zone_map = crate::libbb::xfuncs_printf::xzalloc(
    ((*ptr_to_globals).u.Super.s_zmap_blocks as libc::c_uint)
      .wrapping_mul(BLOCK_SIZE as libc::c_int as libc::c_uint) as size_t,
  ) as *mut libc::c_char;
  (*ptr_to_globals).inode_buffer = xmalloc(
    div_roundup(
      (*ptr_to_globals).u.Super.s_ninodes as libc::c_uint,
      (if (*ptr_to_globals).version2 as libc::c_int != 0 {
        MINIX2_INODES_PER_BLOCK as libc::c_int
      } else {
        MINIX1_INODES_PER_BLOCK as libc::c_int
      }) as libc::c_uint,
    )
    .wrapping_mul(BLOCK_SIZE as libc::c_int as libc::c_uint) as size_t,
  ) as *mut libc::c_char;
  (*ptr_to_globals).inode_count = xmalloc(
    ((*ptr_to_globals).u.Super.s_ninodes as libc::c_uint).wrapping_add(1i32 as libc::c_uint)
      as size_t,
  ) as *mut libc::c_uchar;
  (*ptr_to_globals).zone_count = xmalloc(if (*ptr_to_globals).version2 as libc::c_int != 0 {
    (*ptr_to_globals).u.Super.s_zones
  } else {
    (*ptr_to_globals).u.Super.s_nzones as libc::c_uint
  } as size_t) as *mut libc::c_uchar;
  if ((*ptr_to_globals).u.Super.s_imap_blocks as libc::c_uint)
    .wrapping_mul(BLOCK_SIZE as libc::c_int as libc::c_uint) as i64
    != read(
      dev_fd as libc::c_int,
      (*ptr_to_globals).inode_map as *mut libc::c_void,
      ((*ptr_to_globals).u.Super.s_imap_blocks as libc::c_uint)
        .wrapping_mul(BLOCK_SIZE as libc::c_int as libc::c_uint) as size_t,
    ) as i64
  {
    die(b"can\'t read inode map\x00" as *const u8 as *const libc::c_char);
  }
  if ((*ptr_to_globals).u.Super.s_zmap_blocks as libc::c_uint)
    .wrapping_mul(BLOCK_SIZE as libc::c_int as libc::c_uint) as i64
    != read(
      dev_fd as libc::c_int,
      (*ptr_to_globals).zone_map as *mut libc::c_void,
      ((*ptr_to_globals).u.Super.s_zmap_blocks as libc::c_uint)
        .wrapping_mul(BLOCK_SIZE as libc::c_int as libc::c_uint) as size_t,
    ) as i64
  {
    die(b"can\'t read zone map\x00" as *const u8 as *const libc::c_char);
  }
  if div_roundup(
    (*ptr_to_globals).u.Super.s_ninodes as libc::c_uint,
    (if (*ptr_to_globals).version2 as libc::c_int != 0 {
      MINIX2_INODES_PER_BLOCK as libc::c_int
    } else {
      MINIX1_INODES_PER_BLOCK as libc::c_int
    }) as libc::c_uint,
  )
  .wrapping_mul(BLOCK_SIZE as libc::c_int as libc::c_uint) as i64
    != read(
      dev_fd as libc::c_int,
      (*ptr_to_globals).inode_buffer as *mut libc::c_void,
      div_roundup(
        (*ptr_to_globals).u.Super.s_ninodes as libc::c_uint,
        (if (*ptr_to_globals).version2 as libc::c_int != 0 {
          MINIX2_INODES_PER_BLOCK as libc::c_int
        } else {
          MINIX1_INODES_PER_BLOCK as libc::c_int
        }) as libc::c_uint,
      )
      .wrapping_mul(BLOCK_SIZE as libc::c_int as libc::c_uint) as size_t,
    ) as i64
  {
    die(b"can\'t read inodes\x00" as *const u8 as *const libc::c_char);
  }
  if (2i32 as libc::c_uint)
    .wrapping_add((*ptr_to_globals).u.Super.s_imap_blocks as libc::c_uint)
    .wrapping_add((*ptr_to_globals).u.Super.s_zmap_blocks as libc::c_uint)
    .wrapping_add(div_roundup(
      (*ptr_to_globals).u.Super.s_ninodes as libc::c_uint,
      (if (*ptr_to_globals).version2 as libc::c_int != 0 {
        MINIX2_INODES_PER_BLOCK as libc::c_int
      } else {
        MINIX1_INODES_PER_BLOCK as libc::c_int
      }) as libc::c_uint,
    ))
    != (*ptr_to_globals).u.Super.s_firstdatazone as libc::c_uint
  {
    puts(b"warning: firstzone!=norm_firstzone\x00" as *const u8 as *const libc::c_char);
    (*ptr_to_globals).errors_uncorrected = 1i32 as smallint
  }
  get_dirsize();
  if option_mask32 & OPT_s as libc::c_int as libc::c_uint != 0 {
    printf(b"%u inodes\n%u blocks\nFirstdatazone=%u (%u)\nZonesize=%u\nMaxsize=%u\nFilesystem state=%u\nnamelen=%u\n\n\x00"
                   as *const u8 as *const libc::c_char,
               (*ptr_to_globals).u.Super.s_ninodes as libc::c_uint,
               if (*ptr_to_globals).version2 as libc::c_int != 0 {
                   (*ptr_to_globals).u.Super.s_zones
               } else { (*ptr_to_globals).u.Super.s_nzones as libc::c_uint },
               (*ptr_to_globals).u.Super.s_firstdatazone as libc::c_uint,
               (2i32 as
                    libc::c_uint).wrapping_add((*ptr_to_globals).u.Super.s_imap_blocks
                                                   as
                                                   libc::c_uint).wrapping_add((*ptr_to_globals).u.Super.s_zmap_blocks
                                                                                  as
                                                                                  libc::c_uint).wrapping_add(div_roundup((*ptr_to_globals).u.Super.s_ninodes
                                                                                                                             as
                                                                                                                             libc::c_uint,
                                                                                                                         (if (*ptr_to_globals).version2
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                                 !=
                                                                                                                                 0
                                                                                                                             {
                                                                                                                              MINIX2_INODES_PER_BLOCK
                                                                                                                                  as
                                                                                                                                  libc::c_int
                                                                                                                          } else {
                                                                                                                              MINIX1_INODES_PER_BLOCK
                                                                                                                                  as
                                                                                                                                  libc::c_int
                                                                                                                          })
                                                                                                                             as
                                                                                                                             libc::c_uint)),
               (BLOCK_SIZE as libc::c_int) <<
                   (*ptr_to_globals).u.Super.s_log_zone_size as libc::c_uint,
               (*ptr_to_globals).u.Super.s_max_size,
               (*ptr_to_globals).u.Super.s_state as libc::c_int,
               (*ptr_to_globals).namelen as libc::c_int);
  };
}

unsafe fn get_inode_common(mut nr: libc::c_uint, mut i_mode: u16) {
  (*ptr_to_globals).total += 1;
  if *(*ptr_to_globals).inode_count.offset(nr as isize) == 0 {
    if minix_bit((*ptr_to_globals).inode_map, nr) == 0 {
      printf(
        b"Inode %u is marked as \'unused\', but it is used for file \'%s\'\n\x00" as *const u8
          as *const libc::c_char,
        nr,
        (*ptr_to_globals).current_name.as_mut_ptr(),
      );
      if option_mask32 & OPT_r as libc::c_int as libc::c_uint != 0 {
        if ask(
          b"Mark as \'in use\'\x00" as *const u8 as *const libc::c_char,
          1i32,
        ) != 0
        {
          minix_setbit((*ptr_to_globals).inode_map, nr);
        } else {
          (*ptr_to_globals).errors_uncorrected = 1i32 as smallint
        }
      }
    }
    if i_mode as libc::c_int & 0o170000i32 == 0o40000i32 {
      (*ptr_to_globals).directory += 1
    } else if i_mode as libc::c_int & 0o170000i32 == 0o100000i32 {
      (*ptr_to_globals).regular += 1
    } else if i_mode as libc::c_int & 0o170000i32 == 0o20000i32 {
      (*ptr_to_globals).chardev += 1
    } else if i_mode as libc::c_int & 0o170000i32 == 0o60000i32 {
      (*ptr_to_globals).blockdev += 1
    } else if i_mode as libc::c_int & 0o170000i32 == 0o120000i32 {
      (*ptr_to_globals).symlinks += 1
    } else if !(i_mode as libc::c_int & 0o170000i32 == 0o140000i32) {
      if !(i_mode as libc::c_int & 0o170000i32 == 0o10000i32) {
        printf(
          b"%s has mode %05o\n\x00" as *const u8 as *const libc::c_char,
          (*ptr_to_globals).current_name.as_mut_ptr(),
          i_mode as libc::c_int,
        );
      }
    }
  } else {
    (*ptr_to_globals).links += 1
  }
  let ref mut fresh3 = *(*ptr_to_globals).inode_count.offset(nr as isize);
  *fresh3 = (*fresh3).wrapping_add(1);
  if *fresh3 == 0 {
    puts(b"Warning: inode count too big\x00" as *const u8 as *const libc::c_char);
    let ref mut fresh4 = *(*ptr_to_globals).inode_count.offset(nr as isize);
    *fresh4 = (*fresh4).wrapping_sub(1);
    (*ptr_to_globals).errors_uncorrected = 1i32 as smallint
  };
}

unsafe fn get_inode(mut nr: libc::c_uint) -> *mut minix1_inode {
  let mut inode: *mut minix1_inode = std::ptr::null_mut();
  if nr == 0 || nr > (*ptr_to_globals).u.Super.s_ninodes as libc::c_uint {
    return std::ptr::null_mut();
  }
  inode = ((*ptr_to_globals).inode_buffer as *mut minix1_inode)
    .offset(-1)
    .offset(nr as isize);
  get_inode_common(nr, (*inode).i_mode);
  return inode;
}

unsafe fn get_inode2(mut nr: libc::c_uint) -> *mut minix2_inode {
  let mut inode: *mut minix2_inode = std::ptr::null_mut();
  if nr == 0 || nr > (*ptr_to_globals).u.Super.s_ninodes as libc::c_uint {
    return std::ptr::null_mut();
  }
  inode = ((*ptr_to_globals).inode_buffer as *mut minix2_inode)
    .offset(-1)
    .offset(nr as isize);
  get_inode_common(nr, (*inode).i_mode);
  return inode;
}

unsafe fn check_root() {
  let mut inode: *mut minix1_inode = ((*ptr_to_globals).inode_buffer as *mut minix1_inode)
    .offset(-1)
    .offset(MINIX_ROOT_INO as libc::c_int as isize);
  if inode.is_null() || !((*inode).i_mode as libc::c_int & 0o170000i32 == 0o40000i32) {
    die(b"root inode isn\'t a directory\x00" as *const u8 as *const libc::c_char);
  };
}

unsafe fn check_root2() {
  let mut inode: *mut minix2_inode = ((*ptr_to_globals).inode_buffer as *mut minix2_inode)
    .offset(-1)
    .offset(MINIX_ROOT_INO as libc::c_int as isize);
  if inode.is_null() || !((*inode).i_mode as libc::c_int & 0o170000i32 == 0o40000i32) {
    die(b"root inode isn\'t a directory\x00" as *const u8 as *const libc::c_char);
  };
}
unsafe fn add_zone_common(mut block: libc::c_int, mut corrected: *mut smallint) -> libc::c_int {
  if block == 0 {
    return 0;
  }
  if *(*ptr_to_globals).zone_count.offset(block as isize) != 0 {
    printf(
      b"Already used block is reused in file \'%s\'. \x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).current_name.as_mut_ptr(),
    );
    if ask(b"Clear\x00" as *const u8 as *const libc::c_char, 1i32) != 0 {
      block = 0;
      *corrected = 1i32 as smallint;
      return -1i32;
      /* "please zero out *znr" */
    }
  }
  if minix_bit(
    (*ptr_to_globals).zone_map,
    (block as libc::c_uint)
      .wrapping_sub((*ptr_to_globals).u.Super.s_firstdatazone as libc::c_uint)
      .wrapping_add(1i32 as libc::c_uint),
  ) == 0
  {
    printf(
      b"Block %d in file \'%s\' is marked as \'unused\'. \x00" as *const u8 as *const libc::c_char,
      block,
      (*ptr_to_globals).current_name.as_mut_ptr(),
    );
    if ask(b"Correct\x00" as *const u8 as *const libc::c_char, 1i32) != 0 {
      minix_setbit(
        (*ptr_to_globals).zone_map,
        (block as libc::c_uint)
          .wrapping_sub((*ptr_to_globals).u.Super.s_firstdatazone as libc::c_uint)
          .wrapping_add(1i32 as libc::c_uint),
      );
    }
  }
  let ref mut fresh5 = *(*ptr_to_globals).zone_count.offset(block as isize);
  *fresh5 = (*fresh5).wrapping_add(1);
  if *fresh5 == 0 {
    let ref mut fresh6 = *(*ptr_to_globals).zone_count.offset(block as isize);
    *fresh6 = (*fresh6).wrapping_sub(1)
  }
  return block;
}
unsafe fn add_zone(mut znr: *mut u16, mut corrected: *mut smallint) -> libc::c_int {
  let mut block: libc::c_int = 0;
  block = check_zone_nr(znr, corrected);
  block = add_zone_common(block, corrected);
  if block == -1i32 {
    *znr = 0 as u16;
    block = 0
  }
  return block;
}
unsafe fn add_zone2(mut znr: *mut u32, mut corrected: *mut smallint) -> libc::c_int {
  let mut block: libc::c_int = 0;
  block = check_zone_nr2(znr, corrected);
  block = add_zone_common(block, corrected);
  if block == -1i32 {
    *znr = 0 as u32;
    block = 0
  }
  return block;
}
unsafe fn add_zone_ind(mut znr: *mut u16, mut corrected: *mut smallint) {
  let mut i: libc::c_int = 0;
  let mut block: libc::c_int = 0;
  let mut chg_blk: smallint = 0 as smallint;
  block = add_zone(znr, corrected);
  if block == 0 {
    return;
  }
  read_block(
    block as libc::c_uint,
    (*ptr_to_globals).add_zone_ind_blk.as_mut_ptr() as *mut libc::c_void,
  );
  i = 0;
  while i < BLOCK_SIZE as libc::c_int >> 1i32 {
    add_zone(
      ((*ptr_to_globals).add_zone_ind_blk.as_mut_ptr() as *mut u16).offset(i as isize),
      &mut chg_blk,
    );
    i += 1
  }
  if chg_blk != 0 {
    write_block(
      block as libc::c_uint,
      (*ptr_to_globals).add_zone_ind_blk.as_mut_ptr() as *mut libc::c_void,
    );
  };
}
unsafe fn add_zone_ind2(mut znr: *mut u32, mut corrected: *mut smallint) {
  let mut i: libc::c_int = 0;
  let mut block: libc::c_int = 0;
  let mut chg_blk: smallint = 0 as smallint;
  block = add_zone2(znr, corrected);
  if block == 0 {
    return;
  }
  read_block(
    block as libc::c_uint,
    (*ptr_to_globals).add_zone_ind_blk.as_mut_ptr() as *mut libc::c_void,
  );
  i = 0;
  while i < BLOCK_SIZE as libc::c_int >> 2i32 {
    add_zone2(
      ((*ptr_to_globals).add_zone_ind_blk.as_mut_ptr() as *mut u32).offset(i as isize),
      &mut chg_blk,
    );
    i += 1
  }
  if chg_blk != 0 {
    write_block(
      block as libc::c_uint,
      (*ptr_to_globals).add_zone_ind_blk.as_mut_ptr() as *mut libc::c_void,
    );
  };
}
unsafe fn add_zone_dind(mut znr: *mut u16, mut corrected: *mut smallint) {
  let mut i: libc::c_int = 0;
  let mut block: libc::c_int = 0;
  let mut chg_blk: smallint = 0 as smallint;
  block = add_zone(znr, corrected);
  if block == 0 {
    return;
  }
  read_block(
    block as libc::c_uint,
    (*ptr_to_globals).add_zone_dind_blk.as_mut_ptr() as *mut libc::c_void,
  );
  i = 0;
  while i < BLOCK_SIZE as libc::c_int >> 1i32 {
    add_zone_ind(
      ((*ptr_to_globals).add_zone_dind_blk.as_mut_ptr() as *mut u16).offset(i as isize),
      &mut chg_blk,
    );
    i += 1
  }
  if chg_blk != 0 {
    write_block(
      block as libc::c_uint,
      (*ptr_to_globals).add_zone_dind_blk.as_mut_ptr() as *mut libc::c_void,
    );
  };
}
unsafe fn add_zone_dind2(mut znr: *mut u32, mut corrected: *mut smallint) {
  let mut i: libc::c_int = 0;
  let mut block: libc::c_int = 0;
  let mut chg_blk: smallint = 0 as smallint;
  block = add_zone2(znr, corrected);
  if block == 0 {
    return;
  }
  read_block(
    block as libc::c_uint,
    (*ptr_to_globals).add_zone_dind_blk.as_mut_ptr() as *mut libc::c_void,
  );
  i = 0;
  while i < BLOCK_SIZE as libc::c_int >> 2i32 {
    add_zone_ind2(
      ((*ptr_to_globals).add_zone_dind_blk.as_mut_ptr() as *mut u32).offset(i as isize),
      &mut chg_blk,
    );
    i += 1
  }
  if chg_blk != 0 {
    write_block(
      block as libc::c_uint,
      (*ptr_to_globals).add_zone_dind_blk.as_mut_ptr() as *mut libc::c_void,
    );
  };
}
unsafe fn add_zone_tind2(mut znr: *mut u32, mut corrected: *mut smallint) {
  let mut i: libc::c_int = 0;
  let mut block: libc::c_int = 0;
  let mut chg_blk: smallint = 0 as smallint;
  block = add_zone2(znr, corrected);
  if block == 0 {
    return;
  }
  read_block(
    block as libc::c_uint,
    (*ptr_to_globals).add_zone_tind_blk.as_mut_ptr() as *mut libc::c_void,
  );
  i = 0;
  while i < BLOCK_SIZE as libc::c_int >> 2i32 {
    add_zone_dind2(
      ((*ptr_to_globals).add_zone_tind_blk.as_mut_ptr() as *mut u32).offset(i as isize),
      &mut chg_blk,
    );
    i += 1
  }
  if chg_blk != 0 {
    write_block(
      block as libc::c_uint,
      (*ptr_to_globals).add_zone_tind_blk.as_mut_ptr() as *mut libc::c_void,
    );
  };
}
unsafe fn check_zones(mut i: libc::c_uint) {
  let mut inode: *mut minix1_inode = std::ptr::null_mut();
  if i == 0 || i > (*ptr_to_globals).u.Super.s_ninodes as libc::c_uint {
    return;
  }
  if *(*ptr_to_globals).inode_count.offset(i as isize) as libc::c_int > 1i32 {
    /* have we counted this file already? */
    return;
  }
  inode = ((*ptr_to_globals).inode_buffer as *mut minix1_inode)
    .offset(-1)
    .offset(i as isize);
  if !((*inode).i_mode as libc::c_int & 0o170000i32 == 0o40000i32)
    && !((*inode).i_mode as libc::c_int & 0o170000i32 == 0o100000i32)
    && !((*inode).i_mode as libc::c_int & 0o170000i32 == 0o120000i32)
  {
    return;
  }
  i = 0 as libc::c_uint;
  while i < 7i32 as libc::c_uint {
    add_zone(
      (*inode).i_zone.as_mut_ptr().offset(i as isize),
      &mut (*ptr_to_globals).changed,
    );
    i = i.wrapping_add(1)
  }
  add_zone_ind(
    (*inode).i_zone.as_mut_ptr().offset(7),
    &mut (*ptr_to_globals).changed,
  );
  add_zone_dind(
    (*inode).i_zone.as_mut_ptr().offset(8),
    &mut (*ptr_to_globals).changed,
  );
}
unsafe fn check_zones2(mut i: libc::c_uint) {
  let mut inode: *mut minix2_inode = std::ptr::null_mut();
  if i == 0 || i > (*ptr_to_globals).u.Super.s_ninodes as libc::c_uint {
    return;
  }
  if *(*ptr_to_globals).inode_count.offset(i as isize) as libc::c_int > 1i32 {
    /* have we counted this file already? */
    return;
  }
  inode = ((*ptr_to_globals).inode_buffer as *mut minix2_inode)
    .offset(-1)
    .offset(i as isize);
  if !((*inode).i_mode as libc::c_int & 0o170000i32 == 0o40000i32)
    && !((*inode).i_mode as libc::c_int & 0o170000i32 == 0o100000i32)
    && !((*inode).i_mode as libc::c_int & 0o170000i32 == 0o120000i32)
  {
    return;
  }
  i = 0 as libc::c_uint;
  while i < 7i32 as libc::c_uint {
    add_zone2(
      (*inode).i_zone.as_mut_ptr().offset(i as isize),
      &mut (*ptr_to_globals).changed,
    );
    i = i.wrapping_add(1)
  }
  add_zone_ind2(
    (*inode).i_zone.as_mut_ptr().offset(7),
    &mut (*ptr_to_globals).changed,
  );
  add_zone_dind2(
    (*inode).i_zone.as_mut_ptr().offset(8),
    &mut (*ptr_to_globals).changed,
  );
  add_zone_tind2(
    (*inode).i_zone.as_mut_ptr().offset(9),
    &mut (*ptr_to_globals).changed,
  );
}
unsafe fn check_file(mut dir: *mut minix1_inode, mut offset: libc::c_uint) {
  let mut inode: *mut minix1_inode = std::ptr::null_mut();
  let mut ino: libc::c_int = 0;
  let mut name: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut block: libc::c_int = 0;
  block = map_block(
    dir,
    offset.wrapping_div(BLOCK_SIZE as libc::c_int as libc::c_uint),
  );
  read_block(
    block as libc::c_uint,
    (*ptr_to_globals).check_file_blk.as_mut_ptr() as *mut libc::c_void,
  );
  name = (*ptr_to_globals)
    .check_file_blk
    .as_mut_ptr()
    .offset(offset.wrapping_rem(BLOCK_SIZE as libc::c_int as libc::c_uint) as isize)
    .offset(2);
  ino = *(name.offset(-2) as *mut u16) as libc::c_int;
  if ino as libc::c_uint > (*ptr_to_globals).u.Super.s_ninodes as libc::c_uint {
    printf(
      b"%s contains a bad inode number for file \'%.*s\'. \x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).current_name.as_mut_ptr(),
      (*ptr_to_globals).namelen as libc::c_int,
      name,
    );
    if ask(b"Remove\x00" as *const u8 as *const libc::c_char, 1i32) != 0 {
      *(name.offset(-2) as *mut u16) = 0 as u16;
      write_block(
        block as libc::c_uint,
        (*ptr_to_globals).check_file_blk.as_mut_ptr() as *mut libc::c_void,
      );
    }
    ino = 0
  }
  push_filename(name);
  inode = get_inode(ino as libc::c_uint);
  pop_filename();
  if offset == 0 {
    if !inode.is_null() && (*name.offset(0) as libc::c_int == '.' as i32 && *name.offset(1) == 0) {
      return;
    }
    printf(
      b"%s: bad directory: \'.\' isn\'t first\n\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).current_name.as_mut_ptr(),
    );
    (*ptr_to_globals).errors_uncorrected = 1i32 as smallint
  }
  if offset == (*ptr_to_globals).dirsize as libc::c_uint {
    if !inode.is_null() && strcmp(b"..\x00" as *const u8 as *const libc::c_char, name) == 0 {
      return;
    }
    printf(
      b"%s: bad directory: \'..\' isn\'t second\n\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).current_name.as_mut_ptr(),
    );
    (*ptr_to_globals).errors_uncorrected = 1i32 as smallint
  }
  if inode.is_null() {
    return;
  }
  push_filename(name);
  if option_mask32 & OPT_l as libc::c_int as libc::c_uint != 0 {
    if option_mask32 & OPT_v as libc::c_int as libc::c_uint != 0 {
      printf(
        b"%6d %07o %3d \x00" as *const u8 as *const libc::c_char,
        ino,
        (*inode).i_mode as libc::c_int,
        (*inode).i_nlinks as libc::c_int,
      );
    }
    printf(
      b"%s%s\n\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).current_name.as_mut_ptr(),
      if (*inode).i_mode as libc::c_int & 0o170000i32 == 0o40000i32 {
        b":\x00" as *const u8 as *const libc::c_char
      } else {
        b"\x00" as *const u8 as *const libc::c_char
      },
    );
  }
  check_zones(ino as libc::c_uint);
  if !inode.is_null() && (*inode).i_mode as libc::c_int & 0o170000i32 == 0o40000i32 {
    recursive_check(ino as libc::c_uint);
  }
  pop_filename();
}
unsafe fn check_file2(mut dir: *mut minix2_inode, mut offset: libc::c_uint) {
  let mut inode: *mut minix2_inode = std::ptr::null_mut();
  let mut ino: libc::c_int = 0;
  let mut name: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut block: libc::c_int = 0;
  block = map_block2(
    dir,
    offset.wrapping_div(BLOCK_SIZE as libc::c_int as libc::c_uint),
  );
  read_block(
    block as libc::c_uint,
    (*ptr_to_globals).check_file_blk.as_mut_ptr() as *mut libc::c_void,
  );
  name = (*ptr_to_globals)
    .check_file_blk
    .as_mut_ptr()
    .offset(offset.wrapping_rem(BLOCK_SIZE as libc::c_int as libc::c_uint) as isize)
    .offset(2);
  ino = *(name.offset(-2) as *mut u16) as libc::c_int;
  if ino as libc::c_uint > (*ptr_to_globals).u.Super.s_ninodes as libc::c_uint {
    printf(
      b"%s contains a bad inode number for file \'%.*s\'. \x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).current_name.as_mut_ptr(),
      (*ptr_to_globals).namelen as libc::c_int,
      name,
    );
    if ask(b"Remove\x00" as *const u8 as *const libc::c_char, 1i32) != 0 {
      *(name.offset(-2) as *mut u16) = 0 as u16;
      write_block(
        block as libc::c_uint,
        (*ptr_to_globals).check_file_blk.as_mut_ptr() as *mut libc::c_void,
      );
    }
    ino = 0
  }
  push_filename(name);
  inode = get_inode2(ino as libc::c_uint);
  pop_filename();
  if offset == 0 {
    if !inode.is_null() && (*name.offset(0) as libc::c_int == '.' as i32 && *name.offset(1) == 0) {
      return;
    }
    printf(
      b"%s: bad directory: \'.\' isn\'t first\n\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).current_name.as_mut_ptr(),
    );
    (*ptr_to_globals).errors_uncorrected = 1i32 as smallint
  }
  if offset == (*ptr_to_globals).dirsize as libc::c_uint {
    if !inode.is_null() && strcmp(b"..\x00" as *const u8 as *const libc::c_char, name) == 0 {
      return;
    }
    printf(
      b"%s: bad directory: \'..\' isn\'t second\n\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).current_name.as_mut_ptr(),
    );
    (*ptr_to_globals).errors_uncorrected = 1i32 as smallint
  }
  if inode.is_null() {
    return;
  }
  push_filename(name);
  if option_mask32 & OPT_l as libc::c_int as libc::c_uint != 0 {
    if option_mask32 & OPT_v as libc::c_int as libc::c_uint != 0 {
      printf(
        b"%6d %07o %3d \x00" as *const u8 as *const libc::c_char,
        ino,
        (*inode).i_mode as libc::c_int,
        (*inode).i_nlinks as libc::c_int,
      );
    }
    printf(
      b"%s%s\n\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).current_name.as_mut_ptr(),
      if (*inode).i_mode as libc::c_int & 0o170000i32 == 0o40000i32 {
        b":\x00" as *const u8 as *const libc::c_char
      } else {
        b"\x00" as *const u8 as *const libc::c_char
      },
    );
  }
  check_zones2(ino as libc::c_uint);
  if !inode.is_null() && (*inode).i_mode as libc::c_int & 0o170000i32 == 0o40000i32 {
    recursive_check2(ino as libc::c_uint);
  }
  pop_filename();
}
/* Note: do not assume 0/1, it is 0/nonzero */
unsafe fn recursive_check(mut ino: libc::c_uint) {
  let mut dir: *mut minix1_inode = std::ptr::null_mut(); /* trying to check a mounted filesystem? */
  let mut offset: libc::c_uint = 0;
  dir = ((*ptr_to_globals).inode_buffer as *mut minix1_inode)
    .offset(-1)
    .offset(ino as isize);
  if !((*dir).i_mode as libc::c_int & 0o170000i32 == 0o40000i32) {
    die(b"internal error\x00" as *const u8 as *const libc::c_char);
  }
  if (*dir).i_size < (2i32 * (*ptr_to_globals).dirsize as libc::c_int) as libc::c_uint {
    printf(
      b"%s: bad directory: size<32\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).current_name.as_mut_ptr(),
    );
    (*ptr_to_globals).errors_uncorrected = 1i32 as smallint
  }
  offset = 0 as libc::c_uint;
  while offset < (*dir).i_size {
    check_file(dir, offset);
    offset = offset.wrapping_add((*ptr_to_globals).dirsize as libc::c_uint)
  }
}
unsafe fn recursive_check2(mut ino: libc::c_uint) {
  let mut dir: *mut minix2_inode = std::ptr::null_mut();
  let mut offset: libc::c_uint = 0;
  dir = ((*ptr_to_globals).inode_buffer as *mut minix2_inode)
    .offset(-1)
    .offset(ino as isize);
  if !((*dir).i_mode as libc::c_int & 0o170000i32 == 0o40000i32) {
    die(b"internal error\x00" as *const u8 as *const libc::c_char);
  }
  if (*dir).i_size < (2i32 * (*ptr_to_globals).dirsize as libc::c_int) as libc::c_uint {
    printf(
      b"%s: bad directory: size<32\x00" as *const u8 as *const libc::c_char,
      (*ptr_to_globals).current_name.as_mut_ptr(),
    );
    (*ptr_to_globals).errors_uncorrected = 1i32 as smallint
  }
  offset = 0 as libc::c_uint;
  while offset < (*dir).i_size {
    check_file2(dir, offset);
    offset = offset.wrapping_add((*ptr_to_globals).dirsize as libc::c_uint)
  }
}
unsafe fn bad_zone(mut i: libc::c_int) -> libc::c_int {
  let mut buffer: [libc::c_char; 1024] = [0; 1024];
  crate::libbb::xfuncs_printf::xlseek(
    dev_fd as libc::c_int,
    (BLOCK_SIZE as libc::c_int * i) as off_t,
    0,
  );
  return (BLOCK_SIZE as isize
    != crate::libbb::read::full_read(
      dev_fd as libc::c_int,
      buffer.as_mut_ptr() as *mut libc::c_void,
      BLOCK_SIZE.into(),
    )) as libc::c_int;
}
unsafe fn check_counts() {
  let mut i: libc::c_int = 1i32;
  while i as libc::c_uint <= (*ptr_to_globals).u.Super.s_ninodes as libc::c_uint {
    if option_mask32 & OPT_w as libc::c_int as libc::c_uint != 0
      && (*((*ptr_to_globals).inode_buffer as *mut minix1_inode)
        .offset(-1)
        .offset(i as isize))
      .i_mode as libc::c_int
        != 0
      && minix_bit((*ptr_to_globals).inode_map, i as libc::c_uint) == 0
    {
      printf(
        b"Inode %d has non-zero mode. \x00" as *const u8 as *const libc::c_char,
        i,
      );
      if ask(b"Clear\x00" as *const u8 as *const libc::c_char, 1i32) != 0 {
        (*((*ptr_to_globals).inode_buffer as *mut minix1_inode)
          .offset(-1)
          .offset(i as isize))
        .i_mode = 0 as u16;
        (*ptr_to_globals).changed = 1i32 as smallint
      }
    }
    if *(*ptr_to_globals).inode_count.offset(i as isize) == 0 {
      if !(minix_bit((*ptr_to_globals).inode_map, i as libc::c_uint) == 0) {
        printf(
          b"Unused inode %d is marked as \'used\' in the bitmap. \x00" as *const u8
            as *const libc::c_char,
          i,
        );
        if ask(b"Clear\x00" as *const u8 as *const libc::c_char, 1i32) != 0 {
          minix_clrbit((*ptr_to_globals).inode_map, i as libc::c_uint);
        }
      }
    } else {
      if minix_bit((*ptr_to_globals).inode_map, i as libc::c_uint) == 0 {
        printf(
          b"Inode %d is used, but marked as \'unused\' in the bitmap. \x00" as *const u8
            as *const libc::c_char,
          i,
        );
        if ask(b"Set\x00" as *const u8 as *const libc::c_char, 1i32) != 0 {
          minix_setbit((*ptr_to_globals).inode_map, i as libc::c_uint);
        }
      }
      if (*((*ptr_to_globals).inode_buffer as *mut minix1_inode)
        .offset(-1)
        .offset(i as isize))
      .i_nlinks as libc::c_int
        != *(*ptr_to_globals).inode_count.offset(i as isize) as libc::c_int
      {
        printf(
          b"Inode %d (mode=%07o), i_nlinks=%d, counted=%d. \x00" as *const u8
            as *const libc::c_char,
          i,
          (*((*ptr_to_globals).inode_buffer as *mut minix1_inode)
            .offset(-1)
            .offset(i as isize))
          .i_mode as libc::c_int,
          (*((*ptr_to_globals).inode_buffer as *mut minix1_inode)
            .offset(-1)
            .offset(i as isize))
          .i_nlinks as libc::c_int,
          *(*ptr_to_globals).inode_count.offset(i as isize) as libc::c_int,
        );
        if ask(
          b"Set i_nlinks to count\x00" as *const u8 as *const libc::c_char,
          1i32,
        ) != 0
        {
          (*((*ptr_to_globals).inode_buffer as *mut minix1_inode)
            .offset(-1)
            .offset(i as isize))
          .i_nlinks = *(*ptr_to_globals).inode_count.offset(i as isize);
          (*ptr_to_globals).changed = 1i32 as smallint
        }
      }
    }
    i += 1
  }
  i = (*ptr_to_globals).u.Super.s_firstdatazone as libc::c_uint as libc::c_int;
  while (i as libc::c_uint)
    < (if (*ptr_to_globals).version2 as libc::c_int != 0 {
      (*ptr_to_globals).u.Super.s_zones
    } else {
      (*ptr_to_globals).u.Super.s_nzones as libc::c_uint
    })
  {
    if !((minix_bit(
      (*ptr_to_globals).zone_map,
      (i as libc::c_uint)
        .wrapping_sub((*ptr_to_globals).u.Super.s_firstdatazone as libc::c_uint)
        .wrapping_add(1i32 as libc::c_uint),
    ) != 0) as libc::c_int
      == *(*ptr_to_globals).zone_count.offset(i as isize) as libc::c_int)
    {
      if *(*ptr_to_globals).zone_count.offset(i as isize) == 0 {
        if !(bad_zone(i) != 0) {
          printf(
            b"Zone %d is marked \'in use\', but no file uses it. \x00" as *const u8
              as *const libc::c_char,
            i,
          );
          if ask(b"Unmark\x00" as *const u8 as *const libc::c_char, 1i32) != 0 {
            minix_clrbit(
              (*ptr_to_globals).zone_map,
              (i as libc::c_uint)
                .wrapping_sub((*ptr_to_globals).u.Super.s_firstdatazone as libc::c_uint)
                .wrapping_add(1i32 as libc::c_uint),
            );
          }
        }
      } else {
        printf(
          b"Zone %d: %sin use, counted=%d\n\x00" as *const u8 as *const libc::c_char,
          i,
          if minix_bit(
            (*ptr_to_globals).zone_map,
            (i as libc::c_uint)
              .wrapping_sub((*ptr_to_globals).u.Super.s_firstdatazone as libc::c_uint)
              .wrapping_add(1i32 as libc::c_uint),
          ) != 0
          {
            b"\x00" as *const u8 as *const libc::c_char
          } else {
            b"not \x00" as *const u8 as *const libc::c_char
          },
          *(*ptr_to_globals).zone_count.offset(i as isize) as libc::c_int,
        );
      }
    }
    i += 1
  }
}
unsafe fn check_counts2() {
  let mut i: libc::c_int = 0;
  i = 1i32;
  while i as libc::c_uint <= (*ptr_to_globals).u.Super.s_ninodes as libc::c_uint {
    if option_mask32 & OPT_w as libc::c_int as libc::c_uint != 0
      && (*((*ptr_to_globals).inode_buffer as *mut minix2_inode)
        .offset(-1)
        .offset(i as isize))
      .i_mode as libc::c_int
        != 0
      && minix_bit((*ptr_to_globals).inode_map, i as libc::c_uint) == 0
    {
      printf(
        b"Inode %d has non-zero mode. \x00" as *const u8 as *const libc::c_char,
        i,
      );
      if ask(b"Clear\x00" as *const u8 as *const libc::c_char, 1i32) != 0 {
        (*((*ptr_to_globals).inode_buffer as *mut minix2_inode)
          .offset(-1)
          .offset(i as isize))
        .i_mode = 0 as u16;
        (*ptr_to_globals).changed = 1i32 as smallint
      }
    }
    if *(*ptr_to_globals).inode_count.offset(i as isize) == 0 {
      if !(minix_bit((*ptr_to_globals).inode_map, i as libc::c_uint) == 0) {
        printf(
          b"Unused inode %d is marked as \'used\' in the bitmap. \x00" as *const u8
            as *const libc::c_char,
          i,
        );
        if ask(b"Clear\x00" as *const u8 as *const libc::c_char, 1i32) != 0 {
          minix_clrbit((*ptr_to_globals).inode_map, i as libc::c_uint);
        }
      }
    } else {
      if minix_bit((*ptr_to_globals).inode_map, i as libc::c_uint) == 0 {
        printf(
          b"Inode %d is used, but marked as \'unused\' in the bitmap. \x00" as *const u8
            as *const libc::c_char,
          i,
        );
        if ask(b"Set\x00" as *const u8 as *const libc::c_char, 1i32) != 0 {
          minix_setbit((*ptr_to_globals).inode_map, i as libc::c_uint);
        }
      }
      if (*((*ptr_to_globals).inode_buffer as *mut minix2_inode)
        .offset(-1)
        .offset(i as isize))
      .i_nlinks as libc::c_int
        != *(*ptr_to_globals).inode_count.offset(i as isize) as libc::c_int
      {
        printf(
          b"Inode %d (mode=%07o), i_nlinks=%d, counted=%d. \x00" as *const u8
            as *const libc::c_char,
          i,
          (*((*ptr_to_globals).inode_buffer as *mut minix2_inode)
            .offset(-1)
            .offset(i as isize))
          .i_mode as libc::c_int,
          (*((*ptr_to_globals).inode_buffer as *mut minix2_inode)
            .offset(-1)
            .offset(i as isize))
          .i_nlinks as libc::c_int,
          *(*ptr_to_globals).inode_count.offset(i as isize) as libc::c_int,
        );
        if ask(
          b"Set i_nlinks to count\x00" as *const u8 as *const libc::c_char,
          1i32,
        ) != 0
        {
          (*((*ptr_to_globals).inode_buffer as *mut minix2_inode)
            .offset(-1)
            .offset(i as isize))
          .i_nlinks = *(*ptr_to_globals).inode_count.offset(i as isize) as u16;
          (*ptr_to_globals).changed = 1i32 as smallint
        }
      }
    }
    i += 1
  }
  i = (*ptr_to_globals).u.Super.s_firstdatazone as libc::c_uint as libc::c_int;
  while (i as libc::c_uint)
    < (if (*ptr_to_globals).version2 as libc::c_int != 0 {
      (*ptr_to_globals).u.Super.s_zones
    } else {
      (*ptr_to_globals).u.Super.s_nzones as libc::c_uint
    })
  {
    if !((minix_bit(
      (*ptr_to_globals).zone_map,
      (i as libc::c_uint)
        .wrapping_sub((*ptr_to_globals).u.Super.s_firstdatazone as libc::c_uint)
        .wrapping_add(1i32 as libc::c_uint),
    ) != 0) as libc::c_int
      == *(*ptr_to_globals).zone_count.offset(i as isize) as libc::c_int)
    {
      if *(*ptr_to_globals).zone_count.offset(i as isize) == 0 {
        if !(bad_zone(i) != 0) {
          printf(
            b"Zone %d is marked \'in use\', but no file uses it. \x00" as *const u8
              as *const libc::c_char,
            i,
          );
          if ask(b"Unmark\x00" as *const u8 as *const libc::c_char, 1i32) != 0 {
            minix_clrbit(
              (*ptr_to_globals).zone_map,
              (i as libc::c_uint)
                .wrapping_sub((*ptr_to_globals).u.Super.s_firstdatazone as libc::c_uint)
                .wrapping_add(1i32 as libc::c_uint),
            );
          }
        }
      } else {
        printf(
          b"Zone %d: %sin use, counted=%d\n\x00" as *const u8 as *const libc::c_char,
          i,
          if minix_bit(
            (*ptr_to_globals).zone_map,
            (i as libc::c_uint)
              .wrapping_sub((*ptr_to_globals).u.Super.s_firstdatazone as libc::c_uint)
              .wrapping_add(1i32 as libc::c_uint),
          ) != 0
          {
            b"\x00" as *const u8 as *const libc::c_char
          } else {
            b"not \x00" as *const u8 as *const libc::c_char
          },
          *(*ptr_to_globals).zone_count.offset(i as isize) as libc::c_int,
        );
      }
    }
    i += 1
  }
}
unsafe fn check() {
  memset(
    (*ptr_to_globals).inode_count as *mut libc::c_void,
    0,
    (((*ptr_to_globals).u.Super.s_ninodes as libc::c_uint).wrapping_add(1i32 as libc::c_uint)
      as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
  );
  memset(
    (*ptr_to_globals).zone_count as *mut libc::c_void,
    0,
    ((if (*ptr_to_globals).version2 as libc::c_int != 0 {
      (*ptr_to_globals).u.Super.s_zones
    } else {
      (*ptr_to_globals).u.Super.s_nzones as libc::c_uint
    }) as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
  );
  check_zones(MINIX_ROOT_INO as libc::c_int as libc::c_uint);
  recursive_check(MINIX_ROOT_INO as libc::c_int as libc::c_uint);
  check_counts();
}
unsafe fn check2() {
  memset(
    (*ptr_to_globals).inode_count as *mut libc::c_void,
    0,
    (((*ptr_to_globals).u.Super.s_ninodes as libc::c_uint).wrapping_add(1i32 as libc::c_uint)
      as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
  );
  memset(
    (*ptr_to_globals).zone_count as *mut libc::c_void,
    0,
    ((if (*ptr_to_globals).version2 as libc::c_int != 0 {
      (*ptr_to_globals).u.Super.s_zones
    } else {
      (*ptr_to_globals).u.Super.s_nzones as libc::c_uint
    }) as libc::c_ulong)
      .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
  );
  check_zones2(MINIX_ROOT_INO as libc::c_int as libc::c_uint);
  recursive_check2(MINIX_ROOT_INO as libc::c_int as libc::c_uint);
  check_counts2();
}
pub unsafe fn fsck_minix_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut retcode: libc::c_int = 0;
  xfunc_error_retval = 8i32 as u8;
  let ref mut fresh7 = *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
    as *mut *mut globals);
  *fresh7 = crate::libbb::xfuncs_printf::xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong)
    as *mut globals;
  llvm_asm!("" : : : "memory" : "volatile");
  (*ptr_to_globals).dirsize = 16i32 as smallint;
  (*ptr_to_globals).namelen = 14i32 as smallint;
  (*ptr_to_globals).current_name[0] = '/' as i32 as libc::c_char;
  (*ptr_to_globals).name_component[0] =
    &mut *(*ptr_to_globals).current_name.as_mut_ptr().offset(0) as *mut libc::c_char;
  crate::libbb::getopt32::getopt32(
    argv,
    b"^larvsmf\x00=1:ar\x00" as *const u8 as *const libc::c_char,
  );
  argv = argv.offset(optind as isize);
  (*ptr_to_globals).device_name = *argv.offset(0);
  check_mount();
  if option_mask32 & (OPT_a as libc::c_int | OPT_r as libc::c_int) as libc::c_uint
    == OPT_r as libc::c_int as libc::c_uint
  {
    if isatty(0i32) == 0 || isatty(1i32) == 0 {
      die(b"need terminal for interactive repairs\x00" as *const u8 as *const libc::c_char);
    }
  }
  crate::libbb::xfuncs_printf::xmove_fd(
    crate::libbb::xfuncs_printf::xopen(
      (*ptr_to_globals).device_name,
      if option_mask32 & OPT_r as libc::c_int as libc::c_uint != 0 {
        0o2i32
      } else {
        0
      },
    ),
    dev_fd as libc::c_int,
  );
  /*sync(); paranoia? */
  read_superblock();
  /*
   * Determine whether or not we should continue with the checking.
   * This is based on the status of the filesystem valid and error
   * flags and whether or not the -f switch was specified on the
   * command line.
   */
  printf(
    b"%s: %s\n\x00" as *const u8 as *const libc::c_char,
    applet_name,
    bb_banner.as_ptr(),
  );
  if (*ptr_to_globals).u.Super.s_state as libc::c_int & MINIX_ERROR_FS as libc::c_int == 0
    && (*ptr_to_globals).u.Super.s_state as libc::c_int & MINIX_VALID_FS as libc::c_int != 0
    && option_mask32 & OPT_f as libc::c_int as libc::c_uint == 0
  {
    if option_mask32 & OPT_r as libc::c_int as libc::c_uint != 0 {
      printf(
        b"%s is clean, check is skipped\n\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).device_name,
      );
    }
    return 0;
  } else {
    if option_mask32 & OPT_f as libc::c_int as libc::c_uint != 0 {
      printf(
        b"Forcing filesystem check on %s\n\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).device_name,
      );
    } else if option_mask32 & OPT_r as libc::c_int as libc::c_uint != 0 {
      printf(
        b"Filesystem on %s is dirty, needs checking\n\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).device_name,
      );
    }
  }
  read_tables();
  if option_mask32 & (OPT_a as libc::c_int | OPT_r as libc::c_int) as libc::c_uint
    == OPT_r as libc::c_int as libc::c_uint
  {
    crate::libbb::xfuncs::set_termios_to_raw(0i32, &mut (*ptr_to_globals).sv_termios, 0);
    (*ptr_to_globals).termios_set = 1i32 as smallint
  }
  if (*ptr_to_globals).version2 != 0 {
    check_root2();
    check2();
  } else {
    check_root();
    check();
  }
  if option_mask32 & OPT_v as libc::c_int as libc::c_uint != 0 {
    let mut i: libc::c_int = 0;
    let mut free_cnt: libc::c_int = 0;
    i = 1i32;
    free_cnt = 0;
    while i as libc::c_uint <= (*ptr_to_globals).u.Super.s_ninodes as libc::c_uint {
      if minix_bit((*ptr_to_globals).inode_map, i as libc::c_uint) == 0 {
        free_cnt += 1
      }
      i += 1
    }
    printf(
      b"\n%6u inodes used (%u%%)\n\x00" as *const u8 as *const libc::c_char,
      ((*ptr_to_globals).u.Super.s_ninodes as libc::c_uint).wrapping_sub(free_cnt as libc::c_uint),
      (100i32 as libc::c_uint)
        .wrapping_mul(
          ((*ptr_to_globals).u.Super.s_ninodes as libc::c_uint)
            .wrapping_sub(free_cnt as libc::c_uint),
        )
        .wrapping_div((*ptr_to_globals).u.Super.s_ninodes as libc::c_uint),
    );
    i = (*ptr_to_globals).u.Super.s_firstdatazone as libc::c_uint as libc::c_int;
    free_cnt = 0;
    while (i as libc::c_uint)
      < (if (*ptr_to_globals).version2 as libc::c_int != 0 {
        (*ptr_to_globals).u.Super.s_zones
      } else {
        (*ptr_to_globals).u.Super.s_nzones as libc::c_uint
      })
    {
      if minix_bit(
        (*ptr_to_globals).zone_map,
        (i as libc::c_uint)
          .wrapping_sub((*ptr_to_globals).u.Super.s_firstdatazone as libc::c_uint)
          .wrapping_add(1i32 as libc::c_uint),
      ) == 0
      {
        free_cnt += 1
      }
      i += 1
    }
    printf(b"%6u zones used (%u%%)\n\n%6u regular files\n%6u directories\n%6u character device files\n%6u block device files\n%6u links\n%6u symbolic links\n------\n%6u files\n\x00"
                   as *const u8 as *const libc::c_char,
               (if (*ptr_to_globals).version2 as libc::c_int != 0 {
                    (*ptr_to_globals).u.Super.s_zones
                } else {
                    (*ptr_to_globals).u.Super.s_nzones as libc::c_uint
                }).wrapping_sub(free_cnt as libc::c_uint),
               (100i32 as
                    libc::c_uint).wrapping_mul((if (*ptr_to_globals).version2
                                                       as libc::c_int != 0 {
                                                    (*ptr_to_globals).u.Super.s_zones
                                                } else {
                                                    (*ptr_to_globals).u.Super.s_nzones
                                                        as libc::c_uint
                                                }).wrapping_sub(free_cnt as
                                                                    libc::c_uint)).wrapping_div(if (*ptr_to_globals).version2
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        !=
                                                                                                        0
                                                                                                    {
                                                                                                     (*ptr_to_globals).u.Super.s_zones
                                                                                                 } else {
                                                                                                     (*ptr_to_globals).u.Super.s_nzones
                                                                                                         as
                                                                                                         libc::c_uint
                                                                                                 }),
               (*ptr_to_globals).regular, (*ptr_to_globals).directory,
               (*ptr_to_globals).chardev, (*ptr_to_globals).blockdev,
               (*ptr_to_globals).links - 2i32 * (*ptr_to_globals).directory +
                   1i32, (*ptr_to_globals).symlinks,
               (*ptr_to_globals).total - 2i32 * (*ptr_to_globals).directory +
                   1i32);
  }
  if (*ptr_to_globals).changed != 0 {
    write_tables();
    puts(b"FILE SYSTEM HAS BEEN CHANGED\x00" as *const u8 as *const libc::c_char);
    sync();
  } else if option_mask32 & OPT_r as libc::c_int as libc::c_uint != 0 {
    write_superblock();
  }
  if option_mask32 & (OPT_a as libc::c_int | OPT_r as libc::c_int) as libc::c_uint
    == OPT_r as libc::c_int as libc::c_uint
  {
    crate::libbb::xfuncs::tcsetattr_stdin_TCSANOW(&mut (*ptr_to_globals).sv_termios);
  }
  if (*ptr_to_globals).changed != 0 {
    retcode += 3i32
  }
  if (*ptr_to_globals).errors_uncorrected != 0 {
    retcode += 4i32
  }
  return retcode;
}
