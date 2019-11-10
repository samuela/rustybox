use crate::archival::libarchive::bb_archive::file_header_t;

use crate::libbb::llist::llist_t;



use libc;




































































use libc::mode_t;


extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;



  #[no_mangle]
  fn mkdir(__path: *const libc::c_char, __mode: mode_t) -> libc::c_int;

  #[no_mangle]
  fn xchdir(path: *const libc::c_char);

  #[no_mangle]
  fn xopen_stdin(pathname: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> u32;

  #[no_mangle]
  fn llist_add_to(old_head: *mut *mut llist_t, data: *mut libc::c_void);

  #[no_mangle]
  fn bb_show_usage() -> !;

  #[no_mangle]
  fn init_handle() -> *mut archive_handle_t;

  #[no_mangle]
  fn filter_accept_list(archive_handle: *mut archive_handle_t) -> libc::c_char;

  #[no_mangle]
  fn filter_accept_list_reassign(archive_handle: *mut archive_handle_t) -> libc::c_char;

  #[no_mangle]
  fn unpack_ar_archive(ar_archive: *mut archive_handle_t);

  #[no_mangle]
  fn data_extract_all(archive_handle: *mut archive_handle_t);

  #[no_mangle]
  fn data_extract_to_stdout(archive_handle: *mut archive_handle_t);

  #[no_mangle]
  fn header_list(file_header: *const file_header_t);

  #[no_mangle]
  fn header_verbose_list(file_header: *const file_header_t);
}

use crate::archival::libarchive::bb_archive::archive_handle_t;

#[no_mangle]
pub unsafe extern "C" fn dpkg_deb_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut ar_archive: *mut archive_handle_t = 0 as *mut archive_handle_t;
  let mut tar_archive: *mut archive_handle_t = 0 as *mut archive_handle_t;
  let mut control_tar_llist: *mut llist_t = 0 as *mut llist_t;
  let mut opt: libc::c_uint = 0;
  let mut extract_dir: *const libc::c_char = 0 as *const libc::c_char;
  /* Setup the tar archive handle */
  tar_archive = init_handle();
  /* Setup an ar archive handle that refers to the gzip sub archive */
  ar_archive = init_handle();
  (*ar_archive).dpkg__sub_archive = tar_archive;
  (*ar_archive).filter = Some(
    filter_accept_list_reassign as unsafe extern "C" fn(_: *mut archive_handle_t) -> libc::c_char,
  );
  llist_add_to(
    &mut (*ar_archive).accept,
    b"data.tar\x00" as *const u8 as *const libc::c_char as *mut libc::c_char as *mut libc::c_void,
  );
  llist_add_to(
    &mut control_tar_llist,
    b"control.tar\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      as *mut libc::c_void,
  );
  llist_add_to(
    &mut (*ar_archive).accept,
    b"data.tar.gz\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      as *mut libc::c_void,
  );
  llist_add_to(
    &mut control_tar_llist,
    b"control.tar.gz\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      as *mut libc::c_void,
  );
  llist_add_to(
    &mut (*ar_archive).accept,
    b"data.tar.bz2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      as *mut libc::c_void,
  );
  llist_add_to(
    &mut control_tar_llist,
    b"control.tar.bz2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      as *mut libc::c_void,
  );
  llist_add_to(
    &mut (*ar_archive).accept,
    b"data.tar.lzma\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      as *mut libc::c_void,
  );
  llist_add_to(
    &mut control_tar_llist,
    b"control.tar.lzma\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      as *mut libc::c_void,
  );
  llist_add_to(
    &mut (*ar_archive).accept,
    b"data.tar.xz\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      as *mut libc::c_void,
  );
  llist_add_to(
    &mut control_tar_llist,
    b"control.tar.xz\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      as *mut libc::c_void,
  );
  /* Must have 1 or 2 args */
  opt = getopt32(
    argv,
    b"^cefXx\x00-1:?2:c--efXx:e--cfXx:f--ceXx:X--cefx:x--cefX\x00" as *const u8
      as *const libc::c_char,
  );
  argv = argv.offset(optind as isize);
  //argc -= optind;
  extract_dir = *argv.offset(1);
  if opt & 1i32 as libc::c_uint != 0 {
    // -c
    (*tar_archive).action_header =
      Some(header_verbose_list as unsafe extern "C" fn(_: *const file_header_t) -> ());
    if !extract_dir.is_null() {
      bb_show_usage();
    }
  }
  if opt & 4i32 as libc::c_uint != 0 {
    // -f
    /* Print the entire control file */
    //TODO: standard tool accepts an optional list of fields to print
    (*ar_archive).accept = control_tar_llist;
    llist_add_to(
      &mut (*tar_archive).accept,
      b"./control\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        as *mut libc::c_void,
    );
    (*tar_archive).filter =
      Some(filter_accept_list as unsafe extern "C" fn(_: *mut archive_handle_t) -> libc::c_char);
    (*tar_archive).action_data =
      Some(data_extract_to_stdout as unsafe extern "C" fn(_: *mut archive_handle_t) -> ());
    if !extract_dir.is_null() {
      bb_show_usage();
    }
  }
  if opt & 2i32 as libc::c_uint != 0 {
    // -e
    (*ar_archive).accept = control_tar_llist;
    (*tar_archive).action_data =
      Some(data_extract_all as unsafe extern "C" fn(_: *mut archive_handle_t) -> ());
    if extract_dir.is_null() {
      extract_dir = b"./DEBIAN\x00" as *const u8 as *const libc::c_char
    }
  }
  if opt & (8i32 | 16i32) as libc::c_uint != 0 {
    // -Xx
    if opt & 8i32 as libc::c_uint != 0 {
      (*tar_archive).action_header =
        Some(header_list as unsafe extern "C" fn(_: *const file_header_t) -> ())
    }
    (*tar_archive).action_data =
      Some(data_extract_all as unsafe extern "C" fn(_: *mut archive_handle_t) -> ());
    if extract_dir.is_null() {
      bb_show_usage();
    }
  }
  /* Standard tool supports "-" */
  (*ar_archive).src_fd = xopen_stdin(*argv.offset(0)); /* bb_make_directory(extract_dir, 0777, 0) */
  (*tar_archive).src_fd = (*ar_archive).src_fd;
  if !extract_dir.is_null() {
    mkdir(extract_dir, 0o777i32 as mode_t);
    xchdir(extract_dir);
  }
  /* Do it */
  unpack_ar_archive(ar_archive);
  /* Cleanup */
  return 0i32;
}
