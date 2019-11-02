use libc;
use std::ffi::CStr;
use std::ffi::CString;

use crate::applets::applet_tables::applets;

extern "C" {
  #[no_mangle]
  fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn getuid() -> __uid_t;

  #[no_mangle]
  fn geteuid() -> __uid_t;

  #[no_mangle]
  fn getgid() -> __gid_t;

  #[no_mangle]
  fn setuid(__uid: __uid_t) -> libc::c_int;

  #[no_mangle]
  fn setgid(__gid: __gid_t) -> libc::c_int;

  #[no_mangle]
  fn setresuid(__ruid: __uid_t, __euid: __uid_t, __suid: __uid_t) -> libc::c_int;

  #[no_mangle]
  fn setresgid(__rgid: __gid_t, __egid: __gid_t, __sgid: __gid_t) -> libc::c_int;

  #[no_mangle]
  fn link(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn symlink(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn __errno_location() -> *mut libc::c_int;

  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn fgets_unlocked(
    __s: *mut libc::c_char,
    __n: libc::c_int,
    __stream: *mut FILE,
  ) -> *mut libc::c_char;

  #[no_mangle]
  fn feof_unlocked(__stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);

  #[no_mangle]
  fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;

  #[no_mangle]
  fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;

  /* Search for an entry with a matching username.  */
  #[no_mangle]
  fn bb_internal_getpwnam(__name: *const libc::c_char) -> *mut passwd;

  /* Search for an entry with a matching group ID.  */
  #[no_mangle]
  fn bb_internal_getgrgid(__gid: gid_t) -> *mut group;

  #[no_mangle]
  static bb_errno: *mut libc::c_int;

  #[no_mangle]
  fn skip_whitespace(_: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn bb_get_last_path_component_nostrip(path: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn bb_basename(name: *const libc::c_char) -> *const libc::c_char;

  #[no_mangle]
  fn xmalloc_readlink(path: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn full_write1_str(str: *const libc::c_char) -> ssize_t;

  #[no_mangle]
  fn full_write2_str(str: *const libc::c_char) -> ssize_t;

  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;

  #[no_mangle]
  fn get_uidgid(_: *mut bb_uidgid_t, _: *const libc::c_char) -> libc::c_int;

  #[no_mangle]
  static bb_busybox_exec_path: [libc::c_char; 0];

  #[no_mangle]
  fn xfunc_die() -> !;
  /* DO NOT EDIT. This file is generated from applets.src.h */
  /* vi: set sw=4 ts=4: */
  /*
   * applets.h - a listing of all busybox applets.
   *
   * If you write a new applet, you need to add an entry to this list to make
   * busybox aware of it.
   */
  /*
  name  - applet name as it is typed on command line
  help  - applet name, converted to C (ether-wake: help = ether_wake)
  main  - corresponding <applet>_main to call (bzcat: main = bunzip2)
  l     - location to install link to: [/usr]/[s]bin
  s     - suid type:
          BB_SUID_REQUIRE: will complain if busybox isn't suid
          and is run by non-root (applet_main() will not be called at all)
          BB_SUID_DROP: will drop suid prior to applet_main()
          BB_SUID_MAYBE: neither of the above
          (every instance of BB_SUID_REQUIRE and BB_SUID_MAYBE
          needs to be justified in comment)
          NB: please update FEATURE_SUID help text whenever you add/remove
          BB_SUID_REQUIRE or BB_SUID_MAYBE applet.
  */

  #[no_mangle]
  fn gunzip_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  static bb_msg_requires_arg: [libc::c_char; 0];

  #[no_mangle]
  fn test_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ash_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn get_terminal_width(fd: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn concat_path_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;

  #[no_mangle]
  fn printf_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);

  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;

  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);

  #[no_mangle]
  fn ls_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn set_task_comm(comm: *const libc::c_char);

  #[no_mangle]
  static bb_banner: [libc::c_char; 0];

  #[no_mangle]
  fn kill_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;

  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;

  #[no_mangle]
  fn llist_free(
    elm: *mut llist_t,
    freeit: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
  );

  #[no_mangle]
  static mut xfunc_error_retval: uint8_t;

  #[no_mangle]
  fn bunzip2_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn chown_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn echo_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn hush_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn mallopt(__param: libc::c_int, __val: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn unlzma_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn unxz_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn bzip2_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn cpio_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn dpkg_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn dpkg_deb_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn gzip_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn lzop_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn rpm_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn rpm2cpio_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn tar_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn unzip_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn chvt_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn clear_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn deallocvt_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn dumpkmap_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn fgconsole_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn kbd_mode_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn loadfont_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn setfont_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn loadkmap_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn openvt_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn reset_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn resize_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn setconsole_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn setkeycodes_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn setlogcons_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn showkey_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn basename_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn cat_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn chgrp_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn chmod_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn chroot_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn cksum_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn comm_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn cp_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn cut_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn date_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn dd_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn df_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn dirname_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn dos2unix_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn du_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn env_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn expand_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn expr_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn factor_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn false_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn fold_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn head_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn hostid_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn id_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn install_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn link_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ln_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn logname_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn md5_sha1_sum_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn mkdir_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn mkfifo_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn mknod_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn mktemp_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn mv_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn nice_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn nl_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn nohup_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn nproc_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn od_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn paste_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn printenv_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn pwd_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn readlink_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn realpath_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn rm_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn rmdir_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn seq_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn shred_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn shuf_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn sleep_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn sort_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn split_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn stat_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn stty_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn sum_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn sync_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn fsync_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn tac_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn tail_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn tee_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn timeout_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn touch_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn tr_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn true_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn truncate_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn tty_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn uname_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn uniq_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn unlink_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn usleep_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn uudecode_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn base64_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn uuencode_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn wc_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn who_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn whoami_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn yes_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn pipe_progress_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn run_parts_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn start_stop_daemon_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn which_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn chattr_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn fsck_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn lsattr_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn awk_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn cmp_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn diff_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ed_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn patch_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn sed_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn vi_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn find_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn grep_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn xargs_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn bootchartd_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn halt_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn init_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn nuke_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn resume_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn add_remove_shell_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn addgroup_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn adduser_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn chpasswd_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn cryptpw_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn deluser_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn getty_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  /* Needs to be run by root or be suid root - needs to change uid and gid: */
  #[no_mangle]
  fn login_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  /* Needs to be run by root or be suid root - needs to change /etc/{passwd,shadow}: */
  #[no_mangle]
  fn passwd_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  /* Needs to be run by root or be suid root - needs to change uid and gid: */
  #[no_mangle]
  fn su_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn sulogin_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  /* Needs to be run by root or be suid root - needs to change uid and gid: */
  #[no_mangle]
  fn vlock_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn makemime_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn popmaildir_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn reformime_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn sendmail_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn adjtimex_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn bc_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn dc_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn beep_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn chat_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn conspy_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn crond_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn crontab_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn devmem_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn fbsplash_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn hdparm_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn hexedit_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn i2cget_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn i2cset_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn i2cdump_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn i2cdetect_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn i2ctransfer_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn less_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn lsscsi_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn makedevs_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn man_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn microcom_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn mt_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn nandwrite_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn partprobe_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn raidautorun_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn readahead_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn runlevel_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn rx_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn setfattr_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn setserial_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn strings_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn time_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ts_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ttysize_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ubi_tools_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ubirename_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn volname_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn watchdog_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn modinfo_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn lsmod_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn modprobe_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn arp_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn arping_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn brctl_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn dnsd_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ether_wake_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ftpd_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ftpgetput_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn hostname_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn httpd_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ifconfig_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ifenslave_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ifplugd_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ifupdown_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn inetd_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ip_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ipaddr_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn iplink_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn iproute_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn iprule_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn iptunnel_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ipneigh_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ipcalc_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn fakeidentd_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn nameif_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn nbdclient_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn nc_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn netstat_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn nslookup_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ntpd_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ping_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ping6_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn pscan_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn route_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn slattach_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ssl_client_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn tc_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn tcpudpsvd_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn telnet_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn telnetd_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn tftp_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn tftpd_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn traceroute_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn traceroute6_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn tunctl_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn vconfig_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn wget_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn whois_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn zcip_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn lpd_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn lpqr_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn free_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn fuser_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn iostat_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn lsof_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn mpstat_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn nmeter_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn pgrep_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn pidof_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn pmap_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn powertop_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ps_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn pstree_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn pwdx_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn smemcap_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn sysctl_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn top_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn uptime_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn watch_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn chpst_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn runsv_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn runsvdir_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn sv_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn svc_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn svok_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn svlogd_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn cttyhack_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn klogd_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn logger_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn logread_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn syslogd_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn acpid_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn blkdiscard_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn blkid_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn blockdev_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn cal_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn chrt_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn dmesg_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn eject_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn fallocate_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn fatattr_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn fbset_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn fdformat_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn fdisk_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn findfs_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn flock_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn freeramdisk_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn fsck_minix_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn fsfreeze_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn fstrim_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn getopt_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn hexdump_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn xxd_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn hwclock_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ionice_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ipcrm_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ipcs_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn last_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn losetup_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn lspci_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn lsusb_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn mdev_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn mesg_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn mkfs_ext2_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn mkfs_minix_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn mkfs_vfat_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn mkswap_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn more_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn mount_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn mountpoint_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn nsenter_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn pivot_root_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn rdate_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn rdev_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn readprofile_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn renice_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn rev_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn rtcwake_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn script_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn scriptreplay_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn setarch_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn setpriv_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn setsid_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn swap_on_off_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn switch_root_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn taskset_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn uevent_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn umount_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn unshare_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn wall_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn udhcpc6_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn udhcpc_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn udhcpd_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn dhcprelay_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn dumpleases_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  /* A bit of bunzip2 internals are exposed for compressed help support: */
  #[no_mangle]
  fn unpack_bz2_data(
    packed: *const libc::c_char,
    packed_len: libc::c_int,
    unpacked_len: libc::c_int,
  ) -> *mut libc::c_char;
}

pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;

/* NB: unaligned parameter should be a pointer, aligned one -
 * a lvalue. This makes it more likely to not swap them by mistake
 */
/* #elif ... - add your favorite arch today! */
/* Unaligned, fixed-endian accessors */
/* unxz needs an aligned fixed-endian accessor.
 * (however, the compiler does not realize it's aligned, the cast is still necessary)
 */
/* ---- Size-saving "small" ints (arch-dependent) ----------- */
/* add other arches which benefit from this... */
pub type smallint = libc::c_schar;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type mode_t = __mode_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
  pub tv_sec: __time_t,
  pub tv_nsec: __syscall_slong_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
  pub st_dev: __dev_t,
  pub st_ino: __ino_t,
  pub st_nlink: __nlink_t,
  pub st_mode: __mode_t,
  pub st_uid: __uid_t,
  pub st_gid: __gid_t,
  pub __pad0: libc::c_int,
  pub st_rdev: __dev_t,
  pub st_size: __off_t,
  pub st_blksize: __blksize_t,
  pub st_blocks: __blkcnt_t,
  pub st_atim: timespec,
  pub st_mtim: timespec,
  pub st_ctim: timespec,
  pub __glibc_reserved: [__syscall_slong_t; 3],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
  pub _flags: libc::c_int,
  pub _IO_read_ptr: *mut libc::c_char,
  pub _IO_read_end: *mut libc::c_char,
  pub _IO_read_base: *mut libc::c_char,
  pub _IO_write_base: *mut libc::c_char,
  pub _IO_write_ptr: *mut libc::c_char,
  pub _IO_write_end: *mut libc::c_char,
  pub _IO_buf_base: *mut libc::c_char,
  pub _IO_buf_end: *mut libc::c_char,
  pub _IO_save_base: *mut libc::c_char,
  pub _IO_backup_base: *mut libc::c_char,
  pub _IO_save_end: *mut libc::c_char,
  pub _markers: *mut _IO_marker,
  pub _chain: *mut _IO_FILE,
  pub _fileno: libc::c_int,
  pub _flags2: libc::c_int,
  pub _old_offset: __off_t,
  pub _cur_column: libc::c_ushort,
  pub _vtable_offset: libc::c_schar,
  pub _shortbuf: [libc::c_char; 1],
  pub _lock: *mut libc::c_void,
  pub _offset: __off64_t,
  pub __pad1: *mut libc::c_void,
  pub __pad2: *mut libc::c_void,
  pub __pad3: *mut libc::c_void,
  pub __pad4: *mut libc::c_void,
  pub __pad5: size_t,
  pub _mode: libc::c_int,
  pub _unused2: [libc::c_char; 20],
}

pub type _IO_lock_t = ();

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
  pub _next: *mut _IO_marker,
  pub _sbuf: *mut _IO_FILE,
  pub _pos: libc::c_int,
}

pub type FILE = _IO_FILE;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
  pub pw_name: *mut libc::c_char,
  pub pw_passwd: *mut libc::c_char,
  pub pw_uid: __uid_t,
  pub pw_gid: __gid_t,
  pub pw_gecos: *mut libc::c_char,
  pub pw_dir: *mut libc::c_char,
  pub pw_shell: *mut libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
  pub gr_name: *mut libc::c_char,
  pub gr_passwd: *mut libc::c_char,
  pub gr_gid: __gid_t,
  pub gr_mem: *mut *mut libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct bb_uidgid_t {
  pub uid: uid_t,
  pub gid: gid_t,
}

/* real uid */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct suid_config_t {
  pub m_next: *mut suid_config_t,
  pub m_ugid: bb_uidgid_t,
  pub m_applet: libc::c_int,
  pub m_mode: mode_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct llist_t {
  pub link: *mut llist_t,
  pub data: *mut libc::c_char,
}

pub const BB_SUID_DROP: bb_suid_t = 0;
pub const BB_SUID_REQUIRE: bb_suid_t = 2;
pub type bb_suid_t = libc::c_uint;
pub const BB_SUID_MAYBE: bb_suid_t = 1;

#[inline(always)]
unsafe extern "C" fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}

// TODO: get rid of Options
#[no_mangle]
pub static mut applet_main: [Option<
  unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
>; 396] = unsafe {
  [
    Some(
      test_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      test_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      acpid_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      add_remove_shell_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      addgroup_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      adduser_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      adjtimex_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      uname_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      arp_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      arping_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ash_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      awk_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      base64_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      basename_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(bc_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(
      beep_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      blkdiscard_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      blkid_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      blockdev_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      bootchartd_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      brctl_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      bunzip2_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      bunzip2_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      bzip2_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      cal_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      cat_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      chat_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      chattr_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      chgrp_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      chmod_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      chown_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      chpasswd_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      chpst_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      chroot_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      chrt_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      chvt_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      cksum_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      clear_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      cmp_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      comm_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      conspy_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(cp_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(
      cpio_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      crond_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      crontab_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      cryptpw_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      cttyhack_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      cut_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      date_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(dc_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(dd_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(
      deallocvt_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      deluser_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      deluser_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      modprobe_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      devmem_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(df_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(
      dhcprelay_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      diff_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      dirname_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      dmesg_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      dnsd_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      hostname_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      dos2unix_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      dpkg_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      dpkg_deb_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(du_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(
      dumpkmap_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      dumpleases_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      echo_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(ed_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(
      grep_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      eject_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      env_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      chpst_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      chpst_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ether_wake_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      expand_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      expr_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      factor_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      fakeidentd_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      fallocate_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      false_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      fatattr_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      fbset_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      fbsplash_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      freeramdisk_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      fdformat_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      fdisk_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      fgconsole_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      grep_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      find_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      findfs_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      flock_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      fold_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      free_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      freeramdisk_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      fsck_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      fsck_minix_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      fsfreeze_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      fstrim_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      fsync_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ftpd_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ftpgetput_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ftpgetput_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      fuser_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      getopt_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      getty_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      grep_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(id_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(
      gunzip_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      gzip_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      halt_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      hexdump_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      hdparm_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      head_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      hexdump_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      hexedit_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      hostid_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      hostname_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      httpd_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      hush_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      hwclock_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      i2cdetect_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      i2cdump_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      i2cget_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      i2cset_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      i2ctransfer_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(id_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(
      ifconfig_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ifupdown_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ifenslave_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ifplugd_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ifupdown_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      inetd_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      init_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      modprobe_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      install_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ionice_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      iostat_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(ip_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(
      ipaddr_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ipcalc_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ipcrm_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ipcs_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      iplink_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ipneigh_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      iproute_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      iprule_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      iptunnel_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      kbd_mode_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      kill_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      kill_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      kill_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      klogd_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      last_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      less_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      link_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      setarch_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      setarch_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      init_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(ln_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(
      loadfont_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      loadkmap_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      logger_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      login_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      logname_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      logread_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      losetup_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      lpd_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      lpqr_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      lpqr_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(ls_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(
      lsattr_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      lsmod_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      lsof_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      lspci_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      lsscsi_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      lsusb_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      unlzma_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      unlzma_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      lzop_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      makedevs_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      makemime_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      man_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      md5_sha1_sum_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      mdev_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      mesg_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      microcom_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      mkdir_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      mkfs_vfat_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      mkfs_ext2_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      mkfifo_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      mkfs_ext2_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      mkfs_minix_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      mkfs_vfat_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      mknod_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      cryptpw_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      mkswap_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      mktemp_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      modinfo_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      modprobe_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      more_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      mount_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      mountpoint_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      mpstat_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(mt_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(mv_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(
      nameif_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      nandwrite_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      nandwrite_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      nbdclient_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(nc_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(
      netstat_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      nice_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(nl_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(
      nmeter_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      nohup_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      scripted_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      nproc_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      nsenter_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      nslookup_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ntpd_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      nuke_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(od_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(
      openvt_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      partprobe_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      passwd_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      paste_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      patch_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      pgrep_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      pidof_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ping_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ping6_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      pipe_progress_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      pivot_root_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      pgrep_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      pmap_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      popmaildir_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      halt_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      powertop_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      printenv_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      printf_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(ps_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(
      pscan_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      pstree_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      pwd_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      pwdx_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      raidautorun_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      rdate_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      rdev_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      readahead_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      readlink_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      readprofile_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      realpath_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      halt_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      reformime_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      add_remove_shell_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      renice_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      reset_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      resize_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      resume_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      rev_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(rm_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(
      rmdir_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      modprobe_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      route_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      rpm_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      rpm2cpio_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      rtcwake_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      switch_root_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      run_parts_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      runlevel_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      runsv_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      runsvdir_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(rx_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(
      script_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      scriptreplay_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      sed_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      sendmail_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      seq_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      setarch_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      setconsole_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      setfattr_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      setfont_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      setkeycodes_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      setlogcons_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      setpriv_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      setserial_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      setsid_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      chpst_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ash_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      md5_sha1_sum_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      md5_sha1_sum_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      md5_sha1_sum_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      md5_sha1_sum_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      showkey_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      shred_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      shuf_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      slattach_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      sleep_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      smemcap_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      chpst_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      sort_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      split_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ssl_client_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      start_stop_daemon_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      stat_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      strings_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      stty_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(su_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(
      sulogin_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      sum_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(sv_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(
      svc_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      svlogd_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      svok_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      swap_on_off_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      swap_on_off_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      switch_root_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      sync_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      sysctl_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      syslogd_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      tac_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      tail_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      tar_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      taskset_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(tc_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(
      tcpudpsvd_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      tee_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      telnet_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      telnetd_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      test_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      tftp_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      tftpd_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      time_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      timeout_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      top_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      touch_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(tr_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(
      traceroute_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      traceroute6_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      true_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      truncate_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(ts_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(
      tty_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ttysize_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      tunctl_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ubi_tools_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ubi_tools_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ubi_tools_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ubirename_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ubi_tools_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ubi_tools_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      ubi_tools_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      udhcpc_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      udhcpc6_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      udhcpd_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      tcpudpsvd_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      uevent_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      umount_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      uname_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      expand_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      uniq_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      dos2unix_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      unlink_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      unlzma_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      unshare_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      unxz_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      unzip_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      uptime_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      who_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      usleep_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      uudecode_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      uuencode_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      vconfig_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(vi_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(
      vlock_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      volname_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      who_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      wall_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      watch_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      watchdog_main
        as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(wc_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int),
    Some(
      wget_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      which_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      who_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      whoami_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      whois_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      xargs_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      xxd_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      unxz_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      unxz_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      yes_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      gunzip_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
    Some(
      zcip_main as unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
    ),
  ]
};

#[no_mangle]
pub static mut applet_suid: [uint8_t; 99] = [
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0x2i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0x1i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0x8i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0x40i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0x80i32 as uint8_t,
  0i32 as uint8_t,
  0x5i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0x20i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0x5i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0x8i32 as uint8_t,
  0x2i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
  0i32 as uint8_t,
];

#[no_mangle]
pub static mut applet_install_loc: [uint8_t; 198] = [
  0x33i32 as uint8_t,
  0x42i32 as uint8_t,
  0x44i32 as uint8_t,
  0x12i32 as uint8_t,
  0x42i32 as uint8_t,
  0x31i32 as uint8_t,
  0x31i32 as uint8_t,
  0x33i32 as uint8_t,
  0x23i32 as uint8_t,
  0x22i32 as uint8_t,
  0x34i32 as uint8_t,
  0x33i32 as uint8_t,
  0x13i32 as uint8_t,
  0x14i32 as uint8_t,
  0x11i32 as uint8_t,
  0x41i32 as uint8_t,
  0x43i32 as uint8_t,
  0x33i32 as uint8_t,
  0x33i32 as uint8_t,
  0x33i32 as uint8_t,
  0x11i32 as uint8_t,
  0x41i32 as uint8_t,
  0x33i32 as uint8_t,
  0x31i32 as uint8_t,
  0x31i32 as uint8_t,
  0x31i32 as uint8_t,
  0x44i32 as uint8_t,
  0x22i32 as uint8_t,
  0x41i32 as uint8_t,
  0x33i32 as uint8_t,
  0x41i32 as uint8_t,
  0x31i32 as uint8_t,
  0x33i32 as uint8_t,
  0x13i32 as uint8_t,
  0x13i32 as uint8_t,
  0x11i32 as uint8_t,
  0x33i32 as uint8_t,
  0x33i32 as uint8_t,
  0x34i32 as uint8_t,
  0x33i32 as uint8_t,
  0x34i32 as uint8_t,
  0x11i32 as uint8_t,
  0x24i32 as uint8_t,
  0x41i32 as uint8_t,
  0x32i32 as uint8_t,
  0x31i32 as uint8_t,
  0x32i32 as uint8_t,
  0x33i32 as uint8_t,
  0x22i32 as uint8_t,
  0x42i32 as uint8_t,
  0x12i32 as uint8_t,
  0x34i32 as uint8_t,
  0x33i32 as uint8_t,
  0x21i32 as uint8_t,
  0x31i32 as uint8_t,
  0x11i32 as uint8_t,
  0x32i32 as uint8_t,
  0x32i32 as uint8_t,
  0x33i32 as uint8_t,
  0x13i32 as uint8_t,
  0x14i32 as uint8_t,
  0x42i32 as uint8_t,
  0x44i32 as uint8_t,
  0x44i32 as uint8_t,
  0x23i32 as uint8_t,
  0x22i32 as uint8_t,
  0x24i32 as uint8_t,
  0x24i32 as uint8_t,
  0x32i32 as uint8_t,
  0x11i32 as uint8_t,
  0x22i32 as uint8_t,
  0x31i32 as uint8_t,
  0x23i32 as uint8_t,
  0x22i32 as uint8_t,
  0x22i32 as uint8_t,
  0x11i32 as uint8_t,
  0x43i32 as uint8_t,
  0x32i32 as uint8_t,
  0x13i32 as uint8_t,
  0x11i32 as uint8_t,
  0x10i32 as uint8_t,
  0x24i32 as uint8_t,
  0x13i32 as uint8_t,
  0x23i32 as uint8_t,
  0x42i32 as uint8_t,
  0x33i32 as uint8_t,
  0x11i32 as uint8_t,
  0x32i32 as uint8_t,
  0x33i32 as uint8_t,
  0x33i32 as uint8_t,
  0x13i32 as uint8_t,
  0x12i32 as uint8_t,
  0x33i32 as uint8_t,
  0x32i32 as uint8_t,
  0x13i32 as uint8_t,
  0x22i32 as uint8_t,
  0x23i32 as uint8_t,
  0x22i32 as uint8_t,
  0x31i32 as uint8_t,
  0x12i32 as uint8_t,
  0x22i32 as uint8_t,
  0x11i32 as uint8_t,
  0x11i32 as uint8_t,
  0x11i32 as uint8_t,
  0x42i32 as uint8_t,
  0x44i32 as uint8_t,
  0x13i32 as uint8_t,
  0x31i32 as uint8_t,
  0x33i32 as uint8_t,
  0x34i32 as uint8_t,
  0x33i32 as uint8_t,
  0x14i32 as uint8_t,
  0x33i32 as uint8_t,
  0x34i32 as uint8_t,
  0x33i32 as uint8_t,
  0x13i32 as uint8_t,
  0x11i32 as uint8_t,
  0x21i32 as uint8_t,
  0x33i32 as uint8_t,
  0x24i32 as uint8_t,
  0x14i32 as uint8_t,
  0x13i32 as uint8_t,
  0x33i32 as uint8_t,
  0x31i32 as uint8_t,
  0x42i32 as uint8_t,
  0x44i32 as uint8_t,
  0x43i32 as uint8_t,
  0x23i32 as uint8_t,
  0x41i32 as uint8_t,
  0x33i32 as uint8_t,
  0x13i32 as uint8_t,
  0x11i32 as uint8_t,
  0x21i32 as uint8_t,
  0x12i32 as uint8_t,
  0x43i32 as uint8_t,
  0x12i32 as uint8_t,
  0x32i32 as uint8_t,
  0x33i32 as uint8_t,
  0x13i32 as uint8_t,
  0x41i32 as uint8_t,
  0x13i32 as uint8_t,
  0x32i32 as uint8_t,
  0x34i32 as uint8_t,
  0x14i32 as uint8_t,
  0x31i32 as uint8_t,
  0x13i32 as uint8_t,
  0x33i32 as uint8_t,
  0x33i32 as uint8_t,
  0x33i32 as uint8_t,
  0x23i32 as uint8_t,
  0x31i32 as uint8_t,
  0x33i32 as uint8_t,
  0x33i32 as uint8_t,
  0x12i32 as uint8_t,
  0x13i32 as uint8_t,
  0x21i32 as uint8_t,
  0x33i32 as uint8_t,
  0x43i32 as uint8_t,
  0x23i32 as uint8_t,
  0x22i32 as uint8_t,
  0x21i32 as uint8_t,
  0x32i32 as uint8_t,
  0x13i32 as uint8_t,
  0x23i32 as uint8_t,
  0x33i32 as uint8_t,
  0x43i32 as uint8_t,
  0x33i32 as uint8_t,
  0x34i32 as uint8_t,
  0x33i32 as uint8_t,
  0x31i32 as uint8_t,
  0x33i32 as uint8_t,
  0x31i32 as uint8_t,
  0x33i32 as uint8_t,
  0x23i32 as uint8_t,
  0x44i32 as uint8_t,
  0x44i32 as uint8_t,
  0x44i32 as uint8_t,
  0x24i32 as uint8_t,
  0x43i32 as uint8_t,
  0x23i32 as uint8_t,
  0x11i32 as uint8_t,
  0x33i32 as uint8_t,
  0x33i32 as uint8_t,
  0x33i32 as uint8_t,
  0x33i32 as uint8_t,
  0x33i32 as uint8_t,
  0x31i32 as uint8_t,
  0x23i32 as uint8_t,
  0x31i32 as uint8_t,
  0x33i32 as uint8_t,
  0x13i32 as uint8_t,
  0x32i32 as uint8_t,
  0x33i32 as uint8_t,
  0x33i32 as uint8_t,
  0x33i32 as uint8_t,
  0x33i32 as uint8_t,
  0x33i32 as uint8_t,
  0x21i32 as uint8_t,
];

#[no_mangle]
pub static mut applet_numbers: [uint16_t; 1] = [218i32 as uint16_t];

/*
 * Utility routines.
 *
 * Copyright (C) tons of folks.  Tracking down who wrote what
 * isn't something I'm going to worry about...  If you wrote something
 * here, please feel free to acknowledge your work.
 *
 * Based in part on code from sash, Copyright (c) 1999 by David I. Bell
 * Permission has been granted to redistribute this code under GPL.
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* We are trying to not use printf, this benefits the case when selected
 * applets are really simple. Example:
 *
 * $ ./busybox
 * ...
 * Currently defined functions:
 *         basename, false, true
 *
 * $ size busybox
 *    text    data     bss     dec     hex filename
 *    4473      52      72    4597    11f5 busybox
 *
 * FEATURE_INSTALLER or FEATURE_SUID will still link printf routines in. :(
 */
/* for mallopt */
/* Declare <applet>_main() */
/* Include generated applet names, pointers to <applet>_main, etc */
/* ...and if applet_tables generator says we have only one applet... */
static mut packed_scripts: [libc::c_char; 111] = [
  104i32 as libc::c_char,
  0o61i32 as libc::c_char,
  0o61i32 as libc::c_char,
  0o101i32 as libc::c_char,
  0o131i32 as libc::c_char,
  0o46i32 as libc::c_char,
  0o123i32 as libc::c_char,
  0o131i32 as libc::c_char,
  0o71i32 as libc::c_char,
  0o173i32 as libc::c_char,
  0o333i32 as libc::c_char,
  0o14i32 as libc::c_char,
  0i32 as libc::c_char,
  0i32 as libc::c_char,
  0o20i32 as libc::c_char,
  0o133i32 as libc::c_char,
  128i32 as libc::c_char,
  0o100i32 as libc::c_char,
  0o20i32 as libc::c_char,
  0o100i32 as libc::c_char,
  0o1i32 as libc::c_char,
  0o262i32 as libc::c_char,
  0o1i32 as libc::c_char,
  0o4i32 as libc::c_char,
  0i32 as libc::c_char,
  0o76i32 as libc::c_char,
  0o345i32 as libc::c_char,
  0o317i32 as libc::c_char,
  0o104i32 as libc::c_char,
  0o40i32 as libc::c_char,
  0i32 as libc::c_char,
  0o124i32 as libc::c_char,
  82i32 as libc::c_char,
  0o171i32 as libc::c_char,
  0o46i32 as libc::c_char,
  0o151i32 as libc::c_char,
  0o66i32 as libc::c_char,
  0o243i32 as libc::c_char,
  0o324i32 as libc::c_char,
  0o300i32 as libc::c_char,
  0o214i32 as libc::c_char,
  0o320i32 as libc::c_char,
  0o315i32 as libc::c_char,
  0o350i32 as libc::c_char,
  0o211i32 as libc::c_char,
  0o204i32 as libc::c_char,
  0o144i32 as libc::c_char,
  0o321i32 as libc::c_char,
  140i32 as libc::c_char,
  0o221i32 as libc::c_char,
  0o240i32 as libc::c_char,
  0o163i32 as libc::c_char,
  0o350i32 as libc::c_char,
  0o254i32 as libc::c_char,
  0o104i32 as libc::c_char,
  0o123i32 as libc::c_char,
  0o320i32 as libc::c_char,
  0o45i32 as libc::c_char,
  0o315i32 as libc::c_char,
  0o147i32 as libc::c_char,
  0o177i32 as libc::c_char,
  0o211i32 as libc::c_char,
  0o236i32 as libc::c_char,
  0o15i32 as libc::c_char,
  193i32 as libc::c_char,
  0o264i32 as libc::c_char,
  0o257i32 as libc::c_char,
  0o36i32 as libc::c_char,
  0o312i32 as libc::c_char,
  0o216i32 as libc::c_char,
  0o264i32 as libc::c_char,
  0o36i32 as libc::c_char,
  0o32i32 as libc::c_char,
  0o151i32 as libc::c_char,
  0o152i32 as libc::c_char,
  0o342i32 as libc::c_char,
  0o21i32 as libc::c_char,
  0o134i32 as libc::c_char,
  0o361i32 as libc::c_char,
  0o243i32 as libc::c_char,
  214i32 as libc::c_char,
  0o256i32 as libc::c_char,
  0o246i32 as libc::c_char,
  0o344i32 as libc::c_char,
  0o47i32 as libc::c_char,
  0o4i32 as libc::c_char,
  0o113i32 as libc::c_char,
  0o14i32 as libc::c_char,
  0o356i32 as libc::c_char,
  0o144i32 as libc::c_char,
  0o317i32 as libc::c_char,
  0o120i32 as libc::c_char,
  0o74i32 as libc::c_char,
  0o70i32 as libc::c_char,
  0o75i32 as libc::c_char,
  0o100i32 as libc::c_char,
  69i32 as libc::c_char,
  0o374i32 as libc::c_char,
  0o133i32 as libc::c_char,
  0o364i32 as libc::c_char,
  0o330i32 as libc::c_char,
  0o273i32 as libc::c_char,
  0o222i32 as libc::c_char,
  0o51i32 as libc::c_char,
  0o302i32 as libc::c_char,
  0o204i32 as libc::c_char,
  0o201i32 as libc::c_char,
  0o313i32 as libc::c_char,
  0o336i32 as libc::c_char,
  0o330i32 as libc::c_char,
  0o140i32 as libc::c_char,
];
/* "Do not compress usage text if uncompressed text is small
 *  and we don't include bunzip2 code for other reasons"
 *
 * Useful for mass one-applet rebuild (bunzip2 code is ~2.7k).
 *
 * Unlike BUNZIP2, if FEATURE_SEAMLESS_BZ2 is on, bunzip2 code is built but
 * still may be unused if none of the selected applets calls open_zipped()
 * or its friends; we test for (FEATURE_SEAMLESS_BZ2 && <APPLET>) instead.
 * For example, only if TAR and FEATURE_SEAMLESS_BZ2 are both selected,
 * then bunzip2 code will be linked in anyway, and disabling help compression
 * would be not optimal:
 */

#[no_mangle]
pub unsafe extern "C" fn string_array_len(argv: *mut *mut libc::c_char) -> libc::c_uint {
  let mut start: *mut *mut libc::c_char = argv;
  let mut current: *mut *mut libc::c_char = argv;
  while !(*current).is_null() {
    current = current.offset(1)
  }
  return current.wrapping_offset_from(start) as libc::c_long as libc::c_uint;
}

#[no_mangle]
pub unsafe extern "C" fn bb_show_usage() -> ! {
  panic!("TODO: implement bb_show_usage");

  // let mut p: *const libc::c_char = 0 as *const libc::c_char; /* common string */
  // p = unpack_bz2_data(
  //   packed_usage.as_ptr(),
  //   ::std::mem::size_of::<[libc::c_char; 33501]>() as libc::c_ulong as libc::c_int,
  //   ::std::mem::size_of::<[libc::c_char; 95697]>() as libc::c_ulong as libc::c_int,
  // );
  // let mut usage_string: *const libc::c_char = p;
  // let mut ap: libc::c_int = find_applet_by_name(applet_name);
  // if ap < 0i32 || usage_string.is_null() {
  //   xfunc_die();
  // }
  // while ap != 0 {
  //   loop {
  //     let fresh0 = p;
  //     p = p.offset(1);
  //     if !(*fresh0 != 0) {
  //       break;
  //     }
  //   }
  //   ap -= 1
  // }
  // full_write2_str(bb_banner.as_ptr());
  // full_write2_str(b" multi-call binary.\n\x00" as *const u8 as *const libc::c_char);
  // if *p as libc::c_int == '\u{8}' as i32 {
  //   full_write2_str(b"\nNo help available\n\x00" as *const u8 as *const libc::c_char);
  // } else {
  //   full_write2_str(b"\nUsage: \x00" as *const u8 as *const libc::c_char);
  //   full_write2_str(applet_name);
  //   if *p.offset(0) != 0 {
  //     if *p.offset(0) as libc::c_int != '\n' as i32 {
  //       full_write2_str(b" \x00" as *const u8 as *const libc::c_char);
  //     }
  //     full_write2_str(p);
  //   }
  //   full_write2_str(b"\n\x00" as *const u8 as *const libc::c_char);
  // }
  // xfunc_die();
}

unsafe fn find_applet_by_name(name: &str) -> Option<usize> {
  applet_names_sorted()
    .binary_search(&String::from(name))
    .ok()
}

/* The code below can well be in applets/applets.c, as it is used only
 * for busybox binary, not "individual" binaries.
 * However, keeping it here and linking it into libbusybox.so
 * (together with remaining tiny applets/applets.o)
 * makes it possible to avoid --whole-archive at link time.
 * This makes (shared busybox) + libbusybox smaller.
 * (--gc-sections would be even better....)
 */
#[no_mangle]
pub static mut applet_name: *const libc::c_char = 0 as *const libc::c_char;

/* If not built as a single-applet executable... */
static mut ruid: uid_t = 0;
static mut suid_config: *mut suid_config_t = 0 as *const suid_config_t as *mut suid_config_t;
static mut suid_cfg_readable: bool = false;

/* libbb candidate */
unsafe extern "C" fn get_trimmed_slice(
  mut s: *mut libc::c_char,
  mut e: *mut libc::c_char,
) -> *mut libc::c_char {
  loop
  /* First, consider the value at e to be nul and back up until we
   * reach a non-space char.  Set the char after that (possibly at
   * the original e) to nul. */
  {
    let fresh2 = e;
    e = e.offset(-1);
    if !(fresh2 > s) {
      break;
    }
    if ({
      let mut bb__isspace: libc::c_uchar = (*e as libc::c_int - 9i32) as libc::c_uchar;
      (bb__isspace as libc::c_int == ' ' as i32 - 9i32
        || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
    }) == 0
    {
      break;
    }
  }
  *e.offset(1) = '\u{0}' as i32 as libc::c_char;
  /* Next, advance past all leading space and return a ptr to the
   * first non-space char; possibly the terminating nul. */
  return skip_whitespace(s);
}

unsafe extern "C" fn parse_config_file() {
  /* Don't depend on the tools to combine strings. */
  static mut config_file: [libc::c_char; 18] = [
    47, 101, 116, 99, 47, 98, 117, 115, 121, 98, 111, 120, 46, 99, 111, 110, 102, 0,
  ];
  let mut sct_head: *mut suid_config_t = 0 as *mut suid_config_t;
  let mut applet_no: libc::c_int = 0;
  let mut f: *mut FILE = 0 as *mut FILE;
  let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
  let mut lc: libc::c_uint = 0;
  let mut section: smallint = 0;
  let mut st: stat = stat {
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
  ruid = getuid();
  if ruid == 0i32 as libc::c_uint {
    /* run by root - don't need to even read config file */
    return;
  }
  if stat(config_file.as_ptr(), &mut st) != 0i32
    || !(st.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint)
    || st.st_uid != 0i32 as libc::c_uint
    || st.st_mode & (0o200i32 >> 3i32 | 0o200i32 >> 3i32 >> 3i32) as libc::c_uint != 0
    || {
      f = fopen_for_read(config_file.as_ptr());
      f.is_null()
    }
  {
    /* Cannot open? */
    return;
  } /* while (1) */
  suid_cfg_readable = 1i32 != 0;
  sct_head = 0 as *mut suid_config_t;
  lc = 0i32 as libc::c_uint;
  section = lc as smallint;
  's_65: loop {
    let mut buffer: [libc::c_char; 256] = [0; 256];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if fgets_unlocked(
      buffer.as_mut_ptr(),
      ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
      f,
    )
    .is_null()
    {
      /* Are we done? */
      // Looks like bloat
      //if (ferror(f)) {   /* Make sure it wasn't a read error. */
      //	errmsg = "reading";
      //	goto pe_label;
      //}
      fclose(f); /* Success, so set the pointer. */
      suid_config = sct_head; /* Got a (partial) line. */
      return;
    }
    s = buffer.as_mut_ptr();
    lc = lc.wrapping_add(1);
    /* If a line is too long for our buffer, we consider it an error.
     * The following test does mistreat one corner case though.
     * If the final line of the file does not end with a newline and
     * yet exactly fills the buffer, it will be treated as too long
     * even though there isn't really a problem.  But it isn't really
     * worth adding code to deal with such an unlikely situation, and
     * we do err on the side of caution.  Besides, the line would be
     * too long if it did end with a newline. */
    if strchr(s, '\n' as i32).is_null() && feof_unlocked(f) == 0 {
      errmsg = b"line too long\x00" as *const u8 as *const libc::c_char;
      break;
    } else {
      /* Trim leading and trailing whitespace, ignoring comments, and
       * check if the resulting string is empty. */
      s = get_trimmed_slice(s, strchrnul(s, '#' as i32));
      if *s == 0 {
        continue;
      }
      /* Check for a section header. */
      if *s as libc::c_int == '[' as i32 {
        /* Unlike the old code, we ignore leading and trailing
         * whitespace for the section name.  We also require that
         * there are no stray characters after the closing bracket. */
        let mut e: *mut libc::c_char = strchr(s, ']' as i32);
        if e.is_null() || *e.offset(1) as libc::c_int != 0 || {
          s = get_trimmed_slice(s.offset(1), e);
          (*s) == 0
        } {
          /* Missing name? */
          errmsg = b"section header\x00" as *const u8 as *const libc::c_char;
          break;
        } else if strcasecmp(s, b"SUID\x00" as *const u8 as *const libc::c_char) == 0i32 {
          section = 1i32 as smallint
        } else {
          /* Right now we only have one section so just check it.
           * If more sections are added in the future, please don't
           * resort to cascading ifs with multiple strcasecmp calls.
           * That kind of bloated code is all too common.  A loop
           * and a string table would be a better choice unless the
           * number of sections is very small. */
          section = -1i32 as smallint
        }
      } else if section as libc::c_int == 1i32 {
        /* Unknown section so set to skip. */
        /* Process sections. */
        /* SUID */
        /* Since we trimmed leading and trailing space above, we're
         * now looking for strings of the form
         *    <key>[::space::]*=[::space::]*<value>
         * where both key and value could contain inner whitespace. */
        /* First get the key (an applet name in our case). */
        let mut e_0: *mut libc::c_char = strchr(s, '=' as i32);
        if !e_0.is_null() {
          s = get_trimmed_slice(s, e_0)
        }
        if e_0.is_null() || *s == 0 {
          /* Missing '=' or empty key. */
          errmsg = b"keyword\x00" as *const u8 as *const libc::c_char;
          break;
        } else {
          /* Ok, we have an applet name.  Process the rhs if this
           * applet is currently built in and ignore it otherwise.
           * Note: this can hide config file bugs which only pop
           * up when the busybox configuration is changed. */
          match find_applet_by_name(&ptr_to_str(s)) {
            None => continue,
            Some(n) => applet_no = n as i32,
          };

          let mut i: libc::c_uint = 0;
          let mut sct: *mut suid_config_t = 0 as *mut suid_config_t;
          /* Note: We currently don't check for duplicates!
           * The last config line for each applet will be the
           * one used since we insert at the head of the list.
           * I suppose this could be considered a feature. */
          sct =
            xzalloc(::std::mem::size_of::<suid_config_t>() as libc::c_ulong) as *mut suid_config_t;
          (*sct).m_applet = applet_no;
          /*sct->m_mode = 0;*/
          (*sct).m_next = sct_head;
          sct_head = sct;
          /* Get the specified mode. */
          e_0 = skip_whitespace(e_0.offset(1));
          i = 0i32 as libc::c_uint;
          while i < 3i32 as libc::c_uint {
            /* There are 4 chars for each of user/group/other.
             * "x-xx" instead of "x-" are to make
             * "idx > 3" check catch invalid chars.
             */
            static mut mode_chars: [libc::c_char; 13] =
              [83, 115, 120, 45, 83, 115, 120, 45, 120, 45, 120, 120, 0];
            static mut mode_mask: [libc::c_ushort; 10] = [
              0o4000i32 as libc::c_ushort,
              (0o4000i32 | 0o100i32) as libc::c_ushort,
              0o100i32 as libc::c_ushort,
              0i32 as libc::c_ushort,
              0o2000i32 as libc::c_ushort,
              (0o2000i32 | 0o100i32 >> 3i32) as libc::c_ushort,
              (0o100i32 >> 3i32) as libc::c_ushort,
              0i32 as libc::c_ushort,
              (0o100i32 >> 3i32 >> 3i32) as libc::c_ushort,
              0i32 as libc::c_ushort,
            ];
            let mut q: *const libc::c_char = strchrnul(
              mode_chars
                .as_ptr()
                .offset((4i32 as libc::c_uint).wrapping_mul(i) as isize),
              *e_0 as libc::c_int,
            );
            let mut idx: libc::c_uint = q.wrapping_offset_from(
              mode_chars
                .as_ptr()
                .offset((4i32 as libc::c_uint).wrapping_mul(i) as isize),
            ) as libc::c_long as libc::c_uint;
            if idx > 3i32 as libc::c_uint {
              errmsg = b"mode\x00" as *const u8 as *const libc::c_char;
              break 's_65;
            } else {
              (*sct).m_mode |= mode_mask
                [q.wrapping_offset_from(mode_chars.as_ptr()) as libc::c_long as usize]
                as libc::c_uint;
              e_0 = e_0.offset(1);
              i = i.wrapping_add(1)
            }
          }
          /* Now get the user/group info. */
          s = skip_whitespace(e_0);
          /* Default is 0.0, else parse USER.GROUP: */
          if !(*s != 0) {
            continue;
          }
          /* We require whitespace between mode and USER.GROUP */
          if s == e_0 || {
            e_0 = strchr(s, '.' as i32); /* get_uidgid needs USER:GROUP syntax */
            e_0.is_null()
          } {
            errmsg = b"uid.gid\x00" as *const u8 as *const libc::c_char;
            break;
          } else {
            *e_0 = ':' as i32 as libc::c_char;
            if !(get_uidgid(&mut (*sct).m_ugid, s) == 0i32) {
              continue;
            }
            errmsg = b"unknown user/group\x00" as *const u8 as *const libc::c_char;
            break;
          }
        }
      } else {
        /* Unknown sections are ignored. */
        /* Encountering configuration lines prior to seeing a
         * section header is treated as an error.  This is how
         * the old code worked, but it may not be desirable.
         * We may want to simply ignore such lines in case they
         * are used in some future version of busybox. */
        if !(section == 0) {
          continue;
        }
        errmsg = b"keyword outside section\x00" as *const u8 as *const libc::c_char;
        break;
      }
    }
  }
  fclose(f);
  bb_error_msg(
    b"parse error in %s, line %u: %s\x00" as *const u8 as *const libc::c_char,
    config_file.as_ptr(),
    lc,
    errmsg,
  );
  /* Release any allocated memory before returning. */
  llist_free(sct_head as *mut llist_t, None);
}

/* check if u is member of group g */
unsafe extern "C" fn ingroup(mut u: uid_t, mut g: gid_t) -> libc::c_int {
  let mut grp: *mut group = bb_internal_getgrgid(g); /* real gid */
  if !grp.is_null() {
    let mut mem: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char; /* run by root - no need to check more */
    mem = (*grp).gr_mem;
    while !(*mem).is_null() {
      let mut pwd: *mut passwd = bb_internal_getpwnam(*mem);
      if !pwd.is_null() && (*pwd).pw_uid == u {
        return 1i32;
      }
      mem = mem.offset(1)
    }
  }
  return 0i32;
}

// TODO: make this usize
unsafe extern "C" fn check_suid(mut applet_no: libc::c_int) {
  let mut current_block: u64;
  let mut rgid: gid_t = 0;
  if ruid == 0i32 as libc::c_uint {
    /* set by parse_config_file() */
    return;
  }
  rgid = getgid();
  if suid_cfg_readable {
    let mut uid: uid_t = 0;
    let mut sct: *mut suid_config_t = 0 as *mut suid_config_t;
    let mut m: mode_t = 0;
    sct = suid_config;
    loop {
      if sct.is_null() {
        current_block = 7187160828046810477;
        break;
      }
      if (*sct).m_applet == applet_no {
        current_block = 14059243314339256598;
        break;
      }
      sct = (*sct).m_next
    }
    match current_block {
      7187160828046810477 => {}
      _ => {
        /* Is this user allowed to run this applet? */
        m = (*sct).m_mode;
        if (*sct).m_ugid.uid == ruid {
          /* same uid */
          m >>= 6i32
        } else if (*sct).m_ugid.gid == rgid || ingroup(ruid, (*sct).m_ugid.gid) != 0 {
          /* same group / in group */
          m >>= 3i32
        }
        if m & (0o100i32 >> 3i32 >> 3i32) as libc::c_uint == 0 {
          /* is x bit not set? */
          bb_simple_error_msg_and_die(
            b"you have no permission to run this applet\x00" as *const u8 as *const libc::c_char,
          );
        }
        /* We set effective AND saved ids. If saved-id is not set
         * like we do below, seteuid(0) can still later succeed! */
        /* Are we directed to change gid
         * (APPLET = *s* USER.GROUP or APPLET = *S* USER.GROUP)?
         */
        if (*sct).m_mode & 0o2000i32 as libc::c_uint != 0 {
          rgid = (*sct).m_ugid.gid
        }
        /* else: we will set egid = rgid, thus dropping sgid effect */
        if setresgid(-1i32 as __gid_t, rgid, rgid) != 0 {
          bb_simple_perror_msg_and_die(b"setresgid\x00" as *const u8 as *const libc::c_char);
        }
        /* Are we directed to change uid
         * (APPLET = s** USER.GROUP or APPLET = S** USER.GROUP)?
         */
        uid = ruid;
        if (*sct).m_mode & 0o4000i32 as libc::c_uint != 0 {
          uid = (*sct).m_ugid.uid
        }
        /* else: we will set euid = ruid, thus dropping suid effect */
        if setresuid(-1i32 as __uid_t, uid, uid) != 0 {
          bb_simple_perror_msg_and_die(b"setresuid\x00" as *const u8 as *const libc::c_char);
        }
        current_block = 14136749492126903395;
      }
    }
  } else {
    current_block = 7187160828046810477;
  }
  match current_block {
    7187160828046810477 => {
      if applet_suid[(applet_no / 4i32) as usize] as libc::c_int >> 2i32 * (applet_no % 4i32) & 3i32
        == BB_SUID_REQUIRE as libc::c_int
      {
        /* Real uid is not 0. If euid isn't 0 too, suid bit
         * is most probably not set on our executable */
        if geteuid() != 0 {
          bb_simple_error_msg_and_die(
            b"must be suid to work properly\x00" as *const u8 as *const libc::c_char,
          );
        }
      } else if applet_suid[(applet_no / 4i32) as usize] as libc::c_int >> 2i32 * (applet_no % 4i32)
        & 3i32
        == BB_SUID_DROP as libc::c_int
      {
        /*
         * Drop all privileges.
         *
         * Don't check for errors: in normal use, they are impossible,
         * and in special cases, exiting is harmful. Example:
         * 'unshare --user' when user's shell is also from busybox.
         *
         * 'unshare --user' creates a new user namespace without any
         * uid mappings. Thus, busybox binary is setuid nobody:nogroup
         * within the namespace, as that is the only user. However,
         * since no uids are mapped, calls to setgid/setuid
         * fail (even though they would do nothing).
         */
        setgid(rgid);
        setuid(ruid);
      }
    }
    _ => {}
  }
  llist_free(suid_config as *mut llist_t, None);
}

/* FEATURE_SUID */
static mut usr_bin: [libc::c_char; 10] = [47, 117, 115, 114, 47, 98, 105, 110, 47, 0];
static mut usr_sbin: [libc::c_char; 11] = [47, 117, 115, 114, 47, 115, 98, 105, 110, 47, 0];

// Initialized in run_static_initializers
static mut install_dir: [*const libc::c_char; 5] = [0 as *const libc::c_char; 5];

unsafe fn applet_names_sorted() -> Vec<String> {
  let mut ret: Vec<String> = applets.iter().map(|a| ptr_to_str(a.name)).collect();
  ret.sort();
  ret
}

unsafe fn APPLET_INSTALL_LOC(i: usize) -> usize {
  if i & 1 != 0 {
    (applet_install_loc[i.wrapping_div(2)] >> 4) as usize
  } else {
    (applet_install_loc[i.wrapping_div(2)] & 0xf) as usize
  }
}

/* create (sym)links for each applet */
unsafe extern "C" fn install_links(
  mut rustybox_path: *const libc::c_char,
  use_symbolic_links: bool,
  mut custom_install_dir: *mut libc::c_char,
) {
  /* directory table
   * this should be consistent w/ the enum,
   * busybox.h::bb_install_loc_t, or else... */
  let mut fpc: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut rc: libc::c_int = 0;

  let lf = if use_symbolic_links {
    symlink as unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int
  } else {
    link as unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int
  };

  let appnames = applet_names_sorted();
  let mut i: usize = 0;
  while i < applet_main.len() {
    fpc = concat_path_file(
      if !custom_install_dir.is_null() {
        custom_install_dir
      } else {
        install_dir[APPLET_INSTALL_LOC(i)]
      },
      str_to_ptr(&appnames[i]),
    );

    // debug: bb_error_msg("%slinking %s to busybox",
    //		use_symbolic_links ? "sym" : "", fpc);
    rc = lf(rustybox_path, fpc);
    if rc != 0i32 && *bb_errno != 17i32 {
      bb_simple_perror_msg(fpc);
    }
    free(fpc as *mut libc::c_void);

    i += 1
  }
}

unsafe fn find_script_by_name(name: &str) -> Option<usize> {
  find_applet_by_name(name)
    .and_then(|applet_no| applet_numbers.iter().position(|&i| i as usize == applet_no))
}

// Originally:
// int scripted_main(int argc UNUSED_PARAM, char **argv)
// {
//   int script = find_script_by_name(applet_name);
//   if (script >= 0)
// #if ENABLE_ASH || ENABLE_SH_IS_ASH || ENABLE_BASH_IS_ASH
//     exit(ash_main(-script - 1, argv));
// #elif ENABLE_HUSH || ENABLE_SH_IS_HUSH || ENABLE_BASH_IS_HUSH
//     exit(hush_main(-script - 1, argv));
// #else
//     return 1;
// #endif
//   return 0;
// }
#[no_mangle]
pub unsafe extern "C" fn scripted_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let exitcode = match find_script_by_name(&ptr_to_str(applet_name)) {
    Some(script) => ash_main(-(script as i32) - 1, argv),
    None => 0,
  };
  ::std::process::exit(exitcode)
}

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
#[no_mangle]
pub unsafe extern "C" fn get_script_content(mut n: libc::c_uint) -> *mut libc::c_char {
  let mut t: *mut libc::c_char = unpack_bz2_data(
    packed_scripts.as_ptr(),
    ::std::mem::size_of::<[libc::c_char; 111]>() as libc::c_ulong as libc::c_int,
    87i32,
  );
  if !t.is_null() {
    while n != 0i32 as libc::c_uint {
      loop {
        let fresh4 = t;
        t = t.offset(1);
        if !(*fresh4 as libc::c_int != '\u{0}' as i32) {
          break;
        }
      }
      n = n.wrapping_sub(1)
    }
  }
  return t;
}

unsafe fn print_rustybox_help() {
  /* -1 prevent last comma to be in the very last pos */
  let output_width = get_terminal_width(2) - 1;

  // See https://linux.die.net/man/2/dup2.
  dup2(1, 2);
  full_write2_str(bb_banner.as_ptr()); /* reuse const string */
  eprintln!(" multi-call binary."); /* reuse */
  eprintln!("BusyBox is copyrighted by many authors between 1998-2015.\nLicensed under GPLv2. See source distribution for detailed\ncopyright notices.\n\nUsage: busybox [function [arguments]...]\n   or: busybox --list[-full]\n   or: busybox --show SCRIPT\n   or: busybox --install [-s] [DIR]\n   or: function [arguments]...\n\n\tBusyBox is a multi-call binary that combines many common Unix\n\tutilities into a single executable.  Most people will create a\n\tlink to busybox for each function they wish to use and BusyBox\n\twill act like whatever it was invoked as.\n\nCurrently defined functions:");

  let mut col: libc::c_int = 0;
  for appname in applet_names_sorted().iter() {
    let len2 = appname.len() + 2;
    if col >= output_width - len2 as i32 {
      eprintln!(",");
      col = 0
    }
    if col == 0 {
      col = 6;
      eprint!("\t")
    } else {
      eprint!(", ")
    }
    eprint!("{}", appname);
    col += len2 as i32;
  }
  eprintln!();
}

unsafe fn rustybox_main(argv: &[String]) -> i32 {
  if argv.len() == 1 {
    /* Called without arguments */
    print_rustybox_help();
    return 0;
  } else {
    if argv[1] == "--show" {
      if argv.len() < 3 {
        bb_error_msg_and_die(
          bb_msg_requires_arg.as_ptr(),
          b"--show\x00" as *const u8 as *const libc::c_char,
        );
      }
      match find_script_by_name(&argv[2]) {
        None => bb_error_msg_and_die(
          b"script \'%s\' not found\x00" as *const u8 as *const libc::c_char,
          str_to_ptr(&argv[2]),
        ),

        Some(n) => {
          full_write1_str(get_script_content(n as libc::c_uint));
          return 0;
        }
      }
    }

    if argv[1].starts_with("--list") {
      dup2(1, 2);
      let appnames = applet_names_sorted();
      let mut i: usize = 0;
      while i < applet_main.len() {
        if argv[1].len() > 6 {
          /* --list-full? */
          full_write2_str(install_dir[APPLET_INSTALL_LOC(i)].offset(1));
        }

        eprintln!("{}", &appnames[i]);
        i += 1
      }
      return 0;
    }

    if argv[1] == "--install" {
      let mut busybox: *const libc::c_char = 0 as *const libc::c_char;
      busybox = xmalloc_readlink(bb_busybox_exec_path.as_ptr());
      if busybox.is_null() {
        /* bb_busybox_exec_path is usually "/proc/self/exe".
         * In chroot, readlink("/proc/self/exe") usually fails.
         * In such case, better use argv[0] as symlink target
         * if it is a full path name.
         */
        if !argv[0].starts_with("/") {
          bb_error_msg_and_die(
            b"\'%s\' is not an absolute path\x00" as *const u8 as *const libc::c_char,
            str_to_ptr(&argv[0]),
          );
        }
        busybox = str_to_ptr(&argv[0])
      }

      /* busybox --install [-s] [DIR]:
       * -s: make symlinks
       * DIR: directory to install links to
       */
      let use_symbolic_links = (argv.len() > 2) && (argv[2] == "-s");
      install_links(busybox, use_symbolic_links, str_to_ptr(&argv[3]));
      return 0;
    }

    /* We support "busybox /a/path/to/applet args..." too. Allows for
     * "#!/bin/busybox"-style wrappers */
    applet_name = bb_get_last_path_component_nostrip(str_to_ptr(&argv[1]));

    if argv[1] == "--help" {
      /* "busybox --help [<applet>]" */
      if argv.len() < 3 {
        // Missing the applet to ask for help with.
        print_rustybox_help();
        return 0;
      } else {
        /* convert to "<applet> --help" */
        run_applet_and_exit(
          &ptr_to_str(applet_name),
          &[argv[2].clone(), "--help".into()],
        );
      }
    }

    /* "busybox <applet> arg1 arg2 ..." */
    run_applet_and_exit(&ptr_to_str(applet_name), &argv[1..]);
  }
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

pub unsafe fn run_applet_no_and_exit(applet_no: usize, name: &str, argv: &[String]) -> ! {
  let argc = argv.len() as i32;

  /* We do not use argv[0]: do not want to repeat massaging of
   * "-/sbin/halt" -> "halt", for example. */
  applet_name = str_to_ptr(name);

  /* Special case. POSIX says "test --help"
   * should be no different from e.g. "test --foo".
   * Thus for "test", we skip --help check.
   * "true" and "false" are also special.
   */
  // TODO: get rid of these magic numbers.
  if applet_no != 332 && applet_no != 342 && applet_no != 82 {
    if argc == 2 && argv[1] == "--help" {
      /* Make "foo --help" exit with 0: */
      xfunc_error_retval = 0 as uint8_t;
      bb_show_usage();
    }
  }

  check_suid(applet_no as i32);
  xfunc_error_retval =
    applet_main[applet_no].expect("non-null function pointer")(argc, str_vec_to_ptrs(argv))
      as uint8_t;

  /* Note: applet_main() may also not return (die on a xfunc or such) */
  xfunc_die();
}

unsafe fn run_applet_and_exit(name: &str, argv: &[String]) -> ! {
  if name.starts_with("rustybox") {
    ::std::process::exit(rustybox_main(argv));
  } else {
    /* find_applet_by_name() search is more expensive, so goes second */
    match find_applet_by_name(name) {
      None => {
        eprintln!("{}: applet not found", ptr_to_str(applet_name));

        /* POSIX: "If a command is not found, the exit status shall be 127" */
        ::std::process::exit(127);
      }
      Some(applet_no) => run_applet_no_and_exit(applet_no, name, argv),
    }
  }
}

pub unsafe fn main() {
  /* Tweak malloc for reduced memory consumption */
  /* M_TRIM_THRESHOLD is the maximum amount of freed top-most memory
   * to keep before releasing to the OS
   * Default is way too big: 256k
   */
  mallopt(-1i32, 8i32 * 1024i32);

  /* M_MMAP_THRESHOLD is the request size threshold for using mmap()
   * Default is too big: 256k
   */
  mallopt(-3i32, 32i32 * 1024i32 - 256i32);

  let argv: Vec<String> = ::std::env::args().collect();
  applet_name = bb_basename(str_to_ptr(argv[0].trim_start_matches('-')));
  parse_config_file(); /* ...maybe, if FEATURE_SUID_CONFIG */
  run_applet_and_exit(&ptr_to_str(applet_name), &argv)
}

unsafe fn ptr_to_str(strptr: *const libc::c_char) -> String {
  CStr::from_ptr(strptr).to_string_lossy().into_owned()
}

fn str_to_ptr(string: &str) -> *mut libc::c_char {
  CString::new(string.as_bytes())
    .expect("CString::new failed.")
    .into_raw()
}

fn str_vec_to_ptrs(strings: &[String]) -> *mut *mut libc::c_char {
  let mut ret: Vec<*mut libc::c_char> = Vec::new();
  for arg in strings {
    ret.push(str_to_ptr(arg));
  }
  ret.push(::std::ptr::null_mut());

  // This is necessary because otherwise `ret` is dropped prematurely. We need
  // the pointer to remain valid when calling into C code. Probably introduces a
  // small memory leak, but we'll live with it for now.
  let mut nodrop = ::std::mem::ManuallyDrop::new(ret);
  nodrop.as_mut_ptr()
}

unsafe extern "C" fn run_static_initializers() {
  install_dir = [
    &*usr_bin.as_ptr().offset(8) as *const libc::c_char,
    &*usr_bin.as_ptr().offset(4) as *const libc::c_char,
    &*usr_sbin.as_ptr().offset(4) as *const libc::c_char,
    usr_bin.as_ptr(),
    usr_sbin.as_ptr(),
  ]
}

#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
