use libc;
use libc::FILE;

use crate::librb::{
  __suseconds_t, off_t, size_t, smallint, stat, time_t, timespec, timeval, uint16_t, uint32_t,
  uint8_t, uoff_t,
};

extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);

  #[no_mangle]
  fn isatty(__fd: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn unlink(__name: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  static mut stderr: *mut FILE;

  #[no_mangle]
  fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;

  #[no_mangle]
  fn utimes(__file: *const libc::c_char, __tvp: *const timeval) -> libc::c_int;

  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn bb_copyfd_eof(fd1: libc::c_int, fd2: libc::c_int) -> off_t;

  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);

  #[no_mangle]
  fn xunlink(pathname: *const libc::c_char);

  #[no_mangle]
  fn open3_or_warn(
    pathname: *const libc::c_char,
    flags: libc::c_int,
    mode: libc::c_int,
  ) -> libc::c_int;

  #[no_mangle]
  fn xasprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;

  #[no_mangle]
  fn setup_unzip_on_fd(fd: libc::c_int, fail_if_not_compressed: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn open_zipped(fname: *const libc::c_char, fail_if_not_compressed: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn xclose(fd: libc::c_int);

  #[no_mangle]
  static mut option_mask32: uint32_t;

  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;

  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> uint32_t;

  #[no_mangle]
  fn xfunc_die() -> !;

  #[no_mangle]
  fn bb_show_usage() -> !;

  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);

  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;

  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);

  #[no_mangle]
  static mut applet_name: *const libc::c_char;

  #[no_mangle]
  fn init_transformer_state(xstate: *mut transformer_state_t);

  #[no_mangle]
  fn unpack_gz_stream(xstate: *mut transformer_state_t) -> libc::c_longlong;

  #[no_mangle]
  fn unpack_bz2_stream(xstate: *mut transformer_state_t) -> libc::c_longlong;

  #[no_mangle]
  fn unpack_lzma_stream(xstate: *mut transformer_state_t) -> libc::c_longlong;

  #[no_mangle]
  fn unpack_xz_stream(xstate: *mut transformer_state_t) -> libc::c_longlong;
}

// TODO: this is defined in bb_archive.h. We should pull it out into a common
// module.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct transformer_state_t {
  pub signature_skipped: smallint,
  pub xformer: Option<unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong>,
  pub src_fd: libc::c_int,
  pub dst_fd: libc::c_int,
  pub mem_output_size_max: size_t,
  pub mem_output_size: size_t,
  pub mem_output_buf: *mut libc::c_char,
  pub bytes_out: off_t,
  pub bytes_in: off_t,
  pub crc32: uint32_t,
  pub mtime: time_t,
  pub magic: C2RustUnnamed,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
  pub b: [uint8_t; 8],
  pub b16: [uint16_t; 4],
  pub b32: [uint32_t; 2],
}

pub type C2RustUnnamed_0 = libc::c_int;
pub const BBUNPK_OPT_STDOUT: C2RustUnnamed_0 = 1;
/* only some decompressors: */
pub const BBUNPK_OPT_KEEP: C2RustUnnamed_0 = 4;
pub const BBUNPK_OPT_VERBOSE: C2RustUnnamed_0 = 8;
pub const BBUNPK_SEAMLESS_MAGIC: C2RustUnnamed_0 = -2147483648;
pub const BBUNPK_OPT_FORCE: C2RustUnnamed_0 = 2;
pub const BBUNPK_OPT_TEST: C2RustUnnamed_0 = 64;
/* not included in BBUNPK_OPTSTR: */
pub const BBUNPK_OPT_DECOMPRESS: C2RustUnnamed_0 = 32;
pub const BBUNPK_OPT_QUIET: C2RustUnnamed_0 = 16;

