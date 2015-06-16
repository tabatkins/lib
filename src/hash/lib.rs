// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_hash"]
#![crate_type = "lib"]
#![feature(plugin, no_std, const_fn, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core as core;
extern crate lrs_base as base;
extern crate lrs_wrapping as wrapping;

mod lrs { pub use base::lrs::*; }

use base::prelude::*;

pub mod xx_hash;

/// Objects that can be hashed.
pub trait Hash {
    /// Hashes the object into a hasher.
    ///
    /// [argument, h]
    /// The hasher to hash into.
    fn stateful_hash<H: Hasher>(&self, h: &mut H);

    /// Hashes a slice of objects into a hasher.
    ///
    /// [argument, val]
    /// The objects to hash.
    ///
    /// [argument, h]
    /// The hasher to hash into.
    fn stateful_hash_slice<H: Hasher>(val: &[Self], h: &mut H)
        where Self: Sized
    {
        for el in val {
            el.stateful_hash(h);
        }
    }

    /// Returns the hash of the object.
    ///
    /// [argument, seed]
    /// A seed for the hasher.
    fn hash<H: Hasher>(&self, seed: H::Seed) -> H::Digest {
        let mut hasher = H::new(seed);
        self.stateful_hash(&mut hasher);
        hasher.digest()
    }

    /// Returns the hash of a slice of objects.
    ///
    /// [argument, val]
    /// The objects to hash.
    ///
    /// [argument, seed]
    /// A seed for the hasher.
    fn hash_slice<H: Hasher>(val: &[Self], seed: H::Seed) -> H::Digest
        where Self: Sized
    {
        let mut hasher = H::new(seed);
        for el in val {
            el.stateful_hash(&mut hasher);
        }
        hasher.digest()
    }
}

/// Objects that can hash other objects.
pub trait Hasher {
    /// The type used to seed a hash operation.
    type Seed;
    /// The output of the hash operation.
    type Digest;

    /// Creates a new hasher for stateful hashing.
    ///
    /// [argument, seed]
    /// The seed of the hash operation.
    fn new(seed: Self::Seed) -> Self;

    /// Resets the hasher to it's initial state with a new seed.
    ///
    /// [argument, seed]
    /// The new seed of the hasher.
    fn reset(&mut self, seed: Self::Seed);

    /// Adds a slice of bytes to the hasher.
    ///
    /// [argument, val]
    /// The bytes to add to the hash state.
    fn write_bytes (&mut self, val: &[u8] );

    /// Adds a `u8` to the hasher.
    ///
    /// [argument, val]
    /// The value to add to the hash state.
    fn write_u8    (&mut self, val: u8    ) { self.write_bytes(val.as_ref()); }

    /// Adds a `u16` to the hasher.
    ///
    /// [argument, val]
    /// The value to add to the hash state.
    fn write_u16   (&mut self, val: u16   ) { self.write_bytes(val.as_ref()); }

    /// Adds a `u32` to the hasher.
    ///
    /// [argument, val]
    /// The value to add to the hash state.
    fn write_u32   (&mut self, val: u32   ) { self.write_bytes(val.as_ref()); }

    /// Adds a `u64` to the hasher.
    ///
    /// [argument, val]
    /// The value to add to the hash state.
    fn write_u64   (&mut self, val: u64   ) { self.write_bytes(val.as_ref()); }

    /// Adds a `usize` to the hasher.
    ///
    /// [argument, val]
    /// The value to add to the hash state.
    fn write_usize (&mut self, val: usize ) { self.write_bytes(val.as_ref()); }

    /// Adds an `i8` to the hasher.
    ///
    /// [argument, val]
    /// The value to add to the hash state.
    fn write_i8    (&mut self, val: i8    ) { self.write_bytes(val.as_ref()); }

    /// Adds an `i16` to the hasher.
    ///
    /// [argument, val]
    /// The value to add to the hash state.
    fn write_i16   (&mut self, val: i16   ) { self.write_bytes(val.as_ref()); }

    /// Adds an `i32` to the hasher.
    ///
    /// [argument, val]
    /// The value to add to the hash state.
    fn write_i32   (&mut self, val: i32   ) { self.write_bytes(val.as_ref()); }

