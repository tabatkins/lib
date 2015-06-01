// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_mem"]
#![crate_type = "lib"]
#![feature(plugin, no_std, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core as core;
extern crate lrs_base as base;
extern crate lrs_cty as cty;
extern crate lrs_fmt as fmt;
extern crate lrs_fd as fd;
extern crate lrs_syscall as syscall;

#[prelude_import] use base::prelude::*;
use core::{slice};
use core::ops::{Range};
use base::{error};
use base::into::{Into};
use cty::{MAP_SHARED, MAP_PRIVATE, c_int, PAGE_SIZE, MAP_FIXED, MREMAP_FIXED};
use flags::{MemMapFlags, MemProtFlags, MemReMapFlags, MMAP_ANON, MemSyncFlags};
use syscall::{mmap, munmap, mremap, msync};
use fd::{FDContainer};

mod lrs { pub use fmt::lrs::*; pub use cty; }

pub mod flags;
pub mod adv;

const PAGE_SIZE_MASK: usize = PAGE_SIZE - 1;

pub struct MemMap {
    ptr: *mut u8,
    len: usize,
}

impl MemMap {
    fn common(mut len: usize, protection: MemProtFlags, shared: bool,
              flags: MemMapFlags, fd: c_int, at: u64) -> Result<MemMap> {
        let flags = flags.0 | match shared {
            true => MAP_SHARED,
            _ => MAP_PRIVATE,
        };
        if flags & MAP_FIXED != 0 {
            abort!();
        }
        len = (len + PAGE_SIZE_MASK) & !PAGE_SIZE_MASK;
        let rv = unsafe { mmap(0, len, protection.0, flags, fd, at) };
        if rv < 0 && rv > -4096 {
            Err(error::Errno(-rv as c_int))
        } else {
            Ok(MemMap { ptr: rv as *mut u8, len: len })
        }
    }

    /// Creates an memory mapping which is not backed by a file.
    ///
    /// [argument, len]
    /// The size of the mapping.
    ///
    /// [argument, protection]
    /// The protection of the mapped region.
    ///
    /// [argument, shared]
    /// Whether this mapping can be shared with other processes.
    ///
    /// [argument, flags]
    /// Flags to use when creating this mapping.
    ///
    /// = Remarks
    ///
    /// The real size of the mapping can be larger than the `len` argument.
    ///
    /// The `MAP_FIXED` flag must not be used with this interface.
    ///
    /// = See also
    ///
    /// * link:man:mmap(2) and MAP_ANONYMOUS therein
    pub fn anon(len: usize, protection: MemProtFlags, shared: bool,
                flags: MemMapFlags) -> Result<MemMap> {
        MemMap::common(len, protection, shared, flags | MMAP_ANON, -1, 0)
    }

    /// Creates an memory mapping of a file.
    ///
    /// [argument, file]
    /// The file that will be mapped.
    ///
    /// [argument, at]
    /// The position in the file at which the mapping starts.
    ///
    /// [argument, len]
    /// The size of the mapping.
    ///
    /// [argument, protection]
    /// The protection of the mapped region.
    ///
    /// [argument, shared]
    /// Whether this mapping can be shared with other processes.
    ///
    /// [argument, flags]
    /// Flags to use when creating this mapping.
    ///
    /// = Remarks
    ///
    /// The real size of the mapping can be larger than the `len` argument.
    ///
    /// = See also
    ///
    /// * link:man:mmap(2)
    pub fn file<F>(file: &F, at: u64, len: usize, protection: MemProtFlags, shared: bool,
                   flags: MemMapFlags) -> Result<MemMap>
        where F: FDContainer,
    {
        MemMap::common(len, protection, shared, flags, file.borrow(), at)
    }

    /// Resizes an existing mapping.
    ///
    /// [argument, new_size]
    /// The new size of the mapping.
    ///
    /// [argument, flags]
    /// Flags to use when remapping this mapping.
    ///
    /// = Remarks
    ///
    /// The `MREMAP_FIXED` flag must not be used with this interface.
    ///
    /// = See also
    ///
    /// * link:man:mremap(2)
    pub fn resize(&mut self, mut new_size: usize, flags: MemReMapFlags) -> Result {
        if flags.0 & MREMAP_FIXED != 0 {
            abort!();
        }
        new_size = (new_size + PAGE_SIZE_MASK) & !PAGE_SIZE_MASK;
        let rv = unsafe { mremap(self.ptr as usize, self.len, new_size, flags.0, 0) };
        if rv < 0 && rv > -4096 {
            Err(error::Errno(-rv as c_int))
        } else {
            self.ptr = rv as *mut u8;
            self.len = new_size;
            Ok(())
        }
    }

    /// Flushes changes to a mapped file to the filesystem.
    ///
    /// [argument, range]
    /// The range to be flushed.
    ///
    /// [argument, flags]
    /// Flags to use for synchronization.
    ///
    /// = See also
    ///
    /// * link:man:msync(2)
    pub fn sync<R>(&self, range: R, flags: MemSyncFlags) -> Result
        where R: Into<Range<Option<usize>>>,
    {
        let range = match range.into() {
            Range { start: None, end: None } => Range { start: 0, end: self.len },
            Range { start: None, end: Some(e) } => Range { start: 0, end: e },
            Range { start: Some(s), end: None } => Range { start: s, end: 0 },
            Range { start: Some(s), end: Some(e) } => Range { start: s, end: e },
        };
        if range.start > range.end || range.end > self.len {
            return Err(error::InvalidArgument);
        }
        rv!(msync(self.ptr as usize + range.start, range.end - range.start, flags.0))
    }
}

impl Deref for MemMap {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { slice::from_ptr(self.ptr, self.len) }
    }
}

impl DerefMut for MemMap {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_ptr(self.ptr, self.len) }
    }
}

impl Drop for MemMap {
    fn drop(&mut self) {
        unsafe { rv!(munmap(self.ptr as usize, self.len)).unwrap(); }
    }
}