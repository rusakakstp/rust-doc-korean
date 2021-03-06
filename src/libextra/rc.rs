// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[allow(missing_doc)];

/** Task-local reference counted smart pointers

Task-local reference counted smart pointers are an alternative to managed boxes with deterministic
destruction. They are restricted to containing types that are either `Send` or `Freeze` (or both) to
prevent cycles.

Neither `Rc<T>` or `RcMut<T>` is ever `Send` and `RcMut<T>` is never `Freeze`. If `T` is `Freeze`, a
cycle cannot be created with `Rc<T>` because there is no way to modify it after creation.

*/


use std::cast;
use std::libc::{c_void, size_t, malloc, free};
use std::ptr;
use std::sys;
use std::unstable::intrinsics;

struct RcBox<T> {
    value: T,
    count: uint
}

/// Immutable reference counted pointer type
#[unsafe_no_drop_flag]
#[no_send]
pub struct Rc<T> {
    priv ptr: *mut RcBox<T>,
}

impl<T> Rc<T> {
    unsafe fn new(value: T) -> Rc<T> {
        let ptr = malloc(sys::size_of::<RcBox<T>>() as size_t) as *mut RcBox<T>;
        assert!(!ptr::is_null(ptr));
        intrinsics::move_val_init(&mut *ptr, RcBox{value: value, count: 1});
        Rc{ptr: ptr}
    }
}

// FIXME: #6516: should be a static method
pub fn rc_from_owned<T: Send>(value: T) -> Rc<T> {
    unsafe { Rc::new(value) }
}

// FIXME: #6516: should be a static method
pub fn rc_from_const<T: Freeze>(value: T) -> Rc<T> {
    unsafe { Rc::new(value) }
}

impl<T> Rc<T> {
    #[inline]
    pub fn borrow<'r>(&'r self) -> &'r T {
        unsafe { cast::copy_lifetime(self, &(*self.ptr).value) }
    }
}

#[unsafe_destructor]
impl<T> Drop for Rc<T> {
    fn drop(&self) {
        unsafe {
            if self.ptr.is_not_null() {
                (*self.ptr).count -= 1;
                if (*self.ptr).count == 0 {
                    ptr::read_ptr(self.ptr);
                    free(self.ptr as *c_void)
                }
            }
        }
    }
}

impl<T> Clone for Rc<T> {
    /// Return a shallow copy of the reference counted pointer.
    #[inline]
    fn clone(&self) -> Rc<T> {
        unsafe {
            (*self.ptr).count += 1;
            Rc{ptr: self.ptr}
        }
    }
}

impl<T: DeepClone> DeepClone for Rc<T> {
    /// Return a deep copy of the reference counted pointer.
    #[inline]
    fn deep_clone(&self) -> Rc<T> {
        unsafe { Rc::new(self.borrow().deep_clone()) }
    }
}

#[cfg(test)]
mod test_rc {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn test_clone() {
        let x = rc_from_owned(Cell::new(5));
        let y = x.clone();
        do x.borrow().with_mut_ref |inner| {
            *inner = 20;
        }
        assert_eq!(y.borrow().take(), 20);
    }

    #[test]
    fn test_deep_clone() {
        let x = rc_from_owned(Cell::new(5));
        let y = x.deep_clone();
        do x.borrow().with_mut_ref |inner| {
            *inner = 20;
        }
        assert_eq!(y.borrow().take(), 5);
    }

    #[test]
    fn test_simple() {
        let x = rc_from_const(5);
        assert_eq!(*x.borrow(), 5);
    }

    #[test]
    fn test_simple_clone() {
        let x = rc_from_const(5);
        let y = x.clone();
        assert_eq!(*x.borrow(), 5);
        assert_eq!(*y.borrow(), 5);
    }

    #[test]
    fn test_destructor() {
        let x = rc_from_owned(~5);
        assert_eq!(**x.borrow(), 5);
    }
}

#[abi = "rust-intrinsic"]
extern "rust-intrinsic" {
    fn init<T>() -> T;
    fn uninit<T>() -> T;
}

#[deriving(Eq)]
enum Borrow {
    Mutable,
    Immutable,
    Nothing
}

struct RcMutBox<T> {
    value: T,
    count: uint,
    borrow: Borrow
}

/// Mutable reference counted pointer type
#[non_owned]
#[no_send]
#[mutable] // XXX remove after snap
#[no_freeze]
#[unsafe_no_drop_flag]
pub struct RcMut<T> {
    priv ptr: *mut RcMutBox<T>,
}

impl<T> RcMut<T> {
    unsafe fn new(value: T) -> RcMut<T> {
        let ptr = malloc(sys::size_of::<RcMutBox<T>>() as size_t) as *mut RcMutBox<T>;
        assert!(!ptr::is_null(ptr));
        intrinsics::move_val_init(&mut *ptr, RcMutBox{value: value, count: 1, borrow: Nothing});
        RcMut{ptr: ptr}
    }
}

