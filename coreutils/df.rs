use libc;

use libc::free;
extern "C" {
  #[no_mangle]
  fn setmntent(__file: *const libc::c_char, __mode: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn getmntent(__stream: *mut FILE) -> *mut mntent;
  #[no_mangle]
  fn endmntent(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn statvfs(__file: *const libc::c_char, __buf: *mut statvfs) -> libc::c_int;

  #[no_mangle]
  fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
  #[no_mangle]
  fn make_human_readable_str(
    size: libc::c_ulonglong,
    block_size: libc::c_ulong,
    display_unit: libc::c_ulong,
  ) -> *const libc::c_char;
  #[no_mangle]
  static kmg_i_suffixes: [suffix_mult; 0];
  #[no_mangle]
  fn xatoull_range_sfx(
    str: *const libc::c_char,
    l: libc::c_ulonglong,
    u: libc::c_ulonglong,
    sfx: *const suffix_mult,
  ) -> libc::c_ulonglong;
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn find_mount_point(name: *const libc::c_char, subdir_too: libc::c_int) -> *mut mntent;
  //UNUSED: unsigned FAST_FUNC unicode_padding_to_width(unsigned width, const char *src);
  //UNUSED: char* FAST_FUNC unicode_conv_to_printable2(uni_stat_t *stats, const char *src, unsigned width, int flags);
  #[no_mangle]
  fn unicode_conv_to_printable(
    stats: *mut uni_stat_t,
    src: *const libc::c_char,
  ) -> *mut libc::c_char;
}

use libc::FILE;

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

pub type __fsblkcnt64_t = libc::c_ulong;
pub type __fsfilcnt64_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statvfs {
  pub f_bsize: libc::c_ulong,
  pub f_frsize: libc::c_ulong,
  pub f_blocks: __fsblkcnt64_t,
  pub f_bfree: __fsblkcnt64_t,
  pub f_bavail: __fsblkcnt64_t,
  pub f_files: __fsfilcnt64_t,
  pub f_ffree: __fsfilcnt64_t,
  pub f_favail: __fsfilcnt64_t,
  pub f_fsid: libc::c_ulong,
  pub f_flag: libc::c_ulong,
  pub f_namemax: libc::c_ulong,
  pub __f_spare: [libc::c_int; 6],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct uni_stat_t {
  pub byte_count: libc::c_uint,
  pub unicode_count: libc::c_uint,
  pub unicode_width: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct suffix_mult {
  pub suffix: [libc::c_char; 4],
  pub mult: libc::c_uint,
}
pub const OPT_POSIX: C2RustUnnamed = 2;
pub const OPT_FSTYPE: C2RustUnnamed = 4;
pub const OPT_INODE: C2RustUnnamed = 16;
pub const OPT_ALL: C2RustUnnamed = 8;
pub const OPT_HUMAN: C2RustUnnamed = 64;
pub const OPT_BSIZE: C2RustUnnamed = 32;
pub const OPT_MEGA: C2RustUnnamed = 128;
pub type C2RustUnnamed = libc::c_uint;
pub const OPT_KILO: C2RustUnnamed = 1;
#[inline(always)]
unsafe extern "C" fn xatoul_range_sfx(
  mut str: *const libc::c_char,
  mut l: libc::c_ulong,
  mut u: libc::c_ulong,
  mut sfx: *const suffix_mult,
) -> libc::c_ulong {
  return xatoull_range_sfx(str, l as libc::c_ulonglong, u as libc::c_ulonglong, sfx)
    as libc::c_ulong;
}

/*
 * Mini df implementation for busybox
 *
 * Copyright (C) 1999-2004 by Erik Andersen <andersen@codepoet.org>
 * based on original code by (I think) Bruce Perens <bruce@pixar.com>.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Mar 16, 2003      Manuel Novoa III   (mjn3@codepoet.org)
 *
 * Size reduction.  Removed floating point dependency.  Added error checking
 * on output.  Output stats on 0-sized filesystems if specifically listed on
 * the command line.  Properly round *-blocks, Used, and Available quantities.
 *
 * Aug 28, 2008      Bernhard Reutner-Fischer
 *
 * Implement -P and -B; better coreutils compat; cleanup
 */
//config:config DF
//config:	bool "df (6.8 kb)"
//config:	default y
//config:	help
//config:	df reports the amount of disk space used and available
//config:	on filesystems.
//config:
//config:config FEATURE_DF_FANCY
//config:	bool "Enable -a, -i, -B"
//config:	default y
//config:	depends on DF
//config:	help
//config:	-a Show all filesystems
//config:	-i Inodes
//config:	-B <SIZE> Blocksize
//applet:IF_DF(APPLET_NOEXEC(df, df, BB_DIR_BIN, BB_SUID_DROP, df))
//kbuild:lib-$(CONFIG_DF) += df.o
/* BB_AUDIT SUSv3 _NOT_ compliant -- option -t missing. */
/* http://www.opengroup.org/onlinepubs/007904975/utilities/df.html */
//usage:#define df_trivial_usage
//usage:	"[-Pk"
//usage:	IF_FEATURE_HUMAN_READABLE("mh")
//usage:	"T"
//usage:	IF_FEATURE_DF_FANCY("ai] [-B SIZE")
//usage:	"] [FILESYSTEM]..."
//usage:#define df_full_usage "\n\n"
//usage:       "Print filesystem usage statistics\n"
//usage:     "\n	-P	POSIX output format"
//usage:     "\n	-k	1024-byte blocks (default)"
//usage:	IF_FEATURE_HUMAN_READABLE(
//usage:     "\n	-m	1M-byte blocks"
//usage:     "\n	-h	Human readable (e.g. 1K 243M 2G)"
//usage:	)
//usage:     "\n	-T	Print filesystem type"
//usage:	IF_FEATURE_DF_FANCY(
//usage:     "\n	-a	Show all filesystems"
//usage:     "\n	-i	Inodes"
//usage:     "\n	-B SIZE	Blocksize"
//usage:	)
//usage:
//usage:#define df_example_usage
//usage:       "$ df\n"
//usage:       "Filesystem           1K-blocks      Used Available Use% Mounted on\n"
//usage:       "/dev/sda3              8690864   8553540    137324  98% /\n"
//usage:       "/dev/sda1                64216     36364     27852  57% /boot\n"
//usage:       "$ df /dev/sda3\n"
//usage:       "Filesystem           1K-blocks      Used Available Use% Mounted on\n"
//usage:       "/dev/sda3              8690864   8553540    137324  98% /\n"
//usage:       "$ POSIXLY_CORRECT=sure df /dev/sda3\n"
//usage:       "Filesystem         512B-blocks      Used Available Use% Mounted on\n"
//usage:       "/dev/sda3             17381728  17107080    274648  98% /\n"
//usage:       "$ POSIXLY_CORRECT=yep df -P /dev/sda3\n"
//usage:       "Filesystem          512-blocks      Used Available Capacity Mounted on\n"
//usage:       "/dev/sda3             17381728  17107080    274648      98% /\n"
#[no_mangle]
pub unsafe extern "C" fn df_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut df_disp_hr: libc::c_ulong = 1024i32 as libc::c_ulong;
  let mut status: libc::c_int = 0i32;
  let mut opt: libc::c_uint = 0;
  let mut mount_table: *mut FILE = 0 as *mut FILE;
  let mut mount_entry: *mut mntent = 0 as *mut mntent;
  let mut s: statvfs = statvfs {
    f_bsize: 0,
    f_frsize: 0,
    f_blocks: 0,
    f_bfree: 0,
    f_bavail: 0,
    f_files: 0,
    f_ffree: 0,
    f_favail: 0,
    f_fsid: 0,
    f_flag: 0,
    f_namemax: 0,
    __f_spare: [0; 6],
  };
  let mut disp_units_hdr: *const libc::c_char = 0 as *const libc::c_char;
  let mut chp: *mut libc::c_char = 0 as *mut libc::c_char;
  opt = getopt32(
    argv,
    b"^kPTaiB:hm\x00k-mB:m-Bk:B-km\x00" as *const u8 as *const libc::c_char,
    &mut chp as *mut *mut libc::c_char,
  );
  if opt & OPT_MEGA as libc::c_int as libc::c_uint != 0 {
    df_disp_hr = (1024i32 * 1024i32) as libc::c_ulong
  }
  if opt & OPT_BSIZE as libc::c_int as libc::c_uint != 0 {
    let mut current_block_7: u64;
    /* GNU coreutils 8.25 accepts "-BMiB" form too */
    let mut i: libc::c_int = 0;
    i = 0i32;
    loop {
      if !((*kmg_i_suffixes.as_ptr().offset(i as isize)).suffix[0] != 0) {
        current_block_7 = 9606288038608642794;
        break;
      }
      if strcmp(
        (*kmg_i_suffixes.as_ptr().offset(i as isize))
          .suffix
          .as_ptr(),
        chp,
      ) == 0i32
      {
        df_disp_hr = (*kmg_i_suffixes.as_ptr().offset(i as isize)).mult as libc::c_ulong;
        current_block_7 = 17833034027772472439;
        break;
      } else {
        i += 1
      }
    }
    match current_block_7 {
      9606288038608642794 => {
        /* Range used to disallow 0 */
        df_disp_hr = xatoul_range_sfx(
          chp,
          1i32 as libc::c_ulong,
          (9223372036854775807i64 as libc::c_ulong)
            .wrapping_mul(2u64)
            .wrapping_add(1u64),
          kmg_i_suffixes.as_ptr(),
        )
      }
      _ => {}
    }
  }
  /* From the manpage of df from coreutils-6.10:
   * Disk space is shown in 1K blocks by default, unless the environment
   * variable POSIXLY_CORRECT is set, in which case 512-byte blocks are used.
   */
  if !getenv(b"POSIXLY_CORRECT\x00" as *const u8 as *const libc::c_char).is_null() {
    /* TODO - a new libbb function? */
    df_disp_hr = 512i32 as libc::c_ulong
  }
  if opt & OPT_HUMAN as libc::c_int as libc::c_uint != 0 {
    df_disp_hr = 0i32 as libc::c_ulong;
    disp_units_hdr = b"     Size\x00" as *const u8 as *const libc::c_char
  }
  if opt & OPT_INODE as libc::c_int as libc::c_uint != 0 {
    disp_units_hdr = b"   Inodes\x00" as *const u8 as *const libc::c_char
  }
  if disp_units_hdr.is_null() {
    disp_units_hdr = xasprintf(
      b"%s-blocks\x00" as *const u8 as *const libc::c_char,
      make_human_readable_str(
        df_disp_hr as libc::c_ulonglong,
        0i32 as libc::c_ulong,
        (opt & OPT_POSIX as libc::c_int as libc::c_uint != 0) as libc::c_int as libc::c_ulong,
      ),
    )
  }
  printf(
    b"Filesystem           %s%-15sUsed Available %s Mounted on\n\x00" as *const u8
      as *const libc::c_char,
    if opt & OPT_FSTYPE as libc::c_int as libc::c_uint != 0 {
      b"Type       \x00" as *const u8 as *const libc::c_char
    } else {
      b"\x00" as *const u8 as *const libc::c_char
    },
    disp_units_hdr,
    if opt & OPT_POSIX as libc::c_int as libc::c_uint != 0 {
      b"Capacity\x00" as *const u8 as *const libc::c_char
    } else {
      b"Use%\x00" as *const u8 as *const libc::c_char
    },
  );
  mount_table = 0 as *mut FILE;
  argv = argv.offset(optind as isize);
  if (*argv.offset(0)).is_null() {
    mount_table = setmntent(
      b"/proc/mounts\x00" as *const u8 as *const libc::c_char,
      b"r\x00" as *const u8 as *const libc::c_char,
    );
    if mount_table.is_null() {
      bb_simple_perror_msg_and_die(b"/proc/mounts\x00" as *const u8 as *const libc::c_char);
    }
  }
  let mut current_block_76: u64;
  loop {
    let mut device: *const libc::c_char = 0 as *const libc::c_char;
    let mut mount_point: *const libc::c_char = 0 as *const libc::c_char;
    let mut fs_type: *const libc::c_char = 0 as *const libc::c_char;
    if !mount_table.is_null() {
      mount_entry = getmntent(mount_table);
      if mount_entry.is_null() {
        endmntent(mount_table);
        break;
      } else {
        current_block_76 = 5891011138178424807;
      }
    } else {
      let fresh0 = argv;
      argv = argv.offset(1);
      mount_point = *fresh0;
      if mount_point.is_null() {
        break;
      }
      mount_entry = find_mount_point(mount_point, 1i32);
      if mount_entry.is_null() {
        bb_error_msg(
          b"%s: can\'t find mount point\x00" as *const u8 as *const libc::c_char,
          mount_point,
        );
        current_block_76 = 6439760897239081377;
      } else {
        current_block_76 = 5891011138178424807;
      }
    }
    match current_block_76 {
      5891011138178424807 => {
        device = (*mount_entry).mnt_fsname;
        /* GNU coreutils 6.10 skips certain mounts, try to be compatible */
        if 1i32 != 0 && strcmp(device, b"rootfs\x00" as *const u8 as *const libc::c_char) == 0i32 {
          continue;
        }
        mount_point = (*mount_entry).mnt_dir;
        fs_type = (*mount_entry).mnt_type;
        if statvfs(mount_point, &mut s) != 0i32 {
          bb_simple_perror_msg(mount_point);
        } else {
          /* Some uclibc versions were seen to lose f_frsize
           * (kernel does return it, but then uclibc does not copy it)
           */
          if s.f_frsize == 0i32 as libc::c_ulong {
            s.f_frsize = s.f_bsize
          } /* 0% if blocks_total == 0, else... */
          if s.f_blocks > 0i32 as libc::c_ulong
            || mount_table.is_null()
            || opt & OPT_ALL as libc::c_int as libc::c_uint != 0
          {
            let mut blocks_used: libc::c_ulonglong = 0;
            let mut blocks_total: libc::c_ulonglong = 0;
            let mut blocks_percent_used: libc::c_uint = 0;
            if opt & OPT_INODE as libc::c_int as libc::c_uint != 0 {
              s.f_blocks = s.f_files;
              s.f_bfree = s.f_ffree;
              s.f_bavail = s.f_bfree;
              s.f_frsize = 1i32 as libc::c_ulong;
              if df_disp_hr != 0 {
                df_disp_hr = 1i32 as libc::c_ulong
              }
            }
            blocks_used = s.f_blocks.wrapping_sub(s.f_bfree) as libc::c_ulonglong;
            blocks_total = blocks_used.wrapping_add(s.f_bavail as libc::c_ulonglong);
            blocks_percent_used = blocks_total as libc::c_uint;
            if blocks_total != 0i32 as libc::c_ulonglong {
              /* Downscale sizes for narrower division */
              let mut u: libc::c_uint = 0;
              while blocks_total >= (2147483647i32 / 101i32) as libc::c_ulonglong {
                blocks_total >>= 1i32;
                blocks_used >>= 1i32
              }
              u = (blocks_used as libc::c_uint)
                .wrapping_mul(100u32)
                .wrapping_add((blocks_total as libc::c_uint).wrapping_div(2i32 as libc::c_uint));
              blocks_percent_used = u.wrapping_div(blocks_total as libc::c_uint)
            }
            let mut uni_stat: uni_stat_t = uni_stat_t {
              byte_count: 0,
              unicode_count: 0,
              unicode_width: 0,
            };
            let mut uni_dev: *mut libc::c_char = unicode_conv_to_printable(&mut uni_stat, device);
            if uni_stat.unicode_width > 20i32 as libc::c_uint
              && opt & OPT_POSIX as libc::c_int as libc::c_uint == 0
            {
              printf(
                b"%s\n%20s\x00" as *const u8 as *const libc::c_char,
                uni_dev,
                b"\x00" as *const u8 as *const libc::c_char,
              );
            } else {
              printf(
                b"%s%*s\x00" as *const u8 as *const libc::c_char,
                uni_dev,
                20i32 - uni_stat.unicode_width as libc::c_int,
                b"\x00" as *const u8 as *const libc::c_char,
              );
            }
            free(uni_dev as *mut libc::c_void);
            if opt & OPT_FSTYPE as libc::c_int as libc::c_uint != 0 {
              let mut uni_type: *mut libc::c_char =
                unicode_conv_to_printable(&mut uni_stat, fs_type);
              if uni_stat.unicode_width > 10i32 as libc::c_uint
                && opt & OPT_POSIX as libc::c_int as libc::c_uint == 0
              {
                printf(
                  b" %s\n%31s\x00" as *const u8 as *const libc::c_char,
                  uni_type,
                  b"\x00" as *const u8 as *const libc::c_char,
                );
              } else {
                printf(
                  b" %s%*s\x00" as *const u8 as *const libc::c_char,
                  uni_type,
                  10i32 - uni_stat.unicode_width as libc::c_int,
                  b"\x00" as *const u8 as *const libc::c_char,
                );
              }
              free(uni_type as *mut libc::c_void);
            }
            printf(
              b" %9s \x00" as *const u8 as *const libc::c_char,
              make_human_readable_str(s.f_blocks as libc::c_ulonglong, s.f_frsize, df_disp_hr),
            );
            printf(
              (b" %9s \x00" as *const u8 as *const libc::c_char).offset(1),
              make_human_readable_str(
                s.f_blocks.wrapping_sub(s.f_bfree) as libc::c_ulonglong,
                s.f_frsize,
                df_disp_hr,
              ),
            );
            printf(
              b"%9s %3u%% %s\n\x00" as *const u8 as *const libc::c_char,
              make_human_readable_str(s.f_bavail as libc::c_ulonglong, s.f_frsize, df_disp_hr),
              blocks_percent_used,
              mount_point,
            );
          }
          continue;
        }
      }
      _ => {}
    }
    status = 1i32
  }
  return status;
}
