// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use] extern crate linux;

use linux::time::*;

fn main() {
    let clock = Real;
    let now = clock.get_time().unwrap();

    let zone = Zone::local().unwrap();

    let exp = zone.expand(now);

    println!("{:?}", exp);
}
