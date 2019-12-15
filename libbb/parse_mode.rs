use libc;
use libc::mode_t;
use libc::umask;
extern "C" {
  #[no_mangle]
  fn strtoul(
    __nptr: *const libc::c_char,
    __endptr: *mut *mut libc::c_char,
    __base: libc::c_int,
  ) -> libc::c_ulong;

}

#[no_mangle]
pub unsafe extern "C" fn bb_parse_mode(
  mut s: *const libc::c_char,
  mut current_mode: libc::c_uint,
) -> libc::c_int {
  static mut who_mask: [mode_t; 4] = [
    (0o4000i32
      | 0o2000i32
      | 0o1000i32
      | (0o400i32 | 0o200i32 | 0o100i32)
      | (0o400i32 | 0o200i32 | 0o100i32) >> 3i32
      | (0o400i32 | 0o200i32 | 0o100i32) >> 3i32 >> 3i32) as mode_t,
    (0o4000i32 | (0o400i32 | 0o200i32 | 0o100i32)) as mode_t,
    (0o2000i32 | (0o400i32 | 0o200i32 | 0o100i32) >> 3i32) as mode_t,
    ((0o400i32 | 0o200i32 | 0o100i32) >> 3i32 >> 3i32) as mode_t,
  ];
  static mut perm_mask: [mode_t; 6] = [
    (0o400i32 | 0o400i32 >> 3i32 | 0o400i32 >> 3i32 >> 3i32) as mode_t,
    (0o200i32 | 0o200i32 >> 3i32 | 0o200i32 >> 3i32 >> 3i32) as mode_t,
    (0o100i32 | 0o100i32 >> 3i32 | 0o100i32 >> 3i32 >> 3i32) as mode_t,
    (0o100i32 | 0o100i32 >> 3i32 | 0o100i32 >> 3i32 >> 3i32) as mode_t,
    (0o4000i32 | 0o2000i32) as mode_t,
    0o1000i32 as mode_t,
  ];
  static mut who_chars: [libc::c_char; 5] = [97, 117, 103, 111, 0];
  static mut perm_chars: [libc::c_char; 7] = [114, 119, 120, 88, 115, 116, 0];
  let mut p: *const libc::c_char = std::ptr::null();
  let mut wholist: mode_t = 0;
  let mut permlist: mode_t = 0;
  let mut new_mode: mode_t = 0;
  let mut op: libc::c_char = 0;
  if ((*s as libc::c_int - '0' as i32) as libc::c_uchar as libc::c_int) < 8i32 {
    let mut tmp: libc::c_ulong = 0;
    let mut e: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    tmp = strtoul(s, &mut e, 8i32);
    if *e as libc::c_int != 0 || tmp > 0o7777u32 as libc::c_ulong {
      /* Check range and trailing chars. */
      return -1i32;
    }
    return tmp as libc::c_int;
  }
  new_mode = current_mode;
  /* Note: we allow empty clauses, and hence empty modes.
   * We treat an empty mode as no change to perms. */
  while *s != 0 {
    /* Process clauses. */
    if *s as libc::c_int == ',' as i32 {
      /* We allow empty clauses. */
      s = s.offset(1)
    } else {
      /* Get a wholist. */
      wholist = 0i32 as mode_t;
      'c_7959: loop {
        p = who_chars.as_ptr();
        loop {
          if *p as libc::c_int == *s as libc::c_int {
            wholist |= who_mask
              [p.wrapping_offset_from(who_chars.as_ptr()) as libc::c_long as libc::c_int as usize];
            s = s.offset(1);
            if *s == 0 {
              return -1i32;
            }
            break;
          } else {
            p = p.offset(1);
            if !(*p != 0) {
              break 'c_7959;
            }
          }
        }
      }
      loop {
        let mut current_block_50: u64;
        /* Process action list. */
        if *s as libc::c_int != '+' as i32 && *s as libc::c_int != '-' as i32 {
          if *s as libc::c_int != '=' as i32 {
            return -1i32;
          }
          /* Since op is '=', clear all bits corresponding to the
           * wholist, or all file bits if wholist is empty. */
          permlist = !(0o4000i32
            | 0o2000i32
            | 0o1000i32
            | (0o400i32 | 0o200i32 | 0o100i32)
            | (0o400i32 | 0o200i32 | 0o100i32) >> 3i32
            | (0o400i32 | 0o200i32 | 0o100i32) >> 3i32 >> 3i32) as mode_t;
          if wholist != 0 {
            permlist = !wholist
          }
          new_mode &= permlist
        }
        let fresh0 = s;
        s = s.offset(1);
        op = *fresh0;
        /* Check for permcopy. */
        p = who_chars.as_ptr().offset(1); /* Skip 'a' entry. */
        loop {
          if *p as libc::c_int == *s as libc::c_int {
            let mut i: libc::c_int = 0i32;
            permlist = who_mask
              [p.wrapping_offset_from(who_chars.as_ptr()) as libc::c_long as libc::c_int as usize]
              & (0o400i32
                | 0o200i32
                | 0o100i32
                | (0o400i32 | 0o200i32 | 0o100i32) >> 3i32
                | (0o400i32 | 0o200i32 | 0o100i32) >> 3i32 >> 3i32) as libc::c_uint
              & new_mode;
            loop {
              if permlist & perm_mask[i as usize] != 0 {
                permlist |= perm_mask[i as usize]
              }
              i += 1;
              if !(i < 3i32) {
                break;
              }
            }
            s = s.offset(1);
            current_block_50 = 4187030238080514946;
            break;
          } else {
            p = p.offset(1);
            if !(*p != 0) {
              current_block_50 = 14775119014532381840;
              break;
            }
          }
        }
        match current_block_50 {
          14775119014532381840 => {
            /* It was not a permcopy, so get a permlist. */
            permlist = 0i32 as mode_t;
            'c_7982: loop {
              p = perm_chars.as_ptr();
              loop {
                if *p as libc::c_int == *s as libc::c_int {
                  if *p as libc::c_int != 'X' as i32
                    || new_mode
                      & (0o40000i32 | 0o100i32 | 0o100i32 >> 3i32 | 0o100i32 >> 3i32 >> 3i32)
                        as libc::c_uint
                      != 0
                  {
                    permlist |= perm_mask[p.wrapping_offset_from(perm_chars.as_ptr())
                      as libc::c_long as libc::c_int
                      as usize]
                  }
                  s = s.offset(1);
                  if *s == 0 {
                    break 'c_7982;
                  } else {
                    break;
                  }
                } else {
                  p = p.offset(1);
                  if !(*p != 0) {
                    break 'c_7982;
                  }
                }
              }
            }
          }
          _ => {}
        }
        if permlist != 0 {
          /* The permlist was nonempty. */
          let mut tmp_0: mode_t = wholist;
          if wholist == 0 {
            let mut u_mask: mode_t = umask(0i32 as mode_t);
            umask(u_mask);
            tmp_0 = !u_mask
          }
          permlist &= tmp_0;
          if op as libc::c_int == '-' as i32 {
            new_mode &= !permlist
          } else {
            new_mode |= permlist
          }
        }
        if !(*s as libc::c_int != 0 && *s as libc::c_int != ',' as i32) {
          break;
        }
      }
    }
  }
  return new_mode as libc::c_int;
}
