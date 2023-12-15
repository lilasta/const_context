use core::marker::PhantomData;

use crate::context::ConstContext;
use crate::value::bool::ConstBool;
use crate::value::ConstValue;
use crate::variable::list::{find_variable, is_variable_in};
use crate::variable::Variable;

pub trait Condition {
    type Bool<Ctx: ConstContext>: ConstBool;
}

impl<T: ConstValue<Type = bool>> Condition for T {
    type Bool<Ctx: ConstContext> = Self;
}

pub struct IsSet<Var>(PhantomData<Var>)
where
    Var: Variable;

impl<Var> Condition for IsSet<Var>
where
    Var: Variable,
{
    type Bool<Ctx: ConstContext> = IsSet_<Ctx, Var>;
}

pub struct IsSet_<Ctx, Var>(PhantomData<(Ctx, Var)>);

impl<Ctx, Var> ConstValue for IsSet_<Ctx, Var>
where
    Ctx: ConstContext,
    Var: Variable,
{
    type Type = bool;
    const VALUE: Self::Type = is_variable_in::<Ctx::Variables, Var>();
}

pub struct GetBool<Var>(PhantomData<Var>)
where
    Var: Variable<Type = bool>;

impl<Var> Condition for GetBool<Var>
where
    Var: Variable<Type = bool>,
{
    type Bool<Ctx: ConstContext> = GetBool_<Ctx, Var>;
}

pub struct GetBool_<Ctx, Var>(PhantomData<(Ctx, Var)>);

impl<Ctx, Var> ConstValue for GetBool_<Ctx, Var>
where
    Ctx: ConstContext,
    Var: Variable<Type = bool>,
{
    type Type = bool;
    const VALUE: Self::Type = find_variable::<Ctx::Variables, Var>();
}
