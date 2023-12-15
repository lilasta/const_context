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

pub struct ConstNot<Bool>(PhantomData<Bool>);

impl<Bool> ConstValue for ConstNot<Bool>
where
    Bool: ConstBool,
{
    type Type = bool;
    const VALUE: Self::Type = !Bool::VALUE;
}

pub struct ConstAnd<B1, B2>(PhantomData<(B1, B2)>);

impl<B1, B2> ConstValue for ConstAnd<B1, B2>
where
    B1: ConstBool,
    B2: ConstBool,
{
    type Type = bool;
    const VALUE: Self::Type = B1::VALUE && B2::VALUE;
}
