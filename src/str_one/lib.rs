// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_str_one"]
#![crate_type = "lib"]
#![feature(type_ascription)]
#![allow(mutable_transmutes)]
#![no_std]

extern crate lrs_cty_base as cty_base;
extern crate lrs_base as base;
extern crate lrs_arch_fns as arch_fns;
extern crate lrs_parse as parse;
extern crate lrs_fmt as fmt;

pub use byte_str::{ByteStr};
pub use no_null_str::{NoNullStr};
pub use c_str::{CStr};

mod std { pub use ::base::std::*; }

mod byte_str;
mod no_null_str;
mod c_str;

mod cmp;
mod conv;
