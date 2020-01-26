use crate::archival::libarchive::bb_archive::archive_handle_t;
use crate::archival::libarchive::bb_archive::file_header_t;
use crate::libbb::xfuncs_printf::xmalloc;
use crate::librb::bb_uidgid_t;
use crate::librb::fd_pair;
use crate::librb::size_t;
use crate::librb::uoff_t;
use libc;
use libc::close;
use libc::fprintf;
use libc::free;
use libc::gid_t;
use libc::lstat;
use libc::mode_t;
use libc::off_t;
use libc::pid_t;
use libc::printf;
use libc::stat;
use libc::strcpy;
use libc::uid_t;
use libc::FILE;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn gnu_dev_major(__dev: libc::dev_t) -> libc::c_uint;

  #[no_mangle]
  fn gnu_dev_minor(__dev: libc::dev_t) -> libc::c_uint;

  #[no_mangle]
  static mut stdin: *mut FILE;

  #[no_mangle]
  static mut stderr: *mut FILE;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  fn mkdir(__path: *const libc::c_char, __mode: mode_t) -> libc::c_int;

  #[no_mangle]
  static mut option_mask32: u32;

  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];

  #[no_mangle]
  static cpio_TRAILER: [libc::c_char; 0];

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct globals {
  pub owner_ugid: bb_uidgid_t,
}

