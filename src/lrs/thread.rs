// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Multi-threading
//!
//! = Examples
//!
//! ----
//! let mut array = [1u8; 1024];
//! {
//!     let res = scoped(|| {
//!         println!("getting to work");
//!         for i in 0..SIZE {
//!             array[i] = 2;
//!         }
//!         println!("done working");
//!     });
//!     println!("joining");
//!     res.unwrap();
//!     println!("joined");
//! }
//! for i in 0..SIZE {
//!     assert!(array[i] == 2);
//! }
//! ----

pub use lrs_thread::{Builder, spawn, scoped, JoinGuard};
