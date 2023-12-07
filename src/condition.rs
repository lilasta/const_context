use core::marker::PhantomData;

use crate::action::ActionContext;
use crate::value::bool::ConstBool;
use crate::value::ConstValue;
use crate::variable::list::{find_variable, is_variable_in};
use crate::variable::Variable;

pub trait Condition {
    type Bool<Ctx: ActionContext>: ConstBool;
}

pub struct IsSet<Var>(PhantomData<Var>)
where
    Var: Variable;

impl<Var> Condition for IsSet<Var>
where
    Var: Variable,
{
    type Bool<Ctx: ActionContext> = IsSet_<Ctx, Var>;
}

pub struct IsSet_<Ctx, Var>(PhantomData<(Ctx, Var)>);

impl<Ctx, Var> ConstValue for IsSet_<Ctx, Var>
where
    Ctx: ActionContext,
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
    type Bool<Ctx: ActionContext> = GetBool_<Ctx, Var>;
}

pub struct GetBool_<Ctx, Var>(PhantomData<(Ctx, Var)>);

impl<Ctx, Var> ConstValue for GetBool_<Ctx, Var>
where
    Ctx: ActionContext,
    Var: Variable<Type = bool>,
{
    type Type = bool;
    const VALUE: Self::Type = find_variable::<Ctx::Variables, Var>();
}
