// Copyright 2013-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::collections::{LinkedList, VecDeque, BTreeSet, BTreeMap};


#[test]
fn test_format() {
    let s = fmt::format(format_args!("Hello, {}!", "world"));
    assert_eq!(s, "Hello, world!");

    let u32_array = [10u32, 20, 64, 255, 0xffffffff];
    t!(format!("{:?}", u32_array), "[10, 20, 64, 255, 4294967295]");
    t!(format!("{:o}", u32_array), "[12, 24, 100, 377, 37777777777]");
    t!(format!("{:b}", u32_array),
       "[1010, 10100, 1000000, 11111111, 11111111111111111111111111111111]");
    t!(format!("{:x}", u32_array), "[a, 14, 40, ff, ffffffff]");
    t!(format!("{:X}", u32_array), "[A, 14, 40, FF, FFFFFFFF]");

    let f32_array = [10f32, 20.0 , 64.0, 255.0];
    t!(format!("{:?}", f32_array), "[10, 20, 64, 255]");
    t!(format!("{:e}", f32_array), "[1e1, 2e1, 6.4e1, 2.55e2]");
    t!(format!("{:E}", f32_array), "[1E1, 2E1, 6.4E1, 2.55E2]");

    let u32_vec = vec![10u32, 20, 64, 255, 0xffffffff];
    t!(format!("{:?}", u32_vec), "[10, 20, 64, 255, 4294967295]");
    t!(format!("{:o}", u32_vec), "[12, 24, 100, 377, 37777777777]");
    t!(format!("{:b}", u32_vec),
       "[1010, 10100, 1000000, 11111111, 11111111111111111111111111111111]");
    t!(format!("{:x}", u32_vec), "[a, 14, 40, ff, ffffffff]");
    t!(format!("{:X}", u32_vec), "[A, 14, 40, FF, FFFFFFFF]");

    let f32_vec = vec![10f32, 20.0 , 64.0, 255.0];
    t!(format!("{:?}", f32_vec), "[10, 20, 64, 255]");
    t!(format!("{:e}", f32_vec), "[1e1, 2e1, 6.4e1, 2.55e2]");
    t!(format!("{:E}", f32_vec), "[1E1, 2E1, 6.4E1, 2.55E2]");

    let u32_dlist: LinkedList<_> = u32_vec.into_iter().collect();
    t!(format!("{:?}", u32_dlist), "[10, 20, 64, 255, 4294967295]");
    t!(format!("{:o}", u32_dlist), "[12, 24, 100, 377, 37777777777]");
    t!(format!("{:b}", u32_dlist),
       "[1010, 10100, 1000000, 11111111, 11111111111111111111111111111111]");
    t!(format!("{:x}", u32_dlist), "[a, 14, 40, ff, ffffffff]");
    t!(format!("{:X}", u32_dlist), "[A, 14, 40, FF, FFFFFFFF]");

    let f32_dlist: LinkedList<_> = f32_vec.into_iter().collect();
    t!(format!("{:?}", f32_dlist), "[10, 20, 64, 255]");
    t!(format!("{:e}", f32_dlist), "[1e1, 2e1, 6.4e1, 2.55e2]");
    t!(format!("{:E}", f32_dlist), "[1E1, 2E1, 6.4E1, 2.55E2]");

    let u32_ring_buf: VecDeque<_> = u32_dlist.into_iter().collect();
    t!(format!("{:?}", u32_ring_buf), "[10, 20, 64, 255, 4294967295]");
    t!(format!("{:o}", u32_ring_buf), "[12, 24, 100, 377, 37777777777]");
    t!(format!("{:b}", u32_ring_buf),
       "[1010, 10100, 1000000, 11111111, 11111111111111111111111111111111]");
    t!(format!("{:x}", u32_ring_buf), "[a, 14, 40, ff, ffffffff]");
    t!(format!("{:X}", u32_ring_buf), "[A, 14, 40, FF, FFFFFFFF]");

    let f32_ring_buf: VecDeque<_> = f32_dlist.into_iter().collect();
    t!(format!("{:?}", f32_ring_buf), "[10, 20, 64, 255]");
    t!(format!("{:e}", f32_ring_buf), "[1e1, 2e1, 6.4e1, 2.55e2]");
    t!(format!("{:E}", f32_ring_buf), "[1E1, 2E1, 6.4E1, 2.55E2]");

    let u32_btree_set: BTreeSet<_> = u32_ring_buf.into_iter().collect();
    t!(format!("{:?}", u32_btree_set), "{10, 20, 64, 255, 4294967295}");
    t!(format!("{:o}", u32_btree_set), "{12, 24, 100, 377, 37777777777}");
    t!(format!("{:b}", u32_btree_set),
       "{1010, 10100, 1000000, 11111111, 11111111111111111111111111111111}");
    t!(format!("{:x}", u32_btree_set), "{a, 14, 40, ff, ffffffff}");
    t!(format!("{:X}", u32_btree_set), "{A, 14, 40, FF, FFFFFFFF}");

    let mut u32_btree_map: BTreeMap<u32, u32> = BTreeMap::new();
    for x in u32_btree_set.iter() {
        u32_btree_map.insert(*x, *x);
    };

    t!(format!("{:?}", u32_btree_map),
       "{10: 10, 20: 20, 64: 64, \
       255: 255, 4294967295: 4294967295}");
    t!(format!("{:o}", u32_btree_map),
       "{12: 12, 24: 24, 100: 100, 377: 377, 37777777777: 37777777777}");
    t!(format!("{:b}", u32_btree_map),
       "{1010: 1010, 10100: 10100, 1000000: 1000000, 11111111: 11111111, \
       11111111111111111111111111111111: 11111111111111111111111111111111}");
    t!(format!("{:x}", u32_btree_map),
       "{a: a, 14: 14, 40: 40, ff: ff, ffffffff: ffffffff}");
    t!(format!("{:X}", u32_btree_map),
       "{A: A, 14: 14, 40: 40, FF: FF, FFFFFFFF: FFFFFFFF}");
}
