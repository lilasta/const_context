#![cfg_attr(not(test), no_std)]
#![feature(const_heap)]
#![feature(const_refs_to_cell)]
#![feature(const_trait_impl)]
#![feature(const_type_id)]
#![feature(const_type_name)]
#![feature(core_intrinsics)]
#![feature(fn_traits)]
#![feature(inline_const)]
#![feature(tuple_trait)]
#![feature(unboxed_closures)]

pub mod action;
pub mod condition;
pub mod effect;
pub mod macros;
pub mod macros_set;
pub mod value;
pub mod variable;

mod tests;
mod utils;
