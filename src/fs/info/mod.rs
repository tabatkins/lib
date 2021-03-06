// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use fmt::{Debug, Write};
use core::{mem};
use cty::{PATH_MAX, c_ulong};
use syscall::{statfs, StatfsType};
use rv::{retry};
use rmo::{ToRmo};
use str_one::{CStr};
use str_two::{CString};

use {rmo_cstr, Pool};

use self::mount::{Flags};
use self::types::{FileSystem};

pub mod types;
pub mod mount;

pub fn from_statfs(s: StatfsType) -> FileSystemInfo {
    FileSystemInfo(s)
}

/// Filesystem information.
#[derive(Pod, Eq)]
pub struct FileSystemInfo(StatfsType);

impl FileSystemInfo {
    /// Returns information about the filesystem located at a path.
    ///
    /// [argument, path]
    /// A path inside the filesystem's mount point.
    pub fn from_path<P>(path: P) -> Result<FileSystemInfo>
        where P: for<'a> ToRmo<Pool<'a>, CStr, CString<Pool<'a>>>,
    {
        let mut path_buf: [d8; PATH_MAX] = unsafe { mem::uninit() };
        let path = try!(rmo_cstr(&path, &mut path_buf));
        let mut buf = mem::zeroed();
        retry(|| statfs(&path, &mut buf)).map(|_| FileSystemInfo(buf))
    }

    /// Returns the type of the filesystem.
    pub fn ty(&self) -> FileSystem {
        FileSystem(self.0.f_type as c_ulong)
    }

    /// Returns the block size of the filesystem.
    pub fn block_size(&self) -> u64 {
        self.0.f_bsize as u64
    }

    /// Returns the number of blocks in the filesystem.
    pub fn blocks(&self) -> u64 {
        self.0.f_blocks as u64
    }

    /// Returns the number of free blocks in the filesystem.
    pub fn free_blocks(&self) -> u64 {
        self.0.f_bfree as u64
    }

    /// Returns the number of free blocks usable by unprivileged users.
    pub fn available_blocks(&self) -> u64 {
        self.0.f_bavail as u64
    }

    /// Returns the number of files in the filesystem.
    pub fn files(&self) -> u64 {
        self.0.f_files as u64
    }

    /// Returns the number of free inodes in the filesystem.
    pub fn free_files(&self) -> u64 {
        self.0.f_ffree as u64
    }

    /// Returns the maximum length of a filename in the filesystem.
    pub fn max_name_len(&self) -> u64 {
        self.0.f_namelen as u64
    }

    /// Returns the fragment size of the filesystem.
    pub fn fragment_size(&self) -> u64 {
        self.0.f_frsize as u64
    }

    /// Returns the flags the filesystem is mounted with.
    pub fn mount_flags(&self) -> Flags {
        Flags(self.0.f_flags as c_ulong)
    }
}

impl Debug for FileSystemInfo {
    fn fmt<W: Write+?Sized>(&self, mut w: &mut W) -> Result {
        write!(w, "Flags {{ ty: {:?}, block_size: {}, blocks: {}, free_blocks: {}, \
                           available_blocks: {}, files: {}, free_files: {}, \
                           max_name_len: {}, fragment_size: {}, mount_flags: {:?} }}",
                   self.ty(), self.block_size(), self.blocks(), self.free_blocks(),
                   self.available_blocks(), self.files(), self.free_files(),
                   self.max_name_len(), self.fragment_size(), self.mount_flags())
    }
}
