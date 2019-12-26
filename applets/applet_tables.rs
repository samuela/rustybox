use crate::archival::bbunzip::bunzip2_main;
use crate::archival::bbunzip::gunzip_main;
use crate::archival::bbunzip::unlzma_main;
use crate::archival::bbunzip::unxz_main;
use crate::archival::bzip2::bzip2_main;
use crate::archival::cpio::cpio_main;
use crate::archival::dpkg::dpkg_main;
use crate::archival::dpkg_deb::dpkg_deb_main;
use crate::archival::gzip::gzip_main;
use crate::archival::lzop::lzop_main;
use crate::archival::rpm::rpm2cpio_main;
use crate::archival::rpm::rpm_main;
use crate::archival::tar::tar_main;
use crate::archival::unzip::unzip_main;
use crate::console_tools::chvt::chvt_main;
use crate::console_tools::clear::clear_main;
use crate::console_tools::deallocvt::deallocvt_main;
use crate::console_tools::dumpkmap::dumpkmap_main;
use crate::console_tools::fgconsole::fgconsole_main;
use crate::console_tools::kbd_mode::kbd_mode_main;
use crate::console_tools::loadfont::loadfont_main;
use crate::console_tools::loadfont::setfont_main;
use crate::console_tools::loadkmap::loadkmap_main;
use crate::console_tools::openvt::openvt_main;
use crate::console_tools::reset::reset_main;
use crate::console_tools::resize::resize_main;
use crate::console_tools::setconsole::setconsole_main;
use crate::console_tools::setkeycodes::setkeycodes_main;
use crate::console_tools::setlogcons::setlogcons_main;
use crate::console_tools::showkey::showkey_main;
use crate::coreutils::basename::basename_main;
use crate::coreutils::cat::cat_main;
use crate::coreutils::chgrp::chgrp_main;
use crate::coreutils::chmod::chmod_main;
use crate::coreutils::chown::chown_main;
use crate::coreutils::chroot::chroot_main;
use crate::coreutils::cksum::cksum_main;
use crate::coreutils::comm::comm_main;
use crate::coreutils::cp::cp_main;
use crate::coreutils::cut::cut_main;
use crate::coreutils::date::date_main;
use crate::coreutils::dd::dd_main;
use crate::coreutils::df::df_main;
use crate::coreutils::dirname::dirname_main;
use crate::coreutils::dos2unix::dos2unix_main;
use crate::coreutils::du::du_main;
use crate::coreutils::echo::echo_main;
use crate::coreutils::env::env_main;
use crate::coreutils::expand::expand_main;
use crate::coreutils::expr::expr_main;
use crate::coreutils::factor::factor_main;
use crate::coreutils::fold::fold_main;
use crate::coreutils::head::head_main;
use crate::coreutils::hostid::hostid_main;
use crate::coreutils::id::id_main;
use crate::coreutils::install::install_main;
use crate::coreutils::link::link_main;
use crate::coreutils::ln::ln_main;
use crate::coreutils::logname::logname_main;
use crate::coreutils::ls::ls_main;
use crate::coreutils::md5_sha1_sum::md5_sha1_sum_main;
use crate::coreutils::mkdir::mkdir_main;
use crate::coreutils::mkfifo::mkfifo_main;
use crate::coreutils::mknod::mknod_main;
use crate::coreutils::mktemp::mktemp_main;
use crate::coreutils::mv::mv_main;
use crate::coreutils::nice::nice_main;
use crate::coreutils::nl::nl_main;
use crate::coreutils::nohup::nohup_main;
use crate::coreutils::nproc::nproc_main;
use crate::coreutils::od::od_main;
use crate::coreutils::paste::paste_main;
use crate::coreutils::printenv::printenv_main;
use crate::coreutils::printf::printf_main;
use crate::coreutils::pwd::pwd_main;
use crate::coreutils::r#false::false_main;
use crate::coreutils::r#true::true_main;
use crate::coreutils::readlink::readlink_main;
use crate::coreutils::realpath::realpath_main;
use crate::coreutils::rm::rm_main;
use crate::coreutils::rmdir::rmdir_main;
use crate::coreutils::seq::seq_main;
use crate::coreutils::shred::shred_main;
use crate::coreutils::shuf::shuf_main;
use crate::coreutils::sleep::sleep_main;
use crate::coreutils::sort::sort_main;
use crate::coreutils::split::split_main;
use crate::coreutils::stat::stat_main;
use crate::coreutils::stty::stty_main;
use crate::coreutils::sum::sum_main;
use crate::coreutils::sync::fsync_main;
use crate::coreutils::sync::sync_main;
use crate::coreutils::tac::tac_main;
use crate::coreutils::tail::tail_main;
use crate::coreutils::tee::tee_main;
use crate::coreutils::test::test_main;
use crate::coreutils::timeout::timeout_main;
use crate::coreutils::touch::touch_main;
use crate::coreutils::tr::tr_main;
use crate::coreutils::truncate::truncate_main;
use crate::coreutils::tty::tty_main;
use crate::coreutils::uname::uname_main;
use crate::coreutils::uniq::uniq_main;
use crate::coreutils::unlink::unlink_main;
use crate::coreutils::usleep::usleep_main;
use crate::coreutils::uudecode::base64_main;
use crate::coreutils::uudecode::uudecode_main;
use crate::coreutils::uuencode::uuencode_main;
use crate::coreutils::wc::wc_main;
use crate::coreutils::who::who_main;
use crate::coreutils::whoami::whoami_main;
use crate::coreutils::yes::yes_main;
use crate::debianutils::pipe_progress::pipe_progress_main;
use crate::debianutils::run_parts::run_parts_main;
use crate::debianutils::start_stop_daemon::start_stop_daemon_main;
use crate::debianutils::which::which_main;
use crate::e2fsprogs::chattr::chattr_main;
use crate::e2fsprogs::fsck::fsck_main;
use crate::e2fsprogs::lsattr::lsattr_main;
use crate::editors::awk::awk_main;
use crate::editors::cmp::cmp_main;
use crate::editors::diff::diff_main;
use crate::editors::ed::ed_main;
use crate::editors::patch::patch_main;
use crate::editors::sed::sed_main;
use crate::editors::vi::vi_main;
use crate::findutils::find::find_main;
use crate::findutils::grep::grep_main;
use crate::findutils::xargs::xargs_main;
use crate::init::bootchartd::bootchartd_main;
use crate::init::halt::halt_main;
use crate::init::init::init_main;
use crate::klibc_utils::nuke::nuke_main;
use crate::klibc_utils::resume::resume_main;
use crate::loginutils::add_remove_shell::add_remove_shell_main;
use crate::loginutils::addgroup::addgroup_main;
use crate::loginutils::adduser::adduser_main;
use crate::loginutils::chpasswd::chpasswd_main;
use crate::loginutils::cryptpw::cryptpw_main;
use crate::loginutils::deluser::deluser_main;
use crate::loginutils::getty::getty_main;
/* Needs to be run by root or be suid root - needs to change uid and gid: */
use crate::loginutils::login::login_main;
/* Needs to be run by root or be suid root - needs to change /etc/{passwd,shadow}: */
use crate::loginutils::passwd::passwd_main;
/* Needs to be run by root or be suid root - needs to change uid and gid: */
use crate::loginutils::su::su_main;
use crate::loginutils::sulogin::sulogin_main;
/* Needs to be run by root or be suid root - needs to change uid and gid: */
use crate::loginutils::vlock::vlock_main;
use crate::mailutils::makemime::makemime_main;
use crate::mailutils::popmaildir::popmaildir_main;
use crate::mailutils::reformime::reformime_main;
use crate::mailutils::sendmail::sendmail_main;
use crate::miscutils::adjtimex::adjtimex_main;
use crate::miscutils::bc::bc_main;
use crate::miscutils::bc::dc_main;
use crate::miscutils::beep::beep_main;
use crate::miscutils::chat::chat_main;
use crate::miscutils::conspy::conspy_main;
use crate::miscutils::crond::crond_main;
use crate::miscutils::crontab::crontab_main;
use crate::miscutils::devmem::devmem_main;
use crate::miscutils::fbsplash::fbsplash_main;
use crate::miscutils::hdparm::hdparm_main;
use crate::miscutils::hexedit::hexedit_main;
use crate::miscutils::i2c_tools::i2cdetect_main;
use crate::miscutils::i2c_tools::i2cdump_main;
use crate::miscutils::i2c_tools::i2cget_main;
use crate::miscutils::i2c_tools::i2cset_main;
use crate::miscutils::i2c_tools::i2ctransfer_main;
use crate::miscutils::less::less_main;
use crate::miscutils::lsscsi::lsscsi_main;
use crate::miscutils::makedevs::makedevs_main;
use crate::miscutils::man::man_main;
use crate::miscutils::microcom::microcom_main;
use crate::miscutils::mt::mt_main;
use crate::miscutils::nandwrite::nandwrite_main;
use crate::miscutils::partprobe::partprobe_main;
use crate::miscutils::raidautorun::raidautorun_main;
use crate::miscutils::readahead::readahead_main;
use crate::miscutils::runlevel::runlevel_main;
use crate::miscutils::rx::rx_main;
use crate::miscutils::setfattr::setfattr_main;
use crate::miscutils::setserial::setserial_main;
use crate::miscutils::strings::strings_main;
use crate::miscutils::time::time_main;
use crate::miscutils::ts::ts_main;
use crate::miscutils::ttysize::ttysize_main;
use crate::miscutils::ubi_tools::ubi_tools_main;
use crate::miscutils::ubirename::ubirename_main;
use crate::miscutils::volname::volname_main;
use crate::miscutils::watchdog::watchdog_main;
use crate::modutils::modinfo::modinfo_main;
use crate::modutils::modprobe_small::lsmod_main;
use crate::modutils::modprobe_small::modprobe_main;
use crate::networking::arp::arp_main;
use crate::networking::arping::arping_main;
use crate::networking::brctl::brctl_main;
use crate::networking::dnsd::dnsd_main;
use crate::networking::ether_wake::ether_wake_main;
use crate::networking::ftpd::ftpd_main;
use crate::networking::ftpgetput::ftpgetput_main;
use crate::networking::hostname::hostname_main;
use crate::networking::httpd::httpd_main;
use crate::networking::ifconfig::ifconfig_main;
use crate::networking::ifenslave::ifenslave_main;
use crate::networking::ifplugd::ifplugd_main;
use crate::networking::ifupdown::ifupdown_main;
use crate::networking::inetd::inetd_main;
use crate::networking::ip::ip_main;
use crate::networking::ip::ipaddr_main;
use crate::networking::ip::iplink_main;
use crate::networking::ip::ipneigh_main;
use crate::networking::ip::iproute_main;
use crate::networking::ip::iprule_main;
use crate::networking::ip::iptunnel_main;
use crate::networking::ipcalc::ipcalc_main;
use crate::networking::isrv_identd::fakeidentd_main;
use crate::networking::nameif::nameif_main;
use crate::networking::nbd_client::nbdclient_main;
use crate::networking::nc::nc_main;
use crate::networking::netstat::netstat_main;
use crate::networking::nslookup::nslookup_main;
use crate::networking::ntpd::ntpd_main;
use crate::networking::ping::ping6_main;
use crate::networking::ping::ping_main;
use crate::networking::pscan::pscan_main;
use crate::networking::route::route_main;
use crate::networking::slattach::slattach_main;
use crate::networking::ssl_client::ssl_client_main;
use crate::networking::tc::tc_main;
use crate::networking::tcpudp::tcpudpsvd_main;
use crate::networking::telnet::telnet_main;
use crate::networking::telnetd::telnetd_main;
use crate::networking::tftp::tftp_main;
use crate::networking::tftp::tftpd_main;
use crate::networking::traceroute::traceroute6_main;
use crate::networking::traceroute::traceroute_main;
use crate::networking::tunctl::tunctl_main;
use crate::networking::udhcp::d6_dhcpc::udhcpc6_main;
use crate::networking::udhcp::dhcpc::udhcpc_main;
use crate::networking::udhcp::dhcpd::udhcpd_main;
use crate::networking::udhcp::dhcprelay::dhcprelay_main;
use crate::networking::udhcp::dumpleases::dumpleases_main;
use crate::networking::vconfig::vconfig_main;
use crate::networking::wget::wget_main;
use crate::networking::whois::whois_main;
use crate::networking::zcip::zcip_main;
use crate::printutils::lpd::lpd_main;
use crate::printutils::lpr::lpqr_main;
use crate::procps::free::free_main;
use crate::procps::fuser::fuser_main;
use crate::procps::iostat::iostat_main;
use crate::procps::kill::kill_main;
use crate::procps::lsof::lsof_main;
use crate::procps::mpstat::mpstat_main;
use crate::procps::nmeter::nmeter_main;
use crate::procps::pgrep::pgrep_main;
use crate::procps::pidof::pidof_main;
use crate::procps::pmap::pmap_main;
use crate::procps::powertop::powertop_main;
use crate::procps::ps::ps_main;
use crate::procps::pstree::pstree_main;
use crate::procps::pwdx::pwdx_main;
use crate::procps::smemcap::smemcap_main;
use crate::procps::sysctl::sysctl_main;
use crate::procps::top::top_main;
use crate::procps::uptime::uptime_main;
use crate::procps::watch::watch_main;
use crate::runit::chpst::chpst_main;
use crate::runit::runsv::runsv_main;
use crate::runit::runsvdir::runsvdir_main;
use crate::runit::sv::sv_main;
use crate::runit::sv::svc_main;
use crate::runit::sv::svok_main;
use crate::runit::svlogd::svlogd_main;
use crate::shell::ash::ash_main;
use crate::shell::cttyhack::cttyhack_main;
use crate::shell::hush::hush_main;
use crate::sysklogd::klogd::klogd_main;
use crate::sysklogd::logread::logread_main;
use crate::sysklogd::syslogd_and_logger::logger_main;
use crate::sysklogd::syslogd_and_logger::syslogd_main;
use crate::util_linux::acpid::acpid_main;
use crate::util_linux::blkdiscard::blkdiscard_main;
use crate::util_linux::blkid::blkid_main;
use crate::util_linux::blockdev::blockdev_main;
use crate::util_linux::cal::cal_main;
use crate::util_linux::chrt::chrt_main;
use crate::util_linux::dmesg::dmesg_main;
use crate::util_linux::eject::eject_main;
use crate::util_linux::fallocate::fallocate_main;
use crate::util_linux::fatattr::fatattr_main;
use crate::util_linux::fbset::fbset_main;
use crate::util_linux::fdformat::fdformat_main;
use crate::util_linux::fdisk::fdisk_main;
use crate::util_linux::findfs::findfs_main;
use crate::util_linux::flock::flock_main;
use crate::util_linux::freeramdisk::freeramdisk_main;
use crate::util_linux::fsck_minix::fsck_minix_main;
use crate::util_linux::fsfreeze::fsfreeze_main;
use crate::util_linux::fstrim::fstrim_main;
use crate::util_linux::getopt::getopt_main;
use crate::util_linux::hexdump::hexdump_main;
use crate::util_linux::hexdump_xxd::xxd_main;
use crate::util_linux::hwclock::hwclock_main;
use crate::util_linux::ionice::ionice_main;
use crate::util_linux::ipcrm::ipcrm_main;
use crate::util_linux::ipcs::ipcs_main;
use crate::util_linux::last_fancy::last_main;
use crate::util_linux::losetup::losetup_main;
use crate::util_linux::lspci::lspci_main;
use crate::util_linux::lsusb::lsusb_main;
use crate::util_linux::mdev::mdev_main;
use crate::util_linux::mesg::mesg_main;
use crate::util_linux::mkfs_ext2::mkfs_ext2_main;
use crate::util_linux::mkfs_minix::mkfs_minix_main;
use crate::util_linux::mkfs_vfat::mkfs_vfat_main;
use crate::util_linux::mkswap::mkswap_main;
use crate::util_linux::more::more_main;
use crate::util_linux::mount::mount_main;
use crate::util_linux::mountpoint::mountpoint_main;
use crate::util_linux::nsenter::nsenter_main;
use crate::util_linux::pivot_root::pivot_root_main;
use crate::util_linux::rdate::rdate_main;
use crate::util_linux::rdev::rdev_main;
use crate::util_linux::readprofile::readprofile_main;
use crate::util_linux::renice::renice_main;
use crate::util_linux::rev::rev_main;
use crate::util_linux::rtcwake::rtcwake_main;
use crate::util_linux::script::script_main;
use crate::util_linux::scriptreplay::scriptreplay_main;
use crate::util_linux::setarch::setarch_main;
use crate::util_linux::setpriv::setpriv_main;
use crate::util_linux::setsid::setsid_main;
use crate::util_linux::swaponoff::swap_on_off_main;
use crate::util_linux::switch_root::switch_root_main;
use crate::util_linux::taskset::taskset_main;
use crate::util_linux::uevent::uevent_main;
use crate::util_linux::umount::umount_main;
use crate::util_linux::unshare::unshare_main;
use crate::util_linux::wall::wall_main;
use libc;

