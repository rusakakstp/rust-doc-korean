// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// xfail-test FIXME #3796
#[deny(dead_assignment)];
fn main() {
    let mut x = 1;
    let f: &fn() -> int = || { x + 20 };
    assert_eq!(f(), 21);
    x += 1;
    assert_eq!(f(), 22);
}