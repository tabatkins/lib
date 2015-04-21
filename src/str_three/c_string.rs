// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use arch_fns::{memchr, all_bytes};
use core::{mem};
use base::{error};
use rmo::{Rmo, ToOwned};
use str_one::c_str::{CStr};
use str_two::c_string::{CString};

pub trait ToCString {
    fn to_cstring(&self) -> Result<CString>;
    fn rmo_cstr<'a>(&'a self, _buf: &'a mut [u8]) -> Result<Rmo<'a, CStr>> {
        self.to_cstring().map(|r| Rmo::Owned(r))
    }
}

impl<'b, T: ToCString+?Sized> ToCString for &'b T {
    fn to_cstring(&self) -> Result<CString> { (**self).to_cstring() }
    fn rmo_cstr<'a>(&'a self, buf: &'a mut [u8]) -> Result<Rmo<'a, CStr>> {
        (**self).rmo_cstr(buf)
    }
}

impl ToCString for CStr {
    fn to_cstring(&self) -> Result<CString> {
        self.as_ref().to_owned().map(|o| unsafe { CString::from_bytes_unchecked(o) })
    }
    fn rmo_cstr<'a>(&'a self, buf: &'a mut [u8]) -> Result<Rmo<'a, CStr>> {
        let bytes = self.as_ref();
        if bytes.len() <= buf.len() {
            mem::copy(buf, bytes);
            Ok(Rmo::Ref(unsafe { CStr::from_bytes_unchecked(&buf[..bytes.len()]) }))
        } else {
            self.to_cstring().map(|o| Rmo::Owned(o))
        }
    }
}

impl ToCString for [u8] {
    fn to_cstring(&self) -> Result<CString> {
        if let Some(idx) = memchr(self, 0) {
            if idx == self.len() - 1 || all_bytes(&self[idx+1..], 0) {
                self[..idx+1].to_owned().map(|o| unsafe { CString::from_bytes_unchecked(o) })
            } else {
                Err(error::InvalidArgument)
            }
        } else {
            let mut vec = try!(self.to_owned());
            try!(vec.reserve(1));
            vec.push(0);
            Ok(unsafe { CString::from_bytes_unchecked(vec) })
        }
    }

    fn rmo_cstr<'a>(&'a self, buf: &'a mut [u8]) -> Result<Rmo<'a, CStr>> {
        if let Some(idx) = memchr(self, 0) {
            if idx == self.len() - 1 || all_bytes(&self[idx+1..], 0) {
                Ok(unsafe { Rmo::Ref(CStr::from_bytes_unchecked(&self[..idx+1])) })
            } else {
                Err(error::InvalidArgument)
            }
        } else {
            if self.len() >= buf.len() {
                let mut vec = try!(self.to_owned());
                try!(vec.reserve(1));
                vec.push(0);
                Ok(Rmo::Owned(unsafe { CString::from_bytes_unchecked(vec) }))
            } else {
                mem::copy(buf, self);
                buf[self.len()] = 0;
                Ok(unsafe { Rmo::Ref(CStr::from_bytes_unchecked(&buf[..self.len()+1])) })
            }
        }
    }
}

impl ToCString for [i8] {
    fn to_cstring(&self) -> Result<CString> { self.as_ref().to_cstring() }
    fn rmo_cstr<'a>(&'a self, buf: &'a mut [u8]) -> Result<Rmo<'a, CStr>> {
        self.as_ref().rmo_cstr(buf)
    }
}

impl ToCString for str {
    fn to_cstring(&self) -> Result<CString> { self.as_ref().to_cstring() }
    fn rmo_cstr<'a>(&'a self, buf: &'a mut [u8]) -> Result<Rmo<'a, CStr>> {
        self.as_ref().rmo_cstr(buf)
    }
}