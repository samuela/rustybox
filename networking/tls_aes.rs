use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
extern "C" {
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}

use libc::uint32_t;
 use libc::uint8_t;
pub type bb__aliased_uint32_t = uint32_t;
use crate::librb::size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tls_aes {
  pub key: [uint32_t; 60],
  pub rounds: libc::c_uint,
}
/*
 * Copyright (C) 2017 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* This AES implementation is derived from tiny-AES128-C code,
 * which was put by its author into public domain:
 *
 * tiny-AES128-C/unlicense.txt, Dec 8, 2014
 * """
 * This is free and unencumbered software released into the public domain.
 *
 * Anyone is free to copy, modify, publish, use, compile, sell, or
 * distribute this software, either in source code form or as a compiled
 * binary, for any purpose, commercial or non-commercial, and by any
 * means.
 *
 * In jurisdictions that recognize copyright laws, the author or authors
 * of this software dedicate any and all copyright interest in the
 * software to the public domain. We make this dedication for the benefit
 * of the public at large and to the detriment of our heirs and
 * successors. We intend this dedication to be an overt act of
 * relinquishment in perpetuity of all present and future rights to this
 * software under copyright law.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 * IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 * """
 */
/* Note that only original tiny-AES128-C code is public domain.
 * The derived code in this file has been expanded to also implement aes192
 * and aes256 and use more efficient word-sized operations in many places,
 * and put under GPLv2 license.
 */
