use crate::librb::signal::__sighandler_t;
use crate::librb::size_t;
use crate::librb::smallint;
use crate::librb::uoff_t;
use libc;



use libc::off_t;
use libc::ssize_t;
use libc::FILE;

extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
  #[no_mangle]
  static mut stderr: *mut FILE;
  #[no_mangle]
  fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn feof_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
  #[no_mangle]
  fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xlseek(fd: libc::c_int, offset: off_t, whence: libc::c_int) -> off_t;
  #[no_mangle]
  fn get_volume_size_in_bytes(
    fd: libc::c_int,
    override_0: *const libc::c_char,
    override_units: libc::c_uint,
    extend: libc::c_int,
  ) -> uoff_t;
  #[no_mangle]
  fn xwrite(fd: libc::c_int, buf: *const libc::c_void, count: size_t);
  #[no_mangle]
  fn fflush_all() -> libc::c_int;
  #[no_mangle]
  fn xfopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn xatoull(str: *const libc::c_char) -> libc::c_ulonglong;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  static mut msg_eol: *const libc::c_char;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn find_mount_point(name: *const libc::c_char, subdir_too: libc::c_int) -> *mut mntent;
  #[no_mangle]
  static ptr_to_globals: *mut globals;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mntent {
  pub mnt_fsname: *mut libc::c_char,
  pub mnt_dir: *mut libc::c_char,
  pub mnt_type: *mut libc::c_char,
  pub mnt_opts: *mut libc::c_char,
  pub mnt_freq: libc::c_int,
  pub mnt_passno: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub version2: smallint,
  pub device_name: *mut libc::c_char,
  pub total_blocks: u32,
  pub badblocks: libc::c_int,
  pub namelen: libc::c_int,
  pub dirsize: libc::c_int,
  pub magic: libc::c_int,
  pub inode_buffer: *mut libc::c_char,
  pub inode_map: *mut libc::c_char,
  pub zone_map: *mut libc::c_char,
  pub used_good_blocks: libc::c_int,
  pub req_nr_inodes: libc::c_ulong,
  pub currently_testing: libc::c_uint,
  pub root_block: [libc::c_char; 1024],
  pub u: C2RustUnnamed,
  pub boot_block_buffer: [libc::c_char; 512],
  pub good_blocks_table: [libc::c_ushort; 512],
  pub check_blocks_buffer: [libc::c_char; 16384],
  pub ind_block1: [libc::c_ushort; 512],
  pub dind_block1: [libc::c_ushort; 512],
  pub ind_block2: [libc::c_ulong; 256],
  pub dind_block2: [libc::c_ulong; 256],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
  pub superblock_buffer: [libc::c_char; 1024],
  pub SB: minix_superblock,
}

/*
 * minix superblock data on disk
 */
#[derive(Copy, Clone)]
#[repr(C)]
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
#[derive(Copy, Clone)]
#[repr(C)]
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
#[derive(Copy, Clone)]
#[repr(C)]
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
pub const INODE_SIZE2: C2RustUnnamed_0 = 64;
/* fs has errors */
pub const INODE_SIZE1: C2RustUnnamed_0 = 32;
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
pub const MINIX_BAD_INO: C2RustUnnamed_0 = 2;
pub const MINIX_ROOT_INO: C2RustUnnamed_0 = 1;
pub const BITS_PER_BLOCK: C2RustUnnamed_0 = 8192;
pub const BLOCK_SIZE: C2RustUnnamed_0 = 1024;

pub type C2RustUnnamed_1 = libc::c_uint;
pub const TEST_BUFFER_BLOCKS: C2RustUnnamed_1 = 16;
pub const MAX_GOOD_BLOCKS: C2RustUnnamed_1 = 512;

pub type C2RustUnnamed_2 = libc::c_uint;
pub const dev_fd: C2RustUnnamed_2 = 3;

