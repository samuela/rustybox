use crate::librb::size_t;
use crate::librb::uoff_t;

use libc;
use libc::off_t;
use libc::printf;
use libc::stat;
use libc::strcpy;
use libc::time;
use libc::time_t;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);

  #[no_mangle]
  fn xfstat(fd_0: libc::c_int, buf: *mut stat, errmsg: *const libc::c_char);

  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn xlseek(fd_0: libc::c_int, offset: off_t, whence: libc::c_int) -> off_t;

  #[no_mangle]
  fn get_volume_size_in_bytes(
    fd_0: libc::c_int,
    override_0: *const libc::c_char,
    override_units: libc::c_uint,
    extend: libc::c_int,
  ) -> uoff_t;

  #[no_mangle]
  fn safe_strncpy(
    dst: *mut libc::c_char,
    src: *const libc::c_char,
    size: size_t,
  ) -> *mut libc::c_char;

  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn xwrite(fd_0: libc::c_int, buf: *const libc::c_void, count: size_t);

  #[no_mangle]
  fn xclose(fd_0: libc::c_int);

  #[no_mangle]
  fn generate_uuid(buf: *mut u8);

  #[no_mangle]
  static mut option_mask32: u32;

  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;

  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);

  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;

  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;

  #[no_mangle]
  fn find_mount_point(name: *const libc::c_char, subdir_too: libc::c_int) -> *mut mntent;
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

/*
 * Structure of a blocks group descriptor
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ext2_group_desc {
  pub bg_block_bitmap: u32,
  pub bg_inode_bitmap: u32,
  pub bg_inode_table: u32,
  pub bg_free_blocks_count: u16,
  pub bg_free_inodes_count: u16,
  pub bg_used_dirs_count: u16,
  pub bg_pad: u16,
  pub bg_reserved: [u32; 3],
}

/*
 * Macro-instructions used to manage group descriptors
 */
/* limits imposed by 16-bit value gd_free_{blocks,inode}_count */
/*
 * Constants relative to the data blocks
 */
/*
 * Inode flags
 */
/* Secure deletion */
/* Undelete */
/* Compress file */
/* Synchronous updates */
/* Immutable file */
/* writes to file may only append */
/* do not dump file */
/* do not update atime */
/* Reserved for compression usage... */
/* One or more compressed clusters */
/* Access raw compressed data */
/* Compression error */
/* End compression flags --- maybe not all used */
/* btree format dir */
/* hash-indexed directory */
/* file data should be journaled */
/* file tail should not be merged */
/* Synchronous directory modifications */
/* Top of directory hierarchies*/
/* Inode uses extents */
/* reserved for ext2 lib */
/* User visible flags */
/* User modifiable flags */
/*
 * ioctl commands
 */

/*
 * Structure of an inode on the disk
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ext2_inode {
  pub i_mode: u16,
  pub i_uid: u16,
  pub i_size: u32,
  pub i_atime: u32,
  pub i_ctime: u32,
  pub i_mtime: u32,
  pub i_dtime: u32,
  pub i_gid: u16,
  pub i_links_count: u16,
  pub i_blocks: u32,
  pub i_flags: u32,
  pub osd1: C2RustUnnamed_3,
  pub i_block: [u32; 15],
  pub i_generation: u32,
  pub i_file_acl: u32,
  pub i_dir_acl: u32,
  pub i_faddr: u32,
  pub osd2: C2RustUnnamed,
  /* OS dependent 2 */
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
  pub linux2: C2RustUnnamed_2,
  pub hurd2: C2RustUnnamed_1,
  pub masix2: C2RustUnnamed_0,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
  pub m_i_frag: u8,
  pub m_i_fsize: u8,
  pub m_pad1: u16,
  pub m_i_reserved2: [u32; 2],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
  pub h_i_frag: u8,
  pub h_i_fsize: u8,
  pub h_i_mode_high: u16,
  pub h_i_uid_high: u16,
  pub h_i_gid_high: u16,
  pub h_i_author: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
  pub l_i_frag: u8,
  pub l_i_fsize: u8,
  pub i_pad1: u16,
  pub l_i_uid_high: u16,
  pub l_i_gid_high: u16,
  pub l_i_reserved2: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
  pub linux1: C2RustUnnamed_6,
  pub hurd1: C2RustUnnamed_5,
  pub masix1: C2RustUnnamed_4,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
  pub m_i_reserved1: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
  pub h_i_translator: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
  pub l_i_reserved1: u32,
}
/*
 * File system states
 */
/* Unmounted cleanly */
/* Errors detected */
/*
 * Mount flags
 */
/* Do mount-time checks */
/* Create files with directory's group */
/* Some debugging messages */
/* Continue on errors */
/* Remount fs ro on errors */
/* Panic on errors */
/* Mimics the Minix statfs */
/* Disable 32-bit UIDs */
/*
 * Maximal mount counts between two filesystem checks
 */
/* Allow 20 mounts */
/* Don't use interval check */
/*
 * Behaviour when detecting errors
 */
/* Continue execution */
/* Remount fs read-only */
/* Panic */

/*
 * Structure of the super block
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ext2_super_block {
  pub s_inodes_count: u32,
  pub s_blocks_count: u32,
  pub s_r_blocks_count: u32,
  pub s_free_blocks_count: u32,
  pub s_free_inodes_count: u32,
  pub s_first_data_block: u32,
  pub s_log_block_size: u32,
  pub s_log_frag_size: i32,
  pub s_blocks_per_group: u32,
  pub s_frags_per_group: u32,
  pub s_inodes_per_group: u32,
  pub s_mtime: u32,
  pub s_wtime: u32,
  pub s_mnt_count: u16,
  pub s_max_mnt_count: i16,
  pub s_magic: u16,
  pub s_state: u16,
  pub s_errors: u16,
  pub s_minor_rev_level: u16,
  pub s_lastcheck: u32,
  pub s_checkinterval: u32,
  pub s_creator_os: u32,
  pub s_rev_level: u32,
  pub s_def_resuid: u16,
  pub s_def_resgid: u16,
  pub s_first_ino: u32,
  pub s_inode_size: u16,
  pub s_block_group_nr: u16,
  pub s_feature_compat: u32,
  pub s_feature_incompat: u32,
  pub s_feature_ro_compat: u32,
  pub s_uuid: [u8; 16],
  pub s_volume_name: [libc::c_char; 16],
  pub s_last_mounted: [libc::c_char; 64],
  pub s_algorithm_usage_bitmap: u32,
  pub s_prealloc_blocks: u8,
  pub s_prealloc_dir_blocks: u8,
  pub s_reserved_gdt_blocks: u16,
  pub s_journal_uuid: [u8; 16],
  pub s_journal_inum: u32,
  pub s_journal_dev: u32,
  pub s_last_orphan: u32,
  pub s_hash_seed: [u32; 4],
  pub s_def_hash_version: u8,
  pub s_jnl_backup_type: u8,
  pub s_reserved_word_pad: u16,
  pub s_default_mount_opts: u32,
  pub s_first_meta_bg: u32,
  pub s_mkfs_time: u32,
  pub s_jnl_blocks: [u32; 17],
  pub s_blocks_count_hi: u32,
  pub s_r_blocks_count_hi: u32,
  pub s_free_blocks_count_hi: u32,
  pub s_min_extra_isize: u16,
  pub s_want_extra_isize: u16,
  pub s_flags: u32,
  pub s_raid_stride: u16,
  pub s_mmp_interval: u16,
  pub s_mmp_block: u64,
  pub s_raid_stripe_width: u32,
  pub s_log_groups_per_flex: u8,
  pub s_reserved_char_pad2: u8,
  pub s_reserved_pad: u16,
  pub s_reserved: [u32; 162],
  /* Padding to the end of the block */
}

