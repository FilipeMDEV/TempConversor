// Copyright 2019 The Fuchsia Authors
//
// Licensed under a BSD-style license <LICENSE-BSD>, Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0>, or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to
// those terms.

// See comment in `include.rs` for why we disable the prelude.
#![no_implicit_prelude]
#![allow(warnings)]

include!("include.rs");

// An enum is `IntoBytes` if if has a defined repr.

#[derive(imp::IntoBytes)]
#[repr(C)]
enum C {
    A,
}

util_assert_impl_all!(C: imp::IntoBytes);

#[derive(imp::IntoBytes)]
#[repr(u8)]
enum U8 {
    A,
}

util_assert_impl_all!(U8: imp::IntoBytes);

#[derive(imp::IntoBytes)]
#[repr(u16)]
enum U16 {
    A,
}

util_assert_impl_all!(U16: imp::IntoBytes);

#[derive(imp::IntoBytes)]
#[repr(u32)]
enum U32 {
    A,
}

util_assert_impl_all!(U32: imp::IntoBytes);

#[derive(imp::IntoBytes)]
#[repr(u64)]
enum U64 {
    A,
}

util_assert_impl_all!(U64: imp::IntoBytes);

#[derive(imp::IntoBytes)]
#[repr(usize)]
enum Usize {
    A,
}

util_assert_impl_all!(Usize: imp::IntoBytes);

#[derive(imp::IntoBytes)]
#[repr(i8)]
enum I8 {
    A,
}

util_assert_impl_all!(I8: imp::IntoBytes);

#[derive(imp::IntoBytes)]
#[repr(i16)]
enum I16 {
    A,
}

util_assert_impl_all!(I16: imp::IntoBytes);

#[derive(imp::IntoBytes)]
#[repr(i32)]
enum I32 {
    A,
}

util_assert_impl_all!(I32: imp::IntoBytes);

#[derive(imp::IntoBytes)]
#[repr(i64)]
enum I64 {
    A,
}

util_assert_impl_all!(I64: imp::IntoBytes);

#[derive(imp::IntoBytes)]
#[repr(isize)]
enum Isize {
    A,
}

util_assert_impl_all!(Isize: imp::IntoBytes);
