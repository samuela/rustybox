use libc;





































































extern "C" {
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
  fn test_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ash_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn printf_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn ls_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn kill_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn bunzip2_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn chown_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn echo_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

  #[no_mangle]
  fn hush_main(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

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

  // Defined in appletlib.rs for some reason.
  #[no_mangle]
  fn scripted_main(_argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;

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
}

#[derive(Copy, Clone)]
pub enum InstallLoc {
  DIR_USR_SBIN,
  DIR_USR_BIN,
  DIR_SBIN,
  DIR_BIN,
  DIR_ROOT,
}

#[derive(Copy, Clone, PartialEq)]
pub enum SUID {
  BB_SUID_REQUIRE,
  BB_SUID_MAYBE,
  BB_SUID_DROP,
}

// TODO: it's not clear to me how if at all noexec and nofork are actually used
// in the code. Should they be removed?
#[derive(Clone)]
#[repr(C)]
pub struct bb_applet {
  pub name: &'static str,
  pub main: &'static str,
  pub entrypoint: unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int,
  pub install_loc: InstallLoc,
  pub need_suid: SUID,

  /* true if instead of fork(); exec("applet"); waitpid();
   * one can do fork(); exit(applet_main(argc,argv)); waitpid(); */
  pub noexec: bool,

  /* Even nicer */
  /* true if instead of fork(); exec("applet"); waitpid();
   * one can simply call applet_main(argc,argv); */
  pub nofork: bool,

  pub usage: &'static str,
}

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

// TODO: write a test asserting these are in sorted order by name.
pub static applets: [bb_applet; 396] = [
  bb_applet {
    name: "[",
    main: "test",
    entrypoint: test_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/[")
  },
  bb_applet {
    name: "[[",
    main: "test",
    entrypoint: test_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/[[")
  },
  bb_applet {
    name: "acpid",
    main: "acpid",
    entrypoint: acpid_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/acpid")
  },
  bb_applet {
    name: "add-shell",
    main: "add_remove_shell",
    entrypoint: add_remove_shell_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/add-shell")
  },
  bb_applet {
    name: "addgroup",
    main: "addgroup",
    entrypoint: addgroup_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/addgroup")
  },
  bb_applet {
    name: "adduser",
    main: "adduser",
    entrypoint: adduser_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/adduser")
  },
  bb_applet {
    name: "adjtimex",
    main: "adjtimex",
    entrypoint: adjtimex_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/adjtimex")
  },
  bb_applet {
    name: "arch",
    main: "uname",
    entrypoint: uname_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/arch")
  },
  bb_applet {
    name: "arp",
    main: "arp",
    entrypoint: arp_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/arp")
  },
  bb_applet {
    name: "arping",
    main: "arping",
    entrypoint: arping_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/arping")
  },
  bb_applet {
    name: "ash",
    main: "ash",
    entrypoint: ash_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/ash")
  },
  bb_applet {
    name: "awk",
    main: "awk",
    entrypoint: awk_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/awk")
  },
  bb_applet {
    name: "base64",
    main: "base64",
    entrypoint: base64_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/base64")
  },
  bb_applet {
    name: "basename",
    main: "basename",
    entrypoint: basename_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/basename")
  },
  bb_applet {
    name: "bc",
    main: "bc",
    entrypoint: bc_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/bc")
  },
  bb_applet {
    name: "beep",
    main: "beep",
    entrypoint: beep_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/beep")
  },
  bb_applet {
    name: "blkdiscard",
    main: "blkdiscard",
    entrypoint: blkdiscard_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/blkdiscard")
  },
  bb_applet {
    name: "blkid",
    main: "blkid",
    entrypoint: blkid_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/blkid")
  },
  bb_applet {
    name: "blockdev",
    main: "blockdev",
    entrypoint: blockdev_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/blockdev")
  },
  bb_applet {
    name: "bootchartd",
    main: "bootchartd",
    entrypoint: bootchartd_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/bootchartd")
  },
  bb_applet {
    name: "brctl",
    main: "brctl",
    entrypoint: brctl_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/brctl")
  },
  bb_applet {
    name: "bunzip2",
    main: "bunzip2",
    entrypoint: bunzip2_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/bunzip2")
  },
  bb_applet {
    name: "bzcat",
    main: "bunzip2",
    entrypoint: bunzip2_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/bzcat")
  },
  bb_applet {
    name: "bzip2",
    main: "bzip2",
    entrypoint: bzip2_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/bzip2")
  },
  bb_applet {
    name: "cal",
    main: "cal",
    entrypoint: cal_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/cal")
  },
  bb_applet {
    name: "cat",
    main: "cat",
    entrypoint: cat_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/cat")
  },
  bb_applet {
    name: "chat",
    main: "chat",
    entrypoint: chat_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/chat")
  },
  bb_applet {
    name: "chattr",
    main: "chattr",
    entrypoint: chattr_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/chattr")
  },
  bb_applet {
    name: "chgrp",
    main: "chgrp",
    entrypoint: chgrp_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/chgrp")
  },
  bb_applet {
    name: "chmod",
    main: "chmod",
    entrypoint: chmod_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/chmod")
  },
  bb_applet {
    name: "chown",
    main: "chown",
    entrypoint: chown_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/chown")
  },
  bb_applet {
    name: "chpasswd",
    main: "chpasswd",
    entrypoint: chpasswd_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/chpasswd")
  },
  bb_applet {
    name: "chpst",
    main: "chpst",
    entrypoint: chpst_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/chpst")
  },
  bb_applet {
    name: "chroot",
    main: "chroot",
    entrypoint: chroot_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/chroot")
  },
  bb_applet {
    name: "chrt",
    main: "chrt",
    entrypoint: chrt_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/chrt")
  },
  bb_applet {
    name: "chvt",
    main: "chvt",
    entrypoint: chvt_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/chvt")
  },
  bb_applet {
    name: "cksum",
    main: "cksum",
    entrypoint: cksum_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/cksum")
  },
  bb_applet {
    name: "clear",
    main: "clear",
    entrypoint: clear_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/clear")
  },
  bb_applet {
    name: "cmp",
    main: "cmp",
    entrypoint: cmp_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/cmp")
  },
  bb_applet {
    name: "comm",
    main: "comm",
    entrypoint: comm_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/comm")
  },
  bb_applet {
    name: "conspy",
    main: "conspy",
    entrypoint: conspy_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/conspy")
  },
  bb_applet {
    name: "cp",
    main: "cp",
    entrypoint: cp_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/cp")
  },
  bb_applet {
    name: "cpio",
    main: "cpio",
    entrypoint: cpio_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/cpio")
  },
  bb_applet {
    name: "crond",
    main: "crond",
    entrypoint: crond_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/crond")
  },
  bb_applet {
    name: "crontab",
    main: "crontab",
    entrypoint: crontab_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_REQUIRE,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/crontab")
  },
  bb_applet {
    name: "cryptpw",
    main: "cryptpw",
    entrypoint: cryptpw_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/cryptpw")
  },
  bb_applet {
    name: "cttyhack",
    main: "cttyhack",
    entrypoint: cttyhack_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/cttyhack")
  },
  bb_applet {
    name: "cut",
    main: "cut",
    entrypoint: cut_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/cut")
  },
  bb_applet {
    name: "date",
    main: "date",
    entrypoint: date_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/date")
  },
  bb_applet {
    name: "dc",
    main: "dc",
    entrypoint: dc_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/dc")
  },
  bb_applet {
    name: "dd",
    main: "dd",
    entrypoint: dd_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/dd")
  },
  bb_applet {
    name: "deallocvt",
    main: "deallocvt",
    entrypoint: deallocvt_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/deallocvt")
  },
  bb_applet {
    name: "delgroup",
    main: "deluser",
    entrypoint: deluser_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/delgroup")
  },
  bb_applet {
    name: "deluser",
    main: "deluser",
    entrypoint: deluser_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/deluser")
  },
  bb_applet {
    name: "depmod",
    main: "modprobe",
    entrypoint: modprobe_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/depmod")
  },
  bb_applet {
    name: "devmem",
    main: "devmem",
    entrypoint: devmem_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/devmem")
  },
  bb_applet {
    name: "df",
    main: "df",
    entrypoint: df_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/df")
  },
  bb_applet {
    name: "dhcprelay",
    main: "dhcprelay",
    entrypoint: dhcprelay_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/dhcprelay")
  },
  bb_applet {
    name: "diff",
    main: "diff",
    entrypoint: diff_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/diff")
  },
  bb_applet {
    name: "dirname",
    main: "dirname",
    entrypoint: dirname_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/dirname")
  },
  bb_applet {
    name: "dmesg",
    main: "dmesg",
    entrypoint: dmesg_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/dmesg")
  },
  bb_applet {
    name: "dnsd",
    main: "dnsd",
    entrypoint: dnsd_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/dnsd")
  },
  bb_applet {
    name: "dnsdomainname",
    main: "hostname",
    entrypoint: hostname_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/dnsdomainname")
  },
  bb_applet {
    name: "dos2unix",
    main: "dos2unix",
    entrypoint: dos2unix_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/dos2unix")
  },
  bb_applet {
    name: "dpkg-deb",
    main: "dpkg_deb",
    entrypoint: dpkg_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/dpkg-deb")
  },
  bb_applet {
    name: "dpkg",
    main: "dpkg",
    entrypoint: dpkg_deb_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/dpkg")
  },
  bb_applet {
    name: "du",
    main: "du",
    entrypoint: du_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/du")
  },
  bb_applet {
    name: "dumpkmap",
    main: "dumpkmap",
    entrypoint: dumpkmap_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/dumpkmap")
  },
  bb_applet {
    name: "dumpleases",
    main: "dumpleases",
    entrypoint: dumpleases_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/dumpleases")
  },
  bb_applet {
    name: "echo",
    main: "echo",
    entrypoint: echo_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/echo")
  },
  bb_applet {
    name: "ed",
    main: "ed",
    entrypoint: ed_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/ed")
  },
  bb_applet {
    name: "egrep",
    main: "grep",
    entrypoint: grep_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/egrep")
  },
  bb_applet {
    name: "eject",
    main: "eject",
    entrypoint: eject_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/eject")
  },
  bb_applet {
    name: "env",
    main: "env",
    entrypoint: env_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/env")
  },
  bb_applet {
    name: "envdir",
    main: "chpst",
    entrypoint: chpst_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/envdir")
  },
  bb_applet {
    name: "envuidgid",
    main: "chpst",
    entrypoint: chpst_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/envuidgid")
  },
  bb_applet {
    name: "ether-wake",
    main: "ether_wake",
    entrypoint: ether_wake_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/ether-wake")
  },
  bb_applet {
    name: "expand",
    main: "expand",
    entrypoint: expand_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/expand")
  },
  bb_applet {
    name: "expr",
    main: "expr",
    entrypoint: expr_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/expr")
  },
  bb_applet {
    name: "factor",
    main: "factor",
    entrypoint: factor_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/factor")
  },
  bb_applet {
    name: "fakeidentd",
    main: "fakeidentd",
    entrypoint: fakeidentd_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/fakeidentd")
  },
  bb_applet {
    name: "fallocate",
    main: "fallocate",
    entrypoint: fallocate_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/fallocate")
  },
  bb_applet {
    name: "false",
    main: "false",
    entrypoint: false_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/false")
  },
  bb_applet {
    name: "fatattr",
    main: "fatattr",
    entrypoint: fatattr_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/fatattr")
  },
  bb_applet {
    name: "fbset",
    main: "fbset",
    entrypoint: fbset_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/fbset")
  },
  bb_applet {
    name: "fbsplash",
    main: "fbsplash",
    entrypoint: fbsplash_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/fbsplash")
  },
  bb_applet {
    name: "fdflush",
    main: "freeramdisk",
    entrypoint: freeramdisk_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/fdflush")
  },
  bb_applet {
    name: "fdformat",
    main: "fdformat",
    entrypoint: fdformat_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/fdformat")
  },
  bb_applet {
    name: "fdisk",
    main: "fdisk",
    entrypoint: fdisk_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/fdisk")
  },
  bb_applet {
    name: "fgconsole",
    main: "fgconsole",
    entrypoint: fgconsole_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/fgconsole")
  },
  bb_applet {
    name: "fgrep",
    main: "grep",
    entrypoint: grep_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/fgrep")
  },
  bb_applet {
    name: "find",
    main: "find",
    entrypoint: find_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/find")
  },
  bb_applet {
    name: "findfs",
    main: "findfs",
    entrypoint: findfs_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_MAYBE,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/findfs")
  },
  bb_applet {
    name: "flock",
    main: "flock",
    entrypoint: flock_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/flock")
  },
  bb_applet {
    name: "fold",
    main: "fold",
    entrypoint: fold_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/fold")
  },
  bb_applet {
    name: "free",
    main: "free",
    entrypoint: free_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/free")
  },
  bb_applet {
    name: "freeramdisk",
    main: "freeramdisk",
    entrypoint: freeramdisk_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/freeramdisk")
  },
  bb_applet {
    name: "fsck.minix",
    main: "fsck_minix",
    entrypoint: fsck_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/fsck.minix")
  },
  bb_applet {
    name: "fsck",
    main: "fsck",
    entrypoint: fsck_minix_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/fsck")
  },
  bb_applet {
    name: "fsfreeze",
    main: "fsfreeze",
    entrypoint: fsfreeze_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/fsfreeze")
  },
  bb_applet {
    name: "fstrim",
    main: "fstrim",
    entrypoint: fstrim_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/fstrim")
  },
  bb_applet {
    name: "fsync",
    main: "fsync",
    entrypoint: fsync_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/fsync")
  },
  bb_applet {
    name: "ftpd",
    main: "ftpd",
    entrypoint: ftpd_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/ftpd")
  },
  bb_applet {
    name: "ftpget",
    main: "ftpgetput",
    entrypoint: ftpgetput_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/ftpget")
  },
  bb_applet {
    name: "ftpput",
    main: "ftpgetput",
    entrypoint: ftpgetput_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/ftpput")
  },
  bb_applet {
    name: "fuser",
    main: "fuser",
    entrypoint: fuser_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/fuser")
  },
  bb_applet {
    name: "getopt",
    main: "getopt",
    entrypoint: getopt_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/getopt")
  },
  bb_applet {
    name: "getty",
    main: "getty",
    entrypoint: getty_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/getty")
  },
  bb_applet {
    name: "grep",
    main: "grep",
    entrypoint: grep_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/grep")
  },
  bb_applet {
    name: "groups",
    main: "id",
    entrypoint: id_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/groups")
  },
  bb_applet {
    name: "gunzip",
    main: "gunzip",
    entrypoint: gunzip_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/gunzip")
  },
  bb_applet {
    name: "gzip",
    main: "gzip",
    entrypoint: gzip_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/gzip")
  },
  bb_applet {
    name: "halt",
    main: "halt",
    entrypoint: halt_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/halt")
  },
  bb_applet {
    name: "hd",
    main: "hexdump",
    entrypoint: hexdump_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/hd")
  },
  bb_applet {
    name: "hdparm",
    main: "hdparm",
    entrypoint: hdparm_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/hdparm")
  },
  bb_applet {
    name: "head",
    main: "head",
    entrypoint: head_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/head")
  },
  bb_applet {
    name: "hexdump",
    main: "hexdump",
    entrypoint: hexdump_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/hexdump")
  },
  bb_applet {
    name: "hexedit",
    main: "hexedit",
    entrypoint: hexedit_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/hexedit")
  },
  bb_applet {
    name: "hostid",
    main: "hostid",
    entrypoint: hostid_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/hostid")
  },
  bb_applet {
    name: "hostname",
    main: "hostname",
    entrypoint: hostname_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/hostname")
  },
  bb_applet {
    name: "httpd",
    main: "httpd",
    entrypoint: httpd_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/httpd")
  },
  bb_applet {
    name: "hush",
    main: "hush",
    entrypoint: hush_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/hush")
  },
  bb_applet {
    name: "hwclock",
    main: "hwclock",
    entrypoint: hwclock_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/hwclock")
  },
  bb_applet {
    name: "i2cdetect",
    main: "i2cdetect",
    entrypoint: i2cdetect_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/i2cdetect")
  },
  bb_applet {
    name: "i2cdump",
    main: "i2cdump",
    entrypoint: i2cdump_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/i2cdump")
  },
  bb_applet {
    name: "i2cget",
    main: "i2cget",
    entrypoint: i2cget_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/i2cget")
  },
  bb_applet {
    name: "i2cset",
    main: "i2cset",
    entrypoint: i2cset_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/i2cset")
  },
  bb_applet {
    name: "i2ctransfer",
    main: "i2ctransfer",
    entrypoint: i2ctransfer_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/i2ctransfer")
  },
  bb_applet {
    name: "id",
    main: "id",
    entrypoint: id_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/id")
  },
  bb_applet {
    name: "ifconfig",
    main: "ifconfig",
    entrypoint: ifconfig_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/ifconfig")
  },
  bb_applet {
    name: "ifdown",
    main: "ifupdown",
    entrypoint: ifupdown_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/ifdown")
  },
  bb_applet {
    name: "ifenslave",
    main: "ifenslave",
    entrypoint: ifenslave_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/ifenslave")
  },
  bb_applet {
    name: "ifplugd",
    main: "ifplugd",
    entrypoint: ifplugd_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/ifplugd")
  },
  bb_applet {
    name: "ifup",
    main: "ifupdown",
    entrypoint: ifupdown_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/ifup")
  },
  bb_applet {
    name: "inetd",
    main: "inetd",
    entrypoint: inetd_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/inetd")
  },
  bb_applet {
    name: "init",
    main: "init",
    entrypoint: init_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/init")
  },
  bb_applet {
    name: "insmod",
    main: "modprobe",
    entrypoint: modprobe_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/insmod")
  },
  bb_applet {
    name: "install",
    main: "install",
    entrypoint: install_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/install")
  },
  bb_applet {
    name: "ionice",
    main: "ionice",
    entrypoint: ionice_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/ionice")
  },
  bb_applet {
    name: "iostat",
    main: "iostat",
    entrypoint: iostat_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/iostat")
  },
  bb_applet {
    name: "ip",
    main: "ip",
    entrypoint: ip_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/ip")
  },
  bb_applet {
    name: "ipaddr",
    main: "ipaddr",
    entrypoint: ipaddr_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/ipaddr")
  },
  bb_applet {
    name: "ipcalc",
    main: "ipcalc",
    entrypoint: ipcalc_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/ipcalc")
  },
  bb_applet {
    name: "ipcrm",
    main: "ipcrm",
    entrypoint: ipcrm_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/ipcrm")
  },
  bb_applet {
    name: "ipcs",
    main: "ipcs",
    entrypoint: ipcs_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/ipcs")
  },
  bb_applet {
    name: "iplink",
    main: "iplink",
    entrypoint: iplink_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/iplink")
  },
  bb_applet {
    name: "ipneigh",
    main: "ipneigh",
    entrypoint: ipneigh_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/ipneigh")
  },
  bb_applet {
    name: "iproute",
    main: "iproute",
    entrypoint: iproute_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/iproute")
  },
  bb_applet {
    name: "iprule",
    main: "iprule",
    entrypoint: iprule_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/iprule")
  },
  bb_applet {
    name: "iptunnel",
    main: "iptunnel",
    entrypoint: iptunnel_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/iptunnel")
  },
  bb_applet {
    name: "kbd_mode",
    main: "kbd_mode",
    entrypoint: kbd_mode_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/kbd_mode")
  },
  bb_applet {
    name: "kill",
    main: "kill",
    entrypoint: kill_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/kill")
  },
  bb_applet {
    name: "killall",
    main: "kill",
    entrypoint: kill_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/killall")
  },
  bb_applet {
    name: "killall5",
    main: "kill",
    entrypoint: kill_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/killall5")
  },
  bb_applet {
    name: "klogd",
    main: "klogd",
    entrypoint: klogd_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/klogd")
  },
  bb_applet {
    name: "last",
    main: "last",
    entrypoint: last_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/last")
  },
  bb_applet {
    name: "less",
    main: "less",
    entrypoint: less_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/less")
  },
  bb_applet {
    name: "link",
    main: "link",
    entrypoint: link_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/link")
  },
  bb_applet {
    name: "linux32",
    main: "setarch",
    entrypoint: setarch_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/linux32")
  },
  bb_applet {
    name: "linux64",
    main: "setarch",
    entrypoint: setarch_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/linux64")
  },
  bb_applet {
    name: "linuxrc",
    main: "init",
    entrypoint: init_main,
    install_loc: InstallLoc::DIR_ROOT,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/linuxrc")
  },
  bb_applet {
    name: "ln",
    main: "ln",
    entrypoint: ln_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/ln")
  },
  bb_applet {
    name: "loadfont",
    main: "loadfont",
    entrypoint: loadfont_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/loadfont")
  },
  bb_applet {
    name: "loadkmap",
    main: "loadkmap",
    entrypoint: loadkmap_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/loadkmap")
  },
  bb_applet {
    name: "logger",
    main: "logger",
    entrypoint: logger_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/logger")
  },
  bb_applet {
    name: "login",
    main: "login",
    entrypoint: login_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_REQUIRE,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/login")
  },
  bb_applet {
    name: "logname",
    main: "logname",
    entrypoint: logname_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/logname")
  },
  bb_applet {
    name: "logread",
    main: "logread",
    entrypoint: logread_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/logread")
  },
  bb_applet {
    name: "losetup",
    main: "losetup",
    entrypoint: losetup_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/losetup")
  },
  bb_applet {
    name: "lpd",
    main: "lpd",
    entrypoint: lpd_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/lpd")
  },
  bb_applet {
    name: "lpq",
    main: "lpqr",
    entrypoint: lpqr_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/lpq")
  },
  bb_applet {
    name: "lpr",
    main: "lpqr",
    entrypoint: lpqr_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/lpr")
  },
  bb_applet {
    name: "ls",
    main: "ls",
    entrypoint: ls_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/ls")
  },
  bb_applet {
    name: "lsattr",
    main: "lsattr",
    entrypoint: lsattr_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/lsattr")
  },
  bb_applet {
    name: "lsmod",
    main: "lsmod",
    entrypoint: lsmod_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/lsmod")
  },
  bb_applet {
    name: "lsof",
    main: "lsof",
    entrypoint: lsof_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/lsof")
  },
  bb_applet {
    name: "lspci",
    main: "lspci",
    entrypoint: lspci_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/lspci")
  },
  bb_applet {
    name: "lsscsi",
    main: "lsscsi",
    entrypoint: lsscsi_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/lsscsi")
  },
  bb_applet {
    name: "lsusb",
    main: "lsusb",
    entrypoint: lsusb_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/lsusb")
  },
  bb_applet {
    name: "lzcat",
    main: "unlzma",
    entrypoint: unlzma_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/lzcat")
  },
  bb_applet {
    name: "lzma",
    main: "unlzma",
    entrypoint: unlzma_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/lzma")
  },
  bb_applet {
    name: "lzop",
    main: "lzop",
    entrypoint: lzop_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/lzop")
  },
  bb_applet {
    name: "makedevs",
    main: "makedevs",
    entrypoint: makedevs_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/makedevs")
  },
  bb_applet {
    name: "makemime",
    main: "makemime",
    entrypoint: makemime_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/makemime")
  },
  bb_applet {
    name: "man",
    main: "man",
    entrypoint: man_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/man")
  },
  bb_applet {
    name: "md5sum",
    main: "md5_sha1_sum",
    entrypoint: md5_sha1_sum_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/md5sum")
  },
  bb_applet {
    name: "mdev",
    main: "mdev",
    entrypoint: mdev_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/mdev")
  },
  bb_applet {
    name: "mesg",
    main: "mesg",
    entrypoint: mesg_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/mesg")
  },
  bb_applet {
    name: "microcom",
    main: "microcom",
    entrypoint: microcom_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/microcom")
  },
  bb_applet {
    name: "mkdir",
    main: "mkdir",
    entrypoint: mkdir_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/mkdir")
  },
  bb_applet {
    name: "mkdosfs",
    main: "mkfs_vfat",
    entrypoint: mkfs_vfat_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/mkdosfs")
  },
  bb_applet {
    name: "mke2fs",
    main: "mkfs_ext2",
    entrypoint: mkfs_ext2_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/mke2fs")
  },
  bb_applet {
    name: "mkfifo",
    main: "mkfifo",
    entrypoint: mkfifo_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/mkfifo")
  },
  bb_applet {
    name: "mkfs.ext2",
    main: "mkfs_ext2",
    entrypoint: mkfs_ext2_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/mkfs.ext2")
  },
  bb_applet {
    name: "mkfs.minix",
    main: "mkfs_minix",
    entrypoint: mkfs_minix_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/mkfs.minix")
  },
  bb_applet {
    name: "mkfs.vfat",
    main: "mkfs_vfat",
    entrypoint: mkfs_vfat_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/mkfs.vfat")
  },
  bb_applet {
    name: "mknod",
    main: "mknod",
    entrypoint: mknod_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/mknod")
  },
  bb_applet {
    name: "mkpasswd",
    main: "cryptpw",
    entrypoint: cryptpw_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/mkpasswd")
  },
  bb_applet {
    name: "mkswap",
    main: "mkswap",
    entrypoint: mkswap_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/mkswap")
  },
  bb_applet {
    name: "mktemp",
    main: "mktemp",
    entrypoint: mktemp_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/mktemp")
  },
  bb_applet {
    name: "modinfo",
    main: "modinfo",
    entrypoint: modinfo_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/modinfo")
  },
  bb_applet {
    name: "modprobe",
    main: "modprobe",
    entrypoint: modprobe_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/modprobe")
  },
  bb_applet {
    name: "more",
    main: "more",
    entrypoint: more_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/more")
  },
  bb_applet {
    name: "mount",
    main: "mount",
    entrypoint: mount_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_MAYBE,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/mount")
  },
  bb_applet {
    name: "mountpoint",
    main: "mountpoint",
    entrypoint: mountpoint_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/mountpoint")
  },
  bb_applet {
    name: "mpstat",
    main: "mpstat",
    entrypoint: mpstat_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/mpstat")
  },
  bb_applet {
    name: "mt",
    main: "mt",
    entrypoint: mt_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/mt")
  },
  bb_applet {
    name: "mv",
    main: "mv",
    entrypoint: mv_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/mv")
  },
  bb_applet {
    name: "nameif",
    main: "nameif",
    entrypoint: nameif_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/nameif")
  },
  bb_applet {
    name: "nanddump",
    main: "nandwrite",
    entrypoint: nandwrite_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/nanddump")
  },
  bb_applet {
    name: "nandwrite",
    main: "nandwrite",
    entrypoint: nandwrite_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/nandwrite")
  },
  bb_applet {
    name: "nbd-client",
    main: "nbdclient",
    entrypoint: nbdclient_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/nbd-client")
  },
  bb_applet {
    name: "nc",
    main: "nc",
    entrypoint: nc_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/nc")
  },
  bb_applet {
    name: "netstat",
    main: "netstat",
    entrypoint: netstat_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/netstat")
  },
  bb_applet {
    name: "nice",
    main: "nice",
    entrypoint: nice_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/nice")
  },
  bb_applet {
    name: "nl",
    main: "nl",
    entrypoint: nl_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/nl")
  },
  bb_applet {
    name: "nmeter",
    main: "nmeter",
    entrypoint: nmeter_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/nmeter")
  },
  bb_applet {
    name: "nohup",
    main: "nohup",
    entrypoint: nohup_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/nohup")
  },
  bb_applet {
    name: "nologin",
    main: "scripted",
    entrypoint: scripted_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/nologin")
  },
  bb_applet {
    name: "nproc",
    main: "nproc",
    entrypoint: nproc_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/nproc")
  },
  bb_applet {
    name: "nsenter",
    main: "nsenter",
    entrypoint: nsenter_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/nsenter")
  },
  bb_applet {
    name: "nslookup",
    main: "nslookup",
    entrypoint: nslookup_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/nslookup")
  },
  bb_applet {
    name: "ntpd",
    main: "ntpd",
    entrypoint: ntpd_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/ntpd")
  },
  bb_applet {
    name: "nuke",
    main: "nuke",
    entrypoint: nuke_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/nuke")
  },
  bb_applet {
    name: "od",
    main: "od",
    entrypoint: od_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/od")
  },
  bb_applet {
    name: "openvt",
    main: "openvt",
    entrypoint: openvt_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/openvt")
  },
  bb_applet {
    name: "partprobe",
    main: "partprobe",
    entrypoint: partprobe_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/partprobe")
  },
  bb_applet {
    name: "passwd",
    main: "passwd",
    entrypoint: passwd_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_REQUIRE,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/passwd")
  },
  bb_applet {
    name: "paste",
    main: "paste",
    entrypoint: paste_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/paste")
  },
  bb_applet {
    name: "patch",
    main: "patch",
    entrypoint: patch_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/patch")
  },
  bb_applet {
    name: "pgrep",
    main: "pgrep",
    entrypoint: pgrep_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/pgrep")
  },
  bb_applet {
    name: "pidof",
    main: "pidof",
    entrypoint: pidof_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/pidof")
  },
  bb_applet {
    name: "ping",
    main: "ping",
    entrypoint: ping_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_MAYBE,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/ping")
  },
  bb_applet {
    name: "ping6",
    main: "ping6",
    entrypoint: ping6_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_MAYBE,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/ping6")
  },
  bb_applet {
    name: "pipe_progress",
    main: "pipe_progress",
    entrypoint: pipe_progress_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/pipe_progress")
  },
  bb_applet {
    name: "pivot_root",
    main: "pivot_root",
    entrypoint: pivot_root_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/pivot_root")
  },
  bb_applet {
    name: "pkill",
    main: "pgrep",
    entrypoint: pgrep_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/pkill")
  },
  bb_applet {
    name: "pmap",
    main: "pmap",
    entrypoint: pmap_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/pmap")
  },
  bb_applet {
    name: "popmaildir",
    main: "popmaildir",
    entrypoint: popmaildir_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/popmaildir")
  },
  bb_applet {
    name: "poweroff",
    main: "halt",
    entrypoint: halt_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/poweroff")
  },
  bb_applet {
    name: "powertop",
    main: "powertop",
    entrypoint: powertop_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/powertop")
  },
  bb_applet {
    name: "printenv",
    main: "printenv",
    entrypoint: printenv_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/printenv")
  },
  bb_applet {
    name: "printf",
    main: "printf",
    entrypoint: printf_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/printf")
  },
  bb_applet {
    name: "ps",
    main: "ps",
    entrypoint: ps_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/ps")
  },
  bb_applet {
    name: "pscan",
    main: "pscan",
    entrypoint: pscan_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/pscan")
  },
  bb_applet {
    name: "pstree",
    main: "pstree",
    entrypoint: pstree_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/pstree")
  },
  bb_applet {
    name: "pwd",
    main: "pwd",
    entrypoint: pwd_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/pwd")
  },
  bb_applet {
    name: "pwdx",
    main: "pwdx",
    entrypoint: pwdx_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/pwdx")
  },
  bb_applet {
    name: "raidautorun",
    main: "raidautorun",
    entrypoint: raidautorun_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/raidautorun")
  },
  bb_applet {
    name: "rdate",
    main: "rdate",
    entrypoint: rdate_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/rdate")
  },
  bb_applet {
    name: "rdev",
    main: "rdev",
    entrypoint: rdev_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/rdev")
  },
  bb_applet {
    name: "readahead",
    main: "readahead",
    entrypoint: readahead_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/readahead")
  },
  bb_applet {
    name: "readlink",
    main: "readlink",
    entrypoint: readlink_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/readlink")
  },
  bb_applet {
    name: "readprofile",
    main: "readprofile",
    entrypoint: readprofile_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/readprofile")
  },
  bb_applet {
    name: "realpath",
    main: "realpath",
    entrypoint: realpath_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/realpath")
  },
  bb_applet {
    name: "reboot",
    main: "halt",
    entrypoint: halt_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/reboot")
  },
  bb_applet {
    name: "reformime",
    main: "reformime",
    entrypoint: reformime_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/reformime")
  },
  bb_applet {
    name: "remove-shell",
    main: "add_remove_shell",
    entrypoint: add_remove_shell_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/remove-shell")
  },
  bb_applet {
    name: "renice",
    main: "renice",
    entrypoint: renice_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/renice")
  },
  bb_applet {
    name: "reset",
    main: "reset",
    entrypoint: reset_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/reset")
  },
  bb_applet {
    name: "resize",
    main: "resize",
    entrypoint: resize_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/resize")
  },
  bb_applet {
    name: "resume",
    main: "resume",
    entrypoint: resume_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/resume")
  },
  bb_applet {
    name: "rev",
    main: "rev",
    entrypoint: rev_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/rev")
  },
  bb_applet {
    name: "rm",
    main: "rm",
    entrypoint: rm_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/rm")
  },
  bb_applet {
    name: "rmdir",
    main: "rmdir",
    entrypoint: rmdir_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/rmdir")
  },
  bb_applet {
    name: "rmmod",
    main: "modprobe",
    entrypoint: modprobe_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/rmmod")
  },
  bb_applet {
    name: "route",
    main: "route",
    entrypoint: route_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/route")
  },
  bb_applet {
    name: "rpm",
    main: "rpm",
    entrypoint: rpm_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/rpm")
  },
  bb_applet {
    name: "rpm2cpio",
    main: "rpm2cpio",
    entrypoint: rpm2cpio_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/rpm2cpio")
  },
  bb_applet {
    name: "rtcwake",
    main: "rtcwake",
    entrypoint: rtcwake_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/rtcwake")
  },
  bb_applet {
    name: "run-init",
    main: "switch_root",
    entrypoint: switch_root_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/run-init")
  },
  bb_applet {
    name: "run-parts",
    main: "run_parts",
    entrypoint: run_parts_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/run-parts")
  },
  bb_applet {
    name: "runlevel",
    main: "runlevel",
    entrypoint: runlevel_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/runlevel")
  },
  bb_applet {
    name: "runsv",
    main: "runsv",
    entrypoint: runsv_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/runsv")
  },
  bb_applet {
    name: "runsvdir",
    main: "runsvdir",
    entrypoint: runsvdir_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/runsvdir")
  },
  bb_applet {
    name: "rx",
    main: "rx",
    entrypoint: rx_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/rx")
  },
  bb_applet {
    name: "script",
    main: "script",
    entrypoint: script_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/script")
  },
  bb_applet {
    name: "scriptreplay",
    main: "scriptreplay",
    entrypoint: scriptreplay_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/scriptreplay")
  },
  bb_applet {
    name: "sed",
    main: "sed",
    entrypoint: sed_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/sed")
  },
  bb_applet {
    name: "sendmail",
    main: "sendmail",
    entrypoint: sendmail_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/sendmail")
  },
  bb_applet {
    name: "seq",
    main: "seq",
    entrypoint: seq_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/seq")
  },
  bb_applet {
    name: "setarch",
    main: "setarch",
    entrypoint: setarch_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/setarch")
  },
  bb_applet {
    name: "setconsole",
    main: "setconsole",
    entrypoint: setconsole_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/setconsole")
  },
  bb_applet {
    name: "setfattr",
    main: "setfattr",
    entrypoint: setfattr_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/setfattr")
  },
  bb_applet {
    name: "setfont",
    main: "setfont",
    entrypoint: setfont_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/setfont")
  },
  bb_applet {
    name: "setkeycodes",
    main: "setkeycodes",
    entrypoint: setkeycodes_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/setkeycodes")
  },
  bb_applet {
    name: "setlogcons",
    main: "setlogcons",
    entrypoint: setlogcons_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/setlogcons")
  },
  bb_applet {
    name: "setpriv",
    main: "setpriv",
    entrypoint: setpriv_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/setpriv")
  },
  bb_applet {
    name: "setserial",
    main: "setserial",
    entrypoint: setserial_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/setserial")
  },
  bb_applet {
    name: "setsid",
    main: "setsid",
    entrypoint: setsid_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/setsid")
  },
  bb_applet {
    name: "setuidgid",
    main: "chpst",
    entrypoint: chpst_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/setuidgid")
  },
  bb_applet {
    name: "sh",
    main: "ash",
    entrypoint: ash_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/sh")
  },
  bb_applet {
    name: "sha1sum",
    main: "md5_sha1_sum",
    entrypoint: md5_sha1_sum_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/sha1sum")
  },
  bb_applet {
    name: "sha256sum",
    main: "md5_sha1_sum",
    entrypoint: md5_sha1_sum_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/sha256sum")
  },
  bb_applet {
    name: "sha3sum",
    main: "md5_sha1_sum",
    entrypoint: md5_sha1_sum_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/sha3sum")
  },
  bb_applet {
    name: "sha512sum",
    main: "md5_sha1_sum",
    entrypoint: md5_sha1_sum_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/sha512sum")
  },
  bb_applet {
    name: "showkey",
    main: "showkey",
    entrypoint: showkey_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/showkey")
  },
  bb_applet {
    name: "shred",
    main: "shred",
    entrypoint: shred_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/shred")
  },
  bb_applet {
    name: "shuf",
    main: "shuf",
    entrypoint: shuf_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/shuf")
  },
  bb_applet {
    name: "slattach",
    main: "slattach",
    entrypoint: slattach_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/slattach")
  },
  bb_applet {
    name: "sleep",
    main: "sleep",
    entrypoint: sleep_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/sleep")
  },
  bb_applet {
    name: "smemcap",
    main: "smemcap",
    entrypoint: smemcap_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/smemcap")
  },
  bb_applet {
    name: "softlimit",
    main: "chpst",
    entrypoint: chpst_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/softlimit")
  },
  bb_applet {
    name: "sort",
    main: "sort",
    entrypoint: sort_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/sort")
  },
  bb_applet {
    name: "split",
    main: "split",
    entrypoint: split_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/split")
  },
  bb_applet {
    name: "ssl_client",
    main: "ssl_client",
    entrypoint: ssl_client_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/ssl_client")
  },
  bb_applet {
    name: "start-stop-daemon",
    main: "start_stop_daemon",
    entrypoint: start_stop_daemon_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/start-stop-daemon")
  },
  bb_applet {
    name: "stat",
    main: "stat",
    entrypoint: stat_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/stat")
  },
  bb_applet {
    name: "strings",
    main: "strings",
    entrypoint: strings_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/strings")
  },
  bb_applet {
    name: "stty",
    main: "stty",
    entrypoint: stty_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/stty")
  },
  bb_applet {
    name: "su",
    main: "su",
    entrypoint: su_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_REQUIRE,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/su")
  },
  bb_applet {
    name: "sulogin",
    main: "sulogin",
    entrypoint: sulogin_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/sulogin")
  },
  bb_applet {
    name: "sum",
    main: "sum",
    entrypoint: sum_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/sum")
  },
  bb_applet {
    name: "sv",
    main: "sv",
    entrypoint: sv_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/sv")
  },
  bb_applet {
    name: "svc",
    main: "svc",
    entrypoint: svc_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/svc")
  },
  bb_applet {
    name: "svlogd",
    main: "svlogd",
    entrypoint: svlogd_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/svlogd")
  },
  bb_applet {
    name: "svok",
    main: "svok",
    entrypoint: svok_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/svok")
  },
  bb_applet {
    name: "swapoff",
    main: "swap_on_off",
    entrypoint: swap_on_off_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/swapoff")
  },
  bb_applet {
    name: "swapon",
    main: "swap_on_off",
    entrypoint: swap_on_off_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/swapon")
  },
  bb_applet {
    name: "switch_root",
    main: "switch_root",
    entrypoint: switch_root_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/switch_root")
  },
  bb_applet {
    name: "sync",
    main: "sync",
    entrypoint: sync_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/sync")
  },
  bb_applet {
    name: "sysctl",
    main: "sysctl",
    entrypoint: sysctl_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/sysctl")
  },
  bb_applet {
    name: "syslogd",
    main: "syslogd",
    entrypoint: syslogd_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/syslogd")
  },
  bb_applet {
    name: "tac",
    main: "tac",
    entrypoint: tac_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/tac")
  },
  bb_applet {
    name: "tail",
    main: "tail",
    entrypoint: tail_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/tail")
  },
  bb_applet {
    name: "tar",
    main: "tar",
    entrypoint: tar_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/tar")
  },
  bb_applet {
    name: "taskset",
    main: "taskset",
    entrypoint: taskset_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/taskset")
  },
  bb_applet {
    name: "tc",
    main: "tc",
    entrypoint: tc_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/tc")
  },
  bb_applet {
    name: "tcpsvd",
    main: "tcpudpsvd",
    entrypoint: tcpudpsvd_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/tcpsvd")
  },
  bb_applet {
    name: "tee",
    main: "tee",
    entrypoint: tee_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/tee")
  },
  bb_applet {
    name: "telnet",
    main: "telnet",
    entrypoint: telnet_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/telnet")
  },
  bb_applet {
    name: "telnetd",
    main: "telnetd",
    entrypoint: telnetd_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/telnetd")
  },
  bb_applet {
    name: "test",
    main: "test",
    entrypoint: test_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/test")
  },
  bb_applet {
    name: "tftp",
    main: "tftp",
    entrypoint: tftp_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/tftp")
  },
  bb_applet {
    name: "tftpd",
    main: "tftpd",
    entrypoint: tftpd_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/tftpd")
  },
  bb_applet {
    name: "time",
    main: "time",
    entrypoint: time_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/time")
  },
  bb_applet {
    name: "timeout",
    main: "timeout",
    entrypoint: timeout_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/timeout")
  },
  bb_applet {
    name: "top",
    main: "top",
    entrypoint: top_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/top")
  },
  bb_applet {
    name: "touch",
    main: "touch",
    entrypoint: touch_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/touch")
  },
  bb_applet {
    name: "tr",
    main: "tr",
    entrypoint: tr_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/tr")
  },
  bb_applet {
    name: "traceroute",
    main: "traceroute",
    entrypoint: traceroute_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_MAYBE,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/traceroute")
  },
  bb_applet {
    name: "traceroute6",
    main: "traceroute6",
    entrypoint: traceroute6_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_MAYBE,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/traceroute6")
  },
  bb_applet {
    name: "true",
    main: "true",
    entrypoint: true_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/true")
  },
  bb_applet {
    name: "truncate",
    main: "truncate",
    entrypoint: truncate_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/truncate")
  },
  bb_applet {
    name: "ts",
    main: "ts",
    entrypoint: ts_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/ts")
  },
  bb_applet {
    name: "tty",
    main: "tty",
    entrypoint: tty_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/tty")
  },
  bb_applet {
    name: "ttysize",
    main: "ttysize",
    entrypoint: ttysize_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/ttysize")
  },
  bb_applet {
    name: "tunctl",
    main: "tunctl",
    entrypoint: tunctl_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/tunctl")
  },
  bb_applet {
    name: "ubiattach",
    main: "ubi_tools",
    entrypoint: ubi_tools_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/ubiattach")
  },
  bb_applet {
    name: "ubidetach",
    main: "ubi_tools",
    entrypoint: ubi_tools_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/ubidetach")
  },
  bb_applet {
    name: "ubimkvol",
    main: "ubi_tools",
    entrypoint: ubi_tools_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/ubimkvol")
  },
  bb_applet {
    name: "ubirename",
    main: "ubirename",
    entrypoint: ubirename_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/ubirename")
  },
  bb_applet {
    name: "ubirmvol",
    main: "ubi_tools",
    entrypoint: ubi_tools_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/ubirmvol")
  },
  bb_applet {
    name: "ubirsvol",
    main: "ubi_tools",
    entrypoint: ubi_tools_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/ubirsvol")
  },
  bb_applet {
    name: "ubiupdatevol",
    main: "ubi_tools",
    entrypoint: ubi_tools_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/ubiupdatevol")
  },
  bb_applet {
    name: "udhcpc",
    main: "udhcpc",
    entrypoint: udhcpc_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/udhcpc")
  },
  bb_applet {
    name: "udhcpc6",
    main: "udhcpc6",
    entrypoint: udhcpc6_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/udhcpc6")
  },
  bb_applet {
    name: "udhcpd",
    main: "udhcpd",
    entrypoint: udhcpd_main,
    install_loc: InstallLoc::DIR_USR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/udhcpd")
  },
  bb_applet {
    name: "udpsvd",
    main: "tcpudpsvd",
    entrypoint: tcpudpsvd_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/udpsvd")
  },
  bb_applet {
    name: "uevent",
    main: "uevent",
    entrypoint: uevent_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/uevent")
  },
  bb_applet {
    name: "umount",
    main: "umount",
    entrypoint: umount_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/umount")
  },
  bb_applet {
    name: "uname",
    main: "uname",
    entrypoint: uname_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/uname")
  },
  bb_applet {
    name: "unexpand",
    main: "expand",
    entrypoint: expand_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/unexpand")
  },
  bb_applet {
    name: "uniq",
    main: "uniq",
    entrypoint: uniq_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/uniq")
  },
  bb_applet {
    name: "unix2dos",
    main: "dos2unix",
    entrypoint: dos2unix_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/unix2dos")
  },
  bb_applet {
    name: "unlink",
    main: "unlink",
    entrypoint: unlink_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/unlink")
  },
  bb_applet {
    name: "unlzma",
    main: "unlzma",
    entrypoint: unlzma_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/unlzma")
  },
  bb_applet {
    name: "unshare",
    main: "unshare",
    entrypoint: unshare_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/unshare")
  },
  bb_applet {
    name: "unxz",
    main: "unxz",
    entrypoint: unxz_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/unxz")
  },
  bb_applet {
    name: "unzip",
    main: "unzip",
    entrypoint: unzip_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/unzip")
  },
  bb_applet {
    name: "uptime",
    main: "uptime",
    entrypoint: uptime_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/uptime")
  },
  bb_applet {
    name: "users",
    main: "who",
    entrypoint: who_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/users")
  },
  bb_applet {
    name: "usleep",
    main: "usleep",
    entrypoint: usleep_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/usleep")
  },
  bb_applet {
    name: "uudecode",
    main: "uudecode",
    entrypoint: uudecode_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/uudecode")
  },
  bb_applet {
    name: "uuencode",
    main: "uuencode",
    entrypoint: uuencode_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/uuencode")
  },
  bb_applet {
    name: "vconfig",
    main: "vconfig",
    entrypoint: vconfig_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/vconfig")
  },
  bb_applet {
    name: "vi",
    main: "vi",
    entrypoint: vi_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/vi")
  },
  bb_applet {
    name: "vlock",
    main: "vlock",
    entrypoint: vlock_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_REQUIRE,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/vlock")
  },
  bb_applet {
    name: "volname",
    main: "volname",
    entrypoint: volname_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/volname")
  },
  bb_applet {
    name: "w",
    main: "who",
    entrypoint: who_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/w")
  },
  bb_applet {
    name: "wall",
    main: "wall",
    entrypoint: wall_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_REQUIRE,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/wall")
  },
  bb_applet {
    name: "watch",
    main: "watch",
    entrypoint: watch_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/watch")
  },
  bb_applet {
    name: "watchdog",
    main: "watchdog",
    entrypoint: watchdog_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/watchdog")
  },
  bb_applet {
    name: "wc",
    main: "wc",
    entrypoint: wc_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/wc")
  },
  bb_applet {
    name: "wget",
    main: "wget",
    entrypoint: wget_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/wget")
  },
  bb_applet {
    name: "which",
    main: "which",
    entrypoint: which_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/which")
  },
  bb_applet {
    name: "who",
    main: "who",
    entrypoint: who_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/who")
  },
  bb_applet {
    name: "whoami",
    main: "whoami",
    entrypoint: whoami_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: true,
    usage: std::include_str!("../usage/whoami")
  },
  bb_applet {
    name: "whois",
    main: "whois",
    entrypoint: whois_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/whois")
  },
  bb_applet {
    name: "xargs",
    main: "xargs",
    entrypoint: xargs_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/xargs")
  },
  bb_applet {
    name: "xxd",
    main: "xxd",
    entrypoint: xxd_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/xxd")
  },
  bb_applet {
    name: "xz",
    main: "unxz",
    entrypoint: unxz_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/xz")
  },
  bb_applet {
    name: "xzcat",
    main: "unxz",
    entrypoint: unxz_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/xzcat")
  },
  bb_applet {
    name: "yes",
    main: "yes",
    entrypoint: yes_main,
    install_loc: InstallLoc::DIR_USR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: true,
    nofork: false,
    usage: std::include_str!("../usage/yes")
  },
  bb_applet {
    name: "zcat",
    main: "gunzip",
    entrypoint: gunzip_main,
    install_loc: InstallLoc::DIR_BIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/zcat")
  },
  bb_applet {
    name: "zcip",
    main: "zcip",
    entrypoint: zcip_main,
    install_loc: InstallLoc::DIR_SBIN,
    need_suid: SUID::BB_SUID_DROP,
    noexec: false,
    nofork: false,
    usage: std::include_str!("../usage/zcip")
  },
];