    /// Adds an `i64` to the hasher.
    ///
    /// [argument, val]
    /// The value to add to the hash state.
    fn write_i64   (&mut self, val: i64   ) { self.write_bytes(val.as_ref()); }

    /// Adds an `isize` to the hasher.
    ///
    /// [argument, val]
    /// The value to add to the hash state.
    fn write_isize (&mut self, val: isize ) { self.write_bytes(val.as_ref()); }

    /// Returns the digest of the hasher.
    fn digest(&self) -> Self::Digest;

    /// Hashes a sequence of bytes.
    ///
    /// [argument, val]
    /// The bytes to hash.
    ///
    /// [argument, seed]
    /// The seed of the operation.
    fn hash_bytes( val: &[u8], seed: Self::Seed) -> Self::Digest;

    /// Hashes a `u8`.
    ///
    /// [argument, val]
    /// The value to hash.
    ///
    /// [argument, seed]
    /// The seed of the operation.
    fn hash_u8(    val: u8,    seed: Self::Seed) -> Self::Digest { Self::hash_bytes(val.as_ref(), seed) }

    /// Hashes a `u16`.
    ///
    /// [argument, val]
    /// The value to hash.
    ///
    /// [argument, seed]
    /// The seed of the operation.
    fn hash_u16(   val: u16,   seed: Self::Seed) -> Self::Digest { Self::hash_bytes(val.as_ref(), seed) }

    /// Hashes a `u32`.
    ///
    /// [argument, val]
    /// The value to hash.
    ///
    /// [argument, seed]
    /// The seed of the operation.
    fn hash_u32(   val: u32,   seed: Self::Seed) -> Self::Digest { Self::hash_bytes(val.as_ref(), seed) }

    /// Hashes a `u64`.
    ///
    /// [argument, val]
    /// The value to hash.
    ///
    /// [argument, seed]
    /// The seed of the operation.
    fn hash_u64(   val: u64,   seed: Self::Seed) -> Self::Digest { Self::hash_bytes(val.as_ref(), seed) }

    /// Hashes a `usize`.
    ///
    /// [argument, val]
    /// The value to hash.
    ///
    /// [argument, seed]
    /// The seed of the operation.
    fn hash_usize( val: usize, seed: Self::Seed) -> Self::Digest { Self::hash_bytes(val.as_ref(), seed) }

    /// Hashes an `i8`.
    ///
    /// [argument, val]
    /// The value to hash.
    ///
    /// [argument, seed]
    /// The seed of the operation.
    fn hash_i8(    val: i8,    seed: Self::Seed) -> Self::Digest { Self::hash_bytes(val.as_ref(), seed) }

    /// Hashes an `i16`.
    ///
    /// [argument, val]
    /// The value to hash.
    ///
    /// [argument, seed]
    /// The seed of the operation.
    fn hash_i16(   val: i16,   seed: Self::Seed) -> Self::Digest { Self::hash_bytes(val.as_ref(), seed) }

    /// Hashes an `i32`.
    ///
    /// [argument, val]
    /// The value to hash.
    ///
    /// [argument, seed]
    /// The seed of the operation.
    fn hash_i32(   val: i32,   seed: Self::Seed) -> Self::Digest { Self::hash_bytes(val.as_ref(), seed) }

    /// Hashes an `i64`.
    ///
    /// [argument, val]
    /// The value to hash.
    ///
    /// [argument, seed]
    /// The seed of the operation.
    fn hash_i64(   val: i64,   seed: Self::Seed) -> Self::Digest { Self::hash_bytes(val.as_ref(), seed) }

    /// Hashes an `usize`.
    ///
    /// [argument, val]
    /// The value to hash.
    ///
    /// [argument, seed]
    /// The seed of the operation.
    fn hash_isize( val: isize, seed: Self::Seed) -> Self::Digest { Self::hash_bytes(val.as_ref(), seed) }
}

mod impls {
    mod num;
    mod slice;
    mod result;
    mod option;
    mod errno;
    mod bool;
    mod char;
    mod tuple;
    mod ptr;
}
