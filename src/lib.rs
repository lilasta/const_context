#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "conditional", feature(associated_const_equality))]
#![cfg_attr(feature = "conditional", feature(specialization))]
#![feature(const_heap)]
#![feature(const_mut_refs)]
#![feature(const_refs_to_cell)]
#![feature(const_trait_impl)]
#![feature(const_transmute_copy)]
#![feature(const_type_id)]
#![feature(const_type_name)]
#![feature(core_intrinsics)]
#![feature(fn_traits)]
#![feature(inline_const)]
#![feature(never_type)]
#![feature(tuple_trait)]
#![feature(unboxed_closures)]

pub mod action;
#[cfg(feature = "conditional")]
pub mod conditional;
pub mod effect;
pub mod macros;
pub mod macros_set;
pub mod value;
pub mod variable;

mod tests;
mod utils;