// All fields are little-endian
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ext2_dir {
  pub inode1: u32,
  pub rec_len1: u16,
  pub name_len1: u8,
  pub file_type1: u8,
  pub name1: [libc::c_char; 4],
  pub inode2: u32,
  pub rec_len2: u16,
  pub name_len2: u8,
  pub file_type2: u8,
  pub name2: [libc::c_char; 4],
  pub inode3: u32,
  pub rec_len3: u16,
  pub name_len3: u8,
  pub file_type3: u8,
  pub name3: [libc::c_char; 12],
}

// 128 and 256-byte inodes:
// 128-byte inode is described by struct ext2_inode.
// 256-byte one just has these fields appended:
//      __u16   i_extra_isize;
//      __u16   i_pad1;
//      u32   i_ctime_extra;  /* extra Change time (nsec << 2 | epoch) */
//      u32   i_mtime_extra;  /* extra Modification time (nsec << 2 | epoch) */
//      u32   i_atime_extra;  /* extra Access time (nsec << 2 | epoch) */
//      u32   i_crtime;       /* File creation time */
//      u32   i_crtime_extra; /* extra File creation time (nsec << 2 | epoch)*/
//      u32   i_version_hi;   /* high 32 bits for 64-bit version */
// the rest is padding.
//
// linux/ext2_fs.h has "#define i_size_high i_dir_acl" which suggests that even
// 128-byte inode is capable of describing large files (i_dir_acl is meaningful
// only for directories, which never need i_size_high).
//
// Standard mke2fs creates a filesystem with 256-byte inodes if it is
// bigger than 0.5GB.
// Standard mke2fs 1.41.9:
// Usage: mke2fs [-c|-l filename] [-b block-size] [-f fragment-size]
//	[-i bytes-per-inode] [-I inode-size] [-J journal-options]
//	[-G meta group size] [-N number-of-inodes]
//	[-m reserved-blocks-percentage] [-o creator-os]
//	[-g blocks-per-group] [-L volume-label] [-M last-mounted-directory]
//	[-O feature[,...]] [-r fs-revision] [-E extended-option[,...]]
//	[-T fs-type] [-U UUID] [-jnqvFSV] device [blocks-count]
//
// Options not commented below are taken but silently ignored:

pub type C2RustUnnamed_7 = libc::c_uint;
// pub const OPT_S: C2RustUnnamed_7 = 16777216;
pub const OPT_F: C2RustUnnamed_7 = 8388608;
// pub const OPT_v: C2RustUnnamed_7 = 4194304;
//OPT_V = 1 << 25,	// -V version. bbox applets don't support that
// dry run: do not write anything
// pub const OPT_q: C2RustUnnamed_7 = 2097152;
pub const OPT_n: C2RustUnnamed_7 = 1048576;
// pub const OPT_j: C2RustUnnamed_7 = 524288;
// pub const OPT_U: C2RustUnnamed_7 = 262144;
// pub const OPT_T: C2RustUnnamed_7 = 131072;
// pub const OPT_E: C2RustUnnamed_7 = 65536;
// pub const OPT_r: C2RustUnnamed_7 = 32768;
// pub const OPT_O: C2RustUnnamed_7 = 16384;
// label
// pub const OPT_M: C2RustUnnamed_7 = 8192;
// pub const OPT_L: C2RustUnnamed_7 = 4096;
// pub const OPT_g: C2RustUnnamed_7 = 2048;
// percentage of blocks reserved for superuser
// pub const OPT_o: C2RustUnnamed_7 = 1024;
// pub const OPT_m: C2RustUnnamed_7 = 512;
// pub const OPT_N: C2RustUnnamed_7 = 256;
// pub const OPT_G: C2RustUnnamed_7 = 128;
// custom inode size, in bytes
// pub const OPT_J: C2RustUnnamed_7 = 64;
// bytes per inode
pub const OPT_I: C2RustUnnamed_7 = 32;
pub const OPT_i: C2RustUnnamed_7 = 16;
// block size, in bytes
// pub const OPT_f: C2RustUnnamed_7 = 8;
pub const OPT_b: C2RustUnnamed_7 = 4;
// pub const OPT_l: C2RustUnnamed_7 = 2;
// pub const OPT_c: C2RustUnnamed_7 = 1;

// For some reason this function isn't actually defined in the C sources, at
// least AFAICT. So we use this dummy implementation.
pub fn BUG_wrong_field_size() -> libc::c_uint {
  panic!("BUG_wrong_field_size")
}

