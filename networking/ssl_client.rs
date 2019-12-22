use crate::librb::tls_state;
use libc;
extern "C" {
  pub type tls_handshake_data;
}

pub type tls_state_t = tls_state;
#[inline]
unsafe extern "C" fn new_tls_state() -> *mut tls_state_t {
  let mut tls: *mut tls_state_t = crate::libbb::xfuncs_printf::xzalloc(::std::mem::size_of::<
    tls_state_t,
  >() as libc::c_ulong) as *mut tls_state_t;
  return tls;
}
/*
 * Copyright (C) 2017 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config SSL_CLIENT
//config:	bool "ssl_client (25 kb)"
//config:	default y
//config:	select TLS
//config:	help
//config:	This tool pipes data to/from a socket, TLS-encrypting it.
//applet:IF_SSL_CLIENT(APPLET(ssl_client, BB_DIR_USR_BIN, SUID_DROP))
//kbuild:lib-$(CONFIG_SSL_CLIENT) += ssl_client.o
//usage:#define ssl_client_trivial_usage
//usage:       "[-e] -s FD [-r FD] [-n SNI]"
//usage:#define ssl_client_full_usage ""
#[no_mangle]
pub unsafe extern "C" fn ssl_client_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut tls: *mut tls_state_t = std::ptr::null_mut();
  let mut sni: *const libc::c_char = 0 as *const libc::c_char;
  let mut opt: libc::c_int = 0;
  // INIT_G();
  tls = new_tls_state();
  opt = crate::libbb::getopt32::getopt32(
    argv,
    b"es:+r:+n:\x00" as *const u8 as *const libc::c_char,
    &mut (*tls).ofd as *mut libc::c_int,
    &mut (*tls).ifd as *mut libc::c_int,
    &mut sni as *mut *const libc::c_char,
  ) as libc::c_int;
  if opt & 1i32 << 2i32 == 0 {
    /* -r N defaults to -s N */
    (*tls).ifd = (*tls).ofd
  }
  if opt & 3i32 << 1i32 == 0 {
    if (*argv.offset(1)).is_null() {
      crate::libbb::appletlib::bb_show_usage();
    }
    /* Undocumented debug feature: without -s and -r, takes HOST arg and connects to it */
    //
    // Talk to kernel.org:
    // printf "GET / HTTP/1.1\r\nHost: kernel.org\r\n\r\n" | busybox ssl_client kernel.org
    if sni.is_null() {
      sni = *argv.offset(1)
    }
    (*tls).ofd = crate::libbb::xconnect::create_and_connect_stream_or_die(*argv.offset(1), 443i32);
    (*tls).ifd = (*tls).ofd
  }
  crate::networking::tls::tls_handshake(tls, sni);
  crate::networking::tls::tls_run_copy_loop(tls, (opt & 1i32) as libc::c_uint);
  return 0i32;
}
