// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![doc(primitive = "isize")]

pub const BITS:  usize = ::linux_core::num::isize::BITS;
pub const BYTES: usize = ::linux_core::num::isize::BYTES;
pub const MIN:   isize = ::linux_core::num::isize::MIN;
pub const MAX:   isize = ::linux_core::num::isize::MAX;