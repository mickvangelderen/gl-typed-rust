#![allow(non_camel_case_types)]
#![allow(clippy::all)]
//
//! This module defines a set of zero sized types that can be used as markers or
//! whatever. The symbols can be re-used to represent different variants in
//! multiple enums, which is why are defined in their own module.

include!(concat!(env!("OUT_DIR"), "/symbols.rs"));