/*
 * Common code for gunzip-like applets
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//kbuild:lib-$(CONFIG_ZCAT) += bbunzip.o
//kbuild:lib-$(CONFIG_GUNZIP) += bbunzip.o
//kbuild:lib-$(CONFIG_BZCAT) += bbunzip.o
//kbuild:lib-$(CONFIG_BUNZIP2) += bbunzip.o
/* lzop_main() uses bbunpack(), need this: */
//kbuild:lib-$(CONFIG_LZOP) += bbunzip.o
//kbuild:lib-$(CONFIG_LZOPCAT) += bbunzip.o
//kbuild:lib-$(CONFIG_UNLZOP) += bbunzip.o
/* bzip2_main() too: */
//kbuild:lib-$(CONFIG_BZIP2) += bbunzip.o
/* gzip_main() too: */
//kbuild:lib-$(CONFIG_GZIP) += bbunzip.o
unsafe extern "C" fn open_to_or_warn(
  mut to_fd: libc::c_int,
  mut filename: *const libc::c_char,
  mut flags: libc::c_int,
  mut mode: libc::c_int,
) -> libc::c_int {
  let mut fd: libc::c_int = open3_or_warn(filename, flags, mode);
  if fd < 0i32 {
    return 1i32;
  }
  xmove_fd(fd, to_fd);
  return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn append_ext(
  mut filename: *mut libc::c_char,
  mut expected_ext: *const libc::c_char,
) -> *mut libc::c_char {
  return xasprintf(
    b"%s.%s\x00" as *const u8 as *const libc::c_char,
    filename,
    expected_ext,
  );
}
#[no_mangle]
pub unsafe extern "C" fn bbunpack(
  mut argv: *mut *mut libc::c_char,
  mut unpacker: Option<unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong>,
  mut make_new_name: Option<
    unsafe extern "C" fn(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char,
  >,
  mut expected_ext: *const libc::c_char,
) -> libc::c_int {
  let mut del: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut current_block: u64;
  let mut stat_buf: stat = stat {
    st_dev: 0,
    st_ino: 0,
    st_nlink: 0,
    st_mode: 0,
    st_uid: 0,
    st_gid: 0,
    __pad0: 0,
    st_rdev: 0,
    st_size: 0,
    st_blksize: 0,
    st_blocks: 0,
    st_atim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_mtim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    st_ctim: timespec {
      tv_sec: 0,
      tv_nsec: 0,
    },
    __glibc_reserved: [0; 3],
  };
  let mut status: libc::c_longlong = 0i32 as libc::c_longlong;
  let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut new_name: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut exitcode: smallint = 0i32 as smallint;
  let mut xstate: transformer_state_t = transformer_state_t {
    signature_skipped: 0,
    xformer: None,
    src_fd: 0,
    dst_fd: 0,
    mem_output_size_max: 0,
    mem_output_size: 0,
    mem_output_buf: 0 as *mut libc::c_char,
    bytes_out: 0,
    bytes_in: 0,
    crc32: 0,
    mtime: 0,
    magic: C2RustUnnamed { b: [0; 8] },
  };
  loop {
    /* NB: new_name is *maybe* malloc'ed! */
    new_name = 0 as *mut libc::c_char; /* can be NULL - 'streaming' bunzip2 */
    filename = *argv;
    if !filename.is_null()
      && (*filename.offset(0) as libc::c_int == '-' as i32 && *filename.offset(1) == 0)
    {
      filename = 0 as *mut libc::c_char
    }
    /* Open src */
    if !filename.is_null() {
      if option_mask32 & BBUNPK_SEAMLESS_MAGIC as libc::c_int as libc::c_uint == 0 {
        if stat(filename, &mut stat_buf) != 0i32 {
          current_block = 18200770512078651936;
        } else if open_to_or_warn(0i32, filename, 0i32, 0i32) != 0 {
          current_block = 7403963845855106808;
        } else {
          current_block = 2668756484064249700;
        }
      } else {
        /* "clever zcat" with FILE */
        /* fail_if_not_compressed because zcat refuses uncompressed input */
        let mut fd: libc::c_int = open_zipped(filename, 1i32);
        if fd < 0i32 {
          current_block = 18200770512078651936;
        } else {
          xmove_fd(fd, 0i32);
          current_block = 2668756484064249700;
        }
      }
      match current_block {
        2668756484064249700 => {}
        7403963845855106808 => {}
        _ => {
          bb_simple_perror_msg(filename);
          current_block = 7403963845855106808;
        }
      }
    } else if option_mask32 & BBUNPK_SEAMLESS_MAGIC as libc::c_int as libc::c_uint != 0 {
      /* "clever zcat" on stdin */
      if setup_unzip_on_fd(0i32, 1i32) != 0 {
        current_block = 7403963845855106808;
      } else {
        current_block = 2668756484064249700;
      }
    } else {
      current_block = 2668756484064249700;
    }
    match current_block {
      2668756484064249700 => {
        /* Special cases: test, stdout */
        if option_mask32
          & (BBUNPK_OPT_STDOUT as libc::c_int | BBUNPK_OPT_TEST as libc::c_int) as libc::c_uint
          != 0
        {
          if option_mask32 & BBUNPK_OPT_TEST as libc::c_int as libc::c_uint != 0 {
            if open_to_or_warn(
              1i32,
              b"/dev/null\x00" as *const u8 as *const libc::c_char,
              0o1i32,
              0i32,
            ) != 0
            {
              xfunc_die();
            }
          }
          filename = 0 as *mut libc::c_char
        }
        /* Open dst if we are going to unpack to file */
        if !filename.is_null() {
          new_name = make_new_name.expect("non-null function pointer")(filename, expected_ext);
          if new_name.is_null() {
            bb_error_msg(
              b"%s: unknown suffix - ignored\x00" as *const u8 as *const libc::c_char,
              filename,
            );
            current_block = 7403963845855106808;
          } else {
            /* -f: overwrite existing output files */
            if option_mask32 & BBUNPK_OPT_FORCE as libc::c_int as libc::c_uint != 0 {
              unlink(new_name);
            }
            /* O_EXCL: "real" bunzip2 doesn't overwrite files */
            /* GNU gunzip does not bail out, but goes to next file */
            if open_to_or_warn(
              1i32,
              new_name,
              0o1i32 | 0o100i32 | 0o200i32,
              stat_buf.st_mode as libc::c_int,
            ) != 0
            {
              current_block = 7403963845855106808;
            } else {
              current_block = 1836292691772056875;
            }
          }
        } else {
          current_block = 1836292691772056875;
        }
        match current_block {
          7403963845855106808 => {}
          _ => {
            /* Check that the input is sane */
            if option_mask32 & BBUNPK_OPT_FORCE as libc::c_int as libc::c_uint == 0
              && isatty(0i32) != 0
            {
              bb_simple_error_msg_and_die(
                b"compressed data not read from terminal, use -f to force it\x00" as *const u8
                  as *const libc::c_char,
              );
            }
            if option_mask32 & BBUNPK_SEAMLESS_MAGIC as libc::c_int as libc::c_uint == 0 {
              init_transformer_state(&mut xstate);
              /*xstate.signature_skipped = 0; - already is */
              /*xstate.src_fd = STDIN_FILENO; - already is */
              xstate.dst_fd = 1i32;
              status = unpacker.expect("non-null function pointer")(&mut xstate);
              if status < 0i32 as libc::c_longlong {
                exitcode = 1i32 as smallint
              }
            } else if bb_copyfd_eof(0i32, 1i32) < 0i32 as libc::c_long {
              /* Disk full, tty closed, etc. No point in continuing */
              xfunc_die(); /* with error check! */
            }
            if option_mask32 & BBUNPK_OPT_STDOUT as libc::c_int as libc::c_uint == 0 {
              xclose(1i32);
            }
            if !filename.is_null() {
              del = new_name;
              if status >= 0i32 as libc::c_longlong {
                let mut new_name_len: libc::c_uint = 0;
                /* TODO: restore other things? */
                if xstate.mtime != 0i32 as libc::c_long {
                  let mut times: [timeval; 2] = [timeval {
                    tv_sec: 0,
                    tv_usec: 0,
                  }; 2];
                  times[0].tv_sec = xstate.mtime;
                  times[1].tv_sec = times[0].tv_sec;
                  times[0].tv_usec = 0i32 as __suseconds_t;
                  times[1].tv_usec = times[0].tv_usec;
                  /* ignoring errors */
                  utimes(new_name, times.as_mut_ptr() as *const timeval);
                }
                new_name_len = strlen(new_name) as libc::c_uint;
                /* Note: we closed it first.
                 * On some systems calling utimes
                 * then closing resets the mtime
                 * back to current time. */
                /* Restore source filename (unless tgz -> tar case) */
                if new_name == filename {
                  new_name_len = strlen(filename) as libc::c_uint;
                  *filename.offset(new_name_len as isize) = '.' as i32 as libc::c_char
                }
                /* Extreme bloat for gunzip compat */
                /* Some users do want this info... */
                if 1i32 != 0
                  && option_mask32 & BBUNPK_OPT_VERBOSE as libc::c_int as libc::c_uint != 0
                {
                  let mut percent: libc::c_uint = if status != 0 {
                    ((stat_buf.st_size as uoff_t).wrapping_mul(100u32 as libc::c_ulong)
                      as libc::c_ulonglong)
                      .wrapping_div(status as libc::c_ulonglong)
                  } else {
                    0i32 as libc::c_ulonglong
                  } as libc::c_uint;
                  fprintf(
                    stderr,
                    b"%s: %u%% - replaced with %.*s\n\x00" as *const u8 as *const libc::c_char,
                    filename,
                    100u32.wrapping_sub(percent),
                    new_name_len,
                    new_name,
                  );
                }
                /* Delete _source_ file */
                del = filename;
                if option_mask32 & BBUNPK_OPT_KEEP as libc::c_int as libc::c_uint != 0 {
                  /* ... unless -k */
                  del = 0 as *mut libc::c_char
                }
              } /* with error check! */
              if !del.is_null() {
                xunlink(del);
              }
              current_block = 7298725476856358922;
            } else {
              current_block = 16658872821858055392;
            }
          }
        }
      }
      _ => {}
    }
    match current_block {
      7403963845855106808 => {
        exitcode = 1i32 as smallint;
        current_block = 7298725476856358922;
      }
      _ => {}
    }
    match current_block {
      7298725476856358922 => {
        if new_name != filename {
          free(new_name as *mut libc::c_void);
        }
      }
      _ => {}
    }
    if !(!(*argv).is_null() && {
      argv = argv.offset(1);
      !(*argv).is_null()
    }) {
      break;
    }
  }
  if option_mask32 & BBUNPK_OPT_STDOUT as libc::c_int as libc::c_uint != 0 {
    xclose(1i32);
  }
  return exitcode as libc::c_int;
}
unsafe extern "C" fn make_new_name_generic(
  mut filename: *mut libc::c_char,
  mut expected_ext: *const libc::c_char,
) -> *mut libc::c_char {
  let mut extension: *mut libc::c_char = strrchr(filename, '.' as i32);
  if extension.is_null() || strcmp(extension.offset(1), expected_ext) != 0i32 {
    /* Mimic GNU gunzip - "real" bunzip2 tries to */
    /* unpack file anyway, to file.out */
    return 0 as *mut libc::c_char;
  }
  *extension = '\u{0}' as i32 as libc::c_char;
  return filename;
}
/*
 * Uncompress applet for busybox (c) 2002 Glenn McGrath
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//usage:#define uncompress_trivial_usage
//usage:       "[-cf] [FILE]..."
//usage:#define uncompress_full_usage "\n\n"
//usage:       "Decompress .Z file[s]\n"
//usage:     "\n	-c	Write to stdout"
//usage:     "\n	-f	Overwrite"
//config:config UNCOMPRESS
//config:	bool "uncompress (7.1 kb)"
//config:	default n  # ancient
//config:	help
//config:	uncompress is used to decompress archives created by compress.
//config:	Not much used anymore, replaced by gzip/gunzip.
//applet:IF_UNCOMPRESS(APPLET(uncompress, BB_DIR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_UNCOMPRESS) += bbunzip.o
/*
 * Gzip implementation for busybox
 *
 * Based on GNU gzip v1.2.4 Copyright (C) 1992-1993 Jean-loup Gailly.
 *
 * Originally adjusted for busybox by Sven Rudolph <sr1@inf.tu-dresden.de>
 * based on gzip sources
 *
 * Adjusted further by Erik Andersen <andersen@codepoet.org> to support files as
 * well as stdin/stdout, and to generally behave itself wrt command line
 * handling.
 *
 * General cleanup to better adhere to the style guide and make use of standard
 * busybox functions by Glenn McGrath
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 *
 * gzip (GNU zip) -- compress files with zip algorithm and 'compress' interface
 * Copyright (C) 1992-1993 Jean-loup Gailly
 * The unzip code was written and put in the public domain by Mark Adler.
 * Portions of the lzw code are derived from the public domain 'compress'
 * written by Spencer Thomas, Joe Orost, James Woods, Jim McKie, Steve Davies,
 * Ken Turkowski, Dave Mack and Peter Jannesen.
 */
//usage:#define gunzip_trivial_usage
//usage:       "[-cfkt] [FILE]..."
//usage:#define gunzip_full_usage "\n\n"
//usage:       "Decompress FILEs (or stdin)\n"
//usage:     "\n	-c	Write to stdout"
//usage:     "\n	-f	Force"
//usage:     "\n	-k	Keep input files"
//usage:     "\n	-t	Test file integrity"
//usage:
//usage:#define gunzip_example_usage
//usage:       "$ ls -la /tmp/BusyBox*\n"
//usage:       "-rw-rw-r--    1 andersen andersen   557009 Apr 11 10:55 /tmp/BusyBox-0.43.tar.gz\n"
//usage:       "$ gunzip /tmp/BusyBox-0.43.tar.gz\n"
//usage:       "$ ls -la /tmp/BusyBox*\n"
//usage:       "-rw-rw-r--    1 andersen andersen  1761280 Apr 14 17:47 /tmp/BusyBox-0.43.tar\n"
//usage:
//usage:#define zcat_trivial_usage
//usage:       "[FILE]..."
//usage:#define zcat_full_usage "\n\n"
//usage:       "Decompress to stdout"
//config:config GUNZIP
//config:	bool "gunzip (11 kb)"
//config:	default y
//config:	select FEATURE_GZIP_DECOMPRESS
//config:	help
//config:	gunzip is used to decompress archives created by gzip.
//config:	You can use the '-t' option to test the integrity of
//config:	an archive, without decompressing it.
//config:
//config:config ZCAT
//config:	bool "zcat (24 kb)"
//config:	default y
//config:	select FEATURE_GZIP_DECOMPRESS
//config:	help
//config:	Alias to "gunzip -c".
//config:
//config:config FEATURE_GUNZIP_LONG_OPTIONS
//config:	bool "Enable long options"
//config:	default y
//config:	depends on (GUNZIP || ZCAT) && LONG_OPTS
//applet:IF_GUNZIP(APPLET(gunzip, BB_DIR_BIN, BB_SUID_DROP))
//               APPLET_ODDNAME:name  main    location    suid_type     help
//applet:IF_ZCAT(APPLET_ODDNAME(zcat, gunzip, BB_DIR_BIN, BB_SUID_DROP, zcat))
unsafe extern "C" fn make_new_name_gunzip(
  mut filename: *mut libc::c_char,
  mut _expected_ext: *const libc::c_char,
) -> *mut libc::c_char {
  let mut extension: *mut libc::c_char = strrchr(filename, '.' as i32);
  if extension.is_null() {
    return 0 as *mut libc::c_char;
  }
  extension = extension.offset(1);
  if strcmp(
    extension,
    (b"tgz\x00" as *const u8 as *const libc::c_char).offset(1),
  ) == 0i32
  {
    *extension.offset(-1i32 as isize) = '\u{0}' as i32 as libc::c_char
  } else if strcmp(extension, b"tgz\x00" as *const u8 as *const libc::c_char) == 0i32 {
    filename = xstrdup(filename);
    extension = strrchr(filename, '.' as i32);
    *extension.offset(2) = 'a' as i32 as libc::c_char;
    *extension.offset(3) = 'r' as i32 as libc::c_char
  } else {
    return 0 as *mut libc::c_char;
  }
  return filename;
}
static mut gunzip_longopts: [libc::c_char; 47] = [
  115, 116, 100, 111, 117, 116, 0, 0, 99, 116, 111, 45, 115, 116, 100, 111, 117, 116, 0, 0, 99,
  102, 111, 114, 99, 101, 0, 0, 102, 116, 101, 115, 116, 0, 0, 116, 110, 111, 45, 110, 97, 109,
  101, 0, 0, 110, 0,
];
/*
 * Linux kernel build uses gzip -d -n. We accept and ignore it.
 * Man page says:
 * -n --no-name
 * gzip: do not save the original file name and time stamp.
 * (The original name is always saved if the name had to be truncated.)
 * gunzip: do not restore the original file name/time even if present
 * (remove only the gzip suffix from the compressed file name).
 * This option is the default when decompressing.
 * -N --name
 * gzip: always save the original file name and time stamp (this is the default)
 * gunzip: restore the original file name and time stamp if present.
 */
#[no_mangle]
pub unsafe extern "C" fn gunzip_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  getopt32long(
    argv,
    b"cfkvqdtn\x00" as *const u8 as *const libc::c_char,
    gunzip_longopts.as_ptr(),
  );
  argv = argv.offset(optind as isize);
  /* If called as zcat...
   * Normally, "zcat" is just "gunzip -c".
   * But if seamless magic is enabled, then we are much more clever.
   */
  if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(1) as libc::c_int == 'c' as i32) {
    option_mask32 |=
      (BBUNPK_OPT_STDOUT as libc::c_int | BBUNPK_SEAMLESS_MAGIC as libc::c_int) as libc::c_uint
  }
  return bbunpack(
    argv,
    Some(unpack_gz_stream as unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong),
    Some(
      make_new_name_gunzip
        as unsafe extern "C" fn(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char,
    ),
    0 as *const libc::c_char,
  );
}

/*
 * Busybox main internal header file
 *
 * Based in part on code from sash, Copyright (c) 1999 by David I. Bell
 * Permission has been granted to redistribute this code under GPL.
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* TODO: and glibc? */
/* There are two incompatible basename's, let's not use them! */
/* See the dirname/basename man page for details */
/* dirname,basename */
/* Don't do this here:
 * #include <sys/sysinfo.h>
 * Some linux/ includes pull in conflicting definition
 * of struct sysinfo (only in some toolchanins), which breaks build.
 * Include sys/sysinfo.h only in those files which need it.
 */
/* Just in case libc doesn't define some of these... */
//This breaks on bionic:
//# if !defined(__socklen_t_defined) && !defined(_SOCKLEN_T_DECLARED)
// /* We #define socklen_t *after* includes, otherwise we get
// * typedef redefinition errors from system headers
// * (in case "is it defined already" detection above failed)
// */
//#  define socklen_t bb_socklen_t
//   typedef unsigned socklen_t;
//# endif
//if this is still needed, add a fix along the lines of
//  ifdef SPECIFIC_BROKEN_LIBC_CHECK / typedef socklen_t / endif
//in platform.h instead!
/*
 * Use '%m' to append error string on platforms that support it,
 * '%s' and strerror() on those that don't.
 */
/*nothing*/
/* Some libc's forget to declare these, do it ourself */
/* klogctl is in libc's klog.h, but we cheat and not #include that */
/* Busybox does not use threads, we can speed up stdio. */
/* Above functions are required by POSIX.1-2008, below ones are extensions */
/* musl <= 1.1.15 does not support fflush_unlocked(NULL) */
//# undef  fflush
//# define fflush(stream) fflush_unlocked(stream)
/* Make all declarations hidden (-fvisibility flag only affects definitions) */
/* (don't include system headers after this until corresponding pop!) */
/* Tested to work correctly with all int types (IIRC :]) */
/* Large file support */
/* Note that CONFIG_LFS=y forces bbox to be built with all common ops
 * (stat, lseek etc) mapped to "largefile" variants by libc.
 * Practically it means that open() automatically has O_LARGEFILE added
 * and all filesize/file_offset parameters and struct members are "large"
 * (in today's world - signed 64bit). For full support of large files,
 * we need a few helper #defines (below) and careful use of off_t
 * instead of int/ssize_t. No lseek64(), O_LARGEFILE etc necessary */
/* CONFIG_LFS is on */
/* "long" is long enough on this system */
/* usage: sz = BB_STRTOOFF(s, NULL, 10); if (errno || sz < 0) die(); */
/* usage: printf("size: %"OFF_FMT"d (%"OFF_FMT"x)\n", sz, sz); */
/* scary. better ideas? (but do *test* them first!) */
/* Users report bionic to use 32-bit off_t even if LARGEFILE support is requested.
 * We misdetected that. Don't let it build:
 */
/* Some useful definitions */
/* Macros for min/max.  */
/* buffer allocation schemes */
/* glibc uses __errno_location() to get a ptr to errno */
/* We can just memorize it once - no multithreading in busybox :) */
/* dmalloc will redefine these to it's own implementation. It is safe
 * to have the prototypes here unconditionally.  */
/* After v = xrealloc_vector(v, SHIFT, idx) it's ok to use
 * at least v[idx] and v[idx+1], for all idx values.
 * SHIFT specifies how many new elements are added (1:2, 2:4, ..., 8:256...)
 * when all elements are used up. New elements are zeroed out.
 * xrealloc_vector(v, SHIFT, idx) *MUST* be called with consecutive IDXs -
 * skipping an index is a bad bug - it may miss a realloc!
 */
//TODO: supply a pointer to char[11] buffer (avoid statics)?
/* cp.c, mv.c, install.c depend on these values. CAREFUL when changing them! */
/* -p */
/* !-d */
/* -R */
/* -f */
/* -i */
/* -l */
/* -s */
/* -L */
/* -H */
/* -a = -pdR (mapped in cp.c) */
/* -r = -dR  (mapped in cp.c) */
/* -P = -d   (mapped in cp.c) */
/* -v */
/* -u */
/* -T */
/* --remove-destination */
/* bit 17 skipped for "cp --parents" */
/* cp --reflink=auto */
/* cp --reflink[=always] */
/*
 * Hole. cp may have some bits set here,
 * they should not affect remove_file()/copy_file()
 */
/* NB: without FILEUTILS_RECUR in flags, it will basically "cat"
 * the source, not copy (unless "source" is a directory).
 * This makes "cp /dev/null file" and "install /dev/null file" (!!!)
 * work coreutils-compatibly. */
/*ACTION_REVERSE      = (1 << 4), - unused */
/* more than enough for "/dev/ttyXXX" */
/* bb_copyfd_XX print read/write errors and return -1 if they occur */
/* "short" copy can be detected by return value < size */
/* this helper yells "short read!" if param is not -1 */
/* xxxx_strip version can modify its parameter:
 * "/"        -> "/"
 * "abc"      -> "abc"
 * "abc/def"  -> "def"
 * "abc/def/" -> "def" !!
 */
/* "abc/def/" -> "" and it never modifies 'path' */
/* Simpler version: does not special case "/" string */
/* NB: can violate const-ness (similarly to strchr) */
/* !RETURNS_MALLOC: it's a realloc-like function */
/* bb_signals(BB_FATAL_SIGS, handler) catches all signals which
 * otherwise would kill us, except for those resulting from bugs:
 * SIGSEGV, SIGILL, SIGFPE.
 * Other fatal signals not included (TODO?):
 * SIGBUS   Bus error (bad memory access)
 * SIGPOLL  Pollable event. Synonym of SIGIO
 * SIGPROF  Profiling timer expired
 * SIGSYS   Bad argument to routine
 * SIGTRAP  Trace/breakpoint trap
 *
 * The only known arch with some of these sigs not fitting
 * into 32 bits is parisc (SIGXCPU=33, SIGXFSZ=34, SIGSTKFLT=36).
 * Dance around with long long to guard against that...
 */
// Write to pipe with no readers
// Quit from keyboard
// Abort signal from abort(3)
// Timer signal from alarm(2)
// Virtual alarm clock
// CPU time limit exceeded
// File size limit exceeded
// Yes kids, these are also fatal!
/* Unlike signal() and bb_signals, sets handler with sigaction()
 * and in a way that while signal handler is run, no other signals
 * will be blocked; syscalls will not be restarted: */
/* syscalls like read() will be interrupted with EINTR: */
/* syscalls like read() won't be interrupted (though select/poll will be): */
/* Will do sigaction(signum, act, NULL): */
/* SIG_BLOCK/SIG_UNBLOCK all signals: */
/* Return old set in the same set: */
/* Standard handler which just records signo */
/* not FAST_FUNC! */
/* In this form code with pipes is much more readable */
/* Useful for having small structure members/global variables */
/* | AF_DECnet */
/* | AF_IPX */
/* SO_REUSEADDR allows a server to rebind to an address that is already
 * "in use" by old connections to e.g. previous server instance which is
 * killed or crashed. Without it bind will fail until all such connections
 * time out. Linux does not allow multiple live binds on same ip:port
 * regardless of SO_REUSEADDR (unlike some other flavors of Unix).
 * Turn it on before you call bind(). */
/* On Linux this never fails. */
/* NB: returns port in host byte order */
/* Create stream socket, and allocate suitable lsa.
 * (lsa of correct size and lsa->sa.sa_family (AF_INET/AF_INET6))
 * af == AF_UNSPEC will result in trying to create IPv6 socket,
 * and if kernel doesn't support it, fall back to IPv4.
 * This is useful if you plan to bind to resulting local lsa.
 */
/* Create server socket bound to bindaddr:port. bindaddr can be NULL,
 * numeric IP ("N.N.N.N") or numeric IPv6 address,
 * and can have ":PORT" suffix (for IPv6 use "[X:X:...:X]:PORT").
 * Only if there is no suffix, port argument is used */
/* NB: these set SO_REUSEADDR before bind */
/* Create client TCP socket connected to peer:port. Peer cannot be NULL.
 * Peer can be numeric IP ("N.N.N.N"), numeric IPv6 address or hostname,
 * and can have ":PORT" suffix (for IPv6 use "[X:X:...:X]:PORT").
 * If there is no suffix, port argument is used */
/* Connect to peer identified by lsa */
/* Get local address of bound or accepted socket */
/* Get remote address of connected or accepted socket */
/* Return malloc'ed len_and_sockaddr with socket address of host:port
 * Currently will return IPv4 or IPv6 sockaddrs only
 * (depending on host), but in theory nothing prevents e.g.
 * UNIX socket address being returned, IPX sockaddr etc...
 * On error does bb_error_msg and returns NULL */
/* Version which dies on error */
/* Same, useful if you want to force family (e.g. IPv6) */
/* Assign sin[6]_port member if the socket is an AF_INET[6] one,
 * otherwise no-op. Useful for ftp.
 * NB: does NOT do htons() internally, just direct assignment. */
/* Retrieve sin[6]_port or return -1 for non-INET[6] lsa's */
/* Reverse DNS. Returns NULL on failure. */
/* This one doesn't append :PORTNUM */
/* This one also doesn't fall back to dotted IP (returns NULL) */
/* inet_[ap]ton on steroids */
// "old" (ipv4 only) API
// users: traceroute.c hostname.c - use _list_ of all IPs
// Also mount.c and inetd.c are using gethostbyname(),
// + inet_common.c has additional IPv4-only stuff
/* opaque */
// RFC 5246
// sequence number
//   Each connection state contains a sequence number, which is
//   maintained separately for read and write states.  The sequence
//   number MUST be set to zero whenever a connection state is made the
//   active state.  Sequence numbers are of type uint64 and may not
//   exceed 2^64-1.
/*uint64_t read_seq64_be;*/
/*uint8_t *server_write_MAC_key;*/
//used by AES_GCM
/* 0 if argv[0] is NULL: */
/* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
 * But potentially slow, don't use in one-billion-times loops */
/* Note: does not use stdio, writes to fd 2 directly */
// gcc-4.1.1 still isn't good enough at optimizing it
// (+200 bytes compared to macro)
//static ALWAYS_INLINE
//int LONE_DASH(const char *s) { return s[0] == '-' && !s[1]; }
//static ALWAYS_INLINE
//int NOT_LONE_DASH(const char *s) { return s[0] != '-' || s[1]; }
/* Returns a string with unprintable chars replaced by '?' or
 * SUBST_WCHAR. This function is unicode-aware. */
/* Prints unprintable char ch as ^C or M-c to file
 * (M-c is used only if ch is ORed with PRINTABLE_META),
 * else it is printed as-is (except for ch = 0x9b) */
/* Return a string that is the printable representation of character ch.
 * Buffer must hold at least four characters. */
// NB: will return short read on error, not -1,
// if some data was read before error occurred
// Reads one line a-la fgets (but doesn't save terminating '\n').
// Reads byte-by-byte. Useful when it is important to not read ahead.
// Bytes are appended to pfx (which must be malloced, or NULL).
/* Reads block up to *maxsz_p (default: INT_MAX - 4095) */
/* Returns NULL if file can't be opened (default max size: INT_MAX - 4095) */
/* Never returns NULL */
/* Else use variable one (a bit more expensive) */
/* Autodetects gzip/bzip2 formats. fd may be in the middle of the file! */
/* Autodetects .gz etc */
/* lzma has no signature, need a little helper. NB: exist only for ENABLE_FEATURE_SEAMLESS_LZMA=y */
// NB: will return short write on error, not -1,
// if some data was written before error occurred
/* Close fd, but check for failures (some types of write errors) */
/* Reads and prints to stdout till eof, then closes FILE. Exits on error: */
/* Reads a line from a text file, up to a newline or NUL byte, inclusive.
 * Returns malloc'ed char*. If end is NULL '\n' isn't considered
 * end of line. If end isn't NULL, length of the chunk is stored in it.
 * Returns NULL if EOF/error.
 */
/* Reads up to (and including) TERMINATING_STRING: */
/* Same, with limited max size, and returns the length (excluding NUL): */
/* Chops off TERMINATING_STRING from the end: */
/* Reads up to (and including) "\n" or NUL byte: */
/* Chops off '\n' from the end, unlike fgets: */
/* Same, but doesn't try to conserve space (may have some slack after the end) */
/* extern char *xmalloc_fgetline_fast(FILE *file) FAST_FUNC RETURNS_MALLOC; */
/* Prints warning to stderr and returns NULL on failure: */
/* "Opens" stdin if filename is special, else just opens file: */
/* not FAST_FUNC! */
/* Wrapper which restarts poll on EINTR or ENOMEM.
 * On other errors complains [perror("poll")] and returns.
 * Warning! May take (much) longer than timeout_ms to return!
 * If this is a problem, use bare poll and open-code EINTR/ENOMEM handling */
/* Convert each alpha char in str to lower-case */
/* Returns a pointer past the formatted number, does NOT null-terminate */
/* Intelligent formatters of bignums */
/* If block_size == 0, display size without fractional part,
 * else display (size * block_size) with one decimal digit.
 * If display_unit == 0, show value no bigger than 1024 with suffix (K,M,G...),
 * else divide by display_unit and do not use suffix. */
/* "1024.0G" */
//TODO: provide pointer to buf (avoid statics)?
/* Put a string of hex bytes ("1b2e66fe"...), return advanced pointer */
/* Reverse */
/* Generate a UUID */
/* Last element is marked by mult == 0 */
/* Specialized: */
/* Using xatoi() instead of naive atoi() is not always convenient -
 * in many places people want *non-negative* values, but store them
 * in signed int. Therefore we need this one:
 * dies if input is not in [0, INT_MAX] range. Also will reject '-0' etc.
 * It should really be named xatoi_nonnegative (since it allows 0),
 * but that would be too long.
 */
/* Useful for reading port numbers */
/* These parse entries in /etc/passwd and /etc/group.  This is desirable
 * for BusyBox since we want to avoid using the glibc NSS stuff, which
 * increases target size and is often not needed on embedded systems.  */
/* wrapper: allows string to contain numeric uid or gid */
/* always sets uid and gid; returns 0 on failure */
/* always sets uid and gid; exits on failure */
/* chown-like handling of "user[:[group]" */
/* versions which cache results (useful for ps, ls etc) */
/* internally usernames are saved in fixed-sized char[] buffers */
/*
 * Returns (-1) terminated malloced result of getgroups().
 * Reallocs group_array (useful for repeated calls).
 * ngroups is an initial size of array. It is rounded up to 32 for realloc.
 * ngroups is updated on return.
 * ngroups can be NULL: bb_getgroups(NULL, NULL) is valid usage.
 * Dies on errors (on Linux, only xrealloc can cause this, not internal getgroups call).
 */
/* BB_EXECxx always execs (it's not doing NOFORK/NOEXEC stuff),
 * but it may exec busybox and call applet instead of searching PATH.
 */
/* xvfork() can't be a _function_, return after vfork in child mangles stack
 * in the parent. It must be a macro. */
/* NOMMU friendy fork+exec: */
/* wait4pid: unlike waitpid, waits ONLY for one process.
 * Returns sig + 0x180 if child is killed by signal.
 * It's safe to pass negative 'pids' from failed [v]fork -
 * wait4pid will return -1 (and will not clobber [v]fork's errno).
 * IOW: rc = wait4pid(spawn(argv));
 *      if (rc < 0) bb_perror_msg("%s", argv[0]);
 *      if (rc > 0) bb_error_msg("exit code: %d", rc & 0xff);
 */
/* ***********************************************************************/
/* spawn_and_wait/run_nofork_applet/run_applet_no_and_exit need to work */
/* carefully together to reinit some global state while not disturbing  */
/* other. Be careful if you change them. Consult docs/nofork_noexec.txt */
/* ***********************************************************************/
/* Same as wait4pid(spawn(argv)), but with NOFORK/NOEXEC if configured: */
/* Does NOT check that applet is NOFORK, just blindly runs it */
/* Helpers for daemonization.
 *
 * bb_daemonize(flags) = daemonize, does not compile on NOMMU
 *
 * bb_daemonize_or_rexec(flags, argv) = daemonizes on MMU (and ignores argv),
 *      rexec's itself on NOMMU with argv passed as command line.
 * Thus bb_daemonize_or_rexec may cause your <applet>_main() to be re-executed
 * from the start. (It will detect it and not reexec again second time).
 * You have to audit carefully that you don't do something twice as a result
 * (opening files/sockets, parsing config files etc...)!
 *
 * Both of the above will redirect fd 0,1,2 to /dev/null and drop ctty
 * (will do setsid()).
 *
 * fork_or_rexec(argv) = bare-bones fork on MMU,
 *      "vfork + re-exec ourself" on NOMMU. No fd redirection, no setsid().
 *      On MMU ignores argv.
 *
 * Helper for network daemons in foreground mode:
 *
 * bb_sanitize_stdio() = make sure that fd 0,1,2 are opened by opening them
 * to /dev/null if they are not.
 */
/* internal use */
//DAEMON_DOUBLE_FORK     = 1 << 4, /* double fork to avoid controlling tty */
/* Clear dangerous stuff, set PATH. Return 1 if was run by different user. */
/* For top, ps. Some argv[i] are replaced by malloced "-opt" strings */
/* { "-", NULL } */
/* BSD-derived getopt() functions require that optind be set to 1 in
 * order to reset getopt() state.  This used to be generally accepted
 * way of resetting getopt().  However, glibc's getopt()
 * has additional getopt() state beyond optind (specifically, glibc
 * extensions such as '+' and '-' at the start of the string), and requires
 * that optind be set to zero to reset its state.  BSD-derived versions
 * of getopt() misbehaved if optind is set to 0 in order to reset getopt(),
 * and glibc's getopt() used to coredump if optind is set 1 in order
 * to reset getopt().
 * Then BSD introduced additional variable "optreset" which should be
 * set to 1 in order to reset getopt().  Sigh.  Standards, anyone?
 *
 * By ~2008, OpenBSD 3.4 was changed to survive glibc-like optind = 0
 * (to interpret it as if optreset was set).
 */
/*def __GLIBC__*/
/* BSD style */
/* Having next pointer as a first member allows easy creation
 * of "llist-compatible" structs, and using llist_FOO functions
 * on them.
 */
/* BTW, surprisingly, changing API to
 *   llist_t *llist_add_to(llist_t *old_head, void *data)
 * etc does not result in smaller code... */
/* start_stop_daemon and udhcpc are special - they want
 * to create pidfiles regardless of FEATURE_PIDFILE */
/* True only if we created pidfile which is *file*, not /dev/null etc */
/* We need to export XXX_main from libbusybox
 * only if we build "individual" binaries
 */
/* Embedded script support */
/* Applets which are useful from another applets */
/* If shell needs them, they exist even if not enabled as applets */
/* Similar, but used by chgrp, not shell */
/* Used by ftpd */
/* Don't need IF_xxx() guard for these */
/* FEATURE_GZIP_DECOMPRESS */
/*
 * Modified for busybox by Glenn McGrath
 * Added support output to stdout by Thomas Lundquist <thomasez@zelow.no>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//usage:#define bunzip2_trivial_usage
//usage:       "[-cfk] [FILE]..."
//usage:#define bunzip2_full_usage "\n\n"
//usage:       "Decompress FILEs (or stdin)\n"
//usage:     "\n	-c	Write to stdout"
//usage:     "\n	-f	Force"
//usage:     "\n	-k	Keep input files"
//usage:#define bzcat_trivial_usage
//usage:       "[FILE]..."
//usage:#define bzcat_full_usage "\n\n"
//usage:       "Decompress to stdout"
//config:config BUNZIP2
//config:	bool "bunzip2 (8.7 kb)"
//config:	default y
//config:	select FEATURE_BZIP2_DECOMPRESS
//config:	help
//config:	bunzip2 is a compression utility using the Burrows-Wheeler block
//config:	sorting text compression algorithm, and Huffman coding. Compression
//config:	is generally considerably better than that achieved by more
//config:	conventional LZ77/LZ78-based compressors, and approaches the
//config:	performance of the PPM family of statistical compressors.
//config:
//config:	Unless you have a specific application which requires bunzip2, you
//config:	should probably say N here.
//config:
//config:config BZCAT
//config:	bool "bzcat (8.7 kb)"
//config:	default y
//config:	select FEATURE_BZIP2_DECOMPRESS
//config:	help
//config:	Alias to "bunzip2 -c".
//applet:IF_BUNZIP2(APPLET(bunzip2, BB_DIR_USR_BIN, BB_SUID_DROP))
//                APPLET_ODDNAME:name   main     location        suid_type     help
//applet:IF_BZCAT(APPLET_ODDNAME(bzcat, bunzip2, BB_DIR_USR_BIN, BB_SUID_DROP, bzcat))
#[no_mangle]
pub unsafe extern "C" fn bunzip2_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  getopt32(argv, b"cfkvqdt\x00" as *const u8 as *const libc::c_char);
  argv = argv.offset(optind as isize);
  if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(2) as libc::c_int == 'c' as i32) {
    /* bzcat */
    option_mask32 |= BBUNPK_OPT_STDOUT as libc::c_int as libc::c_uint
  }
  return bbunpack(
    argv,
    Some(
      unpack_bz2_stream as unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong,
    ),
    Some(
      make_new_name_generic
        as unsafe extern "C" fn(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char,
    ),
    b"bz2\x00" as *const u8 as *const libc::c_char,
  );
}
/*
 * Small lzma deflate implementation.
 * Copyright (C) 2006  Aurelien Jacobs <aurel@gnuage.org>
 *
 * Based on bunzip.c from busybox
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//usage:#define unlzma_trivial_usage
//usage:       "[-cfk] [FILE]..."
//usage:#define unlzma_full_usage "\n\n"
//usage:       "Decompress FILE (or stdin)\n"
//usage:     "\n	-c	Write to stdout"
//usage:     "\n	-f	Force"
//usage:     "\n	-k	Keep input files"
//usage:
//usage:#define lzma_trivial_usage
//usage:       "-d [-cfk] [FILE]..."
//usage:#define lzma_full_usage "\n\n"
//usage:       "Decompress FILE (or stdin)\n"
//usage:     "\n	-d	Decompress"
//usage:     "\n	-c	Write to stdout"
//usage:     "\n	-f	Force"
//usage:     "\n	-k	Keep input files"
//usage:
//usage:#define lzcat_trivial_usage
//usage:       "[FILE]..."
//usage:#define lzcat_full_usage "\n\n"
//usage:       "Decompress to stdout"
//config:config UNLZMA
//config:	bool "unlzma (7.5 kb)"
//config:	default y
//config:	help
//config:	unlzma is a compression utility using the Lempel-Ziv-Markov chain
//config:	compression algorithm, and range coding. Compression
//config:	is generally considerably better than that achieved by the bzip2
//config:	compressors.
//config:
//config:config LZCAT
//config:	bool "lzcat (7.5 kb)"
//config:	default y
//config:	help
//config:	Alias to "unlzma -c".
//config:
//config:config LZMA
//config:	bool "lzma -d"
//config:	default y
//config:	help
//config:	Enable this option if you want commands like "lzma -d" to work.
//config:	IOW: you'll get lzma applet, but it will always require -d option.
//applet:IF_UNLZMA(APPLET(unlzma, BB_DIR_USR_BIN, BB_SUID_DROP))
//                APPLET_ODDNAME:name   main    location        suid_type     help
//applet:IF_LZCAT(APPLET_ODDNAME(lzcat, unlzma, BB_DIR_USR_BIN, BB_SUID_DROP, lzcat))
//applet:IF_LZMA( APPLET_ODDNAME(lzma,  unlzma, BB_DIR_USR_BIN, BB_SUID_DROP, lzma))
//kbuild:lib-$(CONFIG_UNLZMA) += bbunzip.o
//kbuild:lib-$(CONFIG_LZCAT) += bbunzip.o
//kbuild:lib-$(CONFIG_LZMA) += bbunzip.o
#[no_mangle]
pub unsafe extern "C" fn unlzma_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opts: libc::c_int =
    getopt32(argv, b"cfkvqdt\x00" as *const u8 as *const libc::c_char) as libc::c_int;
  /* lzma without -d or -t? */
  if *applet_name.offset(2) as libc::c_int == 'm' as i32
    && opts & (BBUNPK_OPT_DECOMPRESS as libc::c_int | BBUNPK_OPT_TEST as libc::c_int) == 0
  {
    bb_show_usage();
  }
  /* lzcat? */
  if 1i32 != 0 && *applet_name.offset(2) as libc::c_int == 'c' as i32 {
    option_mask32 |= BBUNPK_OPT_STDOUT as libc::c_int as libc::c_uint
  }
  argv = argv.offset(optind as isize);
  return bbunpack(
    argv,
    Some(
      unpack_lzma_stream as unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong,
    ),
    Some(
      make_new_name_generic
        as unsafe extern "C" fn(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char,
    ),
    b"lzma\x00" as *const u8 as *const libc::c_char,
  );
}
//usage:#define unxz_trivial_usage
//usage:       "[-cfk] [FILE]..."
//usage:#define unxz_full_usage "\n\n"
//usage:       "Decompress FILE (or stdin)\n"
//usage:     "\n	-c	Write to stdout"
//usage:     "\n	-f	Force"
//usage:     "\n	-k	Keep input files"
//usage:
//usage:#define xz_trivial_usage
//usage:       "-d [-cfk] [FILE]..."
//usage:#define xz_full_usage "\n\n"
//usage:       "Decompress FILE (or stdin)\n"
//usage:     "\n	-d	Decompress"
//usage:     "\n	-c	Write to stdout"
//usage:     "\n	-f	Force"
//usage:     "\n	-k	Keep input files"
//usage:
//usage:#define xzcat_trivial_usage
//usage:       "[FILE]..."
//usage:#define xzcat_full_usage "\n\n"
//usage:       "Decompress to stdout"
//config:config UNXZ
//config:	bool "unxz (13 kb)"
//config:	default y
//config:	help
//config:	unxz is a unlzma successor.
//config:
//config:config XZCAT
//config:	bool "xzcat (13 kb)"
//config:	default y
//config:	help
//config:	Alias to "unxz -c".
//config:
//config:config XZ
//config:	bool "xz -d"
//config:	default y
//config:	help
//config:	Enable this option if you want commands like "xz -d" to work.
//config:	IOW: you'll get xz applet, but it will always require -d option.
//applet:IF_UNXZ(APPLET(unxz, BB_DIR_USR_BIN, BB_SUID_DROP))
//                APPLET_ODDNAME:name   main  location        suid_type     help
//applet:IF_XZCAT(APPLET_ODDNAME(xzcat, unxz, BB_DIR_USR_BIN, BB_SUID_DROP, xzcat))
//applet:IF_XZ(   APPLET_ODDNAME(xz,    unxz, BB_DIR_USR_BIN, BB_SUID_DROP, xz))
//kbuild:lib-$(CONFIG_UNXZ) += bbunzip.o
//kbuild:lib-$(CONFIG_XZCAT) += bbunzip.o
//kbuild:lib-$(CONFIG_XZ) += bbunzip.o
#[no_mangle]
pub unsafe extern "C" fn unxz_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut opts: libc::c_int =
    getopt32(argv, b"cfkvqdt\x00" as *const u8 as *const libc::c_char) as libc::c_int;
  /* xz without -d or -t? */
  if *applet_name.offset(2) as libc::c_int == '\u{0}' as i32
    && opts & (BBUNPK_OPT_DECOMPRESS as libc::c_int | BBUNPK_OPT_TEST as libc::c_int) == 0
  {
    bb_show_usage();
  }
  /* xzcat? */
  if 1i32 != 0 && *applet_name.offset(2) as libc::c_int == 'c' as i32 {
    option_mask32 |= BBUNPK_OPT_STDOUT as libc::c_int as libc::c_uint
  }
  argv = argv.offset(optind as isize);
  return bbunpack(
    argv,
    Some(unpack_xz_stream as unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong),
    Some(
      make_new_name_generic
        as unsafe extern "C" fn(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char,
    ),
    b"xz\x00" as *const u8 as *const libc::c_char,
  );
}