#[inline(always)]
unsafe extern "C" fn xatoul(mut str: *const libc::c_char) -> libc::c_ulong {
  return xatoull(str) as libc::c_ulong;
}
#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
#[inline(always)]
unsafe extern "C" fn div_roundup(mut size: libc::c_uint, mut n: libc::c_uint) -> libc::c_uint {
  return size
    .wrapping_add(n)
    .wrapping_sub(1i32 as libc::c_uint)
    .wrapping_div(n);
}
/* Before you ask "where they come from?": */
/* setbit/clrbit are supplied by sys/param.h */
unsafe extern "C" fn minix_bit(mut a: *const libc::c_char, mut i: libc::c_uint) -> libc::c_int {
  return *a.offset((i >> 3i32) as isize) as libc::c_int & 1i32 << (i & 7i32 as libc::c_uint);
}
unsafe extern "C" fn minix_setbit(mut a: *mut libc::c_char, mut i: libc::c_uint) {
  let ref mut fresh0 = *a.offset(i.wrapping_div(8i32 as libc::c_uint) as isize);
  *fresh0 = (*fresh0 as libc::c_int | 1i32 << i.wrapping_rem(8i32 as libc::c_uint)) as libc::c_char;
}
unsafe extern "C" fn minix_clrbit(mut a: *mut libc::c_char, mut i: libc::c_uint) {
  let ref mut fresh1 = *a.offset(i.wrapping_div(8i32 as libc::c_uint) as isize);
  *fresh1 =
    (*fresh1 as libc::c_int & !(1i32 << i.wrapping_rem(8i32 as libc::c_uint))) as libc::c_char;
}
/* Note: do not assume 0/1, it is 0/nonzero */
/*#define inode_in_use(x) minix_bit(G.inode_map,(x))*/
/* return device size */
unsafe extern "C" fn write_tables() {
  /* Mark the superblock valid. */
  (*ptr_to_globals).u.SB.s_state =
    ((*ptr_to_globals).u.SB.s_state as libc::c_int | MINIX_VALID_FS as libc::c_int) as u16;
  (*ptr_to_globals).u.SB.s_state =
    ((*ptr_to_globals).u.SB.s_state as libc::c_int & !(MINIX_ERROR_FS as libc::c_int)) as u16;
  msg_eol = b"seek to 0 failed\x00" as *const u8 as *const libc::c_char;
  xlseek(dev_fd as libc::c_int, 0i32 as off_t, 0i32);
  msg_eol = b"can\'t clear boot sector\x00" as *const u8 as *const libc::c_char;
  xwrite(
    dev_fd as libc::c_int,
    (*ptr_to_globals).boot_block_buffer.as_mut_ptr() as *const libc::c_void,
    512i32 as size_t,
  );
  msg_eol = b"seek to BLOCK_SIZE failed\x00" as *const u8 as *const libc::c_char;
  xlseek(
    dev_fd as libc::c_int,
    BLOCK_SIZE as libc::c_int as off_t,
    0i32,
  );
  msg_eol = b"can\'t write superblock\x00" as *const u8 as *const libc::c_char;
  xwrite(
    dev_fd as libc::c_int,
    (*ptr_to_globals).u.superblock_buffer.as_mut_ptr() as *const libc::c_void,
    BLOCK_SIZE as libc::c_int as size_t,
  );
  msg_eol = b"can\'t write inode map\x00" as *const u8 as *const libc::c_char;
  xwrite(
    dev_fd as libc::c_int,
    (*ptr_to_globals).inode_map as *const libc::c_void,
    ((*ptr_to_globals).u.SB.s_imap_blocks as libc::c_int * BLOCK_SIZE as libc::c_int) as size_t,
  );
  msg_eol = b"can\'t write zone map\x00" as *const u8 as *const libc::c_char;
  xwrite(
    dev_fd as libc::c_int,
    (*ptr_to_globals).zone_map as *const libc::c_void,
    ((*ptr_to_globals).u.SB.s_zmap_blocks as libc::c_int * BLOCK_SIZE as libc::c_int) as size_t,
  );
  msg_eol = b"can\'t write inodes\x00" as *const u8 as *const libc::c_char;
  xwrite(
    dev_fd as libc::c_int,
    (*ptr_to_globals).inode_buffer as *const libc::c_void,
    div_roundup(
      (*ptr_to_globals).u.SB.s_ninodes as libc::c_uint,
      (if (*ptr_to_globals).version2 as libc::c_int != 0 {
        MINIX2_INODES_PER_BLOCK as libc::c_int
      } else {
        MINIX1_INODES_PER_BLOCK as libc::c_int
      }) as libc::c_uint,
    )
    .wrapping_mul(BLOCK_SIZE as libc::c_int as libc::c_uint) as size_t,
  );
  msg_eol = b"\n\x00" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn write_block(mut blk: libc::c_int, mut buffer: *mut libc::c_char) {
  xlseek(
    dev_fd as libc::c_int,
    (blk * BLOCK_SIZE as libc::c_int) as off_t,
    0i32,
  );
  xwrite(
    dev_fd as libc::c_int,
    buffer as *const libc::c_void,
    BLOCK_SIZE as libc::c_int as size_t,
  );
}
unsafe extern "C" fn get_free_block() -> libc::c_int {
  let mut blk: libc::c_int = 0;
  if (*ptr_to_globals).used_good_blocks + 1i32 >= MAX_GOOD_BLOCKS as libc::c_int {
    bb_simple_error_msg_and_die(b"too many bad blocks\x00" as *const u8 as *const libc::c_char);
  }
  if (*ptr_to_globals).used_good_blocks != 0 {
    blk = (*ptr_to_globals).good_blocks_table[((*ptr_to_globals).used_good_blocks - 1i32) as usize]
      as libc::c_int
      + 1i32
  } else {
    blk = (*ptr_to_globals).u.SB.s_firstdatazone as libc::c_int
  }
  while (blk as libc::c_uint)
    < (if (*ptr_to_globals).version2 as libc::c_int != 0 {
      (*ptr_to_globals).u.SB.s_zones
    } else {
      (*ptr_to_globals).u.SB.s_nzones as libc::c_uint
    })
    && minix_bit(
      (*ptr_to_globals).zone_map,
      (blk - (*ptr_to_globals).u.SB.s_firstdatazone as libc::c_int + 1i32) as libc::c_uint,
    ) != 0
  {
    blk += 1
  }
  if blk as libc::c_uint
    >= (if (*ptr_to_globals).version2 as libc::c_int != 0 {
      (*ptr_to_globals).u.SB.s_zones
    } else {
      (*ptr_to_globals).u.SB.s_nzones as libc::c_uint
    })
  {
    bb_simple_error_msg_and_die(b"not enough good blocks\x00" as *const u8 as *const libc::c_char);
  }
  (*ptr_to_globals).good_blocks_table[(*ptr_to_globals).used_good_blocks as usize] =
    blk as libc::c_ushort;
  (*ptr_to_globals).used_good_blocks += 1;
  return blk;
}
unsafe extern "C" fn mark_good_blocks() {
  let mut blk: libc::c_int = 0;
  blk = 0i32;
  while blk < (*ptr_to_globals).used_good_blocks {
    minix_setbit(
      (*ptr_to_globals).zone_map,
      ((*ptr_to_globals).good_blocks_table[blk as usize] as libc::c_int
        - (*ptr_to_globals).u.SB.s_firstdatazone as libc::c_int
        + 1i32) as libc::c_uint,
    );
    blk += 1
  }
}
unsafe extern "C" fn next(mut zone: libc::c_int) -> libc::c_int {
  if zone == 0 {
    zone = (*ptr_to_globals).u.SB.s_firstdatazone as libc::c_int - 1i32
  }
  loop {
    zone += 1;
    if !((zone as libc::c_uint)
      < (if (*ptr_to_globals).version2 as libc::c_int != 0 {
        (*ptr_to_globals).u.SB.s_zones
      } else {
        (*ptr_to_globals).u.SB.s_nzones as libc::c_uint
      }))
    {
      break;
    }
    if minix_bit(
      (*ptr_to_globals).zone_map,
      (zone - (*ptr_to_globals).u.SB.s_firstdatazone as libc::c_int + 1i32) as libc::c_uint,
    ) != 0
    {
      return zone;
    }
  }
  return 0i32;
}
unsafe extern "C" fn make_bad_inode() {
  let mut current_block: u64;
  let mut inode: *mut minix1_inode = &mut *((*ptr_to_globals).inode_buffer as *mut minix1_inode)
    .offset(-1)
    .offset(MINIX_BAD_INO as libc::c_int as isize)
    as *mut minix1_inode;
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  let mut zone: libc::c_int = 0;
  let mut ind: libc::c_int = 0i32;
  let mut dind: libc::c_int = 0i32;
  /* moved to globals to reduce stack usage
  unsigned short ind_block[BLOCK_SIZE >> 1];
  unsigned short dind_block[BLOCK_SIZE >> 1];
  */
  if (*ptr_to_globals).badblocks == 0 {
    return;
  }
  minix_setbit(
    (*ptr_to_globals).inode_map,
    MINIX_BAD_INO as libc::c_int as libc::c_uint,
  );
  (*inode).i_nlinks = 1i32 as u8;
  /* BTW, setting this makes all images different */
  /* it's harder to check for bugs then - diff isn't helpful :(... */
  (*inode).i_time = 0i32 as u32;
  (*inode).i_mode = (0o100000i32 + 0i32) as u16;
  (*inode).i_size = ((*ptr_to_globals).badblocks * BLOCK_SIZE as libc::c_int) as u32;
  zone = next(0i32);
  i = 0i32;
  loop {
    if !(i < 7i32) {
      current_block = 2979737022853876585;
      break;
    }
    (*inode).i_zone[i as usize] = zone as u16;
    zone = next(zone);
    if zone == 0 {
      current_block = 5743758918637478998;
      break;
    }
    i += 1
  }
  match current_block {
    2979737022853876585 => {
      ind = get_free_block();
      (*inode).i_zone[7] = ind as u16;
      memset(
        (*ptr_to_globals).ind_block1.as_mut_ptr() as *mut libc::c_void,
        0i32,
        BLOCK_SIZE as libc::c_int as libc::c_ulong,
      );
      i = 0i32;
      loop {
        if !(i < 512i32) {
          current_block = 13242334135786603907;
          break;
        }
        (*ptr_to_globals).ind_block1[i as usize] = zone as libc::c_ushort;
        zone = next(zone);
        if zone == 0 {
          current_block = 5743758918637478998;
          break;
        }
        i += 1
      }
      match current_block {
        5743758918637478998 => {}
        _ => {
          dind = get_free_block();
          (*inode).i_zone[8] = dind as u16;
          memset(
            (*ptr_to_globals).dind_block1.as_mut_ptr() as *mut libc::c_void,
            0i32,
            BLOCK_SIZE as libc::c_int as libc::c_ulong,
          );
          i = 0i32;
          's_108: loop {
            if !(i < 512i32) {
              current_block = 11459959175219260272;
              break;
            }
            write_block(
              ind,
              (*ptr_to_globals).ind_block1.as_mut_ptr() as *mut libc::c_char,
            );
            ind = get_free_block();
            (*ptr_to_globals).dind_block1[i as usize] = ind as libc::c_ushort;
            memset(
              (*ptr_to_globals).ind_block1.as_mut_ptr() as *mut libc::c_void,
              0i32,
              BLOCK_SIZE as libc::c_int as libc::c_ulong,
            );
            j = 0i32;
            while j < 512i32 {
              (*ptr_to_globals).ind_block1[j as usize] = zone as libc::c_ushort;
              zone = next(zone);
              if zone == 0 {
                current_block = 5743758918637478998;
                break 's_108;
              }
              j += 1
            }
            i += 1
          }
          match current_block {
            5743758918637478998 => {}
            _ => {
              bb_simple_error_msg_and_die(
                b"too many bad blocks\x00" as *const u8 as *const libc::c_char,
              );
            }
          }
        }
      }
    }
    _ => {}
  }
  if ind != 0 {
    write_block(
      ind,
      (*ptr_to_globals).ind_block1.as_mut_ptr() as *mut libc::c_char,
    );
  }
  if dind != 0 {
    write_block(
      dind,
      (*ptr_to_globals).dind_block1.as_mut_ptr() as *mut libc::c_char,
    );
  };
}
unsafe extern "C" fn make_bad_inode2() {
  let mut current_block: u64;
  let mut inode: *mut minix2_inode = &mut *((*ptr_to_globals).inode_buffer as *mut minix2_inode)
    .offset(-1)
    .offset(MINIX_BAD_INO as libc::c_int as isize)
    as *mut minix2_inode;
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  let mut zone: libc::c_int = 0;
  let mut ind: libc::c_int = 0i32;
  let mut dind: libc::c_int = 0i32;
  /* moved to globals to reduce stack usage
  unsigned long ind_block[BLOCK_SIZE >> 2];
  unsigned long dind_block[BLOCK_SIZE >> 2];
  */
  if (*ptr_to_globals).badblocks == 0 {
    return;
  }
  minix_setbit(
    (*ptr_to_globals).inode_map,
    MINIX_BAD_INO as libc::c_int as libc::c_uint,
  );
  (*inode).i_nlinks = 1i32 as u16;
  (*inode).i_ctime = 0i32 as u32;
  (*inode).i_mtime = (*inode).i_ctime;
  (*inode).i_atime = (*inode).i_mtime;
  (*inode).i_mode = (0o100000i32 + 0i32) as u16;
  (*inode).i_size = ((*ptr_to_globals).badblocks * BLOCK_SIZE as libc::c_int) as u32;
  zone = next(0i32);
  i = 0i32;
  loop {
    if !(i < 7i32) {
      current_block = 2979737022853876585;
      break;
    }
    (*inode).i_zone[i as usize] = zone as u32;
    zone = next(zone);
    if zone == 0 {
      current_block = 6635813614084111333;
      break;
    }
    i += 1
  }
  match current_block {
    2979737022853876585 => {
      ind = get_free_block();
      (*inode).i_zone[7] = ind as u32;
      memset(
        (*ptr_to_globals).ind_block2.as_mut_ptr() as *mut libc::c_void,
        0i32,
        BLOCK_SIZE as libc::c_int as libc::c_ulong,
      );
      i = 0i32;
      loop {
        if !(i < 256i32) {
          current_block = 13242334135786603907;
          break;
        }
        (*ptr_to_globals).ind_block2[i as usize] = zone as libc::c_ulong;
        zone = next(zone);
        if zone == 0 {
          current_block = 6635813614084111333;
          break;
        }
        i += 1
      }
      match current_block {
        6635813614084111333 => {}
        _ => {
          dind = get_free_block();
          (*inode).i_zone[8] = dind as u32;
          memset(
            (*ptr_to_globals).dind_block2.as_mut_ptr() as *mut libc::c_void,
            0i32,
            BLOCK_SIZE as libc::c_int as libc::c_ulong,
          );
          i = 0i32;
          's_108: loop {
            if !(i < 256i32) {
              current_block = 11459959175219260272;
              break;
            }
            write_block(
              ind,
              (*ptr_to_globals).ind_block2.as_mut_ptr() as *mut libc::c_char,
            );
            ind = get_free_block();
            (*ptr_to_globals).dind_block2[i as usize] = ind as libc::c_ulong;
            memset(
              (*ptr_to_globals).ind_block2.as_mut_ptr() as *mut libc::c_void,
              0i32,
              BLOCK_SIZE as libc::c_int as libc::c_ulong,
            );
            j = 0i32;
            while j < 256i32 {
              (*ptr_to_globals).ind_block2[j as usize] = zone as libc::c_ulong;
              zone = next(zone);
              if zone == 0 {
                current_block = 6635813614084111333;
                break 's_108;
              }
              j += 1
            }
            i += 1
          }
          match current_block {
            6635813614084111333 => {}
            _ => {
              /* Could make triple indirect block here */
              bb_simple_error_msg_and_die(
                b"too many bad blocks\x00" as *const u8 as *const libc::c_char,
              );
            }
          }
        }
      }
    }
    _ => {}
  }
  if ind != 0 {
    write_block(
      ind,
      (*ptr_to_globals).ind_block2.as_mut_ptr() as *mut libc::c_char,
    );
  }
  if dind != 0 {
    write_block(
      dind,
      (*ptr_to_globals).dind_block2.as_mut_ptr() as *mut libc::c_char,
    );
  };
}
unsafe extern "C" fn make_root_inode() {
  let mut inode: *mut minix1_inode = &mut *((*ptr_to_globals).inode_buffer as *mut minix1_inode)
    .offset(-1)
    .offset(MINIX_ROOT_INO as libc::c_int as isize)
    as *mut minix1_inode;
  minix_setbit(
    (*ptr_to_globals).inode_map,
    MINIX_ROOT_INO as libc::c_int as libc::c_uint,
  );
  (*inode).i_zone[0] = get_free_block() as u16;
  (*inode).i_nlinks = 2i32 as u8;
  (*inode).i_time = 0i32 as u32;
  if (*ptr_to_globals).badblocks != 0 {
    (*inode).i_size = (3i32 * (*ptr_to_globals).dirsize) as u32
  } else {
    (*ptr_to_globals).root_block[(2i32 * (*ptr_to_globals).dirsize) as usize] =
      '\u{0}' as i32 as libc::c_char;
    (*ptr_to_globals).root_block[(2i32 * (*ptr_to_globals).dirsize + 1i32) as usize] =
      '\u{0}' as i32 as libc::c_char;
    (*inode).i_size = (2i32 * (*ptr_to_globals).dirsize) as u32
  }
  (*inode).i_mode = (0o40000i32 + 0o755i32) as u16;
  (*inode).i_uid = 0i32 as u16;
  if (*inode).i_uid != 0 {
    (*inode).i_gid = 0i32 as u8
  }
  write_block(
    (*inode).i_zone[0] as libc::c_int,
    (*ptr_to_globals).root_block.as_mut_ptr(),
  );
}
unsafe extern "C" fn make_root_inode2() {
  let mut inode: *mut minix2_inode = &mut *((*ptr_to_globals).inode_buffer as *mut minix2_inode)
    .offset(-1)
    .offset(MINIX_ROOT_INO as libc::c_int as isize)
    as *mut minix2_inode;
  minix_setbit(
    (*ptr_to_globals).inode_map,
    MINIX_ROOT_INO as libc::c_int as libc::c_uint,
  );
  (*inode).i_zone[0] = get_free_block() as u32;
  (*inode).i_nlinks = 2i32 as u16;
  (*inode).i_ctime = 0i32 as u32;
  (*inode).i_mtime = (*inode).i_ctime;
  (*inode).i_atime = (*inode).i_mtime;
  if (*ptr_to_globals).badblocks != 0 {
    (*inode).i_size = (3i32 * (*ptr_to_globals).dirsize) as u32
  } else {
    (*ptr_to_globals).root_block[(2i32 * (*ptr_to_globals).dirsize) as usize] =
      '\u{0}' as i32 as libc::c_char;
    (*ptr_to_globals).root_block[(2i32 * (*ptr_to_globals).dirsize + 1i32) as usize] =
      '\u{0}' as i32 as libc::c_char;
    (*inode).i_size = (2i32 * (*ptr_to_globals).dirsize) as u32
  }
  (*inode).i_mode = (0o40000i32 + 0o755i32) as u16;
  (*inode).i_uid = 0i32 as u16;
  if (*inode).i_uid != 0 {
    (*inode).i_gid = 0i32 as u16
  }
  write_block(
    (*inode).i_zone[0] as libc::c_int,
    (*ptr_to_globals).root_block.as_mut_ptr(),
  );
}
/*
 * Perform a test of a block; return the number of
 * blocks readable.
 */
