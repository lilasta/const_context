use core::marker::PhantomData;

use crate::value::ConstValue;

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
