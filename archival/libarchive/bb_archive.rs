// Things that used to live in include/bb_archive.h.

use crate::libbb::llist::llist_t;
use crate::librb::bb_uidgid_t;
use crate::librb::size_t;
use crate::librb::smallint;
use crate::librb::uoff_t;
use libc;


use libc::off_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_header_t {
  pub name: *mut libc::c_char,
  pub link_target: *mut libc::c_char,
  pub tar__uname: *mut libc::c_char,
  pub tar__gname: *mut libc::c_char,
  pub size: libc::off_t,
  pub uid: libc::uid_t,
  pub gid: libc::gid_t,
  pub mode: libc::mode_t,
  pub mtime: libc::time_t,
  pub device: libc::dev_t,
}

// Declared in bb_archive.h but defined in get_header_cpio.c.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hardlinks_t {
  pub next: *mut hardlinks_t,
  pub inode: libc::c_int, /* TODO: must match maj/min too! */
  pub mode: libc::c_int,
  pub mtime: libc::c_int, /* These three are useful only in corner case */
  pub uid: libc::c_int,   /* of hardlinks with zero size body */
  pub gid: libc::c_int,
  pub name: [libc::c_char; 1],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_handle_t {
  /* Flags. 1st since it is most used member */
  pub ah_flags: libc::c_uint,

  /* The raw stream as read from disk or stdin */
  pub src_fd: libc::c_int,

  /* Define if the header and data component should be processed */
  pub filter: Option<unsafe extern "C" fn(_: *mut archive_handle_t) -> libc::c_char>,

  /* List of files that have been accepted */
  pub accept: *mut llist_t,
  /* List of files that have been rejected */
  pub reject: *mut llist_t,
  /* List of files that have successfully been worked on */
  pub passed: *mut llist_t,

  /* Currently processed file's header */
  pub file_header: *mut file_header_t,

  /* List of link placeholders */
  pub link_placeholders: *mut llist_t,

  /* Process the header component, e.g. tar -t */
  pub action_header: Option<unsafe extern "C" fn(_: *const file_header_t) -> ()>,

  /* Process the data component, e.g. extract to filesystem */
  pub action_data: Option<unsafe extern "C" fn(_: *mut archive_handle_t) -> ()>,

  /* Function that skips data */
  pub seek: Option<unsafe extern "C" fn(_: libc::c_int, _: off_t) -> ()>,

  /* Count processed bytes */
  pub offset: off_t,

  /* Archiver specific. Can make it a union if it ever gets big */
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

  /* Temporary storage */
  pub dpkg__buffer: *mut libc::c_char,
  /* How to process any sub archive, e.g. get_header_tar_gz */
  pub dpkg__action_data_subarchive:
    Option<unsafe extern "C" fn(_: *mut archive_handle_t) -> libc::c_char>,
  /* Contains the handle to a sub archive */
  pub dpkg__sub_archive: *mut archive_handle_t,
}

// Declared in bb_archive.h but defined in common.c.
// "TRAILER!!!"
#[no_mangle]
pub static mut cpio_TRAILER: [libc::c_char; 11] = [84, 82, 65, 73, 76, 69, 82, 33, 33, 33, 0];

#[derive(Copy, Clone)]
#[repr(C)]
pub struct transformer_state_t {
  pub signature_skipped: smallint,

  pub xformer: Option<unsafe extern "C" fn(_: *mut transformer_state_t) -> libc::c_longlong>,

  pub src_fd: libc::c_int,         /* Source */
  pub dst_fd: libc::c_int,         /* Output */
  pub mem_output_size_max: size_t, /* if non-zero, decompress to RAM instead of fd */
  pub mem_output_size: size_t,
  pub mem_output_buf: *mut libc::c_char,

  pub bytes_out: libc::off_t,
  pub bytes_in: libc::off_t, /* used in unzip code only: needs to know packed size */
  pub crc32: u32,
  pub mtime: libc::time_t, /* gunzip code may set this on exit */

  pub magic: TransformerMagic, /* if we read magic, it's saved here */
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union TransformerMagic {
  pub b: [u8; 8],
  pub b16: [u16; 4],
  pub b32: [u32; 2],
}
