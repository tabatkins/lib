// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! File handling.

pub use linux_file::{File, Seek, Advice, info_no_follow};
pub use linux_file::_info as info;
pub use linux_file::flags::{Flags, Mode};
pub use linux_file::info::{Info, Type};
pub use linux_dev::{Device, DeviceType};