/* * applets.h - a listing of all busybox applets.
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
        SUID_REQUIRE: will complain if busybox isn't suid
        and is run by non-root (applet_main() will not be called at all)
        SUID_DROP: will drop suid prior to applet_main()
        SUID_MAYBE: neither of the above
        (every instance of SUID_REQUIRE and SUID_MAYBE
        needs to be justified in comment)
        NB: please update FEATURE_SUID help text whenever you add/remove
        SUID_REQUIRE or SUID_MAYBE applet.
*/

#[derive(Copy, Clone)]
pub enum InstallLoc {
  DIR_USR_SBIN,
  DIR_USR_BIN,
  DIR_SBIN,
  DIR_BIN,
  DIR_ROOT,
}

#[derive(PartialEq)]
pub enum SUID {
  SUID_REQUIRE,
  SUID_MAYBE,
  SUID_DROP,
}

// TODO: it's not clear to me how if at all noexec and nofork are actually used
// in the code. Should they be removed?
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
name  - applet name as it is typed on command line
help  - applet name, converted to C (ether-wake: help = ether_wake)
main  - corresponding <applet>_main to call (bzcat: main = bunzip2)
l     - location to install link to: [/usr]/[s]bin
s     - suid type:
        SUID_REQUIRE: will complain if busybox isn't suid
        and is run by non-root (applet_main() will not be called at all)
        SUID_DROP: will drop suid prior to applet_main()
        SUID_MAYBE: neither of the above
        (every instance of SUID_REQUIRE and SUID_MAYBE
        needs to be justified in comment)
        NB: please update FEATURE_SUID help text whenever you add/remove
        SUID_REQUIRE or SUID_MAYBE applet.
*/

