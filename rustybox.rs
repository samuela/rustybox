#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(asm)]
#![feature(c_variadic)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(main)]
#![feature(ptr_wrapping_offset_from)]

pub mod applets {
  pub mod applet_tables;
  pub mod applets;
  pub mod usage;
  pub mod usage_pod;
} // mod applets
pub mod archival {
  pub mod bbunzip;
  pub mod bzip2;
  pub mod cpio;
  pub mod dpkg;
  pub mod dpkg_deb;
  pub mod gzip;
  pub mod libarchive {
    pub mod common;
    pub mod data_align;
    pub mod data_extract_all;
    pub mod data_extract_to_command;
    pub mod data_extract_to_stdout;
    pub mod data_skip;
    pub mod decompress_bunzip2;
    pub mod decompress_gunzip;
    pub mod decompress_unlzma;
    pub mod decompress_unxz;
    pub mod filter_accept_all;
    pub mod filter_accept_list;
    pub mod filter_accept_list_reassign;
    pub mod filter_accept_reject_list;
    pub mod find_list_entry;
    pub mod get_header_ar;
    pub mod get_header_cpio;
    pub mod get_header_tar;
    pub mod get_header_tar_bz2;
    pub mod get_header_tar_gz;
    pub mod get_header_tar_lzma;
    pub mod get_header_tar_xz;
    pub mod header_list;
    pub mod header_skip;
    pub mod header_verbose_list;
    pub mod init_handle;
    pub mod lzo1x_1;
    pub mod lzo1x_1o;
    pub mod lzo1x_d;
    pub mod open_transformer;
    pub mod seek_by_jump;
    pub mod seek_by_read;
    pub mod unpack_ar_archive;
    pub mod unsafe_prefix;
    pub mod unsafe_symlink_target;
  } // mod libarchive
  pub mod lzop;
  pub mod rpm;
  pub mod tar;
  pub mod unzip;
} // mod archival
pub mod console_tools {
  pub mod chvt;
  pub mod clear;
  pub mod deallocvt;
  pub mod dumpkmap;
  pub mod fgconsole;
  pub mod kbd_mode;
  pub mod loadfont;
  pub mod loadkmap;
  pub mod openvt;
  pub mod reset;
  pub mod resize;
  pub mod setconsole;
  pub mod setkeycodes;
  pub mod setlogcons;
  pub mod showkey;
} // mod console_tools
pub mod coreutils {
  pub mod basename;
  pub mod cat;
  pub mod chgrp;
  pub mod chmod;
  pub mod chown;
  pub mod chroot;
  pub mod cksum;
  pub mod comm;
  pub mod cp;
  pub mod cut;
  pub mod date;
  pub mod dd;
  pub mod df;
  pub mod dirname;
  pub mod dos2unix;
  pub mod du;
  pub mod echo;
  pub mod env;
  pub mod expand;
  pub mod expr;
  pub mod factor;
  pub mod fold;
  pub mod head;
  pub mod hostid;
  pub mod id;
  pub mod install;
  pub mod libcoreutils {
    pub mod cp_mv_stat;
    pub mod getopt_mk_fifo_nod;
  } // mod libcoreutils
  pub mod r#false;
  pub mod link;
  pub mod ln;
  pub mod logname;
  pub mod ls;
  pub mod md5_sha1_sum;
  pub mod mkdir;
  pub mod mkfifo;
  pub mod mknod;
  pub mod mktemp;
  pub mod mv;
  pub mod nice;
  pub mod nl;
  pub mod nohup;
  pub mod nproc;
  pub mod od;
  pub mod paste;
  pub mod printenv;
  pub mod printf;
  pub mod pwd;
  pub mod readlink;
  pub mod realpath;
  pub mod rm;
  pub mod rmdir;
  pub mod seq;
  pub mod shred;
  pub mod shuf;
  pub mod sleep;
  pub mod sort;
  pub mod split;
  pub mod stat;
  pub mod stty;
  pub mod sum;
  pub mod sync;
  pub mod tac;
  pub mod tail;
  pub mod tee;
  pub mod test;
  pub mod test_ptr_hack;
  pub mod timeout;
  pub mod touch;
  pub mod tr;
  pub mod r#true;
  pub mod truncate;
  pub mod tty;
  pub mod uname;
  pub mod uniq;
  pub mod unlink;
  pub mod usleep;
  pub mod uudecode;
  pub mod uuencode;
  pub mod wc;
  pub mod who;
  pub mod whoami;
  pub mod yes;
} // mod coreutils
pub mod debianutils {
  pub mod pipe_progress;
  pub mod run_parts;
  pub mod start_stop_daemon;
  pub mod which;
} // mod debianutils
pub mod e2fsprogs {
  pub mod chattr;
  pub mod e2fs_lib;
  pub mod fsck;
  pub mod lsattr;
} // mod e2fsprogs
pub mod editors {
  pub mod awk;
  pub mod cmp;
  pub mod diff;
  pub mod ed;
  pub mod patch;
  pub mod sed;
  pub mod vi;
} // mod editors
pub mod findutils {
  pub mod find;
  pub mod grep;
  pub mod xargs;
} // mod findutils
pub mod init {
  pub mod bootchartd;
  pub mod halt;
  pub mod init;
} // mod init
pub mod klibc_utils {
  pub mod nuke;
  pub mod resume;
} // mod klibc_utils
pub mod libbb {
  pub mod appletlib;
  pub mod ask_confirmation;
  pub mod auto_string;
  pub mod bb_askpass;
  pub mod bb_bswap_64;
  pub mod bb_cat;
  pub mod bb_do_delay;
  pub mod bb_getgroups;
  pub mod bb_getsockname;
  pub mod bb_pwd;
  pub mod bb_qsort;
  pub mod bb_strtonum;
  pub mod capability;
  pub mod change_identity;
  pub mod chomp;
  pub mod common_bufsiz;
  pub mod compare_string_array;
  pub mod concat_path_file;
  pub mod concat_subpath_file;
  pub mod copy_file;
  pub mod copyfd;
  pub mod correct_password;
  pub mod crc32;
  pub mod default_error_retval;
  pub mod device_open;
  pub mod dump;
  pub mod duration;
  pub mod endofname;
  pub mod executable;
  pub mod fclose_nonstdin;
  pub mod fflush_stdout_and_exit;
  pub mod fgets_str;
  pub mod find_mount_point;
  pub mod find_pid_by_name;
  pub mod find_root_device;
  pub mod full_write;
  pub mod get_console;
  pub mod get_cpu_count;
  pub mod get_last_path_component;
  pub mod get_line_from_file;
  pub mod get_shell_name;
  pub mod get_volsize;
  pub mod getopt32;
  pub mod getopt_allopts;
  pub mod getpty;
  pub mod hash_md5_sha;
  pub mod herror_msg;
  pub mod human_readable;
  pub mod in_ether;
  pub mod inet_cksum;
  pub mod inet_common;
  pub mod inode_hash;
  pub mod isdirectory;
  pub mod isqrt;
  pub mod kernel_version;
  pub mod last_char_is;
  pub mod lineedit;
  pub mod lineedit_ptr_hack;
  pub mod llist;
  pub mod logenv;
  pub mod login;
  pub mod r#loop;
  pub mod make_directory;
  pub mod makedev;
  pub mod match_fstype;
  pub mod messages;
  pub mod missing_syscalls;
  pub mod mode_string;
  pub mod nuke_str;
  pub mod obscure;
  pub mod parse_config;
  pub mod parse_mode;
  pub mod percent_decode;
  pub mod perror_msg;
  pub mod perror_nomsg;
  pub mod perror_nomsg_and_die;
  pub mod pidfile;
  pub mod platform;
  pub mod print_flags;
  pub mod print_numbered_lines;
  pub mod printable;
  pub mod printable_string;
  pub mod process_escape_sequence;
  pub mod procps;
  pub mod progress;
  pub mod ptr_to_globals;
  pub mod pw_encrypt;
  pub mod read;
  pub mod read_key;
  pub mod read_printf;
  pub mod recursive_action;
  pub mod remove_file;
  pub mod replace;
  pub mod rtc;
  pub mod run_shell;
  pub mod safe_gethostname;
  pub mod safe_poll;
  pub mod safe_strncpy;
  pub mod safe_write;
  pub mod securetty;
  pub mod setup_environment;
  pub mod signals;
  pub mod simplify_path;
  pub mod single_argv;
  pub mod skip_whitespace;
  pub mod speed_table;
  pub mod str_tolower;
  pub mod strrstr;
  pub mod sysconf;
  pub mod time;
  pub mod trim;
  pub mod u_signal_names;
  pub mod ubi;
  pub mod udp_io;
  pub mod unicode;
  pub mod update_passwd;
  pub mod utmp;
  pub mod uuencode;
  pub mod verror_msg;
  pub mod vfork_daemon_rexec;
  pub mod warn_ignoring_args;
  pub mod wfopen;
  pub mod wfopen_input;
  pub mod write;
  pub mod xatonum;
  pub mod xconnect;
  pub mod xfunc_die;
  pub mod xfuncs;
  pub mod xfuncs_printf;
  pub mod xgetcwd;
  pub mod xgethostbyname;
  pub mod xreadlink;
  pub mod xrealloc_vector;
  pub mod xregcomp;
} // mod libbb
pub mod libpwdgrp {
  pub mod pwd_grp;
  pub mod uidgid_get;
} // mod libpwdgrp
pub mod loginutils {
  pub mod add_remove_shell;
  pub mod addgroup;
  pub mod adduser;
  pub mod chpasswd;
  pub mod cryptpw;
  pub mod deluser;
  pub mod getty;
  pub mod login;
  pub mod passwd;
  pub mod su;
  pub mod sulogin;
  pub mod vlock;
} // mod loginutils
pub mod mailutils {
  pub mod mail;
  pub mod makemime;
  pub mod popmaildir;
  pub mod reformime;
  pub mod sendmail;
} // mod mailutils
pub mod miscutils {
  pub mod adjtimex;
  pub mod bc;
  pub mod beep;
  pub mod chat;
  pub mod conspy;
  pub mod crond;
  pub mod crontab;
  pub mod devmem;
  pub mod fbsplash;
  pub mod hdparm;
  pub mod hexedit;
  pub mod i2c_tools;
  pub mod less;
  pub mod lsscsi;
  pub mod makedevs;
  pub mod man;
  pub mod microcom;
  pub mod mt;
  pub mod nandwrite;
  pub mod partprobe;
  pub mod raidautorun;
  pub mod readahead;
  pub mod runlevel;
  pub mod rx;
  pub mod setfattr;
  pub mod setserial;
  pub mod strings;
  pub mod time;
  pub mod ts;
  pub mod ttysize;
  pub mod ubi_tools;
  pub mod ubirename;
  pub mod volname;
  pub mod watchdog;
} // mod miscutils
pub mod modutils {
  pub mod modinfo;
  pub mod modprobe_small;
  pub mod modutils;
} // mod modutils
pub mod networking {
  pub mod arp;
  pub mod arping;
  pub mod brctl;
  pub mod dnsd;
  pub mod ether_wake;
  pub mod ftpd;
  pub mod ftpgetput;
  pub mod hostname;
  pub mod httpd;
  pub mod ifconfig;
  pub mod ifenslave;
  pub mod ifplugd;
  pub mod ifupdown;
  pub mod inetd;
  pub mod interface;
  pub mod ip;
  pub mod ipcalc;
  pub mod isrv;
  pub mod isrv_identd;
  pub mod libiproute {
    pub mod ip_parse_common_args;
    pub mod ipaddress;
    pub mod iplink;
    pub mod ipneigh;
    pub mod iproute;
    pub mod iprule;
    pub mod iptunnel;
    pub mod libnetlink;
    pub mod ll_addr;
    pub mod ll_map;
    pub mod ll_proto;
    pub mod ll_types;
    pub mod rt_names;
    pub mod rtm_map;
    pub mod utils;
  } // mod libiproute
  pub mod nameif;
  pub mod nbd_client;
  pub mod nc;
  pub mod netstat;
  pub mod nslookup;
  pub mod ntpd;
  pub mod parse_pasv_epsv;
  pub mod ping;
  pub mod pscan;
  pub mod route;
  pub mod slattach;
  pub mod ssl_client;
  pub mod tc;
  pub mod tcpudp;
  pub mod tcpudp_perhost;
  pub mod telnet;
  pub mod telnetd;
  pub mod tftp;
  pub mod tls;
  pub mod tls_aes;
  pub mod tls_aesgcm;
  pub mod tls_fe;
  pub mod tls_pstm;
  pub mod tls_pstm_montgomery_reduce;
  pub mod tls_pstm_mul_comba;
  pub mod tls_pstm_sqr_comba;
  pub mod tls_rsa;
  pub mod traceroute;
  pub mod tunctl;
  pub mod udhcp {
    pub mod arpping;
    pub mod common;
    pub mod d6_dhcpc;
    pub mod d6_packet;
    pub mod d6_socket;
    pub mod dhcpc;
    pub mod dhcpd;
    pub mod dhcprelay;
    pub mod domain_codec;
    pub mod dumpleases;
    pub mod packet;
    pub mod signalpipe;
    pub mod socket;
  } // mod udhcp
  pub mod vconfig;
  pub mod wget;
  pub mod whois;
  pub mod zcip;
} // mod networking
pub mod printutils {
  pub mod lpd;
  pub mod lpr;
} // mod printutils
pub mod procps {
  pub mod free;
  pub mod fuser;
  pub mod iostat;
  pub mod kill;
  pub mod lsof;
  pub mod mpstat;
  pub mod nmeter;
  pub mod pgrep;
  pub mod pidof;
  pub mod pmap;
  pub mod powertop;
  pub mod ps;
  pub mod pstree;
  pub mod pwdx;
  pub mod smemcap;
  pub mod sysctl;
  pub mod top;
  pub mod uptime;
  pub mod watch;
} // mod procps
pub mod runit {
  pub mod chpst;
  pub mod runsv;
  pub mod runsvdir;
  pub mod sv;
  pub mod svlogd;
} // mod runit
pub mod shell {
  pub mod ash;
  pub mod ash_ptr_hack;
  pub mod cttyhack;
  pub mod hush;
  pub mod r#match;
  pub mod math;
  pub mod random;
  pub mod shell_common;
} // mod shell
pub mod sysklogd {
  pub mod klogd;
  pub mod logread;
  pub mod syslogd_and_logger;
} // mod sysklogd
pub mod util_linux;

fn main() {
  unsafe { libbb::appletlib::main() }
}
