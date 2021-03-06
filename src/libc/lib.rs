// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_libc"]
#![crate_type = "lib"]
#![feature(lang_items, const_fn)]
#![no_std]
#![allow(non_camel_case_types)]

use core::marker::{Pod, Copy};
pub use arch::{pthread_attr_t, pthread_t};

#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
#[path = "x86_64.rs"]
mod arch;

#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
#[path = "x32.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

#[cfg(target_arch = "arm")]
#[path = "arm.rs"]
mod arch;

#[cfg(target_arch = "aarch64")]
#[path = "aarch64.rs"]
mod arch;

pub const PTHREAD_CREATE_JOINABLE: i32 = 0;
pub const PTHREAD_CREATE_DETACHED: i32 = 1;

#[repr(C)]
pub struct pthread_key_t {
    data: u32,
}

impl pthread_key_t {
    pub const fn new() -> pthread_key_t {
        pthread_key_t {
            data: 0,
        }
    }
}

unsafe impl Pod for pthread_key_t { }
impl Copy for pthread_key_t { }

unsafe impl Pod for pthread_t { }
impl Copy for pthread_t { }

unsafe impl Pod for pthread_attr_t { }
impl Copy for pthread_attr_t { }

#[cfg(not(any(no_link_args, no_libc)))]
#[link(name = "c")]
extern { }

#[cfg(not(any(no_link_args, no_libc)))]
#[link(name = "pthread")]
extern { }

#[allow(improper_ctypes)]
extern {
    pub fn memchr(s: *const u8, c: i32, n: usize) -> *const u8;
    pub fn memset(s: *const u8, c: i32, n: usize) -> *const u8;
    pub fn memrchr(s: *const u8, c: i32, n: usize) -> *const u8;
    pub fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32;
    pub fn fork() -> i32;
    pub fn __errno_location() -> *mut i32;
    pub fn realloc(ptr: *mut d8, size: usize) -> *mut d8;
    pub fn strlen(s: *const u8) -> usize;
}

#[allow(improper_ctypes)]
extern {
    pub fn pthread_create(thread: *mut pthread_t, attr: *const pthread_attr_t,
                          start: unsafe extern fn(*mut u8) -> *mut u8,
                          arg: *mut u8) -> i32;
    pub fn pthread_join(thread: pthread_t, retval: *mut *mut u8) -> i32;
    pub fn pthread_key_create(key: *mut pthread_key_t, dest: extern fn(*mut u8)) -> i32;
    pub fn pthread_setspecific(key: pthread_key_t, val: *mut u8) -> i32;
    pub fn pthread_getspecific(key: pthread_key_t) -> *mut u8;
    pub fn pthread_attr_init(attr: *mut pthread_attr_t) -> i32;
    pub fn pthread_attr_destroy(attr: *mut pthread_attr_t) -> i32;
    pub fn pthread_attr_setdetachstate(attr: *mut pthread_attr_t, state: i32) -> i32;
    pub fn pthread_attr_getdetachstate(attr: *const pthread_attr_t, state: *mut i32) -> i32;
    pub fn pthread_attr_setguardsize(attr: *mut pthread_attr_t, size: usize) -> i32;
    pub fn pthread_attr_getguardsize(attr: *const pthread_attr_t, size: *mut usize) -> i32;
    pub fn pthread_attr_setstacksize(attr: *mut pthread_attr_t, size: usize) -> i32;
    pub fn pthread_attr_getstacksize(attr: *const pthread_attr_t, size: *mut usize) -> i32;
}

pub fn errno() -> i32 {
    unsafe { *__errno_location() }
}