pub type C2RustUnnamed = libc::c_uint;
pub const OPT_2STDOUT: C2RustUnnamed = 16384;
pub const OPT_QUIET: C2RustUnnamed = 8192;
pub const OPT_PASSTHROUGH: C2RustUnnamed = 4096;
pub const OPT_FORMAT: C2RustUnnamed = 2048;
pub const OPT_CREATE: C2RustUnnamed = 1024;
pub const OPTBIT_2STDOUT: C2RustUnnamed = 14;
pub const OPTBIT_QUIET: C2RustUnnamed = 13;
pub const OPTBIT_PASSTHROUGH: C2RustUnnamed = 12;
pub const OPTBIT_FORMAT: C2RustUnnamed = 11;
pub const OPTBIT_CREATE: C2RustUnnamed = 10;
pub const OPTBIT_OWNER: C2RustUnnamed = 9;
pub const OPT_OWNER: C2RustUnnamed = 512;
pub const OPT_FILE: C2RustUnnamed = 256;
pub const OPT_DEREF: C2RustUnnamed = 128;
pub const OPT_PRESERVE_MTIME: C2RustUnnamed = 64;
pub const OPT_CREATE_LEADING_DIR: C2RustUnnamed = 32;
pub const OPT_VERBOSE: C2RustUnnamed = 16;
pub const OPT_UNCONDITIONAL: C2RustUnnamed = 8;
pub const OPT_NUL_TERMINATED: C2RustUnnamed = 4;
pub const OPT_TEST: C2RustUnnamed = 2;
pub const OPT_EXTRACT: C2RustUnnamed = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inodes_s {
  pub next: *mut inodes_s,
  pub names: *mut name_s,
  pub st: stat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct name_s {
  pub next: *mut name_s,
  pub name: [libc::c_char; 1],
}

unsafe extern "C" fn cpio_pad4(mut size: off_t) -> off_t {
  let mut i: libc::c_int = 0;
  i = (-size & 3i32 as libc::c_long) as libc::c_int;
  size += i as libc::c_long;
  loop {
    i -= 1;
    if !(i >= 0) {
      break;
    }
    crate::libbb::xfuncs_printf::bb_putchar('\u{0}' as i32);
  }
  return size;
}
/* Return value will become exit code.
 * It's ok to exit instead of return. */
#[inline(never)]
unsafe extern "C" fn cpio_o() -> libc::c_int {
  let mut links: *mut inodes_s = std::ptr::null_mut(); /* output bytes count */
  let mut bytes: off_t = 0 as off_t; /* line == NULL: EOF */
  let mut current_block_47: u64;
  loop {
    let mut name: *const libc::c_char = std::ptr::null();
    let mut line: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut st: stat = std::mem::zeroed();
    line = if option_mask32 & OPT_NUL_TERMINATED as libc::c_int as libc::c_uint != 0 {
      crate::libbb::get_line_from_file::bb_get_chunk_from_file(
        stdin,
        std::ptr::null_mut::<size_t>(),
      )
    } else {
      crate::libbb::get_line_from_file::xmalloc_fgetline(stdin)
    };
    if !line.is_null() {
      /* Strip leading "./[./]..." from the filename */
      name = line;
      while *name.offset(0) as libc::c_int == '.' as i32
        && *name.offset(1) as libc::c_int == '/' as i32
      {
        loop {
          name = name.offset(1);
          if !(*name as libc::c_int == '/' as i32) {
            break;
          }
        }
      }
      if *name == 0 {
        /* line is empty */
        free(line as *mut libc::c_void); /* paranoia */
        continue;
      } else if if option_mask32 & OPT_DEREF as libc::c_int as libc::c_uint != 0 {
        stat(name, &mut st)
      } else {
        lstat(name, &mut st)
      } != 0
      {
        current_block_47 = 16800535488375699875;
      } else {
        if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .owner_ugid
          .uid
          != std::u32::MAX
        {
          st.st_uid = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .owner_ugid
            .uid
        }
        if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .owner_ugid
          .gid
          != std::u32::MAX
        {
          st.st_gid = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .owner_ugid
            .gid
        }
        if !(st.st_mode & 0o170000i32 as libc::c_uint == 0o120000i32 as libc::c_uint
          || st.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint)
        {
          st.st_size = 0 as off_t
        }
        /* Store hardlinks for later processing, dont output them */
        if !(st.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint)
          && st.st_nlink > 1i32 as libc::c_ulong
        {
          let mut n: *mut name_s = std::ptr::null_mut();
          let mut l: *mut inodes_s = std::ptr::null_mut();
          /* Do we have this hardlink remembered? */
          l = links;
          loop {
            if l.is_null() {
              /* Not found: add new item to "links" list */
              l = crate::libbb::xfuncs_printf::xzalloc(
                ::std::mem::size_of::<inodes_s>() as libc::c_ulong
              ) as *mut inodes_s;
              (*l).st = st;
              (*l).next = links;
              links = l;
              break;
            } else {
              if (*l).st.st_ino == st.st_ino {
                break;
              }
              l = (*l).next
            }
          }
          /* Add new name to "l->names" list */
          n = xmalloc((::std::mem::size_of::<name_s>() as libc::c_ulong).wrapping_add(strlen(name)))
            as *mut name_s;
          strcpy((*n).name.as_mut_ptr(), name);
          (*n).next = (*l).names;
          (*l).names = n;
          free(line as *mut libc::c_void);
          continue;
        } else {
          current_block_47 = 14945149239039849694;
        }
      }
    } else {
      current_block_47 = 6299548793047770735;
    }
    loop {
      match current_block_47 {
        16800535488375699875 => {
          crate::libbb::perror_msg::bb_simple_perror_msg_and_die(name);
        }
        6299548793047770735 => {
          if !links.is_null() {
            /* Output hardlink's data */
            st = (*links).st;
            name = (*(*links).names).name.as_mut_ptr();
            (*links).names = (*(*links).names).next;
            /* NB: we leak links->names and/or links,
             * this is intended (we exit soon anyway) */
            if (*links).names.is_null() {
              links = (*links).next
            } else {
              st.st_size = 0 as off_t
            }
          } else {
            /* GNU cpio is reported to emit file data
             * only for the last instance. Mimic that. */
            /* If no (more) hardlinks to output,
             * output "trailer" entry */
            name = cpio_TRAILER.as_ptr();
            /* st.st_nlink = 1; - GNU cpio does this */
            memset(
              &mut st as *mut stat as *mut libc::c_void,
              0,
              ::std::mem::size_of::<stat>() as libc::c_ulong,
            );
          }
          current_block_47 = 14945149239039849694;
        }
        _ => {
          bytes += printf(
            b"070701%08X%08X%08X%08X%08X%08X%08X%08X%08X%08X%08X%08X00000000%s%c\x00" as *const u8
              as *const libc::c_char,
            st.st_ino as u32,
            st.st_mode,
            st.st_uid,
            st.st_gid,
            st.st_nlink as u32,
            st.st_mtime as u32,
            st.st_size as u32,
            gnu_dev_major(st.st_dev),
            gnu_dev_minor(st.st_dev),
            gnu_dev_major(st.st_rdev),
            gnu_dev_minor(st.st_rdev),
            strlen(name).wrapping_add(1i32 as libc::c_ulong) as libc::c_uint,
            name,
            '\u{0}' as i32,
          ) as libc::c_long;
          bytes = cpio_pad4(bytes);
          if st.st_size != 0 {
            if st.st_mode & 0o170000i32 as libc::c_uint == 0o120000i32 as libc::c_uint {
              /* st.st_size == 0 is a must, but for uniformity
               * in the output, we zero out everything */
              let mut lpath: *mut libc::c_char =
                crate::libbb::xreadlink::xmalloc_readlink_or_warn(name); /* S_ISREG */
              if lpath.is_null() {
                current_block_47 = 16800535488375699875;
                continue;
              }
              bytes += printf(b"%s\x00" as *const u8 as *const libc::c_char, lpath) as libc::c_long;
              free(lpath as *mut libc::c_void);
            } else {
              let mut fd: libc::c_int = crate::libbb::xfuncs_printf::xopen(name, 0);
              crate::libbb::xfuncs_printf::fflush_all();
              /* We must abort if file got shorter too! */
              crate::libbb::copyfd::bb_copyfd_exact_size(fd, 1i32, st.st_size);
              bytes += st.st_size;
              close(fd);
            }
            bytes = cpio_pad4(bytes)
          }
          if line.is_null() {
            if name != cpio_TRAILER.as_ptr() {
              current_block_47 = 6299548793047770735;
              continue;
            }
            /* TODO: GNU cpio pads trailer to 512 bytes, do we want that? */
            return 0;
          } else {
            free(line as *mut libc::c_void);
            break;
          }
        }
      }
    }
  }
  /* end of "while (1)" */
}
pub unsafe fn cpio_main(mut _argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut current_block: u64;
  let mut archive_handle: *mut archive_handle_t = std::ptr::null_mut();
  let mut cpio_filename: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut cpio_owner: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut cpio_fmt: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
  let mut opt: libc::c_uint = 0;
  let mut long_opts: *const libc::c_char =
        b"extract\x00\x00ilist\x00\x00tcreate\x00\x00oformat\x00\x01Hpass-through\x00\x00powner\x00\x01Rverbose\x00\x00vnull\x00\x000quiet\x00\x00\xffto-stdout\x00\x00\xfe\x00"
            as *const u8 as *const libc::c_char;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .owner_ugid
    .uid = -1i64 as uid_t;
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .owner_ugid
    .gid = -1i64 as gid_t;
  archive_handle = crate::archival::libarchive::init_handle::init_handle();
  /* archive_handle->src_fd = STDIN_FILENO; - done by init_handle */
  (*archive_handle).ah_flags = (1i32 << 3i32) as libc::c_uint;
  /* As of now we do not enforce this: */
  /* -i,-t,-o,-p are mutually exclusive */
  /* -u,-d,-m make sense only with -i or -p */
  /* -L makes sense only with -o or -p */
  opt = crate::libbb::getopt32::getopt32long(
    argv,
    b"it0uvdmLF:R:oH:p\x00" as *const u8 as *const libc::c_char,
    long_opts,
    &mut cpio_filename as *mut *mut libc::c_char,
    &mut cpio_owner as *mut *mut libc::c_char,
    &mut cpio_fmt as *mut *const libc::c_char,
  );
  argv = argv.offset(optind as isize);
  if opt & OPT_OWNER as libc::c_int as libc::c_uint != 0 {
    /* -R */
    crate::libpwdgrp::uidgid_get::parse_chown_usergroup_or_die(
      &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).owner_ugid,
      cpio_owner,
    );
    (*archive_handle).cpio__owner = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).owner_ugid
  }
  if opt & (OPT_FILE as libc::c_int | OPT_CREATE as libc::c_int) as libc::c_uint
    == OPT_FILE as libc::c_int as libc::c_uint
  {
    /* -F without -o */
    crate::libbb::xfuncs_printf::xmove_fd(crate::libbb::xfuncs_printf::xopen(cpio_filename, 0), 0);
  }
  if opt & OPT_PASSTHROUGH as libc::c_int as libc::c_uint != 0 {
    let mut pid: pid_t = 0;
    let mut pp: fd_pair = fd_pair { rd: 0, wr: 0 };
    if (*argv.offset(0)).is_null() {
      crate::libbb::appletlib::bb_show_usage();
    }
    if opt & OPT_CREATE_LEADING_DIR as libc::c_int as libc::c_uint != 0 {
      mkdir(*argv.offset(0), 0o777i32 as mode_t);
    }
    /* Crude existence check:
     * close(xopen(argv[0], O_RDONLY | O_DIRECTORY));
     * We can also xopen, fstat, IS_DIR, later fchdir.
     * This would check for existence earlier and cleaner.
     * As it stands now, if we fail xchdir later,
     * child dies on EPIPE, unless it caught
     * a diffrerent problem earlier.
     * This is good enough for now.
     */
    crate::libbb::xfuncs_printf::xpipe(&mut pp.rd);
    pid = crate::libbb::xfuncs_printf::xfork();
    if pid == 0 {
      /* child */
      close(pp.rd);
      crate::libbb::xfuncs_printf::xmove_fd(pp.wr, 1i32);
      current_block = 15458570239431914930;
    } else {
      /* parent */
      /* undo fork_or_rexec() damage */
      let fresh0 = argv;
      argv = argv.offset(1);
      crate::libbb::xfuncs_printf::xchdir(*fresh0);
      close(pp.wr);
      crate::libbb::xfuncs_printf::xmove_fd(pp.rd, 0);
      //opt &= ~OPT_PASSTHROUGH;
      opt |= OPT_EXTRACT as libc::c_int as libc::c_uint;
      current_block = 8255155894120985361;
    }
  } else if opt & OPT_CREATE as libc::c_int as libc::c_uint != 0 {
    if *cpio_fmt.offset(0) as libc::c_int != 'n' as i32 {
      /* -o */
      /* we _require_ "-H newc" */
      crate::libbb::appletlib::bb_show_usage();
    }
    if opt & OPT_FILE as libc::c_int as libc::c_uint != 0 {
      crate::libbb::xfuncs_printf::xmove_fd(
        crate::libbb::xfuncs_printf::xopen(cpio_filename, 0o1i32 | 0o100i32 | 0o1000i32),
        1i32,
      );
    }
    current_block = 15458570239431914930;
  } else {
    current_block = 8255155894120985361;
  }
  match current_block {
    15458570239431914930 => return cpio_o(),
    _ => {
      /* One of either extract or test options must be given */
      if opt & (OPT_TEST as libc::c_int | OPT_EXTRACT as libc::c_int) as libc::c_uint
        == 0 as libc::c_uint
      {
        crate::libbb::appletlib::bb_show_usage();
      }
      if opt & OPT_TEST as libc::c_int as libc::c_uint != 0 {
        /* if both extract and test options are given, ignore extract option */
        opt &= !(OPT_EXTRACT as libc::c_int) as libc::c_uint;
        (*archive_handle).action_header = Some(
          crate::archival::libarchive::header_list::header_list
            as unsafe extern "C" fn(_: *const file_header_t) -> (),
        )
      }
      if opt & OPT_EXTRACT as libc::c_int as libc::c_uint != 0 {
        (*archive_handle).action_data = Some(
          crate::archival::libarchive::data_extract_all::data_extract_all
            as unsafe extern "C" fn(_: *mut archive_handle_t) -> (),
        );
        if opt & OPT_2STDOUT as libc::c_int as libc::c_uint != 0 {
          (*archive_handle).action_data = Some(
            crate::archival::libarchive::data_extract_to_stdout::data_extract_to_stdout
              as unsafe extern "C" fn(_: *mut archive_handle_t) -> (),
          )
        }
      }
      if opt & OPT_UNCONDITIONAL as libc::c_int as libc::c_uint != 0 {
        (*archive_handle).ah_flags |= (1i32 << 2i32) as libc::c_uint;
        (*archive_handle).ah_flags &= !(1i32 << 3i32) as libc::c_uint
      }
      if opt & OPT_VERBOSE as libc::c_int as libc::c_uint != 0 {
        if (*archive_handle).action_header
          == Some(
            crate::archival::libarchive::header_list::header_list
              as unsafe extern "C" fn(_: *const file_header_t) -> (),
          )
        {
          (*archive_handle).action_header = Some(
            crate::archival::libarchive::header_verbose_list::header_verbose_list
              as unsafe extern "C" fn(_: *const file_header_t) -> (),
          )
        } else {
          (*archive_handle).action_header = Some(
            crate::archival::libarchive::header_list::header_list
              as unsafe extern "C" fn(_: *const file_header_t) -> (),
          )
        }
      }
      if opt & OPT_CREATE_LEADING_DIR as libc::c_int as libc::c_uint != 0 {
        (*archive_handle).ah_flags |= (1i32 << 1i32) as libc::c_uint
      }
      if opt & OPT_PRESERVE_MTIME as libc::c_int as libc::c_uint != 0 {
        (*archive_handle).ah_flags |= (1i32 << 0) as libc::c_uint
      }
      while !(*argv).is_null() {
        (*archive_handle).filter = Some(
          crate::archival::libarchive::filter_accept_list::filter_accept_list
            as unsafe extern "C" fn(_: *mut archive_handle_t) -> libc::c_char,
        );
        crate::libbb::llist::llist_add_to(
          &mut (*archive_handle).accept,
          *argv as *mut libc::c_void,
        );
        argv = argv.offset(1)
      }
      /* see get_header_cpio */
      (*archive_handle).cpio__blocks = -1i32 as off_t as uoff_t;
      while crate::archival::libarchive::get_header_cpio::get_header_cpio(archive_handle)
        as libc::c_int
        == 0
      {}
      crate::archival::libarchive::unsafe_symlink_target::create_links_from_list(
        (*archive_handle).link_placeholders,
      );
      if (*archive_handle).cpio__blocks != -1i32 as off_t as libc::c_ulong
        && opt & OPT_QUIET as libc::c_int as libc::c_uint == 0
      {
        fprintf(
          stderr,
          b"%lu blocks\n\x00" as *const u8 as *const libc::c_char,
          (*archive_handle).cpio__blocks,
        );
      }
      return 0;
    }
  };
}
