use crate::libbb::llist::llist_t;

use crate::librb::__mode_t;
use crate::librb::__off_t;
use crate::librb::bb_uidgid_t;

use crate::librb::fd_pair;
use crate::librb::mode_t;
use crate::librb::off_t;
use crate::librb::pid_t;
use crate::librb::size_t;
use crate::librb::smallint;
use crate::librb::time_t;
use crate::librb::uoff_t;
use libc;
use libc::gid_t;
use libc::stat;
use libc::uid_t;
use libc::FILE;

extern "C" {
  pub type hardlinks_t;

  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);

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
  fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;

  #[no_mangle]
  fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;

  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn bb_copyfd_exact_size(fd1: libc::c_int, fd2: libc::c_int, size: off_t);

  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);

  #[no_mangle]
  fn xmalloc_readlink_or_warn(path: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn xchdir(path: *const libc::c_char);

  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn xpipe(filedes: *mut libc::c_int);

  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn bb_get_chunk_from_file(file: *mut FILE, end: *mut size_t) -> *mut libc::c_char;

  #[no_mangle]
  fn xmalloc_fgetline(file: *mut FILE) -> *mut libc::c_char;

  #[no_mangle]
  fn fflush_all() -> libc::c_int;

  #[no_mangle]
  fn parse_chown_usergroup_or_die(u: *mut bb_uidgid_t, user_group: *mut libc::c_char);

  #[no_mangle]
  fn xfork() -> pid_t;

  #[no_mangle]
  static mut option_mask32: u32;

  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> u32;

  #[no_mangle]
  fn llist_add_to(old_head: *mut *mut llist_t, data: *mut libc::c_void);

  #[no_mangle]
  fn bb_show_usage() -> !;

  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;

  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];

  #[no_mangle]
  static cpio_TRAILER: [libc::c_char; 0];

  #[no_mangle]
  fn init_handle() -> *mut archive_handle_t;

  #[no_mangle]
  fn filter_accept_list(archive_handle: *mut archive_handle_t) -> libc::c_char;

  #[no_mangle]
  fn data_extract_all(archive_handle: *mut archive_handle_t);

  #[no_mangle]
  fn data_extract_to_stdout(archive_handle: *mut archive_handle_t);

  #[no_mangle]
  fn header_list(file_header: *const file_header_t);

  #[no_mangle]
  fn header_verbose_list(file_header: *const file_header_t);

  #[no_mangle]
  fn get_header_cpio(archive_handle: *mut archive_handle_t) -> libc::c_char;

  #[no_mangle]
  fn create_links_from_list(list: *mut llist_t);
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub owner_ugid: bb_uidgid_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_header_t {
  pub name: *mut libc::c_char,
  pub link_target: *mut libc::c_char,
  pub tar__uname: *mut libc::c_char,
  pub tar__gname: *mut libc::c_char,
  pub size: off_t,
  pub uid: uid_t,
  pub gid: gid_t,
  pub mode: mode_t,
  pub mtime: time_t,
  pub device: libc::dev_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_handle_t {
  pub ah_flags: libc::c_uint,
  pub src_fd: libc::c_int,
  pub filter: Option<unsafe extern "C" fn(_: *mut archive_handle_t) -> libc::c_char>,
  pub accept: *mut llist_t,
  pub reject: *mut llist_t,
  pub passed: *mut llist_t,
  pub file_header: *mut file_header_t,
  pub link_placeholders: *mut llist_t,
  pub action_header: Option<unsafe extern "C" fn(_: *const file_header_t) -> ()>,
  pub action_data: Option<unsafe extern "C" fn(_: *mut archive_handle_t) -> ()>,
  pub seek: Option<unsafe extern "C" fn(_: libc::c_int, _: off_t) -> ()>,
  pub offset: off_t,
  pub tar__strip_components: libc::c_uint,
  pub tar__end: smallint,
  pub tar__longname: *mut libc::c_char,
  pub tar__linkname: *mut libc::c_char,
  pub tar__to_command: *mut libc::c_char,
  pub tar__to_command_shell: *const libc::c_char,
  pub cpio__blocks: uoff_t,
  pub cpio__owner: bb_uidgid_t,
  pub cpio__hardlinks_to_create: *mut hardlinks_t,
  pub cpio__created_hardlinks: *mut hardlinks_t,
  pub dpkg__buffer: *mut libc::c_char,
  pub dpkg__action_data_subarchive:
    Option<unsafe extern "C" fn(_: *mut archive_handle_t) -> libc::c_char>,
  pub dpkg__sub_archive: *mut archive_handle_t,
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct inodes_s {
  pub next: *mut inodes_s,
  pub names: *mut name_s,
  pub st: stat,
}

#[derive(Copy, Clone)]
#[repr(C)]
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
    if !(i >= 0i32) {
      break;
    }
    bb_putchar('\u{0}' as i32);
  }
  return size;
}
/* Return value will become exit code.
 * It's ok to exit instead of return. */
