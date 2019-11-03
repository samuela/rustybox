use libc;
extern "C" {
  pub type tls_handshake_data;
  #[no_mangle]
  fn xzalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn create_and_connect_stream_or_die(peer: *const libc::c_char, port: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn tls_handshake(tls: *mut tls_state_t, sni: *const libc::c_char);
  #[no_mangle]
  fn tls_run_copy_loop(tls: *mut tls_state_t, flags: libc::c_uint);
  #[no_mangle]
  fn getopt32(argv: *mut *mut libc::c_char, applet_opts: *const libc::c_char, _: ...) -> uint32_t;
  #[no_mangle]
  fn bb_show_usage() -> !;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tls_aes {
  pub key: [uint32_t; 60],
  pub rounds: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tls_state {
  pub flags: libc::c_uint,
  pub ofd: libc::c_int,
  pub ifd: libc::c_int,
  pub min_encrypted_len_on_read: libc::c_uint,
  pub cipher_id: uint16_t,
  pub MAC_size: libc::c_uint,
  pub key_size: libc::c_uint,
  pub IV_size: libc::c_uint,
  pub outbuf: *mut uint8_t,
  pub outbuf_size: libc::c_int,
  pub inbuf_size: libc::c_int,
  pub ofs_to_buffered: libc::c_int,
  pub buffered_size: libc::c_int,
  pub inbuf: *mut uint8_t,
  pub hsd: *mut tls_handshake_data,
  pub write_seq64_be: uint64_t,
  pub client_write_key: *mut uint8_t,
  pub server_write_key: *mut uint8_t,
  pub client_write_IV: *mut uint8_t,
  pub server_write_IV: *mut uint8_t,
  pub client_write_MAC_key: [uint8_t; 32],
  pub server_write_MAC_k__: [uint8_t; 32],
  pub client_write_k__: [uint8_t; 32],
  pub server_write_k__: [uint8_t; 32],
  pub client_write_I_: [uint8_t; 4],
  pub server_write_I_: [uint8_t; 4],
  pub aes_encrypt: tls_aes,
  pub aes_decrypt: tls_aes,
  pub H: [uint8_t; 16],
}
pub type tls_state_t = tls_state;
#[inline]
unsafe extern "C" fn new_tls_state() -> *mut tls_state_t {
  let mut tls: *mut tls_state_t =
    xzalloc(::std::mem::size_of::<tls_state_t>() as libc::c_ulong) as *mut tls_state_t;
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
//applet:IF_SSL_CLIENT(APPLET(ssl_client, BB_DIR_USR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_SSL_CLIENT) += ssl_client.o
//usage:#define ssl_client_trivial_usage
//usage:       "[-e] -s FD [-r FD] [-n SNI]"
//usage:#define ssl_client_full_usage ""
#[no_mangle]
pub unsafe extern "C" fn ssl_client_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut tls: *mut tls_state_t = 0 as *mut tls_state_t;
  let mut sni: *const libc::c_char = 0 as *const libc::c_char;
  let mut opt: libc::c_int = 0;
  // INIT_G();
  tls = new_tls_state();
  opt = getopt32(
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
      bb_show_usage();
    }
    /* Undocumented debug feature: without -s and -r, takes HOST arg and connects to it */
    //
    // Talk to kernel.org:
    // printf "GET / HTTP/1.1\r\nHost: kernel.org\r\n\r\n" | busybox ssl_client kernel.org
    if sni.is_null() {
      sni = *argv.offset(1)
    }
    (*tls).ofd = create_and_connect_stream_or_die(*argv.offset(1), 443i32);
    (*tls).ifd = (*tls).ofd
  }
  tls_handshake(tls, sni);
  tls_run_copy_loop(tls, (opt & 1i32) as libc::c_uint);
  return 0i32;
}