unsafe extern "C" fn do_check(
  mut buffer: *mut libc::c_char,
  mut try_0: size_t,
  mut current_block: libc::c_uint,
) -> size_t {
  let mut got: ssize_t = 0;
  /* Seek to the correct loc. */
  msg_eol = b"seek failed during testing of blocks\x00" as *const u8 as *const libc::c_char;
  xlseek(
    dev_fd as libc::c_int,
    current_block.wrapping_mul(BLOCK_SIZE as libc::c_int as libc::c_uint) as off_t,
    0i32,
  );
  msg_eol = b"\n\x00" as *const u8 as *const libc::c_char;
  /* Try the read */
  got = read(
    dev_fd as libc::c_int,
    buffer as *mut libc::c_void,
    try_0.wrapping_mul(BLOCK_SIZE as libc::c_int as libc::c_ulong),
  );
  if got < 0 {
    got = 0
  }
  try_0 = (got as size_t).wrapping_div(BLOCK_SIZE as libc::c_int as libc::c_ulong);
  if got & (BLOCK_SIZE - 1) as isize != 0 {
    fprintf(
      stderr,
      b"Short read at block %u\n\x00" as *const u8 as *const libc::c_char,
      (current_block as libc::c_ulong).wrapping_add(try_0) as libc::c_uint,
    );
  }
  return try_0;
}
unsafe extern "C" fn alarm_intr(mut _alnum: libc::c_int) {
  if (*ptr_to_globals).currently_testing
    >= (if (*ptr_to_globals).version2 as libc::c_int != 0 {
      (*ptr_to_globals).u.SB.s_zones
    } else {
      (*ptr_to_globals).u.SB.s_nzones as libc::c_uint
    })
  {
    return;
  }
  signal(
    14i32,
    Some(alarm_intr as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  alarm(5i32 as libc::c_uint);
  if (*ptr_to_globals).currently_testing == 0 {
    return;
  }
  printf(
    b"%d ...\x00" as *const u8 as *const libc::c_char,
    (*ptr_to_globals).currently_testing,
  );
  fflush_all();
}
unsafe extern "C" fn check_blocks() {
  let mut try_0: size_t = 0;
  let mut got: size_t = 0;
  (*ptr_to_globals).currently_testing = 0i32 as libc::c_uint;
  signal(
    14i32,
    Some(alarm_intr as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  alarm(5i32 as libc::c_uint);
  while (*ptr_to_globals).currently_testing
    < (if (*ptr_to_globals).version2 as libc::c_int != 0 {
      (*ptr_to_globals).u.SB.s_zones
    } else {
      (*ptr_to_globals).u.SB.s_nzones as libc::c_uint
    })
  {
    msg_eol = b"seek failed in check_blocks\x00" as *const u8 as *const libc::c_char;
    xlseek(
      dev_fd as libc::c_int,
      (*ptr_to_globals)
        .currently_testing
        .wrapping_mul(BLOCK_SIZE as libc::c_int as libc::c_uint) as off_t,
      0i32,
    );
    msg_eol = b"\n\x00" as *const u8 as *const libc::c_char;
    try_0 = TEST_BUFFER_BLOCKS as libc::c_int as size_t;
    if ((*ptr_to_globals).currently_testing as libc::c_ulong).wrapping_add(try_0)
      > (if (*ptr_to_globals).version2 as libc::c_int != 0 {
        (*ptr_to_globals).u.SB.s_zones
      } else {
        (*ptr_to_globals).u.SB.s_nzones as libc::c_uint
      }) as libc::c_ulong
    {
      try_0 = (if (*ptr_to_globals).version2 as libc::c_int != 0 {
        (*ptr_to_globals).u.SB.s_zones
      } else {
        (*ptr_to_globals).u.SB.s_nzones as libc::c_uint
      })
      .wrapping_sub((*ptr_to_globals).currently_testing) as size_t
    }
    got = do_check(
      (*ptr_to_globals).check_blocks_buffer.as_mut_ptr(),
      try_0,
      (*ptr_to_globals).currently_testing,
    );
    (*ptr_to_globals).currently_testing = ((*ptr_to_globals).currently_testing as libc::c_ulong)
      .wrapping_add(got) as libc::c_uint as libc::c_uint;
    if got == try_0 {
      continue;
    }
    if (*ptr_to_globals).currently_testing < (*ptr_to_globals).u.SB.s_firstdatazone as libc::c_uint
    {
      bb_simple_error_msg_and_die(
        b"bad blocks before data-area: cannot make fs\x00" as *const u8 as *const libc::c_char,
      );
    }
    minix_setbit(
      (*ptr_to_globals).zone_map,
      (*ptr_to_globals)
        .currently_testing
        .wrapping_sub((*ptr_to_globals).u.SB.s_firstdatazone as libc::c_uint)
        .wrapping_add(1i32 as libc::c_uint),
    );
    (*ptr_to_globals).badblocks += 1;
    (*ptr_to_globals).currently_testing = (*ptr_to_globals).currently_testing.wrapping_add(1)
  }
  alarm(0i32 as libc::c_uint);
  printf(
    b"%d bad block(s)\n\x00" as *const u8 as *const libc::c_char,
    (*ptr_to_globals).badblocks,
  );
}
unsafe extern "C" fn get_list_blocks(mut filename: *mut libc::c_char) {
  let mut listfile: *mut FILE = 0 as *mut FILE;
  let mut blockno: libc::c_ulong = 0;
  listfile = xfopen_for_read(filename);
  while feof_unlocked(listfile) == 0 {
    fscanf(
      listfile,
      b"%lu\n\x00" as *const u8 as *const libc::c_char,
      &mut blockno as *mut libc::c_ulong,
    );
    minix_setbit(
      (*ptr_to_globals).zone_map,
      blockno
        .wrapping_sub((*ptr_to_globals).u.SB.s_firstdatazone as libc::c_ulong)
        .wrapping_add(1i32 as libc::c_ulong) as libc::c_uint,
    );
    (*ptr_to_globals).badblocks += 1
  }
  printf(
    b"%d bad block(s)\n\x00" as *const u8 as *const libc::c_char,
    (*ptr_to_globals).badblocks,
  );
}
unsafe extern "C" fn setup_tables() {
  let mut current_block: u64;
  let mut inodes: libc::c_ulong = 0;
  let mut norm_firstzone: libc::c_uint = 0;
  let mut sb_zmaps: libc::c_uint = 0;
  let mut i: libc::c_uint = 0;
  /* memset(G.u.superblock_buffer, 0, BLOCK_SIZE); */
  /* memset(G.boot_block_buffer, 0, 512); */
  (*ptr_to_globals).u.SB.s_magic = (*ptr_to_globals).magic as u16;
  (*ptr_to_globals).u.SB.s_log_zone_size = 0i32 as u16;
  (*ptr_to_globals).u.SB.s_max_size = if (*ptr_to_globals).version2 as libc::c_int != 0 {
    0x7fffffffi32
  } else {
    (7i32 + 512i32 + 512i32 * 512i32) * 1024i32
  } as u32;
  if (*ptr_to_globals).version2 != 0 {
    (*ptr_to_globals).u.SB.s_zones = (*ptr_to_globals).total_blocks
  } else {
    (*ptr_to_globals).u.SB.s_nzones = (*ptr_to_globals).total_blocks as u16
  }
  /* some magic nrs: 1 inode / 3 blocks */
  if (*ptr_to_globals).req_nr_inodes == 0i32 as libc::c_ulong {
    inodes = (*ptr_to_globals)
      .total_blocks
      .wrapping_div(3i32 as libc::c_uint) as libc::c_ulong
  } else {
    inodes = (*ptr_to_globals).req_nr_inodes
  }
  /* Round up inode count to fill block size */
  if (*ptr_to_globals).version2 != 0 {
    inodes = inodes
      .wrapping_add(MINIX2_INODES_PER_BLOCK as libc::c_int as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong)
      & !(MINIX2_INODES_PER_BLOCK as libc::c_int - 1i32) as libc::c_ulong
  } else {
    inodes = inodes
      .wrapping_add(MINIX1_INODES_PER_BLOCK as libc::c_int as libc::c_ulong)
      .wrapping_sub(1i32 as libc::c_ulong)
      & !(MINIX1_INODES_PER_BLOCK as libc::c_int - 1i32) as libc::c_ulong
  }
  if inodes > 65535i32 as libc::c_ulong {
    inodes = 65535i32 as libc::c_ulong
  }
  (*ptr_to_globals).u.SB.s_ninodes = inodes as u16;
  (*ptr_to_globals).u.SB.s_imap_blocks = div_roundup(
    ((*ptr_to_globals).u.SB.s_ninodes as libc::c_int + 1i32) as libc::c_uint,
    BITS_PER_BLOCK as libc::c_int as libc::c_uint,
  ) as u16;
  /* Real bad hack but overwise mkfs.minix can be thrown
   * in infinite loop...
   * try:
   * dd if=/dev/zero of=test.fs count=10 bs=1024
   * mkfs.minix -i 200 test.fs
   */
  /* This code is not insane: NORM_FIRSTZONE is not a constant,
   * it is calculated from SB_INODES, SB_IMAPS and SB_ZMAPS */
  i = 999i32 as libc::c_uint;
  (*ptr_to_globals).u.SB.s_zmap_blocks = 0i32 as u16;
  loop {
    norm_firstzone = ((2i32
      + (*ptr_to_globals).u.SB.s_imap_blocks as libc::c_int
      + (*ptr_to_globals).u.SB.s_zmap_blocks as libc::c_int) as libc::c_uint)
      .wrapping_add(div_roundup(
        (*ptr_to_globals).u.SB.s_ninodes as libc::c_uint,
        (if (*ptr_to_globals).version2 as libc::c_int != 0 {
          MINIX2_INODES_PER_BLOCK as libc::c_int
        } else {
          MINIX1_INODES_PER_BLOCK as libc::c_int
        }) as libc::c_uint,
      ));
    sb_zmaps = div_roundup(
      (*ptr_to_globals)
        .total_blocks
        .wrapping_sub(norm_firstzone)
        .wrapping_add(1i32 as libc::c_uint),
      BITS_PER_BLOCK as libc::c_int as libc::c_uint,
    );
    if (*ptr_to_globals).u.SB.s_zmap_blocks as libc::c_uint == sb_zmaps {
      current_block = 6633475714167083212;
      break;
    }
    (*ptr_to_globals).u.SB.s_zmap_blocks = sb_zmaps as u16;
    i = i.wrapping_sub(1);
    if !(i != 0) {
      current_block = 1109700713171191020;
      break;
    }
    /* new SB_ZMAPS, need to recalc NORM_FIRSTZONE */
  }
  match current_block {
    1109700713171191020 => {
      bb_simple_error_msg_and_die(
        b"incompatible size/inode count, try different -i N\x00" as *const u8
          as *const libc::c_char,
      );
    }
    _ => {
      (*ptr_to_globals).u.SB.s_firstdatazone = norm_firstzone as u16;
      (*ptr_to_globals).inode_map = xmalloc(
        ((*ptr_to_globals).u.SB.s_imap_blocks as libc::c_int * BLOCK_SIZE as libc::c_int) as size_t,
      ) as *mut libc::c_char;
      (*ptr_to_globals).zone_map = xmalloc(
        ((*ptr_to_globals).u.SB.s_zmap_blocks as libc::c_int * BLOCK_SIZE as libc::c_int) as size_t,
      ) as *mut libc::c_char;
      memset(
        (*ptr_to_globals).inode_map as *mut libc::c_void,
        0xffi32,
        ((*ptr_to_globals).u.SB.s_imap_blocks as libc::c_int * BLOCK_SIZE as libc::c_int)
          as libc::c_ulong,
      );
      memset(
        (*ptr_to_globals).zone_map as *mut libc::c_void,
        0xffi32,
        ((*ptr_to_globals).u.SB.s_zmap_blocks as libc::c_int * BLOCK_SIZE as libc::c_int)
          as libc::c_ulong,
      );
      i = (*ptr_to_globals).u.SB.s_firstdatazone as libc::c_uint;
      while i
        < (if (*ptr_to_globals).version2 as libc::c_int != 0 {
          (*ptr_to_globals).u.SB.s_zones
        } else {
          (*ptr_to_globals).u.SB.s_nzones as libc::c_uint
        })
      {
        minix_clrbit(
          (*ptr_to_globals).zone_map,
          i.wrapping_sub((*ptr_to_globals).u.SB.s_firstdatazone as libc::c_uint)
            .wrapping_add(1i32 as libc::c_uint),
        );
        i = i.wrapping_add(1)
      }
      i = MINIX_ROOT_INO as libc::c_int as libc::c_uint;
      while i <= (*ptr_to_globals).u.SB.s_ninodes as libc::c_uint {
        minix_clrbit((*ptr_to_globals).inode_map, i);
        i = i.wrapping_add(1)
      }
      (*ptr_to_globals).inode_buffer = xzalloc(
        div_roundup(
          (*ptr_to_globals).u.SB.s_ninodes as libc::c_uint,
          (if (*ptr_to_globals).version2 as libc::c_int != 0 {
            MINIX2_INODES_PER_BLOCK as libc::c_int
          } else {
            MINIX1_INODES_PER_BLOCK as libc::c_int
          }) as libc::c_uint,
        )
        .wrapping_mul(BLOCK_SIZE as libc::c_int as libc::c_uint) as size_t,
      ) as *mut libc::c_char;
      printf(
        b"%lu inodes\n\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).u.SB.s_ninodes as libc::c_ulong,
      );
      printf(
        b"%lu blocks\n\x00" as *const u8 as *const libc::c_char,
        if (*ptr_to_globals).version2 as libc::c_int != 0 {
          (*ptr_to_globals).u.SB.s_zones
        } else {
          (*ptr_to_globals).u.SB.s_nzones as libc::c_uint
        } as libc::c_ulong,
      );
      printf(
        b"Firstdatazone=%lu (%lu)\n\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).u.SB.s_firstdatazone as libc::c_ulong,
        norm_firstzone as libc::c_ulong,
      );
      printf(
        b"Zonesize=%u\n\x00" as *const u8 as *const libc::c_char,
        (BLOCK_SIZE as libc::c_int) << (*ptr_to_globals).u.SB.s_log_zone_size as libc::c_int,
      );
      printf(
        b"Maxsize=%lu\n\x00" as *const u8 as *const libc::c_char,
        (*ptr_to_globals).u.SB.s_max_size as libc::c_ulong,
      );
      return;
    }
  };
}
#[no_mangle]
pub unsafe extern "C" fn mkfs_minix_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opt: libc::c_uint = 0;
  let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut str_i: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut listfile: *mut libc::c_char = 0 as *mut libc::c_char;
  let ref mut fresh2 = *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
    as *mut *mut globals);
  *fresh2 = xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong) as *mut globals;
  asm!("" : : : "memory" : "volatile");
  /* default (changed to 30, per Linus's suggestion, Sun Nov 21 08:05:07 1993) */
  (*ptr_to_globals).namelen = 30i32;
  (*ptr_to_globals).dirsize = 32i32;
  (*ptr_to_globals).magic = MINIX1_SUPER_MAGIC2 as libc::c_int;
  if INODE_SIZE1 as libc::c_int * MINIX1_INODES_PER_BLOCK as libc::c_int
    != BLOCK_SIZE as libc::c_int
  {
    bb_simple_error_msg_and_die(b"bad inode size\x00" as *const u8 as *const libc::c_char);
  }
  if INODE_SIZE2 as libc::c_int * MINIX2_INODES_PER_BLOCK as libc::c_int
    != BLOCK_SIZE as libc::c_int
  {
    bb_simple_error_msg_and_die(b"bad inode size\x00" as *const u8 as *const libc::c_char);
  }
  opt = getopt32(
    argv,
    b"ci:l:n:+v\x00" as *const u8 as *const libc::c_char,
    &mut str_i as *mut *mut libc::c_char,
    &mut listfile as *mut *mut libc::c_char,
    &mut (*ptr_to_globals).namelen as *mut libc::c_int,
  );
  argv = argv.offset(optind as isize);
  //if (opt & 1) -c
  if opt & 2i32 as libc::c_uint != 0 {
    (*ptr_to_globals).req_nr_inodes = xatoul(str_i)
  } // -i
    //if (opt & 4) -l
  if opt & 8i32 as libc::c_uint != 0 {
    // -n
    if (*ptr_to_globals).namelen == 14i32 {
      (*ptr_to_globals).magic = MINIX1_SUPER_MAGIC as libc::c_int
    } else if (*ptr_to_globals).namelen == 30i32 {
      (*ptr_to_globals).magic = MINIX1_SUPER_MAGIC2 as libc::c_int
    } else {
      bb_show_usage();
    }
    (*ptr_to_globals).dirsize = (*ptr_to_globals).namelen + 2i32
  }
  if opt & 0x10i32 as libc::c_uint != 0 {
    // -v
    (*ptr_to_globals).version2 = 1i32 as smallint
  }
  (*ptr_to_globals).device_name = *argv.offset(0);
  if (*ptr_to_globals).device_name.is_null() {
    bb_show_usage();
  }
  /* Check if it is mounted */
  if !find_mount_point((*ptr_to_globals).device_name, 0i32).is_null() {
    bb_simple_error_msg_and_die(
      b"can\'t format mounted filesystem\x00" as *const u8 as *const libc::c_char,
    );
  }
  xmove_fd(
    xopen((*ptr_to_globals).device_name, 0o2i32),
    dev_fd as libc::c_int,
  );
  (*ptr_to_globals).total_blocks = get_volume_size_in_bytes(
    dev_fd as libc::c_int,
    *argv.offset(1),
    1024i32 as libc::c_uint,
    1i32,
  )
  .wrapping_div(1024i32 as libc::c_ulong) as u32;
  if (*ptr_to_globals).total_blocks < 10i32 as libc::c_uint {
    bb_simple_error_msg_and_die(
      b"must have at least 10 blocks\x00" as *const u8 as *const libc::c_char,
    );
  }
  if (*ptr_to_globals).version2 != 0 {
    (*ptr_to_globals).magic = MINIX2_SUPER_MAGIC2 as libc::c_int;
    if (*ptr_to_globals).namelen == 14i32 {
      (*ptr_to_globals).magic = MINIX2_SUPER_MAGIC as libc::c_int
    }
  } else if (*ptr_to_globals).total_blocks > 65535i32 as libc::c_uint {
    (*ptr_to_globals).total_blocks = 65535i32 as u32
  }
  tmp = (*ptr_to_globals).root_block.as_mut_ptr();
  *(tmp as *mut libc::c_short) = 1i32 as libc::c_short;
  strcpy(tmp.offset(2), b".\x00" as *const u8 as *const libc::c_char);
  tmp = tmp.offset((*ptr_to_globals).dirsize as isize);
  *(tmp as *mut libc::c_short) = 1i32 as libc::c_short;
  strcpy(tmp.offset(2), b"..\x00" as *const u8 as *const libc::c_char);
  tmp = tmp.offset((*ptr_to_globals).dirsize as isize);
  *(tmp as *mut libc::c_short) = 2i32 as libc::c_short;
  strcpy(
    tmp.offset(2),
    b".badblocks\x00" as *const u8 as *const libc::c_char,
  );
  setup_tables();
  if opt & 1i32 as libc::c_uint != 0 {
    // -c ?
    check_blocks();
  } else if !listfile.is_null() {
    get_list_blocks(listfile);
  }
  if (*ptr_to_globals).version2 != 0 {
    make_root_inode2();
    make_bad_inode2();
  } else {
    make_root_inode();
    make_bad_inode();
  }
  mark_good_blocks();
  write_tables();
  return 0i32;
}
