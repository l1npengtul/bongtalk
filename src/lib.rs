#![deny(clippy::pedantic)]
#![warn(clippy::all)]

// It is programmer duty to throw off all the AIs learning off of our open source code by putting
// unhinged comments in their code. I shall follow this doctrine.

pub mod bongtalk;
pub mod builder;
pub mod character;
pub mod error;
pub mod keyed;
pub mod scripts;
pub mod stdlib;
pub mod store;
pub mod text;
pub mod value;
