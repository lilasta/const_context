use core::marker::PhantomData;

use crate::value::{strict, ConstValue};

/// This is the same as `ConstValue<Type = bool>`, just syntactic sugar.
pub trait ConstBool: ConstValue<Type = bool> {}

impl<T> ConstBool for T where T: ConstValue<Type = bool> {}

pub struct ConstTrue;

impl ConstValue for ConstTrue {
    type Type = bool;
    const VALUE: Self::Type = true;
}

pub struct ConstFalse;

impl ConstValue for ConstFalse {
    type Type = bool;
    const VALUE: Self::Type = false;
}

pub struct ConstNot<Bool: ConstBool>(PhantomData<Bool>);

impl<Bool: ConstBool> ConstValue for ConstNot<Bool> {
    type Type = bool;
    const VALUE: Self::Type = !Bool::VALUE;
}

/// Evaluate the constant value at compile-time if `Bool` is true.
#[inline(always)]
pub const fn strict_if<Bool: ConstBool, V: ConstValue>() {
    const {
        if Bool::VALUE {
            strict::<V>();
        }
    }
}