unsafe extern "C" fn int_log2(mut arg: libc::c_uint) -> libc::c_uint {
  let mut r: libc::c_uint = 0i32 as libc::c_uint;
  loop {
    arg >>= 1i32;
    if !(arg != 0i32 as libc::c_uint) {
      break;
    }
    r = r.wrapping_add(1)
  }
  return r;
}
// taken from mkfs_minix.c. libbb candidate?
// "u32 size", since we never use it for anything >32 bits
unsafe extern "C" fn div_roundup(mut size: u32, mut n: u32) -> u32 {
  // Overflow-resistant
  let mut res: u32 = size.wrapping_div(n);
  if res.wrapping_mul(n) != size {
    res = res.wrapping_add(1)
  }
  return res;
}
unsafe extern "C" fn allocate(
  mut bitmap: *mut u8,
  mut blocksize: u32,
  mut start: u32,
  mut end: u32,
) {
  let mut i: u32 = 0;
  //bb_error_msg("ALLOC: [%u][%u][%u]: [%u-%u]:=[%x],[%x]", blocksize, start, end, start/8, blocksize - end/8 - 1, (1 << (start & 7)) - 1, (u8)(0xFF00 >> (end & 7)));
  memset(
    bitmap as *mut libc::c_void,
    0i32,
    blocksize as libc::c_ulong,
  ); //0..7 => 00000000..01111111
  i = start.wrapping_div(8i32 as libc::c_uint); //0..7 => 00000000..11111110
  memset(bitmap as *mut libc::c_void, 0xffi32, i as libc::c_ulong);
  *bitmap.offset(i as isize) = ((1i32 << (start & 7i32 as libc::c_uint)) - 1i32) as u8;
  i = end.wrapping_div(8i32 as libc::c_uint);
  let ref mut fresh0 =
    *bitmap.offset(blocksize.wrapping_sub(i).wrapping_sub(1i32 as libc::c_uint) as isize);
  *fresh0 = (*fresh0 as libc::c_int | 0x7f00i32 >> (end & 7i32 as libc::c_uint)) as u8;
  memset(
    bitmap.offset(blocksize as isize).offset(-(i as isize)) as *mut libc::c_void,
    0xffi32,
    i as libc::c_ulong,
  );
  // N.B. no overflow here!
}
unsafe extern "C" fn has_super(mut x: u32) -> u32 {
  // 0, 1 and powers of 3, 5, 7 up to 2^32 limit
  static mut supers: [u32; 46] = [
    0i32 as u32,
    1i32 as u32,
    3i32 as u32,
    5i32 as u32,
    7i32 as u32,
    9i32 as u32,
    25i32 as u32,
    27i32 as u32,
    49i32 as u32,
    81i32 as u32,
    125i32 as u32,
    243i32 as u32,
    343i32 as u32,
    625i32 as u32,
    729i32 as u32,
    2187i32 as u32,
    2401i32 as u32,
    3125i32 as u32,
    6561i32 as u32,
    15625i32 as u32,
    16807i32 as u32,
    19683i32 as u32,
    59049i32 as u32,
    78125i32 as u32,
    117649i32 as u32,
    177147i32 as u32,
    390625i32 as u32,
    531441i32 as u32,
    823543i32 as u32,
    1594323i32 as u32,
    1953125i32 as u32,
    4782969i32 as u32,
    5764801i32 as u32,
    9765625i32 as u32,
    14348907i32 as u32,
    40353607i32 as u32,
    43046721i32 as u32,
    48828125i32 as u32,
    129140163i32 as u32,
    244140625i32 as u32,
    282475249i32 as u32,
    387420489i32 as u32,
    1162261467i32 as u32,
    1220703125i32 as u32,
    1977326743i32 as u32,
    3486784401i64 as u32,
  ];
  let mut sp: *const u32 = supers.as_ptr().offset(
    (::std::mem::size_of::<[u32; 46]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<u32>() as libc::c_ulong) as libc::c_uint as isize,
  );
  loop {
    sp = sp.offset(-1);
    if x == *sp {
      return 1i32 as u32;
    }
    if x > *sp {
      return 0i32 as u32;
    }
  }
}
/* predefined output descriptor */
unsafe extern "C" fn PUT(mut off: u64, mut buf: *mut libc::c_void, mut size: u32) {
  //bb_error_msg("PUT[%llu]:[%u]", off, size);
  xlseek(3i32, off as off_t, 0i32); // superblock
  xwrite(3i32, buf, size as size_t); // group descriptors
}
#[no_mangle]
pub unsafe extern "C" fn mkfs_ext2_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut i: libc::c_uint = 0;
  let mut pos: libc::c_uint = 0;
  let mut n: libc::c_uint = 0;
  let mut bs: libc::c_uint = 0;
  let mut bpi: libc::c_uint = 0;
  let mut blocksize: libc::c_uint = 0;
  let mut blocksize_log2: libc::c_uint = 0;
  let mut inodesize: libc::c_uint = 0;
  let mut user_inodesize: libc::c_uint = 0;
  let mut reserved_percent: libc::c_uint = 5i32 as libc::c_uint;
  let mut kilobytes: libc::c_ulonglong = 0;
  let mut nblocks: u32 = 0;
  let mut nblocks_full: u32 = 0;
  let mut nreserved: u32 = 0;
  let mut ngroups: u32 = 0;
  let mut bytes_per_inode: u32 = 0;
  let mut first_block: u32 = 0;
  let mut inodes_per_group: u32 = 0;
  let mut group_desc_blocks: u32 = 0;
  let mut inode_table_blocks: u32 = 0;
  let mut lost_and_found_blocks: u32 = 0;
  let mut timestamp: time_t = 0;
  let mut label: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
  let mut st: stat = std::mem::zeroed();
  let mut sb: *mut ext2_super_block = 0 as *mut ext2_super_block;
  let mut gd: *mut ext2_group_desc = 0 as *mut ext2_group_desc;
  let mut inode: *mut ext2_inode = 0 as *mut ext2_inode;
  let mut dir: *mut ext2_dir = 0 as *mut ext2_dir;
  let mut buf: *mut u8 = 0 as *mut u8;
  // using global "option_mask32" instead of local "opts":
  // we are register starved here
  /*opts =*/
  getopt32(
    argv,
    b"cl:b:+f:i:+I:+J:G:N:m:+o:g:L:M:O:r:E:T:U:jnqvFS\x00" as *const u8 as *const libc::c_char,
    0 as *mut libc::c_void,
    &mut bs as *mut libc::c_uint,
    0 as *mut libc::c_void,
    &mut bpi as *mut libc::c_uint,
    &mut user_inodesize as *mut libc::c_uint,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
    &mut reserved_percent as *mut libc::c_uint,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
    &mut label as *mut *const libc::c_char,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
  ); // argv[0] -- device
  argv = argv.offset(optind as isize);
  // open the device, check the device is a block device
  xmove_fd(xopen(*argv.offset(0), 0o1i32), 3i32);
  xfstat(3i32, &mut st, *argv.offset(0));
  if !(st.st_mode & 0o170000i32 as libc::c_uint == 0o60000i32 as libc::c_uint)
    && option_mask32 & OPT_F as libc::c_int as libc::c_uint == 0
  {
    bb_error_msg_and_die(
      b"%s: not a block device\x00" as *const u8 as *const libc::c_char,
      *argv.offset(0),
    );
  }
  // check if it is mounted
  // N.B. what if we format a file? find_mount_point will return false negative since
  // it is loop block device which is mounted!
  if !find_mount_point(*argv.offset(0), 0i32).is_null() {
    bb_simple_error_msg_and_die(
      b"can\'t format mounted filesystem\x00" as *const u8 as *const libc::c_char,
    );
  }
  // get size in kbytes
  kilobytes = get_volume_size_in_bytes(
    3i32,
    *argv.offset(1),
    1024i32 as libc::c_uint,
    (option_mask32 & OPT_n as libc::c_int as libc::c_uint == 0) as libc::c_int,
  )
  .wrapping_div(1024i32 as libc::c_ulong) as libc::c_ulonglong;
  bytes_per_inode = 16384i32 as u32;
  if kilobytes < (512i32 * 1024i32) as libc::c_ulonglong {
    bytes_per_inode = 4096i32 as u32
  }
  if kilobytes < (3i32 * 1024i32) as libc::c_ulonglong {
    bytes_per_inode = 8192i32 as u32
  }
  if option_mask32 & OPT_i as libc::c_int as libc::c_uint != 0 {
    bytes_per_inode = bpi
  }
  // Determine block size and inode size
  // block size is a multiple of 1024
  // inode size is a multiple of 128
  blocksize = 1024i32 as libc::c_uint; // 128
  inodesize = ::std::mem::size_of::<ext2_inode>() as libc::c_ulong as libc::c_uint;
  if kilobytes >= (512i32 * 1024i32) as libc::c_ulonglong {
    // mke2fs 1.41.9 compat
    blocksize = 4096i32 as libc::c_uint;
    inodesize = 256i32 as libc::c_uint
  }
  if 1i32 << 16i32 > 4096i32 {
    // kilobytes >> 22 == size in 4gigabyte chunks.
    // if size >= 16k gigs, blocksize must be increased.
    // Try "mke2fs -F image $((16 * 1024*1024*1024))"
    while kilobytes >> 22i32 >= blocksize as libc::c_ulonglong {
      blocksize = blocksize.wrapping_mul(2i32 as libc::c_uint)
    }
  }
  if option_mask32 & OPT_b as libc::c_int as libc::c_uint != 0 {
    blocksize = bs
  }
  if blocksize < (1i32 << 10i32) as libc::c_uint
    || blocksize > (1i32 << 16i32) as libc::c_uint
    || blocksize & blocksize.wrapping_sub(1i32 as libc::c_uint) != 0
  {
    // not power of 2
    bb_error_msg_and_die(
      b"blocksize %u is bad\x00" as *const u8 as *const libc::c_char,
      blocksize,
    );
  }
  // Do we have custom inode size?
  if option_mask32 & OPT_I as libc::c_int as libc::c_uint != 0 {
    if (user_inodesize as libc::c_ulong) < ::std::mem::size_of::<ext2_inode>() as libc::c_ulong
      || user_inodesize > blocksize
      || user_inodesize & user_inodesize.wrapping_sub(1i32 as libc::c_uint) != 0
    {
      // not power of 2
      bb_error_msg(
        b"-%c is bad\x00" as *const u8 as *const libc::c_char,
        'I' as i32,
      );
    } else {
      inodesize = user_inodesize
    }
  }
  if (bytes_per_inode as i32 as libc::c_uint) < blocksize {
    bb_error_msg_and_die(
      b"-%c is bad\x00" as *const u8 as *const libc::c_char,
      'i' as i32,
    );
  }
  // number of bits in one block, i.e. 8*blocksize
  first_block = ((1i32 << 10i32) as libc::c_uint == blocksize) as libc::c_int as u32;
  blocksize_log2 = int_log2(blocksize);
  // Determine number of blocks
  kilobytes >>= blocksize_log2.wrapping_sub(10i32 as libc::c_uint);
  nblocks = kilobytes as u32;
  if nblocks as libc::c_ulonglong != kilobytes {
    bb_simple_error_msg_and_die(
      b"block count doesn\'t fit in 32 bits\x00" as *const u8 as *const libc::c_char,
    );
  }
  // Experimentally, standard mke2fs won't work on images smaller than 60k
  if nblocks < 60i32 as libc::c_uint {
    bb_simple_error_msg_and_die(b"need >= 60 blocks\x00" as *const u8 as *const libc::c_char);
  }
  // How many reserved blocks?
  if reserved_percent > 50i32 as libc::c_uint {
    bb_error_msg_and_die(
      b"-%c is bad\x00" as *const u8 as *const libc::c_char,
      'm' as i32,
    );
  }
  nreserved = (nblocks as u64)
    .wrapping_mul(reserved_percent as libc::c_ulong)
    .wrapping_div(100i32 as libc::c_ulong) as u32;
  // N.B. killing e2fsprogs feature! Unused blocks don't account in calculations
  nblocks_full = nblocks;
  loop
  // If last block group is too small, nblocks may be decreased in order
  // to discard it, and control returns here to recalculate some
  // parameters.
  // Note: blocksize and bytes_per_inode are never recalculated.
  // N.B. a block group can have no more than blocks_per_group blocks
  {
    ngroups = div_roundup(
      nblocks.wrapping_sub(first_block),
      (8i32 as libc::c_uint).wrapping_mul(blocksize),
    );
    group_desc_blocks = div_roundup(
      ngroups,
      (blocksize as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<ext2_group_desc>() as libc::c_ulong) as u32,
    );
    // TODO: reserved blocks must be marked as such in the bitmaps,
    // or resulting filesystem is corrupt
    // N.B. e2fsprogs does as follows!
    let mut overhead: u32 = 0;
    let mut remainder: u32 = 0;
    // ninodes is the max number of inodes in this filesystem
    let mut ninodes: u32 = (nblocks_full as u64)
      .wrapping_mul(blocksize as libc::c_ulong)
      .wrapping_div(bytes_per_inode as libc::c_ulong) as u32;
    if ninodes < (11i32 + 1i32) as libc::c_uint {
      ninodes = (11i32 + 1i32) as u32
    }
    inodes_per_group = div_roundup(ninodes, ngroups);
    // minimum number because the first EXT2_GOOD_OLD_FIRST_INO-1 are reserved
    if inodes_per_group < 16i32 as libc::c_uint {
      inodes_per_group = 16i32 as u32
    }
    // a block group can't have more inodes than blocks
    if inodes_per_group > (8i32 as libc::c_uint).wrapping_mul(blocksize) {
      inodes_per_group = (8i32 as libc::c_uint).wrapping_mul(blocksize)
    }
    // adjust inodes per group so they completely fill the inode table blocks in the descriptor
    inodes_per_group = div_roundup(inodes_per_group.wrapping_mul(inodesize), blocksize)
      .wrapping_mul(blocksize)
      .wrapping_div(inodesize);
    // make sure the number of inodes per group is a multiple of 8
    inodes_per_group &= !7i32 as libc::c_uint;
    inode_table_blocks = div_roundup(inodes_per_group.wrapping_mul(inodesize), blocksize);
    // to be useful, lost+found should occupy at least 2 blocks (but not exceeding 16*1024 bytes),
    // and at most EXT2_NDIR_BLOCKS. So reserve these blocks right now
    /* Or e2fsprogs comment verbatim (what does it mean?):
     * Ensure that lost+found is at least 2 blocks, so we always
     * test large empty blocks for big-block filesystems. */
    lost_and_found_blocks = if 12i32 < 16i32 >> blocksize_log2.wrapping_sub(10i32 as libc::c_uint) {
      12i32
    } else {
      (16i32) >> blocksize_log2.wrapping_sub(10i32 as libc::c_uint)
    } as u32;
    // the last group needs more attention: isn't it too small for possible overhead?
    overhead = (if has_super(ngroups.wrapping_sub(1i32 as libc::c_uint)) != 0 {
      (1i32 as libc::c_uint).wrapping_add(group_desc_blocks)
    } else {
      0i32 as libc::c_uint
    })
    .wrapping_add(1i32 as libc::c_uint)
    .wrapping_add(1i32 as libc::c_uint)
    .wrapping_add(inode_table_blocks);
    remainder = nblocks
      .wrapping_sub(first_block)
      .wrapping_rem((8i32 as libc::c_uint).wrapping_mul(blocksize));
    // //can't happen, nblocks >= 60 guarantees this
    // //if ((1 == ngroups)
    // // && remainder
    // // && (remainder < overhead + 1/* "/" */ + lost_and_found_blocks)
    // //) {
    // //	bb_error_msg_and_die("way small device");
    // //}
    // Standard mke2fs uses 50. Looks like a bug in our calculation
    // of "remainder" or "overhead" - we don't match standard mke2fs
    // when we transition from one group to two groups
    // (a bit after 8M image size), but it works for two->three groups
    // transition (at 16M).
    if !(remainder != 0 && remainder < overhead.wrapping_add(50i32 as libc::c_uint)) {
      break;
    }
    //bb_error_msg("CHOP[%u]", remainder);
    nblocks = (nblocks as libc::c_uint).wrapping_sub(remainder) as u32 as u32
  }
  if nblocks_full.wrapping_sub(nblocks) != 0 {
    printf(
      b"warning: %u blocks unused\n\n\x00" as *const u8 as *const libc::c_char,
      nblocks_full.wrapping_sub(nblocks),
    );
  }
  printf(b"Filesystem label=%s\nOS type: Linux\nBlock size=%u (log=%u)\nFragment size=%u (log=%u)\n%u inodes, %u blocks\n%u blocks (%u%%) reserved for the super user\nFirst data block=%u\nMaximum filesystem blocks=%u\n%u block groups\n%u blocks per group, %u fragments per group\n%u inodes per group\x00"
               as *const u8 as *const libc::c_char, label, blocksize,
           blocksize_log2.wrapping_sub(10i32 as libc::c_uint), blocksize,
           blocksize_log2.wrapping_sub(10i32 as libc::c_uint),
           inodes_per_group.wrapping_mul(ngroups), nblocks, nreserved,
           reserved_percent, first_block,
           group_desc_blocks.wrapping_mul(blocksize.wrapping_div(::std::mem::size_of::<ext2_group_desc>()
                                                                     as
                                                                     libc::c_ulong
                                                                     as
                                                                     libc::c_uint)).wrapping_mul((8i32
                                                                                                      as
                                                                                                      libc::c_uint).wrapping_mul(blocksize)),
           ngroups, (8i32 as libc::c_uint).wrapping_mul(blocksize),
           (8i32 as libc::c_uint).wrapping_mul(blocksize), inodes_per_group);
  let mut fmt: *const libc::c_char =
    b"\nSuperblock backups stored on blocks:\n\t%u\x00" as *const u8 as *const libc::c_char;
  pos = first_block;
  i = 1i32 as libc::c_uint;
  while i < ngroups {
    pos = pos.wrapping_add((8i32 as libc::c_uint).wrapping_mul(blocksize));
    if has_super(i) != 0 {
      printf(fmt, pos);
      fmt = b", %u\x00" as *const u8 as *const libc::c_char
    }
    i = i.wrapping_add(1)
  }
  bb_putchar('\n' as i32);
  if option_mask32 & OPT_n as libc::c_int as libc::c_uint != 0 {
    return 0i32;
  }
  // TODO: 3/5 refuse if mounted
  // TODO: 4/5 compat options
  // TODO: 1/5 sanity checks
  // TODO: 0/5 more verbose error messages
  // TODO: 4/5 bigendianness: recheck, wait for ARM reporters
  // TODO: 2/5 reserved GDT: how to mark but not allocate?
  // TODO: 3/5 dir_index?
  // fill the superblock
  sb = xzalloc(1024i32 as size_t) as *mut ext2_super_block; // revision 1 filesystem
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_rev_level = 1i32 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_rev_level = 1i32 as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_rev_level = 1i32 as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u16>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_magic = 0xef53i32 as u32 as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_magic = 0xef53i32 as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_magic = 0xef53i32 as u8 as u16
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u16>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_inode_size = inodesize as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_inode_size = inodesize as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_inode_size = inodesize as u8 as u16
  } else {
    BUG_wrong_field_size();
  }
  // set "Required extra isize" and "Desired extra isize" fields to 28
  if inodesize as libc::c_ulong != ::std::mem::size_of::<ext2_inode>() as libc::c_ulong {
    if ::std::mem::size_of::<u16>() as libc::c_ulong == 4i32 as libc::c_ulong {
      (*sb).s_min_extra_isize = 0x1ci32 as u32 as u16
    } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 2i32 as libc::c_ulong {
      (*sb).s_min_extra_isize = 0x1ci32 as u16
    } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 1i32 as libc::c_ulong {
      (*sb).s_min_extra_isize = 0x1ci32 as u8 as u16
    } else {
      BUG_wrong_field_size();
    }
    if ::std::mem::size_of::<u16>() as libc::c_ulong == 4i32 as libc::c_ulong {
      (*sb).s_want_extra_isize = 0x1ci32 as u32 as u16
    } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 2i32 as libc::c_ulong {
      (*sb).s_want_extra_isize = 0x1ci32 as u16
    } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 1i32 as libc::c_ulong {
      (*sb).s_want_extra_isize = 0x1ci32 as u8 as u16
    } else {
      BUG_wrong_field_size();
    }
  }
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_first_ino = 11i32 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_first_ino = 11i32 as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_first_ino = 11i32 as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_log_block_size = blocksize_log2.wrapping_sub(10i32 as libc::c_uint)
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_log_block_size = blocksize_log2.wrapping_sub(10i32 as libc::c_uint) as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_log_block_size = blocksize_log2.wrapping_sub(10i32 as libc::c_uint) as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<i32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_log_frag_size = blocksize_log2.wrapping_sub(10i32 as libc::c_uint) as i32
  } else if ::std::mem::size_of::<i32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_log_frag_size = blocksize_log2.wrapping_sub(10i32 as libc::c_uint) as u16 as i32
  } else if ::std::mem::size_of::<i32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_log_frag_size = blocksize_log2.wrapping_sub(10i32 as libc::c_uint) as u8 as i32
  } else {
    BUG_wrong_field_size();
  }
  // first 1024 bytes of the device are for boot record. If block size is 1024 bytes, then
  // the first block is 1, otherwise 0
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_first_data_block = first_block
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_first_data_block = first_block as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_first_data_block = first_block as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  // block and inode bitmaps occupy no more than one block, so maximum number of blocks is
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_blocks_per_group = (8i32 as libc::c_uint).wrapping_mul(blocksize)
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_blocks_per_group = (8i32 as libc::c_uint).wrapping_mul(blocksize) as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_blocks_per_group = (8i32 as libc::c_uint).wrapping_mul(blocksize) as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_frags_per_group = (8i32 as libc::c_uint).wrapping_mul(blocksize)
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_frags_per_group = (8i32 as libc::c_uint).wrapping_mul(blocksize) as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_frags_per_group = (8i32 as libc::c_uint).wrapping_mul(blocksize) as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  // blocks
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_blocks_count = nblocks
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_blocks_count = nblocks as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_blocks_count = nblocks as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  // reserve blocks for superuser
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_r_blocks_count = nreserved
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_r_blocks_count = nreserved as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_r_blocks_count = nreserved as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  // ninodes
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_inodes_per_group = inodes_per_group
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_inodes_per_group = inodes_per_group as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_inodes_per_group = inodes_per_group as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_inodes_count = inodes_per_group.wrapping_mul(ngroups)
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_inodes_count = inodes_per_group.wrapping_mul(ngroups) as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_inodes_count = inodes_per_group.wrapping_mul(ngroups) as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_free_inodes_count = inodes_per_group
      .wrapping_mul(ngroups)
      .wrapping_sub(11i32 as libc::c_uint)
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_free_inodes_count = inodes_per_group
      .wrapping_mul(ngroups)
      .wrapping_sub(11i32 as libc::c_uint) as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_free_inodes_count = inodes_per_group
      .wrapping_mul(ngroups)
      .wrapping_sub(11i32 as libc::c_uint) as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  // timestamps
  timestamp = time(0 as *mut time_t);
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_mkfs_time = timestamp as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_mkfs_time = timestamp as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_mkfs_time = timestamp as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_wtime = timestamp as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_wtime = timestamp as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_wtime = timestamp as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_lastcheck = timestamp as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_lastcheck = timestamp as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_lastcheck = timestamp as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  // misc. Values are chosen to match mke2fs 1.41.9
  if ::std::mem::size_of::<u16>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_state = 1i32 as u32 as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_state = 1i32 as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_state = 1i32 as u8 as u16
  } else {
    BUG_wrong_field_size(); // TODO: what's 1?
  } // 180 days
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_creator_os = 0i32 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_creator_os = 0i32 as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_creator_os = 0i32 as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_checkinterval = (24i32 * 60i32 * 60i32 * 180i32) as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_checkinterval = (24i32 * 60i32 * 60i32 * 180i32) as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_checkinterval = (24i32 * 60i32 * 60i32 * 180i32) as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u16>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_errors = 1i32 as u32 as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_errors = 1i32 as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_errors = 1i32 as u8 as u16
  } else {
    BUG_wrong_field_size();
  }
  // mke2fs 1.41.9 also sets EXT3_FEATURE_COMPAT_RESIZE_INODE
  // and if >= 0.5GB, EXT3_FEATURE_RO_COMPAT_LARGE_FILE.
  // we use values which match "mke2fs -O ^resize_inode":
  // in this case 1.41.9 never sets EXT3_FEATURE_RO_COMPAT_LARGE_FILE.
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_feature_compat = (0i32 | 0x10i32 * 0i32 | 0x20i32 * 1i32) as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_feature_compat = (0i32 | 0x10i32 * 0i32 | 0x20i32 * 1i32) as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_feature_compat = (0i32 | 0x10i32 * 0i32 | 0x20i32 * 1i32) as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_feature_incompat = 0x2i32 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_feature_incompat = 0x2i32 as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_feature_incompat = 0x2i32 as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_feature_ro_compat = 0x1i32 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_feature_ro_compat = 0x1i32 as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_feature_ro_compat = 0x1i32 as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_flags = (0x2i32 * 1i32) as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_flags = (0x2i32 * 1i32) as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_flags = (0x2i32 * 1i32) as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  generate_uuid((*sb).s_uuid.as_mut_ptr());
  if ::std::mem::size_of::<u8>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_def_hash_version = 1i32 as u32 as u8
  } else if ::std::mem::size_of::<u8>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_def_hash_version = 1i32 as u16 as u8
  } else if ::std::mem::size_of::<u8>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_def_hash_version = 1i32 as u8
  } else {
    BUG_wrong_field_size();
  }
  generate_uuid((*sb).s_hash_seed.as_mut_ptr() as *mut u8);
  /*
   * From e2fsprogs: add "jitter" to the superblock's check interval so that we
   * don't check all the filesystems at the same time.  We use a
   * kludgy hack of using the UUID to derive a random jitter value.
   */
  if ::std::mem::size_of::<i16>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_max_mnt_count = (20i32
      + (*sb).s_uuid[((::std::mem::size_of::<[u8; 16]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<u8>() as libc::c_ulong)
        as libc::c_uint)
        .wrapping_sub(1i32 as libc::c_uint) as usize] as libc::c_int
        % 20i32) as u32 as i16
  } else if ::std::mem::size_of::<i16>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_max_mnt_count = (20i32
      + (*sb).s_uuid[((::std::mem::size_of::<[u8; 16]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<u8>() as libc::c_ulong)
        as libc::c_uint)
        .wrapping_sub(1i32 as libc::c_uint) as usize] as libc::c_int
        % 20i32) as u16 as i16
  } else if ::std::mem::size_of::<i16>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_max_mnt_count = (20i32
      + (*sb).s_uuid[((::std::mem::size_of::<[u8; 16]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<u8>() as libc::c_ulong)
        as libc::c_uint)
        .wrapping_sub(1i32 as libc::c_uint) as usize] as libc::c_int
        % 20i32) as u8 as i16
  } else {
    BUG_wrong_field_size();
  }
  // write the label
  safe_strncpy(
    (*sb).s_volume_name.as_mut_ptr(),
    label,
    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
  );
  // calculate filesystem skeleton structures
  gd = xzalloc(group_desc_blocks.wrapping_mul(blocksize) as size_t) as *mut ext2_group_desc;
  buf = xmalloc(blocksize as size_t) as *mut u8;
  (*sb).s_free_blocks_count = 0i32 as u32;
  i = 0i32 as libc::c_uint;
  pos = first_block;
  n = nblocks.wrapping_sub(first_block);
  while i < ngroups {
    let mut overhead_0: u32 = pos.wrapping_add(if has_super(i) != 0 {
      (1i32 as libc::c_uint).wrapping_add(group_desc_blocks)
    } else {
      0i32 as libc::c_uint
    });
    let mut free_blocks: u32 = 0;
    // fill group descriptors
    if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
      (*gd.offset(i as isize)).bg_block_bitmap = overhead_0.wrapping_add(0i32 as libc::c_uint)
    } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
      (*gd.offset(i as isize)).bg_block_bitmap =
        overhead_0.wrapping_add(0i32 as libc::c_uint) as u16 as u32
    } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
      (*gd.offset(i as isize)).bg_block_bitmap =
        overhead_0.wrapping_add(0i32 as libc::c_uint) as u8 as u32
    } else {
      BUG_wrong_field_size();
    }
    if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
      (*gd.offset(i as isize)).bg_inode_bitmap = overhead_0.wrapping_add(1i32 as libc::c_uint)
    } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
      (*gd.offset(i as isize)).bg_inode_bitmap =
        overhead_0.wrapping_add(1i32 as libc::c_uint) as u16 as u32
    } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
      (*gd.offset(i as isize)).bg_inode_bitmap =
        overhead_0.wrapping_add(1i32 as libc::c_uint) as u8 as u32
    } else {
      BUG_wrong_field_size();
    }
    if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
      (*gd.offset(i as isize)).bg_inode_table = overhead_0.wrapping_add(2i32 as libc::c_uint)
    } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
      (*gd.offset(i as isize)).bg_inode_table =
        overhead_0.wrapping_add(2i32 as libc::c_uint) as u16 as u32
    } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
      (*gd.offset(i as isize)).bg_inode_table =
        overhead_0.wrapping_add(2i32 as libc::c_uint) as u8 as u32
    } else {
      BUG_wrong_field_size();
    }
    overhead_0 = overhead_0
      .wrapping_sub(pos)
      .wrapping_add(1i32 as libc::c_uint)
      .wrapping_add(1i32 as libc::c_uint)
      .wrapping_add(inode_table_blocks);
    (*gd.offset(i as isize)).bg_free_inodes_count = inodes_per_group as u16;
    //STORE_LE(gd[i].bg_used_dirs_count, 0);
    // N.B. both "/" and "/lost+found" are within the first block group
    // "/" occupies 1 block, "/lost+found" occupies lost_and_found_blocks...
    if 0i32 as libc::c_uint == i {
      // ... thus increased overhead for the first block group ...
      overhead_0 = (overhead_0 as libc::c_uint)
        .wrapping_add((1i32 as libc::c_uint).wrapping_add(lost_and_found_blocks))
        as u32 as u32;
      // ... and 2 used directories
      if ::std::mem::size_of::<u16>() as libc::c_ulong == 4i32 as libc::c_ulong {
        (*gd.offset(i as isize)).bg_used_dirs_count = 2i32 as u32 as u16
      } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 2i32 as libc::c_ulong {
        (*gd.offset(i as isize)).bg_used_dirs_count = 2i32 as u16
      } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 1i32 as libc::c_ulong {
        (*gd.offset(i as isize)).bg_used_dirs_count = 2i32 as u8 as u16
      } else {
        BUG_wrong_field_size();
      }
      // well known reserved inodes belong to the first block too
      let ref mut fresh1 = (*gd.offset(i as isize)).bg_free_inodes_count;
      *fresh1 = (*fresh1 as libc::c_int - 11i32) as u16
    }
    // cache free block count of the group
    free_blocks = (if n < (8i32 as libc::c_uint).wrapping_mul(blocksize) {
      n
    } else {
      (8i32 as libc::c_uint).wrapping_mul(blocksize)
    })
    .wrapping_sub(overhead_0);
    // mark preallocated blocks as allocated
    //bb_error_msg("ALLOC: [%u][%u][%u]", blocksize, overhead, blocks_per_group - (free_blocks + overhead));
    allocate(
      buf,
      blocksize,
      overhead_0,
      (8i32 as libc::c_uint)
        .wrapping_mul(blocksize)
        .wrapping_sub(free_blocks.wrapping_add(overhead_0)),
    );
    // dump block bitmap
    PUT(
      ((if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
        (*gd.offset(i as isize)).bg_block_bitmap
      } else {
        BUG_wrong_field_size() as libc::c_uint
      }) as u64)
        .wrapping_mul(blocksize as libc::c_ulong),
      buf as *mut libc::c_void,
      blocksize,
    );
    if ::std::mem::size_of::<u16>() as libc::c_ulong == 4i32 as libc::c_ulong {
      (*gd.offset(i as isize)).bg_free_blocks_count = free_blocks as u16
    } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 2i32 as libc::c_ulong {
      (*gd.offset(i as isize)).bg_free_blocks_count = free_blocks as u16
    } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 1i32 as libc::c_ulong {
      (*gd.offset(i as isize)).bg_free_blocks_count = free_blocks as u8 as u16
    } else {
      BUG_wrong_field_size();
    }
    // mark preallocated inodes as allocated
    allocate(
      buf,
      blocksize,
      inodes_per_group.wrapping_sub((*gd.offset(i as isize)).bg_free_inodes_count as libc::c_uint),
      (8i32 as libc::c_uint)
        .wrapping_mul(blocksize)
        .wrapping_sub(inodes_per_group),
    );
    // dump inode bitmap
    //PUT((u64)(FETCH_LE32(gd[i].bg_block_bitmap)) * blocksize, buf, blocksize);
    //but it's right after block bitmap, so we can just:
    xwrite(3i32, buf as *const libc::c_void, blocksize as size_t);
    if ::std::mem::size_of::<u16>() as libc::c_ulong == 4i32 as libc::c_ulong {
      (*gd.offset(i as isize)).bg_free_inodes_count =
        (*gd.offset(i as isize)).bg_free_inodes_count as u32 as u16
    } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 2i32 as libc::c_ulong {
      (*gd.offset(i as isize)).bg_free_inodes_count = (*gd.offset(i as isize)).bg_free_inodes_count
    } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 1i32 as libc::c_ulong {
      (*gd.offset(i as isize)).bg_free_inodes_count =
        (*gd.offset(i as isize)).bg_free_inodes_count as u8 as u16
    } else {
      BUG_wrong_field_size();
    }
    // count overall free blocks
    (*sb).s_free_blocks_count =
      ((*sb).s_free_blocks_count as libc::c_uint).wrapping_add(free_blocks) as u32 as u32;
    i = i.wrapping_add(1);
    pos = pos.wrapping_add((8i32 as libc::c_uint).wrapping_mul(blocksize));
    n = n.wrapping_sub((8i32 as libc::c_uint).wrapping_mul(blocksize))
  }
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*sb).s_free_blocks_count = (*sb).s_free_blocks_count
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*sb).s_free_blocks_count = (*sb).s_free_blocks_count as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*sb).s_free_blocks_count = (*sb).s_free_blocks_count as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  // dump filesystem skeleton structures
  //	printf("Writing superblocks and filesystem accounting information: ");
  i = 0i32 as libc::c_uint;
  pos = first_block;
  while i < ngroups {
    // dump superblock and group descriptors and their backups
    if has_super(i) != 0 {
      // N.B. 1024 byte blocks are special
      PUT(
        (pos as u64)
          .wrapping_mul(blocksize as libc::c_ulong)
          .wrapping_add(
            (if 0i32 as libc::c_uint == i && 1024i32 as libc::c_uint != blocksize {
              1024i32
            } else {
              0i32
            }) as libc::c_ulong,
          ),
        sb as *mut libc::c_void,
        1024i32 as u32,
      );
      PUT(
        (pos as u64)
          .wrapping_mul(blocksize as libc::c_ulong)
          .wrapping_add(blocksize as libc::c_ulong),
        gd as *mut libc::c_void,
        group_desc_blocks.wrapping_mul(blocksize),
      );
    }
    i = i.wrapping_add(1);
    pos = pos.wrapping_add((8i32 as libc::c_uint).wrapping_mul(blocksize))
  }
  // zero boot sectors
  memset(buf as *mut libc::c_void, 0i32, blocksize as libc::c_ulong);
  // Disabled: standard mke2fs doesn't do this, and
  // on SPARC this destroys Sun disklabel.
  // Users who need/want zeroing can easily do it with dd.
  //PUT(0, buf, 1024); // N.B. 1024 <= blocksize, so buf[0..1023] contains zeros
  // zero inode tables
  i = 0i32 as libc::c_uint;
  while i < ngroups {
    n = 0i32 as libc::c_uint;
    while n < inode_table_blocks {
      PUT(
        ((if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
          (*gd.offset(i as isize)).bg_inode_table
        } else {
          BUG_wrong_field_size() as libc::c_uint
        })
        .wrapping_add(n) as u64)
          .wrapping_mul(blocksize as libc::c_ulong),
        buf as *mut libc::c_void,
        blocksize,
      );
      n = n.wrapping_add(1)
    }
    i = i.wrapping_add(1)
  }
  // prepare directory inode
  inode = buf as *mut ext2_inode;
  if ::std::mem::size_of::<u16>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*inode).i_mode = (0o40000i32
      | (0o400i32 | 0o200i32 | 0o100i32)
      | 0o400i32 >> 3i32
      | 0o400i32 >> 3i32 >> 3i32
      | 0o100i32 >> 3i32
      | 0o100i32 >> 3i32 >> 3i32) as u32 as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*inode).i_mode = (0o40000i32
      | (0o400i32 | 0o200i32 | 0o100i32)
      | 0o400i32 >> 3i32
      | 0o400i32 >> 3i32 >> 3i32
      | 0o100i32 >> 3i32
      | 0o100i32 >> 3i32 >> 3i32) as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*inode).i_mode = (0o40000i32
      | (0o400i32 | 0o200i32 | 0o100i32)
      | 0o400i32 >> 3i32
      | 0o400i32 >> 3i32 >> 3i32
      | 0o100i32 >> 3i32
      | 0o100i32 >> 3i32 >> 3i32) as u8 as u16
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*inode).i_mtime = timestamp as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*inode).i_mtime = timestamp as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*inode).i_mtime = timestamp as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*inode).i_atime = timestamp as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*inode).i_atime = timestamp as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*inode).i_atime = timestamp as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*inode).i_ctime = timestamp as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*inode).i_ctime = timestamp as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*inode).i_ctime = timestamp as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*inode).i_size = blocksize
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*inode).i_size = blocksize as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*inode).i_size = blocksize as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  // inode->i_blocks stores the number of 512 byte data blocks
  // (512, because it goes directly to struct stat without scaling)
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*inode).i_blocks = blocksize.wrapping_div(512i32 as libc::c_uint)
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*inode).i_blocks = blocksize.wrapping_div(512i32 as libc::c_uint) as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*inode).i_blocks = blocksize.wrapping_div(512i32 as libc::c_uint) as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  // dump root dir inode
  if ::std::mem::size_of::<u16>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*inode).i_links_count = 3i32 as u32 as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*inode).i_links_count = 3i32 as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*inode).i_links_count = 3i32 as u8 as u16
  } else {
    BUG_wrong_field_size(); // "/.", "/..", "/lost+found/.." point to this inode
  }
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*inode).i_block[0] = (if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong
    {
      (*gd.offset(0)).bg_inode_table
    } else {
      BUG_wrong_field_size() as libc::c_uint
    })
    .wrapping_add(inode_table_blocks)
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*inode).i_block[0] = (if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong
    {
      (*gd.offset(0)).bg_inode_table
    } else {
      BUG_wrong_field_size() as libc::c_uint
    })
    .wrapping_add(inode_table_blocks) as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*inode).i_block[0] = (if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong
    {
      (*gd.offset(0)).bg_inode_table
    } else {
      BUG_wrong_field_size() as libc::c_uint
    })
    .wrapping_add(inode_table_blocks) as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  PUT(
    ((if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
      (*gd.offset(0)).bg_inode_table
    } else {
      BUG_wrong_field_size() as libc::c_uint
    }) as u64)
      .wrapping_mul(blocksize as libc::c_ulong)
      .wrapping_add(((2i32 - 1i32) as libc::c_uint).wrapping_mul(inodesize) as libc::c_ulong),
    buf as *mut libc::c_void,
    inodesize,
  );
  // dump lost+found dir inode
  if ::std::mem::size_of::<u16>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*inode).i_links_count = 2i32 as u32 as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*inode).i_links_count = 2i32 as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*inode).i_links_count = 2i32 as u8 as u16
  } else {
    BUG_wrong_field_size(); // both "/lost+found" and "/lost+found/." point to this inode
  } // use next block
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*inode).i_size = lost_and_found_blocks.wrapping_mul(blocksize)
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*inode).i_size = lost_and_found_blocks.wrapping_mul(blocksize) as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*inode).i_size = lost_and_found_blocks.wrapping_mul(blocksize) as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*inode).i_blocks = lost_and_found_blocks
      .wrapping_mul(blocksize)
      .wrapping_div(512i32 as libc::c_uint)
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*inode).i_blocks = lost_and_found_blocks
      .wrapping_mul(blocksize)
      .wrapping_div(512i32 as libc::c_uint) as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*inode).i_blocks = lost_and_found_blocks
      .wrapping_mul(blocksize)
      .wrapping_div(512i32 as libc::c_uint) as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  n = (if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*inode).i_block[0]
  } else {
    BUG_wrong_field_size() as libc::c_uint
  })
  .wrapping_add(1i32 as libc::c_uint);
  i = 0i32 as libc::c_uint;
  while i < lost_and_found_blocks {
    if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
      (*inode).i_block[i as usize] = i.wrapping_add(n)
    } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
      (*inode).i_block[i as usize] = i.wrapping_add(n) as u16 as u32
    } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
      (*inode).i_block[i as usize] = i.wrapping_add(n) as u8 as u32
    } else {
      BUG_wrong_field_size();
    }
    i = i.wrapping_add(1)
  }
  //bb_error_msg("LAST BLOCK USED[%u]", i + n);
  PUT(
    ((if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
      (*gd.offset(0)).bg_inode_table
    } else {
      BUG_wrong_field_size() as libc::c_uint
    }) as u64)
      .wrapping_mul(blocksize as libc::c_ulong)
      .wrapping_add(((11i32 - 1i32) as libc::c_uint).wrapping_mul(inodesize) as libc::c_ulong),
    buf as *mut libc::c_void,
    inodesize,
  );
  // dump directories
  memset(buf as *mut libc::c_void, 0i32, blocksize as libc::c_ulong);
  dir = buf as *mut ext2_dir;
  // dump 2nd+ blocks of "/lost+found"
  if ::std::mem::size_of::<u16>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*dir).rec_len1 = blocksize as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*dir).rec_len1 = blocksize as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*dir).rec_len1 = blocksize as u8 as u16
  } else {
    BUG_wrong_field_size(); // e2fsck 1.41.4 compat (1.41.9 does not need this)
  }
  i = 1i32 as libc::c_uint;
  while i < lost_and_found_blocks {
    PUT(
      ((if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
        (*gd.offset(0)).bg_inode_table
      } else {
        BUG_wrong_field_size() as libc::c_uint
      })
      .wrapping_add(inode_table_blocks)
      .wrapping_add(1i32 as libc::c_uint)
      .wrapping_add(i) as u64)
        .wrapping_mul(blocksize as libc::c_ulong),
      buf as *mut libc::c_void,
      blocksize,
    );
    i = i.wrapping_add(1)
  }
  // dump 1st block of "/lost+found"
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*dir).inode1 = 11i32 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*dir).inode1 = 11i32 as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*dir).inode1 = 11i32 as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u16>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*dir).rec_len1 = 12i32 as u32 as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*dir).rec_len1 = 12i32 as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*dir).rec_len1 = 12i32 as u8 as u16
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u8>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*dir).name_len1 = 1i32 as u32 as u8
  } else if ::std::mem::size_of::<u8>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*dir).name_len1 = 1i32 as u16 as u8
  } else if ::std::mem::size_of::<u8>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*dir).name_len1 = 1i32 as u8
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u8>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*dir).file_type1 = 2i32 as u32 as u8
  } else if ::std::mem::size_of::<u8>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*dir).file_type1 = 2i32 as u16 as u8
  } else if ::std::mem::size_of::<u8>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*dir).file_type1 = 2i32 as u8
  } else {
    BUG_wrong_field_size();
  }
  (*dir).name1[0] = '.' as i32 as libc::c_char;
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*dir).inode2 = 2i32 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*dir).inode2 = 2i32 as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*dir).inode2 = 2i32 as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u16>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*dir).rec_len2 = blocksize.wrapping_sub(12i32 as libc::c_uint) as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*dir).rec_len2 = blocksize.wrapping_sub(12i32 as libc::c_uint) as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*dir).rec_len2 = blocksize.wrapping_sub(12i32 as libc::c_uint) as u8 as u16
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u8>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*dir).name_len2 = 2i32 as u32 as u8
  } else if ::std::mem::size_of::<u8>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*dir).name_len2 = 2i32 as u16 as u8
  } else if ::std::mem::size_of::<u8>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*dir).name_len2 = 2i32 as u8
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u8>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*dir).file_type2 = 2i32 as u32 as u8
  } else if ::std::mem::size_of::<u8>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*dir).file_type2 = 2i32 as u16 as u8
  } else if ::std::mem::size_of::<u8>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*dir).file_type2 = 2i32 as u8
  } else {
    BUG_wrong_field_size();
  }
  (*dir).name2[0] = '.' as i32 as libc::c_char;
  (*dir).name2[1] = '.' as i32 as libc::c_char;
  PUT(
    ((if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
      (*gd.offset(0)).bg_inode_table
    } else {
      BUG_wrong_field_size() as libc::c_uint
    })
    .wrapping_add(inode_table_blocks)
    .wrapping_add(1i32 as libc::c_uint) as u64)
      .wrapping_mul(blocksize as libc::c_ulong),
    buf as *mut libc::c_void,
    blocksize,
  );
  // dump root dir block
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*dir).inode1 = 2i32 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*dir).inode1 = 2i32 as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*dir).inode1 = 2i32 as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u16>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*dir).rec_len2 = 12i32 as u32 as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*dir).rec_len2 = 12i32 as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*dir).rec_len2 = 12i32 as u8 as u16
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*dir).inode3 = 11i32 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*dir).inode3 = 11i32 as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*dir).inode3 = 11i32 as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u16>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*dir).rec_len3 = blocksize
      .wrapping_sub(12i32 as libc::c_uint)
      .wrapping_sub(12i32 as libc::c_uint) as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*dir).rec_len3 = blocksize
      .wrapping_sub(12i32 as libc::c_uint)
      .wrapping_sub(12i32 as libc::c_uint) as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*dir).rec_len3 = blocksize
      .wrapping_sub(12i32 as libc::c_uint)
      .wrapping_sub(12i32 as libc::c_uint) as u8 as u16
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u8>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*dir).name_len3 = 10i32 as u32 as u8
  } else if ::std::mem::size_of::<u8>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*dir).name_len3 = 10i32 as u16 as u8
  } else if ::std::mem::size_of::<u8>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*dir).name_len3 = 10i32 as u8
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u8>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*dir).file_type3 = 2i32 as u32 as u8
  } else if ::std::mem::size_of::<u8>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*dir).file_type3 = 2i32 as u16 as u8
  } else if ::std::mem::size_of::<u8>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*dir).file_type3 = 2i32 as u8
  } else {
    BUG_wrong_field_size();
  }
  strcpy(
    (*dir).name3.as_mut_ptr(),
    b"lost+found\x00" as *const u8 as *const libc::c_char,
  );
  PUT(
    ((if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
      (*gd.offset(0)).bg_inode_table
    } else {
      BUG_wrong_field_size() as libc::c_uint
    })
    .wrapping_add(inode_table_blocks)
    .wrapping_add(0i32 as libc::c_uint) as u64)
      .wrapping_mul(blocksize as libc::c_ulong),
    buf as *mut libc::c_void,
    blocksize,
  );
  // cleanup
  xclose(3i32);
  return 0i32;
}