lazy_static! {
  pub static ref applets: Vec<bb_applet> = {
    let mut appy_mcappface: Vec<bb_applet> = Vec::new();

    #[cfg(feature = "test-bracket1")]
    appy_mcappface.push(bb_applet {
      name: "[",
      main: "test",
      entrypoint: test_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/["),
    });
    #[cfg(feature = "test-bracket2")]
    appy_mcappface.push(bb_applet {
      name: "[[",
      main: "test",
      entrypoint: test_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/[["),
    });
    #[cfg(feature = "acpid")]
    appy_mcappface.push(bb_applet {
      name: "acpid",
      main: "acpid",
      entrypoint: acpid_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/acpid"),
    });
    #[cfg(feature = "add-shell")]
    appy_mcappface.push(bb_applet {
      name: "add-shell",
      main: "add_remove_shell",
      entrypoint: add_remove_shell_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/add-shell"),
    });
    #[cfg(feature = "addgroup")]
    appy_mcappface.push(bb_applet {
      name: "addgroup",
      main: "addgroup",
      entrypoint: addgroup_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/addgroup"),
    });
    #[cfg(feature = "adduser")]
    appy_mcappface.push(bb_applet {
      name: "adduser",
      main: "adduser",
      entrypoint: adduser_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/adduser"),
    });
    #[cfg(feature = "adjtimex")]
    appy_mcappface.push(bb_applet {
      name: "adjtimex",
      main: "adjtimex",
      entrypoint: adjtimex_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/adjtimex"),
    });
    #[cfg(feature = "arch")]
    appy_mcappface.push(bb_applet {
      name: "arch",
      main: "uname",
      entrypoint: uname_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/arch"),
    });
    #[cfg(feature = "arp")]
    appy_mcappface.push(bb_applet {
      name: "arp",
      main: "arp",
      entrypoint: arp_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/arp"),
    });
    #[cfg(feature = "arping")]
    appy_mcappface.push(bb_applet {
      name: "arping",
      main: "arping",
      entrypoint: arping_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/arping"),
    });
    #[cfg(feature = "ash")]
    appy_mcappface.push(bb_applet {
      name: "ash",
      main: "ash",
      entrypoint: ash_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/ash"),
    });
    #[cfg(feature = "awk")]
    appy_mcappface.push(bb_applet {
      name: "awk",
      main: "awk",
      entrypoint: awk_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/awk"),
    });
    #[cfg(feature = "base64")]
    appy_mcappface.push(bb_applet {
      name: "base64",
      main: "base64",
      entrypoint: base64_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/base64"),
    });
    #[cfg(feature = "basename")]
    appy_mcappface.push(bb_applet {
      name: "basename",
      main: "basename",
      entrypoint: basename_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/basename"),
    });
    #[cfg(feature = "bc")]
    appy_mcappface.push(bb_applet {
      name: "bc",
      main: "bc",
      entrypoint: bc_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/bc"),
    });
    #[cfg(feature = "beep")]
    appy_mcappface.push(bb_applet {
      name: "beep",
      main: "beep",
      entrypoint: beep_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/beep"),
    });
    #[cfg(feature = "blkdiscard")]
    appy_mcappface.push(bb_applet {
      name: "blkdiscard",
      main: "blkdiscard",
      entrypoint: blkdiscard_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/blkdiscard"),
    });
    #[cfg(feature = "blkid")]
    appy_mcappface.push(bb_applet {
      name: "blkid",
      main: "blkid",
      entrypoint: blkid_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/blkid"),
    });
    #[cfg(feature = "blockdev")]
    appy_mcappface.push(bb_applet {
      name: "blockdev",
      main: "blockdev",
      entrypoint: blockdev_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/blockdev"),
    });
    #[cfg(feature = "bootchartd")]
    appy_mcappface.push(bb_applet {
      name: "bootchartd",
      main: "bootchartd",
      entrypoint: bootchartd_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/bootchartd"),
    });
    #[cfg(feature = "brctl")]
    appy_mcappface.push(bb_applet {
      name: "brctl",
      main: "brctl",
      entrypoint: brctl_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/brctl"),
    });
    #[cfg(feature = "bunzip2")]
    appy_mcappface.push(bb_applet {
      name: "bunzip2",
      main: "bunzip2",
      entrypoint: bunzip2_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/bunzip2"),
    });
    #[cfg(feature = "bzcat")]
    appy_mcappface.push(bb_applet {
      name: "bzcat",
      main: "bunzip2",
      entrypoint: bunzip2_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/bzcat"),
    });
    #[cfg(feature = "bzip2")]
    appy_mcappface.push(bb_applet {
      name: "bzip2",
      main: "bzip2",
      entrypoint: bzip2_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/bzip2"),
    });
    #[cfg(feature = "cal")]
    appy_mcappface.push(bb_applet {
      name: "cal",
      main: "cal",
      entrypoint: cal_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/cal"),
    });
    #[cfg(feature = "cat")]
    appy_mcappface.push(bb_applet {
      name: "cat",
      main: "cat",
      entrypoint: cat_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/cat"),
    });
    #[cfg(feature = "chat")]
    appy_mcappface.push(bb_applet {
      name: "chat",
      main: "chat",
      entrypoint: chat_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/chat"),
    });
    #[cfg(feature = "chattr")]
    appy_mcappface.push(bb_applet {
      name: "chattr",
      main: "chattr",
      entrypoint: chattr_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/chattr"),
    });
    #[cfg(feature = "chgrp")]
    appy_mcappface.push(bb_applet {
      name: "chgrp",
      main: "chgrp",
      entrypoint: chgrp_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/chgrp"),
    });
    #[cfg(feature = "chmod")]
    appy_mcappface.push(bb_applet {
      name: "chmod",
      main: "chmod",
      entrypoint: chmod_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/chmod"),
    });
    #[cfg(feature = "chown")]
    appy_mcappface.push(bb_applet {
      name: "chown",
      main: "chown",
      entrypoint: chown_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/chown"),
    });
    #[cfg(feature = "chpasswd")]
    appy_mcappface.push(bb_applet {
      name: "chpasswd",
      main: "chpasswd",
      entrypoint: chpasswd_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/chpasswd"),
    });
    #[cfg(feature = "chpst")]
    appy_mcappface.push(bb_applet {
      name: "chpst",
      main: "chpst",
      entrypoint: chpst_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/chpst"),
    });
    #[cfg(feature = "chroot")]
    appy_mcappface.push(bb_applet {
      name: "chroot",
      main: "chroot",
      entrypoint: chroot_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/chroot"),
    });
    #[cfg(feature = "chrt")]
    appy_mcappface.push(bb_applet {
      name: "chrt",
      main: "chrt",
      entrypoint: chrt_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/chrt"),
    });
    #[cfg(feature = "chvt")]
    appy_mcappface.push(bb_applet {
      name: "chvt",
      main: "chvt",
      entrypoint: chvt_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/chvt"),
    });
    #[cfg(feature = "cksum")]
    appy_mcappface.push(bb_applet {
      name: "cksum",
      main: "cksum",
      entrypoint: cksum_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/cksum"),
    });
    #[cfg(feature = "clear")]
    appy_mcappface.push(bb_applet {
      name: "clear",
      main: "clear",
      entrypoint: clear_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/clear"),
    });
    #[cfg(feature = "cmp")]
    appy_mcappface.push(bb_applet {
      name: "cmp",
      main: "cmp",
      entrypoint: cmp_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/cmp"),
    });
    #[cfg(feature = "comm")]
    appy_mcappface.push(bb_applet {
      name: "comm",
      main: "comm",
      entrypoint: comm_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/comm"),
    });
    #[cfg(feature = "conspy")]
    appy_mcappface.push(bb_applet {
      name: "conspy",
      main: "conspy",
      entrypoint: conspy_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/conspy"),
    });
    #[cfg(feature = "cp")]
    appy_mcappface.push(bb_applet {
      name: "cp",
      main: "cp",
      entrypoint: cp_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/cp"),
    });
    #[cfg(feature = "cpio")]
    appy_mcappface.push(bb_applet {
      name: "cpio",
      main: "cpio",
      entrypoint: cpio_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/cpio"),
    });
    #[cfg(feature = "crond")]
    appy_mcappface.push(bb_applet {
      name: "crond",
      main: "crond",
      entrypoint: crond_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/crond"),
    });
    #[cfg(feature = "crontab")]
    appy_mcappface.push(bb_applet {
      name: "crontab",
      main: "crontab",
      entrypoint: crontab_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_REQUIRE,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/crontab"),
    });
    #[cfg(feature = "cryptpw")]
    appy_mcappface.push(bb_applet {
      name: "cryptpw",
      main: "cryptpw",
      entrypoint: cryptpw_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/cryptpw"),
    });
    #[cfg(feature = "cttyhack")]
    appy_mcappface.push(bb_applet {
      name: "cttyhack",
      main: "cttyhack",
      entrypoint: cttyhack_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/cttyhack"),
    });
    #[cfg(feature = "cut")]
    appy_mcappface.push(bb_applet {
      name: "cut",
      main: "cut",
      entrypoint: cut_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/cut"),
    });
    #[cfg(feature = "date")]
    appy_mcappface.push(bb_applet {
      name: "date",
      main: "date",
      entrypoint: date_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/date"),
    });
    #[cfg(feature = "dc")]
    appy_mcappface.push(bb_applet {
      name: "dc",
      main: "dc",
      entrypoint: dc_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/dc"),
    });
    #[cfg(feature = "dd")]
    appy_mcappface.push(bb_applet {
      name: "dd",
      main: "dd",
      entrypoint: dd_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/dd"),
    });
    #[cfg(feature = "deallocvt")]
    appy_mcappface.push(bb_applet {
      name: "deallocvt",
      main: "deallocvt",
      entrypoint: deallocvt_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/deallocvt"),
    });
    #[cfg(feature = "delgroup")]
    appy_mcappface.push(bb_applet {
      name: "delgroup",
      main: "deluser",
      entrypoint: deluser_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/delgroup"),
    });
    #[cfg(feature = "deluser")]
    appy_mcappface.push(bb_applet {
      name: "deluser",
      main: "deluser",
      entrypoint: deluser_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/deluser"),
    });
    #[cfg(feature = "depmod")]
    appy_mcappface.push(bb_applet {
      name: "depmod",
      main: "modprobe",
      entrypoint: modprobe_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/depmod"),
    });
    #[cfg(feature = "devmem")]
    appy_mcappface.push(bb_applet {
      name: "devmem",
      main: "devmem",
      entrypoint: devmem_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/devmem"),
    });
    #[cfg(feature = "df")]
    appy_mcappface.push(bb_applet {
      name: "df",
      main: "df",
      entrypoint: df_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/df"),
    });
    #[cfg(feature = "dhcprelay")]
    appy_mcappface.push(bb_applet {
      name: "dhcprelay",
      main: "dhcprelay",
      entrypoint: dhcprelay_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/dhcprelay"),
    });
    #[cfg(feature = "diff")]
    appy_mcappface.push(bb_applet {
      name: "diff",
      main: "diff",
      entrypoint: diff_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/diff"),
    });
    #[cfg(feature = "dirname")]
    appy_mcappface.push(bb_applet {
      name: "dirname",
      main: "dirname",
      entrypoint: dirname_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/dirname"),
    });
    #[cfg(feature = "dmesg")]
    appy_mcappface.push(bb_applet {
      name: "dmesg",
      main: "dmesg",
      entrypoint: dmesg_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/dmesg"),
    });
    #[cfg(feature = "dnsd")]
    appy_mcappface.push(bb_applet {
      name: "dnsd",
      main: "dnsd",
      entrypoint: dnsd_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/dnsd"),
    });
    #[cfg(feature = "dnsdomainname")]
    appy_mcappface.push(bb_applet {
      name: "dnsdomainname",
      main: "hostname",
      entrypoint: hostname_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/dnsdomainname"),
    });
    #[cfg(feature = "dos2unix")]
    appy_mcappface.push(bb_applet {
      name: "dos2unix",
      main: "dos2unix",
      entrypoint: dos2unix_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/dos2unix"),
    });
    #[cfg(feature = "dpkg")]
    appy_mcappface.push(bb_applet {
      name: "dpkg",
      main: "dpkg",
      entrypoint: dpkg_deb_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/dpkg"),
    });
    #[cfg(feature = "dpkg-deb")]
    appy_mcappface.push(bb_applet {
      name: "dpkg-deb",
      main: "dpkg_deb",
      entrypoint: dpkg_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/dpkg-deb"),
    });
    #[cfg(feature = "du")]
    appy_mcappface.push(bb_applet {
      name: "du",
      main: "du",
      entrypoint: du_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/du"),
    });
    #[cfg(feature = "dumpkmap")]
    appy_mcappface.push(bb_applet {
      name: "dumpkmap",
      main: "dumpkmap",
      entrypoint: dumpkmap_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/dumpkmap"),
    });
    #[cfg(feature = "dumpleases")]
    appy_mcappface.push(bb_applet {
      name: "dumpleases",
      main: "dumpleases",
      entrypoint: dumpleases_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/dumpleases"),
    });
    #[cfg(feature = "echo")]
    appy_mcappface.push(bb_applet {
      name: "echo",
      main: "echo",
      entrypoint: echo_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/echo"),
    });
    #[cfg(feature = "ed")]
    appy_mcappface.push(bb_applet {
      name: "ed",
      main: "ed",
      entrypoint: ed_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/ed"),
    });
    #[cfg(feature = "egrep")]
    appy_mcappface.push(bb_applet {
      name: "egrep",
      main: "grep",
      entrypoint: grep_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/egrep"),
    });
    #[cfg(feature = "eject")]
    appy_mcappface.push(bb_applet {
      name: "eject",
      main: "eject",
      entrypoint: eject_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/eject"),
    });
    #[cfg(feature = "env")]
    appy_mcappface.push(bb_applet {
      name: "env",
      main: "env",
      entrypoint: env_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/env"),
    });
    #[cfg(feature = "envdir")]
    appy_mcappface.push(bb_applet {
      name: "envdir",
      main: "chpst",
      entrypoint: chpst_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/envdir"),
    });
    #[cfg(feature = "envuidgid")]
    appy_mcappface.push(bb_applet {
      name: "envuidgid",
      main: "chpst",
      entrypoint: chpst_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/envuidgid"),
    });
    #[cfg(feature = "ether-wake")]
    appy_mcappface.push(bb_applet {
      name: "ether-wake",
      main: "ether_wake",
      entrypoint: ether_wake_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/ether-wake"),
    });
    #[cfg(feature = "expand")]
    appy_mcappface.push(bb_applet {
      name: "expand",
      main: "expand",
      entrypoint: expand_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/expand"),
    });
    #[cfg(feature = "expr")]
    appy_mcappface.push(bb_applet {
      name: "expr",
      main: "expr",
      entrypoint: expr_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/expr"),
    });
    #[cfg(feature = "factor")]
    appy_mcappface.push(bb_applet {
      name: "factor",
      main: "factor",
      entrypoint: factor_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/factor"),
    });
    #[cfg(feature = "fakeidentd")]
    appy_mcappface.push(bb_applet {
      name: "fakeidentd",
      main: "fakeidentd",
      entrypoint: fakeidentd_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/fakeidentd"),
    });
    #[cfg(feature = "fallocate")]
    appy_mcappface.push(bb_applet {
      name: "fallocate",
      main: "fallocate",
      entrypoint: fallocate_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/fallocate"),
    });
    #[cfg(feature = "false")]
    appy_mcappface.push(bb_applet {
      name: "false",
      main: "false",
      entrypoint: false_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/false"),
    });
    #[cfg(feature = "fatattr")]
    appy_mcappface.push(bb_applet {
      name: "fatattr",
      main: "fatattr",
      entrypoint: fatattr_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/fatattr"),
    });
    #[cfg(feature = "fbset")]
    appy_mcappface.push(bb_applet {
      name: "fbset",
      main: "fbset",
      entrypoint: fbset_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/fbset"),
    });
    #[cfg(feature = "fbsplash")]
    appy_mcappface.push(bb_applet {
      name: "fbsplash",
      main: "fbsplash",
      entrypoint: fbsplash_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/fbsplash"),
    });
    #[cfg(feature = "fdflush")]
    appy_mcappface.push(bb_applet {
      name: "fdflush",
      main: "freeramdisk",
      entrypoint: freeramdisk_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/fdflush"),
    });
    #[cfg(feature = "fdformat")]
    appy_mcappface.push(bb_applet {
      name: "fdformat",
      main: "fdformat",
      entrypoint: fdformat_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/fdformat"),
    });
    #[cfg(feature = "fdisk")]
    appy_mcappface.push(bb_applet {
      name: "fdisk",
      main: "fdisk",
      entrypoint: fdisk_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/fdisk"),
    });
    #[cfg(feature = "fgconsole")]
    appy_mcappface.push(bb_applet {
      name: "fgconsole",
      main: "fgconsole",
      entrypoint: fgconsole_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/fgconsole"),
    });
    #[cfg(feature = "fgrep")]
    appy_mcappface.push(bb_applet {
      name: "fgrep",
      main: "grep",
      entrypoint: grep_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/fgrep"),
    });
    #[cfg(feature = "find")]
    appy_mcappface.push(bb_applet {
      name: "find",
      main: "find",
      entrypoint: find_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/find"),
    });
    #[cfg(feature = "findfs")]
    appy_mcappface.push(bb_applet {
      name: "findfs",
      main: "findfs",
      entrypoint: findfs_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_MAYBE,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/findfs"),
    });
    #[cfg(feature = "flock")]
    appy_mcappface.push(bb_applet {
      name: "flock",
      main: "flock",
      entrypoint: flock_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/flock"),
    });
    #[cfg(feature = "fold")]
    appy_mcappface.push(bb_applet {
      name: "fold",
      main: "fold",
      entrypoint: fold_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/fold"),
    });
    #[cfg(feature = "free")]
    appy_mcappface.push(bb_applet {
      name: "free",
      main: "free",
      entrypoint: free_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/free"),
    });
    #[cfg(feature = "freeramdisk")]
    appy_mcappface.push(bb_applet {
      name: "freeramdisk",
      main: "freeramdisk",
      entrypoint: freeramdisk_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/freeramdisk"),
    });
    #[cfg(feature = "fsck")]
    appy_mcappface.push(bb_applet {
      name: "fsck",
      main: "fsck",
      entrypoint: fsck_minix_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/fsck"),
    });
    #[cfg(feature = "fsck_minix")]
    appy_mcappface.push(bb_applet {
      name: "fsck.minix",
      main: "fsck_minix",
      entrypoint: fsck_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/fsck.minix"),
    });
    #[cfg(feature = "fsfreeze")]
    appy_mcappface.push(bb_applet {
      name: "fsfreeze",
      main: "fsfreeze",
      entrypoint: fsfreeze_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/fsfreeze"),
    });
    #[cfg(feature = "fstrim")]
    appy_mcappface.push(bb_applet {
      name: "fstrim",
      main: "fstrim",
      entrypoint: fstrim_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/fstrim"),
    });
    #[cfg(feature = "fsync")]
    appy_mcappface.push(bb_applet {
      name: "fsync",
      main: "fsync",
      entrypoint: fsync_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/fsync"),
    });
    #[cfg(feature = "ftpd")]
    appy_mcappface.push(bb_applet {
      name: "ftpd",
      main: "ftpd",
      entrypoint: ftpd_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/ftpd"),
    });
    #[cfg(feature = "ftpget")]
    appy_mcappface.push(bb_applet {
      name: "ftpget",
      main: "ftpgetput",
      entrypoint: ftpgetput_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/ftpget"),
    });
    #[cfg(feature = "ftpput")]
    appy_mcappface.push(bb_applet {
      name: "ftpput",
      main: "ftpgetput",
      entrypoint: ftpgetput_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/ftpput"),
    });
    #[cfg(feature = "fuser")]
    appy_mcappface.push(bb_applet {
      name: "fuser",
      main: "fuser",
      entrypoint: fuser_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/fuser"),
    });
    #[cfg(feature = "getopt")]
    appy_mcappface.push(bb_applet {
      name: "getopt",
      main: "getopt",
      entrypoint: getopt_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/getopt"),
    });
    #[cfg(feature = "getty")]
    appy_mcappface.push(bb_applet {
      name: "getty",
      main: "getty",
      entrypoint: getty_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/getty"),
    });
    #[cfg(feature = "grep")]
    appy_mcappface.push(bb_applet {
      name: "grep",
      main: "grep",
      entrypoint: grep_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/grep"),
    });
    #[cfg(feature = "groups")]
    appy_mcappface.push(bb_applet {
      name: "groups",
      main: "id",
      entrypoint: id_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/groups"),
    });
    #[cfg(feature = "gunzip")]
    appy_mcappface.push(bb_applet {
      name: "gunzip",
      main: "gunzip",
      entrypoint: gunzip_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/gunzip"),
    });
    #[cfg(feature = "gzip")]
    appy_mcappface.push(bb_applet {
      name: "gzip",
      main: "gzip",
      entrypoint: gzip_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/gzip"),
    });
    #[cfg(feature = "halt")]
    appy_mcappface.push(bb_applet {
      name: "halt",
      main: "halt",
      entrypoint: halt_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/halt"),
    });
    #[cfg(feature = "hd")]
    appy_mcappface.push(bb_applet {
      name: "hd",
      main: "hexdump",
      entrypoint: hexdump_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/hd"),
    });
    #[cfg(feature = "hdparm")]
    appy_mcappface.push(bb_applet {
      name: "hdparm",
      main: "hdparm",
      entrypoint: hdparm_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/hdparm"),
    });
    #[cfg(feature = "head")]
    appy_mcappface.push(bb_applet {
      name: "head",
      main: "head",
      entrypoint: head_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/head"),
    });
    #[cfg(feature = "hexdump")]
    appy_mcappface.push(bb_applet {
      name: "hexdump",
      main: "hexdump",
      entrypoint: hexdump_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/hexdump"),
    });
    #[cfg(feature = "hexedit")]
    appy_mcappface.push(bb_applet {
      name: "hexedit",
      main: "hexedit",
      entrypoint: hexedit_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/hexedit"),
    });
    #[cfg(feature = "hostid")]
    appy_mcappface.push(bb_applet {
      name: "hostid",
      main: "hostid",
      entrypoint: hostid_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/hostid"),
    });
    #[cfg(feature = "hostname")]
    appy_mcappface.push(bb_applet {
      name: "hostname",
      main: "hostname",
      entrypoint: hostname_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/hostname"),
    });
    #[cfg(feature = "httpd")]
    appy_mcappface.push(bb_applet {
      name: "httpd",
      main: "httpd",
      entrypoint: httpd_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/httpd"),
    });
    #[cfg(feature = "hush")]
    appy_mcappface.push(bb_applet {
      name: "hush",
      main: "hush",
      entrypoint: hush_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/hush"),
    });
    #[cfg(feature = "hwclock")]
    appy_mcappface.push(bb_applet {
      name: "hwclock",
      main: "hwclock",
      entrypoint: hwclock_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/hwclock"),
    });
    #[cfg(feature = "i2cdetect")]
    appy_mcappface.push(bb_applet {
      name: "i2cdetect",
      main: "i2cdetect",
      entrypoint: i2cdetect_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/i2cdetect"),
    });
    #[cfg(feature = "i2cdump")]
    appy_mcappface.push(bb_applet {
      name: "i2cdump",
      main: "i2cdump",
      entrypoint: i2cdump_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/i2cdump"),
    });
    #[cfg(feature = "i2cget")]
    appy_mcappface.push(bb_applet {
      name: "i2cget",
      main: "i2cget",
      entrypoint: i2cget_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/i2cget"),
    });
    #[cfg(feature = "i2cset")]
    appy_mcappface.push(bb_applet {
      name: "i2cset",
      main: "i2cset",
      entrypoint: i2cset_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/i2cset"),
    });
    #[cfg(feature = "i2ctransfer")]
    appy_mcappface.push(bb_applet {
      name: "i2ctransfer",
      main: "i2ctransfer",
      entrypoint: i2ctransfer_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/i2ctransfer"),
    });
    #[cfg(feature = "id")]
    appy_mcappface.push(bb_applet {
      name: "id",
      main: "id",
      entrypoint: id_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/id"),
    });
    #[cfg(feature = "ifconfig")]
    appy_mcappface.push(bb_applet {
      name: "ifconfig",
      main: "ifconfig",
      entrypoint: ifconfig_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/ifconfig"),
    });
    #[cfg(feature = "ifdown")]
    appy_mcappface.push(bb_applet {
      name: "ifdown",
      main: "ifupdown",
      entrypoint: ifupdown_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/ifdown"),
    });
    #[cfg(feature = "ifenslave")]
    appy_mcappface.push(bb_applet {
      name: "ifenslave",
      main: "ifenslave",
      entrypoint: ifenslave_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/ifenslave"),
    });
    #[cfg(feature = "ifplugd")]
    appy_mcappface.push(bb_applet {
      name: "ifplugd",
      main: "ifplugd",
      entrypoint: ifplugd_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/ifplugd"),
    });
    #[cfg(feature = "ifup")]
    appy_mcappface.push(bb_applet {
      name: "ifup",
      main: "ifupdown",
      entrypoint: ifupdown_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/ifup"),
    });
    #[cfg(feature = "inetd")]
    appy_mcappface.push(bb_applet {
      name: "inetd",
      main: "inetd",
      entrypoint: inetd_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/inetd"),
    });
    #[cfg(feature = "init")]
    appy_mcappface.push(bb_applet {
      name: "init",
      main: "init",
      entrypoint: init_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/init"),
    });
    #[cfg(feature = "insmod")]
    appy_mcappface.push(bb_applet {
      name: "insmod",
      main: "modprobe",
      entrypoint: modprobe_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/insmod"),
    });
    #[cfg(feature = "install")]
    appy_mcappface.push(bb_applet {
      name: "install",
      main: "install",
      entrypoint: install_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/install"),
    });
    #[cfg(feature = "ionice")]
    appy_mcappface.push(bb_applet {
      name: "ionice",
      main: "ionice",
      entrypoint: ionice_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/ionice"),
    });
    #[cfg(feature = "iostat")]
    appy_mcappface.push(bb_applet {
      name: "iostat",
      main: "iostat",
      entrypoint: iostat_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/iostat"),
    });
    #[cfg(feature = "ip")]
    appy_mcappface.push(bb_applet {
      name: "ip",
      main: "ip",
      entrypoint: ip_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/ip"),
    });
    #[cfg(feature = "ipaddr")]
    appy_mcappface.push(bb_applet {
      name: "ipaddr",
      main: "ipaddr",
      entrypoint: ipaddr_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/ipaddr"),
    });
    #[cfg(feature = "ipcalc")]
    appy_mcappface.push(bb_applet {
      name: "ipcalc",
      main: "ipcalc",
      entrypoint: ipcalc_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/ipcalc"),
    });
    #[cfg(feature = "ipcrm")]
    appy_mcappface.push(bb_applet {
      name: "ipcrm",
      main: "ipcrm",
      entrypoint: ipcrm_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/ipcrm"),
    });
    #[cfg(feature = "ipcs")]
    appy_mcappface.push(bb_applet {
      name: "ipcs",
      main: "ipcs",
      entrypoint: ipcs_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/ipcs"),
    });
    #[cfg(feature = "iplink")]
    appy_mcappface.push(bb_applet {
      name: "iplink",
      main: "iplink",
      entrypoint: iplink_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/iplink"),
    });
    #[cfg(feature = "ipneigh")]
    appy_mcappface.push(bb_applet {
      name: "ipneigh",
      main: "ipneigh",
      entrypoint: ipneigh_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/ipneigh"),
    });
    #[cfg(feature = "iproute")]
    appy_mcappface.push(bb_applet {
      name: "iproute",
      main: "iproute",
      entrypoint: iproute_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/iproute"),
    });
    #[cfg(feature = "iprule")]
    appy_mcappface.push(bb_applet {
      name: "iprule",
      main: "iprule",
      entrypoint: iprule_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/iprule"),
    });
    #[cfg(feature = "iptunnel")]
    appy_mcappface.push(bb_applet {
      name: "iptunnel",
      main: "iptunnel",
      entrypoint: iptunnel_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/iptunnel"),
    });
    #[cfg(feature = "kbd_mode")]
    appy_mcappface.push(bb_applet {
      name: "kbd_mode",
      main: "kbd_mode",
      entrypoint: kbd_mode_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/kbd_mode"),
    });
    #[cfg(feature = "kill")]
    appy_mcappface.push(bb_applet {
      name: "kill",
      main: "kill",
      entrypoint: kill_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/kill"),
    });
    #[cfg(feature = "killall")]
    appy_mcappface.push(bb_applet {
      name: "killall",
      main: "kill",
      entrypoint: kill_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/killall"),
    });
    #[cfg(feature = "killall5")]
    appy_mcappface.push(bb_applet {
      name: "killall5",
      main: "kill",
      entrypoint: kill_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/killall5"),
    });
    #[cfg(feature = "klogd")]
    appy_mcappface.push(bb_applet {
      name: "klogd",
      main: "klogd",
      entrypoint: klogd_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/klogd"),
    });
    #[cfg(feature = "last")]
    appy_mcappface.push(bb_applet {
      name: "last",
      main: "last",
      entrypoint: last_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/last"),
    });
    #[cfg(feature = "less")]
    appy_mcappface.push(bb_applet {
      name: "less",
      main: "less",
      entrypoint: less_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/less"),
    });
    #[cfg(feature = "link")]
    appy_mcappface.push(bb_applet {
      name: "link",
      main: "link",
      entrypoint: link_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/link"),
    });
    #[cfg(feature = "linux32")]
    appy_mcappface.push(bb_applet {
      name: "linux32",
      main: "setarch",
      entrypoint: setarch_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/linux32"),
    });
    #[cfg(feature = "linux64")]
    appy_mcappface.push(bb_applet {
      name: "linux64",
      main: "setarch",
      entrypoint: setarch_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/linux64"),
    });
    #[cfg(feature = "linuxrc")]
    appy_mcappface.push(bb_applet {
      name: "linuxrc",
      main: "init",
      entrypoint: init_main,
      install_loc: InstallLoc::DIR_ROOT,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/linuxrc"),
    });
    #[cfg(feature = "ln")]
    appy_mcappface.push(bb_applet {
      name: "ln",
      main: "ln",
      entrypoint: ln_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/ln"),
    });
    #[cfg(feature = "loadfont")]
    appy_mcappface.push(bb_applet {
      name: "loadfont",
      main: "loadfont",
      entrypoint: loadfont_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/loadfont"),
    });
    #[cfg(feature = "loadkmap")]
    appy_mcappface.push(bb_applet {
      name: "loadkmap",
      main: "loadkmap",
      entrypoint: loadkmap_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/loadkmap"),
    });
    #[cfg(feature = "logger")]
    appy_mcappface.push(bb_applet {
      name: "logger",
      main: "logger",
      entrypoint: logger_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/logger"),
    });
    #[cfg(feature = "login")]
    appy_mcappface.push(bb_applet {
      name: "login",
      main: "login",
      entrypoint: login_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_REQUIRE,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/login"),
    });
    #[cfg(feature = "logname")]
    appy_mcappface.push(bb_applet {
      name: "logname",
      main: "logname",
      entrypoint: logname_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/logname"),
    });
    #[cfg(feature = "logread")]
    appy_mcappface.push(bb_applet {
      name: "logread",
      main: "logread",
      entrypoint: logread_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/logread"),
    });
    #[cfg(feature = "losetup")]
    appy_mcappface.push(bb_applet {
      name: "losetup",
      main: "losetup",
      entrypoint: losetup_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/losetup"),
    });
    #[cfg(feature = "lpd")]
    appy_mcappface.push(bb_applet {
      name: "lpd",
      main: "lpd",
      entrypoint: lpd_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/lpd"),
    });
    #[cfg(feature = "lpq")]
    appy_mcappface.push(bb_applet {
      name: "lpq",
      main: "lpqr",
      entrypoint: lpqr_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/lpq"),
    });
    #[cfg(feature = "lpr")]
    appy_mcappface.push(bb_applet {
      name: "lpr",
      main: "lpqr",
      entrypoint: lpqr_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/lpr"),
    });
    #[cfg(feature = "ls")]
    appy_mcappface.push(bb_applet {
      name: "ls",
      main: "ls",
      entrypoint: ls_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/ls"),
    });
    #[cfg(feature = "lsattr")]
    appy_mcappface.push(bb_applet {
      name: "lsattr",
      main: "lsattr",
      entrypoint: lsattr_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/lsattr"),
    });
    #[cfg(feature = "lsmod")]
    appy_mcappface.push(bb_applet {
      name: "lsmod",
      main: "lsmod",
      entrypoint: lsmod_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/lsmod"),
    });
    #[cfg(feature = "lsof")]
    appy_mcappface.push(bb_applet {
      name: "lsof",
      main: "lsof",
      entrypoint: lsof_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/lsof"),
    });
    #[cfg(feature = "lspci")]
    appy_mcappface.push(bb_applet {
      name: "lspci",
      main: "lspci",
      entrypoint: lspci_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/lspci"),
    });
    #[cfg(feature = "lsscsi")]
    appy_mcappface.push(bb_applet {
      name: "lsscsi",
      main: "lsscsi",
      entrypoint: lsscsi_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/lsscsi"),
    });
    #[cfg(feature = "lsusb")]
    appy_mcappface.push(bb_applet {
      name: "lsusb",
      main: "lsusb",
      entrypoint: lsusb_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/lsusb"),
    });
    #[cfg(feature = "lzcat")]
    appy_mcappface.push(bb_applet {
      name: "lzcat",
      main: "unlzma",
      entrypoint: unlzma_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/lzcat"),
    });
    #[cfg(feature = "lzma")]
    appy_mcappface.push(bb_applet {
      name: "lzma",
      main: "unlzma",
      entrypoint: unlzma_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/lzma"),
    });
    #[cfg(feature = "lzop")]
    appy_mcappface.push(bb_applet {
      name: "lzop",
      main: "lzop",
      entrypoint: lzop_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/lzop"),
    });
    #[cfg(feature = "makedevs")]
    appy_mcappface.push(bb_applet {
      name: "makedevs",
      main: "makedevs",
      entrypoint: makedevs_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/makedevs"),
    });
    #[cfg(feature = "makemime")]
    appy_mcappface.push(bb_applet {
      name: "makemime",
      main: "makemime",
      entrypoint: makemime_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/makemime"),
    });
    #[cfg(feature = "man")]
    appy_mcappface.push(bb_applet {
      name: "man",
      main: "man",
      entrypoint: man_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/man"),
    });
    #[cfg(feature = "md5sum")]
    appy_mcappface.push(bb_applet {
      name: "md5sum",
      main: "md5_sha1_sum",
      entrypoint: md5_sha1_sum_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/md5sum"),
    });
    #[cfg(feature = "mdev")]
    appy_mcappface.push(bb_applet {
      name: "mdev",
      main: "mdev",
      entrypoint: mdev_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/mdev"),
    });
    #[cfg(feature = "mesg")]
    appy_mcappface.push(bb_applet {
      name: "mesg",
      main: "mesg",
      entrypoint: mesg_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/mesg"),
    });
    #[cfg(feature = "microcom")]
    appy_mcappface.push(bb_applet {
      name: "microcom",
      main: "microcom",
      entrypoint: microcom_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/microcom"),
    });
    #[cfg(feature = "mkdir")]
    appy_mcappface.push(bb_applet {
      name: "mkdir",
      main: "mkdir",
      entrypoint: mkdir_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/mkdir"),
    });
    #[cfg(feature = "mkdosfs")]
    appy_mcappface.push(bb_applet {
      name: "mkdosfs",
      main: "mkfs_vfat",
      entrypoint: mkfs_vfat_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/mkdosfs"),
    });
    #[cfg(feature = "mke2fs")]
    appy_mcappface.push(bb_applet {
      name: "mke2fs",
      main: "mkfs_ext2",
      entrypoint: mkfs_ext2_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/mke2fs"),
    });
    #[cfg(feature = "mkfifo")]
    appy_mcappface.push(bb_applet {
      name: "mkfifo",
      main: "mkfifo",
      entrypoint: mkfifo_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/mkfifo"),
    });
    #[cfg(feature = "mkfs_ext2")]
    appy_mcappface.push(bb_applet {
      name: "mkfs.ext2",
      main: "mkfs_ext2",
      entrypoint: mkfs_ext2_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/mkfs.ext2"),
    });
    #[cfg(feature = "mkfs_minix")]
    appy_mcappface.push(bb_applet {
      name: "mkfs.minix",
      main: "mkfs_minix",
      entrypoint: mkfs_minix_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/mkfs.minix"),
    });
    #[cfg(feature = "mkfs_vfat")]
    appy_mcappface.push(bb_applet {
      name: "mkfs.vfat",
      main: "mkfs_vfat",
      entrypoint: mkfs_vfat_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/mkfs.vfat"),
    });
    #[cfg(feature = "mknod")]
    appy_mcappface.push(bb_applet {
      name: "mknod",
      main: "mknod",
      entrypoint: mknod_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/mknod"),
    });
    #[cfg(feature = "mkpasswd")]
    appy_mcappface.push(bb_applet {
      name: "mkpasswd",
      main: "cryptpw",
      entrypoint: cryptpw_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/mkpasswd"),
    });
    #[cfg(feature = "mkswap")]
    appy_mcappface.push(bb_applet {
      name: "mkswap",
      main: "mkswap",
      entrypoint: mkswap_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/mkswap"),
    });
    #[cfg(feature = "mktemp")]
    appy_mcappface.push(bb_applet {
      name: "mktemp",
      main: "mktemp",
      entrypoint: mktemp_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/mktemp"),
    });
    #[cfg(feature = "modinfo")]
    appy_mcappface.push(bb_applet {
      name: "modinfo",
      main: "modinfo",
      entrypoint: modinfo_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/modinfo"),
    });
    #[cfg(feature = "modprobe")]
    appy_mcappface.push(bb_applet {
      name: "modprobe",
      main: "modprobe",
      entrypoint: modprobe_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/modprobe"),
    });
    #[cfg(feature = "more")]
    appy_mcappface.push(bb_applet {
      name: "more",
      main: "more",
      entrypoint: more_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/more"),
    });
    #[cfg(feature = "mount")]
    appy_mcappface.push(bb_applet {
      name: "mount",
      main: "mount",
      entrypoint: mount_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_MAYBE,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/mount"),
    });
    #[cfg(feature = "mountpoint")]
    appy_mcappface.push(bb_applet {
      name: "mountpoint",
      main: "mountpoint",
      entrypoint: mountpoint_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/mountpoint"),
    });
    #[cfg(feature = "mpstat")]
    appy_mcappface.push(bb_applet {
      name: "mpstat",
      main: "mpstat",
      entrypoint: mpstat_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/mpstat"),
    });
    #[cfg(feature = "mt")]
    appy_mcappface.push(bb_applet {
      name: "mt",
      main: "mt",
      entrypoint: mt_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/mt"),
    });
    #[cfg(feature = "mv")]
    appy_mcappface.push(bb_applet {
      name: "mv",
      main: "mv",
      entrypoint: mv_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/mv"),
    });
    #[cfg(feature = "nameif")]
    appy_mcappface.push(bb_applet {
      name: "nameif",
      main: "nameif",
      entrypoint: nameif_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/nameif"),
    });
    #[cfg(feature = "nanddump")]
    appy_mcappface.push(bb_applet {
      name: "nanddump",
      main: "nandwrite",
      entrypoint: nandwrite_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/nanddump"),
    });
    #[cfg(feature = "nandwrite")]
    appy_mcappface.push(bb_applet {
      name: "nandwrite",
      main: "nandwrite",
      entrypoint: nandwrite_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/nandwrite"),
    });
    #[cfg(feature = "nbd-client")]
    appy_mcappface.push(bb_applet {
      name: "nbd-client",
      main: "nbdclient",
      entrypoint: nbdclient_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/nbd-client"),
    });
    #[cfg(feature = "nc")]
    appy_mcappface.push(bb_applet {
      name: "nc",
      main: "nc",
      entrypoint: nc_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/nc"),
    });
    #[cfg(feature = "netstat")]
    appy_mcappface.push(bb_applet {
      name: "netstat",
      main: "netstat",
      entrypoint: netstat_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/netstat"),
    });
    #[cfg(feature = "nice")]
    appy_mcappface.push(bb_applet {
      name: "nice",
      main: "nice",
      entrypoint: nice_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/nice"),
    });
    #[cfg(feature = "nl")]
    appy_mcappface.push(bb_applet {
      name: "nl",
      main: "nl",
      entrypoint: nl_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/nl"),
    });
    #[cfg(feature = "nmeter")]
    appy_mcappface.push(bb_applet {
      name: "nmeter",
      main: "nmeter",
      entrypoint: nmeter_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/nmeter"),
    });
    #[cfg(feature = "nohup")]
    appy_mcappface.push(bb_applet {
      name: "nohup",
      main: "nohup",
      entrypoint: nohup_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/nohup"),
    });
    #[cfg(feature = "nproc")]
    appy_mcappface.push(bb_applet {
      name: "nproc",
      main: "nproc",
      entrypoint: nproc_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/nproc"),
    });
    #[cfg(feature = "nsenter")]
    appy_mcappface.push(bb_applet {
      name: "nsenter",
      main: "nsenter",
      entrypoint: nsenter_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/nsenter"),
    });
    #[cfg(feature = "nslookup")]
    appy_mcappface.push(bb_applet {
      name: "nslookup",
      main: "nslookup",
      entrypoint: nslookup_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/nslookup"),
    });
    #[cfg(feature = "ntpd")]
    appy_mcappface.push(bb_applet {
      name: "ntpd",
      main: "ntpd",
      entrypoint: ntpd_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/ntpd"),
    });
    #[cfg(feature = "nuke")]
    appy_mcappface.push(bb_applet {
      name: "nuke",
      main: "nuke",
      entrypoint: nuke_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/nuke"),
    });
    #[cfg(feature = "od")]
    appy_mcappface.push(bb_applet {
      name: "od",
      main: "od",
      entrypoint: od_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/od"),
    });
    #[cfg(feature = "openvt")]
    appy_mcappface.push(bb_applet {
      name: "openvt",
      main: "openvt",
      entrypoint: openvt_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/openvt"),
    });
    #[cfg(feature = "partprobe")]
    appy_mcappface.push(bb_applet {
      name: "partprobe",
      main: "partprobe",
      entrypoint: partprobe_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/partprobe"),
    });
    #[cfg(feature = "passwd")]
    appy_mcappface.push(bb_applet {
      name: "passwd",
      main: "passwd",
      entrypoint: passwd_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_REQUIRE,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/passwd"),
    });
    #[cfg(feature = "paste")]
    appy_mcappface.push(bb_applet {
      name: "paste",
      main: "paste",
      entrypoint: paste_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/paste"),
    });
    #[cfg(feature = "patch")]
    appy_mcappface.push(bb_applet {
      name: "patch",
      main: "patch",
      entrypoint: patch_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/patch"),
    });
    #[cfg(feature = "pgrep")]
    appy_mcappface.push(bb_applet {
      name: "pgrep",
      main: "pgrep",
      entrypoint: pgrep_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/pgrep"),
    });
    #[cfg(feature = "pidof")]
    appy_mcappface.push(bb_applet {
      name: "pidof",
      main: "pidof",
      entrypoint: pidof_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/pidof"),
    });
    #[cfg(feature = "ping")]
    appy_mcappface.push(bb_applet {
      name: "ping",
      main: "ping",
      entrypoint: ping_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_MAYBE,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/ping"),
    });
    #[cfg(feature = "ping6")]
    appy_mcappface.push(bb_applet {
      name: "ping6",
      main: "ping6",
      entrypoint: ping6_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_MAYBE,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/ping6"),
    });
    #[cfg(feature = "pipe_progress")]
    appy_mcappface.push(bb_applet {
      name: "pipe_progress",
      main: "pipe_progress",
      entrypoint: pipe_progress_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/pipe_progress"),
    });
    #[cfg(feature = "pivot_root")]
    appy_mcappface.push(bb_applet {
      name: "pivot_root",
      main: "pivot_root",
      entrypoint: pivot_root_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/pivot_root"),
    });
    #[cfg(feature = "pkill")]
    appy_mcappface.push(bb_applet {
      name: "pkill",
      main: "pgrep",
      entrypoint: pgrep_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/pkill"),
    });
    #[cfg(feature = "pmap")]
    appy_mcappface.push(bb_applet {
      name: "pmap",
      main: "pmap",
      entrypoint: pmap_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/pmap"),
    });
    #[cfg(feature = "popmaildir")]
    appy_mcappface.push(bb_applet {
      name: "popmaildir",
      main: "popmaildir",
      entrypoint: popmaildir_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/popmaildir"),
    });
    #[cfg(feature = "poweroff")]
    appy_mcappface.push(bb_applet {
      name: "poweroff",
      main: "halt",
      entrypoint: halt_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/poweroff"),
    });
    #[cfg(feature = "powertop")]
    appy_mcappface.push(bb_applet {
      name: "powertop",
      main: "powertop",
      entrypoint: powertop_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/powertop"),
    });
    #[cfg(feature = "printenv")]
    appy_mcappface.push(bb_applet {
      name: "printenv",
      main: "printenv",
      entrypoint: printenv_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/printenv"),
    });
    #[cfg(feature = "printf")]
    appy_mcappface.push(bb_applet {
      name: "printf",
      main: "printf",
      entrypoint: printf_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/printf"),
    });
    #[cfg(feature = "ps")]
    appy_mcappface.push(bb_applet {
      name: "ps",
      main: "ps",
      entrypoint: ps_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/ps"),
    });
    #[cfg(feature = "pscan")]
    appy_mcappface.push(bb_applet {
      name: "pscan",
      main: "pscan",
      entrypoint: pscan_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/pscan"),
    });
    #[cfg(feature = "pstree")]
    appy_mcappface.push(bb_applet {
      name: "pstree",
      main: "pstree",
      entrypoint: pstree_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/pstree"),
    });
    #[cfg(feature = "pwd")]
    appy_mcappface.push(bb_applet {
      name: "pwd",
      main: "pwd",
      entrypoint: pwd_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/pwd"),
    });
    #[cfg(feature = "pwdx")]
    appy_mcappface.push(bb_applet {
      name: "pwdx",
      main: "pwdx",
      entrypoint: pwdx_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/pwdx"),
    });
    #[cfg(feature = "raidautorun")]
    appy_mcappface.push(bb_applet {
      name: "raidautorun",
      main: "raidautorun",
      entrypoint: raidautorun_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/raidautorun"),
    });
    #[cfg(feature = "rdate")]
    appy_mcappface.push(bb_applet {
      name: "rdate",
      main: "rdate",
      entrypoint: rdate_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/rdate"),
    });
    #[cfg(feature = "rdev")]
    appy_mcappface.push(bb_applet {
      name: "rdev",
      main: "rdev",
      entrypoint: rdev_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/rdev"),
    });
    #[cfg(feature = "readahead")]
    appy_mcappface.push(bb_applet {
      name: "readahead",
      main: "readahead",
      entrypoint: readahead_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/readahead"),
    });
    #[cfg(feature = "readlink")]
    appy_mcappface.push(bb_applet {
      name: "readlink",
      main: "readlink",
      entrypoint: readlink_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/readlink"),
    });
    #[cfg(feature = "readprofile")]
    appy_mcappface.push(bb_applet {
      name: "readprofile",
      main: "readprofile",
      entrypoint: readprofile_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/readprofile"),
    });
    #[cfg(feature = "realpath")]
    appy_mcappface.push(bb_applet {
      name: "realpath",
      main: "realpath",
      entrypoint: realpath_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/realpath"),
    });
    #[cfg(feature = "reboot")]
    appy_mcappface.push(bb_applet {
      name: "reboot",
      main: "halt",
      entrypoint: halt_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/reboot"),
    });
    #[cfg(feature = "reformime")]
    appy_mcappface.push(bb_applet {
      name: "reformime",
      main: "reformime",
      entrypoint: reformime_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/reformime"),
    });
    #[cfg(feature = "remove-shell")]
    appy_mcappface.push(bb_applet {
      name: "remove-shell",
      main: "add_remove_shell",
      entrypoint: add_remove_shell_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/remove-shell"),
    });
    #[cfg(feature = "renice")]
    appy_mcappface.push(bb_applet {
      name: "renice",
      main: "renice",
      entrypoint: renice_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/renice"),
    });
    #[cfg(feature = "reset")]
    appy_mcappface.push(bb_applet {
      name: "reset",
      main: "reset",
      entrypoint: reset_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/reset"),
    });
    #[cfg(feature = "resize")]
    appy_mcappface.push(bb_applet {
      name: "resize",
      main: "resize",
      entrypoint: resize_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/resize"),
    });
    #[cfg(feature = "resume")]
    appy_mcappface.push(bb_applet {
      name: "resume",
      main: "resume",
      entrypoint: resume_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/resume"),
    });
    #[cfg(feature = "rev")]
    appy_mcappface.push(bb_applet {
      name: "rev",
      main: "rev",
      entrypoint: rev_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/rev"),
    });
    #[cfg(feature = "rm")]
    appy_mcappface.push(bb_applet {
      name: "rm",
      main: "rm",
      entrypoint: rm_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/rm"),
    });
    #[cfg(feature = "rmdir")]
    appy_mcappface.push(bb_applet {
      name: "rmdir",
      main: "rmdir",
      entrypoint: rmdir_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/rmdir"),
    });
    #[cfg(feature = "rmmod")]
    appy_mcappface.push(bb_applet {
      name: "rmmod",
      main: "modprobe",
      entrypoint: modprobe_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/rmmod"),
    });
    #[cfg(feature = "route")]
    appy_mcappface.push(bb_applet {
      name: "route",
      main: "route",
      entrypoint: route_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/route"),
    });
    #[cfg(feature = "rpm")]
    appy_mcappface.push(bb_applet {
      name: "rpm",
      main: "rpm",
      entrypoint: rpm_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/rpm"),
    });
    #[cfg(feature = "rpm2cpio")]
    appy_mcappface.push(bb_applet {
      name: "rpm2cpio",
      main: "rpm2cpio",
      entrypoint: rpm2cpio_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/rpm2cpio"),
    });
    #[cfg(feature = "rtcwake")]
    appy_mcappface.push(bb_applet {
      name: "rtcwake",
      main: "rtcwake",
      entrypoint: rtcwake_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/rtcwake"),
    });
    #[cfg(feature = "run-init")]
    appy_mcappface.push(bb_applet {
      name: "run-init",
      main: "switch_root",
      entrypoint: switch_root_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/run-init"),
    });
    #[cfg(feature = "run-parts")]
    appy_mcappface.push(bb_applet {
      name: "run-parts",
      main: "run_parts",
      entrypoint: run_parts_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/run-parts"),
    });
    #[cfg(feature = "runlevel")]
    appy_mcappface.push(bb_applet {
      name: "runlevel",
      main: "runlevel",
      entrypoint: runlevel_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/runlevel"),
    });
    #[cfg(feature = "runsv")]
    appy_mcappface.push(bb_applet {
      name: "runsv",
      main: "runsv",
      entrypoint: runsv_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/runsv"),
    });
    #[cfg(feature = "runsvdir")]
    appy_mcappface.push(bb_applet {
      name: "runsvdir",
      main: "runsvdir",
      entrypoint: runsvdir_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/runsvdir"),
    });
    #[cfg(feature = "rx")]
    appy_mcappface.push(bb_applet {
      name: "rx",
      main: "rx",
      entrypoint: rx_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/rx"),
    });
    #[cfg(feature = "script")]
    appy_mcappface.push(bb_applet {
      name: "script",
      main: "script",
      entrypoint: script_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/script"),
    });
    #[cfg(feature = "scriptreplay")]
    appy_mcappface.push(bb_applet {
      name: "scriptreplay",
      main: "scriptreplay",
      entrypoint: scriptreplay_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/scriptreplay"),
    });
    #[cfg(feature = "sed")]
    appy_mcappface.push(bb_applet {
      name: "sed",
      main: "sed",
      entrypoint: sed_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/sed"),
    });
    #[cfg(feature = "sendmail")]
    appy_mcappface.push(bb_applet {
      name: "sendmail",
      main: "sendmail",
      entrypoint: sendmail_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/sendmail"),
    });
    #[cfg(feature = "seq")]
    appy_mcappface.push(bb_applet {
      name: "seq",
      main: "seq",
      entrypoint: seq_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/seq"),
    });
    #[cfg(feature = "setarch")]
    appy_mcappface.push(bb_applet {
      name: "setarch",
      main: "setarch",
      entrypoint: setarch_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/setarch"),
    });
    #[cfg(feature = "setconsole")]
    appy_mcappface.push(bb_applet {
      name: "setconsole",
      main: "setconsole",
      entrypoint: setconsole_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/setconsole"),
    });
    #[cfg(feature = "setfattr")]
    appy_mcappface.push(bb_applet {
      name: "setfattr",
      main: "setfattr",
      entrypoint: setfattr_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/setfattr"),
    });
    #[cfg(feature = "setfont")]
    appy_mcappface.push(bb_applet {
      name: "setfont",
      main: "setfont",
      entrypoint: setfont_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/setfont"),
    });
    #[cfg(feature = "setkeycodes")]
    appy_mcappface.push(bb_applet {
      name: "setkeycodes",
      main: "setkeycodes",
      entrypoint: setkeycodes_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/setkeycodes"),
    });
    #[cfg(feature = "setlogcons")]
    appy_mcappface.push(bb_applet {
      name: "setlogcons",
      main: "setlogcons",
      entrypoint: setlogcons_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/setlogcons"),
    });
    #[cfg(feature = "setpriv")]
    appy_mcappface.push(bb_applet {
      name: "setpriv",
      main: "setpriv",
      entrypoint: setpriv_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/setpriv"),
    });
    #[cfg(feature = "setserial")]
    appy_mcappface.push(bb_applet {
      name: "setserial",
      main: "setserial",
      entrypoint: setserial_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/setserial"),
    });
    #[cfg(feature = "setsid")]
    appy_mcappface.push(bb_applet {
      name: "setsid",
      main: "setsid",
      entrypoint: setsid_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/setsid"),
    });
    #[cfg(feature = "setuidgid")]
    appy_mcappface.push(bb_applet {
      name: "setuidgid",
      main: "chpst",
      entrypoint: chpst_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/setuidgid"),
    });
    #[cfg(feature = "sh")]
    appy_mcappface.push(bb_applet {
      name: "sh",
      main: "ash",
      entrypoint: ash_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/sh"),
    });
    #[cfg(feature = "sha1sum")]
    appy_mcappface.push(bb_applet {
      name: "sha1sum",
      main: "md5_sha1_sum",
      entrypoint: md5_sha1_sum_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/sha1sum"),
    });
    #[cfg(feature = "sha256sum")]
    appy_mcappface.push(bb_applet {
      name: "sha256sum",
      main: "md5_sha1_sum",
      entrypoint: md5_sha1_sum_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/sha256sum"),
    });
    #[cfg(feature = "sha3sum")]
    appy_mcappface.push(bb_applet {
      name: "sha3sum",
      main: "md5_sha1_sum",
      entrypoint: md5_sha1_sum_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/sha3sum"),
    });
    #[cfg(feature = "sha512sum")]
    appy_mcappface.push(bb_applet {
      name: "sha512sum",
      main: "md5_sha1_sum",
      entrypoint: md5_sha1_sum_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/sha512sum"),
    });
    #[cfg(feature = "showkey")]
    appy_mcappface.push(bb_applet {
      name: "showkey",
      main: "showkey",
      entrypoint: showkey_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/showkey"),
    });
    #[cfg(feature = "shred")]
    appy_mcappface.push(bb_applet {
      name: "shred",
      main: "shred",
      entrypoint: shred_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/shred"),
    });
    #[cfg(feature = "shuf")]
    appy_mcappface.push(bb_applet {
      name: "shuf",
      main: "shuf",
      entrypoint: shuf_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/shuf"),
    });
    #[cfg(feature = "slattach")]
    appy_mcappface.push(bb_applet {
      name: "slattach",
      main: "slattach",
      entrypoint: slattach_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/slattach"),
    });
    #[cfg(feature = "sleep")]
    appy_mcappface.push(bb_applet {
      name: "sleep",
      main: "sleep",
      entrypoint: sleep_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/sleep"),
    });
    #[cfg(feature = "smemcap")]
    appy_mcappface.push(bb_applet {
      name: "smemcap",
      main: "smemcap",
      entrypoint: smemcap_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/smemcap"),
    });
    #[cfg(feature = "softlimit")]
    appy_mcappface.push(bb_applet {
      name: "softlimit",
      main: "chpst",
      entrypoint: chpst_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/softlimit"),
    });
    #[cfg(feature = "sort")]
    appy_mcappface.push(bb_applet {
      name: "sort",
      main: "sort",
      entrypoint: sort_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/sort"),
    });
    #[cfg(feature = "split")]
    appy_mcappface.push(bb_applet {
      name: "split",
      main: "split",
      entrypoint: split_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/split"),
    });
    #[cfg(feature = "ssl_client")]
    appy_mcappface.push(bb_applet {
      name: "ssl_client",
      main: "ssl_client",
      entrypoint: ssl_client_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/ssl_client"),
    });
    #[cfg(feature = "start-stop-daemon")]
    appy_mcappface.push(bb_applet {
      name: "start-stop-daemon",
      main: "start_stop_daemon",
      entrypoint: start_stop_daemon_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/start-stop-daemon"),
    });
    #[cfg(feature = "stat")]
    appy_mcappface.push(bb_applet {
      name: "stat",
      main: "stat",
      entrypoint: stat_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/stat"),
    });
    #[cfg(feature = "strings")]
    appy_mcappface.push(bb_applet {
      name: "strings",
      main: "strings",
      entrypoint: strings_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/strings"),
    });
    #[cfg(feature = "stty")]
    appy_mcappface.push(bb_applet {
      name: "stty",
      main: "stty",
      entrypoint: stty_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/stty"),
    });
    #[cfg(feature = "su")]
    appy_mcappface.push(bb_applet {
      name: "su",
      main: "su",
      entrypoint: su_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_REQUIRE,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/su"),
    });
    #[cfg(feature = "sulogin")]
    appy_mcappface.push(bb_applet {
      name: "sulogin",
      main: "sulogin",
      entrypoint: sulogin_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/sulogin"),
    });
    #[cfg(feature = "sum")]
    appy_mcappface.push(bb_applet {
      name: "sum",
      main: "sum",
      entrypoint: sum_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/sum"),
    });
    #[cfg(feature = "sv")]
    appy_mcappface.push(bb_applet {
      name: "sv",
      main: "sv",
      entrypoint: sv_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/sv"),
    });
    #[cfg(feature = "svc")]
    appy_mcappface.push(bb_applet {
      name: "svc",
      main: "svc",
      entrypoint: svc_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/svc"),
    });
    #[cfg(feature = "svlogd")]
    appy_mcappface.push(bb_applet {
      name: "svlogd",
      main: "svlogd",
      entrypoint: svlogd_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/svlogd"),
    });
    #[cfg(feature = "svok")]
    appy_mcappface.push(bb_applet {
      name: "svok",
      main: "svok",
      entrypoint: svok_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/svok"),
    });
    #[cfg(feature = "swapoff")]
    appy_mcappface.push(bb_applet {
      name: "swapoff",
      main: "swap_on_off",
      entrypoint: swap_on_off_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/swapoff"),
    });
    #[cfg(feature = "swapon")]
    appy_mcappface.push(bb_applet {
      name: "swapon",
      main: "swap_on_off",
      entrypoint: swap_on_off_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/swapon"),
    });
    #[cfg(feature = "switch_root")]
    appy_mcappface.push(bb_applet {
      name: "switch_root",
      main: "switch_root",
      entrypoint: switch_root_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/switch_root"),
    });
    #[cfg(feature = "sync")]
    appy_mcappface.push(bb_applet {
      name: "sync",
      main: "sync",
      entrypoint: sync_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/sync"),
    });
    #[cfg(feature = "sysctl")]
    appy_mcappface.push(bb_applet {
      name: "sysctl",
      main: "sysctl",
      entrypoint: sysctl_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/sysctl"),
    });
    #[cfg(feature = "syslogd")]
    appy_mcappface.push(bb_applet {
      name: "syslogd",
      main: "syslogd",
      entrypoint: syslogd_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/syslogd"),
    });
    #[cfg(feature = "tac")]
    appy_mcappface.push(bb_applet {
      name: "tac",
      main: "tac",
      entrypoint: tac_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/tac"),
    });
    #[cfg(feature = "tail")]
    appy_mcappface.push(bb_applet {
      name: "tail",
      main: "tail",
      entrypoint: tail_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/tail"),
    });
    #[cfg(feature = "tar")]
    appy_mcappface.push(bb_applet {
      name: "tar",
      main: "tar",
      entrypoint: tar_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/tar"),
    });
    #[cfg(feature = "taskset")]
    appy_mcappface.push(bb_applet {
      name: "taskset",
      main: "taskset",
      entrypoint: taskset_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/taskset"),
    });
    #[cfg(feature = "tc")]
    appy_mcappface.push(bb_applet {
      name: "tc",
      main: "tc",
      entrypoint: tc_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/tc"),
    });
    #[cfg(feature = "tcpsvd")]
    appy_mcappface.push(bb_applet {
      name: "tcpsvd",
      main: "tcpudpsvd",
      entrypoint: tcpudpsvd_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/tcpsvd"),
    });
    #[cfg(feature = "tee")]
    appy_mcappface.push(bb_applet {
      name: "tee",
      main: "tee",
      entrypoint: tee_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/tee"),
    });
    #[cfg(feature = "telnet")]
    appy_mcappface.push(bb_applet {
      name: "telnet",
      main: "telnet",
      entrypoint: telnet_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/telnet"),
    });
    #[cfg(feature = "telnetd")]
    appy_mcappface.push(bb_applet {
      name: "telnetd",
      main: "telnetd",
      entrypoint: telnetd_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/telnetd"),
    });
    #[cfg(feature = "test")]
    appy_mcappface.push(bb_applet {
      name: "test",
      main: "test",
      entrypoint: test_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/test"),
    });
    #[cfg(feature = "tftp")]
    appy_mcappface.push(bb_applet {
      name: "tftp",
      main: "tftp",
      entrypoint: tftp_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/tftp"),
    });
    #[cfg(feature = "tftpd")]
    appy_mcappface.push(bb_applet {
      name: "tftpd",
      main: "tftpd",
      entrypoint: tftpd_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/tftpd"),
    });
    #[cfg(feature = "time")]
    appy_mcappface.push(bb_applet {
      name: "time",
      main: "time",
      entrypoint: time_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/time"),
    });
    #[cfg(feature = "timeout")]
    appy_mcappface.push(bb_applet {
      name: "timeout",
      main: "timeout",
      entrypoint: timeout_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/timeout"),
    });
    #[cfg(feature = "top")]
    appy_mcappface.push(bb_applet {
      name: "top",
      main: "top",
      entrypoint: top_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/top"),
    });
    #[cfg(feature = "touch")]
    appy_mcappface.push(bb_applet {
      name: "touch",
      main: "touch",
      entrypoint: touch_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/touch"),
    });
    #[cfg(feature = "tr")]
    appy_mcappface.push(bb_applet {
      name: "tr",
      main: "tr",
      entrypoint: tr_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/tr"),
    });
    #[cfg(feature = "traceroute")]
    appy_mcappface.push(bb_applet {
      name: "traceroute",
      main: "traceroute",
      entrypoint: traceroute_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_MAYBE,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/traceroute"),
    });
    #[cfg(feature = "traceroute6")]
    appy_mcappface.push(bb_applet {
      name: "traceroute6",
      main: "traceroute6",
      entrypoint: traceroute6_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_MAYBE,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/traceroute6"),
    });
    #[cfg(feature = "true")]
    appy_mcappface.push(bb_applet {
      name: "true",
      main: "true",
      entrypoint: true_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/true"),
    });
    #[cfg(feature = "truncate")]
    appy_mcappface.push(bb_applet {
      name: "truncate",
      main: "truncate",
      entrypoint: truncate_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/truncate"),
    });
    #[cfg(feature = "ts")]
    appy_mcappface.push(bb_applet {
      name: "ts",
      main: "ts",
      entrypoint: ts_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/ts"),
    });
    #[cfg(feature = "tty")]
    appy_mcappface.push(bb_applet {
      name: "tty",
      main: "tty",
      entrypoint: tty_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/tty"),
    });
    #[cfg(feature = "ttysize")]
    appy_mcappface.push(bb_applet {
      name: "ttysize",
      main: "ttysize",
      entrypoint: ttysize_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/ttysize"),
    });
    #[cfg(feature = "tunctl")]
    appy_mcappface.push(bb_applet {
      name: "tunctl",
      main: "tunctl",
      entrypoint: tunctl_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/tunctl"),
    });
    #[cfg(feature = "ubiattach")]
    appy_mcappface.push(bb_applet {
      name: "ubiattach",
      main: "ubi_tools",
      entrypoint: ubi_tools_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/ubiattach"),
    });
    #[cfg(feature = "ubidetach")]
    appy_mcappface.push(bb_applet {
      name: "ubidetach",
      main: "ubi_tools",
      entrypoint: ubi_tools_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/ubidetach"),
    });
    #[cfg(feature = "ubimkvol")]
    appy_mcappface.push(bb_applet {
      name: "ubimkvol",
      main: "ubi_tools",
      entrypoint: ubi_tools_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/ubimkvol"),
    });
    #[cfg(feature = "ubirename")]
    appy_mcappface.push(bb_applet {
      name: "ubirename",
      main: "ubirename",
      entrypoint: ubirename_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/ubirename"),
    });
    #[cfg(feature = "ubirmvol")]
    appy_mcappface.push(bb_applet {
      name: "ubirmvol",
      main: "ubi_tools",
      entrypoint: ubi_tools_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/ubirmvol"),
    });
    #[cfg(feature = "ubirsvol")]
    appy_mcappface.push(bb_applet {
      name: "ubirsvol",
      main: "ubi_tools",
      entrypoint: ubi_tools_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/ubirsvol"),
    });
    #[cfg(feature = "ubiupdatevol")]
    appy_mcappface.push(bb_applet {
      name: "ubiupdatevol",
      main: "ubi_tools",
      entrypoint: ubi_tools_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/ubiupdatevol"),
    });
    #[cfg(feature = "udhcpc")]
    appy_mcappface.push(bb_applet {
      name: "udhcpc",
      main: "udhcpc",
      entrypoint: udhcpc_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/udhcpc"),
    });
    #[cfg(feature = "udhcpc6")]
    appy_mcappface.push(bb_applet {
      name: "udhcpc6",
      main: "udhcpc6",
      entrypoint: udhcpc6_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/udhcpc6"),
    });
    #[cfg(feature = "udhcpd")]
    appy_mcappface.push(bb_applet {
      name: "udhcpd",
      main: "udhcpd",
      entrypoint: udhcpd_main,
      install_loc: InstallLoc::DIR_USR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/udhcpd"),
    });
    #[cfg(feature = "udpsvd")]
    appy_mcappface.push(bb_applet {
      name: "udpsvd",
      main: "tcpudpsvd",
      entrypoint: tcpudpsvd_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/udpsvd"),
    });
    #[cfg(feature = "uevent")]
    appy_mcappface.push(bb_applet {
      name: "uevent",
      main: "uevent",
      entrypoint: uevent_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/uevent"),
    });
    #[cfg(feature = "umount")]
    appy_mcappface.push(bb_applet {
      name: "umount",
      main: "umount",
      entrypoint: umount_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/umount"),
    });
    #[cfg(feature = "uname")]
    appy_mcappface.push(bb_applet {
      name: "uname",
      main: "uname",
      entrypoint: uname_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/uname"),
    });
    #[cfg(feature = "unexpand")]
    appy_mcappface.push(bb_applet {
      name: "unexpand",
      main: "expand",
      entrypoint: expand_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/unexpand"),
    });
    #[cfg(feature = "uniq")]
    appy_mcappface.push(bb_applet {
      name: "uniq",
      main: "uniq",
      entrypoint: uniq_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/uniq"),
    });
    #[cfg(feature = "unix2dos")]
    appy_mcappface.push(bb_applet {
      name: "unix2dos",
      main: "dos2unix",
      entrypoint: dos2unix_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/unix2dos"),
    });
    #[cfg(feature = "unlink")]
    appy_mcappface.push(bb_applet {
      name: "unlink",
      main: "unlink",
      entrypoint: unlink_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/unlink"),
    });
    #[cfg(feature = "unlzma")]
    appy_mcappface.push(bb_applet {
      name: "unlzma",
      main: "unlzma",
      entrypoint: unlzma_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/unlzma"),
    });
    #[cfg(feature = "unshare")]
    appy_mcappface.push(bb_applet {
      name: "unshare",
      main: "unshare",
      entrypoint: unshare_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/unshare"),
    });
    #[cfg(feature = "unxz")]
    appy_mcappface.push(bb_applet {
      name: "unxz",
      main: "unxz",
      entrypoint: unxz_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/unxz"),
    });
    #[cfg(feature = "unzip")]
    appy_mcappface.push(bb_applet {
      name: "unzip",
      main: "unzip",
      entrypoint: unzip_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/unzip"),
    });
    #[cfg(feature = "uptime")]
    appy_mcappface.push(bb_applet {
      name: "uptime",
      main: "uptime",
      entrypoint: uptime_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/uptime"),
    });
    #[cfg(feature = "users")]
    appy_mcappface.push(bb_applet {
      name: "users",
      main: "who",
      entrypoint: who_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/users"),
    });
    #[cfg(feature = "usleep")]
    appy_mcappface.push(bb_applet {
      name: "usleep",
      main: "usleep",
      entrypoint: usleep_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/usleep"),
    });
    #[cfg(feature = "uudecode")]
    appy_mcappface.push(bb_applet {
      name: "uudecode",
      main: "uudecode",
      entrypoint: uudecode_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/uudecode"),
    });
    #[cfg(feature = "uuencode")]
    appy_mcappface.push(bb_applet {
      name: "uuencode",
      main: "uuencode",
      entrypoint: uuencode_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/uuencode"),
    });
    #[cfg(feature = "vconfig")]
    appy_mcappface.push(bb_applet {
      name: "vconfig",
      main: "vconfig",
      entrypoint: vconfig_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/vconfig"),
    });
    #[cfg(feature = "vi")]
    appy_mcappface.push(bb_applet {
      name: "vi",
      main: "vi",
      entrypoint: vi_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/vi"),
    });
    #[cfg(feature = "vlock")]
    appy_mcappface.push(bb_applet {
      name: "vlock",
      main: "vlock",
      entrypoint: vlock_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_REQUIRE,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/vlock"),
    });
    #[cfg(feature = "volname")]
    appy_mcappface.push(bb_applet {
      name: "volname",
      main: "volname",
      entrypoint: volname_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/volname"),
    });
    #[cfg(feature = "w")]
    appy_mcappface.push(bb_applet {
      name: "w",
      main: "who",
      entrypoint: who_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/w"),
    });
    #[cfg(feature = "wall")]
    appy_mcappface.push(bb_applet {
      name: "wall",
      main: "wall",
      entrypoint: wall_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_REQUIRE,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/wall"),
    });
    #[cfg(feature = "watch")]
    appy_mcappface.push(bb_applet {
      name: "watch",
      main: "watch",
      entrypoint: watch_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/watch"),
    });
    #[cfg(feature = "watchdog")]
    appy_mcappface.push(bb_applet {
      name: "watchdog",
      main: "watchdog",
      entrypoint: watchdog_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/watchdog"),
    });
    #[cfg(feature = "wc")]
    appy_mcappface.push(bb_applet {
      name: "wc",
      main: "wc",
      entrypoint: wc_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/wc"),
    });
    #[cfg(feature = "wget")]
    appy_mcappface.push(bb_applet {
      name: "wget",
      main: "wget",
      entrypoint: wget_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/wget"),
    });
    #[cfg(feature = "which")]
    appy_mcappface.push(bb_applet {
      name: "which",
      main: "which",
      entrypoint: which_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/which"),
    });
    #[cfg(feature = "who")]
    appy_mcappface.push(bb_applet {
      name: "who",
      main: "who",
      entrypoint: who_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/who"),
    });
    #[cfg(feature = "whoami")]
    appy_mcappface.push(bb_applet {
      name: "whoami",
      main: "whoami",
      entrypoint: whoami_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: true,
      usage: std::include_str!("../usage/whoami"),
    });
    #[cfg(feature = "whois")]
    appy_mcappface.push(bb_applet {
      name: "whois",
      main: "whois",
      entrypoint: whois_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/whois"),
    });
    #[cfg(feature = "xargs")]
    appy_mcappface.push(bb_applet {
      name: "xargs",
      main: "xargs",
      entrypoint: xargs_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/xargs"),
    });
    #[cfg(feature = "xxd")]
    appy_mcappface.push(bb_applet {
      name: "xxd",
      main: "xxd",
      entrypoint: xxd_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/xxd"),
    });
    #[cfg(feature = "xz")]
    appy_mcappface.push(bb_applet {
      name: "xz",
      main: "unxz",
      entrypoint: unxz_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/xz"),
    });
    #[cfg(feature = "xzcat")]
    appy_mcappface.push(bb_applet {
      name: "xzcat",
      main: "unxz",
      entrypoint: unxz_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/xzcat"),
    });
    #[cfg(feature = "yes")]
    appy_mcappface.push(bb_applet {
      name: "yes",
      main: "yes",
      entrypoint: yes_main,
      install_loc: InstallLoc::DIR_USR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: true,
      nofork: false,
      usage: std::include_str!("../usage/yes"),
    });
    #[cfg(feature = "zcat")]
    appy_mcappface.push(bb_applet {
      name: "zcat",
      main: "gunzip",
      entrypoint: gunzip_main,
      install_loc: InstallLoc::DIR_BIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/zcat"),
    });
    #[cfg(feature = "zcip")]
    appy_mcappface.push(bb_applet {
      name: "zcip",
      main: "zcip",
      entrypoint: zcip_main,
      install_loc: InstallLoc::DIR_SBIN,
      need_suid: SUID::SUID_DROP,
      noexec: false,
      nofork: false,
      usage: std::include_str!("../usage/zcip"),
    });

    // See https://stackoverflow.com/questions/51272571/how-do-i-check-if-a-slice-is-sorted.
    // We use < and not <= since names should be unique.
    assert!(appy_mcappface.windows(2).all(|w| w[0].name < w[1].name));

    appy_mcappface
  };
}
