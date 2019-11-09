use crate::archival::libarchive::bb_archive::hardlinks_t;
use crate::archival::libarchive::bb_archive::file_header_t;
use crate::libbb::llist::llist_t;
use crate::librb::bb_uidgid_t;
use crate::librb::size_t;
use crate::librb::smallint;
use crate::librb::uoff_t;
use libc;
use libc::free;
use libc::gid_t;
use libc::mode_t;
use libc::off_t;
use libc::ssize_t;
use libc::time_t;
use libc::uid_t;

extern "C" {

  #[no_mangle]
  fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn overlapping_strcpy(dst: *mut libc::c_char, src: *const libc::c_char);
  #[no_mangle]
  fn full_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn xread(fd: libc::c_int, buf: *mut libc::c_void, count: size_t);
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_makedev(major: libc::c_uint, minor: libc::c_uint) -> libc::c_ulonglong;
  #[no_mangle]
  static cpio_TRAILER: [libc::c_char; 0];
  #[no_mangle]
  fn data_skip(archive_handle: *mut archive_handle_t);
  #[no_mangle]
  fn data_align(archive_handle: *mut archive_handle_t, boundary: libc::c_uint);
}

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
#[no_mangle]
pub unsafe extern "C" fn get_header_cpio(
  mut archive_handle: *mut archive_handle_t,
) -> libc::c_char {
  let mut file_header: *mut file_header_t = (*archive_handle).file_header;
  let mut cpio_header: [libc::c_char; 110] = [0; 110];
  let mut namesize: libc::c_int = 0;
  let mut major: libc::c_int = 0;
  let mut minor: libc::c_int = 0;
  let mut nlink: libc::c_int = 0;
  let mut mode: libc::c_int = 0;
  let mut inode: libc::c_int = 0;
  let mut size: libc::c_uint = 0;
  let mut uid: libc::c_uint = 0;
  let mut gid: libc::c_uint = 0;
  let mut mtime: libc::c_uint = 0;
  /* There can be padding before archive header */
  data_align(archive_handle, 4i32 as libc::c_uint);
  size = full_read(
    (*archive_handle).src_fd,
    cpio_header.as_mut_ptr() as *mut libc::c_void,
    110i32 as size_t,
  ) as libc::c_uint;
  if !(size == 0i32 as libc::c_uint) {
    if size != 110i32 as libc::c_uint {
      bb_simple_error_msg_and_die(b"short read\x00" as *const u8 as *const libc::c_char);
    }
    (*archive_handle).offset += 110i32 as libc::c_long;
    if is_prefixed_with(
      &mut *cpio_header.as_mut_ptr().offset(0),
      b"07070\x00" as *const u8 as *const libc::c_char,
    )
    .is_null()
      || cpio_header[5] as libc::c_int != '1' as i32 && cpio_header[5] as libc::c_int != '2' as i32
    {
      bb_simple_error_msg_and_die(
        b"unsupported cpio format, use newc or crc\x00" as *const u8 as *const libc::c_char,
      );
    }
    if sscanf(
      cpio_header.as_mut_ptr().offset(6),
      b"%8x%8x%8x%8x%8x%8x%8x%*16c%8x%8x%8x\x00" as *const u8 as *const libc::c_char,
      &mut inode as *mut libc::c_int,
      &mut mode as *mut libc::c_int,
      &mut uid as *mut libc::c_uint,
      &mut gid as *mut libc::c_uint,
      &mut nlink as *mut libc::c_int,
      &mut mtime as *mut libc::c_uint,
      &mut size as *mut libc::c_uint,
      &mut major as *mut libc::c_int,
      &mut minor as *mut libc::c_int,
      &mut namesize as *mut libc::c_int,
    ) != 10i32
    {
      bb_simple_error_msg_and_die(b"damaged cpio file\x00" as *const u8 as *const libc::c_char);
    }
    (*file_header).mode = mode as mode_t;
    /* "cpio -R USER:GRP" support: */
    if (*archive_handle).cpio__owner.uid != -1i64 as uid_t {
      uid = (*archive_handle).cpio__owner.uid
    } /* paranoia: limit names to 8k chars */
    if (*archive_handle).cpio__owner.gid != -1i64 as gid_t {
      gid = (*archive_handle).cpio__owner.gid
    }
    (*file_header).uid = uid;
    (*file_header).gid = gid;
    (*file_header).mtime = mtime as time_t;
    (*file_header).size = size as off_t;
    namesize &= 0x1fffi32;
    (*file_header).name = xzalloc((namesize + 1i32) as size_t) as *mut libc::c_char;
    /* Read in filename */
    xread(
      (*archive_handle).src_fd,
      (*file_header).name as *mut libc::c_void,
      namesize as size_t,
    );
    if *(*file_header).name.offset(0) as libc::c_int == '/' as i32 {
      /* Testcase: echo /etc/hosts | cpio -pvd /tmp
       * Without this code, it tries to unpack /etc/hosts
       * into "/etc/hosts", not "etc/hosts".
       */
      let mut p: *mut libc::c_char = (*file_header).name;
      loop {
        p = p.offset(1);
        if !(*p as libc::c_int == '/' as i32) {
          break;
        }
      }
      overlapping_strcpy((*file_header).name, p);
    }
    (*archive_handle).offset += namesize as libc::c_long;
    /* Update offset amount and skip padding before file contents */
    data_align(archive_handle, 4i32 as libc::c_uint);
    if strcmp((*file_header).name, cpio_TRAILER.as_ptr()) == 0i32 {
      /* Always round up. ">> 9" divides by 512 */
      (*archive_handle).cpio__blocks =
        ((*archive_handle).offset + 511i32 as libc::c_long) as uoff_t >> 9i32
    } else {
      (*file_header).link_target = 0 as *mut libc::c_char; /* paranoia: limit names to 8k chars */
      if (*file_header).mode & 0o170000i32 as libc::c_uint == 0o120000i32 as libc::c_uint {
        (*file_header).size &= 0x1fffi32 as libc::c_long;
        (*file_header).link_target =
          xzalloc(((*file_header).size + 1) as size_t) as *mut libc::c_char;
        xread(
          (*archive_handle).src_fd,
          (*file_header).link_target as *mut libc::c_void,
          (*file_header).size as size_t,
        );
        (*archive_handle).offset += (*file_header).size;
        (*file_header).size = 0i32 as off_t
        /* Stop possible seeks in future */
      }
      // TODO: data_extract_all can't deal with hardlinks to non-files...
      // when fixed, change S_ISREG to !S_ISDIR here
      if nlink > 1i32
        && (*file_header).mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint
      {
        let mut new: *mut hardlinks_t = xmalloc(
          (::std::mem::size_of::<hardlinks_t>() as libc::c_ulong)
            .wrapping_add(namesize as libc::c_ulong),
        ) as *mut hardlinks_t;
        (*new).inode = inode;
        (*new).mode = mode;
        (*new).mtime = mtime as libc::c_int;
        (*new).uid = uid as libc::c_int;
        (*new).gid = gid as libc::c_int;
        strcpy((*new).name.as_mut_ptr(), (*file_header).name);
        /* Put file on a linked list for later */
        if size == 0i32 as libc::c_uint {
          (*new).next = (*archive_handle).cpio__hardlinks_to_create;
          (*archive_handle).cpio__hardlinks_to_create = new;
          return 0i32 as libc::c_char;
          /* Skip this one */
          /* TODO: this breaks cpio -t (it does not show hardlinks) */
        }
        (*new).next = (*archive_handle).cpio__created_hardlinks;
        (*archive_handle).cpio__created_hardlinks = new
      }
      (*file_header).device =
        bb_makedev(major as libc::c_uint, minor as libc::c_uint) as libc::dev_t;
      if (*archive_handle).filter.expect("non-null function pointer")(archive_handle) as libc::c_int
        == 0i32
      {
        (*archive_handle)
          .action_data
          .expect("non-null function pointer")(archive_handle);
        //TODO: run "echo /etc/hosts | cpio -pv /tmp" twice. On 2nd run:
        //cpio: etc/hosts not created: newer or same age file exists
        //etc/hosts  <-- should NOT show it
        //2 blocks <-- should say "0 blocks"
        (*archive_handle)
          .action_header
          .expect("non-null function pointer")(file_header);
      } else {
        data_skip(archive_handle);
      }
      (*archive_handle).offset += (*file_header).size;
      free((*file_header).link_target as *mut libc::c_void);
      free((*file_header).name as *mut libc::c_void);
      (*file_header).link_target = 0 as *mut libc::c_char;
      (*file_header).name = 0 as *mut libc::c_char;
      return 0i32 as libc::c_char;
    }
  }
  free((*file_header).link_target as *mut libc::c_void);
  free((*file_header).name as *mut libc::c_void);
  while !(*archive_handle).cpio__hardlinks_to_create.is_null() {
    let mut current_block_86: u64;
    let mut cur: *mut hardlinks_t = 0 as *mut hardlinks_t;
    let mut make_me: *mut hardlinks_t = (*archive_handle).cpio__hardlinks_to_create;
    (*archive_handle).cpio__hardlinks_to_create = (*make_me).next;
    memset(
      file_header as *mut libc::c_void,
      0i32,
      ::std::mem::size_of::<file_header_t>() as libc::c_ulong,
    );
    (*file_header).mtime = (*make_me).mtime as time_t;
    (*file_header).name = (*make_me).name.as_mut_ptr();
    (*file_header).mode = (*make_me).mode as mode_t;
    (*file_header).uid = (*make_me).uid as uid_t;
    (*file_header).gid = (*make_me).gid as gid_t;
    /*file_header->size = 0;*/
    /*file_header->link_target = NULL;*/
    /* Try to find a file we are hardlinked to */
    cur = (*archive_handle).cpio__created_hardlinks;
    loop {
      if cur.is_null() {
        current_block_86 = 9705665520141849625;
        break;
      }
      /* TODO: must match maj/min too! */
      if (*cur).inode == (*make_me).inode {
        (*file_header).link_target = (*cur).name.as_mut_ptr();
        /* link_target != NULL, size = 0: "I am a hardlink" */
        if (*archive_handle).filter.expect("non-null function pointer")(archive_handle)
          as libc::c_int
          == 0i32
        {
          (*archive_handle)
            .action_data
            .expect("non-null function pointer")(archive_handle);
        }
        free(make_me as *mut libc::c_void);
        current_block_86 = 17688141731389699982;
        break;
      } else {
        cur = (*cur).next
      }
    }
    match current_block_86 {
      9705665520141849625 => {
        /* Oops... no file with such inode was created... do it now
         * (happens when hardlinked files are empty (zero length)) */
        if (*archive_handle).filter.expect("non-null function pointer")(archive_handle)
          as libc::c_int
          == 0i32
        {
          (*archive_handle)
            .action_data
            .expect("non-null function pointer")(archive_handle);
        }
        /* Move to the list of created hardlinked files */
        (*make_me).next = (*archive_handle).cpio__created_hardlinks;
        (*archive_handle).cpio__created_hardlinks = make_me
      }
      _ => {}
    }
  }
  while !(*archive_handle).cpio__created_hardlinks.is_null() {
    let mut p_0: *mut hardlinks_t = (*archive_handle).cpio__created_hardlinks;
    (*archive_handle).cpio__created_hardlinks = (*p_0).next;
    free(p_0 as *mut libc::c_void);
  }
  return 1i32 as libc::c_char;
  /* "No more files to process" */
}
