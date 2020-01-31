use crate::archival::libarchive::bb_archive::archive_handle_t;
use crate::archival::libarchive::bb_archive::file_header_t;
use crate::libbb::llist::llist_t;
use libc;
use libc::mode_t;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn mkdir(__path: *const libc::c_char, __mode: mode_t) -> libc::c_int;

}

pub unsafe fn dpkg_deb_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut ar_archive: *mut archive_handle_t = std::ptr::null_mut();
  let mut tar_archive: *mut archive_handle_t = std::ptr::null_mut();
  let mut control_tar_llist: *mut llist_t = std::ptr::null_mut();
  let mut opt: libc::c_uint = 0;
  let mut extract_dir: *const libc::c_char = std::ptr::null();
  /* Setup the tar archive handle */
  tar_archive = crate::archival::libarchive::init_handle::init_handle();
  /* Setup an ar archive handle that refers to the gzip sub archive */
  ar_archive = crate::archival::libarchive::init_handle::init_handle();
  (*ar_archive).dpkg__sub_archive = tar_archive;
  (*ar_archive).filter = Some(
    crate::archival::libarchive::filter_accept_list_reassign::filter_accept_list_reassign
      as unsafe fn(_: *mut archive_handle_t) -> libc::c_char,
  );
  crate::libbb::llist::llist_add_to(
    &mut (*ar_archive).accept,
    b"data.tar\x00" as *const u8 as *const libc::c_char as *mut libc::c_char as *mut libc::c_void,
  );
  crate::libbb::llist::llist_add_to(
    &mut control_tar_llist,
    b"control.tar\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      as *mut libc::c_void,
  );
  crate::libbb::llist::llist_add_to(
    &mut (*ar_archive).accept,
    b"data.tar.gz\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      as *mut libc::c_void,
  );
  crate::libbb::llist::llist_add_to(
    &mut control_tar_llist,
    b"control.tar.gz\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      as *mut libc::c_void,
  );
  crate::libbb::llist::llist_add_to(
    &mut (*ar_archive).accept,
    b"data.tar.bz2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      as *mut libc::c_void,
  );
  crate::libbb::llist::llist_add_to(
    &mut control_tar_llist,
    b"control.tar.bz2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      as *mut libc::c_void,
  );
  crate::libbb::llist::llist_add_to(
    &mut (*ar_archive).accept,
    b"data.tar.lzma\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      as *mut libc::c_void,
  );
  crate::libbb::llist::llist_add_to(
    &mut control_tar_llist,
    b"control.tar.lzma\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      as *mut libc::c_void,
  );
  crate::libbb::llist::llist_add_to(
    &mut (*ar_archive).accept,
    b"data.tar.xz\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      as *mut libc::c_void,
  );
  crate::libbb::llist::llist_add_to(
    &mut control_tar_llist,
    b"control.tar.xz\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
      as *mut libc::c_void,
  );
  /* Must have 1 or 2 args */
  opt = crate::libbb::getopt32::getopt32(
    argv,
    b"^cefXx\x00-1:?2:c--efXx:e--cfXx:f--ceXx:X--cefx:x--cefX\x00" as *const u8
      as *const libc::c_char,
  );
  argv = argv.offset(optind as isize);
  //argc -= optind;
  extract_dir = *argv.offset(1);
  if opt & 1i32 as libc::c_uint != 0 {
    // -c
    (*tar_archive).action_header = Some(
      crate::archival::libarchive::header_verbose_list::header_verbose_list
        as unsafe fn(_: *const file_header_t) -> (),
    );
    if !extract_dir.is_null() {
      crate::libbb::appletlib::bb_show_usage();
    }
  }
  if opt & 4i32 as libc::c_uint != 0 {
    // -f
    /* Print the entire control file */
    //TODO: standard tool accepts an optional list of fields to print
    (*ar_archive).accept = control_tar_llist;
    crate::libbb::llist::llist_add_to(
      &mut (*tar_archive).accept,
      b"./control\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        as *mut libc::c_void,
    );
    (*tar_archive).filter = Some(
      crate::archival::libarchive::filter_accept_list::filter_accept_list
        as unsafe fn(_: *mut archive_handle_t) -> libc::c_char,
    );
    (*tar_archive).action_data = Some(
      crate::archival::libarchive::data_extract_to_stdout::data_extract_to_stdout
        as unsafe fn(_: *mut archive_handle_t) -> (),
    );
    if !extract_dir.is_null() {
      crate::libbb::appletlib::bb_show_usage();
    }
  }
  if opt & 2i32 as libc::c_uint != 0 {
    // -e
    (*ar_archive).accept = control_tar_llist;
    (*tar_archive).action_data = Some(
      crate::archival::libarchive::data_extract_all::data_extract_all
        as unsafe fn(_: *mut archive_handle_t) -> (),
    );
    if extract_dir.is_null() {
      extract_dir = b"./DEBIAN\x00" as *const u8 as *const libc::c_char
    }
  }
  if opt & (8i32 | 16i32) as libc::c_uint != 0 {
    // -Xx
    if opt & 8i32 as libc::c_uint != 0 {
      (*tar_archive).action_header = Some(
        crate::archival::libarchive::header_list::header_list
          as unsafe fn(_: *const file_header_t) -> (),
      )
    }
    (*tar_archive).action_data = Some(
      crate::archival::libarchive::data_extract_all::data_extract_all
        as unsafe fn(_: *mut archive_handle_t) -> (),
    );
    if extract_dir.is_null() {
      crate::libbb::appletlib::bb_show_usage();
    }
  }
  /* Standard tool supports "-" */
  (*ar_archive).src_fd = crate::libbb::wfopen_input::xopen_stdin(*argv.offset(0)); /* bb_make_directory(extract_dir, 0777, 0) */
  (*tar_archive).src_fd = (*ar_archive).src_fd;
  if !extract_dir.is_null() {
    mkdir(extract_dir, 0o777i32 as mode_t);
    crate::libbb::xfuncs_printf::xchdir(extract_dir);
  }
  /* Do it */
  crate::archival::libarchive::unpack_ar_archive::unpack_ar_archive(ar_archive);
  /* Cleanup */
  return 0;
}
