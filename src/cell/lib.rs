// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_cell"]
#![crate_type = "lib"]
#![feature(lang_items, optin_builtin_traits, const_fn)]
#![no_std]

extern crate lrs_base as base;

pub use ref_cell::{RefCell, RefCellStatus, RefCellBorrow, RefCellBorrowMut};
pub use cell::{Cell};

pub mod std { pub use ::base::std::*; }

pub mod ref_cell;
pub mod cell;
