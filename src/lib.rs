#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "conditional", feature(associated_const_equality))]
#![cfg_attr(feature = "conditional", feature(specialization))]
#![feature(adt_const_params)]
#![feature(const_heap)]
#![feature(const_ptr_write)]
#![feature(const_type_id)]
#![feature(const_type_name)]
#![feature(core_intrinsics)]
#![feature(fn_traits)]
#![feature(impl_trait_in_assoc_type)]
#![feature(inline_const)]
#![feature(tuple_trait)]
#![feature(type_alias_impl_trait)]
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
