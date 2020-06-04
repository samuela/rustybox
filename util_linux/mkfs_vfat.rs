use super::mkfs_ext2::BUG_wrong_field_size;
use crate::librb::size_t;
use crate::librb::uoff_t;

use libc;
use libc::fprintf;
use libc::ioctl;
use libc::off_t;
use libc::stat;
use libc::strcpy;
use libc::time;
use libc::time_t;
use libc::FILE;
extern "C" {
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  static mut stderr: *mut FILE;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hd_geometry {
  pub heads: libc::c_uchar,
  pub sectors: libc::c_uchar,
  pub cylinders: libc::c_ushort,
  pub start: libc::c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct floppy_struct {
  pub size: libc::c_uint,
  pub sect: libc::c_uint,
  pub head: libc::c_uint,
  pub track: libc::c_uint,
  pub stretch: libc::c_uint,
  pub gap: libc::c_uchar,
  pub rate: libc::c_uchar,
  pub spec1: libc::c_uchar,
  pub fmt_gap: libc::c_uchar,
  pub name: *const libc::c_char,
}

/* FAT32 filesystem looks like this:
 * sector -nn...-1: "hidden" sectors, all sectors before this partition
 * (-h hidden-sectors sets it. Useful only for boot loaders,
 *  they need to know _disk_ offset in order to be able to correctly
 *  address sectors relative to start of disk)
 * sector 0: boot sector
 * sector 1: info sector
 * sector 2: set aside for boot code which didn't fit into sector 0
 * ...(zero-filled sectors)...
 * sector B: backup copy of sector 0 [B set by -b backup-boot-sector]
 * sector B+1: backup copy of sector 1
 * sector B+2: backup copy of sector 2
 * ...(zero-filled sectors)...
 * sector R: FAT#1 [R set by -R reserved-sectors]
 * ...(FAT#1)...
 * sector R+fat_size: FAT#2
 * ...(FAT#2)...
 * sector R+fat_size*2: cluster #2
 * ...(cluster #2)...
 * sector R+fat_size*2+clust_size: cluster #3
 * ...(the rest is filled by clusters till the end)...
 */

pub type C2RustUnnamed = libc::c_uint;
pub const reserved_sect: C2RustUnnamed = 6;
// TODO: make these cmdline options
// dont forget sanity check: backup_boot_sector + 3 <= reserved_sect
pub const backup_boot_sector: C2RustUnnamed = 3;
// Perhaps this should remain constant
pub const info_sector_number: C2RustUnnamed = 1;
// how many blocks we try to read while testing

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct msdos_dir_entry {
  pub name: [libc::c_char; 11],
  pub attr: u8,
  pub lcase: u8,
  pub ctime_cs: u8,
  pub ctime: u16,
  pub cdate: u16,
  pub adate: u16,
  pub starthi: u16,
  pub time: u16,
  pub date: u16,
  pub start: u16,
  pub size: u32,
  /* 01c file size in bytes */
}
/* Example of boot sector's beginning:
0000  eb 58 90 4d 53 57 49 4e  34 2e 31 00 02 08 26 00  |...MSWIN4.1...&.|
0010  02 00 00 00 00 f8 00 00  3f 00 ff 00 3f 00 00 00  |........?...?...|
0020  54 9b d0 00 0d 34 00 00  00 00 00 00 02 00 00 00  |T....4..........|
0030  01 00 06 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
0040  80 00 29 71 df 51 e0 4e  4f 20 4e 41 4d 45 20 20  |..)q.Q.NO NAME  |
0050  20 20 46 41 54 33 32 20  20 20 33 c9 8e d1 bc f4  |  FAT32   3.....|
*/

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct msdos_volume_info {
  pub drive_number: u8,
  pub reserved: u8,
  pub ext_boot_sign: u8,
  pub volume_id32: u32,
  pub volume_label: [libc::c_char; 11],
  pub fs_type: [libc::c_char; 8],
  /* 052 typically "FATnn" */
}

/* 05a end. Total size 26 (0x1a) bytes */

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct msdos_boot_sector {
  pub boot_jump_and_sys_id: [libc::c_char; 11],
  pub bytes_per_sect: u16,
  pub sect_per_clust: u8,
  pub reserved_sect: u16,
  pub fats: u8,
  pub dir_entries: u16,
  pub volume_size_sect: u16,
  pub media_byte: u8,
  pub sect_per_fat: u16,
  pub sect_per_track: u16,
  pub heads: u16,
  pub hidden: u32,
  pub fat32_volume_size_sect: u32,
  pub fat32_sect_per_fat: u32,
  pub fat32_flags: u16,
  pub fat32_version: [u8; 2],
  pub fat32_root_cluster: u32,
  pub fat32_info_sector: u16,
  pub fat32_backup_boot: u16,
  pub reserved2: [u32; 3],
  pub vi: msdos_volume_info,
  pub boot_code: [libc::c_char; 420],
  pub boot_sign: u16,
  /* 1fe */
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct fat32_fsinfo {
  pub signature1: u32,
  pub reserved1: [u32; 120],
  pub signature2: u32,
  pub free_clusters: u32,
  pub next_cluster: u32,
  pub reserved2: [u32; 3],
  pub reserved3: u16,
  pub boot_sign: u16,
  /* 1fe */
}

pub type C2RustUnnamed_0 = libc::c_uint;
// [IGNORED] sector size
pub const OPT_v: C2RustUnnamed_0 = 65536;
// [IGNORED] location of backup boot sector
pub const OPT_c: C2RustUnnamed_0 = 4;
// verbose
// [IGNORED] sectors per cluster
// pub const OPT_S: C2RustUnnamed_0 = 32768;
// [IGNORED] number of reserved sectors
// pub const OPT_s: C2RustUnnamed_0 = 16384;
// [IGNORED] root directory entries
// pub const OPT_R: C2RustUnnamed_0 = 8192;
// volume label
// pub const OPT_r: C2RustUnnamed_0 = 4096;
// [IGNORED] message file
// pub const OPT_n: C2RustUnnamed_0 = 2048;
// [IGNORED] bad block filename
// pub const OPT_m: C2RustUnnamed_0 = 1024;
// [IGNORED] volume ID
// pub const OPT_l: C2RustUnnamed_0 = 512;
// [IGNORED] don't bark at entire disk devices
// pub const OPT_i: C2RustUnnamed_0 = 256;
// [IGNORED] number of hidden sectors
// pub const OPT_I: C2RustUnnamed_0 = 128;
// [IGNORED, implied 32] choose FAT size
// pub const OPT_h: C2RustUnnamed_0 = 64;
// [IGNORED] number of FATs
// pub const OPT_F: C2RustUnnamed_0 = 32;
// [IGNORED] create a new file
// pub const OPT_f: C2RustUnnamed_0 = 16;
// [IGNORED] check filesystem
// pub const OPT_C: C2RustUnnamed_0 = 8;
// [IGNORED] atari format
// pub const OPT_b: C2RustUnnamed_0 = 2;
// pub const OPT_A: C2RustUnnamed_0 = 1;

static mut boot_code: [i8; 59] = [
  14, 31, -66, 119, 124, -84, 34, -64, 116, 11, 86, -76, 14, -69, 7, 0, -51, 16, 94, -21, -16, 50,
  -28, -51, 22, -51, 25, -21, -2, 84, 104, 105, 115, 32, 105, 115, 32, 110, 111, 116, 32, 97, 32,
  98, 111, 111, 116, 97, 98, 108, 101, 32, 100, 105, 115, 107, 13, 10, 0,
];
/* compat:
 * mkdosfs 2.11 (12 Mar 2005)
 * Usage: mkdosfs [-A] [-c] [-C] [-v] [-I] [-l bad-block-file]
 *        [-b backup-boot-sector]
 *        [-m boot-msg-file] [-n volume-name] [-i volume-id]
 *        [-s sectors-per-cluster] [-S logical-sector-size]
 *        [-f number-of-FATs]
 *        [-h hidden-sectors] [-F fat-size] [-r root-dir-entries]
 *        [-R reserved-sectors]
 *        /dev/name [blocks]
 */
pub unsafe fn mkfs_vfat_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut st: stat = std::mem::zeroed();
  let mut volume_label: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
  let mut buf: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut device_name: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut volume_size_bytes: uoff_t = 0;
  let mut volume_size_sect: uoff_t = 0;
  let mut total_clust: u32 = 0;
  let mut volume_id: u32 = 0;
  let mut dev: libc::c_int = 0;
  let mut bytes_per_sect: libc::c_uint = 0;
  let mut sect_per_fat: libc::c_uint = 0;
  let mut opts: libc::c_uint = 0;
  let mut sect_per_track: u16 = 0;
  let mut media_byte: u8 = 0;
  let mut sect_per_clust: u8 = 0;
  let mut heads: u8 = 0;
  opts = crate::libbb::getopt32::getopt32(
    argv,
    b"^Ab:cCf:F:h:Ii:l:m:n:r:R:s:S:v\x00-1\x00" as *const u8 as *const libc::c_char,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
    &mut volume_label as *mut *const libc::c_char,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
    0 as *mut libc::c_void,
  );
  argv = argv.offset(optind as isize);
  // cache device name
  device_name = *argv.offset(0);
  // default volume ID = creation time
  volume_id = time(0 as *mut time_t) as u32;
  dev = crate::libbb::xfuncs_printf::xopen(device_name, 0o2i32);
  crate::libbb::xfuncs_printf::xfstat(dev, &mut st, device_name);
  //
  // Get image size and sector size
  //
  bytes_per_sect = 512i32 as libc::c_uint;
  if !(st.st_mode & 0o170000i32 as libc::c_uint == 0o60000i32 as libc::c_uint) {
    if !(st.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint) {
      if (*argv.offset(1)).is_null() {
        crate::libbb::verror_msg::bb_simple_error_msg_and_die(
          b"image size must be specified\x00" as *const u8 as *const libc::c_char,
        );
      }
    }
    // not a block device, skip bad sectors check
    opts &= !(OPT_c as libc::c_int) as libc::c_uint
  } else {
    let mut min_bytes_per_sect: libc::c_int = 0;
    // get true sector size
    // (parameter must be int*, not long* or size_t*)
    crate::libbb::xfuncs_printf::bb_xioctl(
      dev,
      0u32 << 0 + 8i32 + 8i32 + 14i32
        | (0x12i32 << 0 + 8i32) as libc::c_uint
        | (104i32 << 0) as libc::c_uint
        | (0i32 << 0 + 8i32 + 8i32) as libc::c_uint,
      &mut min_bytes_per_sect as *mut libc::c_int as *mut libc::c_void,
      b"BLKSSZGET\x00" as *const u8 as *const libc::c_char,
    );
    if min_bytes_per_sect > 512i32 {
      bytes_per_sect = min_bytes_per_sect as libc::c_uint;
      crate::libbb::verror_msg::bb_error_msg(
        b"for this device sector size is %u\x00" as *const u8 as *const libc::c_char,
        min_bytes_per_sect,
      );
    }
  }
  volume_size_bytes = crate::libbb::get_volsize::get_volume_size_in_bytes(
    dev,
    *argv.offset(1),
    1024i32 as libc::c_uint,
    1i32,
  );
  volume_size_sect = volume_size_bytes.wrapping_div(bytes_per_sect as libc::c_ulong);
  //
  // Find out or guess media parameters
  //
  media_byte = 0xf8i32 as u8;
  heads = 255i32 as u8;
  sect_per_track = 63i32 as u16;
  sect_per_clust = 1i32 as u8;
  let mut geometry: hd_geometry = hd_geometry {
    heads: 0,
    sectors: 0,
    cylinders: 0,
    start: 0,
  };
  // size (in sectors), sect (per track), head
  let mut param: floppy_struct = floppy_struct {
    size: 0,
    sect: 0,
    head: 0,
    track: 0,
    stretch: 0,
    gap: 0,
    rate: 0,
    spec1: 0,
    fmt_gap: 0,
    name: 0 as *const libc::c_char,
  };
  let mut current_block_50: u64;
  // N.B. whether to use HDIO_GETGEO or HDIO_REQ?
  if ioctl(
    dev,
    0x301i32 as libc::c_ulong,
    &mut geometry as *mut hd_geometry,
  ) == 0
    && geometry.sectors as libc::c_int != 0
    && geometry.heads as libc::c_int != 0
  {
    // hard drive
    sect_per_track = geometry.sectors as u16;
    heads = geometry.heads;
    current_block_50 = 3603352160459597315;
  } else {
    // floppy, loop, or regular file
    let mut not_floppy: libc::c_int = ioctl(
      dev,
      (2u32 << 0 + 8i32 + 8i32 + 14i32
        | (2i32 << 0 + 8i32) as libc::c_uint
        | (0x4i32 << 0) as libc::c_uint) as libc::c_ulong
        | (::std::mem::size_of::<floppy_struct>() as libc::c_ulong) << 0 + 8i32 + 8i32,
      &mut param as *mut floppy_struct,
    );
    if not_floppy == 0 {
      // floppy disk
      sect_per_track = param.sect as u16;
      heads = param.head as u8;
      volume_size_sect = param.size as uoff_t;
      volume_size_bytes = param.size.wrapping_mul(512i32 as libc::c_uint) as uoff_t
    }
    // setup the media descriptor byte
    match volume_size_sect {
      720 => {
        // 5.25", 2, 9, 40 - 360K
        media_byte = 0xfdi32 as u8;
        current_block_50 = 3392087639489470149;
      }
      1440 | 2400 => {
        // 3.5", 2, 9, 80 - 720K
        // 5.25", 2, 15, 80 - 1200K
        media_byte = 0xf9i32 as u8;
        current_block_50 = 3392087639489470149;
      }
      2880 | 5760 => {
        current_block_50 = 15819562344915545825;
      }
      _ => {
        // anything else
        if not_floppy != 0 {
          current_block_50 = 3603352160459597315;
        } else {
          current_block_50 = 15819562344915545825;
        }
      }
    }
    match current_block_50 {
      3603352160459597315 => {}
      _ => {
        match current_block_50 {
          15819562344915545825 =>
          // 3.5", 2, 18, 80 - 1440K
          // 3.5", 2, 36, 80 - 2880K
          {
            media_byte = 0xf0i32 as u8
          }
          _ => {}
        }
        // not floppy, but size matches floppy exactly.
        // perhaps it is a floppy image.
        // we already set media_byte as if it is a floppy,
        // now set sect_per_track and heads.
        heads = 2i32 as u8;
        sect_per_track =
          (volume_size_sect as libc::c_uint).wrapping_div(160i32 as libc::c_uint) as u16;
        if (sect_per_track as libc::c_int) < 9i32 {
          sect_per_track = 9i32 as u16
        }
        current_block_50 = 5372832139739605200;
      }
    }
  }
  match current_block_50 {
    3603352160459597315 => {
      /* For FAT32, try to do the same as M$'s format command
       * (see http://www.win.tue.nl/~aeb/linux/fs/fat/fatgen103.pdf p. 20):
       * fs size <= 260M: 0.5k clusters
       * fs size <=   8G: 4k clusters
       * fs size <=  16G: 8k clusters
       * fs size >   16G: 16k clusters
       */
      sect_per_clust = 1i32 as u8;
      if volume_size_bytes >= (260i32 * 1024i32 * 1024i32) as libc::c_ulong {
        sect_per_clust = 8i32 as u8;
        /* fight gcc: */
        /* "error: integer overflow in expression" */
        /* "error: right shift count >= width of type" */
        if ::std::mem::size_of::<off_t>() as libc::c_ulong > 4i32 as libc::c_ulong {
          let mut t: libc::c_uint = (volume_size_bytes >> 31i32 >> 1i32) as libc::c_uint;
          if t >= (8i32 / 4i32) as libc::c_uint {
            sect_per_clust = 16i32 as u8
          }
          if t >= (16i32 / 4i32) as libc::c_uint {
            sect_per_clust = 32i32 as u8
          }
        }
      }
    }
    _ => {}
  }
  //
  // Calculate number of clusters, sectors/cluster, sectors/FAT
  // (an initial guess for sect_per_clust should already be set)
  //
  // "mkdosfs -v -F 32 image5k 5" is the minimum:
  // 2 sectors for FATs and 2 data sectors
  if (volume_size_sect.wrapping_sub(reserved_sect as libc::c_int as libc::c_ulong) as off_t)
    < 4i32 as libc::c_long
  {
    crate::libbb::verror_msg::bb_simple_error_msg_and_die(
      b"the image is too small for FAT32\x00" as *const u8 as *const libc::c_char,
    );
  }
  sect_per_fat = 1i32 as libc::c_uint;
  loop {
    let mut spf_adj: libc::c_int = 0;
    let mut tcl: uoff_t = volume_size_sect
      .wrapping_sub(reserved_sect as libc::c_int as libc::c_ulong)
      .wrapping_sub((2i32 as libc::c_uint).wrapping_mul(sect_per_fat) as libc::c_ulong)
      .wrapping_div(sect_per_clust as libc::c_ulong);
    // tcl may be > MAX_CLUST_32 here, but it may be
    // because sect_per_fat is underestimated,
    // and with increased sect_per_fat it still may become
    // <= MAX_CLUST_32. Therefore, we do not check
    // against MAX_CLUST_32, but against a bigger const:
    if !(tcl > 0x80ffffffu32 as libc::c_ulong) {
      total_clust = tcl as u32; // fits in u32
                                // Every cluster needs 4 bytes in FAT. +2 entries since
                                // FAT has space for non-existent clusters 0 and 1.
                                // Let's see how many sectors that needs.
                                //May overflow at "*4":
                                //spf_adj = ((total_clust+2) * 4 + bytes_per_sect-1) / bytes_per_sect - sect_per_fat;
                                //Same in the more obscure, non-overflowing form:
      spf_adj = total_clust
        .wrapping_add(2i32 as libc::c_uint)
        .wrapping_add(bytes_per_sect.wrapping_div(4i32 as libc::c_uint))
        .wrapping_sub(1i32 as libc::c_uint)
        .wrapping_div(bytes_per_sect.wrapping_div(4i32 as libc::c_uint))
        .wrapping_sub(sect_per_fat) as libc::c_int;
      if spf_adj <= 0 {
        // do not need to adjust sect_per_fat.
        // so, was total_clust too big after all?
        if total_clust <= 0xffffff0i32 as libc::c_uint {
          break; // no
        }
      } else {
        // adjust sect_per_fat, go back and recalc total_clust
        // (note: just "sect_per_fat += spf_adj" isn't ok)
        sect_per_fat = sect_per_fat.wrapping_add(
          (spf_adj as libc::c_uint).wrapping_div(2i32 as libc::c_uint) | 1i32 as libc::c_uint,
        );
        continue;
      }
    }
    // yes, total_clust is _a bit_ too big
    if sect_per_clust as libc::c_int == 128i32 {
      crate::libbb::verror_msg::bb_simple_error_msg_and_die(
        b"can\'t make FAT32 with >128 sectors/cluster\x00" as *const u8 as *const libc::c_char,
      );
    }
    sect_per_clust = (sect_per_clust as libc::c_int * 2i32) as u8;
    sect_per_fat = sect_per_fat.wrapping_div(2i32 as libc::c_uint) | 1i32 as libc::c_uint
  }
  //
  // Print info
  //
  if opts & OPT_v as libc::c_int as libc::c_uint != 0 {
    fprintf(stderr,
                b"Device \'%s\':\nheads:%u, sectors/track:%u, bytes/sector:%u\nmedia descriptor:%02x\ntotal sectors:%lu, clusters:%u, sectors/cluster:%u\nFATs:2, sectors/FAT:%u\nvolumeID:%08x, label:\'%s\'\n\x00"
                    as *const u8 as *const libc::c_char, device_name,
                heads as libc::c_int, sect_per_track as libc::c_int,
                bytes_per_sect, media_byte as libc::c_int, volume_size_sect,
                total_clust as libc::c_int, sect_per_clust as libc::c_int,
                sect_per_fat, volume_id as libc::c_int, volume_label);
  }
  //
  // Write filesystem image sequentially (no seeking)
  //
  // (a | b) is poor man's max(a, b)
  let mut bufsize: libc::c_uint = reserved_sect as libc::c_int as libc::c_uint;
  bufsize |= 2i32 as libc::c_uint;
  bufsize |= sect_per_clust as libc::c_uint;
  buf = crate::libbb::xfuncs_printf::xzalloc(bufsize.wrapping_mul(bytes_per_sect) as size_t)
    as *mut libc::c_char;
  //bufsize |= sect_per_fat; // can be quite large
  // use this instead
  // boot and fsinfo sectors, and their copies
  let mut boot_blk: *mut msdos_boot_sector = buf as *mut libc::c_void as *mut msdos_boot_sector;
  let mut info: *mut fat32_fsinfo =
    buf.offset(bytes_per_sect as isize) as *mut libc::c_void as *mut fat32_fsinfo;
  strcpy(
    (*boot_blk).boot_jump_and_sys_id.as_mut_ptr(),
    b"\xebX\x90mkdosfs\x00" as *const u8 as *const libc::c_char,
  );
  if ::std::mem::size_of::<u16>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*boot_blk).bytes_per_sect = bytes_per_sect as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*boot_blk).bytes_per_sect = bytes_per_sect as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*boot_blk).bytes_per_sect = bytes_per_sect as u8 as u16
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u8>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*boot_blk).sect_per_clust = sect_per_clust as u32 as u8
  } else if ::std::mem::size_of::<u8>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*boot_blk).sect_per_clust = sect_per_clust as u16 as u8
  } else if ::std::mem::size_of::<u8>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*boot_blk).sect_per_clust = sect_per_clust
  } else {
    BUG_wrong_field_size();
  }
  // cast in needed on big endian to suppress a warning
  if ::std::mem::size_of::<u16>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*boot_blk).reserved_sect = reserved_sect as libc::c_int as u16 as u32 as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*boot_blk).reserved_sect = reserved_sect as libc::c_int as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*boot_blk).reserved_sect = reserved_sect as libc::c_int as u16 as u8 as u16
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u8>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*boot_blk).fats = 2i32 as u32 as u8
  } else if ::std::mem::size_of::<u8>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*boot_blk).fats = 2i32 as u16 as u8
  } else if ::std::mem::size_of::<u8>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*boot_blk).fats = 2i32 as u8
  } else {
    BUG_wrong_field_size();
  }
  //STORE_LE(boot_blk->dir_entries, 0); // for FAT32, stays 0
  if volume_size_sect <= 0xffffi32 as libc::c_ulong {
    if ::std::mem::size_of::<u16>() as libc::c_ulong == 4i32 as libc::c_ulong {
      (*boot_blk).volume_size_sect = volume_size_sect as u32 as u16
    } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 2i32 as libc::c_ulong {
      (*boot_blk).volume_size_sect = volume_size_sect as u16
    } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 1i32 as libc::c_ulong {
      (*boot_blk).volume_size_sect = volume_size_sect as u8 as u16
    } else {
      BUG_wrong_field_size();
    }
  }
  if ::std::mem::size_of::<u8>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*boot_blk).media_byte = media_byte as u32 as u8
  } else if ::std::mem::size_of::<u8>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*boot_blk).media_byte = media_byte as u16 as u8
  } else if ::std::mem::size_of::<u8>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*boot_blk).media_byte = media_byte
  } else {
    BUG_wrong_field_size();
  }
  // wrong: this would make Linux think that it's fat12/16:
  //if (sect_per_fat <= 0xffff)
  //	STORE_LE(boot_blk->sect_per_fat, sect_per_fat);
  // works:
  //STORE_LE(boot_blk->sect_per_fat, 0);
  if ::std::mem::size_of::<u16>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*boot_blk).sect_per_track = sect_per_track as u32 as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*boot_blk).sect_per_track = sect_per_track
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*boot_blk).sect_per_track = sect_per_track as u8 as u16
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u16>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*boot_blk).heads = heads as u32 as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*boot_blk).heads = heads as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*boot_blk).heads = heads as u16
  } else {
    BUG_wrong_field_size();
  }
  //STORE_LE(boot_blk->hidden, 0);
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*boot_blk).fat32_volume_size_sect = volume_size_sect as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*boot_blk).fat32_volume_size_sect = volume_size_sect as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*boot_blk).fat32_volume_size_sect = volume_size_sect as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*boot_blk).fat32_sect_per_fat = sect_per_fat
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*boot_blk).fat32_sect_per_fat = sect_per_fat as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*boot_blk).fat32_sect_per_fat = sect_per_fat as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  //STORE_LE(boot_blk->fat32_flags, 0);
  //STORE_LE(boot_blk->fat32_version[2], 0,0);
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*boot_blk).fat32_root_cluster = 2
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*boot_blk).fat32_root_cluster = 2
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*boot_blk).fat32_root_cluster = 2
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u16>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*boot_blk).fat32_info_sector = info_sector_number as libc::c_int as u32 as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*boot_blk).fat32_info_sector = info_sector_number as libc::c_int as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*boot_blk).fat32_info_sector = info_sector_number as libc::c_int as u8 as u16
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u16>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*boot_blk).fat32_backup_boot = backup_boot_sector as libc::c_int as u32 as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*boot_blk).fat32_backup_boot = backup_boot_sector as libc::c_int as u16
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*boot_blk).fat32_backup_boot = backup_boot_sector as libc::c_int as u8 as u16
  } else {
    BUG_wrong_field_size();
  }
  //STORE_LE(boot_blk->reserved2[3], 0,0,0);
  if ::std::mem::size_of::<u8>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*boot_blk).vi.ext_boot_sign = 0x29
  } else if ::std::mem::size_of::<u8>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*boot_blk).vi.ext_boot_sign = 0x29
  } else if ::std::mem::size_of::<u8>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*boot_blk).vi.ext_boot_sign = 0x29
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*boot_blk).vi.volume_id32 = volume_id
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*boot_blk).vi.volume_id32 = volume_id as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*boot_blk).vi.volume_id32 = volume_id as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  memcpy(
    (*boot_blk).vi.fs_type.as_mut_ptr() as *mut libc::c_void,
    b"FAT32   \x00" as *const u8 as *const libc::c_char as *const libc::c_void,
    ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
  );
  strncpy(
    (*boot_blk).vi.volume_label.as_mut_ptr(),
    volume_label,
    ::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong,
  );
  memcpy(
    (*boot_blk).boot_code.as_mut_ptr() as *mut libc::c_void,
    boot_code.as_ptr() as *const libc::c_void,
    ::std::mem::size_of::<[libc::c_char; 59]>() as libc::c_ulong,
  );
  if ::std::mem::size_of::<u16>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*boot_blk).boot_sign = 0xaa55
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*boot_blk).boot_sign = 0xaa55
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*boot_blk).boot_sign = 85 // 0xaa55u8 as u16
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*info).signature1 = 0x41615252
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*info).signature1 = 21074 // 0x41615252u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*info).signature1 = 82 // 0x41615252u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*info).signature2 = 0x61417272
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*info).signature2 = 29298 // 0x61417272u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*info).signature2 = 114 // 0x61417272u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  // we've allocated cluster 2 for the root dir
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*info).free_clusters = total_clust.wrapping_sub(1i32 as libc::c_uint)
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*info).free_clusters = total_clust.wrapping_sub(1i32 as libc::c_uint) as u16 as u32
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*info).free_clusters = total_clust.wrapping_sub(1i32 as libc::c_uint) as u8 as u32
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u32>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*info).next_cluster = 2
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*info).next_cluster = 2
  } else if ::std::mem::size_of::<u32>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*info).next_cluster = 2
  } else {
    BUG_wrong_field_size();
  }
  if ::std::mem::size_of::<u16>() as libc::c_ulong == 4i32 as libc::c_ulong {
    (*info).boot_sign = 0xaa55
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 2i32 as libc::c_ulong {
    (*info).boot_sign = 0xaa55
  } else if ::std::mem::size_of::<u16>() as libc::c_ulong == 1i32 as libc::c_ulong {
    (*info).boot_sign = 85 // 0xaa55u8 as u16
  } else {
    BUG_wrong_field_size();
  }
  // 1st copy
  crate::libbb::xfuncs_printf::xwrite(
    dev,
    buf as *const libc::c_void,
    bytes_per_sect.wrapping_mul(backup_boot_sector as libc::c_int as libc::c_uint) as size_t,
  );
  // 2nd copy and possibly zero sectors
  crate::libbb::xfuncs_printf::xwrite(
    dev,
    buf as *const libc::c_void,
    bytes_per_sect.wrapping_mul(
      (reserved_sect as libc::c_int - backup_boot_sector as libc::c_int) as libc::c_uint,
    ) as size_t,
  );
  // file allocation tables
  let mut i: libc::c_uint = 0;
  let mut j: libc::c_uint = 0;
  let mut fat: *mut libc::c_uchar = buf as *mut libc::c_void as *mut libc::c_uchar;
  memset(
    buf as *mut libc::c_void,
    0,
    bytes_per_sect.wrapping_mul(2i32 as libc::c_uint) as libc::c_ulong,
  );
  // initial FAT entries
  *(fat as *mut u32).offset(0) = (0xfffff00i32 | media_byte as libc::c_int) as u32;
  *(fat as *mut u32).offset(1) = 0xffffffffu32;
  // mark cluster 2 as EOF (used for root dir)
  *(fat as *mut u32).offset(2) = 0xffffff8i32 as u32;
  i = 0 as libc::c_uint;
  while i < 2i32 as libc::c_uint {
    crate::libbb::xfuncs_printf::xwrite(dev, buf as *const libc::c_void, bytes_per_sect as size_t);
    j = 1i32 as libc::c_uint;
    while j < sect_per_fat {
      crate::libbb::xfuncs_printf::xwrite(
        dev,
        buf.offset(bytes_per_sect as isize) as *const libc::c_void,
        bytes_per_sect as size_t,
      );
      j = j.wrapping_add(1)
    }
    i = i.wrapping_add(1)
  }
  // root directory
  // empty directory is just a set of zero bytes
  memset(
    buf as *mut libc::c_void,
    0,
    (sect_per_clust as libc::c_uint).wrapping_mul(bytes_per_sect) as libc::c_ulong,
  );
  if *volume_label.offset(0) != 0 {
    // create dir entry for volume_label
    let mut de: *mut msdos_dir_entry = std::ptr::null_mut();
    de = buf as *mut libc::c_void as *mut msdos_dir_entry;
    strncpy(
      (*de).name.as_mut_ptr(),
      volume_label,
      ::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong,
    );
    if ::std::mem::size_of::<u8>() as libc::c_ulong == 4i32 as libc::c_ulong {
      (*de).attr = 8
    } else if ::std::mem::size_of::<u8>() as libc::c_ulong == 2i32 as libc::c_ulong {
      (*de).attr = 8
    } else if ::std::mem::size_of::<u8>() as libc::c_ulong == 1i32 as libc::c_ulong {
      (*de).attr = 8
    } else {
      BUG_wrong_field_size();
    }
  }
  crate::libbb::xfuncs_printf::xwrite(
    dev,
    buf as *const libc::c_void,
    (sect_per_clust as libc::c_uint).wrapping_mul(bytes_per_sect) as size_t,
  );
  return 0;
}
// cleanup