// The lookup-tables are marked const so they can be placed in read-only storage instead of RAM
// The numbers below can be computed dynamically trading ROM for RAM -
// This can be useful in (embedded) bootloader applications, where ROM is often limited.
static mut sbox: [uint8_t; 256] = [
  0x63i32 as uint8_t,
  0x7ci32 as uint8_t,
  0x77i32 as uint8_t,
  0x7bi32 as uint8_t,
  0xf2i32 as uint8_t,
  0x6bi32 as uint8_t,
  0x6fi32 as uint8_t,
  0xc5i32 as uint8_t,
  0x30i32 as uint8_t,
  0x1i32 as uint8_t,
  0x67i32 as uint8_t,
  0x2bi32 as uint8_t,
  0xfei32 as uint8_t,
  0xd7i32 as uint8_t,
  0xabi32 as uint8_t,
  0x76i32 as uint8_t,
  0xcai32 as uint8_t,
  0x82i32 as uint8_t,
  0xc9i32 as uint8_t,
  0x7di32 as uint8_t,
  0xfai32 as uint8_t,
  0x59i32 as uint8_t,
  0x47i32 as uint8_t,
  0xf0i32 as uint8_t,
  0xadi32 as uint8_t,
  0xd4i32 as uint8_t,
  0xa2i32 as uint8_t,
  0xafi32 as uint8_t,
  0x9ci32 as uint8_t,
  0xa4i32 as uint8_t,
  0x72i32 as uint8_t,
  0xc0i32 as uint8_t,
  0xb7i32 as uint8_t,
  0xfdi32 as uint8_t,
  0x93i32 as uint8_t,
  0x26i32 as uint8_t,
  0x36i32 as uint8_t,
  0x3fi32 as uint8_t,
  0xf7i32 as uint8_t,
  0xcci32 as uint8_t,
  0x34i32 as uint8_t,
  0xa5i32 as uint8_t,
  0xe5i32 as uint8_t,
  0xf1i32 as uint8_t,
  0x71i32 as uint8_t,
  0xd8i32 as uint8_t,
  0x31i32 as uint8_t,
  0x15i32 as uint8_t,
  0x4i32 as uint8_t,
  0xc7i32 as uint8_t,
  0x23i32 as uint8_t,
  0xc3i32 as uint8_t,
  0x18i32 as uint8_t,
  0x96i32 as uint8_t,
  0x5i32 as uint8_t,
  0x9ai32 as uint8_t,
  0x7i32 as uint8_t,
  0x12i32 as uint8_t,
  0x80i32 as uint8_t,
  0xe2i32 as uint8_t,
  0xebi32 as uint8_t,
  0x27i32 as uint8_t,
  0xb2i32 as uint8_t,
  0x75i32 as uint8_t,
  0x9i32 as uint8_t,
  0x83i32 as uint8_t,
  0x2ci32 as uint8_t,
  0x1ai32 as uint8_t,
  0x1bi32 as uint8_t,
  0x6ei32 as uint8_t,
  0x5ai32 as uint8_t,
  0xa0i32 as uint8_t,
  0x52i32 as uint8_t,
  0x3bi32 as uint8_t,
  0xd6i32 as uint8_t,
  0xb3i32 as uint8_t,
  0x29i32 as uint8_t,
  0xe3i32 as uint8_t,
  0x2fi32 as uint8_t,
  0x84i32 as uint8_t,
  0x53i32 as uint8_t,
  0xd1i32 as uint8_t,
  0i32 as uint8_t,
  0xedi32 as uint8_t,
  0x20i32 as uint8_t,
  0xfci32 as uint8_t,
  0xb1i32 as uint8_t,
  0x5bi32 as uint8_t,
  0x6ai32 as uint8_t,
  0xcbi32 as uint8_t,
  0xbei32 as uint8_t,
  0x39i32 as uint8_t,
  0x4ai32 as uint8_t,
  0x4ci32 as uint8_t,
  0x58i32 as uint8_t,
  0xcfi32 as uint8_t,
  0xd0i32 as uint8_t,
  0xefi32 as uint8_t,
  0xaai32 as uint8_t,
  0xfbi32 as uint8_t,
  0x43i32 as uint8_t,
  0x4di32 as uint8_t,
  0x33i32 as uint8_t,
  0x85i32 as uint8_t,
  0x45i32 as uint8_t,
  0xf9i32 as uint8_t,
  0x2i32 as uint8_t,
  0x7fi32 as uint8_t,
  0x50i32 as uint8_t,
  0x3ci32 as uint8_t,
  0x9fi32 as uint8_t,
  0xa8i32 as uint8_t,
  0x51i32 as uint8_t,
  0xa3i32 as uint8_t,
  0x40i32 as uint8_t,
  0x8fi32 as uint8_t,
  0x92i32 as uint8_t,
  0x9di32 as uint8_t,
  0x38i32 as uint8_t,
  0xf5i32 as uint8_t,
  0xbci32 as uint8_t,
  0xb6i32 as uint8_t,
  0xdai32 as uint8_t,
  0x21i32 as uint8_t,
  0x10i32 as uint8_t,
  0xffi32 as uint8_t,
  0xf3i32 as uint8_t,
  0xd2i32 as uint8_t,
  0xcdi32 as uint8_t,
  0xci32 as uint8_t,
  0x13i32 as uint8_t,
  0xeci32 as uint8_t,
  0x5fi32 as uint8_t,
  0x97i32 as uint8_t,
  0x44i32 as uint8_t,
  0x17i32 as uint8_t,
  0xc4i32 as uint8_t,
  0xa7i32 as uint8_t,
  0x7ei32 as uint8_t,
  0x3di32 as uint8_t,
  0x64i32 as uint8_t,
  0x5di32 as uint8_t,
  0x19i32 as uint8_t,
  0x73i32 as uint8_t,
  0x60i32 as uint8_t,
  0x81i32 as uint8_t,
  0x4fi32 as uint8_t,
  0xdci32 as uint8_t,
  0x22i32 as uint8_t,
  0x2ai32 as uint8_t,
  0x90i32 as uint8_t,
  0x88i32 as uint8_t,
  0x46i32 as uint8_t,
  0xeei32 as uint8_t,
  0xb8i32 as uint8_t,
  0x14i32 as uint8_t,
  0xdei32 as uint8_t,
  0x5ei32 as uint8_t,
  0xbi32 as uint8_t,
  0xdbi32 as uint8_t,
  0xe0i32 as uint8_t,
  0x32i32 as uint8_t,
  0x3ai32 as uint8_t,
  0xai32 as uint8_t,
  0x49i32 as uint8_t,
  0x6i32 as uint8_t,
  0x24i32 as uint8_t,
  0x5ci32 as uint8_t,
  0xc2i32 as uint8_t,
  0xd3i32 as uint8_t,
  0xaci32 as uint8_t,
  0x62i32 as uint8_t,
  0x91i32 as uint8_t,
  0x95i32 as uint8_t,
  0xe4i32 as uint8_t,
  0x79i32 as uint8_t,
  0xe7i32 as uint8_t,
  0xc8i32 as uint8_t,
  0x37i32 as uint8_t,
  0x6di32 as uint8_t,
  0x8di32 as uint8_t,
  0xd5i32 as uint8_t,
  0x4ei32 as uint8_t,
  0xa9i32 as uint8_t,
  0x6ci32 as uint8_t,
  0x56i32 as uint8_t,
  0xf4i32 as uint8_t,
  0xeai32 as uint8_t,
  0x65i32 as uint8_t,
  0x7ai32 as uint8_t,
  0xaei32 as uint8_t,
  0x8i32 as uint8_t,
  0xbai32 as uint8_t,
  0x78i32 as uint8_t,
  0x25i32 as uint8_t,
  0x2ei32 as uint8_t,
  0x1ci32 as uint8_t,
  0xa6i32 as uint8_t,
  0xb4i32 as uint8_t,
  0xc6i32 as uint8_t,
  0xe8i32 as uint8_t,
  0xddi32 as uint8_t,
  0x74i32 as uint8_t,
  0x1fi32 as uint8_t,
  0x4bi32 as uint8_t,
  0xbdi32 as uint8_t,
  0x8bi32 as uint8_t,
  0x8ai32 as uint8_t,
  0x70i32 as uint8_t,
  0x3ei32 as uint8_t,
  0xb5i32 as uint8_t,
  0x66i32 as uint8_t,
  0x48i32 as uint8_t,
  0x3i32 as uint8_t,
  0xf6i32 as uint8_t,
  0xei32 as uint8_t,
  0x61i32 as uint8_t,
  0x35i32 as uint8_t,
  0x57i32 as uint8_t,
  0xb9i32 as uint8_t,
  0x86i32 as uint8_t,
  0xc1i32 as uint8_t,
  0x1di32 as uint8_t,
  0x9ei32 as uint8_t,
  0xe1i32 as uint8_t,
  0xf8i32 as uint8_t,
  0x98i32 as uint8_t,
  0x11i32 as uint8_t,
  0x69i32 as uint8_t,
  0xd9i32 as uint8_t,
  0x8ei32 as uint8_t,
  0x94i32 as uint8_t,
  0x9bi32 as uint8_t,
  0x1ei32 as uint8_t,
  0x87i32 as uint8_t,
  0xe9i32 as uint8_t,
  0xcei32 as uint8_t,
  0x55i32 as uint8_t,
  0x28i32 as uint8_t,
  0xdfi32 as uint8_t,
  0x8ci32 as uint8_t,
  0xa1i32 as uint8_t,
  0x89i32 as uint8_t,
  0xdi32 as uint8_t,
  0xbfi32 as uint8_t,
  0xe6i32 as uint8_t,
  0x42i32 as uint8_t,
  0x68i32 as uint8_t,
  0x41i32 as uint8_t,
  0x99i32 as uint8_t,
  0x2di32 as uint8_t,
  0xfi32 as uint8_t,
  0xb0i32 as uint8_t,
  0x54i32 as uint8_t,
  0xbbi32 as uint8_t,
  0x16i32 as uint8_t,
];
static mut rsbox: [uint8_t; 256] = [
  0x52i32 as uint8_t,
  0x9i32 as uint8_t,
  0x6ai32 as uint8_t,
  0xd5i32 as uint8_t,
  0x30i32 as uint8_t,
  0x36i32 as uint8_t,
  0xa5i32 as uint8_t,
  0x38i32 as uint8_t,
  0xbfi32 as uint8_t,
  0x40i32 as uint8_t,
  0xa3i32 as uint8_t,
  0x9ei32 as uint8_t,
  0x81i32 as uint8_t,
  0xf3i32 as uint8_t,
  0xd7i32 as uint8_t,
  0xfbi32 as uint8_t,
  0x7ci32 as uint8_t,
  0xe3i32 as uint8_t,
  0x39i32 as uint8_t,
  0x82i32 as uint8_t,
  0x9bi32 as uint8_t,
  0x2fi32 as uint8_t,
  0xffi32 as uint8_t,
  0x87i32 as uint8_t,
  0x34i32 as uint8_t,
  0x8ei32 as uint8_t,
  0x43i32 as uint8_t,
  0x44i32 as uint8_t,
  0xc4i32 as uint8_t,
  0xdei32 as uint8_t,
  0xe9i32 as uint8_t,
  0xcbi32 as uint8_t,
  0x54i32 as uint8_t,
  0x7bi32 as uint8_t,
  0x94i32 as uint8_t,
  0x32i32 as uint8_t,
  0xa6i32 as uint8_t,
  0xc2i32 as uint8_t,
  0x23i32 as uint8_t,
  0x3di32 as uint8_t,
  0xeei32 as uint8_t,
  0x4ci32 as uint8_t,
  0x95i32 as uint8_t,
  0xbi32 as uint8_t,
  0x42i32 as uint8_t,
  0xfai32 as uint8_t,
  0xc3i32 as uint8_t,
  0x4ei32 as uint8_t,
  0x8i32 as uint8_t,
  0x2ei32 as uint8_t,
  0xa1i32 as uint8_t,
  0x66i32 as uint8_t,
  0x28i32 as uint8_t,
  0xd9i32 as uint8_t,
  0x24i32 as uint8_t,
  0xb2i32 as uint8_t,
  0x76i32 as uint8_t,
  0x5bi32 as uint8_t,
  0xa2i32 as uint8_t,
  0x49i32 as uint8_t,
  0x6di32 as uint8_t,
  0x8bi32 as uint8_t,
  0xd1i32 as uint8_t,
  0x25i32 as uint8_t,
  0x72i32 as uint8_t,
  0xf8i32 as uint8_t,
  0xf6i32 as uint8_t,
  0x64i32 as uint8_t,
  0x86i32 as uint8_t,
  0x68i32 as uint8_t,
  0x98i32 as uint8_t,
  0x16i32 as uint8_t,
  0xd4i32 as uint8_t,
  0xa4i32 as uint8_t,
  0x5ci32 as uint8_t,
  0xcci32 as uint8_t,
  0x5di32 as uint8_t,
  0x65i32 as uint8_t,
  0xb6i32 as uint8_t,
  0x92i32 as uint8_t,
  0x6ci32 as uint8_t,
  0x70i32 as uint8_t,
  0x48i32 as uint8_t,
  0x50i32 as uint8_t,
  0xfdi32 as uint8_t,
  0xedi32 as uint8_t,
  0xb9i32 as uint8_t,
  0xdai32 as uint8_t,
  0x5ei32 as uint8_t,
  0x15i32 as uint8_t,
  0x46i32 as uint8_t,
  0x57i32 as uint8_t,
  0xa7i32 as uint8_t,
  0x8di32 as uint8_t,
  0x9di32 as uint8_t,
  0x84i32 as uint8_t,
  0x90i32 as uint8_t,
  0xd8i32 as uint8_t,
  0xabi32 as uint8_t,
  0i32 as uint8_t,
  0x8ci32 as uint8_t,
  0xbci32 as uint8_t,
  0xd3i32 as uint8_t,
  0xai32 as uint8_t,
  0xf7i32 as uint8_t,
  0xe4i32 as uint8_t,
  0x58i32 as uint8_t,
  0x5i32 as uint8_t,
  0xb8i32 as uint8_t,
  0xb3i32 as uint8_t,
  0x45i32 as uint8_t,
  0x6i32 as uint8_t,
  0xd0i32 as uint8_t,
  0x2ci32 as uint8_t,
  0x1ei32 as uint8_t,
  0x8fi32 as uint8_t,
  0xcai32 as uint8_t,
  0x3fi32 as uint8_t,
  0xfi32 as uint8_t,
  0x2i32 as uint8_t,
  0xc1i32 as uint8_t,
  0xafi32 as uint8_t,
  0xbdi32 as uint8_t,
  0x3i32 as uint8_t,
  0x1i32 as uint8_t,
  0x13i32 as uint8_t,
  0x8ai32 as uint8_t,
  0x6bi32 as uint8_t,
  0x3ai32 as uint8_t,
  0x91i32 as uint8_t,
  0x11i32 as uint8_t,
  0x41i32 as uint8_t,
  0x4fi32 as uint8_t,
  0x67i32 as uint8_t,
  0xdci32 as uint8_t,
  0xeai32 as uint8_t,
  0x97i32 as uint8_t,
  0xf2i32 as uint8_t,
  0xcfi32 as uint8_t,
  0xcei32 as uint8_t,
  0xf0i32 as uint8_t,
  0xb4i32 as uint8_t,
  0xe6i32 as uint8_t,
  0x73i32 as uint8_t,
  0x96i32 as uint8_t,
  0xaci32 as uint8_t,
  0x74i32 as uint8_t,
  0x22i32 as uint8_t,
  0xe7i32 as uint8_t,
  0xadi32 as uint8_t,
  0x35i32 as uint8_t,
  0x85i32 as uint8_t,
  0xe2i32 as uint8_t,
  0xf9i32 as uint8_t,
  0x37i32 as uint8_t,
  0xe8i32 as uint8_t,
  0x1ci32 as uint8_t,
  0x75i32 as uint8_t,
  0xdfi32 as uint8_t,
  0x6ei32 as uint8_t,
  0x47i32 as uint8_t,
  0xf1i32 as uint8_t,
  0x1ai32 as uint8_t,
  0x71i32 as uint8_t,
  0x1di32 as uint8_t,
  0x29i32 as uint8_t,
  0xc5i32 as uint8_t,
  0x89i32 as uint8_t,
  0x6fi32 as uint8_t,
  0xb7i32 as uint8_t,
  0x62i32 as uint8_t,
  0xei32 as uint8_t,
  0xaai32 as uint8_t,
  0x18i32 as uint8_t,
  0xbei32 as uint8_t,
  0x1bi32 as uint8_t,
  0xfci32 as uint8_t,
  0x56i32 as uint8_t,
  0x3ei32 as uint8_t,
  0x4bi32 as uint8_t,
  0xc6i32 as uint8_t,
  0xd2i32 as uint8_t,
  0x79i32 as uint8_t,
  0x20i32 as uint8_t,
  0x9ai32 as uint8_t,
  0xdbi32 as uint8_t,
  0xc0i32 as uint8_t,
  0xfei32 as uint8_t,
  0x78i32 as uint8_t,
  0xcdi32 as uint8_t,
  0x5ai32 as uint8_t,
  0xf4i32 as uint8_t,
  0x1fi32 as uint8_t,
  0xddi32 as uint8_t,
  0xa8i32 as uint8_t,
  0x33i32 as uint8_t,
  0x88i32 as uint8_t,
  0x7i32 as uint8_t,
  0xc7i32 as uint8_t,
  0x31i32 as uint8_t,
  0xb1i32 as uint8_t,
  0x12i32 as uint8_t,
  0x10i32 as uint8_t,
  0x59i32 as uint8_t,
  0x27i32 as uint8_t,
  0x80i32 as uint8_t,
  0xeci32 as uint8_t,
  0x5fi32 as uint8_t,
  0x60i32 as uint8_t,
  0x51i32 as uint8_t,
  0x7fi32 as uint8_t,
  0xa9i32 as uint8_t,
  0x19i32 as uint8_t,
  0xb5i32 as uint8_t,
  0x4ai32 as uint8_t,
  0xdi32 as uint8_t,
  0x2di32 as uint8_t,
  0xe5i32 as uint8_t,
  0x7ai32 as uint8_t,
  0x9fi32 as uint8_t,
  0x93i32 as uint8_t,
  0xc9i32 as uint8_t,
  0x9ci32 as uint8_t,
  0xefi32 as uint8_t,
  0xa0i32 as uint8_t,
  0xe0i32 as uint8_t,
  0x3bi32 as uint8_t,
  0x4di32 as uint8_t,
  0xaei32 as uint8_t,
  0x2ai32 as uint8_t,
  0xf5i32 as uint8_t,
  0xb0i32 as uint8_t,
  0xc8i32 as uint8_t,
  0xebi32 as uint8_t,
  0xbbi32 as uint8_t,
  0x3ci32 as uint8_t,
  0x83i32 as uint8_t,
  0x53i32 as uint8_t,
  0x99i32 as uint8_t,
  0x61i32 as uint8_t,
  0x17i32 as uint8_t,
  0x2bi32 as uint8_t,
  0x4i32 as uint8_t,
  0x7ei32 as uint8_t,
  0xbai32 as uint8_t,
  0x77i32 as uint8_t,
  0xd6i32 as uint8_t,
  0x26i32 as uint8_t,
  0xe1i32 as uint8_t,
  0x69i32 as uint8_t,
  0x14i32 as uint8_t,
  0x63i32 as uint8_t,
  0x55i32 as uint8_t,
  0x21i32 as uint8_t,
  0xci32 as uint8_t,
  0x7di32 as uint8_t,
];
// SubWord() is a function that takes a four-byte input word and
// applies the S-box to each of the four bytes to produce an output word.
unsafe extern "C" fn Subword(mut x: uint32_t) -> uint32_t {
  return ((sbox[(x >> 24i32) as usize] as libc::c_int) << 24i32
    | (sbox[(x >> 16i32 & 255i32 as libc::c_uint) as usize] as libc::c_int) << 16i32
    | (sbox[(x >> 8i32 & 255i32 as libc::c_uint) as usize] as libc::c_int) << 8i32
    | sbox[(x & 255i32 as libc::c_uint) as usize] as libc::c_int) as uint32_t;
}
// This function produces Nb(Nr+1) round keys.
// The round keys are used in each round to decrypt the states.
unsafe extern "C" fn KeyExpansion(
  mut RoundKey: *mut uint32_t,
  mut key: *const libc::c_void,
  mut key_len: libc::c_uint,
) -> libc::c_int {
  // The round constant word array, Rcon[i], contains the values given by
  // x to th e power (i-1) being powers of x (x is denoted as {02}) in the field GF(2^8).
  // Note that i starts at 2, not 0.
  static mut Rcon: [uint8_t; 10] = [
    0x1i32 as uint8_t,
    0x2i32 as uint8_t,
    0x4i32 as uint8_t,
    0x8i32 as uint8_t,
    0x10i32 as uint8_t,
    0x20i32 as uint8_t,
    0x40i32 as uint8_t,
    0x80i32 as uint8_t,
    0x1bi32 as uint8_t,
    0x36i32 as uint8_t,
  ];
  let mut rounds: libc::c_int = 0;
  let mut words_key: libc::c_int = 0;
  let mut words_RoundKey: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  let mut j: libc::c_int = 0;
  let mut k: libc::c_int = 0;
  // key_len 16: aes128, rounds 10, words_key 4, words_RoundKey 44
  // key_len 24: aes192, rounds 12, words_key 6, words_RoundKey 52
  // key_len 32: aes256, rounds 14, words_key 8, words_RoundKey 60
  words_key = key_len.wrapping_div(4i32 as libc::c_uint) as libc::c_int;
  rounds =
    (6i32 as libc::c_uint).wrapping_add(key_len.wrapping_div(4i32 as libc::c_uint)) as libc::c_int;
  words_RoundKey = (28i32 as libc::c_uint).wrapping_add(key_len) as libc::c_int;
  // The first round key is the key itself.
  i = 0i32;
  while i < words_key {
    *RoundKey.offset(i as isize) = {
      let mut v: uint32_t = 0;
      v = *((key as *mut uint32_t).offset(i as isize) as *mut bb__aliased_uint32_t);
      ({
        let mut __v: libc::c_uint = 0;
        let mut __x: libc::c_uint = v;
        if 0 != 0 {
          __v = (__x & 0xff000000u32) >> 24i32
            | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
            | (__x & 0xff00i32 as libc::c_uint) << 8i32
            | (__x & 0xffi32 as libc::c_uint) << 24i32
        } else {
          let fresh0 = &mut __v;
          let fresh1;
          let fresh2 = __x;
          asm!("bswap $0" : "=r" (fresh1) : "0"
                               (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2))
                               :);
          c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
        }
        __v
      })
    };
    i += 1
  }
  // i == words_key now
  // All other round keys are found from the previous round keys.
  k = 0i32;
  j = k;
  while i < words_RoundKey {
    let mut tempa: uint32_t = 0;
    tempa = *RoundKey.offset((i - 1i32) as isize);
    if j == 0i32 {
      // RotWord(): rotates the 4 bytes in a word to the left once.
      tempa = tempa << 8i32 | tempa >> 24i32;
      tempa = Subword(tempa);
      tempa ^= (Rcon[k as usize] as uint32_t) << 24i32
    } else if words_key > 6i32 && j == 4i32 {
      tempa = Subword(tempa)
    }
    *RoundKey.offset(i as isize) = *RoundKey.offset((i - words_key) as isize) ^ tempa;
    j += 1;
    if j == words_key {
      j = 0i32;
      k += 1
    }
    i += 1
  }
  return rounds;
}
// This function adds the round key to state.
// The round key is added to the state by an XOR function.
unsafe extern "C" fn AddRoundKey(mut astate: *mut libc::c_uint, mut RoundKeys: *const uint32_t) {
  let mut i: libc::c_int = 0;
  i = 0i32;
  while i < 16i32 {
    let fresh3 = RoundKeys;
    RoundKeys = RoundKeys.offset(1);
    let mut n: uint32_t = *fresh3;
    *astate.offset((i + 0i32) as isize) ^= n >> 24i32;
    *astate.offset((i + 1i32) as isize) ^= n >> 16i32 & 255i32 as libc::c_uint;
    *astate.offset((i + 2i32) as isize) ^= n >> 8i32 & 255i32 as libc::c_uint;
    *astate.offset((i + 3i32) as isize) ^= n & 255i32 as libc::c_uint;
    i += 4i32
  }
}
// The SubBytes Function Substitutes the values in the
// state matrix with values in an S-box.
unsafe extern "C" fn SubBytes(mut astate: *mut libc::c_uint) {
  let mut i: libc::c_int = 0;
  i = 0i32;
  while i < 16i32 {
    *astate.offset(i as isize) = sbox[*astate.offset(i as isize) as usize] as libc::c_uint;
    i += 1
  }
}
// Our code actually stores "columns" (in aes encryption terminology)
// of state in rows: first 4 elements are "row 0, col 0", "row 1, col 0".
// "row 2, col 0", "row 3, col 0". The fifth element is "row 0, col 1",
// and so on.
// The ShiftRows() function shifts the rows in the state to the left.
// Each row is shifted with different offset.
// Offset = Row number. So the first row is not shifted.
unsafe extern "C" fn ShiftRows(mut astate: *mut libc::c_uint) {
  let mut v: libc::c_uint = 0;
  // Rotate first row 1 columns to left
  v = *astate.offset((0i32 * 4i32 + 1i32) as isize);
  *astate.offset((0i32 * 4i32 + 1i32) as isize) = *astate.offset((1i32 * 4i32 + 1i32) as isize);
  *astate.offset((1i32 * 4i32 + 1i32) as isize) = *astate.offset((2i32 * 4i32 + 1i32) as isize);
  *astate.offset((2i32 * 4i32 + 1i32) as isize) = *astate.offset((3i32 * 4i32 + 1i32) as isize);
  *astate.offset((3i32 * 4i32 + 1i32) as isize) = v;
  // Rotate second row 2 columns to left
  v = *astate.offset((0i32 * 4i32 + 2i32) as isize);
  *astate.offset((0i32 * 4i32 + 2i32) as isize) = *astate.offset((2i32 * 4i32 + 2i32) as isize);
  *astate.offset((2i32 * 4i32 + 2i32) as isize) = v;
  v = *astate.offset((1i32 * 4i32 + 2i32) as isize);
  *astate.offset((1i32 * 4i32 + 2i32) as isize) = *astate.offset((3i32 * 4i32 + 2i32) as isize);
  *astate.offset((3i32 * 4i32 + 2i32) as isize) = v;
  // Rotate third row 3 columns to left
  v = *astate.offset((3i32 * 4i32 + 3i32) as isize);
  *astate.offset((3i32 * 4i32 + 3i32) as isize) = *astate.offset((2i32 * 4i32 + 3i32) as isize);
  *astate.offset((2i32 * 4i32 + 3i32) as isize) = *astate.offset((1i32 * 4i32 + 3i32) as isize);
  *astate.offset((1i32 * 4i32 + 3i32) as isize) = *astate.offset((0i32 * 4i32 + 3i32) as isize);
  *astate.offset((0i32 * 4i32 + 3i32) as isize) = v;
}
// MixColumns function mixes the columns of the state matrix
unsafe extern "C" fn MixColumns(mut astate: *mut libc::c_uint) {
  let mut i: libc::c_int = 0;
  i = 0i32;
  while i < 16i32 {
    let mut a: libc::c_uint = 0;
    let mut b: libc::c_uint = 0;
    let mut c: libc::c_uint = 0;
    let mut d: libc::c_uint = 0;
    let mut x: libc::c_uint = 0;
    let mut y: libc::c_uint = 0;
    let mut z: libc::c_uint = 0;
    let mut t: libc::c_uint = 0;
    a = *astate.offset((i + 0i32) as isize);
    b = *astate.offset((i + 1i32) as isize);
    c = *astate.offset((i + 2i32) as isize);
    d = *astate.offset((i + 3i32) as isize);
    x = a << 1i32 ^ b ^ b << 1i32 ^ c ^ d;
    y = a ^ b << 1i32 ^ c ^ c << 1i32 ^ d;
    z = a ^ b ^ c << 1i32 ^ d ^ d << 1i32;
    t = a ^ a << 1i32 ^ b ^ c ^ d << 1i32;
    *astate.offset((i + 0i32) as isize) =
      x ^ (-((x >> 8i32) as libc::c_int) & 0x11bi32) as libc::c_uint;
    *astate.offset((i + 1i32) as isize) =
      y ^ (-((y >> 8i32) as libc::c_int) & 0x11bi32) as libc::c_uint;
    *astate.offset((i + 2i32) as isize) =
      z ^ (-((z >> 8i32) as libc::c_int) & 0x11bi32) as libc::c_uint;
    *astate.offset((i + 3i32) as isize) =
      t ^ (-((t >> 8i32) as libc::c_int) & 0x11bi32) as libc::c_uint;
    i += 4i32
  }
}
// The SubBytes Function Substitutes the values in the
// state matrix with values in an S-box.
unsafe extern "C" fn InvSubBytes(mut astate: *mut libc::c_uint) {
  let mut i: libc::c_int = 0;
  i = 0i32;
  while i < 16i32 {
    *astate.offset(i as isize) = rsbox[*astate.offset(i as isize) as usize] as libc::c_uint;
    i += 1
  }
}
unsafe extern "C" fn InvShiftRows(mut astate: *mut libc::c_uint) {
  let mut v: libc::c_uint = 0;
  // Rotate first row 1 columns to right
  v = *astate.offset((3i32 * 4i32 + 1i32) as isize);
  *astate.offset((3i32 * 4i32 + 1i32) as isize) = *astate.offset((2i32 * 4i32 + 1i32) as isize);
  *astate.offset((2i32 * 4i32 + 1i32) as isize) = *astate.offset((1i32 * 4i32 + 1i32) as isize);
  *astate.offset((1i32 * 4i32 + 1i32) as isize) = *astate.offset((0i32 * 4i32 + 1i32) as isize);
  *astate.offset((0i32 * 4i32 + 1i32) as isize) = v;
  // Rotate second row 2 columns to right
  v = *astate.offset((0i32 * 4i32 + 2i32) as isize);
  *astate.offset((0i32 * 4i32 + 2i32) as isize) = *astate.offset((2i32 * 4i32 + 2i32) as isize);
  *astate.offset((2i32 * 4i32 + 2i32) as isize) = v;
  v = *astate.offset((1i32 * 4i32 + 2i32) as isize);
  *astate.offset((1i32 * 4i32 + 2i32) as isize) = *astate.offset((3i32 * 4i32 + 2i32) as isize);
  *astate.offset((3i32 * 4i32 + 2i32) as isize) = v;
  // Rotate third row 3 columns to right
  v = *astate.offset((0i32 * 4i32 + 3i32) as isize);
  *astate.offset((0i32 * 4i32 + 3i32) as isize) = *astate.offset((1i32 * 4i32 + 3i32) as isize);
  *astate.offset((1i32 * 4i32 + 3i32) as isize) = *astate.offset((2i32 * 4i32 + 3i32) as isize);
  *astate.offset((2i32 * 4i32 + 3i32) as isize) = *astate.offset((3i32 * 4i32 + 3i32) as isize);
  *astate.offset((3i32 * 4i32 + 3i32) as isize) = v;
}
#[inline(always)]
unsafe extern "C" fn Multiply(mut x: libc::c_uint) -> libc::c_uint {
  let mut y: libc::c_uint = 0;
  y = x >> 8i32;
  return (x ^ y ^ y << 1i32 ^ y << 3i32 ^ y << 4i32) & 255i32 as libc::c_uint;
}
// MixColumns function mixes the columns of the state matrix.
// The method used to multiply may be difficult to understand for the inexperienced.
// Please use the references to gain more information.
unsafe extern "C" fn InvMixColumns(mut astate: *mut libc::c_uint) {
  let mut i: libc::c_int = 0;
  i = 0i32;
  while i < 16i32 {
    let mut a: libc::c_uint = 0;
    let mut b: libc::c_uint = 0;
    let mut c: libc::c_uint = 0;
    let mut d: libc::c_uint = 0;
    let mut x: libc::c_uint = 0;
    let mut y: libc::c_uint = 0;
    let mut z: libc::c_uint = 0;
    let mut t: libc::c_uint = 0;
    a = *astate.offset((i + 0i32) as isize);
    b = *astate.offset((i + 1i32) as isize);
    c = *astate.offset((i + 2i32) as isize);
    d = *astate.offset((i + 3i32) as isize);
    x = a << 1i32
      ^ a << 2i32
      ^ a << 3i32
      ^ b
      ^ b << 1i32
      ^ b << 3i32
      ^ c
      ^ c << 2i32
      ^ c << 3i32
      ^ d
      ^ d << 3i32;
    y = a
      ^ a << 3i32
      ^ b << 1i32
      ^ b << 2i32
      ^ b << 3i32
      ^ c
      ^ c << 1i32
      ^ c << 3i32
      ^ d
      ^ d << 2i32
      ^ d << 3i32;
    z = a
      ^ a << 2i32
      ^ a << 3i32
      ^ b
      ^ b << 3i32
      ^ c << 1i32
      ^ c << 2i32
      ^ c << 3i32
      ^ d
      ^ d << 1i32
      ^ d << 3i32;
    t = a
      ^ a << 1i32
      ^ a << 3i32
      ^ b
      ^ b << 2i32
      ^ b << 3i32
      ^ c
      ^ c << 3i32
      ^ d << 1i32
      ^ d << 2i32
      ^ d << 3i32;
    *astate.offset((i + 0i32) as isize) = Multiply(x);
    *astate.offset((i + 1i32) as isize) = Multiply(y);
    *astate.offset((i + 2i32) as isize) = Multiply(z);
    *astate.offset((i + 3i32) as isize) = Multiply(t);
    i += 4i32
  }
}
unsafe extern "C" fn aes_encrypt_1(mut aes: *mut tls_aes, mut astate: *mut libc::c_uint) {
  let mut rounds: libc::c_uint = (*aes).rounds;
  let mut RoundKey: *const uint32_t = (*aes).key.as_mut_ptr();
  loop {
    AddRoundKey(astate, RoundKey);
    RoundKey = RoundKey.offset(4);
    SubBytes(astate);
    ShiftRows(astate);
    rounds = rounds.wrapping_sub(1);
    if rounds == 0i32 as libc::c_uint {
      break;
    }
    MixColumns(astate);
  }
  AddRoundKey(astate, RoundKey);
}
#[no_mangle]
pub unsafe extern "C" fn aes_setkey(
  mut aes: *mut tls_aes,
  mut key: *const libc::c_void,
  mut key_len: libc::c_uint,
) {
  (*aes).rounds = KeyExpansion((*aes).key.as_mut_ptr(), key, key_len) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn aes_encrypt_one_block(
  mut aes: *mut tls_aes,
  mut data: *const libc::c_void,
  mut dst: *mut libc::c_void,
) {
  let mut astate: [libc::c_uint; 16] = [0; 16];
  let mut i: libc::c_uint = 0;
  let mut pt: *const uint8_t = data as *const uint8_t;
  let mut ct: *mut uint8_t = dst as *mut uint8_t;
  i = 0i32 as libc::c_uint;
  while i < 16i32 as libc::c_uint {
    astate[i as usize] = *pt.offset(i as isize) as libc::c_uint;
    i = i.wrapping_add(1)
  }
  aes_encrypt_1(aes, astate.as_mut_ptr());
  i = 0i32 as libc::c_uint;
  while i < 16i32 as libc::c_uint {
    *ct.offset(i as isize) = astate[i as usize] as uint8_t;
    i = i.wrapping_add(1)
  }
}
#[no_mangle]
pub unsafe extern "C" fn aes_cbc_encrypt(
  mut aes: *mut tls_aes,
  mut iv: *mut libc::c_void,
  mut data: *const libc::c_void,
  mut len: size_t,
  mut dst: *mut libc::c_void,
) {
  let mut iv2: [uint8_t; 16] = [0; 16];
  let mut pt: *const uint8_t = data as *const uint8_t;
  let mut ct: *mut uint8_t = dst as *mut uint8_t;
  memcpy(
    iv2.as_mut_ptr() as *mut libc::c_void,
    iv,
    16i32 as libc::c_ulong,
  );
  while len > 0i32 as libc::c_ulong {
    /* almost aes_encrypt_one_block(rounds, RoundKey, pt, ct);
     * but xor'ing of IV with plaintext[] is combined
     * with plaintext[] -> astate[]
     */
    let mut i: libc::c_int = 0;
    let mut astate: [libc::c_uint; 16] = [0; 16];
    i = 0i32;
    while i < 16i32 {
      astate[i as usize] =
        (*pt.offset(i as isize) as libc::c_int ^ iv2[i as usize] as libc::c_int) as libc::c_uint;
      i += 1
    }
    aes_encrypt_1(aes, astate.as_mut_ptr());
    i = 0i32;
    while i < 16i32 {
      let ref mut fresh4 = *ct.offset(i as isize);
      *fresh4 = astate[i as usize] as uint8_t;
      iv2[i as usize] = *fresh4;
      i += 1
    }
    ct = ct.offset(16);
    pt = pt.offset(16);
    len = (len as libc::c_ulong).wrapping_sub(16i32 as libc::c_ulong) as size_t as size_t
  }
}
unsafe extern "C" fn aes_decrypt_1(mut aes: *mut tls_aes, mut astate: *mut libc::c_uint) {
  let mut rounds: libc::c_uint = (*aes).rounds;
  let mut RoundKey: *const uint32_t = (*aes).key.as_mut_ptr();
  RoundKey = RoundKey.offset(rounds.wrapping_mul(4i32 as libc::c_uint) as isize);
  AddRoundKey(astate, RoundKey);
  loop {
    InvShiftRows(astate);
    InvSubBytes(astate);
    RoundKey = RoundKey.offset(-4);
    AddRoundKey(astate, RoundKey);
    rounds = rounds.wrapping_sub(1);
    if rounds == 0i32 as libc::c_uint {
      break;
    }
    InvMixColumns(astate);
  }
}
/*
 * Copyright (C) 2017 Denys Vlasenko
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 *
 * Selected few declarations for AES.
 */
//UNUSED
#[no_mangle]
pub unsafe extern "C" fn aes_cbc_decrypt(
  mut aes: *mut tls_aes,
  mut iv: *mut libc::c_void,
  mut data: *const libc::c_void,
  mut len: size_t,
  mut dst: *mut libc::c_void,
) {
  let mut iv2: [uint8_t; 16] = [0; 16];
  let mut iv3: [uint8_t; 16] = [0; 16];
  let mut ivbuf: *mut uint8_t = 0 as *mut uint8_t;
  let mut ivnext: *mut uint8_t = 0 as *mut uint8_t;
  let mut ct: *const uint8_t = data as *const uint8_t;
  let mut pt: *mut uint8_t = dst as *mut uint8_t;
  ivbuf = memcpy(
    iv2.as_mut_ptr() as *mut libc::c_void,
    iv,
    16i32 as libc::c_ulong,
  ) as *mut uint8_t;
  while len != 0 {
    ivnext = if ivbuf == iv2.as_mut_ptr() {
      iv3.as_mut_ptr()
    } else {
      iv2.as_mut_ptr()
    };
    /* almost aes_decrypt_one_block(rounds, RoundKey, ct, pt)
     * but xor'ing of ivbuf is combined with astate[] -> plaintext[]
     */
    let mut i: libc::c_int = 0;
    let mut astate: [libc::c_uint; 16] = [0; 16];
    i = 0i32;
    while i < 16i32 {
      astate[i as usize] = *ct.offset(i as isize) as libc::c_uint;
      *ivnext.offset(i as isize) = astate[i as usize] as uint8_t;
      i += 1
    }
    aes_decrypt_1(aes, astate.as_mut_ptr());
    i = 0i32;
    while i < 16i32 {
      *pt.offset(i as isize) =
        (astate[i as usize] ^ *ivbuf.offset(i as isize) as libc::c_uint) as uint8_t;
      i += 1
    }
    ivbuf = ivnext;
    ct = ct.offset(16);
    pt = pt.offset(16);
    len = (len as libc::c_ulong).wrapping_sub(16i32 as libc::c_ulong) as size_t as size_t
  }
}