// FIXME: #6516: should be a static method
pub fn rc_mut_from_owned<T: Send>(value: T) -> RcMut<T> {
    unsafe { RcMut::new(value) }
}

// FIXME: #6516: should be a static method
pub fn rc_mut_from_const<T: Freeze>(value: T) -> RcMut<T> {
    unsafe { RcMut::new(value) }
}

impl<T> RcMut<T> {
    /// Fails if there is already a mutable borrow of the box
    #[inline]
    pub fn with_borrow<U>(&self, f: &fn(&T) -> U) -> U {
        unsafe {
            assert!((*self.ptr).borrow != Mutable);
            let previous = (*self.ptr).borrow;
            (*self.ptr).borrow = Immutable;
            let res = f(&(*self.ptr).value);
            (*self.ptr).borrow = previous;
            res
        }
    }

    /// Fails if there is already a mutable or immutable borrow of the box
    #[inline]
    pub fn with_mut_borrow<U>(&self, f: &fn(&mut T) -> U) -> U {
        unsafe {
            assert_eq!((*self.ptr).borrow, Nothing);
            (*self.ptr).borrow = Mutable;
            let res = f(&mut (*self.ptr).value);
            (*self.ptr).borrow = Nothing;
            res
        }
    }
}

#[unsafe_destructor]
impl<T> Drop for RcMut<T> {
    fn drop(&self) {
        unsafe {
            if self.ptr.is_not_null() {
                (*self.ptr).count -= 1;
                if (*self.ptr).count == 0 {
                    ptr::replace_ptr(self.ptr, uninit());
                    free(self.ptr as *c_void)
                }
            }
        }
    }
}

impl<T> Clone for RcMut<T> {
    /// Return a shallow copy of the reference counted pointer.
    #[inline]
    fn clone(&self) -> RcMut<T> {
        unsafe {
            (*self.ptr).count += 1;
            RcMut{ptr: self.ptr}
        }
    }
}

impl<T: DeepClone> DeepClone for RcMut<T> {
    /// Return a deep copy of the reference counted pointer.
    #[inline]
    fn deep_clone(&self) -> RcMut<T> {
        do self.with_borrow |x| {
            // FIXME: #6497: should avoid freeze (slow)
            unsafe { RcMut::new(x.deep_clone()) }
        }
    }
}

#[cfg(test)]
mod test_rc_mut {
    use super::*;

    #[test]
    fn test_clone() {
        let x = rc_mut_from_owned(5);
        let y = x.clone();
        do x.with_mut_borrow |value| {
            *value = 20;
        }
        do y.with_borrow |value| {
            assert_eq!(*value, 20);
        }
    }

    #[test]
    fn test_deep_clone() {
        let x = rc_mut_from_const(5);
        let y = x.deep_clone();
        do x.with_mut_borrow |value| {
            *value = 20;
        }
        do y.with_borrow |value| {
            assert_eq!(*value, 5);
        }
    }

    #[test]
    fn borrow_many() {
        let x = rc_mut_from_owned(5);
        let y = x.clone();

        do x.with_borrow |a| {
            assert_eq!(*a, 5);
            do y.with_borrow |b| {
                assert_eq!(*b, 5);
                do x.with_borrow |c| {
                    assert_eq!(*c, 5);
                }
            }
        }
    }

    #[test]
    fn modify() {
        let x = rc_mut_from_const(5);
        let y = x.clone();

        do y.with_mut_borrow |a| {
            assert_eq!(*a, 5);
            *a = 6;
        }

        do x.with_borrow |a| {
            assert_eq!(*a, 6);
        }
    }

    #[test]
    fn release_immutable() {
        let x = rc_mut_from_owned(5);
        do x.with_borrow |_| {}
        do x.with_mut_borrow |_| {}
    }

    #[test]
    fn release_mutable() {
        let x = rc_mut_from_const(5);
        do x.with_mut_borrow |_| {}
        do x.with_borrow |_| {}
    }

    #[test]
    #[should_fail]
    fn frozen() {
        let x = rc_mut_from_owned(5);
        let y = x.clone();

        do x.with_borrow |_| {
            do y.with_mut_borrow |_| {
            }
        }
    }

    #[test]
    #[should_fail]
    fn mutable_dupe() {
        let x = rc_mut_from_const(5);
        let y = x.clone();

        do x.with_mut_borrow |_| {
            do y.with_mut_borrow |_| {
            }
        }
    }

    #[test]
    #[should_fail]
    fn mutable_freeze() {
        let x = rc_mut_from_owned(5);
        let y = x.clone();

        do x.with_mut_borrow |_| {
            do y.with_borrow |_| {
            }
        }
    }

    #[test]
    #[should_fail]
    fn restore_freeze() {
        let x = rc_mut_from_const(5);
        let y = x.clone();

        do x.with_borrow |_| {
            do x.with_borrow |_| {}
            do y.with_mut_borrow |_| {}
        }
    }
}
