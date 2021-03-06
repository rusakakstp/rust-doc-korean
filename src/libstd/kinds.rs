// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*!
The kind traits

Rust types can be classified in various useful ways according to
intrinsic properties of the type. These classifications, often called
'kinds', are represented as traits.

They cannot be implemented by user code, but are instead implemented
by the compiler automatically for the types to which they apply.

The 3 kinds are

* Copy - types that may be copied without allocation. This includes
  scalar types and managed pointers, and exludes owned pointers. It
  also excludes types that implement `Drop`.

* Send - owned types and types containing owned types.  These types
  may be transferred across task boundaries.

* Freeze - types that are deeply immutable.

`Copy` types include both implicitly copyable types that the compiler
will copy automatically and non-implicitly copyable types that require
the `copy` keyword to copy. Types that do not implement `Copy` may
instead implement `Clone`.

*/

#[allow(missing_doc)];

#[lang="copy"]
pub trait Copy {
    // Empty.
}

#[cfg(stage0)]
#[lang="owned"]
pub trait Send {
    // empty.
}

#[cfg(not(stage0))]
#[lang="send"]
pub trait Send {
    // empty.
}

#[cfg(stage0)]
#[lang="const"]
pub trait Freeze {
    // empty.
}

#[cfg(not(stage0))]
#[lang="freeze"]
pub trait Freeze {
    // empty.
}

#[lang="sized"]
pub trait Sized {
    // Empty.
}