#[inline(never)]
unsafe extern "C" fn cpio_o() -> libc::c_int {
  let mut links: *mut inodes_s = 0 as *mut inodes_s; /* output bytes count */
  let mut bytes: off_t = 0i32 as off_t; /* line == NULL: EOF */
  let mut current_block_47: u64;
  loop {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut st: stat = std::mem::zeroed();
    line = if option_mask32 & OPT_NUL_TERMINATED as libc::c_int as libc::c_uint != 0 {
      bb_get_chunk_from_file(stdin, 0 as *mut size_t)
    } else {
      xmalloc_fgetline(stdin)
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
          != -1i64 as uid_t
        {
          st.st_uid = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .owner_ugid
            .uid
        }
        if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
          .owner_ugid
          .gid
          != -1i64 as gid_t
        {
          st.st_gid = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
            .owner_ugid
            .gid
        }
        if !(st.st_mode & 0o170000i32 as libc::c_uint == 0o120000i32 as libc::c_uint
          || st.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint)
        {
          st.st_size = 0i32 as __off_t
        }
        /* Store hardlinks for later processing, dont output them */
        if !(st.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint)
          && st.st_nlink > 1i32 as libc::c_ulong
        {
          let mut n: *mut name_s = 0 as *mut name_s;
          let mut l: *mut inodes_s = 0 as *mut inodes_s;
          /* Do we have this hardlink remembered? */
          l = links;
          loop {
            if l.is_null() {
              /* Not found: add new item to "links" list */
              l = xzalloc(::std::mem::size_of::<inodes_s>() as libc::c_ulong) as *mut inodes_s;
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
          bb_simple_perror_msg_and_die(name);
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
              st.st_size = 0i32 as __off_t
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
              0i32,
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
              let mut lpath: *mut libc::c_char = xmalloc_readlink_or_warn(name); /* S_ISREG */
              if lpath.is_null() {
                current_block_47 = 16800535488375699875;
                continue;
              }
              bytes += printf(b"%s\x00" as *const u8 as *const libc::c_char, lpath) as libc::c_long;
              free(lpath as *mut libc::c_void);
            } else {
              let mut fd: libc::c_int = xopen(name, 0i32);
              fflush_all();
              /* We must abort if file got shorter too! */
              bb_copyfd_exact_size(fd, 1i32, st.st_size);
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
            return 0i32;
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
#[no_mangle]
pub unsafe extern "C" fn cpio_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut archive_handle: *mut archive_handle_t = 0 as *mut archive_handle_t;
  let mut cpio_filename: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut cpio_owner: *mut libc::c_char = 0 as *mut libc::c_char;
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
  archive_handle = init_handle();
  /* archive_handle->src_fd = STDIN_FILENO; - done by init_handle */
  (*archive_handle).ah_flags = (1i32 << 3i32) as libc::c_uint;
  /* As of now we do not enforce this: */
  /* -i,-t,-o,-p are mutually exclusive */
  /* -u,-d,-m make sense only with -i or -p */
  /* -L makes sense only with -o or -p */
  opt = getopt32long(
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
    parse_chown_usergroup_or_die(
      &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).owner_ugid,
      cpio_owner,
    );
    (*archive_handle).cpio__owner = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).owner_ugid
  }
  if opt & (OPT_FILE as libc::c_int | OPT_CREATE as libc::c_int) as libc::c_uint
    == OPT_FILE as libc::c_int as libc::c_uint
  {
    /* -F without -o */
    xmove_fd(xopen(cpio_filename, 0i32), 0i32);
  }
  if opt & OPT_PASSTHROUGH as libc::c_int as libc::c_uint != 0 {
    let mut pid: pid_t = 0;
    let mut pp: fd_pair = fd_pair { rd: 0, wr: 0 };
    if (*argv.offset(0)).is_null() {
      bb_show_usage();
    }
    if opt & OPT_CREATE_LEADING_DIR as libc::c_int as libc::c_uint != 0 {
      mkdir(*argv.offset(0), 0o777i32 as __mode_t);
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
    xpipe(&mut pp.rd);
    pid = xfork();
    if pid == 0i32 {
      /* child */
      close(pp.rd);
      xmove_fd(pp.wr, 1i32);
      current_block = 15458570239431914930;
    } else {
      /* parent */
      /* undo fork_or_rexec() damage */
      let fresh0 = argv;
      argv = argv.offset(1);
      xchdir(*fresh0);
      close(pp.wr);
      xmove_fd(pp.rd, 0i32);
      //opt &= ~OPT_PASSTHROUGH;
      opt |= OPT_EXTRACT as libc::c_int as libc::c_uint;
      current_block = 8255155894120985361;
    }
  } else if opt & OPT_CREATE as libc::c_int as libc::c_uint != 0 {
    if *cpio_fmt.offset(0) as libc::c_int != 'n' as i32 {
      /* -o */
      /* we _require_ "-H newc" */
      bb_show_usage();
    }
    if opt & OPT_FILE as libc::c_int as libc::c_uint != 0 {
      xmove_fd(xopen(cpio_filename, 0o1i32 | 0o100i32 | 0o1000i32), 1i32);
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
        == 0i32 as libc::c_uint
      {
        bb_show_usage();
      }
      if opt & OPT_TEST as libc::c_int as libc::c_uint != 0 {
        /* if both extract and test options are given, ignore extract option */
        opt &= !(OPT_EXTRACT as libc::c_int) as libc::c_uint;
        (*archive_handle).action_header =
          Some(header_list as unsafe extern "C" fn(_: *const file_header_t) -> ())
      }
      if opt & OPT_EXTRACT as libc::c_int as libc::c_uint != 0 {
        (*archive_handle).action_data =
          Some(data_extract_all as unsafe extern "C" fn(_: *mut archive_handle_t) -> ());
        if opt & OPT_2STDOUT as libc::c_int as libc::c_uint != 0 {
          (*archive_handle).action_data =
            Some(data_extract_to_stdout as unsafe extern "C" fn(_: *mut archive_handle_t) -> ())
        }
      }
      if opt & OPT_UNCONDITIONAL as libc::c_int as libc::c_uint != 0 {
        (*archive_handle).ah_flags |= (1i32 << 2i32) as libc::c_uint;
        (*archive_handle).ah_flags &= !(1i32 << 3i32) as libc::c_uint
      }
      if opt & OPT_VERBOSE as libc::c_int as libc::c_uint != 0 {
        if (*archive_handle).action_header
          == Some(header_list as unsafe extern "C" fn(_: *const file_header_t) -> ())
        {
          (*archive_handle).action_header =
            Some(header_verbose_list as unsafe extern "C" fn(_: *const file_header_t) -> ())
        } else {
          (*archive_handle).action_header =
            Some(header_list as unsafe extern "C" fn(_: *const file_header_t) -> ())
        }
      }
      if opt & OPT_CREATE_LEADING_DIR as libc::c_int as libc::c_uint != 0 {
        (*archive_handle).ah_flags |= (1i32 << 1i32) as libc::c_uint
      }
      if opt & OPT_PRESERVE_MTIME as libc::c_int as libc::c_uint != 0 {
        (*archive_handle).ah_flags |= (1i32 << 0i32) as libc::c_uint
      }
      while !(*argv).is_null() {
        (*archive_handle).filter = Some(
          filter_accept_list as unsafe extern "C" fn(_: *mut archive_handle_t) -> libc::c_char,
        );
        llist_add_to(&mut (*archive_handle).accept, *argv as *mut libc::c_void);
        argv = argv.offset(1)
      }
      /* see get_header_cpio */
      (*archive_handle).cpio__blocks = -1i32 as off_t as uoff_t;
      while get_header_cpio(archive_handle) as libc::c_int == 0i32 {}
      create_links_from_list((*archive_handle).link_placeholders);
      if (*archive_handle).cpio__blocks != -1i32 as off_t as libc::c_ulong
        && opt & OPT_QUIET as libc::c_int as libc::c_uint == 0
      {
        fprintf(
          stderr,
          b"%lu blocks\n\x00" as *const u8 as *const libc::c_char,
          (*archive_handle).cpio__blocks,
        );
      }
      return 0i32;
    }
  };
}
